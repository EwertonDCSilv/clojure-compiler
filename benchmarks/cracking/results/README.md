# Comparação extrema de referência

[Catálogo dos benchmarks](../../README.md) ·
[Guia da suíte](../README.md)

Arquivo: [`extreme.csv`](extreme.csv)

Snapshot do relatório:
[`HEAD 1dc69b5`](https://github.com/EwertonDCSilv/clojure-compiler/commit/1dc69b5b126c193c30e9f24fdddd549abb7ce4cb).

Medições Native × Clojure/JVM refeitas em 2026-07-28 no commit `1dc69b5` com:

```bash
benchmarks/cracking/compare-clojure.sh --scale 25 \
  --csv benchmarks/cracking/results/extreme.csv
```

Os dois lados foram recompilados e executados nesta rodada. O runner aceitou métricas
somente após validar exit status e igualdade dos checksums.

## Ambiente

| Item | Valor |
| --- | --- |
| Arquitetura | x86_64 |
| Sistema | Linux 6.14.0-37-generic |
| CPU | AMD Ryzen 5 8600G, 6 cores / 12 threads |
| Memória | 30 GiB |
| Rust | rustc 1.97.1 |
| Compilador C | GCC 14.2.0 |
| Java | OpenJDK 21.0.9 |
| Clojure/JVM | Clojure 1.12.5, AOT |
| Build do compilador | Cargo `--release` |
| Multiplicador interno | 25× |
| GC | configuração normal |

## Resumo desta execução

- 60 casos comparados, todos com status `OK` e checksums idênticos.
- O nativo teve menor tempo de parede em 57 casos; Clojure/JVM em 3.
- Mediana de `wall_speedup_vs_clojure`: 3,350× entre os casos mensuráveis.
- Tempos de parede acumulados: nativo 7,71 s; Clojure/JVM 22,27 s.
- Tempos de CPU acumulados: nativo 7,58 s; Clojure/JVM 48,66 s.
- O nativo teve menor tempo de CPU nos 60 casos.
- Mediana de `cpu_speedup_vs_clojure`: 7,650× entre os casos mensuráveis.
- O nativo apresentou RSS menor em 59 dos 60 casos.
- Mediana de `rss_ratio_clojure_over_native`: 33,522×.
- Maior RSS nativo: 194,1 MiB em
  `07-object-oriented-design/06-record-updates.clj`.
- Maior RSS Clojure/JVM: 533,2 MiB em
  `06-math-and-logic/05-integer-square-root.clj`.
- Compilação acumulada: 10.484 ms no nativo e 28.513 ms no Clojure/JVM AOT.

Esta é uma execução completa única. Em relação ao snapshot anterior, o agregado nativo
permaneceu estável (7,77 → 7,71 s de parede; 7,61 → 7,58 s de CPU). Como a JVM também
foi medida novamente, diferenças de frequência, JIT e carga da máquina devem ser
tratadas como variação da rodada, não como regressão ou ganho isolado.

## Gráficos comparativos

Os gráficos são gerados diretamente do [`extreme.csv`](extreme.csv) por
[`render-benchmark-charts.rs`](../../render-benchmark-charts.rs). Tempo e CPU usam uma
escala logarítmica de razão: verde à direita favorece o nativo e laranja à esquerda
favorece Clojure/JVM. Memória mostra os valores absolutos dos dois processos em MiB.

### Tempo de parede

[![Comparação do tempo de parede por caso](charts/wall-time.svg)](charts/wall-time.svg)

### Tempo total de CPU

[![Comparação do tempo total de CPU por caso](charts/cpu-time.svg)](charts/cpu-time.svg)

### Pico de memória RSS

[![Comparação do pico de memória RSS por caso](charts/memory-rss.svg)](charts/memory-rss.svg)

## Resumo por teste

`N/J` mostra os valores absolutos nativo/Clojure. O delta é
`(nativo - Clojure) / Clojure`: negativo favorece o nativo; positivo favorece a JVM.

| Caso | Tempo N/J (s) | Δ tempo | CPU N/J (s) | Δ CPU | RSS N/J (MiB) | Δ RSS |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `01-arrays-and-strings/01-vector-sum.clj` | 0.00 / 0.31 | -100.0% | 0.00 / 0.70 | -100.0% | 1.4 / 99.7 | -98.6% |
| `01-arrays-and-strings/02-reverse-vector.clj` | 0.02 / 0.33 | -93.9% | 0.02 / 0.78 | -97.4% | 16.1 / 117.9 | -86.4% |
| `01-arrays-and-strings/03-rotate-left.clj` | 0.03 / 0.33 | -90.9% | 0.02 / 0.79 | -97.5% | 10.4 / 116.5 | -91.0% |
| `01-arrays-and-strings/04-compact-adjacent.clj` | 0.05 / 0.33 | -84.8% | 0.05 / 0.74 | -93.2% | 16.1 / 115.7 | -86.1% |
| `01-arrays-and-strings/05-matrix-diagonals.clj` | 0.01 / 0.32 | -96.9% | 0.01 / 0.73 | -98.6% | 1.4 / 94.8 | -98.5% |
| `01-arrays-and-strings/06-rolling-hash.clj` | 0.00 / 0.33 | -100.0% | 0.00 / 0.74 | -100.0% | 1.4 / 114.7 | -98.7% |
| `02-linked-lists/01-remove-value.clj` | 0.03 / 0.34 | -91.2% | 0.03 / 0.78 | -96.2% | 4.4 / 118.7 | -96.3% |
| `02-linked-lists/02-kth-from-end.clj` | 0.02 / 0.32 | -93.8% | 0.02 / 0.73 | -97.3% | 1.4 / 96.3 | -98.5% |
| `02-linked-lists/03-stable-partition.clj` | 0.04 / 0.37 | -89.2% | 0.04 / 0.90 | -95.6% | 4.7 / 129.3 | -96.4% |
| `02-linked-lists/04-reversed-digits.clj` | 0.10 / 0.40 | -75.0% | 0.09 / 0.85 | -89.4% | 4.8 / 210.9 | -97.7% |
| `02-linked-lists/05-palindrome-list.clj` | 0.05 / 0.33 | -84.8% | 0.05 / 0.75 | -93.3% | 4.7 / 114.9 | -95.9% |
| `02-linked-lists/06-merge-sorted.clj` | 0.02 / 0.33 | -93.9% | 0.01 / 0.80 | -98.8% | 4.6 / 105.4 | -95.7% |
| `03-stacks-and-queues/01-stack-drain.clj` | 0.03 / 0.33 | -90.9% | 0.03 / 0.73 | -95.9% | 4.6 / 128.0 | -96.4% |
| `03-stacks-and-queues/02-min-stack.clj` | 0.11 / 0.33 | -66.7% | 0.10 / 0.73 | -86.3% | 16.3 / 114.4 | -85.7% |
| `03-stacks-and-queues/03-queue-two-stacks.clj` | 0.32 / 0.39 | -17.9% | 0.31 / 0.89 | -65.2% | 14.3 / 203.0 | -92.9% |
| `03-stacks-and-queues/04-balanced-tokens.clj` | 0.06 / 0.32 | -81.2% | 0.06 / 0.73 | -91.8% | 1.4 / 95.9 | -98.5% |
| `03-stacks-and-queues/05-monotonic-spans.clj` | 0.02 / 0.31 | -93.5% | 0.01 / 0.71 | -98.6% | 1.5 / 97.9 | -98.5% |
| `03-stacks-and-queues/06-round-robin.clj` | 0.19 / 0.49 | -61.2% | 0.19 / 0.95 | -80.0% | 4.8 / 309.8 | -98.4% |
| `04-trees-and-graphs/01-tree-height.clj` | 0.17 / 0.32 | -46.9% | 0.16 / 0.72 | -77.8% | 5.2 / 95.3 | -94.6% |
| `04-trees-and-graphs/02-bst-search.clj` | 0.04 / 0.32 | -87.5% | 0.04 / 0.71 | -94.4% | 1.4 / 95.1 | -98.5% |
| `04-trees-and-graphs/03-level-sums.clj` | 0.03 / 0.32 | -90.6% | 0.03 / 0.72 | -95.8% | 1.4 / 95.3 | -98.5% |
| `04-trees-and-graphs/04-balanced-tree.clj` | 0.10 / 0.33 | -69.7% | 0.10 / 0.76 | -86.8% | 5.2 / 97.5 | -94.7% |
| `04-trees-and-graphs/05-graph-reachability.clj` | 0.09 / 0.43 | -79.1% | 0.09 / 0.94 | -90.4% | 5.3 / 229.1 | -97.7% |
| `04-trees-and-graphs/06-connected-components.clj` | 0.13 / 0.42 | -69.0% | 0.13 / 0.90 | -85.6% | 6.6 / 208.8 | -96.9% |
| `05-bit-manipulation/01-popcount.clj` | 0.22 / 0.45 | -51.1% | 0.22 / 0.89 | -75.3% | 1.4 / 232.1 | -99.4% |
| `05-bit-manipulation/02-bit-parity.clj` | 0.45 / 0.62 | -27.4% | 0.45 / 1.08 | -58.3% | 1.5 / 364.0 | -99.6% |
| `05-bit-manipulation/03-hamming-distance.clj` | 0.18 / 0.43 | -58.1% | 0.18 / 0.87 | -79.3% | 1.4 / 232.7 | -99.4% |
| `05-bit-manipulation/04-reverse-low-bits.clj` | 0.09 / 0.36 | -75.0% | 0.09 / 0.78 | -88.5% | 1.4 / 121.4 | -98.8% |
| `05-bit-manipulation/05-power-of-two.clj` | 0.44 / 0.39 | +12.8% | 0.44 / 0.80 | -45.0% | 1.4 / 232.8 | -99.4% |
| `05-bit-manipulation/06-insert-bit-field.clj` | 0.04 / 0.33 | -87.9% | 0.04 / 0.77 | -94.8% | 1.4 / 117.3 | -98.8% |
| `06-math-and-logic/01-euclidean-gcd.clj` | 0.30 / 0.50 | -40.0% | 0.30 / 0.94 | -68.1% | 1.4 / 371.1 | -99.6% |
| `06-math-and-logic/02-least-common-multiple.clj` | 0.13 / 0.37 | -64.9% | 0.13 / 0.79 | -83.5% | 1.4 / 161.1 | -99.1% |
| `06-math-and-logic/03-prime-count.clj` | 0.07 / 0.33 | -78.8% | 0.07 / 0.75 | -90.7% | 1.4 / 109.3 | -98.7% |
| `06-math-and-logic/04-factorial-trailing-zeros.clj` | 0.04 / 0.39 | -89.7% | 0.04 / 0.80 | -95.0% | 1.4 / 236.8 | -99.4% |
| `06-math-and-logic/05-integer-square-root.clj` | 0.28 / 0.59 | -52.5% | 0.28 / 1.05 | -73.3% | 1.5 / 533.2 | -99.7% |
| `06-math-and-logic/06-modular-power.clj` | 0.24 / 0.49 | -51.0% | 0.24 / 0.94 | -74.5% | 1.4 / 363.3 | -99.6% |
| `07-object-oriented-design/01-point-record.clj` | 0.08 / 0.33 | -75.8% | 0.08 / 0.74 | -89.2% | 58.3 / 107.3 | -45.7% |
| `07-object-oriented-design/02-shape-protocol.clj` | 0.10 / 0.34 | -70.6% | 0.10 / 0.77 | -87.0% | 59.8 / 116.1 | -48.5% |
| `07-object-oriented-design/03-payroll-protocol.clj` | 0.06 / 0.33 | -81.8% | 0.06 / 0.77 | -92.2% | 27.6 / 113.1 | -75.6% |
| `07-object-oriented-design/04-card-records.clj` | 0.20 / 0.32 | -37.5% | 0.19 / 0.74 | -74.3% | 65.9 / 108.0 | -38.9% |
| `07-object-oriented-design/05-file-tree-protocol.clj` | 0.09 / 0.35 | -74.3% | 0.08 / 0.87 | -90.8% | 45.8 / 128.7 | -64.4% |
| `07-object-oriented-design/06-record-updates.clj` | 0.40 / 0.34 | +17.6% | 0.40 / 0.76 | -47.4% | 194.1 / 117.2 | +65.5% |
| `08-recursion-and-dp/01-fibonacci.clj` | 0.17 / 0.34 | -50.0% | 0.17 / 0.76 | -77.6% | 1.4 / 121.5 | -98.8% |
| `08-recursion-and-dp/02-staircase-ways.clj` | 0.03 / 0.32 | -90.6% | 0.03 / 0.68 | -95.6% | 1.4 / 96.3 | -98.5% |
| `08-recursion-and-dp/03-grid-paths.clj` | 0.14 / 0.39 | -64.1% | 0.14 / 0.82 | -82.9% | 18.1 / 232.2 | -92.2% |
| `08-recursion-and-dp/04-coin-change.clj` | 0.25 / 0.49 | -49.0% | 0.24 / 0.97 | -75.3% | 20.3 / 364.7 | -94.4% |
| `08-recursion-and-dp/05-longest-increasing-subsequence.clj` | 0.12 / 0.33 | -63.6% | 0.12 / 0.79 | -84.8% | 8.3 / 117.4 | -92.9% |
| `08-recursion-and-dp/06-subset-sum.clj` | 0.15 / 0.45 | -66.7% | 0.14 / 0.88 | -84.1% | 18.2 / 319.9 | -94.3% |
| `09-sorting-and-searching/01-binary-search.clj` | 0.18 / 0.36 | -50.0% | 0.18 / 0.77 | -76.6% | 1.4 / 130.1 | -98.9% |
| `09-sorting-and-searching/02-insertion-sort.clj` | 0.04 / 0.36 | -88.9% | 0.04 / 0.92 | -95.7% | 4.6 / 124.4 | -96.3% |
| `09-sorting-and-searching/03-bubble-sort-vector.clj` | 0.11 / 0.36 | -69.4% | 0.11 / 0.83 | -86.7% | 18.3 / 169.5 | -89.2% |
| `09-sorting-and-searching/04-merge-sorted-vectors.clj` | 0.05 / 0.35 | -85.7% | 0.04 / 0.82 | -95.1% | 15.4 / 121.0 | -87.2% |
| `09-sorting-and-searching/05-rotated-search.clj` | 0.15 / 0.35 | -57.1% | 0.15 / 0.78 | -80.8% | 1.4 / 120.6 | -98.8% |
| `09-sorting-and-searching/06-frequency-table.clj` | 0.09 / 0.36 | -75.0% | 0.09 / 0.84 | -89.3% | 9.2 / 117.4 | -92.2% |
| `10-moderate-problems/01-maximum-subarray.clj` | 0.35 / 0.31 | +12.9% | 0.35 / 0.70 | -50.0% | 5.2 / 95.5 | -94.6% |
| `10-moderate-problems/02-pair-sum-count.clj` | 0.12 / 0.34 | -64.7% | 0.12 / 0.75 | -84.0% | 1.4 / 99.4 | -98.5% |
| `10-moderate-problems/03-mastermind-score.clj` | 0.19 / 0.38 | -50.0% | 0.19 / 0.92 | -79.3% | 5.8 / 130.1 | -95.5% |
| `10-moderate-problems/04-peak-population.clj` | 0.24 / 0.35 | -31.4% | 0.24 / 0.81 | -70.4% | 5.9 / 122.7 | -95.2% |
| `10-moderate-problems/05-arithmetic-swap.clj` | 0.04 / 0.34 | -88.2% | 0.04 / 0.74 | -94.6% | 1.4 / 169.0 | -99.1% |
| `10-moderate-problems/06-board-lengths.clj` | 0.22 / 0.48 | -54.2% | 0.21 / 0.99 | -78.8% | 8.1 / 284.3 | -97.2% |

## Como ler a comparação

- `wall_speedup_vs_clojure` é tempo Clojure dividido pelo tempo nativo.
- `cpu_speedup_vs_clojure` é CPU Clojure dividida pela CPU nativa.
- `rss_ratio_clojure_over_native` é RSS Clojure dividido pelo RSS nativo.
- Nas três colunas, valores maiores que `1` favorecem o nativo; valores menores que `1`
  favorecem Clojure/JVM.

Os dois caminhos foram compilados antes de suas respectivas execuções medidas: o
binário pelo `clojure-compiler` e o namespace JVM por AOT. Os custos aparecem
separadamente em
`native_compile_wall_ms` e `clojure_compile_wall_ms`.

Os valores são uma fotografia desta máquina; frequência dinâmica, carga do sistema,
toolchain, JIT e sistema operacional afetam o resultado. Para conclusões estatísticas,
repita as medições no mesmo ambiente e compare distribuições, não apenas uma execução.

O compilador release foi reconstruído imediatamente antes da rodada no commit
`1dc69b5`.

## Validação do fast path de multiplicação

Comparação do baseline `18f58cc` com o commit candidato
`ADR-0006: fast path de fixnum para *`, ambos construídos em release e executados com
`--scale 25 --opt-level none`:

- [`mul-fastpath-before.csv`](mul-fastpath-before.csv)
- [`mul-fastpath-after.csv`](mul-fastpath-after.csv)

| Métrica | Antes | Depois | Variação |
| --- | ---: | ---: | ---: |
| Tempo de parede acumulado | 22,57 s | 22,50 s | -0,31% |
| Tempo de CPU acumulado | 22,37 s | 22,22 s | -0,67% |
| Compilação acumulada | 5.216 ms | 5.215 ms | -0,02% |

O resultado global ficou essencialmente estável: 18 casos melhoraram, 17 pioraram e 25
empataram na resolução de 0,01 s do runner. Nos 27 arquivos que contêm multiplicação
direta, o tempo acumulado caiu de 7,53 s para 7,29 s (-3,19%). Os ganhos mais claros
foram `least-common-multiple` (0,34 → 0,29 s), `factorial-trailing-zeros`
(0,23 → 0,20 s) e `modular-power` (0,59 → 0,53 s).
