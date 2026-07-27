# clojure-compiler

[English](README.md) · [Português (Brasil)](README.pt-BR.md)

[Project website](https://ewertondcsilv.github.io/clojure-compiler/) ·
[Source code](https://github.com/EwertonDCSilv/clojure-compiler)

Native Clojure compilation without a JVM, built with Rust, Cranelift, and a compact C
runtime. The repository is named `clojure-compiler`; its command-line binary is
`clojure-native`.

> Experimental project under active development. It implements a documented subset of
> Clojure and is not production-ready.

## Overview

`clojure-native` reads, interprets, and AOT-compiles Clojure source code into standalone
native executables. The generated program does not require a JVM at runtime: Cranelift
emits a native object, which is linked with the embedded C runtime.

The repository is both an implementation and an architecture notebook. Specifications,
compatibility boundaries, implementation plans, and architectural decisions live under
[`specs/`](specs/README.md).

## Documentation

| Guide | Purpose |
| --- | --- |
| [`docs/README.md`](docs/README.md) | Documentation map and source-of-truth policy |
| [`docs/overview.md`](docs/overview.md) | Current capabilities and limitations |
| [`docs/usage.md`](docs/usage.md) | CLI, Makefile, installation, tests, and benchmarks |
| [`docs/architecture.md`](docs/architecture.md) | Crates, AOT pipeline, runtime, and GC |
| [`specs/conformance/README.md`](specs/conformance/README.md) | Executable A–E compatibility contract |
| [`benchmarks/README.md`](benchmarks/README.md) | Catalog and methodology for all 90 benchmarks |

## Current capabilities

- Reader with source spans, reader macros, Unicode handling, and deterministic
  diagnostics.
- Bootstrap interpreter used by `eval`, scripts, and macro infrastructure.
- Cranelift AOT code generation for standalone native executables.
- Functions, closures, higher-order functions, fixed/variadic/multiple arities, and
  `apply`.
- `if`, `do`, `let`, `loop/recur`, direct recursion, and expansion of the currently
  supported core macros.
- Tagged fixnums with checked native fast paths for `+`, `-`, `*`, `quot`, `mod`,
  `inc`, `dec`, and integer comparisons.
- Strings, lists, keywords, persistent vectors, and hybrid persistent maps and sets.
  Vectors use a 32-way bitmapped trie; small maps/sets promote to a 32-way HAMT.
- Ordered maps and sets backed by a persistent left-leaning red-black tree.
- Transient vectors with mutable bulk construction, plus transient map/set wrappers
  supporting `transient`, `persistent!`, `conj!`, `assoc!`, and `dissoc!`.
- Records and protocol dispatch through `defrecord`, `defprotocol`, and `extend-type`.
- Native `throw` and `try`/`catch`/`finally`, including nested unwind and GC-safe
  lexical captures.
- Value-based multimethod dispatch through `defmulti` and `defmethod`, with `:default`
  fallback.
- A compiled 26-function `clojure.core` subset including `map`, `filter`, `reduce`,
  `range`, `into`, `mapv`, `take`, `drop`, and `comp`.
- Precise, non-moving, single-threaded mark-sweep GC with generated shadow-stack roots.
- Direct root-stack loads/stores in generated code, removing root helper calls from hot
  paths.

For detailed implementation status, see [`specs/README.md`](specs/README.md). The
optimization roadmap and its decision record are in
[`specs/optime.md`](specs/optime.md) and
[`ADR-0006`](specs/adr/0006-codegen-optimization.md).
The proposed native I/O gate is specified separately in
[`IO_SPEC`](specs/IO_SPEC.md) and [`ADR-0007`](specs/adr/0007-native-io-and-runtime-reader.md);
only the conformance cases marked `active` are implemented today.

## Requirements

- Rust 1.74 or newer and Cargo.
- GNU Make.
- A C compiler available as `cc`, or through the `CC` environment variable.
- GNU `time` at `/usr/bin/time` for benchmarks.
- `cargo-llvm-cov` and the `llvm-tools-preview` Rust component for `make coverage`,
  `make all`, and `make ci`.
- Java for the optional `make benchmarks-compare` Clojure/JVM comparison. The reference
  reports use Java 21; `curl` and network access are needed only to populate the
  first-run artifact cache.
- A host platform supported by the current Cranelift and native linker setup.

## Build the compiler

```bash
make release
./target/release/clojure-native --help
```

Run `make help` to list the build, quality, test, compatibility, benchmark, and
installation targets.

## Install on Linux

```bash
make install
~/.local/bin/clojure-native --help
```

The default destination is `~/.local/bin`. Make sure it is on `PATH`, or override the
destination with `PREFIX=/usr/local`, `BINDIR=/another/bin`, and optionally `DESTDIR`
for staged packaging. A system-wide installation can use
`sudo make install PREFIX=/usr/local`.

## Compile and run a native program

```bash
./target/release/clojure-native build examples/hello.clj -o hello-native
./hello-native
```

Expected output:

```text
Hello from native Clojure
```

Cranelift optimization can be selected with `--opt-level none`, `speed`, or
`speed-and-size`. The current default is `none`; optimized modes remain explicit while
their benchmark regressions are investigated.

## Other CLI commands

```bash
# Print the forms produced by the reader
./target/release/clojure-native read examples/hello.clj

# Evaluate an expression with the bootstrap interpreter
./target/release/clojure-native eval '(reduce + 0 (range 10))'

# Run a source file through the interpreter
./target/release/clojure-native run examples/demo.clj
```

## Test and validate

```bash
make quality
make coverage
make compatibility
make benchmarks

# Run every local gate above
make all

# Reproduce the commands used by GitHub Actions
make ci
```

The Clojure lint gate uses a checksum-pinned `clj-kondo` release. On Linux x86_64 the
script installs it under the ignored `target/tools/` directory when needed; elsewhere,
install `clj-kondo` and expose it on `PATH` or through `CLJ_KONDO_BIN`. Deliberately
invalid conformance fixtures are excluded, while both bootstrap cores, examples,
algorithm benchmarks, and the JVM oracle are checked with warnings treated as errors.

The executable compatibility matrix currently contains 447 cases across levels A–E:
170 active, 245 expected failures, and 32 pending inventory entries. Levels D and E now
include executable pure-library and standalone-application slices in addition to
concrete expected gaps and project inventory, including a Pedestal Hello World HTTP API
target. The matrix also inventories the complete proposed I/O surface as expected
failures without claiming that surface is available. Verification runs
offline without a JVM, checks fixture integrity, and writes reports to
`target/conformance/`.

```bash
make compatibility-list CONFORMANCE_ARGS="--level A"
make compatibility-list CONFORMANCE_ARGS="--namespace clojure.core"
make compatibility
```

See [`specs/conformance/README.md`](specs/conformance/README.md) for filters, checksums,
reports, and the optional manual Clojure/JVM 1.12.5 oracle.

## Benchmarks

Both benchmark suites have a native runner and a comparison runner. Their CSV output
records wall time, CPU time, and peak memory:

Start with the [central benchmark catalog](benchmarks/README.md) for the methodology,
metrics, result reports, and direct links to all 90 cases.

```bash
make benchmarks
make benchmarks-compare
make benchmarks-charts

# Filter one suite while keeping the same entry point
make benchmarks-cracking CRACKING_ARGS="--chapter 08 --scale 10"
```

- [`benchmarks/cracking`](benchmarks/cracking/README.md): 60 chapter-organized cases
  inspired by *Cracking the Coding Interview*.
- [`benchmarks/cormen`](benchmarks/cormen/README.md): 30 CLRS-style algorithm cases
  with checksum validation.

## Project layout

| Path | Purpose |
| --- | --- |
| [`Makefile`](Makefile) | Build, quality, tests, compatibility, benchmarks, and Linux installation |
| `crates/clojure-reader` | Reader and parser |
| `crates/clojure-interp` | Bootstrap interpreter |
| `crates/clojure-analyzer` | Analysis, macro expansion, closures, records, and protocols |
| `crates/clojure-codegen` | Cranelift code generation and embedded C runtime |
| `crates/clojure-native-cli` | `read`, `eval`, `run`, and `build` commands |
| `crates/clojure-test-support` | Conformance runner, schema, oracle, and reports |
| [`tests/conformance`](tests/conformance) | Executable A–E compatibility fixtures |
| `examples` | Clojure examples and performance workloads |
| `specs` | Language scope, runtime model, plans, risks, and ADRs |
| [`docs`](docs/README.md) | Documentation index, website, usage, overview, and architecture guides |

## Known limitations

- This is a Clojure subset, not a drop-in replacement for Clojure/JVM.
- The reader accepts floating-point literals, but native compiled numeric execution is
  currently fixnum-only. Bignums, ratios, and BigDecimal are not implemented.
- User-defined macros, lazy/infinite sequences, dynamic namespace loading, and
  multi-file project compilation are not available on the native path.
- Exception catches are currently catch-all: typed catch hierarchies, multiple catch
  clauses, `ex-info`, and conversion of fatal runtime faults into catchable values
  remain incomplete.
- Multimethod dispatch requires an explicit function and supports equality plus
  `:default`; hierarchy dispatch through `derive`/`isa?` is not implemented.
- General stdin, files, filesystem operations, runtime EDN reading, and stream
  redirection are specified but not implemented; current native output is limited to
  the active `print`/`println` baseline.
- Native compilation targets the host and invokes a system C linker.
- The GC is single-threaded and non-moving. Rooting is still eager; a planned phase
  will place roots from liveness information at allocation safepoints.
- CHAMP maps/sets, transient edit tokens, `disj!`, `pop!`, and invalidation after
  `persistent!` remain future work.
- Java interop and JVM ecosystem libraries are outside the current native runtime.
