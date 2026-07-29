# Clojure 1.12.5 reader syntax coverage

[Conformance contract](README.md) ·
[Frozen catalog](clojure-1.12.5-reader.toml) ·
[Testing strategy](../TESTING_STRATEGY.md)

## Purpose

This metric answers how much of the Clojure 1.12.5 reader surface remains without
using the number of existing tests as the denominator. Counting only existing
fixtures overstates coverage because syntax without a fixture disappears from the
calculation.

The catalog freezes the official reader surface as capabilities. Every capability
owns three scenario slots:

- `normal`: representative valid syntax;
- `boundary`: limits, nesting, resolution, Unicode, or ambiguous token boundaries;
- `error`: malformed or incomplete input whose rejection is observable.

A slot can be excluded only through an explicit `not_applicable` decision. Empty
fixture arrays remain applicable gaps and therefore stay in the denominator.

## Metrics

The report keeps three independent percentages:

| Metric | Numerator | Denominator | Meaning |
| --- | --- | --- | --- |
| Traceability | applicable scenarios with at least one fixture | all applicable scenarios | whether the inventory has executable evidence |
| Native support | scenarios whose referenced fixtures are all `active` | all applicable scenarios | behavior executable and blocking in the native suite |
| Strict JVM parity | active scenarios whose fixtures all use `oracle = "equal"` | all applicable scenarios | compatibility explicitly proven against Clojure/JVM |

The formulas use equal scenario weights:

```text
traceability = traced scenarios / applicable scenarios
native support = active scenarios / applicable scenarios
strict JVM parity = active equal-oracle scenarios / applicable scenarios
remaining = applicable scenarios - covered scenarios
```

Complexity estimates, issue state, lines of implementation, and developer judgment
do not change the percentages. One scenario contributes one unit. Multiple fixtures
may jointly prove a scenario, but they do not increase its weight.

## Capability completion

A capability is:

- `complete` when every applicable scenario is active;
- `partial` when at least one, but not every, applicable scenario is active;
- `missing` when none of its applicable scenarios is active.

Capability completion is reported as a diagnostic distribution, not combined with
the scenario percentage. This avoids assigning arbitrary weights to large and small
reader features.

## Lifecycle interpretation

- `active` contributes to native support.
- `active` plus `oracle = "equal"` contributes to strict JVM parity.
- `expected-diff` remains visible and does not count as strict parity.
- `oracle = "not-applicable"` counts as native evidence but not JVM parity.
- `xfail`, `pending`, and catalog slots without fixtures remain uncovered.
- A closed issue is never implementation evidence by itself.

An existing level-A fixture that is absent from the catalog is a gate failure. A
catalog fixture ID that does not exist, points outside level A, or appears twice
inside one scenario is also a gate failure.

## Current baseline

At the catalog introduction:

| Result | Count | Percentage |
| --- | ---: | ---: |
| Cataloged capabilities | 46 | — |
| Complete / partial / missing capabilities | 12 / 13 / 21 | — |
| Applicable / explicitly excluded scenarios | 129 / 9 | — |
| Traceable scenarios | 62 | 48.06% |
| Native active scenarios | 50 | 38.76% |
| Strict equal-oracle scenarios | 32 | 24.81% |
| Remaining for complete native support | 79 | 61.24% |
| Remaining for strict JVM parity | 97 | 75.19% |

The live command is authoritative; this baseline records the initial denominator and
is not a substitute for rerunning the report.

## Commands and reports

```bash
# Human summary
make reader-syntax-coverage

# Machine-readable stdout
scripts/conformance.sh reader-coverage --json

# Full offline conformance gate; coverage validation runs first
make compatibility
```

Every run writes:

- `target/conformance/reader-syntax-coverage.json`;
- `target/conformance/reader-syntax-coverage.txt`.

`make compatibility` validates the catalog before executing A–E fixtures. The command
does not require a JVM or network connection. The JVM remains a manual oracle used to
promote evidence to strict parity.

## Catalog maintenance

Each `[[capability]]` records:

- stable ID and category;
- representative syntax;
- official source;
- observable behavior;
- tracking issue and dependencies;
- explicit decision;
- fixture IDs for normal, boundary, and error scenarios;
- justified not-applicable scenarios.

Adding official syntax changes the denominator and requires a reviewed catalog diff.
Implementing behavior normally changes a fixture from `xfail` to `active` or fills an
empty scenario with a new fixture ID. Changing only prose cannot improve coverage.
