# External Exercism benchmark and compatibility corpus

[Central benchmark catalog](../README.md) ·
[Upstream attribution](UPSTREAM.md) ·
[Implementation plan](IMPLEMENTATION_PLAN.md) ·
[Compilation matrix](results/compilation.tsv) ·
[Complete checkout matrix](results/all-files.tsv) ·
[Comparative results](results/README.md)

This suite uses public reference solutions from
[`exercism/clojure`](https://github.com/exercism/clojure) as an external,
independently maintained corpus for the native compiler.

The audited upstream snapshot is the official
[`exercism/clojure`](https://github.com/exercism/clojure) repository at commit
[`4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190`](https://github.com/exercism/clojure/tree/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190),
licensed under the
[MIT License](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/LICENSE).
See the complete file-level mapping and separation between upstream and local changes
in [`UPSTREAM.md`](UPSTREAM.md).

The upstream repository is cloned separately instead of becoming a Git submodule. The
default location is `~/github/exercism-clojure`, configurable through
`EXERCISM_CLOJURE_CHECKOUT` or `--checkout`.

## Current coverage

All 101 files at `exercises/practice/*/.meta/example.clj` were passed directly to
`clojure-native build`.

| Result | Cases | Meaning |
| --- | ---: | --- |
| `PASS` | 7 | The unmodified upstream reference file builds natively |
| `FAIL` | 94 | The first compiler blocker is recorded |
| **Total** | **101** | Complete reference-solution inventory in the pinned snapshot |

The literal whole-checkout audit also compiles every `.clj` and `.cljc` file:

| Role | Total | Pass | Fail |
| --- | ---: | ---: | ---: |
| Reference solutions | 101 | 7 | 94 |
| Exercise source/stubs | 120 | 105 | 15 |
| Tests | 114 | 0 | 114 |
| `project.clj` manifests | 113 | 0 | 113 |
| Generators | 30 | 3 | 27 |
| Other tooling | 15 | 1 | 14 |
| **Whole checkout** | **493** | **116** | **377** |

Test and project files are expected to expose `clojure.test`, dependency and project
loading gaps; they are reported separately from complete reference implementations.

The seven passing implementations are:

- `accumulate`
- `binary-search`
- `hello-world`
- `knapsack`
- `prime-factors`
- `square-root`
- `two-fer`

The complete result, including error code, category, source line, first diagnostic and
upstream commit, is versioned in
[`results/compilation.tsv`](results/compilation.tsv). A failure category represents
only the **first blocker**. Removing it may reveal another unsupported construct in the
same exercise.

## Reproduce the 101-file audit

From the compiler repository root:

```bash
make exercism-compatibility

# Explicit checkout or output
benchmarks/exercism/compile-all.sh \
  --checkout ~/github/exercism-clojure \
  --report /tmp/exercism-compilation.tsv

# Compile every .clj/.cljc in the checkout
benchmarks/exercism/compile-all.sh \
  --scope all \
  --report /tmp/exercism-all-files.tsv

# Make any unsupported exercise fail the command
benchmarks/exercism/compile-all.sh --strict
```

`make exercism-compatibility` updates both the 101-reference matrix and the 493-file
whole-checkout matrix. Ordinary compatibility gaps are written to the report without
failing the audit. `--strict` is intended for the future 101/101 reference gate.

## Benchmark subset

The seven passing reference implementations have deterministic workload adapters under
[`01-practice/`](01-practice/). The upstream implementation remains intact; each file
adds only fixed input data, a `benchmark` function and a `-main` checksum entry point.
The copied portions remain attributed to Exercism and its contributors and retain the
upstream MIT terms in [`LICENSE.exercism`](LICENSE.exercism). Exact source links for
each fixture are recorded in [`UPSTREAM.md`](UPSTREAM.md).

| Case | Main pressure |
| --- | --- |
| `accumulate` | closures, vectors and repeated persistent construction |
| `binary-search` | indexed lookup, comparisons and tight loops |
| `hello-world` | direct calls and string equality |
| `knapsack` | recursive branching, maps and GC pressure |
| `prime-factors` | integer arithmetic, `mod`, `quot` and vector growth |
| `square-root` | integer multiplication and loop throughput |
| `two-fer` | variadic arity dispatch and string allocation |

Run the native validation:

```bash
make benchmarks-exercism
```

Compare Native and Clojure/JVM AOT:

```bash
make benchmarks-compare-exercism

# Reproduce the versioned reference load
make benchmarks-compare-exercism \
  EXERCISM_COMPARE_ARGS="--scale 5" \
  EXERCISM_COMPARISON_CSV=benchmarks/exercism/results/extreme.csv
```

The CSV schema, checksum rule and GNU `time` metrics are identical to the Cracking and
Cormen suites. Compilation is measured separately from execution. The JVM side uses
Clojure 1.12.5 and keeps HotSpot enabled after AOT compilation.

## Promotion workflow

The external corpus is deliberately incremental:

1. implement one planned compatibility slice;
2. add unit and integration coverage for that slice;
3. rerun `compile-all.sh`;
4. inspect every exercise that changed from `FAIL` to `PASS`;
5. add semantic inputs and compare the result with Clojure/JVM;
6. add a deterministic performance adapter only when the exercise creates a useful,
   non-duplicated workload;
7. update the matrix, plan and comparative report in the same change.

This prevents a file that merely parses or defines unused functions from being counted
as a completed semantic or performance test.
