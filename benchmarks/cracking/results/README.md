# Comparação extrema de referência

[Catálogo dos benchmarks](../../README.md) ·
[Guia da suíte](../README.md)

Arquivo: [`extreme.csv`](extreme.csv)

Snapshot do relatório:
[`HEAD 3e71bc1`](https://github.com/EwertonDCSilv/clojure-compiler/commit/3e71bc1996b689233c80516b4b4aff52259c2cdf).

Medições Native × Clojure/JVM refeitas em 2026-07-28 no commit `3e71bc1` com:

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
- O nativo teve menor tempo de parede em 56 casos; Clojure/JVM em 3; houve um empate.
- Mediana de `wall_speedup_vs_clojure`: 3,450× entre os casos mensuráveis.
- Tempos de parede acumulados: nativo 8,16 s; Clojure/JVM 23,22 s.
- Tempos de CPU acumulados: nativo 8,06 s; Clojure/JVM 50,93 s.
- O nativo teve menor tempo de CPU nos 60 casos.
- Mediana de `cpu_speedup_vs_clojure`: 8,334× entre os casos mensuráveis.
- O nativo apresentou RSS menor em 59 dos 60 casos.
- Mediana de `rss_ratio_clojure_over_native`: 34,085×.
- Maior RSS nativo: 194,2 MiB em
  `07-object-oriented-design/06-record-updates.clj`.
- Maior RSS Clojure/JVM: 524,8 MiB em
  `06-math-and-logic/05-integer-square-root.clj`.
- Compilação acumulada: 11.604 ms no nativo e 30.089 ms no Clojure/JVM AOT.

Esta é uma execução completa única. Em relação ao snapshot anterior, o agregado nativo
subiu de 7,71 para 8,16 s de parede (+5,8%) e de 7,58 para 8,06 s de CPU (+6,3%).
Como a JVM também subiu nesta rodada, diferenças de frequência, JIT e carga da máquina
impedem atribuir o delta isoladamente às mudanças do compilador; a tendência exige
repetições pareadas.

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
| `01-arrays-and-strings/01-vector-sum.clj` | 0.00 / 0.33 | -100.0% | 0.00 / 0.71 | -100.0% | 1.4 / 94.8 | -98.5% |
| `01-arrays-and-strings/02-reverse-vector.clj` | 0.03 / 0.34 | -91.2% | 0.02 / 0.80 | -97.5% | 16.1 / 118.3 | -86.4% |
| `01-arrays-and-strings/03-rotate-left.clj` | 0.03 / 0.35 | -91.4% | 0.03 / 0.84 | -96.4% | 10.2 / 116.4 | -91.2% |
| `01-arrays-and-strings/04-compact-adjacent.clj` | 0.06 / 0.35 | -82.9% | 0.06 / 0.80 | -92.5% | 15.9 / 118.9 | -86.6% |
| `01-arrays-and-strings/05-matrix-diagonals.clj` | 0.01 / 0.33 | -97.0% | 0.01 / 0.75 | -98.7% | 1.6 / 95.5 | -98.3% |
| `01-arrays-and-strings/06-rolling-hash.clj` | 0.00 / 0.33 | -100.0% | 0.00 / 0.73 | -100.0% | 1.5 / 112.8 | -98.7% |
| `02-linked-lists/01-remove-value.clj` | 0.03 / 0.36 | -91.7% | 0.03 / 0.81 | -96.3% | 4.6 / 116.7 | -96.1% |
| `02-linked-lists/02-kth-from-end.clj` | 0.02 / 0.34 | -94.1% | 0.02 / 0.77 | -97.4% | 1.4 / 100.7 | -98.6% |
| `02-linked-lists/03-stable-partition.clj` | 0.04 / 0.39 | -89.7% | 0.03 / 1.02 | -97.1% | 4.6 / 151.0 | -97.0% |
| `02-linked-lists/04-reversed-digits.clj` | 0.10 / 0.41 | -75.6% | 0.10 / 0.87 | -88.5% | 4.8 / 214.1 | -97.7% |
| `02-linked-lists/05-palindrome-list.clj` | 0.06 / 0.35 | -82.9% | 0.06 / 0.80 | -92.5% | 4.7 / 121.1 | -96.1% |
| `02-linked-lists/06-merge-sorted.clj` | 0.02 / 0.36 | -94.4% | 0.01 / 0.82 | -98.8% | 4.6 / 104.3 | -95.6% |
| `03-stacks-and-queues/01-stack-drain.clj` | 0.03 / 0.36 | -91.7% | 0.03 / 0.82 | -96.3% | 4.6 / 134.9 | -96.6% |
| `03-stacks-and-queues/02-min-stack.clj` | 0.17 / 0.34 | -50.0% | 0.17 / 0.79 | -78.5% | 16.4 / 113.6 | -85.5% |
| `03-stacks-and-queues/03-queue-two-stacks.clj` | 0.40 / 0.40 | +0.0% | 0.39 / 0.88 | -55.7% | 13.9 / 200.6 | -93.0% |
| `03-stacks-and-queues/04-balanced-tokens.clj` | 0.06 / 0.34 | -82.4% | 0.06 / 0.77 | -92.2% | 1.4 / 102.4 | -98.6% |
| `03-stacks-and-queues/05-monotonic-spans.clj` | 0.01 / 0.33 | -97.0% | 0.01 / 0.77 | -98.7% | 1.4 / 103.6 | -98.6% |
| `03-stacks-and-queues/06-round-robin.clj` | 0.19 / 0.51 | -62.7% | 0.19 / 1.08 | -82.4% | 4.8 / 321.5 | -98.5% |
| `04-trees-and-graphs/01-tree-height.clj` | 0.17 / 0.33 | -48.5% | 0.17 / 0.72 | -76.4% | 5.2 / 95.0 | -94.5% |
| `04-trees-and-graphs/02-bst-search.clj` | 0.04 / 0.34 | -88.2% | 0.04 / 0.75 | -94.7% | 1.5 / 94.8 | -98.5% |
| `04-trees-and-graphs/03-level-sums.clj` | 0.03 / 0.33 | -90.9% | 0.03 / 0.78 | -96.2% | 1.4 / 103.2 | -98.6% |
| `04-trees-and-graphs/04-balanced-tree.clj` | 0.10 / 0.34 | -70.6% | 0.10 / 0.80 | -87.5% | 5.3 / 97.0 | -94.5% |
| `04-trees-and-graphs/05-graph-reachability.clj` | 0.09 / 0.46 | -80.4% | 0.09 / 1.01 | -91.1% | 5.3 / 237.4 | -97.8% |
| `04-trees-and-graphs/06-connected-components.clj` | 0.13 / 0.44 | -70.5% | 0.13 / 0.97 | -86.6% | 6.7 / 217.2 | -96.9% |
| `05-bit-manipulation/01-popcount.clj` | 0.22 / 0.45 | -51.1% | 0.22 / 0.89 | -75.3% | 1.4 / 235.4 | -99.4% |
| `05-bit-manipulation/02-bit-parity.clj` | 0.47 / 0.64 | -26.6% | 0.47 / 1.12 | -58.0% | 1.4 / 371.3 | -99.6% |
| `05-bit-manipulation/03-hamming-distance.clj` | 0.19 / 0.44 | -56.8% | 0.19 / 0.85 | -77.6% | 1.4 / 231.6 | -99.4% |
| `05-bit-manipulation/04-reverse-low-bits.clj` | 0.09 / 0.35 | -74.3% | 0.09 / 0.78 | -88.5% | 1.4 / 115.9 | -98.8% |
| `05-bit-manipulation/05-power-of-two.clj` | 0.45 / 0.40 | +12.5% | 0.45 / 0.84 | -46.4% | 1.4 / 234.0 | -99.4% |
| `05-bit-manipulation/06-insert-bit-field.clj` | 0.04 / 0.35 | -88.6% | 0.04 / 0.77 | -94.8% | 1.4 / 115.2 | -98.7% |
| `06-math-and-logic/01-euclidean-gcd.clj` | 0.31 / 0.52 | -40.4% | 0.31 / 1.01 | -69.3% | 1.5 / 371.5 | -99.6% |
| `06-math-and-logic/02-least-common-multiple.clj` | 0.14 / 0.39 | -64.1% | 0.14 / 0.84 | -83.3% | 1.4 / 169.1 | -99.1% |
| `06-math-and-logic/03-prime-count.clj` | 0.08 / 0.36 | -77.8% | 0.08 / 0.76 | -89.5% | 1.4 / 108.2 | -98.7% |
| `06-math-and-logic/04-factorial-trailing-zeros.clj` | 0.04 / 0.40 | -90.0% | 0.04 / 0.84 | -95.2% | 1.4 / 231.0 | -99.4% |
| `06-math-and-logic/05-integer-square-root.clj` | 0.29 / 0.60 | -51.7% | 0.29 / 1.10 | -73.6% | 1.4 / 524.8 | -99.7% |
| `06-math-and-logic/06-modular-power.clj` | 0.25 / 0.49 | -49.0% | 0.25 / 0.94 | -73.4% | 1.4 / 363.1 | -99.6% |
| `07-object-oriented-design/01-point-record.clj` | 0.09 / 0.34 | -73.5% | 0.09 / 0.78 | -88.5% | 58.3 / 108.0 | -46.0% |
| `07-object-oriented-design/02-shape-protocol.clj` | 0.10 / 0.35 | -71.4% | 0.09 / 0.81 | -88.9% | 59.8 / 117.6 | -49.1% |
| `07-object-oriented-design/03-payroll-protocol.clj` | 0.06 / 0.34 | -82.4% | 0.06 / 0.79 | -92.4% | 27.6 / 113.6 | -75.7% |
| `07-object-oriented-design/04-card-records.clj` | 0.21 / 0.33 | -36.4% | 0.21 / 0.76 | -72.4% | 65.8 / 102.4 | -35.7% |
| `07-object-oriented-design/05-file-tree-protocol.clj` | 0.10 / 0.38 | -73.7% | 0.09 / 0.93 | -90.3% | 45.8 / 128.2 | -64.3% |
| `07-object-oriented-design/06-record-updates.clj` | 0.42 / 0.35 | +20.0% | 0.41 / 0.82 | -50.0% | 194.2 / 125.1 | +55.2% |
| `08-recursion-and-dp/01-fibonacci.clj` | 0.17 / 0.35 | -51.4% | 0.17 / 0.73 | -76.7% | 1.4 / 111.3 | -98.7% |
| `08-recursion-and-dp/02-staircase-ways.clj` | 0.03 / 0.34 | -91.2% | 0.03 / 0.74 | -95.9% | 1.4 / 98.3 | -98.5% |
| `08-recursion-and-dp/03-grid-paths.clj` | 0.16 / 0.41 | -61.0% | 0.15 / 0.85 | -82.4% | 18.1 / 230.9 | -92.2% |
| `08-recursion-and-dp/04-coin-change.clj` | 0.26 / 0.51 | -49.0% | 0.26 / 1.01 | -74.3% | 20.4 / 364.7 | -94.4% |
| `08-recursion-and-dp/05-longest-increasing-subsequence.clj` | 0.12 / 0.35 | -65.7% | 0.12 / 0.83 | -85.5% | 8.3 / 116.4 | -92.8% |
| `08-recursion-and-dp/06-subset-sum.clj` | 0.20 / 0.45 | -55.6% | 0.20 / 0.92 | -78.3% | 18.2 / 318.8 | -94.3% |
| `09-sorting-and-searching/01-binary-search.clj` | 0.19 / 0.38 | -50.0% | 0.19 / 0.82 | -76.8% | 1.4 / 135.5 | -98.9% |
| `09-sorting-and-searching/02-insertion-sort.clj` | 0.04 / 0.37 | -89.2% | 0.04 / 0.92 | -95.7% | 4.5 / 122.4 | -96.3% |
| `09-sorting-and-searching/03-bubble-sort-vector.clj` | 0.13 / 0.37 | -64.9% | 0.13 / 0.81 | -84.0% | 18.3 / 160.8 | -88.6% |
| `09-sorting-and-searching/04-merge-sorted-vectors.clj` | 0.06 / 0.37 | -83.8% | 0.05 / 0.89 | -94.4% | 15.4 / 120.9 | -87.2% |
| `09-sorting-and-searching/05-rotated-search.clj` | 0.15 / 0.37 | -59.5% | 0.15 / 0.82 | -81.7% | 1.4 / 123.0 | -98.8% |
| `09-sorting-and-searching/06-frequency-table.clj` | 0.10 / 0.39 | -74.4% | 0.09 / 0.90 | -90.0% | 9.2 / 116.9 | -92.1% |
| `10-moderate-problems/01-maximum-subarray.clj` | 0.36 / 0.33 | +9.1% | 0.36 / 0.74 | -51.4% | 5.3 / 94.5 | -94.4% |
| `10-moderate-problems/02-pair-sum-count.clj` | 0.13 / 0.36 | -63.9% | 0.13 / 0.80 | -83.8% | 1.4 / 94.5 | -98.5% |
| `10-moderate-problems/03-mastermind-score.clj` | 0.19 / 0.40 | -52.5% | 0.19 / 0.98 | -80.6% | 6.0 / 133.5 | -95.5% |
| `10-moderate-problems/04-peak-population.clj` | 0.24 / 0.37 | -35.1% | 0.24 / 0.83 | -71.1% | 6.1 / 114.2 | -94.7% |
| `10-moderate-problems/05-arithmetic-swap.clj` | 0.04 / 0.36 | -88.9% | 0.04 / 0.82 | -95.1% | 1.4 / 177.6 | -99.2% |
| `10-moderate-problems/06-board-lengths.clj` | 0.22 / 0.50 | -56.0% | 0.22 / 1.03 | -78.6% | 8.1 / 284.1 | -97.2% |

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
`3e71bc1`.

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
