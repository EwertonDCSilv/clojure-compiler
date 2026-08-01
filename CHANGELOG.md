# Changelog

All notable changes to `clojure-compiler` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the
project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html) for published
tags.

## [Unreleased]

### Fixed

- Force the C locale (`LC_ALL=C`) in `scripts/aggregate-benchmark-runs.sh` and its
  test. Under a locale with a comma decimal separator (e.g. `pt_BR.UTF-8`), awk's
  `sprintf` embedded extra commas in formatted metrics, corrupting the
  comma-separated output, and the test's own `$column + 0` numeric coercion
  silently truncated fractional values at the locale-specific decimal point,
  masking the corruption.
- Update the `tests/scripts/check-rust-file-size.sh` fixture to the `src/compiler/`
  layout (previously `crates/`), which the size gate now scans after the
  standard-library reorganization.
- Add missing `//!` module documentation to the `clojure-analyzer` submodules
  (`ast`, `top_level`, `analysis`, `optimizations`, `primitives`) introduced
  by the issue #112 split, required by `scripts/check-docs.sh`.

### Changed
- Split `clojure-analyzer`'s 2789-line `lib.rs` into `ast` (backend AST and
  primitive table), `top_level` (`def`/`defn`/`defrecord`/`defprotocol`/
  `extend-type`/`defmulti`/`defmethod` recognition), `analysis` (the `Frame`/
  `Analyzer` scope, capture, and expression-analysis core, kept as one
  cohesive module), `optimizations` (the transient-accumulator rewrite), and
  `primitives` (primitive name table and arity checks) (issue #112).
  `lib.rs` is now a 37-line facade. No behavior change: 12/12 unit tests,
  `make compatibility` (385 active, 0 failed, 0 unexpected), and a full
  Cormen A/B gate (7 repetitions, scale 25, 30 cases) confirm identical
  checksums across all cases and a median wall-time ratio of 1.03x, within
  measurement noise for these short-running benchmarks.
- Add 19 characterization tests for the bootstrap interpreter's `var` and
  `apply` special forms, user-function arity errors (fixed and variadic),
  malformed `let`/`loop` binding vectors, unbound-symbol resolution, `def`
  redefinition, and `recur` outside a loop/fn (issue #116). No new special
  form or semantics; `cargo-llvm-cov` is unavailable in this sandbox, so exact
  coverage deltas are left to CI's existing ratchet.
- Extract namespace-dependency loading (`parse_requires`,
  `implicit_builtin_requires`, `load_deps`, the built-in module table) from
  `clojure-native-cli`'s `main.rs` into a `project` module (issue #115), leaving
  `main.rs` a 397-line composition root of command dispatch. The 63 E2E tests and
  2 unit tests are unaffected; CLI commands, exit codes, and diagnostics are
  byte-identical.
- Split `generate_suite.rs`'s single 2660-line `fixtures()` literal into
  `level_a_b_c_core_cases`, `level_b_stdout_baseline_cases`,
  `level_b_io_prerequisites_cases`, `level_b_io_followups_cases`,
  `level_d_cases`, and `level_e_cases` (issue #111), each independently
  reviewable. `fixtures()` now concatenates them in the original order.
  Regenerating the suite is verified byte-identical (same 1525 files, same
  checksum manifest digest).
- Split `clojure-test-support`'s 1977-line `lib.rs` into `manifest`, `execution`,
  `workspace`, `comparison`, `checksum`, `report`, and `oracle` modules (issue
  #110), leaving `lib.rs` a 76-line facade re-exporting the unchanged public API.
  Behavior-preserving; the conformance CLI, `generate_suite` example, and 26
  unit tests are unaffected.
- Add a positive/negative test matrix for every `clojure-ir` safe pass
  (simplify-cfg, checked-constant-folding, copy-propagation, local-cse,
  branch-simplification, effect-aware-dce), 11 additional verifier diagnostic
  cases (structural, dominance, ABI, and GC-root invariants), and printer
  snapshots covering every `InstructionKind`/`Terminator`, block-parameter
  joins, and loops (issue #117). No pass, output, or generated assembly changes.

- Move Rust unit-test bodies out of production modules into mirrored
  `src/compiler/<crate>/tests/unit/<module>/mod.rs` paths without changing
  assertions (issue #165).

## [0.0.3] - 2026-07-30

### Fixed

- Synchronize the native I/O end-to-end expectation with the `:open-reader`
  operation reported by the current reader contract.
- Release temporary GC roots when a function with no local root slots returns.

### Changed
- Remove the `prototypes/` folder (the `p01-cranelift-exe` disposable spike that
  validated ADR-0001's Cranelift-to-native-executable backend question) and the
  now-unused workspace `exclude` entry for it; `specs/IMPLEMENTATION_PLAN.md` keeps
  documenting the Fase 0.3 methodology as a historical record.
- Untrack four compiled build artifacts accidentally committed to the repository
  root during early native-codegen development (`factorial-native-bigger`,
  `loop-benchmark-native`, `sieve-native` ELF executables, and
  `sieve_native.asm`), and add `.gitignore` entries preventing their
  reintroduction.
- Remove `kw.clj`, a leftover ad-hoc debugging scratch file with no references
  elsewhere in the repository.
- Move the `native_http_server.clj` runnable example from the repository root to
  `examples/`, alongside the other example programs.
- Move the compiler crates to `src/compiler/` and the standard library to
  `src/stdlib/`, updating workspace members, size/coverage baselines, lint and
  coverage scripts, conformance `tracking` pointers, and documentation to the new
  layout. Behavior-preserving; `include_str!` module paths are unchanged because
  both trees moved one level deeper together.
- Refresh the documentation site conformance banner to the current matrix totals
  (528 A–E cases, 372 active), replacing the earlier 460/186 snapshot.

### Added
- Add a closed-`*out*` check to `clojure.core` `flush` (issue #163), which now
  raises `:invalid-input` like `pr`/`prn`/`newline`, and record the native error
  contracts for `spit` on a directory and `with-out-str`/`with-open` bodies that
  throw; promotes 4 fixtures.
- Add the always-available `clojure.edn` namespace (issue #160) with native `read`
  and `read-string` over the runtime EDN reader (sets, catchable `:invalid-input`
  on `#=`/incomplete input); a leading options map is accepted, with `:eof`
  honored by `read`. Promotes 5 fixtures; the custom `:default` tag-reader boundary
  stays pending.
- Add `clojure.core` `read` (issue #158): reads one form from a string reader,
  honoring an `{:eof v}` options map at end-of-input, backed by new `read-from`/
  `reader-eof?` runtime primitives. Runtime reader parse errors (including the
  `#=` eval macro and incomplete forms) now raise a catchable `:invalid-input`
  instead of aborting, so `read`/`read-string` recover with try/catch; promotes 4
  fixtures.
- Add 67 pending level-A reader fixtures (issue #156) covering every previously
  untraced Clojure 1.12.5 reader-syntax scenario (bigint/bigdecimal/ratio/radix/
  symbolic-value/array-class literals, regex/anonymous-function/discard/conditional/
  namespaced-map/tagged/record-constructor reader macros, metadata, lexical,
  diagnostics, and source-mapping), wiring each into the reader catalog and raising
  `reader-syntax-coverage` traceability from 48.06% to 100.00% (0 missing scenarios).
- Add `clojure.core` `ex-info` and `ex-message` (issue #154): `ex-info` builds a
  tagged exception value whose data and message are read back by `ex-data`/
  `ex-message`, while directly-thrown data maps keep map-identity `ex-data` (the
  `cljn.io` `:kind` contract is unchanged). The native `slurp` open failure now
  reports operation `:open-reader`, promoting 5 exception/lifecycle fixtures.
- Add the `clojure.core` printing functions `pr`, `prn`, and `newline` (issue #152):
  variadic `pr`/`prn` write their arguments to the current `*out*` (native output
  does not quote strings, a deliberate reader-syntax divergence from the JVM), and
  all three raise `:invalid-input` when `*out*` is a closed writer, promoting 9
  printing fixtures plus the `argv`/environment/cwd integration case to `active`.
- Add the `cljn.process` working-directory and environment API (issue #147): `cwd`
  (returns a `cljn.io` path value composing with the path-algebra functions) and
  `environment` (an immutable map of all environment variables), backed by
  getcwd/environ runtime primitives, promoting 6 conformance fixtures to `active`.
- Add the `cljn.io` recursive and attribute API (issue #144): `attributes`,
  `copy-tree!` (preserving symbolic links), and `delete-tree!` (with an
  ignore-missing option), both refusing the filesystem root without touching disk,
  completing the `cljn.io` conformance surface (10 fixtures promoted to `active`).
- Add the `cljn.io` path-algebra API (issue #142): `path` now yields a distinct
  path value, `parent`/`file-name`/`join` require it, and `absolute?`, `normalize`
  (lexical), and `real-path` (realpath) join the surface, promoting 10 conformance
  fixtures to `active`.
- Add the `cljn.io` symbolic-link and copy API (issue #140): `create-symlink!`,
  `read-link`, `symlink?`, and `copy!`, backed by symlink/readlink/lstat runtime
  primitives, promoting 12 conformance fixtures to `active`.
- Add the `cljn.io` random-access API (issue #138): `seek!`, `truncate!`, and
  `position` over file readers/writers, backed by fseek/ftruncate/ftell runtime
  primitives and `file-reader?`/`file-writer?` predicates, promoting 9 conformance
  fixtures to `active`.

- Add `make benchmark-page-refresh` to run tests, Native × JVM comparisons, status
  validation, and deterministic Pages data/chart generation in one Bash workflow.
- Add Tidy First rules for AI agents, including change classification, isolated
  branches and pull requests, behavior-preserving evidence, and explicit stop
  conditions.
- Add weekly Dependabot updates for Cargo dependencies and GitHub Actions.
- Add a gradual Rust module-size gate with explicit limits and grandfathered
  baselines that prevent oversized existing files from growing silently.
- Add a pull-request template and explicit AI-agent rules requiring issue linkage,
  scoped changes, TDD evidence, and truthful validation records.
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

- Publish benchmark CSVs and Pages assets from per-case medians over ten complete
  Native × JVM rounds, with cross-round validation and retained raw samples; refresh
  the versioned comparison artifacts with the first ten-round result.
- Update the Cranelift backend crates to 0.134.2, `sha2` to 0.11, and `toml` to
  1.1, including the required code-generation API migration.
- Refresh every ADR `Status` field to reflect the delivered state: ADR-0001–0008 are
  accepted and implemented, ADR-0011 and ADR-0016 are implemented, ADR-0012 is
  partially implemented (file-size gate landed, module extraction pending), and
  ADR-0013 is accepted with P0/P1 delivered (Gates 1–7) while P2/P3 remain pending.
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

- Elide zero-slot shadow-stack frames for rootless functions whose temporary-root
  accounting is balanced, while preserving frame cleanup for heap results (issue
  #149, ADR-0017).
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
