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
| Compiler checkout used for performance and compatibility | `1dc69b5b126c193c30e9f24fdddd549abb7ce4cb` |
| Measurement date | 2026-07-28 |
| [`exercism/clojure`](https://github.com/exercism/clojure) | [`4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190`](https://github.com/exercism/clojure/tree/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190) |
| Clojure/JVM | 1.12.5, AOT with HotSpot enabled |
| Internal load multiplier | 5× |

The documentation/results commit is necessarily newer than the measured compiler
baseline.

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
- All 13 official concept exemplars are versioned as conformance cases; 1 is active
  and 12 are expected failures with tracked first blockers.
- Across complete official solutions, the current inventory is 8/114 passing.
- The complete checkout contains 493 Clojure files: 116 build individually and 377
  fail. This includes 101 practice references, 13 concept exemplars, 120 source/stub
  files, 114 tests, 113 `project.clj` files, 30 generators and 2 tooling files.
- The largest first-blocker groups are top-level values (`20`), missing core functions
  (`16`), regex (`16`) and missing core macros (`11`).

## Performance result

- 8/8 workloads produced identical Native and JVM checksums.
- Accumulated wall time: Native `6.68 s`, JVM `4.22 s`.
- Accumulated CPU time: Native `6.66 s`, JVM `8.00 s`.
- Native used less wall time in 4 cases and less CPU in 6 cases.
- Native used less RSS in 7 cases. `knapsack` remains the exception at
  `351.2 MiB` natively versus `258.5 MiB` on the JVM.
- Median RSS: Native `7.8 MiB`, JVM `249.6 MiB`.
- `prime-factors` remains the main native deficit: `3.55 s` versus `1.16 s` wall
  time.
- The newly promoted `annalyns-infiltration` workload completed with matching
  checksum; Native used `1.08 s` versus `0.60 s` JVM wall time.

The accumulated wall result favors the JVM because `prime-factors`, `knapsack`, and
`annalyns-infiltration` dominate this small suite. Accumulated CPU and median memory
favor Native. This is an engineering baseline, not a general performance claim.

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
| `01-practice/01-accumulate.clj` | 0.16 / 0.44 | -63.6% | 0.16 / 0.92 | -82.6% | 8.2 / 240.8 | -96.6% |
| `01-practice/02-binary-search.clj` | 0.11 / 0.35 | -68.6% | 0.11 / 0.75 | -85.3% | 1.4 / 115.1 | -98.7% |
| `01-practice/03-hello-world.clj` | 0.18 / 0.31 | -41.9% | 0.17 / 0.71 | -76.1% | 7.6 / 97.6 | -92.2% |
| `01-practice/04-knapsack.clj` | 1.02 / 0.55 | +85.5% | 1.02 / 1.14 | -10.5% | 351.2 / 258.5 | +35.9% |
| `01-practice/05-prime-factors.clj` | 3.55 / 1.16 | +206.0% | 3.55 / 1.70 | +108.8% | 16.1 / 1055.3 | -98.5% |
| `01-practice/06-square-root.clj` | 0.04 / 0.32 | -87.5% | 0.04 / 0.74 | -94.6% | 1.4 / 108.7 | -98.7% |
| `01-practice/07-two-fer.clj` | 0.54 / 0.49 | +10.2% | 0.53 / 0.99 | -46.5% | 7.9 / 371.4 | -97.9% |
| `02-concept/01-annalyns-infiltration.clj` | 1.08 / 0.60 | +80.0% | 1.08 / 1.05 | +2.9% | 1.4 / 371.4 | -99.6% |

Reproduce:

```bash
benchmarks/exercism/compare-clojure.sh \
  --scale 5 \
  --csv benchmarks/exercism/results/extreme.csv
```
