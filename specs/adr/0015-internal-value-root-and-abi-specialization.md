# ADR-0015 — Specialize internal values, GC roots, and call boundaries

- Status: **Accepted; experimental implementation not admitted to `safe`**
- Date: 2026-07-28
- Related:
  [ADR-0002](0002-memory-management.md),
  [ADR-0003](0003-value-representation.md),
  [ADR-0006](0006-codegen-optimization.md),
  [ADR-0009](0009-benchmark-performance-study.md),
  [ADR-0010](0010-interprocedural-ephemeral-vectors.md),
  [ADR-0011](0011-rust-crate-unit-testing-strategy.md),
  [ADR-0014](0014-optional-optimization-ir.md), and
  [Optional Optimization IR Specification](../OPTIMIZATION_IR_SPEC.md)

## Implementation status

The first isolated bundle is implemented behind:

```text
--ir-opt safe --ir-experiment adr15
```

It currently delivers:

- conservative compact root frames that omit slots for proven immediate locals and
  omit the frame when no local root is required;
- raw fixnum storage for proven local, `let`, `loop`, and `recur` slots;
- checked raw fixnum arithmetic islands with the original tagged C runtime slow paths;
- compiler-private fixed-arity raw-fixnum entries for proven non-escaping direct
  targets, while retaining the generic `(self, argc, argv)` entry;
- direct specialized calls without generic `argc`, `argv`, or argument spills;
- aggregate deterministic structural JSON through `--ir-stats PATH`;
- candidate forwarding through conformance and benchmark runners; and
- configurable paired Cormen comparison between the admitted `safe` control and the
  ADR-0015 candidate.

The generic boundary remains conservative after differential testing found that
consuming interprocedural parameter facts there was unsound for a polymorphic
application case. Specialized facts are consumed only by the private entry that owns
the complete fixed representation contract.

The first complete seven-pair, scale-25 Cormen run preserved every checksum but was
**not promoted**. Candidate/control was 1.0066 for aggregate wall time and 1.0054 for
CPU: it regressed rather than achieving the required 3% gain, and several per-case
median point estimates exceeded the 3% ceiling. The bundle therefore remains opt-in
and does not alter the admitted `safe` profile or the default `none` profile.

Full safepoint liveness/dirty-root materialization, per-function statistics, complete
runtime-effect descriptors, intrinsic admission, sanitizers, and a passing blocking
Cormen result remain required before the bundle can be admitted to `safe`.

## Context

The current native runtime deliberately uses one tagged machine word as the public
`Value` representation. This gives the compiler and the C runtime a uniform boundary
for dynamic values, collections, closures, errors, and future numeric extensions.
ADR-0002 also requires a precise shadow stack because the collector does not scan the
native stack.

Those choices are sound at observable boundaries, but they do not have to constrain
every internal operation. Hot code still pays for some combination of:

- repeated tag tests, untagging, and retagging around integer operations;
- tagged locals even when every incoming value is proven to have one representation;
- eager shadow-stack stores for values that are not live at a safepoint;
- root frames in functions that cannot allocate or call allocating code;
- argument spills into a generic `(self, argc, argv)` calling convention;
- direct Clojure calls through the generic callable ABI even when the target and arity
  are statically known; and
- C ABI calls whose operation could be represented directly in compiler IR.

The implemented `safe` IR profile has already proved that representation facts are
valuable. It propagates fixnum facts through bindings, loops, and selected direct
calls, removes redundant numeric guards, and passes the blocking Cormen gate with
candidate/control ratios of 0.9568 for wall time and 0.9565 for CPU. The next
optimization layer must use those facts to change internal storage and call shape,
instead of only simplifying operations over still-tagged values.

This ADR refines ADR-0006 and ADR-0014. It does not replace the collector, remove the
generic runtime ABI, make optimization the default, or introduce a new language
feature.

## Decision drivers

- Reduce work executed per iteration, not merely source-level runtime helper names.
- Preserve the tagged `Value` ABI wherever code may be called dynamically.
- Preserve precise GC rooting across allocation, calls, exceptions, and slow paths.
- Keep invalid types, overflow, bounds, arity, and dynamic behavior observationally
  equivalent to the current runtime.
- Make every removed guard, root store, and ABI conversion depend on a verifier-visible
  proof.
- Avoid requiring LLVM, LTO, or target-specific assembly for correctness.
- Measure each transformation against the already admitted `--ir-opt safe` profile.
- Reject aggregate wins that hide confirmed Cormen regressions.

## Decision

Adopt four coordinated compiler transformations behind the optional IR pipeline:

1. **internal unboxed representations for proven values;**
2. **virtual GC roots materialized only at safepoints;**
3. **specialized internal calling conventions for statically closed calls; and**
4. **a classified runtime ABI whose eligible leaf operations become IR intrinsics.**

The generic tagged representation, generic Clojure callable ABI, C runtime slow paths,
and existing `--ir-opt none` path remain available. A transformation enters `safe`
only after satisfying the gates in this ADR.

### Internal representation contract

The optimization IR will distinguish at least:

```text
TaggedValue       externally observable tagged Value
FixnumRaw         signed i64 known to be within the fixnum range
BooleanRaw        backend boolean
CharRaw           validated Unicode scalar value
HeapTagged        tagged reference to a managed heap object
UnknownTagged     any valid tagged Value
```

`FloatRaw` is deferred until boxed-float identity, allocation, NaN, equality, and error
behavior have dedicated tests. Collections, globals, captures, dynamic calls, generic
returns, and runtime operations that consume `Value` continue to use tagged values.

Conversions are explicit IR operations:

```text
guard-fixnum  TaggedValue -> FixnumRaw or slow edge
tag-fixnum    FixnumRaw -> TaggedValue
box           internal representation -> TaggedValue
unbox         TaggedValue -> internal representation or slow edge
```

They carry effects and source information when failure is observable. Optimization may
remove or move a conversion only when dominance, representation, and exception-order
checks prove that the result is unchanged.

Block parameters and loop-carried values may remain unboxed when every incoming edge
has the same proven representation. A conflicting edge forces a tagged join. The
initial implementation is proof-driven and does not require speculative deoptimization:
unknown input remains on the generic tagged path.

Overflow checks stay in the unboxed arithmetic path. An overflow or failed entry guard
uses the same error or generic slow behavior as the direct pipeline; it must not wrap
silently or manufacture a different numeric type.

### Virtual root state

The IR root plan replaces eager root synchronization with a logical root state:

```text
RootState = live heap-capable values + assigned slots + dirty/clean state
```

Normal evaluation updates SSA values and logical liveness. It does not automatically
write the shadow stack. Immediately before a safepoint, lowering must:

1. identify every heap-capable value live during or after the safepoint;
2. assign or reuse a fixed root slot;
3. store only values whose slot is dirty or newly assigned; and
4. make the frame extent visible to the collector before the call can allocate.

Values proven `FixnumRaw`, `BooleanRaw`, or `CharRaw` never consume root slots. Dead
roots may be cleared lazily before a later safepoint. A function with no safepoint and
no requirement to expose heap values may omit `cljn_gc_enter` and `cljn_gc_leave`
entirely.

The verifier checks root coverage after all transformations and before backend
lowering. `UnknownTagged` is always treated as heap-capable. If liveness, dominance, or
effects are uncertain, the value is rooted conservatively.

Generic argument arrays remain roots in the caller for the duration of a call. The
callee must establish any root frame needed by its parameters before its first
safepoint. Direct specialized calls follow the same liveness rule without requiring a
contiguous tagged `argv`.

The current collector is non-moving and single-threaded. A moving or asynchronous
collector invalidates the assumption that unrooted register values remain stable
between safepoints and requires a new decision.

### Specialized internal function ABI

A fixed-arity function or method may receive an internal specialized entry point when:

- its target and arity are statically known;
- the function value does not escape through `FnRef`, a collection, a global, a
  capture, `apply`, or an indirect call;
- every specialized parameter and return representation is proven at all participating
  call sites;
- recursion and every backedge preserve the same facts; and
- the function is not a protocol, multimethod, variadic dispatch target, or another
  dynamically replaceable boundary.

For example, a proven numeric helper may use:

```text
generic entry:     (self: Value, argc: i64, argv: *Value) -> Value
internal entry:    (x: FixnumRaw, y: FixnumRaw) -> FixnumRaw
```

The internal entry avoids `argc`, `argv`, argument root spills, tag conversions, and
generic arity dispatch. If a function can also be observed dynamically, the generic
entry remains as a wrapper or independent generic body. The compiler must not expose
the specialized symbol as the function object's callable address.

Specialization is bounded:

- at most one internal representation signature per method in the first phase;
- no clone when the estimated body growth exceeds the configured budget;
- recursive strongly connected components specialize as one unit or not at all; and
- code-size accounting runs before admission.

### Runtime ABI classification

Every imported runtime function receives one centralized descriptor. Effects are
orthogonal because one call may allocate, throw, mutate state, and safepoint:

```text
RuntimeEffects  = bitset {
  MayThrow, MayAllocate, MaySafepoint, ReadsState, WritesState, PerformsIo
}
RuntimeLowering = Intrinsic | CAbi
RuntimeResult   = representation fact
```

The table is shared by effect analysis, root planning, and code generation. A C runtime
change that adds allocation, collection, mutation, throwing, or I/O must update the
descriptor in the same commit. Rust tests and the C ABI harness verify the symbol and
effect contract.

Only small operations with frozen semantics, an effect set compatible with direct
lowering, and complete differential tests may become IR intrinsics. Allocation,
collection construction, I/O, exception machinery, dynamic dispatch, and complex
structural equality remain behind the C ABI unless a later decision supplies an
equivalent implementation and gate.

An intrinsic is not a handwritten assembly peephole. It is a semantic IR operation
that may lower through Cranelift or a future backend. LLVM or LTO may later optimize
the same simpler boundary, but neither is required by this decision.

### Exceptions and observable ordering

Calls that may throw or safepoint are barriers unless a pass proves otherwise. Before
such a call, the root state must describe every value required by normal return,
catch, finally, or unwind cleanup.

Root-frame elision must preserve the saved GC stack position used by native exception
handlers. Slow paths retain the original source operation and evaluation order.
Specialized calls must report the same process error or catchable value as their
generic equivalent.

## Measurement and admission

### Reproducible pass isolation

Benchmark tooling will support a diagnostic pass manifest in addition to the stable
`--ir-opt none|safe` selection. The manifest is recorded with raw results and allows
one candidate pass or bundle to be enabled or disabled on the same compiler commit.
It is not a new stable language or runtime option.

For a pass under evaluation:

```text
control   = --ir-opt safe + currently admitted pass manifest
candidate = control + exactly the candidate pass or bundle
```

After admission, the previous manifest remains reproducible in the benchmark harness.
The complete candidate is also checked against `--ir-opt none`, but a gain over `none`
cannot hide a regression against the current `safe` control.

### Structural metrics

Add a deterministic optimization-statistics report, per function and aggregate, with:

- tag guards and tag/untag conversions;
- box/unbox operations;
- root-frame slots, root stores, clears, and frame enter/leave calls;
- safepoints and values live at each safepoint;
- generic `argv` spills;
- direct generic, direct specialized, indirect, and C ABI calls;
- C ABI calls grouped by symbol; and
- generated function and object-code size.

Sentinel gates require:

1. a proven allocation-free numeric loop has zero root stores, zero C ABI calls, and
   zero tag conversions in its repeated block;
2. a proven non-escaping numeric helper uses the specialized direct ABI without
   `argc`/`argv`;
3. a heap value live across allocation appears in a root slot before the safepoint;
4. a similar dead heap value does not consume a root store;
5. an escaping, variadic, protocol, multimethod, or indirect target retains the generic
   ABI; and
6. slow paths remain present for unknown types, overflow, bounds, and arity errors.

Instruction counts alone do not promote a pass; they diagnose whether the intended
mechanism actually occurred.

### Correctness and GC gates

Every candidate bundle must pass:

- IR unit tests, verifier tests, negative non-transforming cases, and idempotence;
- direct-versus-candidate differential tests for stdout, stderr, exit status, and
  expected filesystem effects;
- all active conformance cases in `none`, admitted `safe`, and candidate modes;
- arithmetic boundaries, mixed float/fixnum behavior, invalid types, bounds, arities,
  exceptions, `try/finally`, closures, recursion, and higher-order calls;
- `CLJN_GC_STRESS=1` with live locals, captures, arguments, return values, loop-carried
  values, and exception edges across allocations;
- runtime C unit and ABI tests;
- ASan and UBSan;
- the repository coverage gate without lowering thresholds; and
- deterministic IR, optimization statistics, and object import snapshots.

Any mismatch or sanitizer failure rejects the candidate independently of performance.

### Blocking performance gate

The performance control is the current admitted `safe` profile on the same commit.
The gate uses all 30 Cormen cases at scale 25, with at least seven paired repetitions
and alternating order. It versions raw samples, environment metadata, pass manifests,
checksums, medians, MAD, paired ratios, and deterministic 95% bootstrap intervals.

A pass or bundle is rejected when:

1. any checksum changes;
2. aggregate median wall time or CPU time exceeds control;
3. a chapter has a confirmed wall-time or CPU-time regression;
4. a case has a confirmed wall-time or CPU-time regression;
5. the worst case median point estimate exceeds control by more than 3%;
6. aggregate median peak RSS exceeds control by more than 3%, or a case exceeds it by
   more than 10% after a confirmation run;
7. median compiler wall time increases by more than 10%;
8. median native executable size increases by more than 5% without a separately
   accepted performance/code-size trade-off; or
9. any correctness, GC, sanitizer, coverage, or structural gate fails.

A confirmed execution regression uses the ADR-0014 rule: candidate/control median
above 1.01 and a paired 95% interval entirely above 1.00.

An individual enabling pass may remain a candidate when it achieves a required
structural reduction without a statistically confirmed execution gain. It does not
enter `safe` by itself. The first combined ADR-0015 bundle admitted to `safe` must
improve aggregate median Cormen wall time by at least 3% against the previously
admitted `safe` profile while satisfying every local non-regression rule.

Cracking and Exercism remain secondary validation suites. Clojure/JVM measurements are
contextual and are not rewritten by this native-to-native gate.

## Delivery sequence

### Phase 0 — Instrumentation

- Add deterministic optimization statistics and pass manifests.
- Add structural sentinels for roots, tags, generic calls, and C ABI calls.
- Record a clean `safe` control before changing lowering.

Exit: reports are stable and a deliberately inserted root store/call regression is
detected.

### Phase 1 — Root virtualization

- Consume IR liveness and root plans in Cranelift lowering.
- Synchronize dirty roots only immediately before safepoints.
- Omit root frames from verified leaf functions.
- Preserve generic argument rooting and exception unwind state.

Exit: GC stress and sanitizers pass; allocation-free loop sentinels contain no repeated
root traffic; Cormen has no regression.

### Phase 2 — Unboxed regions

- Keep proven fixnum and boolean block parameters and loop slots unboxed.
- Insert explicit boxing only at tagged boundaries.
- Coalesce dominated guards and conversions.
- Preserve overflow and generic slow edges.

Exit: repeated blocks contain no tag conversions; differential numeric and error tests
pass; Cormen has no regression.

### Phase 3 — Specialized direct ABI

- Identify non-escaping fixed-arity methods.
- Build bounded representation signatures and specialized entries.
- Lower direct calls without `argc`/`argv`.
- Keep or synthesize generic wrappers whenever dynamic observation remains possible.

Exit: structural direct-call sentinels pass, higher-order/variadic behavior is unchanged,
and code-size/performance budgets pass.

### Phase 4 — Runtime boundary reduction

- Centralize runtime effect descriptors.
- Promote only fully tested leaf operations to IR intrinsics.
- Share slow blocks where dominance and source-order rules allow it.

Exit: ABI effect tests prevent an allocating helper from being marked leaf; the
combined Cormen gate improves wall time by at least 3% over the previous `safe`.

### Phase 5 — Admission

- Run all gates on a clean commit.
- Version raw results, manifests, environment, and structural reports.
- Add only the passing bundle to `safe`.
- Keep `none` as the default and retain diagnostic rollback.

## Alternatives considered

| Alternative | Advantages | Disadvantages | Decision |
| --- | --- | --- | --- |
| keep tagged values and eager roots everywhere | simplest lowering | preserves the dominant hot-loop costs | rejected |
| remove the shadow stack and conservatively scan the native stack | fewer explicit stores | violates precise-GC decision; backend/platform dependent | rejected |
| immediately adopt backend stack maps | precise roots without explicit frame stores | moving-GC and backend integration are not yet validated | deferred |
| expose unboxed values through the public callable ABI | smallest direct call path | breaks dynamic calls, closures, FFI, and runtime compatibility | rejected |
| duplicate the complete C runtime in Rust/IR | maximum optimizer visibility | semantic drift and maintenance cost | rejected |
| rely on C LTO or LLVM alone | mature interprocedural optimization | cannot infer Clojure value facts or precise roots across opaque calls | insufficient alone |
| verified internal specialization with generic boundaries | removes repeated work while preserving dynamic behavior | adds analysis, wrappers, and verifier obligations | **chosen** |

## Consequences

### Positive

- Proven loops can operate on machine integers without per-iteration tagging or root
  traffic.
- Direct closed calls can avoid generic arity dispatch and argument arrays.
- The collector sees fewer, more accurate root stores without losing precision.
- Runtime effects become one reviewable contract instead of scattered assumptions.
- A future LLVM backend receives cleaner IR and fewer opaque boundaries.

### Negative

- The compiler owns multiple internal representations and conversion points.
- Root-state verification becomes a critical correctness boundary.
- Specialized entries can increase code size and instruction-cache pressure.
- Generic wrappers and recursive components complicate symbol and call-graph handling.
- Incorrect runtime effect metadata can cause premature collection or reordered errors.

### Mitigations

- Conservative joins and fallback to `UnknownTagged`.
- No speculative unboxing in the first implementation.
- Central effect table with C ABI tests.
- Hard clone and code-size budgets.
- Structural snapshots plus GC stress and sanitizers.
- Pass-by-pass and combined Cormen gates against the current `safe`, not historical
  controls.
- Immediate rollback by removing a pass from the admitted manifest.

## Acceptance

This ADR is implemented only when:

1. the generic tagged `Value` and callable ABIs remain valid at every dynamic boundary;
2. root liveness and safepoint coverage are verifier-enforced;
3. proven allocation-free loops contain no repeated root store, C ABI call, or tag
   conversion;
4. eligible direct calls avoid `argc`/`argv` through a bounded specialized ABI;
5. unknown, escaping, variadic, protocol, multimethod, and higher-order calls preserve
   generic behavior;
6. overflow, invalid types, bounds, arity, exceptions, and evaluation order remain
   equivalent;
7. conformance, GC stress, ASan, UBSan, coverage, deterministic-report, and structural
   gates pass;
8. all 30 Cormen checksums remain equal;
9. no aggregate, chapter, or confirmed per-case Cormen execution regression exists;
10. memory, compiler-time, and code-size budgets pass;
11. the combined bundle improves aggregate Cormen wall time by at least 3% against the
    previously admitted `safe`; and
12. `--ir-opt none` remains the default.

## Relationship to previous decisions

- ADR-0002 remains in force: the collector stays precise, non-moving, and
  single-threaded with explicit roots.
- ADR-0003 remains in force at public boundaries; this ADR introduces internal
  representations rather than changing the external `Value`.
- ADR-0006 remains in force and supplies guarded arithmetic and the safepoint invariant;
  this ADR consumes the IR to eliminate remaining conversions, root traffic, and
  generic calls.
- ADR-0014 remains in force: all transformations live behind the optional verified IR
  and use its Cormen promotion policy.
- This ADR does not select LLVM. It makes the IR suitable for either Cranelift or a
  separately decided backend by reducing opaque runtime work first.
