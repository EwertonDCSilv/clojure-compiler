# Compiler Internal Documentation Plan

## Purpose

Document the complete compiler workspace in technical English. The project follows
the GCC practice of keeping interfaces and internal structures complete and
current, and of separating documentation changes from functional changes:

- [GCC Coding Conventions](https://gcc.gnu.org/codingconventions.html)
- [GCC Internals](https://gcc.gnu.org/onlinedocs/gccint/)

The scope includes all ten Rust crates, the native C runtime,
`crates/clojure-native-cli/src/core_compiled.clj`, the command-line interface, conformance support,
generators, and tests. This initiative does not change behavior, APIs, names, or
architecture.

## Documentation standard

- Add `//!` documentation to every crate, module, and executable. Describe its
  responsibility, inputs, outputs, boundaries, and place in the compilation
  pipeline.
- Document every public Rust item with `///`, including fields and enum variants.
  Add `Errors`, `Panics`, `Safety`, and `Examples` sections when they describe a
  real contract.
- Document non-trivial private functions when they encode an algorithm,
  invariant, state transition, ownership rule, garbage-collector contract, ABI
  constraint, source-span transformation, recursive process, complexity
  property, or external side effect. Do not annotate self-explanatory helpers.
- Give every C fragment a subsystem header. Document every `cljn_*` ABI function
  and every non-trivial static function, including the `Value` contract, rooting,
  allocation, errors or `longjmp`, ownership, and complexity where relevant.
- Use `INVARIANT:`, `SAFETY:`, `ABI:`, and `GC:` consistently for critical
  contracts.
- Add docstrings to all 26 functions in
  `crates/clojure-native-cli/src/core_compiled.clj`, recording
  semantics, eager or lazy behavior, limits, and deliberate differences from
  Clojure/JVM.
- Document only implemented behavior. Future capability must be labeled
  `Planned` and linked to the applicable specification or ADR.
- Keep user-facing messages and diagnostics in Portuguese. Only technical
  documentation and source comments are standardized in English.

Detailed templates and review criteria live in
[`DOCUMENTATION_STYLE.md`](DOCUMENTATION_STYLE.md).

## Delivery stages

### 1. Policy and baseline

- Save this plan and create `specs/DOCUMENTATION_STYLE.md`.
- Inventory module documentation, public APIs, algorithms, invariants, examples,
  and architectural references by component.
- Record the baseline for `cargo doc`, tests, conformance, and local changes.
  Never absorb unrelated local edits into documentation commits.

### 2. Fundamental types, frontend, and bootstrap

- Document `clojure-span`, `clojure-diagnostics`, `clojure-syntax`, and
  `clojure-reader`, emphasizing UTF-8 offsets, spans, diagnostic codes, metadata,
  reader macros, and returned forms.
- Document `clojure-value` and `clojure-interp`, explicitly separating bootstrap
  values from the native ABI.
- Add executable examples for pure parsing, span, form, and diagnostic APIs.

### 3. Analyzer and backend

- Document the AST, lexical resolution, captures, arities, `recur`, expansion,
  dispatch, and automatic transients.
- Document Cranelift lowering, the `(self, argc, argv)` convention, value
  classification, GC frames, rooting, safepoints, constants, and fast and slow
  paths.
- Add cross-component comments wherever Rust constants and layouts must match the
  C runtime.

### 4. Native runtime

- Document `runtime.c`, `runtime_all.c`, and every runtime fragment by subsystem:
  values, GC, functions, collections, records, transients, exceptions,
  multimethods, I/O, and the runtime reader.
- Specify amalgamation order, tags, layouts, permanent roots, allocation rules,
  buffer ownership, fatal conditions, and asymptotic costs.
- Preserve code and ABI exactly. Record discovered inconsistencies as separate
  issues or tasks; do not correct them as part of this initiative.

### 5. Orchestration and infrastructure

- Document the CLI, `crates/clojure-native-cli/src/core_compiled.clj`,
  `clojure-test-support`, fixture
  schema, isolated execution, comparisons, checksums, oracle, and reports.
- Document generators and tests by intent and contract without adding repetitive
  comments to individual tests.
- Update `docs/architecture.md` and `specs/COMPILER_PIPELINE.md` only where stale,
  and use them as indexes into rustdoc and ABI comments.

## Documentation gate

Create `scripts/check-docs.sh` and the `make docs-check` target. The gate:

- builds workspace documentation without dependencies, including private items;
- treats missing public documentation and broken intra-doc links as errors;
- runs all workspace doctests;
- verifies structurally that Rust modules have module documentation, each C
  runtime fragment has a subsystem header, every C ABI function named `cljn_*`
  has a contract comment, and every `defn` in
  `crates/clojure-native-cli/src/core_compiled.clj` has a
  docstring.

`docs-check` is part of `make quality`, so the existing CI quality job enforces it
without introducing a separate job. The gate deliberately does not impose
mechanical coverage on trivial private helpers; internal invariants remain a
mandatory review criterion.

## Validation and acceptance

For each stage, run:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
make docs-check
```

For runtime work, additionally run `make test-runtime` and
`make test-runtime-sanitize`. Final validation runs `make quality`,
`make compatibility`, and `make docs-check`.

If conformance starts red because of unexpected passes, acceptance requires the
same baseline set and no new failing `active` case. Comments must not change an
object file, executable, checksum, or benchmark result.

Final acceptance requires complete public documentation, a green documentation
gate, valid examples, a subsystem overview for every major component, and
explicit contracts and invariants on critical functions.

## Delivery organization

Produce independent commits for:

1. policy and documentation gate;
2. frontend;
3. bootstrap;
4. analyzer;
5. code generation and runtime;
6. orchestration and infrastructure.

Each commit must be documentation-only and independently validatable. Generated
files under `target/doc` are never versioned.

## Component inventory

This inventory is the review map for the completed initiative. “Examples” means
executable doctests where a pure API is suitable; executable and integration
components instead point to their contract tests.

| Component | Module/API documentation | Algorithms and invariants | Examples or architectural index |
| --- | --- | --- | --- |
| `clojure-span` | sources, byte spans, display locations | UTF-8 byte/column conversion and range bounds | rustdoc examples; `docs/architecture.md` |
| `clojure-diagnostics` | codes, labels, collections, rendering | deterministic ordering and source lookup | rustdoc examples |
| `clojure-syntax` | every form, metadata, spanned aliases | form kind and printed representation | rustdoc examples |
| `clojure-reader` | entry points and returned forms | delimiters, escapes, metadata, shorthand, recursion | rustdoc examples; pipeline stage 1 |
| `clojure-value` | bootstrap value model and helpers | equality, truthiness, numeric limits, printing | rustdoc examples; bootstrap/native boundary |
| `clojure-interp` | interpreter state and evaluation entry points | environments, arity, tail `recur`, output capture | rustdoc examples; pipeline boundary |
| `clojure-analyzer` | AST, calls, primitives, program records | lexical captures, expansion, dispatch, linearity | crate tests; pipeline stages 2–3 |
| `clojure-codegen` | options, object API, embedded runtime | ABI lowering, GC frames, constants, fast/slow paths | crate tests; pipeline stage 4 |
| Native C runtime | subsystem headers and every `cljn_*` contract | tags, rooting, ownership, allocation, data structures | runtime tests; `runtime_all.c` order |
| `clojure-native-cli` | executable boundary and build stages | temporary files, linker side effects, script/main behavior | end-to-end test module; architecture flow |
| `core_compiled.clj` | docstrings for all 26 functions | eagerness, short-circuiting, transient accumulator | compiled-core end-to-end tests |
| `clojure-test-support` | fixture schema and all public APIs | isolation, timeouts, comparison, checksums, statuses | crate tests; verification architecture |
| Generators and tests | module intent and ownership rules | deterministic fixture generation and unusual test contracts | conformance and runtime suites |

## Baseline

Baseline captured on 2026-07-28 at commit `f5d9cd5`.

- Tracked worktree: clean. Untracked `sieve-native` and `sieve_native.asm` are
  unrelated and excluded from every documentation commit.
- `cargo test --workspace`: green, 136 tests and doctests passing.
- `cargo clippy --workspace --all-targets -- -D warnings`: green.
- `cargo fmt --all --check`: pre-existing formatting differences in
  `clojure-analyzer/src/lib.rs` and `clojure-codegen/src/lib.rs`.
- `make compatibility`: no failing `active` case; 170 passes, 232 expected
  failures, 32 pending cases, and 13 pre-existing unexpected passes.
- Conformance report checksum:
  `b670fb5cae50f8c90c37c2525b9a2b95287692be283372546f434a8045520087`.

The unexpected-pass baseline is:

```text
b.io_dynamic_bindings.flush_on_newline_disabled
b.io_dynamic_bindings.stderr_redirection
c.clojure_core.flush.normal
c.clojure_core.read_string.boundary
c.clojure_core.read_string.normal
c.clojure_core.slurp.boundary
c.clojure_core.slurp.normal
c.clojure_core.spit.normal
c.clojure_core.with_in_str.boundary
c.clojure_core.with_in_str.normal
c.clojure_core.with_open.boundary
c.clojure_core.with_out_str.boundary
c.clojure_core.with_out_str.normal
```
