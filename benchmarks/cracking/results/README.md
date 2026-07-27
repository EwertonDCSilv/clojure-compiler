# Comparação extrema de referência

[Catálogo dos benchmarks](../../README.md) ·
[Guia da suíte](../README.md)

Arquivo: [`extreme.csv`](extreme.csv)

Medições nativas atualizadas em 2026-07-27 no commit `8012102` com:

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
- Mediana de `wall_speedup_vs_clojure`: 3,433× a favor do nativo.
- Tempos de parede acumulados: nativo 9,04 s; Clojure/JVM preservado 24,96 s.
- Tempos de CPU acumulados: nativo 8,86 s; Clojure/JVM preservado 53,88 s.
- O nativo teve menor tempo de CPU nos 60 casos.
- Mediana de `cpu_speedup_vs_clojure`: 8,050× a favor do nativo.
- O nativo apresentou RSS menor em 59 dos 60 casos.
- Mediana de `rss_ratio_clojure_over_native`: 26,757×.
- Maior RSS nativo: 198.532 KiB em
  `07-object-oriented-design/06-record-updates.clj`.
- Maior RSS Clojure/JVM: 525.512 KiB em
  `06-math-and-logic/05-integer-square-root.clj`.
- Compilação acumulada: 7.982 ms no nativo e 33.114 ms preservados no Clojure/JVM AOT.

Em relação à medição nativa imediatamente anterior, os tempos acumulados de parede e
CPU aumentaram respectivamente 3,43% e 3,99%. É uma regressão pequena observada em uma
única rodada; deve ser confirmada com repetições controladas antes de ser atribuída às
mudanças do compilador.

## Resumo por teste

`N/J` mostra os valores absolutos nativo/Clojure. O delta é
`(nativo - Clojure) / Clojure`: negativo favorece o nativo; positivo favorece a JVM.

| Caso | Tempo N/J (s) | Δ tempo | CPU N/J (s) | Δ CPU | RSS N/J (MiB) | Δ RSS |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `01-arrays-and-strings/01-vector-sum.clj` | 0.08 / 0.37 | -78.4% | 0.07 / 0.81 | -91.4% | 18.9 / 102.7 | -81.6% |
| `01-arrays-and-strings/02-reverse-vector.clj` | 0.06 / 0.36 | -83.3% | 0.06 / 0.81 | -92.6% | 19.0 / 110.8 | -82.9% |
| `01-arrays-and-strings/03-rotate-left.clj` | 0.06 / 0.38 | -84.2% | 0.06 / 0.93 | -93.5% | 16.4 / 125.0 | -86.9% |
| `01-arrays-and-strings/04-compact-adjacent.clj` | 0.29 / 0.38 | -23.7% | 0.29 / 0.83 | -65.1% | 18.8 / 116.9 | -84.0% |
| `01-arrays-and-strings/05-matrix-diagonals.clj` | 0.00 / 0.36 | -100.0% | 0.00 / 0.80 | -100.0% | 1.5 / 95.6 | -98.5% |
| `01-arrays-and-strings/06-rolling-hash.clj` | 0.06 / 0.39 | -84.6% | 0.05 / 0.84 | -94.0% | 19.0 / 114.8 | -83.5% |
| `02-linked-lists/01-remove-value.clj` | 0.03 / 0.42 | -92.9% | 0.03 / 0.92 | -96.7% | 4.6 / 118.7 | -96.1% |
| `02-linked-lists/02-kth-from-end.clj` | 0.02 / 0.38 | -94.7% | 0.01 / 0.78 | -98.7% | 1.5 / 96.0 | -98.4% |
| `02-linked-lists/03-stable-partition.clj` | 0.04 / 0.41 | -90.2% | 0.03 / 1.00 | -97.0% | 4.8 / 131.7 | -96.4% |
| `02-linked-lists/04-reversed-digits.clj` | 0.09 / 0.43 | -79.1% | 0.09 / 0.88 | -89.8% | 4.8 / 211.3 | -97.7% |
| `02-linked-lists/05-palindrome-list.clj` | 0.06 / 0.36 | -83.3% | 0.05 / 0.82 | -93.9% | 4.8 / 113.1 | -95.8% |
| `02-linked-lists/06-merge-sorted.clj` | 0.02 / 0.38 | -94.7% | 0.02 / 0.86 | -97.7% | 4.6 / 104.2 | -95.6% |
| `03-stacks-and-queues/01-stack-drain.clj` | 0.02 / 0.38 | -94.7% | 0.02 / 0.84 | -97.6% | 4.5 / 134.4 | -96.6% |
| `03-stacks-and-queues/02-min-stack.clj` | 0.17 / 0.37 | -54.1% | 0.16 / 0.85 | -81.2% | 17.0 / 117.6 | -85.6% |
| `03-stacks-and-queues/03-queue-two-stacks.clj` | 0.34 / 0.43 | -20.9% | 0.33 / 0.95 | -65.3% | 14.2 / 199.8 | -92.9% |
| `03-stacks-and-queues/04-balanced-tokens.clj` | 0.06 / 0.37 | -83.8% | 0.06 / 0.82 | -92.7% | 1.5 / 95.7 | -98.4% |
| `03-stacks-and-queues/05-monotonic-spans.clj` | 0.05 / 0.36 | -86.1% | 0.05 / 0.85 | -94.1% | 19.0 / 103.6 | -81.7% |
| `03-stacks-and-queues/06-round-robin.clj` | 0.20 / 0.55 | -63.6% | 0.20 / 1.11 | -82.0% | 4.9 / 316.1 | -98.5% |
| `04-trees-and-graphs/01-tree-height.clj` | 0.16 / 0.36 | -55.6% | 0.16 / 0.80 | -80.0% | 5.3 / 96.0 | -94.4% |
| `04-trees-and-graphs/02-bst-search.clj` | 0.03 / 0.37 | -91.9% | 0.03 / 0.87 | -96.6% | 1.5 / 103.7 | -98.6% |
| `04-trees-and-graphs/03-level-sums.clj` | 0.02 / 0.37 | -94.6% | 0.02 / 0.82 | -97.6% | 1.5 / 96.6 | -98.5% |
| `04-trees-and-graphs/04-balanced-tree.clj` | 0.10 / 0.37 | -73.0% | 0.10 / 0.86 | -88.4% | 5.3 / 99.5 | -94.7% |
| `04-trees-and-graphs/05-graph-reachability.clj` | 0.09 / 0.49 | -81.6% | 0.09 / 1.12 | -92.0% | 5.4 / 238.6 | -97.7% |
| `04-trees-and-graphs/06-connected-components.clj` | 0.13 / 0.49 | -73.5% | 0.12 / 1.05 | -88.6% | 6.8 / 210.5 | -96.8% |
| `05-bit-manipulation/01-popcount.clj` | 0.21 / 0.47 | -55.3% | 0.21 / 0.95 | -77.9% | 1.4 / 234.4 | -99.4% |
| `05-bit-manipulation/02-bit-parity.clj` | 0.45 / 0.69 | -34.8% | 0.45 / 1.18 | -61.9% | 1.5 / 365.0 | -99.6% |
| `05-bit-manipulation/03-hamming-distance.clj` | 0.18 / 0.49 | -63.3% | 0.18 / 0.95 | -81.1% | 1.5 / 231.9 | -99.4% |
| `05-bit-manipulation/04-reverse-low-bits.clj` | 0.08 / 0.39 | -79.5% | 0.08 / 0.87 | -90.8% | 1.5 / 114.9 | -98.7% |
| `05-bit-manipulation/05-power-of-two.clj` | 0.51 / 0.43 | +18.6% | 0.51 / 0.89 | -42.7% | 1.5 / 231.7 | -99.4% |
| `05-bit-manipulation/06-insert-bit-field.clj` | 0.02 / 0.38 | -94.7% | 0.02 / 0.84 | -97.6% | 1.5 / 115.5 | -98.7% |
| `06-math-and-logic/01-euclidean-gcd.clj` | 0.31 / 0.54 | -42.6% | 0.31 / 1.01 | -69.3% | 1.5 / 363.6 | -99.6% |
| `06-math-and-logic/02-least-common-multiple.clj` | 0.13 / 0.41 | -68.3% | 0.13 / 0.88 | -85.2% | 1.5 / 162.2 | -99.1% |
| `06-math-and-logic/03-prime-count.clj` | 0.06 / 0.37 | -83.8% | 0.06 / 0.77 | -92.2% | 1.5 / 104.5 | -98.6% |
| `06-math-and-logic/04-factorial-trailing-zeros.clj` | 0.03 / 0.45 | -93.3% | 0.03 / 0.91 | -96.7% | 1.5 / 236.1 | -99.4% |
| `06-math-and-logic/05-integer-square-root.clj` | 0.25 / 0.64 | -60.9% | 0.25 / 1.14 | -78.1% | 1.5 / 513.2 | -99.7% |
| `06-math-and-logic/06-modular-power.clj` | 0.27 / 0.53 | -49.1% | 0.27 / 1.00 | -73.0% | 1.5 / 363.7 | -99.6% |
| `07-object-oriented-design/01-point-record.clj` | 0.09 / 0.38 | -76.3% | 0.09 / 0.81 | -88.9% | 58.4 / 108.2 | -46.0% |
| `07-object-oriented-design/02-shape-protocol.clj` | 0.09 / 0.38 | -76.3% | 0.09 / 0.82 | -89.0% | 59.8 / 113.3 | -47.3% |
| `07-object-oriented-design/03-payroll-protocol.clj` | 0.06 / 0.38 | -84.2% | 0.06 / 0.85 | -92.9% | 27.8 / 105.3 | -73.6% |
| `07-object-oriented-design/04-card-records.clj` | 0.20 / 0.37 | -45.9% | 0.19 / 0.85 | -77.6% | 65.9 / 101.9 | -35.3% |
| `07-object-oriented-design/05-file-tree-protocol.clj` | 0.09 / 0.40 | -77.5% | 0.09 / 0.96 | -90.6% | 45.9 / 128.6 | -64.3% |
| `07-object-oriented-design/06-record-updates.clj` | 0.41 / 0.37 | +10.8% | 0.40 / 0.86 | -53.5% | 193.9 / 116.4 | +66.6% |
| `08-recursion-and-dp/01-fibonacci.clj` | 0.15 / 0.38 | -60.5% | 0.15 / 0.84 | -82.1% | 1.5 / 114.8 | -98.7% |
| `08-recursion-and-dp/02-staircase-ways.clj` | 0.02 / 0.36 | -94.4% | 0.02 / 0.82 | -97.6% | 1.5 / 98.5 | -98.5% |
| `08-recursion-and-dp/03-grid-paths.clj` | 0.28 / 0.45 | -37.8% | 0.27 / 0.95 | -71.6% | 18.4 / 238.1 | -92.3% |
| `08-recursion-and-dp/04-coin-change.clj` | 0.34 / 0.56 | -39.3% | 0.33 / 1.11 | -70.3% | 20.1 / 368.4 | -94.5% |
| `08-recursion-and-dp/05-longest-increasing-subsequence.clj` | 0.15 / 0.37 | -59.5% | 0.15 / 0.88 | -83.0% | 11.1 / 116.0 | -90.4% |
| `08-recursion-and-dp/06-subset-sum.clj` | 0.21 / 0.48 | -56.2% | 0.21 / 0.97 | -78.4% | 18.4 / 320.6 | -94.3% |
| `09-sorting-and-searching/01-binary-search.clj` | 0.16 / 0.41 | -61.0% | 0.16 / 0.82 | -80.5% | 1.5 / 126.2 | -98.8% |
| `09-sorting-and-searching/02-insertion-sort.clj` | 0.04 / 0.40 | -90.0% | 0.04 / 0.96 | -95.8% | 4.5 / 124.3 | -96.4% |
| `09-sorting-and-searching/03-bubble-sort-vector.clj` | 0.13 / 0.39 | -66.7% | 0.12 / 0.89 | -86.5% | 18.4 / 169.5 | -89.2% |
| `09-sorting-and-searching/04-merge-sorted-vectors.clj` | 0.12 / 0.38 | -68.4% | 0.12 / 0.90 | -86.7% | 17.5 / 120.5 | -85.5% |
| `09-sorting-and-searching/05-rotated-search.clj` | 0.14 / 0.37 | -62.2% | 0.14 / 0.80 | -82.5% | 1.5 / 115.9 | -98.7% |
| `09-sorting-and-searching/06-frequency-table.clj` | 0.10 / 0.39 | -74.4% | 0.09 / 0.89 | -89.9% | 9.2 / 117.4 | -92.2% |
| `10-moderate-problems/01-maximum-subarray.clj` | 0.59 / 0.35 | +68.6% | 0.58 / 0.78 | -25.6% | 10.4 / 95.0 | -89.1% |
| `10-moderate-problems/02-pair-sum-count.clj` | 0.21 / 0.36 | -41.7% | 0.20 / 0.79 | -74.7% | 18.8 / 94.9 | -80.2% |
| `10-moderate-problems/03-mastermind-score.clj` | 0.30 / 0.42 | -28.6% | 0.29 / 0.99 | -70.7% | 11.5 / 130.6 | -91.2% |
| `10-moderate-problems/04-peak-population.clj` | 0.23 / 0.38 | -39.5% | 0.23 / 0.85 | -72.9% | 5.8 / 112.9 | -94.9% |
| `10-moderate-problems/05-arithmetic-swap.clj` | 0.03 / 0.38 | -92.1% | 0.03 / 0.80 | -96.2% | 1.5 / 168.1 | -99.1% |
| `10-moderate-problems/06-board-lengths.clj` | 0.22 / 0.53 | -58.5% | 0.21 / 1.08 | -80.6% | 8.1 / 291.9 | -97.2% |

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
