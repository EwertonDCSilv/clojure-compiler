# Changelog

All notable changes to `clojure-compiler` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the
project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html) for published
tags.

## [Unreleased]

### Added

- Add a gradual Rust module-size gate with explicit limits and grandfathered
  baselines that prevent oversized existing files from growing silently.
- Add versioned crate and high-risk-module coverage baselines, a no-regression
  ratchet, JSON reports, and a 90% executable-Rust diff-coverage gate to `make coverage`.
- Add an AI-agent story-point guard that blocks feature implementation for
  unestimated Roadmap issues, epics, roll-ups, and issues above 8 points, with
  offline tests and fail-closed local commit/push hooks.
- Add a frozen Clojure 1.12.5 reader capability catalog and an offline
  `reader-syntax-coverage` gate that reports traceability, native executable support,
  strict JVM parity, and the exact remaining scenario counts.
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
- Add ADR-0015 for internal unboxed values, virtual GC roots, specialized direct-call
  ABIs, and measured reduction of C runtime boundaries.
- Add the isolated `--ir-experiment adr15` candidate with compact root frames, raw
  fixnum locals/loops, checked raw arithmetic, specialized fixed-arity entries, and
  generic-boundary fallbacks.
- Add deterministic aggregate lowering metrics through `--ir-stats PATH` and make the
  paired Cormen runner configurable for `safe`-versus-candidate comparisons.
- Add ADR-0013 Gate 1 static module loading: initialized top-level `def` globals with
  permanent GC roots, a `--source-path` multi-file namespace graph with transitive
  `:require` resolution, dependency-cycle and missing-file diagnostics, and an embedded
  built-in source bundle resolved ahead of local roots.
- Add per-namespace symbol resolution (namespace-mangled `def`/`defn`, alias and
  `clojure.core` auto-refer), namespaced keywords, and the `string?`/`int?`/`keyword?`/
  `vector?`/`map?`/`bytes?` type predicates plus `str-split`.
- Add the ADR-0013 P0/P1 native connector namespaces `cljn.http.request`,
  `cljn.http.response`, `cljn.pedestal.chain`, `cljn.pedestal.route`, and
  `cljn.pedestal.connector`: request/response validation with categorized error maps,
  a synchronous interceptor chain (`:enter`/`:leave`/`:error`, termination, recovery),
  a deterministic linear router (`:param` capture, 404/405, ambiguity rejection), and
  `test-request` sharing the same dispatch path as the network route.
- Add the strict bounded HTTP/1.x `parse-http-request` and `serialize-http-response`
  runtime primitives (ADR-0013 Gate 4), rejecting Transfer-Encoding, divergent
  Content-Length, folded headers, NUL, bare CR/LF, invalid tokens/versions, header
  injection, and limit overflow.
- Add the loopback HTTP socket provider (`T_HTTP_SERVER`,
  `http-server-open`/`-port`/`-accept`/`-respond`/`-close`) with a Clojure-driven
  synchronous service loop, bounded timed reads, and GC-sweep descriptor cleanup,
  serving real ephemeral-port loopback requests (ADR-0013 Gate 4).
- Add a self-pipe `SIGINT`/`SIGTERM` stop mechanism, `http-server-stop`, and the
  `cljn.pedestal.service` lifecycle (`start!`/`serve!`/`serve-one!`/`server-port`/
  `stop!`) that drives the connector's shared dispatch over the provider and shuts
  down cleanly on a signal or a dispatch-path stop (ADR-0013 §3/§6, Gate 5).
- Add the root `native_http_server.clj` runnable example with loopback health checks
  and a bounded JSON body-dump endpoint for manual `curl` validation.
- Emit response headers in deterministic lowercase lexical order and cover the
  no-descriptor-leak service contract with a multi-request end-to-end test
  (ADR-0013 Gate 4 acceptance #6, Gate 5).
- Add the `runtime_http.c` HTTP parser/serializer fuzz harness to the C runtime test
  suite, exercising an adversarial corpus and every input prefix under ASan/UBSan via
  `make test-runtime-sanitize`, and an open/close-cycle descriptor-leak end-to-end
  test (ADR-0013 Gate 4 CI hardening).
- Add a subprocess start/serve/stop cycle gate (default 200, `CLJN_HTTP_CYCLES=1000`
  in CI) that verifies repeated full server lifecycles serve one request and exit
  cleanly with no crash or hang (ADR-0013 Gate 4 acceptance #5/#6).
- Add a continuous libFuzzer target for the HTTP request parser
  (`tests/fuzz/http_parse_fuzz.c`) with the `scripts/fuzz-http.sh` runner and the
  `make fuzz-http` entry point, seeded from the deterministic corpus (ADR-0013 Gate 4).
- Add a CI `security` job that runs the runtime C harnesses under ASan/UBSan, the
  1000-cycle HTTP start/serve/stop gate, and an HTTP parser fuzz smoke.
- Add the active `e.pedestal.native_connector_hello` conformance fixture that builds
  and runs a native `cljn.pedestal.*` router connector through the in-memory
  `test-request` dispatch path with no JVM or network, while the upstream
  `e.pedestal.hello_world_api` fixture stays `pending` (ADR-0013 §11, Gate 5).
- Add ADR-0016 defining the Clojure/JVM oracle classification policy (`equal` vs
  `not-applicable` vs `expected-diff`) and a mechanical decision rule based on
  whether the JVM can evaluate a case and whether it deliberately differs.
- Add the separate `benchmarks/http/` suite (ADR-0013 Gate 6) that drives the native
  `cljn.pedestal.*` connector and the pinned upstream Pedestal http-kit connector
  (0.8.2-beta-10) through one shared load client, proves byte-identical responses by
  SHA-256 before comparing timing, and records medians over repetitions with
  environment metadata. It is versioned apart from the language conformance and
  Cracking/Cormen/Exercism catalogs and runs on demand outside CI.
- Add the `tests/differential/pedestal/` interceptor-chain differential (ADR-0013
  Gate 3) that runs a scenario corpus through the compiled `cljn.pedestal.chain` and
  the pinned upstream `io.pedestal.interceptor.chain` under the same
  terminate-on-`:response` rule and diffs the observable output, proving native
  interceptor order, termination, unwind, and recovery match the manual oracle. It
  resolves Pedestal from Clojars and runs on demand outside CI.
- Add `specs/PEDESTAL_UPSTREAM_INVENTORY.md` (ADR-0013 Gate 7): the pinned upstream
  Pedestal snapshot, its namespace inventory, the first compilation blocker per P2
  candidate, and the decision that compiling upstream Pedestal (P2) is not useful
  until general Java interop, an async model, and user macros exist — no P3 work is
  proposed.
- Auto-load compiler-owned `cljn.io` and `cljn.process` modules for qualified calls
  without `:require`, exposing the path/filesystem wrappers backed by existing
  ADR-0007 primitives and stable `:invalid-input` data errors (issue #103).
- Add the `cljn.io` stream API (issue #121): file and in-memory `reader`/`writer`/
  `input-stream`/`output-stream`/`string-reader`/`string-writer`, handle `read-char`/
  `read-line`/`unread-char`/`write!`/`flush!`/`close!`/`closed?`/`writer-string`, and
  the `doto` threading macro, backed by reader/writer closed-state and pushback plus
  new stream runtime primitives, promoting 48 conformance fixtures to `active`.
- Add the `cljn.io` byte-stream API (issue #125): `byte-input-stream`/`byte-output-stream`,
  `read-bytes`, `write-bytes!`, `output-bytes`, and `read-block!`, backed by RD_BYTES/
  WR_BYTES stream kinds and new runtime primitives, promoting 19 conformance fixtures
  to `active`.
- Add the `cljn.io` byte API (issue #119): `bytes` (from a 0..255 vector), `bytes?`,
  `bytes->string` (UTF-8-validated), `bytes->vector`, `string->bytes`, and
  `byte-count`, backed by new `bytes-of-vec`/`bytes->vec`/`valid-utf8?` runtime
  primitives, promoting 18 conformance fixtures from `xfail` to `active`.

### Changed

- Classify the conformance cases the manual Clojure/JVM 1.12.5 oracle cannot treat
  as equal (ADR-0016): 33 native-only or JVM-rejected cases become
  `oracle = not-applicable` and 11 deliberate representation/ordering/`*out*`-routing
  differences become `oracle = expected-diff`, and one stale `expected-diff` that the
  JVM actually matches is demoted to `equal`. Every `status` and `expected.*` output
  is unchanged, so `make compatibility` still enforces the same native behavior;
  `make compatibility-oracle` now runs green with no failures or unexpected passes.
- Stop tracking the local `.lsp/.cache/` editor cache and ignore future cache files.
- Require every AI-assisted feature to start from an issue registered in the public
  project, use a `feature/<issue-number>-<semantic-description>` branch, and reach
  `master` through a pull request; documentation checks now preserve this policy.
- Keep ADR-0015 outside the admitted `safe` profile after its first full Cormen gate
  recorded candidate/control ratios of 1.0066 wall and 1.0054 CPU, failed the required
  3% gain, and exceeded the per-case point-estimate ceiling.
- Mark ADR-0014 and the optimization IR specification as partially implemented rather
  than describing the complete whole-function pipeline as delivered.
- Extend benchmark runners and project documentation with the experimental IR profile
  and its promotion contract.
- Propagate proven fixnum representations through sequential bindings, loop/recur
  fixed points, intrinsic integer results, and non-escaping fixed-arity direct
  functions. Proven arithmetic and comparisons now omit redundant type guards while
  preserving overflow, divide-by-zero, and runtime slow paths.

### Performance

- Refresh all 98 Native versus Clojure/JVM measurements at `424ba20`, regenerate the
  report and GitHub Pages charts, and preserve matching checksums. Cormen native wall
  time fell from 30.06 s to 27.23 s and CPU from 29.95 s to 27.09 s in the new
  single-run snapshot; paired measurements remain authoritative for optimization
  promotion.
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
