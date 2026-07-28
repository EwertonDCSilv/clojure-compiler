# Changelog

All notable changes to `clojure-compiler` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the
project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html) for published
tags.

## [Unreleased]

### Added

- Add this release changelog and enforce `Unreleased` updates through repository
  instructions, contributor guidance, documentation checks, and the staged pre-commit
  hook.
- Add the backend-neutral `clojure-ir` crate with explicit CFG blocks and values,
  representation and effect facts, deterministic printing, structural verification,
  liveness analysis, and conservative root-slot planning.
- Add checked constant folding, CFG and branch simplification, copy propagation, local
  common-subexpression elimination, and effect-aware dead-code elimination.
- Add the opt-in `--ir-opt none|safe` compiler switch while retaining `none` as the
  stable default.
- Add dual-mode language conformance execution through `--ir-opt`.
- Add a paired, alternating Cormen native A/B gate with raw samples, environment
  metadata, medians, MAD, and deterministic bootstrap confidence intervals.

### Changed

- Stop tracking the local `.lsp/.cache/` editor cache and ignore future cache files.
- Mark ADR-0014 and the optimization IR specification as partially implemented rather
  than describing the complete whole-function pipeline as delivered.
- Extend benchmark runners and project documentation with the experimental IR profile
  and its promotion contract.
- Propagate proven fixnum representations through sequential bindings, loop/recur
  fixed points, intrinsic integer results, and non-escaping fixed-arity direct
  functions. Proven arithmetic and comparisons now omit redundant type guards while
  preserving overflow, divide-by-zero, and runtime slow paths.

### Performance

- Preserve all 30 Cormen checksums in the seven-pair, scale-25 IR comparison and pass
  the blocking non-regression gate. The opt-in `safe` profile records
  candidate/control ratios of 0.9568 for wall time and 0.9565 for CPU, corresponding
  to median improvements of 4.32% and 4.35%; `none` remains the default.

## [0.0.2] - 2026-07-28

### Added

- Add static multi-file namespace loading, namespace-qualified definitions, and
  permanently rooted top-level globals.
- Add compiler-owned `cljn.http.*` and `cljn.pedestal.*` namespaces with request and
  response helpers, interceptor chains, deterministic routing, connector validation,
  an HTTP/1.x request parser, and response serialization.
- Add native dynamic streams, `binding`, input helpers, UTF-8 character values, bytes,
  paths, file streams, filesystem operations, process arguments, environment access,
  and runtime `read-string`.
- Add boxed floating-point values and polymorphic numeric operations.
- Add native C runtime unit, ABI, error, GC-stress, ASan, and UBSan test harnesses.
- Add the pinned Exercism conformance and performance corpus.
- Add the multilingual GitHub Pages project site and reproducible Rust benchmark chart
  generator.
- Add repository-wide documentation, TDD, lint, hook, and quality-gate policies.

### Changed

- Modularize the embedded C runtime by subsystem while preserving one amalgamated
  translation unit and public ABI.
- Add structural transients, transient-backed `mapv` and `into`, conservative
  interprocedural uniqueness analysis, and permanent caching of immediate vector
  literals.
- Expand and promote the executable conformance catalog for native I/O, floating-point
  code generation, Exercism, and the Pedestal target.
- Refresh Native versus Clojure/JVM benchmark results, summaries, SVG charts, and the
  project dashboard.

## [0.0.1] - 2026-07-27

### Added

- Establish the Rust workspace, spanned reader, bootstrap interpreter, semantic
  analyzer, Cranelift AOT backend, native CLI, and compact C runtime.
- Compile native control flow, recursion, closures, higher-order calls, multiple and
  variadic arities, `apply`, known core macros, records, protocols, and multimethods.
- Add tagged fixnums, checked native arithmetic fast paths, a precise non-moving
  mark-sweep collector, fixed shadow-stack frames, and direct root stores.
- Add persistent lists, bitmap-trie vectors, hybrid HAMT maps and sets, sorted
  collections, and initial transients.
- Add the executable A–E conformance suite, coverage and CI gates, Clojure linting, and
  90 chapter-organized Cracking and Cormen benchmark programs with JVM comparisons.
- Add bilingual project documentation, architecture specifications, and the initial
  architectural decision records.

[Unreleased]: https://github.com/EwertonDCSilv/clojure-compiler/compare/v0.0.2...HEAD
[0.0.2]: https://github.com/EwertonDCSilv/clojure-compiler/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/EwertonDCSilv/clojure-compiler/releases/tag/v0.0.1
