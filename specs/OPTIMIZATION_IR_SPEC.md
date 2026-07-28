# Optional Optimization IR Specification

- Status: **Planned**
- Decision: [ADR-0014](adr/0014-optional-optimization-ir.md)
- Related:
  [compiler pipeline](COMPILER_PIPELINE.md),
  [ADR-0001](adr/0001-code-generation-backend.md),
  [ADR-0002](adr/0002-memory-management.md),
  [ADR-0006](adr/0006-codegen-optimization.md),
  [ADR-0009](adr/0009-benchmark-performance-study.md),
  [ADR-0010](adr/0010-interprocedural-ephemeral-vectors.md), and
  [optimization plan](optime.md)

## 1. Purpose

This specification defines a compiler-owned intermediate representation and a
conservative optimization pipeline between the analyzed Clojure program and Cranelift.
The IR exists to expose Clojure semantics, effects, control flow, safepoints, and value
facts before backend lowering so that the compiler can emit simpler Cranelift IR and,
consequently, better machine code.

The complete path is **Planned**:

```text
Reader
  -> known expansion
  -> Analyzer AST
  -> Optimization IR
  -> verified optimization passes
  -> root plan
  -> Cranelift IR
  -> machine object
  -> native executable
```

The current direct path remains the default:

```text
Reader -> known expansion -> Analyzer AST -> Cranelift IR -> native executable
```

The optimization IR is opt-in. Cargo release mode, Cranelift optimization level, or
the presence of profiling data must not enable it implicitly.

## 2. Goals

The first complete gate must:

1. preserve the observable semantics of the direct codegen path;
2. remove redundant calls, guards, branches, loads, stores, and allocations where a
   proof is available;
3. make liveness and GC root placement explicit at safepoints;
4. retain exact slow paths for dynamic types, overflow, exceptions, and unsupported
   specialization;
5. remain backend-neutral above the final Cranelift lowering;
6. produce deterministic, source-related IR dumps for tests and diagnosis;
7. keep the feature disabled by default; and
8. pass the Cormen non-regression gate in section 13 before a pass enters the public
   optimization profile.

The implementation must improve measured native execution time. Reducing instruction
count or producing visually smaller assembly is not sufficient acceptance evidence.

## 3. Non-goals

The initial IR does not:

- replace Cranelift;
- rewrite textual assembly or depend on x86_64 instruction spelling;
- introduce a JIT, profile-guided recompilation, or deoptimization;
- change the public tagged `Value` representation or Clojure call ABI;
- change language semantics, numeric overflow behavior, exception order, or evaluation
  order;
- introduce a moving or concurrent collector;
- infer complete static Clojure types;
- make the optimizer the default; or
- accept a Cormen regression in exchange for a gain in another benchmark suite.

LLVM, a C backend, PGO, vectorization, and architecture-specific peepholes remain
separate decisions.

## 4. CLI and configuration contract

### 4.1 Public switch

**Planned:** `clojure-native build` accepts:

```text
--ir-opt none|safe
```

`none` is the default and retains the direct Analyzer-AST-to-Cranelift path. `safe`
selects the verified IR pipeline and only the passes that have satisfied the Cormen
gate.

Examples:

```bash
# Current/default pipeline
clojure-native build app.clj -o app --ir-opt none

# Optional optimization IR
clojure-native build app.clj -o app-opt --ir-opt safe
```

The existing Cranelift option remains independent:

```text
--opt-level none|speed|speed-and-size
```

The primary IR acceptance comparison uses `--opt-level none` on both sides, isolating
frontend optimization from Cranelift's own optimizer. Additional `speed` measurements
are informative and cannot replace the primary gate.

### 4.2 Default invariants

- Omitting `--ir-opt` is equivalent to `--ir-opt none`.
- `cargo build --release` does not change the IR mode.
- No environment variable silently enables IR optimization.
- An unknown IR mode is a CLI error.
- This specification does not permit `safe` to become the default. That change requires
  a superseding ADR and a new benchmark decision.

### 4.3 Diagnostic output

**Planned:** a developer-only dump option writes deterministic textual IR before and
after each selected pass. Dumps contain source identifiers and byte spans, stable block
and value numbering, pass names, and no pointer addresses or temporary paths.

IR dumping must not change optimization decisions or generated code.

## 5. Ownership and crate boundary

**Planned:** add a `clojure-ir` workspace crate.

```text
crates/clojure-ir/
├── src/
│   ├── lib.rs
│   ├── model.rs
│   ├── builder.rs
│   ├── effects.rs
│   ├── verify.rs
│   ├── print.rs
│   ├── analysis/
│   │   ├── dominators.rs
│   │   ├── liveness.rs
│   │   └── loops.rs
│   └── passes/
│       ├── simplify_cfg.rs
│       ├── sccp.rs
│       ├── cse.rs
│       ├── dce.rs
│       ├── guard_elimination.rs
│       └── licm.rs
└── tests/
```

Responsibilities:

- `clojure-analyzer` owns Clojure name resolution, arity, lexical scope, and semantic
  validation.
- `clojure-ir` owns canonical control flow, facts, effects, verification, liveness, and
  backend-neutral transformations.
- `clojure-codegen` owns final lowering to Cranelift, ABI calls, object emission, and
  runtime linkage.
- `clojure-native-cli` owns option parsing and pipeline selection.

Cranelift types, blocks, instructions, and function handles must not appear in the
public `clojure-ir` model.

## 6. Core IR model

### 6.1 Program and functions

The IR is a control-flow graph with block parameters rather than implicit AST nesting
or mutable local slots:

```text
Module
  functions: Function[]
  globals: Global[]

Function
  name
  source span
  parameters: ValueId[]
  entry: BlockId
  blocks: Block[]
  ABI visibility

Block
  parameters: ValueId[]
  instructions: Instruction[]
  terminator: Terminator
```

Block parameters represent values merged from predecessor edges. The verifier rejects
missing, extra, or representation-incompatible branch arguments. There are no implicit
fallthrough edges.

### 6.2 Values and representation facts

Every produced value has one `ValueId` and a conservative representation fact:

```text
UnknownTagged
FixnumTagged
FixnumUnboxed
BooleanImmediate
NilImmediate
HeapReference
CallableTagged
```

Facts are optimization evidence, not new language types. `UnknownTagged` is the
fallback whenever control-flow merge, indirect call, mutable state, or an unsupported
operation prevents a stronger proof.

The public Clojure ABI continues to receive and return tagged `Value`s. An unboxed
fixnum is legal only inside a function and must be checked and retagged before an ABI
boundary or a merge that requires `UnknownTagged`.

### 6.3 Effects

Effects are orthogonal flags, not a single optimistic enum:

```text
ReadsState
WritesState
MayAllocate
MaySafepoint
MayThrow
MayPerformIO
```

An instruction with no flags is pure. Unknown or indirect calls receive every effect
that cannot be disproved. Runtime imports use one centralized effect table shared with
the final lowering.

Effect rules:

- `MayThrow` preserves observable exception and evaluation order.
- `MaySafepoint` constrains movement and triggers root-liveness requirements.
- `WritesState` prevents common-subexpression elimination and unsafe reordering.
- `MayPerformIO` is never removed or duplicated.
- Removing an unused result is legal only when the instruction has no observable
  effect.
- A runtime function changing from non-allocating to allocating must update its effect
  declaration in the same change.

### 6.4 Instructions

The first model must represent:

- tagged and unboxed constants;
- numeric, boolean, comparison, and tag operations;
- checked fixnum arithmetic with explicit overflow edges;
- collection construction and access;
- direct, indirect, primitive, and runtime calls;
- global reads and initialization;
- closure construction and capture reads;
- guards with explicit success and slow-path behavior;
- root-frame operations introduced after liveness;
- source-related throw and runtime error operations; and
- structural operations required by records, protocols, and multimethod dispatch.

The model may grow only when the verifier, printer, final lowering, effects, and tests
are updated together.

### 6.5 Terminators

Every block ends with exactly one:

```text
Branch(target, arguments)
CondBranch(condition, then_target, then_arguments, else_target, else_arguments)
Return(value)
Throw(value)
Unreachable
```

Calls that can throw retain their normal result edge and exception behavior. The
initial implementation may model the current runtime exception transfer as an effect
rather than an explicit exceptional CFG edge, but passes must then treat `MayThrow` as
an ordering barrier.

## 7. Lowering contracts

### 7.1 Analyzer AST to IR

Lowering preserves:

- left-to-right evaluation;
- source spans on effectful and potentially throwing operations;
- lexical captures and current function ABI;
- direct versus indirect calls;
- exact type, overflow, bounds, and arity slow paths;
- `if`, `loop`, and `recur` control flow;
- global initialization order; and
- the current persistent/transient collection boundary.

The lowering itself performs canonicalization but no profitability transform.

### 7.2 IR to Cranelift

Final lowering receives verified IR and:

- maps block parameters to Cranelift block parameters;
- maps internal unboxed values to machine integers;
- emits current tagged ABI values at external boundaries;
- emits the same runtime slow paths as direct codegen;
- lowers planned root slots to the existing shadow-stack frame ABI;
- preserves source-related diagnostics where Cranelift supports them; and
- emits portable Cranelift operations rather than target-specific assembly text.

The first equivalence milestone must support every construct accepted by the current
direct codegen path. An unsupported IR operation is a build error in opt-in mode and
must never silently fall back after partially applying transformations.

## 8. Verifier

The verifier runs:

1. after AST-to-IR lowering;
2. after every pass in tests and diagnostic builds; and
3. once after the complete pass pipeline in release builds.

It rejects:

- use before definition;
- duplicate value definitions;
- invalid block or global references;
- predecessor/parameter arity mismatch;
- incompatible representation facts at merges;
- blocks without terminators;
- instructions after a terminator;
- invalid dominance;
- unboxed values crossing tagged ABI boundaries;
- illegal movement across effect barriers;
- unknown runtime-call effects;
- heap-capable values live across a safepoint without an assigned root; and
- root slots read before initialization.

Verifier failure is a compiler diagnostic, never undefined behavior or a panic in a
user build.

## 9. Safe optimization profile

Pass order is deterministic and versioned. The initial `safe` profile is:

1. unreachable-block and empty-block elimination;
2. sparse conditional constant propagation;
3. checked constant folding;
4. local copy propagation;
5. common-subexpression elimination for pure operations;
6. dead-code elimination for effect-free operations;
7. branch simplification;
8. dominance-based redundant tag/guard elimination;
9. conservative loop-invariant code motion; and
10. liveness-based root planning.

### 9.1 Checked constant folding

Folding must use Clojure-native fixnum limits and current error behavior. It must not
turn a runtime overflow, invalid type, divide-by-zero, bounds error, or exception into a
different value or compile-time failure.

### 9.2 Common-subexpression elimination

CSE is limited to instructions proven pure and non-throwing. It must not merge:

- allocation sites whose identity can be observed;
- collection operations that may allocate;
- global or dynamic-var reads;
- I/O;
- indirect calls; or
- operations with different source-related failure behavior.

### 9.3 Guard elimination

A guard may be removed only when a dominating fact proves the same condition on every
incoming path and no intervening state change invalidates it. Unknown calls invalidate
facts conservatively.

### 9.4 Loop-invariant code motion

LICM initially moves only pure, non-throwing, non-allocating operations whose operands
dominate the loop header. It cannot move an operation across a `MaySafepoint`,
`MayThrow`, state, I/O, or global-initialization boundary.

### 9.5 Inlining and interprocedural passes

Inlining is not part of the first `safe` profile. It may be added later for small,
known, non-recursive functions after:

- call graph and code-size budgets are explicit;
- exception and source-span behavior is preserved;
- recursive and indirect calls remain unchanged; and
- the pass independently satisfies the Cormen gate.

The analyses from ADR-0010 may migrate into `clojure-ir`; their semantics must not be
silently widened during that migration.

## 10. GC and safepoint contract

The optimization IR adopts the ADR-0006 invariant:

> Before every safepoint, every heap-capable value live during or after that safepoint
> is stored in a root slot visible to the collector.

The IR keeps `MaySafepoint` explicit. Liveness runs after transformations and assigns
fixed, reusable root slots. Final lowering flushes dirty live roots immediately before
a safepoint and may clear dead roots before a later safepoint.

`GC:` a pass may not remove, duplicate, or move a safepoint until it proves that
allocation, exception, and root visibility remain equivalent.

`GC:` a value classified as immediate needs no root. `UnknownTagged` is treated as
heap-capable.

`GC:` calls through Clojure's direct or indirect ABI are safepoints unless a reviewed,
central declaration proves otherwise.

GC-stress equivalence is mandatory for transformed cases. A future moving or
asynchronous collector requires a new decision and verifier rules.

## 11. Correctness and determinism gates

Before performance is considered:

- every active conformance case passes with `--ir-opt none` and `safe`;
- every existing `xfail` still fails for its declared reason unless independently
  promoted;
- output, exit status, and deterministic diagnostics match;
- the complete Cormen suite produces the same 30 checksums;
- differential tests compare direct codegen, optimized IR, and the interpreter where
  the interpreter supports the program;
- randomized IR/pass tests preserve behavior;
- GC stress passes for allocation and loop sentinels;
- runtime ASan and UBSan gates remain green; and
- the same source and options produce byte-identical textual IR dumps.

Optimization may change object bytes and assembly shape. It may not change observable
program semantics.

## 12. Structural performance evidence

Each optimization pass has positive and negative structural tests. Depending on the
pass, tests assert:

- fewer runtime arithmetic calls in a hot loop;
- fewer tag guards dominated by an earlier guard;
- fewer root stores between safepoints;
- removal of unreachable blocks or unused pure instructions;
- unchanged calls that may throw, allocate, write state, or perform I/O; and
- bounded frame and code-size growth.

Tests inspect canonical optimization IR and Cranelift IR. Target-specific `objdump`
output may be recorded for investigation but is not a portable correctness contract.

## 13. Blocking Cormen non-regression gate

### 13.1 Comparison

The gate compares two native configurations built from the same commit and Rust/C
toolchain:

```text
Control:   --ir-opt none --opt-level none
Candidate: --ir-opt safe --opt-level none
```

Clojure/JVM values remain in the published comparison for context, but they do not
decide whether the IR regressed native execution. Historical JVM measurements must not
be rewritten by this A/B gate.

### 13.2 Measurement protocol

**Planned:** `benchmarks/cormen/compare-ir.sh`:

- builds the control and candidate compilers or modes once;
- runs all 30 cases at scale 25;
- validates the checksum before accepting a sample;
- performs at least seven paired repetitions;
- alternates control/candidate order per repetition;
- records host, kernel, CPU, affinity, governor when available, Rust version,
  Cranelift version, C compiler, commit, options, and timestamp;
- stores every raw sample rather than only an aggregate;
- reports median, median absolute deviation, paired ratio, and a 95% bootstrap
  confidence interval for wall and CPU time; and
- records compile time, maximum RSS, executable size, and code size as secondary
  metrics.

A run with throttling, checksum mismatch, non-zero status, missing case, or different
scale is invalid rather than a pass.

### 13.3 Promotion rules

A pass or combined `safe` pipeline is not promoted when any of these is true:

1. any Cormen checksum differs;
2. aggregate median wall time or aggregate median CPU time is greater than control;
3. any chapter has a confirmed wall-time or CPU-time regression;
4. any individual case has a confirmed regression;
5. the worst individual median point estimate exceeds control by more than 3%; or
6. the candidate fails correctness, GC, sanitizer, coverage, or deterministic-IR gates.

A confirmed regression means:

```text
candidate/control median ratio > 1.01
and the paired 95% confidence interval is entirely above 1.00
```

The 1% effect threshold prevents timer noise from being mislabeled as a regression; it
is not a regression budget. Aggregate time must still be non-increasing.

For the first complete IR release, the candidate must additionally improve aggregate
median wall time by at least 3%. If it only ties the control, the IR may remain an
internal experiment but is not advertised as an execution-time optimization.

Every pass is measured alone and in the complete pipeline. A combination that regresses
is rejected even when each constituent pass passed independently. A rejected pass stays
outside `safe` until changed and remeasured.

### 13.4 Permanent optional status

Passing this gate permits explicit use of `--ir-opt safe`; it does not permit changing
the default. This ADR intentionally keeps `none` as the default.

## 14. Test matrix

### Unit tests in `clojure-ir`

- builder IDs and deterministic ordering;
- CFG predecessor and successor construction;
- verifier success and every rejection category;
- representation-fact joins;
- effect joins and barriers;
- dominance, loop discovery, and liveness;
- printer normalization;
- each pass: recognition, conservative non-recognition, idempotence, and verifier
  preservation; and
- malformed IR property tests that never panic the verifier.

### Integration tests

- Analyzer AST to unoptimized IR golden cases;
- IR to Cranelift lowering for each instruction and terminator family;
- direct versus IR execution for arithmetic, branches, loops, closures, collections,
  exceptions, protocols, globals, I/O, and GC;
- invalid type, overflow, bounds, and arity slow paths;
- `CLJN_GC_STRESS=1` with heap values live across calls and allocations; and
- CLI default/explicit-option behavior.

### Broad gates

```bash
make quality
make coverage
make compatibility
make test-runtime-sanitize
make benchmarks-cormen
```

The dedicated Cormen A/B gate is additional; a single ordinary benchmark run does not
replace it.

## 15. Delivery sequence

### IR-0 — Freeze the baseline

- version control and environment metadata for the latest Cormen control;
- implement raw paired-sample format and non-regression analysis;
- add small CLIF and runtime-call structural sentinels.

Exit: the gate detects a deliberately slowed control program.

### IR-1 — Model, printer, and verifier

- create `clojure-ir`;
- implement identifiers, blocks, instructions, effects, facts, terminators, printer,
  and verifier;
- add unit and property tests.

Exit: hand-built valid and invalid functions are deterministically verified.

### IR-2 — Semantics-preserving optional lowering

- lower the complete supported Analyzer AST to IR;
- lower unoptimized IR to Cranelift;
- add `--ir-opt`;
- run differential, conformance, GC, and sanitizer gates with no optimization passes.

Exit: opt-in IR is semantically equivalent and default output still uses the direct
path.

### IR-3 — Local safe passes

- CFG simplification, SCCP, checked folding, copies, CSE, and DCE;
- pass-by-pass structural and Cormen gates.

Exit: only individually non-regressing passes enter `safe`.

### IR-4 — Guards, loops, and root liveness

- dominance-based facts and guard elimination;
- conservative LICM;
- liveness-based root plan and safepoint verification;
- GC-stress and sanitizer expansion.

Exit: hot numeric loops contain no redundant runtime or root traffic.

### IR-5 — Interprocedural migration

- move suitable ADR-0010 summaries and transforms behind the verified IR boundary;
- consider bounded inlining separately;
- retain conservative fallback on every unknown call or escape.

Exit: migrated transforms preserve their existing tests and pass the combined Cormen
gate.

### IR-6 — Publish the optional profile

- run seven or more complete paired Cormen repetitions;
- version raw samples and the generated report;
- document accepted passes, rejected passes, compile-time cost, binary-size cost, and
  observed execution improvement.

Exit: every acceptance criterion below is satisfied.

## 16. Acceptance criteria

The specification is implemented only when:

1. `--ir-opt` defaults to `none` in every build profile;
2. the current direct pipeline remains available and supported;
3. `clojure-ir` has deterministic model, printer, verifier, and pass APIs;
4. every currently compiled construct has an IR lowering and Cranelift lowering;
5. all active conformance cases pass in both modes;
6. direct, optimized, and interpreter-supported executions are observationally equal;
7. GC stress, ASan, and UBSan are green;
8. structural tests demonstrate fewer redundant operations in selected hot paths;
9. all 30 Cormen checksums match;
10. the Cormen gate reports zero aggregate, chapter, and confirmed per-case execution
    time regressions;
11. aggregate Cormen wall time improves by at least 3%;
12. raw samples, environment, commit, options, and statistics are versioned; and
13. documentation continues to describe the IR as optional rather than default.

## 17. Risks

| Risk | Mitigation |
| --- | --- |
| misoptimization changes semantics | conservative facts, effect barriers, verifier, differential tests |
| missing root causes use-after-free | explicit safepoints, post-pass liveness, GC stress, ASan |
| optimizer improves totals but harms a case | per-case and per-chapter blocking Cormen rules |
| noise hides a regression | paired repetitions, alternating order, confidence interval, raw samples |
| code size and spills repeat the Cranelift `speed` failure | structural frame/code-size reports and 3% worst-case point-estimate guard |
| dual lowering paths drift | shared semantic operation table, complete differential corpus, planned migration only after evidence |
| compile time grows excessively | record compile time per case and publish the cost; add a budget before any default proposal |
| IR leaks Cranelift details | crate dependency rule and backend-neutral public model |
| exception or I/O is reordered | orthogonal effect flags and verifier barriers |
| optimization becomes default accidentally | CLI tests in debug/release and superseding-ADR requirement |

