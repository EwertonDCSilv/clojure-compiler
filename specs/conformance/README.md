# Executable Clojure conformance suite

The executable fixtures live in [`tests/conformance/`](../../tests/conformance), while
this document records their contract and maintenance policy. The suite classifies the
compiler's current behavior by compatibility levels A–E from
[`COMPATIBILITY_SPEC.md`](../COMPATIBILITY_SPEC.md). Classification follows the code
that is executable today, rather than aspirational scope documents.

Current inventory: **181 cases** — 141 active, 9 expected failures, and 31 pending.

## Run it

```bash
# Offline native verification; no JVM and no downloads
scripts/conformance.sh verify

# Inventory and filters
scripts/conformance.sh list
scripts/conformance.sh list --level A --status active
scripts/conformance.sh list --area arithmetic
scripts/conformance.sh list --namespace clojure.string
```

`verify` builds the release CLI once and reuses it for every case. At most four cases
run concurrently. The machine-readable report is
`target/conformance/report.json`; the human summary is
`target/conformance/report-summary.txt`.

## Levels and current policy

| Level | Directory | Policy |
| --- | --- | --- |
| A | `level-a-syntax` | Reader syntax, trivia, metadata, macros, and diagnostics |
| B | `level-b-semantics` | Native execution, errors, records/protocols, and GC stress |
| C | `level-c-stdlib` | Current compiled `clojure.core`; documented namespaces remain visible |
| D | `level-d-pure-libraries` | Pending project fixtures for pure libraries |
| E | `level-e-ecosystem` | Pending ecosystem, classpath, interop, and application fixtures |

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
├── expected.stdout
├── expected.stderr
└── expected.edn
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
```

Allowed values are:

- `status`: `active`, `xfail`, `pending`;
- `class`: `spec`, `official`, `expected-diff`, `unsupported`;
- `target`: `reader`, `build-run`, `build-error`, `project`;
- `oracle`: `equal`, `expected-diff`, `not-applicable`.

Reader results use structural comparison: map and set order does not affect equality.
Build-error expectations are stable diagnostic fragments/categories, so temporary
paths do not make fixtures platform-specific. Line endings are normalized.

## Checksums

`tests/conformance/checksums.sha256` covers every manifest, input, and expectation.
Verification fails on missing, stale, or changed entries. After an intentional fixture
edit, regenerate the matrix and checksums with:

```bash
cargo run -p clojure-test-support --example generate_suite
```

Review the resulting diff before committing.

## Manual Clojure/JVM oracle

The oracle is deliberately excluded from CI and never downloads artifacts. Supply a
local classpath containing Clojure **1.12.5** and its dependencies:

```bash
CLOJURE_CLASSPATH=/path/to/clojure-1.12.5.jar:/path/to/spec.alpha.jar:/path/to/core.specs.alpha.jar \
  scripts/conformance.sh oracle --check
```

The runner checks the reported Clojure version before executing cases. Updating
expectations is a separate, explicit operation:

```bash
CLOJURE_CLASSPATH=... scripts/conformance.sh oracle --bless
```

`--bless` updates only `oracle = "equal"` cases and refreshes checksums. Declared
differences are never overwritten from the JVM.

## Acceptance gate

CI runs `scripts/conformance.sh verify` without a JVM. The gate requires all active
cases to pass, every xfail to remain an expected failure, no unexpected pass, and an
exact checksum inventory. Unit tests in `clojure-test-support` cover discovery, strict
TOML parsing, path/newline normalization, process timeouts, structural collection
comparison, error categories, checksums, filters, and state accounting.
