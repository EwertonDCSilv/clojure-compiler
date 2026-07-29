# Exercism external comparison

[Suite guide](../README.md) ·
[Upstream attribution](../UPSTREAM.md) ·
[Implementation plan](../IMPLEMENTATION_PLAN.md) ·
[Practice matrix](compilation.tsv) ·
[Conformance corpus](../../../tests/conformance/level-d-pure-libraries/external/exercism/README.md)

Files:

- [`compilation.tsv`](compilation.tsv): build result for all 101 upstream reference
  solutions;
- [`all-files.tsv`](all-files.tsv): individual build result for all 493 `.clj`/`.cljc`
  files in the checkout;
- [`extreme.csv`](extreme.csv): current Native × Clojure/JVM comparison for all eight
  promoted workloads.

The concept compilation matrix is stored with the conformance fixtures at
[`compilation.tsv`](../../../tests/conformance/level-d-pure-libraries/external/exercism/compilation.tsv).
It is a compatibility inventory and is not part of this performance report.

## Snapshots

| Component | Revision |
| --- | --- |
| Compiler checkout used for performance | `a1ecebd` |
| Measurement date | 2026-07-29 |
| [`exercism/clojure`](https://github.com/exercism/clojure) | [`4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190`](https://github.com/exercism/clojure/tree/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190) |
| Clojure/JVM | 1.12.5, AOT with HotSpot enabled |
| Internal load multiplier | 25× |

The performance CSV contains per-case medians from ten complete rounds. The
compatibility inventory remains associated with its separately documented upstream
and compiler snapshots.

## Environment

| Item | Value |
| --- | --- |
| Architecture | x86_64 |
| System | Linux 6.14.0-37-generic |
| CPU | AMD Ryzen 5 8600G, 6 cores / 12 threads |
| Memory | 30.5 GiB |
| Rust | rustc 1.97.1 |
| C compiler | GCC 14.2.0 |
| Java | OpenJDK 21.0.9 |
| Native build | Cargo `--release`, default Cranelift level |

## Compatibility result

- 101 public reference implementations compiled.
- 7 passed `clojure-native build`.
- 94 failed at a tracked first blocker.
- All 13 official concept exemplars are versioned as conformance cases; 3 compile, while
  the stricter executable state is 2 active and 11 expected failures.
- Across complete official solutions, the current compiler inventory is 10/114 passing.
- The complete checkout contains 493 Clojure files: 117 build individually and 376
  fail. This includes 101 practice references, 13 concept exemplars, 120 source/stub
  files, 114 tests, 113 `project.clj` files, 30 generators and 2 tooling files.
- For practice references, the largest first-blocker groups are missing core functions
  (`25`), regex (`16`), unavailable namespaces/other compiler gaps (`16`) and missing
  core macros (`8`).

## Performance result

- 8/8 workloads produced identical Native and JVM checksums.
- Accumulated wall time: Native `35.05 s`, JVM `8.48 s`.
- Accumulated CPU time: Native `35.00 s`, JVM `12.47 s`.
- Native used less wall time in 1 case and less CPU in 3 cases.
- Native used less RSS in 7 cases. `knapsack` remains the exception at
  `1,736.2 MiB` natively versus `396.5 MiB` on the JVM.
- Median RSS: Native `7.7 MiB`, JVM `430.9 MiB`.
- `prime-factors` remains the main native deficit: `18.84 s` versus `3.41 s` wall time.
- The newly promoted `annalyns-infiltration` workload completed with matching
  checksum; Native used `5.58 s` versus `1.26 s` JVM wall time.

The accumulated wall result favors the JVM because `prime-factors`, `knapsack`, and
`annalyns-infiltration` dominate this small suite. Median memory favors Native. This
is an engineering baseline, not a general performance claim. The older 5× snapshot
must not be compared numerically with this 25× result.

## Charts

### Wall time

[![Wall-time comparison](charts/wall-time.svg)](charts/wall-time.svg)

### Total CPU

[![CPU comparison](charts/cpu-time.svg)](charts/cpu-time.svg)

### Peak RSS

[![RSS comparison](charts/memory-rss.svg)](charts/memory-rss.svg)

## Per-case summary

`N/J` means Native/Clojure JVM. Delta is `(Native - JVM) / JVM`: negative values favor
Native.

| Caso | Tempo N/J (s) | Δ tempo | CPU N/J (s) | Δ CPU | RSS N/J (MiB) | Δ RSS |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `01-practice/01-accumulate.clj` | 0.90 / 0.70 | +27.7% | 0.90 / 1.19 | -24.4% | 8.1 / 465.3 | -98.3% |
| `01-practice/02-binary-search.clj` | 0.58 / 0.46 | +26.1% | 0.58 / 0.88 | -34.1% | 1.4 / 232.8 | -99.4% |
| `01-practice/03-hello-world.clj` | 0.79 / 0.33 | +139.4% | 0.79 / 0.71 | +10.5% | 7.5 / 94.8 | -92.1% |
| `01-practice/04-knapsack.clj` | 5.26 / 1.14 | +363.4% | 5.25 / 1.79 | +192.8% | 1736.2 / 396.5 | +337.8% |
| `01-practice/05-prime-factors.clj` | 18.84 / 3.41 | +452.6% | 18.82 / 4.00 | +371.1% | 15.8 / 961.9 | -98.4% |
| `01-practice/06-square-root.clj` | 0.23 / 0.36 | -36.1% | 0.23 / 0.78 | -70.3% | 1.4 / 115.6 | -98.8% |
| `01-practice/07-two-fer.clj` | 2.87 / 0.82 | +250.0% | 2.86 / 1.34 | +113.4% | 7.9 / 467.8 | -98.3% |
| `02-concept/01-annalyns-infiltration.clj` | 5.58 / 1.26 | +342.9% | 5.57 / 1.78 | +211.8% | 1.4 / 593.2 | -99.8% |
