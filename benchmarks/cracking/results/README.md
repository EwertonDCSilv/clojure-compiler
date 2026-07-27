# Comparação extrema de referência

[Catálogo dos benchmarks](../../README.md) ·
[Guia da suíte](../README.md)

Arquivo: [`extreme.csv`](extreme.csv)

Medições nativas atualizadas em 2026-07-27 no commit `663d2d4` com:

```bash
benchmarks/cracking/run.sh --scale 25 --opt-level none \
  --compiler target/release/clojure-native \
  --csv /tmp/clojure-compiler-cracking-native.csv
```

As colunas `clojure_*`, `clojure_version` e `clojure_checksum` foram preservadas sem
alteração da rodada comparativa anterior. As colunas nativas foram substituídas pela
nova execução; `wall_speedup_vs_clojure`, `cpu_speedup_vs_clojure` e
`rss_ratio_clojure_over_native` foram então recalculadas.

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

- 60 casos comparados, 60 com status `OK` e checksums idênticos.
- O nativo teve menor tempo de parede em 57 casos; Clojure/JVM em 3.
- Mediana de `wall_speedup_vs_clojure`: 3,769× a favor do nativo.
- Tempos de parede acumulados: nativo 8,11 s; Clojure/JVM preservado 24,96 s.
- Tempos de CPU acumulados: nativo 7,94 s; Clojure/JVM preservado 53,88 s.
- O nativo teve menor tempo de CPU nos 60 casos.
- Mediana de `cpu_speedup_vs_clojure`: 8,875× a favor do nativo.
- O nativo apresentou RSS menor em 59 dos 60 casos.
- Mediana de `rss_ratio_clojure_over_native`: 26,264×.
- Maior RSS nativo: 198.788 KiB em
  `07-object-oriented-design/06-record-updates.clj`.
- Maior RSS Clojure/JVM: 525.512 KiB em
  `06-math-and-logic/05-integer-square-root.clj`.
- Compilação acumulada: 7.754 ms no nativo e 33.114 ms preservados no Clojure/JVM AOT.

Foram feitas três execuções completas, com tempos acumulados de parede de 8,08 s,
8,18 s e 8,11 s e CPU de 7,91 s, 7,95 s e 7,94 s. O `extreme.csv` guarda integralmente
a execução mediana pelo tempo de parede, de 8,11 s, sem combinar métricas de processos
distintos.

Em relação ao CSV versionado imediatamente anterior, a execução publicada melhorou
10,29% em parede e 10,38% em CPU; 29 casos melhoraram, 30 empataram na resolução de
0,01 s e apenas 1 piorou. Entre os ganhos mais claros estão `grid-paths` (0,28 → 0,14
s), `compact-adjacent` (0,29 → 0,16 s), `coin-change` (0,34 → 0,23 s) e `subset-sum`
(0,21 → 0,15 s). A distribuição é coerente com a construção de `mapv` e `into` por
transiente estrutural introduzida no compilador medido.

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
| `01-arrays-and-strings/01-vector-sum.clj` | 0.08 / 0.37 | -78.4% | 0.07 / 0.81 | -91.4% | 18.8 / 102.7 | -81.7% |
| `01-arrays-and-strings/02-reverse-vector.clj` | 0.04 / 0.36 | -88.9% | 0.04 / 0.81 | -95.1% | 18.1 / 110.8 | -83.7% |
| `01-arrays-and-strings/03-rotate-left.clj` | 0.05 / 0.38 | -86.8% | 0.04 / 0.93 | -95.7% | 15.1 / 125.0 | -87.9% |
| `01-arrays-and-strings/04-compact-adjacent.clj` | 0.16 / 0.38 | -57.9% | 0.16 / 0.83 | -80.7% | 18.1 / 116.9 | -84.5% |
| `01-arrays-and-strings/05-matrix-diagonals.clj` | 0.00 / 0.36 | -100.0% | 0.00 / 0.80 | -100.0% | 1.5 / 95.6 | -98.5% |
| `01-arrays-and-strings/06-rolling-hash.clj` | 0.06 / 0.39 | -84.6% | 0.05 / 0.84 | -94.0% | 18.9 / 114.8 | -83.6% |
| `02-linked-lists/01-remove-value.clj` | 0.03 / 0.42 | -92.9% | 0.03 / 0.92 | -96.7% | 4.6 / 118.7 | -96.1% |
| `02-linked-lists/02-kth-from-end.clj` | 0.02 / 0.38 | -94.7% | 0.02 / 0.78 | -97.4% | 1.5 / 96.0 | -98.4% |
| `02-linked-lists/03-stable-partition.clj` | 0.04 / 0.41 | -90.2% | 0.03 / 1.00 | -97.0% | 4.8 / 131.7 | -96.4% |
| `02-linked-lists/04-reversed-digits.clj` | 0.09 / 0.43 | -79.1% | 0.08 / 0.88 | -90.9% | 4.9 / 211.3 | -97.7% |
| `02-linked-lists/05-palindrome-list.clj` | 0.06 / 0.36 | -83.3% | 0.05 / 0.82 | -93.9% | 4.8 / 113.1 | -95.8% |
| `02-linked-lists/06-merge-sorted.clj` | 0.02 / 0.38 | -94.7% | 0.02 / 0.86 | -97.7% | 4.6 / 104.2 | -95.6% |
| `03-stacks-and-queues/01-stack-drain.clj` | 0.03 / 0.38 | -92.1% | 0.03 / 0.84 | -96.4% | 4.5 / 134.4 | -96.6% |
| `03-stacks-and-queues/02-min-stack.clj` | 0.13 / 0.37 | -64.9% | 0.12 / 0.85 | -85.9% | 16.9 / 117.6 | -85.6% |
| `03-stacks-and-queues/03-queue-two-stacks.clj` | 0.29 / 0.43 | -32.6% | 0.29 / 0.95 | -69.5% | 14.3 / 199.8 | -92.9% |
| `03-stacks-and-queues/04-balanced-tokens.clj` | 0.06 / 0.37 | -83.8% | 0.06 / 0.82 | -92.7% | 1.5 / 95.7 | -98.4% |
| `03-stacks-and-queues/05-monotonic-spans.clj` | 0.05 / 0.36 | -86.1% | 0.04 / 0.85 | -95.3% | 19.0 / 103.6 | -81.7% |
| `03-stacks-and-queues/06-round-robin.clj` | 0.18 / 0.55 | -67.3% | 0.17 / 1.11 | -84.7% | 4.9 / 316.1 | -98.5% |
| `04-trees-and-graphs/01-tree-height.clj` | 0.16 / 0.36 | -55.6% | 0.15 / 0.80 | -81.2% | 5.3 / 96.0 | -94.5% |
| `04-trees-and-graphs/02-bst-search.clj` | 0.03 / 0.37 | -91.9% | 0.03 / 0.87 | -96.6% | 1.5 / 103.7 | -98.5% |
| `04-trees-and-graphs/03-level-sums.clj` | 0.02 / 0.37 | -94.6% | 0.02 / 0.82 | -97.6% | 1.5 / 96.6 | -98.4% |
| `04-trees-and-graphs/04-balanced-tree.clj` | 0.09 / 0.37 | -75.7% | 0.09 / 0.86 | -89.5% | 5.3 / 99.5 | -94.7% |
| `04-trees-and-graphs/05-graph-reachability.clj` | 0.09 / 0.49 | -81.6% | 0.09 / 1.12 | -92.0% | 5.2 / 238.6 | -97.8% |
| `04-trees-and-graphs/06-connected-components.clj` | 0.12 / 0.49 | -75.5% | 0.12 / 1.05 | -88.6% | 6.8 / 210.5 | -96.8% |
| `05-bit-manipulation/01-popcount.clj` | 0.20 / 0.47 | -57.4% | 0.20 / 0.95 | -78.9% | 1.5 / 234.4 | -99.4% |
| `05-bit-manipulation/02-bit-parity.clj` | 0.44 / 0.69 | -36.2% | 0.44 / 1.18 | -62.7% | 1.4 / 365.0 | -99.6% |
| `05-bit-manipulation/03-hamming-distance.clj` | 0.17 / 0.49 | -65.3% | 0.17 / 0.95 | -82.1% | 1.5 / 231.9 | -99.4% |
| `05-bit-manipulation/04-reverse-low-bits.clj` | 0.08 / 0.39 | -79.5% | 0.08 / 0.87 | -90.8% | 1.5 / 114.9 | -98.7% |
| `05-bit-manipulation/05-power-of-two.clj` | 0.47 / 0.43 | +9.3% | 0.47 / 0.89 | -47.2% | 1.5 / 231.7 | -99.4% |
| `05-bit-manipulation/06-insert-bit-field.clj` | 0.02 / 0.38 | -94.7% | 0.02 / 0.84 | -97.6% | 1.5 / 115.5 | -98.7% |
| `06-math-and-logic/01-euclidean-gcd.clj` | 0.31 / 0.54 | -42.6% | 0.31 / 1.01 | -69.3% | 1.5 / 363.6 | -99.6% |
| `06-math-and-logic/02-least-common-multiple.clj` | 0.13 / 0.41 | -68.3% | 0.13 / 0.88 | -85.2% | 1.5 / 162.2 | -99.1% |
| `06-math-and-logic/03-prime-count.clj` | 0.06 / 0.37 | -83.8% | 0.06 / 0.77 | -92.2% | 1.5 / 104.5 | -98.6% |
| `06-math-and-logic/04-factorial-trailing-zeros.clj` | 0.03 / 0.45 | -93.3% | 0.03 / 0.91 | -96.7% | 1.5 / 236.1 | -99.4% |
| `06-math-and-logic/05-integer-square-root.clj` | 0.25 / 0.64 | -60.9% | 0.25 / 1.14 | -78.1% | 1.5 / 513.2 | -99.7% |
| `06-math-and-logic/06-modular-power.clj` | 0.23 / 0.53 | -56.6% | 0.23 / 1.00 | -77.0% | 1.5 / 363.7 | -99.6% |
| `07-object-oriented-design/01-point-record.clj` | 0.08 / 0.38 | -78.9% | 0.08 / 0.81 | -90.1% | 58.3 / 108.2 | -46.2% |
| `07-object-oriented-design/02-shape-protocol.clj` | 0.09 / 0.38 | -76.3% | 0.09 / 0.82 | -89.0% | 59.9 / 113.3 | -47.2% |
| `07-object-oriented-design/03-payroll-protocol.clj` | 0.06 / 0.38 | -84.2% | 0.06 / 0.85 | -92.9% | 27.5 / 105.3 | -73.9% |
| `07-object-oriented-design/04-card-records.clj` | 0.19 / 0.37 | -48.6% | 0.19 / 0.85 | -77.6% | 66.0 / 101.9 | -35.2% |
| `07-object-oriented-design/05-file-tree-protocol.clj` | 0.09 / 0.40 | -77.5% | 0.08 / 0.96 | -91.7% | 46.1 / 128.6 | -64.2% |
| `07-object-oriented-design/06-record-updates.clj` | 0.40 / 0.37 | +8.1% | 0.40 / 0.86 | -53.5% | 194.1 / 116.4 | +66.8% |
| `08-recursion-and-dp/01-fibonacci.clj` | 0.15 / 0.38 | -60.5% | 0.15 / 0.84 | -82.1% | 1.5 / 114.8 | -98.7% |
| `08-recursion-and-dp/02-staircase-ways.clj` | 0.02 / 0.36 | -94.4% | 0.02 / 0.82 | -97.6% | 1.5 / 98.5 | -98.5% |
| `08-recursion-and-dp/03-grid-paths.clj` | 0.14 / 0.45 | -68.9% | 0.13 / 0.95 | -86.3% | 18.3 / 238.1 | -92.3% |
| `08-recursion-and-dp/04-coin-change.clj` | 0.23 / 0.56 | -58.9% | 0.22 / 1.11 | -80.2% | 20.5 / 368.4 | -94.4% |
| `08-recursion-and-dp/05-longest-increasing-subsequence.clj` | 0.13 / 0.37 | -64.9% | 0.13 / 0.88 | -85.2% | 10.0 / 116.0 | -91.4% |
| `08-recursion-and-dp/06-subset-sum.clj` | 0.15 / 0.48 | -68.7% | 0.14 / 0.97 | -85.6% | 18.2 / 320.6 | -94.3% |
| `09-sorting-and-searching/01-binary-search.clj` | 0.15 / 0.41 | -63.4% | 0.15 / 0.82 | -81.7% | 1.5 / 126.2 | -98.8% |
| `09-sorting-and-searching/02-insertion-sort.clj` | 0.04 / 0.40 | -90.0% | 0.04 / 0.96 | -95.8% | 4.6 / 124.3 | -96.3% |
| `09-sorting-and-searching/03-bubble-sort-vector.clj` | 0.12 / 0.39 | -69.2% | 0.11 / 0.89 | -87.6% | 18.3 / 169.5 | -89.2% |
| `09-sorting-and-searching/04-merge-sorted-vectors.clj` | 0.11 / 0.38 | -71.1% | 0.10 / 0.90 | -88.9% | 17.0 / 120.5 | -85.9% |
| `09-sorting-and-searching/05-rotated-search.clj` | 0.13 / 0.37 | -64.9% | 0.13 / 0.80 | -83.8% | 1.5 / 115.9 | -98.7% |
| `09-sorting-and-searching/06-frequency-table.clj` | 0.09 / 0.39 | -76.9% | 0.09 / 0.89 | -89.9% | 9.2 / 117.4 | -92.2% |
| `10-moderate-problems/01-maximum-subarray.clj` | 0.53 / 0.35 | +51.4% | 0.53 / 0.78 | -32.1% | 10.4 / 95.0 | -89.1% |
| `10-moderate-problems/02-pair-sum-count.clj` | 0.19 / 0.36 | -47.2% | 0.18 / 0.79 | -77.2% | 18.6 / 94.9 | -80.4% |
| `10-moderate-problems/03-mastermind-score.clj` | 0.27 / 0.42 | -35.7% | 0.27 / 0.99 | -72.7% | 11.5 / 130.6 | -91.2% |
| `10-moderate-problems/04-peak-population.clj` | 0.23 / 0.38 | -39.5% | 0.23 / 0.85 | -72.9% | 6.0 / 112.9 | -94.7% |
| `10-moderate-problems/05-arithmetic-swap.clj` | 0.03 / 0.38 | -92.1% | 0.03 / 0.80 | -96.2% | 1.5 / 168.1 | -99.1% |
| `10-moderate-problems/06-board-lengths.clj` | 0.20 / 0.53 | -62.3% | 0.20 / 1.08 | -81.5% | 8.0 / 291.9 | -97.3% |
## Como ler a comparação

- `wall_speedup_vs_clojure` é tempo Clojure dividido pelo tempo nativo.
- `cpu_speedup_vs_clojure` é CPU Clojure dividida pela CPU nativa.
- `rss_ratio_clojure_over_native` é RSS Clojure dividido pelo RSS nativo.
- Nas três colunas, valores maiores que `1` favorecem o nativo; valores menores que `1`
  favorecem Clojure/JVM.

Os dois caminhos foram compilados antes de suas respectivas execuções medidas: o
binário pelo `clojure-compiler` e o namespace JVM por AOT. Nesta atualização somente o
nativo foi recompilado e executado. Os custos aparecem separadamente em
`native_compile_wall_ms` e `clojure_compile_wall_ms`.

Os valores são uma fotografia desta máquina; frequência dinâmica, carga do sistema,
toolchain, JIT e sistema operacional afetam o resultado. Para conclusões estatísticas,
repita as medições no mesmo ambiente e compare distribuições, não apenas uma execução.

O compilador release foi reconstruído imediatamente antes da rodada. O `runtime.c`
medido tinha SHA-256
`160fb7fd7fe2478177f9f3ef5cd95a7b7fbf987e7e2c61dd51b11bdeb38d14c6`.

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
