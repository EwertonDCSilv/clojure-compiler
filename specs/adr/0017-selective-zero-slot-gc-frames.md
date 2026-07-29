# ADR-0017 — Elide zero-slot GC frames by temporary-root balance

- Status: **Accepted; implemented in #149**
- Date: 2026-07-29
- Related:
  [ADR-0002](0002-memory-management.md),
  [ADR-0006](0006-codegen-optimization.md),
  [ADR-0015](0015-internal-value-root-and-abi-specialization.md),
  [Memory model](../MEMORY_MODEL.md), and
  [Testing strategy](../TESTING_STRATEGY.md)

## Context

Generated code uses a precise shadow stack. A function frame records the incoming
stack pointer, reserves fixed local root slots, and restores the pointer on normal
return. Expression lowering also pushes temporary roots for heap-capable values and
balances them as enclosing operations consume their operands.

The first compact-frame implementation omitted the frame whenever the fixed local
root plan was empty. That was unsound: a rootless function returning a heap value
leaves the value temporarily pushed for its caller, so returning without
`cljn_gc_leave` leaked one shadow-stack entry per call. Commit `3144b54` fixed the
leak by emitting a zero-slot frame for every rootless function.

The unconditional fix restored correctness but also restored frame traffic where
the generated function has neither local root slots nor an unbalanced temporary
root at return. Allocation alone is not the deciding property. For example, an
equality expression may allocate string operands, balance both operand roots, and
return an immediate boolean without changing the shadow-stack depth.

A ten-pair, alternating Cormen comparison between the historical `b9f4703` build
and the current build did not reproduce the previously inferred aggregate
regression: chapter 06 candidate/control was 0.9714 wall and 0.9724 CPU, with both
confidence intervals crossing 1.0. The 27.23-second historical result was a
single-run snapshot and is not causal evidence against `3144b54`. This decision
therefore addresses the independently reproducible zero-slot frame overhead
without claiming to recover that historical snapshot.

## Decision drivers

- Preserve precise GC and restore every temporary root left for a caller.
- Avoid `cljn_gc_enter`/`cljn_gc_leave` when they cannot change observable root
  state.
- Reuse the code generator's existing conservative temporary-root accounting.
- Treat unknown and heap-capable results conservatively.
- Keep the direct pipeline and the optional ADR-0015 entry behavior consistent.

## Decision

Emit a generated function frame when either condition holds:

1. its root plan contains at least one fixed local slot; or
2. any normal method result leaves a temporary heap root for the caller.

When both conditions are false, omit `cljn_gc_enter` and `cljn_gc_leave`.
Expression-internal pushes and pops remain balanced by the existing lowering
contract. An expression may allocate and still omit the frame when all temporary
roots are consumed before it produces an immediate result.

`expr_pushes` is the conservative executable proof for the second condition. Heap
literals, captures, globals, unknown calls, and joined control-flow results remain
heap-capable and require cleanup. Proven immediate results do not. If the analysis
cannot prove balance, it retains the frame.

The process entry point keeps its frame. It executes once, owns command-line setup,
and explicitly discards top-level results, so optimizing it would not improve a hot
path and would widen the exception-unwind surface unnecessarily.

### GC invariants

- `GC:` a normal return must restore the shadow-stack pointer to the value observed
  at function entry after preserving the returned value according to the callable
  ABI.
- `GC:` a function with fixed local root slots always enters a frame.
- `GC:` a zero-slot function that can leave a temporary heap root always enters a
  frame.
- `GC:` frame elision is legal only when every internal temporary push is balanced
  and no normal result remains pushed.
- `GC:` unknown result kinds are heap-capable and retain the frame.

## Alternatives considered

| Alternative | Advantage | Problem | Decision |
| --- | --- | --- | --- |
| Emit every zero-slot frame | simplest safety rule | adds two runtime calls when stack depth is already balanced | rejected |
| Omit every zero-slot frame | smallest generated path | repeats the shadow-stack leak fixed by `3144b54` | rejected |
| Require a frame for every allocating expression | conservative safepoint rule | confuses allocation with unbalanced root depth and misses safe elision | rejected |
| Use existing temporary-root balance | matches lowering semantics and preserves unknown fallback | depends on maintaining `expr_pushes` with new AST forms | **chosen** |

## Consequences

### Positive

- Pure rootless functions no longer execute zero-slot frame calls.
- Rootless functions with balanced allocating intermediates may also omit the
  frame.
- Rootless functions returning heap values retain the cleanup that prevents
  shadow-stack growth.
- Structural statistics expose the decision through `root_frame_entries`.

### Costs and risks

- Every new AST form must keep `kind_of` and `expr_pushes` conservative.
- A mistaken heap-to-immediate classification could reintroduce root leakage.
- This local optimization does not implement full safepoint liveness or virtual
  roots from ADR-0015.

## Validation

The pure policy unit test covers every combination of fixed slots and a rooted
result. The structural regression test then compares generated frame counts for:

- an immediate rootless result, which adds no frame;
- allocating operands whose temporary roots are balanced before an immediate
  result, which add no frame; and
- a returned heap value, which adds one frame.

The existing end-to-end 12.5-million-call regression remains blocking and verifies
that a rootless allocating function returning a string restores its frame.
GC-stress, runtime sanitizers, compatibility, and the repository push gates remain
required before merge.

The post-implementation 30-case Cormen run used ten paired, alternating repetitions
at scale 25 and preserved every checksum. Candidate/control was 1.0040 wall
(95% CI 0.9933–1.0142) and 1.0035 CPU (95% CI 0.9931–1.0123). The strict promotion
gate did not pass because the aggregate point estimates were above 1.0; neither
interval confirms an aggregate regression or gain.

## Acceptance

This decision is implemented when:

1. the structural test fails on the unconditional-frame implementation and passes
   with selective elision;
2. the existing rootless heap-return regression remains green;
3. generated frame statistics distinguish the immediate and heap-return cases;
4. GC stress and runtime sanitizers pass;
5. all compatibility checksums remain unchanged; and
6. the performance report does not attribute unrelated single-run Cormen variance
   to this frame decision.
