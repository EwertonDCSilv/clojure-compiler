# ADR-0014 — Add an optional compiler-owned optimization IR

- Status: **Accepted; implementation in progress**
- Date: 2026-07-28
- Specification: [Optional Optimization IR](../OPTIMIZATION_IR_SPEC.md)
- Related:
  [ADR-0001](0001-code-generation-backend.md),
  [ADR-0002](0002-memory-management.md),
  [ADR-0006](0006-codegen-optimization.md),
  [ADR-0009](0009-benchmark-performance-study.md),
  [ADR-0010](0010-interprocedural-ephemeral-vectors.md),
  [ADR-0011](0011-rust-crate-unit-testing-strategy.md), and
  [ADR-0012](0012-rust-crate-modularization.md)

## Implementation status

The decision has been adopted without changing the default pipeline. The first
implementation slice provides the backend-neutral `clojure-ir` crate, deterministic
IDs and printing, CFG verification, representation and effect facts, liveness-based
root planning, conservative local passes, the explicit `--ir-opt none|safe` switch,
dual-mode conformance execution, and the paired Cormen A/B harness.

The `safe` spelling is an opt-in performance profile. Its adapter lowers verified pure
scalar islands, materializes proven constants, propagates fixnum representations
through loops and non-escaping direct calls, and removes redundant numeric type guards
before the existing direct Cranelift lowering. Overflow, divide-by-zero, dynamic-type
slow paths, and evaluation order remain intact. Complete Analyzer AST CFG lowering,
IR-to-Cranelift lowering, LICM, optimized root emission, and deterministic CLI dumps
remain open. Consequently, the acceptance checklist at the end of this ADR is not yet
complete and `none` remains the default.

The current complete harness run used all 30 Cormen cases, scale 25, and seven paired
alternating repetitions. It preserved every checksum and **passed** the blocking
performance gate: candidate/control was 0.9568 for aggregate wall time and 0.9565 for
aggregate CPU, or median improvements of 4.32% and 4.35%. Raw samples and the
statistical report are versioned under
[`benchmarks/cormen/results/`](../../../benchmarks/cormen/results/ir-ab-report.md).
These measurements qualify the current explicit profile without enabling it by
default or claiming that the whole-function architecture is complete.

## Context

The implemented native pipeline lowers the Analyzer AST directly to Cranelift IR.
Several useful transformations already exist in the analyzer or codegen:

- guarded fixnum fast paths;
- fixed shadow-stack frames and direct root stores;
- transient construction for proven linear vector accumulators;
- an initial interprocedural linear-parameter summary; and
- permanent caching of immediate vector literals.

These changes improved important workloads, but the current boundary makes broader
optimization increasingly difficult:

- structured AST nesting obscures a canonical control-flow graph;
- value facts, call effects, liveness, and root state are maintained close to backend
  emission;
- transformations can duplicate analysis or depend on Cranelift instruction shape;
- Cranelift cannot recover Clojure-specific facts after values become generic machine
  operations and runtime calls; and
- exception order, slow paths, dynamic calls, and GC safepoints require stronger
  barriers than a conventional pure arithmetic optimizer.

Enabling Cranelift `speed` does not solve this boundary. ADR-0006 recorded a Cormen
regression: `speed` was slower in 25 of 30 cases and increased KMP frame size and spills.
The project therefore needs to simplify the frontend program before Cranelift register
allocation and machine lowering.

The new layer must not silently change the stable path. It must also not declare success
from instruction counts alone: the optimization exists to reduce execution time, and
the Cormen suite is the blocking performance contract.

## Decision drivers

- Preserve dynamic Clojure behavior and exact runtime slow paths.
- Keep Cranelift as the primary native backend.
- Make effects, safepoints, value facts, control flow, and liveness explicit.
- Allow safe compiler-owned optimization without target-specific assembly rewriting.
- Keep the feature reversible and disabled by default.
- Detect incorrect optimization structurally and semantically before benchmarking.
- Prevent aggregate gains from hiding regressions in a Cormen case or chapter.
- Keep GC correctness reviewable across Rust IR and C runtime boundaries.

## Decision

Create a backend-neutral `clojure-ir` crate containing a verified, CFG-based
optimization IR. The Analyzer AST lowers to this IR only when the user explicitly
selects the optional pipeline. Verified IR then lowers to Cranelift IR and the existing
object/runtime link path.

The public selection is:

```text
--ir-opt none|safe
```

`none` remains the default in debug and release. It keeps the current direct lowering.
`safe` enables only IR passes that have satisfied correctness, GC, and Cormen
non-regression gates. The existing `--opt-level` remains an independent Cranelift
setting.

This ADR does not permit making IR optimization the default. A future default change
requires a superseding ADR.

### IR shape

The IR uses:

- explicit functions and basic blocks;
- block parameters for merged values;
- single-definition `ValueId`s;
- explicit branch, conditional branch, return, throw, and unreachable terminators;
- conservative representation facts for tagged, fixnum, immediate, heap, and callable
  values;
- orthogonal effects for state, allocation, safepoints, throws, and I/O; and
- source spans on operations whose result or failure is observable.

The representation is not Cranelift IR and does not expose Cranelift types in its public
model.

### Verification

A verifier runs before optimization, after passes in test/diagnostic builds, and after
the complete release pipeline. It checks CFG integrity, dominance, definitions, block
arguments, representations, effects, tagged ABI boundaries, and GC-root liveness at
safepoints.

Verifier failure produces a compiler diagnostic. It is not treated as unreachable
behavior in a user build.

### Initial safe passes

The first profile is intentionally conservative:

1. CFG simplification;
2. sparse conditional constant propagation;
3. checked constant folding;
4. copy propagation;
5. CSE for pure, non-throwing operations;
6. effect-aware dead-code elimination;
7. branch simplification;
8. dominance-based redundant guard elimination;
9. LICM for pure, non-throwing, non-allocating instructions; and
10. liveness-based root planning.

Inlining and broader interprocedural optimization are excluded initially. They may join
`safe` only after receiving their own budgets, tests, and Cormen evidence.

### Dynamic semantics and slow paths

The IR may unbox and specialize values internally, but all public function boundaries
retain the tagged `Value` ABI. Unknown types, failed tag guards, overflow, bounds,
invalid arity, and unsupported specialization branch to the existing runtime behavior.

Passes cannot reorder or remove operations that may throw, allocate, write state,
perform I/O, or reach a safepoint unless equivalence is explicitly proven.

### GC contract

The IR implements the ADR-0006 safepoint invariant:

> Every heap-capable value live during or after a safepoint is visible in a fixed
> shadow-stack root slot before that safepoint.

Liveness and root planning run after transformations. `UnknownTagged` is conservatively
heap-capable. Clojure direct and indirect calls are safepoints unless a centralized,
reviewed effect declaration proves otherwise.

No part of this decision assumes a moving or asynchronous collector.

## Cormen performance decision

The promotion comparison is native against native on the same commit:

```text
control   = --ir-opt none --opt-level none
candidate = --ir-opt safe --opt-level none
```

Clojure/JVM remains a contextual reference, not the IR regression control.

The planned gate runs all 30 Cormen cases at scale 25 for at least seven paired,
alternating repetitions. It stores raw samples and reports medians, median absolute
deviation, paired ratios, and 95% bootstrap confidence intervals.

A pass or combined pipeline is rejected when:

- a checksum changes;
- aggregate wall or CPU time increases;
- any chapter has a confirmed wall or CPU regression;
- any case has a confirmed wall or CPU regression;
- any case's median point estimate exceeds control by more than 3%; or
- correctness, conformance, GC, sanitizer, coverage, or deterministic-IR gates fail.

A confirmed regression requires both a candidate/control median above `1.01` and a
paired 95% confidence interval entirely above `1.00`. This threshold filters timer
noise; it is not permission to consume 1% performance.

The first complete `safe` profile must also improve aggregate median Cormen wall time by
at least 3%. A tie may remain an internal experiment but is not published as an
execution-time optimization.

Every pass is measured alone and in combination. A combined regression blocks the
combination even when isolated passes passed.

## Alternatives considered

| Alternative | Advantages | Disadvantages | Decision |
| --- | --- | --- | --- |
| rely on Cranelift `speed` | no new IR | already regressed Cormen; lacks Clojure and GC facts | rejected |
| continue optimizing the Analyzer AST | small initial change | no canonical CFG; facts, effects, and liveness remain scattered | rejected as the general optimization boundary |
| manipulate Cranelift IR directly | close to machine code | domain semantics and slow-path identity are already obscured | rejected for compiler-owned semantic passes |
| rewrite emitted assembly | direct instruction control | target-specific, fragile, after register allocation, unsafe around ABI and GC | rejected |
| switch to LLVM | mature optimizer | larger backend/toolchain decision; does not remove the need for correct Clojure effects and rooting | rejected for this scope |
| make the new IR always-on with optional passes | one lowering path | changes the default compiler before equivalence and performance are proven | rejected |
| optional verified IR with retained direct path | reversible, measurable, domain-aware | temporarily maintains two lowering paths | **chosen** |

## Consequences

### Positive

- Clojure-specific facts survive until optimization.
- Effects and safepoints become explicit contracts instead of backend conventions.
- Passes can be unit-tested independently of Cranelift and the host architecture.
- Cranelift receives simpler control flow with fewer guards, calls, loads, and stores.
- The direct path remains available for diagnosis and differential testing.
- Cormen regressions are blocking at case, chapter, and aggregate levels.

### Negative

- The compiler temporarily owns two lowering paths.
- `clojure-ir` adds a substantial verified data model and analysis surface.
- Compile time and memory use may increase in opt-in mode.
- Incorrect effects or liveness can cause semantic errors or GC corruption.
- Strict per-case gates may reject optimizations that help most programs.

The final cost is intentional: an optimization profile must be predictably safe for the
versioned workload, not merely faster on average.

## Delivery constraints

- Follow Red–Green–Refactor for every IR operation and pass.
- A pass requires a positive case, a similar non-transforming case, idempotence where
  applicable, semantic equivalence, and verifier preservation.
- Default-mode CLI tests must precede adding the option.
- Active conformance cases run in both modes.
- GC-sensitive changes require GC stress and runtime sanitizers.
- Benchmark scripts record raw samples and never replace preserved JVM measurements
  when running the native A/B gate.
- Rejected performance results are versioned or reported; assertions are not weakened
  to admit the pass.

## Acceptance

This decision is implemented only when:

1. `--ir-opt none` is the stable default and the direct path remains supported;
2. the IR model, deterministic printer, verifier, and analyses have unit/property tests;
3. every currently supported Analyzer AST construct lowers through the optional path;
4. IR and direct modes are observationally equivalent across active conformance;
5. exception, overflow, invalid-type, bounds, and arity behavior remains equivalent;
6. GC stress, ASan, and UBSan pass with the optimized path;
7. structural tests show removal of redundant hot-path work;
8. all 30 Cormen checksums match;
9. the complete profile has zero aggregate, chapter, or confirmed per-case Cormen
   execution-time regressions;
10. aggregate median Cormen wall time improves by at least 3%;
11. raw measurements and environment metadata are versioned; and
12. documentation states that the IR is optional and not the default.

## Relationship to previous decisions

- ADR-0001 remains in force: Cranelift is the primary backend.
- ADR-0002 remains in force: the collector is precise, non-moving, and single-threaded.
- ADR-0006 supplies fixnum fast paths, effects, and the safepoint-rooting invariant; this
  ADR gives those analyses a stable home.
- ADR-0009 supplies the performance diagnosis and makes Cormen the blocking study.
- ADR-0010 transforms may migrate behind the IR verifier without widening their
  uniqueness proof.
- ADR-0011 defines pass-level unit, negative, integration, and coverage expectations.
- ADR-0012 guides splitting the new crate and preventing another monolithic codegen
  file.

Threads, a moving collector, deoptimization, assembly rewriting, changing the default,
or replacing Cranelift require separate decisions.
