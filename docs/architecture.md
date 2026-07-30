# Architecture

[Documentation index](README.md) · [Overview](overview.md) ·
[Usage](usage.md) · [Specifications](../specs/README.md) ·
[Documentation style](../specs/DOCUMENTATION_STYLE.md)

`clojure-compiler` is a Cargo workspace whose delivered executable is named
`clojure-native`. This page is an index of component boundaries and executable
behavior. Detailed type and function contracts live in the linked Rust modules
and in the native runtime source.

## Workspace components

| Crate | Responsibility and internal reference |
| --- | --- |
| `clojure-span` | [UTF-8 source storage, byte spans, and display locations](../src/compiler/clojure-span/src/lib.rs) |
| `clojure-diagnostics` | [Stable diagnostic codes and deterministic rendering](../src/compiler/clojure-diagnostics/src/lib.rs) |
| `clojure-syntax` | [Spanned reader forms and metadata representation](../src/compiler/clojure-syntax/src/lib.rs) |
| `clojure-reader` | [Tokenization, reader macros, and form parsing](../src/compiler/clojure-reader/src/lib.rs) |
| `clojure-value` | [Bootstrap-interpreter values](../src/compiler/clojure-value/src/lib.rs) |
| `clojure-interp` | [Bootstrap evaluation and its boundary from the native ABI](../src/compiler/clojure-interp/src/lib.rs) |
| `clojure-analyzer` | [Expansion, lexical resolution, captures, `recur`, dispatch, and transient analysis](../src/compiler/clojure-analyzer/src/lib.rs) |
| `clojure-codegen` | [Cranelift lowering, GC frames, ABI imports, and object emission](../src/compiler/clojure-codegen/src/lib.rs) |
| `clojure-native-cli` | [`read`, `eval`, `run`, and the AOT build orchestration](../src/compiler/clojure-native-cli/src/main.rs) |
| `clojure-test-support` | [Fixture schema, isolation, comparisons, checksums, oracle, and reports](../src/compiler/clojure-test-support/src/lib.rs) |

## Compilation flow

```text
UTF-8 .clj source
        |
        v
reader ------> spanned forms
        |
        v
known expansion + analyzer ------> analyzed Program/Expr
        |
        v
Cranelift codegen ------> host object
        |
        v
system C driver + amalgamated runtime ------> native executable
```

The CLI prepends the
[compiled `clojure.core` subset](../src/compiler/clojure-native-cli/src/core_compiled.clj)
and analyzes it together with user forms. It then writes a temporary object and
runtime translation unit, invokes the C compiler driver selected by `CC`, and
removes those temporary files after linking. The precise stage contracts are
indexed by [`COMPILER_PIPELINE.md`](../specs/COMPILER_PIPELINE.md).

After semantic analysis, a conservative post-pass recognizes fresh vector
accumulators used linearly. It covers local loops and the initial
interprocedural linear-parameter pattern. Capture, aliasing, or an unknown call
forces the persistent path. The pass annotates the existing AST; there is no
separate HIR or LIR today.

## Native value and call model

Compiled fixnums and other immediates use tagged `Value` words. Heap values are
pointers to objects tracked by the native collector. Frequent integer
operations have guarded Cranelift fast paths; type errors, overflow, and other
general cases use checked C ABI functions.

Every compiled function uses `(self, argc, argv)`. That uniform convention
supports fixed, multiple, and variadic arities, closures, indirect calls, and
`apply`. `loop`/`recur` lowers to a native backedge without growing the C stack.
The source of truth for tags and layouts is
[`00_types.c`](../src/compiler/clojure-codegen/runtime/00_types.c); matching Rust
declarations carry `ABI:` comments in the codegen module.

## Collections

- Lists are linked cons cells.
- Vectors are persistent 32-way bitmap tries.
- Small maps and sets use compact arrays and promote to 32-way HAMTs.
- Sorted maps and sets use persistent left-leaning red-black trees.
- Records combine nominal identity with associative fields.
- Vector transients own or copy trie nodes; map and set transients currently
  wrap a persistent value in a mutable box.

Persistent updates use path copying and structural sharing. The compiled core
uses transients in `mapv` and vector-targeted `into`; the analyzer can select
the same path when it proves accumulator uniqueness. **Planned:** CHAMP,
general escape analysis, and in-place map/set transients are tracked by the
applicable specifications and ADRs.

## Runtime, I/O, and GC

The embedded C runtime implements allocation, strings, printing, functions,
collections, records, transients, exceptions, multimethods, I/O, the runtime
reader, and slow paths. It is split into ordered subsystem fragments under
[`src/compiler/clojure-codegen/runtime/`](../src/compiler/clojure-codegen/runtime/).
[`runtime_all.c`](../src/compiler/clojure-codegen/runtime/runtime_all.c) documents the
amalgamation order. The fragments still compile as one translation unit and do
not introduce separate libraries, states, or ABIs.

Implemented I/O includes dynamic input and output bindings, string and file
readers/writers, text and byte-array operations, filesystem helpers, command
arguments, file metadata, and the supported runtime `read-string` subset.
The contracts and remaining planned stages are in
[`IO_SPEC.md`](../specs/IO_SPEC.md) and
[`ADR-0007`](../specs/adr/0007-native-io-and-runtime-reader.md).

The collector is precise, non-moving, single-threaded mark-sweep. Each compiled
function opens a frame in the shadow stack. Codegen emits direct root-slot and
stack-pointer accesses, while runtime entry/exit operations delimit the frame.
Any heap value live across an allocating ABI call must remain rooted.
`CLJN_GC_STRESS=1` collects at every allocation to exercise that contract.

Immediate-only non-empty vector literals receive a site identifier. Their first
evaluation constructs and registers the vector in a permanent runtime cache;
later evaluations load the same rooted value. Literals containing dynamic
elements follow normal construction.

## Verification architecture

[`clojure-test-support`](../src/compiler/clojure-test-support/src/lib.rs) discovers
every case below [`tests/conformance/`](../tests/conformance), validates the
strict manifest schema and fixture checksum, and then filters cases for
execution. Executable cases run in isolated temporary directories with bounded
parallelism and timeouts. Text, binary output, structural forms, exit status,
and declared filesystem effects are compared as appropriate.

An `active` mismatch fails. An `xfail` mismatch is expected, while an `xfail`
that passes is an `unexpected-pass` and also fails the gate until promoted.
`pending` cases are schema- and checksum-validated but not executed. JVM oracle
comparison and blessing are explicit maintainer operations, never dependencies
of normal verification.

The [`Makefile`](../Makefile) is the stable operational interface:

```text
make quality
   |-- rustfmt
   |-- documentation structure, rustdoc, and doctests
   |-- clippy and clj-kondo
   `-- workspace tests

make all
   |-- quality
   |-- coverage
   |-- compatibility
   `-- benchmarks
```

Normative behavior and planned work remain in the
[specifications](../specs/README.md). Documentation policy, templates, and
critical contract markers are defined in
[`DOCUMENTATION_STYLE.md`](../specs/DOCUMENTATION_STYLE.md).
