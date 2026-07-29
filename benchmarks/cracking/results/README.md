# Comparação extrema de referência

[Catálogo dos benchmarks](../../README.md) ·
[Guia da suíte](../README.md)

Arquivo: [`extreme.csv`](extreme.csv)

Snapshot do relatório:
[`HEAD a1ecebd`](https://github.com/EwertonDCSilv/clojure-compiler/commit/a1ecebd).

Medições Native × Clojure/JVM refeitas em 2026-07-29 no commit `a1ecebd` como
parte de:

```bash
make benchmark-page-refresh
```

Os dois lados foram recompilados e executados em dez rodadas completas. O agregador
publicou a mediana de cada caso somente após validar esquema, escala, versão, status e
igualdade dos checksums em todas as amostras.

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
- Mediana de `wall_speedup_vs_clojure`: 3,393× entre os casos mensuráveis.
- Tempos de parede acumulados: nativo 8,23 s; Clojure/JVM 23,18 s.
- Tempos de CPU acumulados: nativo 8,06 s; Clojure/JVM 49,68 s.
- O nativo teve menor tempo de CPU nos 60 casos.
- Mediana de `cpu_speedup_vs_clojure`: 8,069× entre os casos mensuráveis.
- O nativo apresentou RSS menor em 59 dos 60 casos.
- Mediana de `rss_ratio_clojure_over_native`: 34,014×.
- Maior RSS nativo: 193,8 MiB em
  `07-object-oriented-design/06-record-updates.clj`.
- Maior RSS Clojure/JVM: 498,0 MiB em
  `06-math-and-logic/05-integer-square-root.clj`.
- Compilação acumulada: 13.060 ms no nativo e 30.064 ms no Clojure/JVM AOT.

Em relação ao artefato publicado imediatamente anterior, a mediana reduziu o agregado
nativo de 8,46 para 8,23 s de parede (-2,7%) e de 8,26 para 8,06 s de CPU (-2,4%).
Diferenças de frequência, JIT e carga da máquina ainda impedem atribuir o delta
isoladamente às mudanças do compilador.

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
| `01-arrays-and-strings/01-vector-sum.clj` | 0.00 / 0.34 | -100.0% | 0.00 / 0.72 | -100.0% | 1.4 / 95.3 | -98.5% |
| `01-arrays-and-strings/02-reverse-vector.clj` | 0.03 / 0.34 | -91.2% | 0.02 / 0.74 | -97.3% | 16.0 / 110.1 | -85.5% |
| `01-arrays-and-strings/03-rotate-left.clj` | 0.03 / 0.35 | -91.4% | 0.02 / 0.82 | -97.6% | 10.4 / 116.4 | -91.1% |
| `01-arrays-and-strings/04-compact-adjacent.clj` | 0.06 / 0.34 | -82.6% | 0.05 / 0.76 | -93.4% | 16.1 / 116.0 | -86.1% |
| `01-arrays-and-strings/05-matrix-diagonals.clj` | 0.01 / 0.34 | -97.0% | 0.01 / 0.72 | -98.6% | 1.5 / 95.5 | -98.4% |
| `01-arrays-and-strings/06-rolling-hash.clj` | 0.00 / 0.34 | -100.0% | 0.00 / 0.74 | -100.0% | 1.5 / 118.2 | -98.8% |
| `02-linked-lists/01-remove-value.clj` | 0.03 / 0.35 | -91.5% | 0.03 / 0.80 | -96.2% | 4.5 / 116.8 | -96.1% |
| `02-linked-lists/02-kth-from-end.clj` | 0.02 / 0.34 | -94.1% | 0.02 / 0.74 | -97.3% | 1.5 / 96.1 | -98.5% |
| `02-linked-lists/03-stable-partition.clj` | 0.04 / 0.40 | -90.0% | 0.04 / 1.01 | -96.0% | 4.6 / 151.0 | -96.9% |
| `02-linked-lists/04-reversed-digits.clj` | 0.10 / 0.41 | -75.9% | 0.10 / 0.86 | -89.0% | 4.8 / 213.0 | -97.8% |
| `02-linked-lists/05-palindrome-list.clj` | 0.06 / 0.35 | -82.9% | 0.06 / 0.77 | -92.9% | 4.6 / 115.2 | -96.0% |
| `02-linked-lists/06-merge-sorted.clj` | 0.02 / 0.35 | -94.3% | 0.02 / 0.79 | -97.5% | 4.4 / 104.7 | -95.8% |
| `03-stacks-and-queues/01-stack-drain.clj` | 0.03 / 0.35 | -91.5% | 0.03 / 0.77 | -96.8% | 4.6 / 128.1 | -96.4% |
| `03-stacks-and-queues/02-min-stack.clj` | 0.13 / 0.34 | -61.8% | 0.12 / 0.78 | -84.6% | 16.5 / 116.3 | -85.8% |
| `03-stacks-and-queues/03-queue-two-stacks.clj` | 0.36 / 0.41 | -9.9% | 0.36 / 0.90 | -59.2% | 14.3 / 202.0 | -92.9% |
| `03-stacks-and-queues/04-balanced-tokens.clj` | 0.06 / 0.34 | -82.4% | 0.06 / 0.75 | -92.0% | 1.5 / 98.8 | -98.5% |
| `03-stacks-and-queues/05-monotonic-spans.clj` | 0.01 / 0.33 | -97.0% | 0.01 / 0.73 | -98.6% | 1.4 / 95.7 | -98.5% |
| `03-stacks-and-queues/06-round-robin.clj` | 0.19 / 0.51 | -62.7% | 0.19 / 1.05 | -81.9% | 4.8 / 322.0 | -98.5% |
| `04-trees-and-graphs/01-tree-height.clj` | 0.17 / 0.34 | -50.0% | 0.17 / 0.74 | -77.0% | 5.3 / 95.5 | -94.5% |
| `04-trees-and-graphs/02-bst-search.clj` | 0.04 / 0.34 | -88.2% | 0.04 / 0.74 | -94.6% | 1.4 / 95.9 | -98.5% |
| `04-trees-and-graphs/03-level-sums.clj` | 0.03 / 0.33 | -90.9% | 0.03 / 0.74 | -95.9% | 1.4 / 95.8 | -98.5% |
| `04-trees-and-graphs/04-balanced-tree.clj` | 0.11 / 0.35 | -68.6% | 0.10 / 0.78 | -87.1% | 5.3 / 97.6 | -94.6% |
| `04-trees-and-graphs/05-graph-reachability.clj` | 0.09 / 0.45 | -80.0% | 0.09 / 0.95 | -90.6% | 5.3 / 228.9 | -97.7% |
| `04-trees-and-graphs/06-connected-components.clj` | 0.13 / 0.44 | -70.5% | 0.13 / 0.95 | -86.3% | 6.5 / 212.4 | -96.9% |
| `05-bit-manipulation/01-popcount.clj` | 0.22 / 0.45 | -51.1% | 0.22 / 0.88 | -75.0% | 1.4 / 233.0 | -99.4% |
| `05-bit-manipulation/02-bit-parity.clj` | 0.47 / 0.63 | -25.4% | 0.47 / 1.08 | -56.5% | 1.4 / 364.9 | -99.6% |
| `05-bit-manipulation/03-hamming-distance.clj` | 0.19 / 0.43 | -56.3% | 0.18 / 0.85 | -78.4% | 1.4 / 232.5 | -99.4% |
| `05-bit-manipulation/04-reverse-low-bits.clj` | 0.09 / 0.36 | -75.3% | 0.09 / 0.78 | -88.4% | 1.4 / 114.9 | -98.8% |
| `05-bit-manipulation/05-power-of-two.clj` | 0.49 / 0.40 | +22.5% | 0.48 / 0.82 | -41.5% | 1.4 / 232.2 | -99.4% |
| `05-bit-manipulation/06-insert-bit-field.clj` | 0.04 / 0.35 | -88.6% | 0.04 / 0.78 | -94.8% | 1.4 / 116.0 | -98.8% |
| `06-math-and-logic/01-euclidean-gcd.clj` | 0.32 / 0.51 | -37.3% | 0.31 / 0.94 | -66.8% | 1.4 / 364.2 | -99.6% |
| `06-math-and-logic/02-least-common-multiple.clj` | 0.14 / 0.38 | -63.2% | 0.14 / 0.80 | -82.5% | 1.4 / 161.8 | -99.1% |
| `06-math-and-logic/03-prime-count.clj` | 0.08 / 0.35 | -77.1% | 0.07 / 0.76 | -90.7% | 1.4 / 108.7 | -98.7% |
| `06-math-and-logic/04-factorial-trailing-zeros.clj` | 0.04 / 0.40 | -90.0% | 0.04 / 0.80 | -95.0% | 1.4 / 231.7 | -99.4% |
| `06-math-and-logic/05-integer-square-root.clj` | 0.30 / 0.58 | -48.7% | 0.30 / 1.03 | -71.0% | 1.4 / 498.0 | -99.7% |
| `06-math-and-logic/06-modular-power.clj` | 0.25 / 0.49 | -49.5% | 0.25 / 0.94 | -73.4% | 1.4 / 366.9 | -99.6% |
| `07-object-oriented-design/01-point-record.clj` | 0.09 / 0.34 | -73.5% | 0.09 / 0.78 | -88.4% | 58.1 / 111.1 | -47.7% |
| `07-object-oriented-design/02-shape-protocol.clj` | 0.10 / 0.34 | -70.6% | 0.09 / 0.76 | -88.1% | 59.6 / 110.0 | -45.8% |
| `07-object-oriented-design/03-payroll-protocol.clj` | 0.07 / 0.34 | -80.9% | 0.06 / 0.76 | -92.1% | 27.5 / 108.6 | -74.7% |
| `07-object-oriented-design/04-card-records.clj` | 0.21 / 0.34 | -38.2% | 0.20 / 0.77 | -73.4% | 65.9 / 101.7 | -35.2% |
| `07-object-oriented-design/05-file-tree-protocol.clj` | 0.09 / 0.37 | -75.7% | 0.09 / 0.89 | -89.8% | 45.9 / 128.2 | -64.2% |
| `07-object-oriented-design/06-record-updates.clj` | 0.42 / 0.35 | +20.0% | 0.42 / 0.80 | -47.2% | 193.8 / 118.4 | +63.7% |
| `08-recursion-and-dp/01-fibonacci.clj` | 0.18 / 0.35 | -48.6% | 0.18 / 0.76 | -76.3% | 1.4 / 115.0 | -98.8% |
| `08-recursion-and-dp/02-staircase-ways.clj` | 0.03 / 0.34 | -91.0% | 0.03 / 0.74 | -95.9% | 1.5 / 101.1 | -98.6% |
| `08-recursion-and-dp/03-grid-paths.clj` | 0.17 / 0.41 | -56.8% | 0.17 / 0.86 | -80.2% | 18.1 / 234.0 | -92.3% |
| `08-recursion-and-dp/04-coin-change.clj` | 0.28 / 0.51 | -45.1% | 0.28 / 1.01 | -72.9% | 20.4 / 364.9 | -94.4% |
| `08-recursion-and-dp/05-longest-increasing-subsequence.clj` | 0.13 / 0.35 | -62.9% | 0.13 / 0.81 | -84.0% | 8.3 / 116.4 | -92.9% |
| `08-recursion-and-dp/06-subset-sum.clj` | 0.19 / 0.45 | -57.8% | 0.18 / 0.90 | -80.0% | 18.1 / 320.1 | -94.3% |
| `09-sorting-and-searching/01-binary-search.clj` | 0.19 / 0.38 | -49.3% | 0.19 / 0.79 | -75.8% | 1.4 / 128.1 | -98.9% |
| `09-sorting-and-searching/02-insertion-sort.clj` | 0.04 / 0.38 | -89.3% | 0.04 / 0.92 | -95.6% | 4.6 / 124.7 | -96.3% |
| `09-sorting-and-searching/03-bubble-sort-vector.clj` | 0.13 / 0.37 | -64.9% | 0.12 / 0.81 | -84.6% | 18.3 / 162.1 | -88.7% |
| `09-sorting-and-searching/04-merge-sorted-vectors.clj` | 0.07 / 0.38 | -82.7% | 0.06 / 0.88 | -93.2% | 15.5 / 120.5 | -87.1% |
| `09-sorting-and-searching/05-rotated-search.clj` | 0.16 / 0.37 | -56.8% | 0.16 / 0.78 | -79.5% | 1.4 / 116.3 | -98.8% |
| `09-sorting-and-searching/06-frequency-table.clj` | 0.10 / 0.38 | -73.7% | 0.10 / 0.84 | -88.1% | 9.0 / 117.6 | -92.3% |
| `10-moderate-problems/01-maximum-subarray.clj` | 0.37 / 0.34 | +10.4% | 0.37 / 0.73 | -49.3% | 5.3 / 96.9 | -94.6% |
| `10-moderate-problems/02-pair-sum-count.clj` | 0.13 / 0.34 | -62.3% | 0.12 / 0.77 | -84.4% | 1.4 / 95.9 | -98.5% |
| `10-moderate-problems/03-mastermind-score.clj` | 0.20 / 0.40 | -50.0% | 0.20 / 0.93 | -78.5% | 5.9 / 128.9 | -95.4% |
| `10-moderate-problems/04-peak-population.clj` | 0.24 / 0.37 | -35.1% | 0.23 / 0.82 | -72.0% | 6.0 / 115.9 | -94.8% |
| `10-moderate-problems/05-arithmetic-swap.clj` | 0.04 / 0.36 | -88.9% | 0.04 / 0.77 | -94.8% | 1.4 / 168.6 | -99.2% |
| `10-moderate-problems/06-board-lengths.clj` | 0.22 / 0.51 | -56.9% | 0.22 / 1.02 | -78.5% | 8.0 / 284.0 | -97.2% |
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

O compilador release foi reconstruído antes das medições no commit `a1ecebd`.

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
