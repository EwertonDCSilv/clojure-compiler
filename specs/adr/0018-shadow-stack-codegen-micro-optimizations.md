# ADR-0018 — Shadow-stack micro-optimizations from llvm-mca and OSACA inner-loop analysis

- Status: **Proposed**
- Date: 2026-07-31
- Related:
  [ADR-0002](0002-memory-management.md),
  [ADR-0006](0006-codegen-optimization.md),
  [ADR-0009](0009-benchmark-performance-study.md),
  [ADR-0017](0017-selective-zero-slot-gc-frames.md), and
  [Memory model](../MEMORY_MODEL.md)

## 1. Context

Two independent tools analyzed the innermost `recur` loop of
`cormen_number_sieve-of-eratosthenes__mark-multiples` — the hottest function in the
sieve benchmark, responsible for the bulk of its 6 101 550 allocations per run
(ADR-0009 §3.2).

**OSACA 0.7.1** (`--arch ZEN4 --syntax ATT --ignore-unknown`) analyzed a manually
extracted `.s` file with begin/end markers. It reports the throughput bound for
instructions present in its database; instructions marked `X` (missing data) are
excluded from port totals.

**llvm-mca 19** (`-mcpu=znver4 --timeline --all-stats`, 100 iterations) analyzed the
same kernel in Intel syntax. It models the full out-of-order pipeline including
register renaming, ROB pressure, structural hazards, and dependency chains — including
the instructions OSACA could not account for.

The kernel spans addresses `0x7694`–`0x783b` in the not-stripped ELF binary
`sieve-of-eratosthenes`, disassembled with `objdump -d` and confirmed against the
source loop structure of `mark-multiples`.

## 2. Measurement results

### 2.1 Summary

| Metric | OSACA | llvm-mca |
|---|---|---|
| Throughput bound (resource) | 14.5 cy/iter | 16.3 cy/iter (Block RThroughput) |
| **Actual cycles/iter (modeled)** | — | **32.84 cy/iter** |
| IPC | — | 2.98 / 6 (50% dispatch utilization) |
| RAT stalls (register rename blocked) | — | **1610 cy = 49% of total** |
| ROB max occupancy | — | 298 / 320 = 93% |
| Integer PRF max occupancy | — | **224 / 224 = 100%** |

The real cost per iteration is **2× higher than OSACA's throughput bound** because
OSACA excluded the `imul`, `add [mem]`, `setne/setge/setle`, `cmovg`, and `call`
instructions from its port model (all marked `X`). llvm-mca captures them all.

### 2.2 Resource pressure per iteration (llvm-mca, Zen4)

```
Resource   cy/iter   bar
AGU0        14.94   ██████████████
AGU1        15.01   ███████████████
AGU2        15.05   ███████████████  ← throughput bottleneck
ALU0        11.43   ███████████
ALU1        12.55   ████████████     (6 cy from IMULs)
ALU2         9.02   █████████
ALU3        11.13   ███████████
Store0      12.54   ████████████
Store1      12.46   ████████████
LSU0        12.24   ████████████
LSU1        11.90   ███████████
LSU2        11.86   ███████████
BRU          7.87   ███████
```

**The three AGU ports are the throughput bottleneck at ~15 cycles per iteration.**
OSACA attributed the bottleneck to ports 0/2/6/10 (integer ALU) because it excluded
the `imul` and `add [mem]` µops that feed AGU. With those included, AGU pressure
exceeds all ALU ports.

### 2.3 True bottleneck: integer PRF exhaustion (49% RAT stalls)

The integer physical register file (PRF) has 224 entries. llvm-mca reports that all
224 were simultaneously in use for the entire steady-state run (`Max number of mappings
used: 224`). The dispatch unit was stalled for register availability (RAT) in 49% of
all cycles — nearly every other cycle.

The cause: the kernel body has ~18 explicit memory loads (one per `lea [rip+X]` /
`mov rdx, [mem]` pair) and 3 `add [mem], -N` read-modify-write decrements. Each load
creates a physical register mapping that remains live until the dependent store
commits. The 3 RMW operations hold PRF entries for ~18 cycles each (visible in the
timeline as `eeeeeeE` chains). With 98 instructions in the loop body and only 224 PRF
slots shared across the reorder window, the rename stage cannot sustain full dispatch
width.

### 2.4 Individual instruction costs (selected, from timeline and wait-time table)

| Instruction | Avg wait (cy) | Exec chain (cy) | Note |
|---|---|---|---|
| `imul rdi, rdx, 8` (push 1) | 7.0 | 3 | Feeds store; only ALU1 can execute |
| `imul r9, rsi, 8` (push 2) | 7.5 | 3 | Same port, sequential |
| `add [rax], -1` (gc\_sp pop 1) | 11.5 | 6 | RMW; holds PRF ~18 cy |
| `add [rax], -1` (gc\_sp pop 2) | 13.5 | 6 | Serialized after pop 1 |
| `add [rdx], -2` (frame pop) | **14.5** | 6 | **Longest wait in loop** |
| `mov rax, [rsp]` (spill reload) | 2.0 | 5 | 4 cy L1 load on critical path |
| `call *[rcx]` (cljn\_assoc) | 2.0 | 100 | Modeled at 100 cy; bodies excluded |

The three RMW decrements of `gc_sp` are the longest-waiting instructions in the loop:
they serialise on the store port and hold PRF entries for ~18 cycles each. They are
the principal drivers of PRF saturation and the 49% RAT stall rate.

### 2.5 Revised imul assessment: D1 does not help throughput

The 6 `imul $0x8` instructions (RThroughput=1.0 on ALU1 exclusively) account for 6 of
the 12.55 cycles of ALU1 pressure. Replacing them with `lea (,%r,8)` would move those
6 µops from ALU1 to AGU0/1/2 — the already-saturated bottleneck. The throughput bound
would not improve; AGU pressure would rise from 15 cy to ~17 cy. D1 as originally
framed provides **latency relief on the address chain** (3 cy → 1 cy per push), not
throughput relief, and the savings reduce to ~1 cy/iter rather than the 4.5 cy
estimated from OSACA alone.

## 3. Root cause

The loop body contains ~50 instructions devoted to shadow-stack bookkeeping (6 push
sequences + 3 pop decrements + 2 frame-refresh stores) versus ~12 instructions of
actual fixnum arithmetic. The bookkeeping pattern is:

```
for each push:
    lea  rcx, [rip + &gc_sp]      # AGU op: load address of gc_sp global
    mov  rdx, [rcx]                # AGU+Load: read gc_sp
    lea  rsi, [rip + &gc_stack]    # AGU op: load address of gc_stack base
    imul rdi, rdx, 8               # ALU1: compute byte offset
    mov  [rsi + rdi], value        # AGU+Store: write gc_stack[gc_sp]
    add  rdx, 1 / lea rax, [rdx+1]
    mov  [rcx], rdx                # AGU+Store: write gc_sp back
```

Every push costs **3 LEA + 1 LOAD + 1 IMUL + 2 STORE = 7 µops on AGU/ALU/Store**.
Six pushes per iteration = 42 bookkeeping µops that saturate AGU to 15 cy. The 3
`add [mem], -N` pop decrements add 9 more fused µops (3 load + 3 ALU + 3 store) and
hold the PRF entries that trigger the 49% RAT stall.

**The primary cause is that `gc_sp` is a global variable read and written on every
push and pop.** Each access round-trips through the AGU pipeline and creates a PRF
mapping. If `gc_sp` were cached in a register for the duration of a function body, the
load and store of `gc_sp` in every sequence would disappear, along with the RMW
decrements.

## 4. Decision drivers

- Target the actual bottleneck: AGU saturation and PRF exhaustion, not ALU pressure.
- Preserve the GC invariants of ADR-0002 and ADR-0006; no ABI changes that require
  reopening those decisions unless the evidence clearly justifies it.
- Prioritize decisions by impact: single-point changes that address the 49% RAT stall
  win more than latency tweaks to individual instructions.
- Keep each decision independently verifiable by disassembling the generated binary.

## 5. Decisions

Decisions are ordered by impact descending.

### D1 — Cache `gc_sp` in a callee-saved register for the function body (highest impact)

Keep `gc_sp` in a dedicated callee-saved integer register (e.g., `%rbp`, freed by
adopting a frame-pointer-free ABI for generated functions, or a sixth callee-saved
register reserved for the GC layer) for the duration of each generated function body.
On function entry, load `gc_sp` from the global once. On every push and pop, update
the register directly. On function exit — and before every call that can reach a GC
safepoint — flush the register back to the global.

Effect on the kernel (per iteration):
- Removes 6 × `lea [rip+&gc_sp]` = 6 AGU ops
- Removes 6 × `mov rdx, [rcx]` = 6 load ops
- Removes 3 × `lea [rip+&gc_sp]; add [rax], -N` = 6 AGU ops
- Removes 3 × PRF mappings held for ~18 cy each

Total: **−18 AGU ops from ~45/iter → AGU pressure: ~15 cy → ~9 cy (−40%).**

Estimated cycles/iter: **32.84 → ~19.7 cy (−40%).**

**ABI constraint:** every generated function must flush the register before any
`MaySafepoint` call (ADR-0006 classification) and reload it after. The C runtime
functions that call back into Clojure must save and restore the register at the
cross-ABI boundary. The shadow-stack frame format (ADR-0002) does not change; only
the mechanism for reading and updating `gc_sp` changes.

**This decision requires opening a limited amendment to ADR-0002** to declare the
register as a GC-layer-reserved callee-saved register and specify its flush protocol.
The amendment does not change the shadow-stack layout, rooting invariants, or GC
algorithm.

### D2 — Decompose `add [mem], -N` gc\_sp decrements into load / sub-reg / store

Replace the three fused `add qword ptr [addr], -N` instructions with explicit
load-subtract-store triples emitted as separate Cranelift IR instructions:

```
tmp = load(gc_sp_addr)
tmp = iadd_imm(tmp, -N)
store(tmp, gc_sp_addr)
```

If D1 is implemented first, D2 becomes obsolete for `gc_sp` (the variable is in a
register). D2 is still valuable independently:

- The three RMW ops hold PRF entries for ~18 cycles each, driving the 49% RAT stall.
  Decomposing them into separate µops lets the OoO engine schedule the load earlier
  and the store later, reducing the mapping lifetime.
- On the current code without D1, D2 alone saves approximately **3 cy/iter** of
  RAT-stall overhead.
- On code with D1, D2 is a no-op for `gc_sp` but may still apply to other RMW sites
  in the runtime.

### D3 — Eliminate the `(%rsp)` spill by reducing live GC-state registers

The stack spill of `new_multiple` (store to `[rsp]` then reload) exists because all
five callee-saved integer registers are occupied by GC state (`%rbx` = prime,
`%r12` = limit, `%r13` = multiple, `%r14` = gc frame base, `%r15` = flags). The
reload has 5 cycles of L1 latency and contributes 2 LSU ops per iteration.

If D1 frees a callee-saved register (by moving `gc_sp` to a dedicated GC register),
`%r14` (frame base) can be eliminated by the same mechanism or by restructuring the
frame-refresh sequence to use `gc_sp` directly. Either outcome frees one
callee-saved register for `new_multiple`, removing the spill entirely.

**Expected saving (with D1):** ~1.5 cy/iter from 2 fewer LSU ops and removal of
the 5-cycle load from the critical path.

### D4 — Replace `imul $0x8` with `shl $3` for latency relief on push chains

The 6 `imul rdi, rdx, 8` instructions (3-cycle latency, exclusively on ALU1) form the
address-generation chain for each shadow-stack push: `load gc_sp → imul → store`. At
3-cycle latency, each push's store cannot begin until 3 cycles after the load. Replacing
with `shl $3` (1-cycle latency, any ALU port) shortens this chain to 1 cycle.

**This does not improve throughput** because the replacement `shl` µops use ALU ports
rather than AGU, and AGU is the bottleneck. However, it reduces the load-to-store
latency on each push, which may allow the OoO engine to issue subsequent stores sooner
and reduce the effective wait time visible in the timeline.

If D1 is implemented, the number of residual `imul` instructions in a push sequence
drops from 6 to 6 (gc\_sp is now a register, so the load disappears but the offset
calculation remains). D4 still applies to those 6 remaining address computations.

**Expected saving:** ~1 cy/iter (latency only, not throughput).

### D5 — Emit direct calls for statically resolved `defn` sites

When the callee is a top-level `defn` whose closure is `NIL`, emit `call <symbol>`
instead of `lea [rip+X], %reg; call *%reg`. This removes the load-to-branch latency
(~2 cy) and gives the branch predictor a direct target.

D5 does not affect the AGU bottleneck or PRF saturation; its impact on the
`mark-multiples` loop is limited to the `cljn_assoc` call site (one call per
iteration). Expected saving: ~0.5 cy/iter on the loop body boundary; main benefit is
at monomorphic call sites throughout the compiled program.

## 6. Alternatives considered

| Alternative | Advantage | Problem | Decision |
|---|---|---|---|
| D1 only via `%rbp` (omit frame pointer) | frees a register cheaply | changes stack-walk assumptions; may conflict with sanitizers | investigate during D1 implementation |
| Thread-local `gc_sp` via `%fs`-relative access | single instruction per access | platform-specific; changes C runtime ABI; complex interaction with `cljn_gc_enter` | deferred |
| `imul→lea` as primary optimization (original D1) | no ABI change | moves pressure to AGU bottleneck; net gain ~1 cy, not 4.5 cy | downgraded to D4 |
| GC-free region for inner loop (escape proof) | eliminates all shadow-stack overhead | requires interprocedural escape analysis not yet implemented | planned separately (ADR-0009 §6) |
| Nursery / bump allocator for ephemeral objects | reduces `cljn_assoc` body cost | targets the 85% cost (assoc body), not the 15% overhead; separate concern | deferred to structural sharing work |
| Keep current code | zero risk | AGU at 100%, PRF at 100%, 49% stall rate; 32.84 cy/iter | rejected |

## 7. Projected impact (revised with llvm-mca data)

Baseline: **32.84 cy/iter** modeled by llvm-mca.

| Decision | cy saved | cy/iter after | Note |
|---|---|---|---|
| D1 (`gc_sp` in register) | ~13 cy | ~19.7 | −40%; removes 18 AGU ops and RMW PRF pressure |
| + D2 (RMW→3-op) | ~2 cy | ~17.7 | Residual benefit if D1 leaves any RMW sites |
| + D3 (eliminate spill) | ~1.5 cy | ~16.2 | Requires D1 to free a callee-saved register |
| + D4 (imul→shl) | ~1 cy | ~15.2 | Latency only; approaches RThroughput bound |
| + D5 (direct call) | ~0.5 cy | ~14.7 | Approaches Block RThroughput of 16.3 cy |
| **D1–D5 combined** | **~18 cy** | **~14.7 cy** | **−55%; near throughput-bound** |

Translation to sieve wall time (scale 25×, mark-multiples fraction ~24% of 4.60 s wall):

| Scenario | mark-mult wall | sieve wall | vs JVM (1.78 s) |
|---|---|---|---|
| Baseline | ~1.11 s | 4.60 s | 2.58× slower |
| D1 only | ~0.67 s | ~4.16 s | 2.34× slower |
| D1–D5 | ~0.50 s | ~3.99 s | 2.24× slower |

**The D1–D5 package closes ~13% of the sieve wall-time gap** without changing the
allocation volume. Closing the remaining 1.24× gap requires eliminating allocations
(escape analysis / auto-transient / structural sharing), not further loop micro-tuning.

## 8. Implementation order

1. **D1** — the highest-impact decision and a prerequisite for D2 and D3 becoming
   no-ops on `gc_sp`. Requires a targeted ADR-0002 amendment for the register
   reservation and flush protocol. Implement and measure with the Cormen paired gate
   before proceeding.
2. **D2** — implement independently of D1 on any remaining `add [mem],-N` sites.
   If D1 eliminates all `gc_sp` RMWs, D2 may be a no-op; confirm by disassembly.
3. **D3** — implement after D1 confirms a free callee-saved register is available.
4. **D4** — implement as a codegen cleanup; no ABI change. Low risk, apply alongside D1.
5. **D5** — implement as a separate, low-risk codegen pass independent of the others.

## 9. Validation

| Decision | Structural assertion |
|---|---|
| D1 | Disassemble a binary from `mark-multiples`; assert zero `lea [rip+&gc_sp]; mov` pairs in the loop body; assert one load at function entry and one store before each `MaySafepoint` call. |
| D2 | Disassemble; assert zero `add qword ptr [mem], $imm` where `mem` is any `gc_sp` address; assert load/sub/store triple at each pop site not covered by D1. |
| D3 | Re-run llvm-mca on the updated kernel; assert the `mov [rsp]` / `mov rax, [rsp]` pair is absent. |
| D4 | Disassemble; assert zero `imul $0x8` in the loop body; assert presence of `shl $3` or scaled-index `lea`. |
| D5 | Compile a `defn` with no captures; assert `call cljn_assoc` (direct symbol), not `lea …; call *%reg`. |
| All | Re-run llvm-mca on the updated kernel; assert cycles/iter ≤ 20 (D1 alone) and ≤ 16 (D1–D5). |

All decisions must additionally pass:

- `make test-runtime-sanitize` (`CLJN_GC_STRESS=1`, ASan, UBSan) — D1 changes the
  flush protocol for `gc_sp`, which is a rooting invariant; any flush omission before a
  safepoint would cause a use-after-collect.
- `make compatibility` — all 30 Cormen checksums unchanged.
- Cormen paired gate (ADR-0014 protocol, ≥10 alternating repetitions) before claiming
  any wall-time result. Single-run snapshots are not evidence.

## 10. Acceptance

This decision is implemented when:

1. the five structural disassembly assertions in §9 each pass on the production binary;
2. `CLJN_GC_STRESS=1` and ASan/UBSan pass on the full end-to-end suite;
3. a re-run of llvm-mca on the updated kernel reports cycles/iter ≤ 16;
4. all 30 Cormen checksums remain unchanged;
5. the Cormen paired gate does not show an aggregate regression; and
6. ADR-0009 is updated with a new snapshot section covering the post-D1–D5 run.
