# External Exercism benchmark

[Central benchmark catalog](../README.md) ·
[Upstream attribution](UPSTREAM.md) ·
[Implementation plan](IMPLEMENTATION_PLAN.md) ·
[Conformance corpus](../../tests/conformance/level-d-pure-libraries/external/exercism/README.md) ·
[Comparative results](results/README.md)

This suite compares execution performance between Native and Clojure/JVM using public
solutions from [`exercism/clojure`](https://github.com/exercism/clojure). It is not the
language-compatibility report.

The audited upstream snapshot is commit
[`4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190`](https://github.com/exercism/clojure/tree/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190),
licensed under the upstream
[MIT License](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/LICENSE).
The exact fixture mapping and local changes are documented in
[`UPSTREAM.md`](UPSTREAM.md).

## Responsibility boundary

| Artifact | Question answered | Gate |
| --- | --- | --- |
| `benchmarks/exercism/` | How fast does an executable workload run in Native versus JVM? | Both sides execute, checksums match, time/CPU/RSS are recorded |
| `tests/conformance/.../external/exercism/` | Which Clojure syntax, semantics and standard-library features are supported? | `active`, `xfail` and `pending` follow the conformance contract |
| `results/compilation.tsv` and `results/all-files.tsv` | What is the first compiler blocker in the pinned upstream checkout? | Inventory only; no performance claim |

A program enters this benchmark only after it executes correctly in both runtimes and
has a deterministic workload. Merely parsing or compiling an upstream file does not
make it a benchmark.

## Benchmark cases

There are eight Native × JVM workloads: seven practice solutions and one concept
solution.

| Case | Main pressure |
| --- | --- |
| `accumulate` | closures, vectors and repeated persistent construction |
| `binary-search` | indexed lookup, comparisons and tight loops |
| `hello-world` | direct calls and string equality |
| `knapsack` | recursive branching, maps and GC pressure |
| `prime-factors` | integer arithmetic, `mod`, `quot` and vector growth |
| `square-root` | integer multiplication and loop throughput |
| `two-fer` | variadic arity dispatch and string allocation |
| `annalyns-infiltration` | boolean branches, truthiness and short function calls |

Every case has a local deterministic adapter, a numeric checksum and a `-main` entry
point. The copied implementation stays intact before the local adapter marker.
Checksums are versioned in [`expected.tsv`](expected.tsv).

Run native validation:

```bash
make benchmarks-exercism
benchmarks/exercism/run.sh --list
```

Compare Native and Clojure/JVM AOT:

```bash
make benchmarks-compare-exercism

make benchmarks-compare-exercism \
  EXERCISM_COMPARE_ARGS="--scale 5" \
  EXERCISM_COMPARISON_CSV=benchmarks/exercism/results/extreme.csv
```

The comparative CSV records wall time, CPU and peak RSS for both runtimes. Compilation
is measured separately from execution. The JVM side uses Clojure 1.12.5 with HotSpot
enabled after AOT compilation. The versioned CSV contains all eight promoted cases,
including `annalyns-infiltration`.

## Compatibility inventory

The external audit remains useful, but it is deliberately reported apart from
performance:

- 101 practice reference implementations:
  [`results/compilation.tsv`](results/compilation.tsv);
- all 493 `.clj` and `.cljc` files in the pinned checkout:
  [`results/all-files.tsv`](results/all-files.tsv);
- 13 official concept exemplars as executable conformance cases:
  [`tests/conformance/.../external/exercism/`](../../tests/conformance/level-d-pure-libraries/external/exercism/).

Current compatibility inventory:

| Corpus | Total | Pass/active | Fail/xfail |
| --- | ---: | ---: | ---: |
| Practice reference solutions | 101 | 7 | 94 |
| Concept conformance cases | 13 | 1 | 12 |
| **Complete official solutions** | **114** | **8** | **106** |
| Whole-checkout files | 493 | 116 | 377 |

These counts describe compiler coverage, not benchmark results. A failure category is
only the first blocker; resolving it may expose another unsupported construct.

The checkout defaults to `~/github/exercism-clojure` and can be changed through
`EXERCISM_CLOJURE_CHECKOUT` or `--checkout`.

```bash
# Refresh practice, concept-conformance and whole-checkout inventories
make exercism-compatibility

# Individual inventories
benchmarks/exercism/compile-all.sh --scope references
benchmarks/exercism/compile-all.sh --scope concepts
benchmarks/exercism/compile-all.sh --scope all
```

The concept scope verifies that the copied exemplar body matches the pinned upstream
source before compiling it. Its report is stored with the conformance corpus, at
[`compilation.tsv`](../../tests/conformance/level-d-pure-libraries/external/exercism/compilation.tsv).

## Promotion workflow

1. implement a planned compatibility slice;
2. add unit and integration coverage for that slice;
3. run the conformance suite and upstream inventories;
4. review every transition from `xfail`/`FAIL` to `active`/`PASS`;
5. compare observable behavior with Clojure/JVM;
6. add a benchmark only when the program provides a useful, non-duplicated workload;
7. record checksum, scale, environment and both revisions.

This repository is not an official Exercism benchmark, and the results do not imply
endorsement by or affiliation with Exercism.
