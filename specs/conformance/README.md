# Executable Clojure conformance suite

[Project README](../../README.md) ·
[Documentation](../../docs/README.md) ·
[Specification index](../README.md)

The executable fixtures live in [`tests/conformance/`](../../tests/conformance), while
this document records their contract and maintenance policy. The suite classifies the
compiler's current behavior by compatibility levels A–E from
[`COMPATIBILITY_SPEC.md`](../COMPATIBILITY_SPEC.md). Classification follows the code
that is executable today, rather than aspirational scope documents.

Current inventory: **460 cases** — 186 active, 242 expected failures, and 32 pending.
The I/O inventory remains intentionally incremental. Output, flushing, dynamic
redirection, `slurp`/`spit`, `read-string`, and string-backed streams have active
coverage; the remaining `xfail` cases keep the full proposed I/O gate from being
reported as delivered.

The 13 official Exercism concept exemplars are included at Level D as an external
language/stdlib corpus: 2 are active and 11 are `xfail`. They verify observable support
and promotion state; they do not measure execution performance. Native × JVM timing,
CPU and memory comparisons remain in the separate
[benchmark catalog](../../benchmarks/README.md).

The latest native-language catalog adds executable groups for exceptions,
multimethods, and transients:

- exceptions: explicit thrown values, normal/caught `finally`, nested unwind, lexical
  capture, and GC stress are active; typed multiple catches and catchable runtime
  faults remain `xfail`;
- multimethods: keyword, numeric, multi-argument, structural, and `:default` dispatch
  are active; invokable-keyword dispatch and `derive`/`isa?` hierarchies remain
  `xfail`;
- transients: vector, map, and set construction plus bulk GC stress are active;
  `disj!`/`pop!`, edit tokens, and invalidation after `persistent!` remain `xfail`;
- the level-D exception library case is active and exercises recovery at an API
  boundary.

## Run it

```bash
# Offline native verification; no JVM and no downloads
make compatibility

# Inventory and filters
make compatibility-list
make compatibility-list CONFORMANCE_ARGS="--level A --status active"
make compatibility-list CONFORMANCE_ARGS="--area arithmetic"
make compatibility-list CONFORMANCE_ARGS="--namespace clojure.string"
```

`verify` builds the release CLI once and reuses it for every case. At most four cases
run concurrently. The machine-readable report is
`target/conformance/report.json`; the human summary is
`target/conformance/report-summary.txt`.

The [`Makefile`](../../Makefile) is the recommended public entry point. The underlying
`scripts/conformance.sh` interface remains available for fixture maintenance and
low-level debugging.

## Levels and current policy

| Level | Directory | Policy |
| --- | --- | --- |
| A | `level-a-syntax` | Reader syntax, trivia, metadata, macros, and diagnostics |
| B | `level-b-semantics` | Native execution, errors, records/protocols, GC stress, and process I/O |
| C | `level-c-stdlib` | Current compiled `clojure.core` plus the planned core/EDN/`cljn.*` I/O inventory |
| D | `level-d-pure-libraries` | Executable single-file pure libraries, concrete gaps, and pending multi-file projects |
| E | `level-e-ecosystem` | Standalone native applications, executable ecosystem gaps, and pending integrated projects, including a Pedestal Hello World API |

Every active function in the embedded `clojure.core` subset has normal, boundary, and
alternate-input calls plus a separate invalid-arity diagnostic case.

Cases have one of three states:

- `active`: must match the committed expectation.
- `xfail`: must fail for the declared reason. A passing `xfail` is an error and must be
  promoted to `active`.
- `pending`: its schema and checksum are validated, but it is not executed.

## Case format

Every case is self-contained:

```text
case-name/
├── case.toml
├── input.clj
├── expected.stdout      # or expected.stdout.bin
├── expected.stderr      # or expected.stderr.bin
├── expected.edn
├── stdin.bin
├── work.before/
└── work.after/
```

Only the expectation appropriate to the target is required. `case.toml` has the
mandatory fields:

```toml
id = "b.arithmetic.addition"
level = "B"
area = "semantics/arithmetic"
status = "active"
class = "spec"
target = "build-run"
oracle = "equal"
timeout_ms = 10000
gc_stress = false
reason = "Implemented by the current native path."
tracking = "specs/COMPATIBILITY_SPEC.md#nível-b"
namespace = "clojure.core" # optional filter metadata

[run] # optional; build-run only
args = ["first", "ação"]
env = { APP_MODE = "test" }
stdin = "stdin.bin"
expected_exit = 0
platforms = ["linux"]
setup_symlinks = [{ path = "link", target = "target.txt" }]
expected_symlinks = [{ path = "copy/link", target = "target.txt" }]
```

Allowed values are:

- `status`: `active`, `xfail`, `pending`;
- `class`: `spec`, `official`, `expected-diff`, `unsupported`;
- `target`: `reader`, `build-run`, `build-error`, `project`;
- `oracle`: `equal`, `expected-diff`, `not-applicable`.

Reader results use structural comparison: map and set order does not affect equality.
Build-error expectations are stable diagnostic fragments/categories, so temporary
paths do not make fixtures platform-specific. Text line endings are normalized.
Binary streams are compared byte for byte.

Every `build-run` executes in a fresh temporary working directory. `work.before/` is
copied into it before launch; when `work.after/` exists, its entries, file bytes,
directories, and declared symlink targets must match exactly. Fixture paths must be
relative normal components: absolute paths and `..` are rejected before execution.
A platform mismatch is reported as skipped rather than passed.

## Checksums

`tests/conformance/checksums.sha256` covers every manifest, input, expectation, and
nested project contract file. Verification fails on missing, stale, or changed entries.
After an intentional fixture edit, regenerate the matrix and checksums with:

```bash
cargo run -p clojure-test-support --example generate_suite
```

Review the resulting diff before committing.

## Manual Clojure/JVM oracle

The oracle is deliberately excluded from CI and never downloads artifacts. Supply a
local classpath containing Clojure **1.12.5** and its dependencies:

```bash
CLOJURE_CLASSPATH=/path/to/clojure-1.12.5.jar:/path/to/spec.alpha.jar:/path/to/core.specs.alpha.jar \
  make compatibility-oracle
```

The runner checks the reported Clojure version before executing cases. Updating
expectations is a separate, explicit operation:

```bash
CLOJURE_CLASSPATH=... scripts/conformance.sh oracle --bless
```

`--bless` updates only `oracle = "equal"` cases and refreshes checksums. Declared
differences are never overwritten from the JVM.

## Acceptance gate

CI runs `make compatibility` without a JVM. The general gate requires all active cases
to pass, every xfail to remain an expected failure, no unexpected pass, and an exact
checksum inventory. Unit tests in `clojure-test-support` cover discovery, strict TOML
parsing, path/newline normalization, process timeouts, structural collection
comparison, binary streams, stdin/argv/env/exit status, filesystem snapshots,
declarative symlinks, error categories, checksums, filters, and state accounting.

The stronger I/O acceptance criteria—zero leaked handles, buffering, GC stress,
sanitizers, recursive safety, and differential runtime reading—are defined in
[`IO_SPEC.md`](../IO_SPEC.md).
