# Comparação extrema de referência

[Catálogo dos benchmarks](../../README.md) ·
[Guia da suíte](../README.md)

Arquivo: [`extreme.csv`](extreme.csv)

Medições nativas atualizadas em 2026-07-27 com:

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
- Mediana de `wall_speedup_vs_clojure`: 3,784× a favor do nativo.
- Tempos de parede acumulados: nativo 8,74 s; Clojure/JVM preservado 24,96 s.
- Tempos de CPU acumulados: nativo 8,52 s; Clojure/JVM preservado 53,88 s.
- Mediana de `cpu_speedup_vs_clojure`: 8,091× a favor do nativo.
- O nativo apresentou RSS menor em 59 dos 60 casos.
- Mediana de `rss_ratio_clojure_over_native`: 26,253×.
- Maior RSS nativo: 198.788 KiB em
  `07-object-oriented-design/06-record-updates.clj`.
- Maior RSS Clojure/JVM: 525.512 KiB em
  `06-math-and-logic/05-integer-square-root.clj`.
- Compilação acumulada: 7.834 ms no nativo e 33.114 ms preservados no Clojure/JVM AOT.

Em relação à medição nativa imediatamente anterior, o tempo acumulado de parede caiu
50,73% e o de CPU caiu 51,34%. A rodada isolada aponta melhora expressiva, mas não
permite atribuir todo o delta a uma otimização específica sem repetições controladas.

## Resumo por teste

`N/J` mostra os valores absolutos nativo/Clojure. O delta é
`(nativo - Clojure) / Clojure`: negativo favorece o nativo; positivo favorece a JVM.

| Caso | Tempo N/J (s) | Δ tempo | CPU N/J (s) | Δ CPU | RSS N/J (MiB) | Δ RSS |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `01-arrays-and-strings/01-vector-sum.clj` | 0.09 / 0.37 | -75.7% | 0.09 / 0.81 | -88.9% | 18.0 / 102.7 | -82.5% |
| `01-arrays-and-strings/02-reverse-vector.clj` | 0.06 / 0.36 | -83.3% | 0.05 / 0.81 | -93.8% | 18.2 / 110.8 | -83.6% |
| `01-arrays-and-strings/03-rotate-left.clj` | 0.06 / 0.38 | -84.2% | 0.06 / 0.93 | -93.5% | 15.8 / 125.0 | -87.3% |
| `01-arrays-and-strings/04-compact-adjacent.clj` | 0.22 / 0.38 | -42.1% | 0.21 / 0.83 | -74.7% | 18.0 / 116.9 | -84.6% |
| `01-arrays-and-strings/05-matrix-diagonals.clj` | 0.01 / 0.36 | -97.2% | 0.00 / 0.80 | -100.0% | 1.5 / 95.6 | -98.4% |
| `01-arrays-and-strings/06-rolling-hash.clj` | 0.06 / 0.39 | -84.6% | 0.05 / 0.84 | -94.0% | 18.1 / 114.8 | -84.2% |
| `02-linked-lists/01-remove-value.clj` | 0.03 / 0.42 | -92.9% | 0.03 / 0.92 | -96.7% | 4.6 / 118.7 | -96.1% |
| `02-linked-lists/02-kth-from-end.clj` | 0.02 / 0.38 | -94.7% | 0.02 / 0.78 | -97.4% | 1.5 / 96.0 | -98.5% |
| `02-linked-lists/03-stable-partition.clj` | 0.04 / 0.41 | -90.2% | 0.03 / 1.00 | -97.0% | 4.8 / 131.7 | -96.4% |
| `02-linked-lists/04-reversed-digits.clj` | 0.10 / 0.43 | -76.7% | 0.09 / 0.88 | -89.8% | 4.8 / 211.3 | -97.8% |
| `02-linked-lists/05-palindrome-list.clj` | 0.06 / 0.36 | -83.3% | 0.05 / 0.82 | -93.9% | 4.8 / 113.1 | -95.8% |
| `02-linked-lists/06-merge-sorted.clj` | 0.02 / 0.38 | -94.7% | 0.02 / 0.86 | -97.7% | 4.6 / 104.2 | -95.6% |
| `03-stacks-and-queues/01-stack-drain.clj` | 0.02 / 0.38 | -94.7% | 0.02 / 0.84 | -97.6% | 4.6 / 134.4 | -96.6% |
| `03-stacks-and-queues/02-min-stack.clj` | 0.13 / 0.37 | -64.9% | 0.13 / 0.85 | -84.7% | 16.2 / 117.6 | -86.2% |
| `03-stacks-and-queues/03-queue-two-stacks.clj` | 0.34 / 0.43 | -20.9% | 0.33 / 0.95 | -65.3% | 13.7 / 199.8 | -93.1% |
| `03-stacks-and-queues/04-balanced-tokens.clj` | 0.06 / 0.37 | -83.8% | 0.06 / 0.82 | -92.7% | 1.5 / 95.7 | -98.4% |
| `03-stacks-and-queues/05-monotonic-spans.clj` | 0.05 / 0.36 | -86.1% | 0.05 / 0.85 | -94.1% | 18.1 / 103.6 | -82.5% |
| `03-stacks-and-queues/06-round-robin.clj` | 0.19 / 0.55 | -65.5% | 0.19 / 1.11 | -82.9% | 4.8 / 316.1 | -98.5% |
| `04-trees-and-graphs/01-tree-height.clj` | 0.16 / 0.36 | -55.6% | 0.16 / 0.80 | -80.0% | 5.4 / 96.0 | -94.4% |
| `04-trees-and-graphs/02-bst-search.clj` | 0.03 / 0.37 | -91.9% | 0.03 / 0.87 | -96.6% | 1.5 / 103.7 | -98.5% |
| `04-trees-and-graphs/03-level-sums.clj` | 0.02 / 0.37 | -94.6% | 0.02 / 0.82 | -97.6% | 1.5 / 96.6 | -98.4% |
| `04-trees-and-graphs/04-balanced-tree.clj` | 0.09 / 0.37 | -75.7% | 0.09 / 0.86 | -89.5% | 5.3 / 99.5 | -94.7% |
| `04-trees-and-graphs/05-graph-reachability.clj` | 0.09 / 0.49 | -81.6% | 0.09 / 1.12 | -92.0% | 5.3 / 238.6 | -97.8% |
| `04-trees-and-graphs/06-connected-components.clj` | 0.13 / 0.49 | -73.5% | 0.13 / 1.05 | -87.6% | 6.6 / 210.5 | -96.9% |
| `05-bit-manipulation/01-popcount.clj` | 0.21 / 0.47 | -55.3% | 0.21 / 0.95 | -77.9% | 1.5 / 234.4 | -99.4% |
| `05-bit-manipulation/02-bit-parity.clj` | 0.45 / 0.69 | -34.8% | 0.45 / 1.18 | -61.9% | 1.5 / 365.0 | -99.6% |
| `05-bit-manipulation/03-hamming-distance.clj` | 0.18 / 0.49 | -63.3% | 0.18 / 0.95 | -81.1% | 1.5 / 231.9 | -99.4% |
| `05-bit-manipulation/04-reverse-low-bits.clj` | 0.08 / 0.39 | -79.5% | 0.08 / 0.87 | -90.8% | 1.5 / 114.9 | -98.7% |
| `05-bit-manipulation/05-power-of-two.clj` | 0.49 / 0.43 | +14.0% | 0.49 / 0.89 | -44.9% | 1.5 / 231.7 | -99.4% |
| `05-bit-manipulation/06-insert-bit-field.clj` | 0.03 / 0.38 | -92.1% | 0.03 / 0.84 | -96.4% | 1.5 / 115.5 | -98.7% |
| `06-math-and-logic/01-euclidean-gcd.clj` | 0.30 / 0.54 | -44.4% | 0.30 / 1.01 | -70.3% | 1.5 / 363.6 | -99.6% |
| `06-math-and-logic/02-least-common-multiple.clj` | 0.13 / 0.41 | -68.3% | 0.13 / 0.88 | -85.2% | 1.5 / 162.2 | -99.1% |
| `06-math-and-logic/03-prime-count.clj` | 0.06 / 0.37 | -83.8% | 0.06 / 0.77 | -92.2% | 1.5 / 104.5 | -98.6% |
| `06-math-and-logic/04-factorial-trailing-zeros.clj` | 0.03 / 0.45 | -93.3% | 0.03 / 0.91 | -96.7% | 1.5 / 236.1 | -99.4% |
| `06-math-and-logic/05-integer-square-root.clj` | 0.25 / 0.64 | -60.9% | 0.25 / 1.14 | -78.1% | 1.5 / 513.2 | -99.7% |
| `06-math-and-logic/06-modular-power.clj` | 0.26 / 0.53 | -50.9% | 0.26 / 1.00 | -74.0% | 1.5 / 363.7 | -99.6% |
| `07-object-oriented-design/01-point-record.clj` | 0.09 / 0.38 | -76.3% | 0.08 / 0.81 | -90.1% | 58.3 / 108.2 | -46.2% |
| `07-object-oriented-design/02-shape-protocol.clj` | 0.10 / 0.38 | -73.7% | 0.09 / 0.82 | -89.0% | 59.8 / 113.3 | -47.2% |
| `07-object-oriented-design/03-payroll-protocol.clj` | 0.06 / 0.38 | -84.2% | 0.06 / 0.85 | -92.9% | 27.5 / 105.3 | -73.9% |
| `07-object-oriented-design/04-card-records.clj` | 0.20 / 0.37 | -45.9% | 0.19 / 0.85 | -77.6% | 66.0 / 101.9 | -35.2% |
| `07-object-oriented-design/05-file-tree-protocol.clj` | 0.09 / 0.40 | -77.5% | 0.08 / 0.96 | -91.7% | 46.1 / 128.6 | -64.2% |
| `07-object-oriented-design/06-record-updates.clj` | 0.41 / 0.37 | +10.8% | 0.40 / 0.86 | -53.5% | 194.1 / 116.4 | +66.8% |
| `08-recursion-and-dp/01-fibonacci.clj` | 0.15 / 0.38 | -60.5% | 0.15 / 0.84 | -82.1% | 1.5 / 114.8 | -98.7% |
| `08-recursion-and-dp/02-staircase-ways.clj` | 0.02 / 0.36 | -94.4% | 0.02 / 0.82 | -97.6% | 1.5 / 98.5 | -98.5% |
| `08-recursion-and-dp/03-grid-paths.clj` | 0.17 / 0.45 | -62.2% | 0.16 / 0.95 | -83.2% | 17.6 / 238.1 | -92.6% |
| `08-recursion-and-dp/04-coin-change.clj` | 0.32 / 0.56 | -42.9% | 0.31 / 1.11 | -72.1% | 19.4 / 368.4 | -94.7% |
| `08-recursion-and-dp/05-longest-increasing-subsequence.clj` | 0.15 / 0.37 | -59.5% | 0.14 / 0.88 | -84.1% | 10.8 / 116.0 | -90.7% |
| `08-recursion-and-dp/06-subset-sum.clj` | 0.20 / 0.48 | -58.3% | 0.19 / 0.97 | -80.4% | 17.6 / 320.6 | -94.5% |
| `09-sorting-and-searching/01-binary-search.clj` | 0.17 / 0.41 | -58.5% | 0.17 / 0.82 | -79.3% | 1.5 / 126.2 | -98.8% |
| `09-sorting-and-searching/02-insertion-sort.clj` | 0.04 / 0.40 | -90.0% | 0.04 / 0.96 | -95.8% | 4.6 / 124.3 | -96.3% |
| `09-sorting-and-searching/03-bubble-sort-vector.clj` | 0.12 / 0.39 | -69.2% | 0.11 / 0.89 | -87.6% | 17.6 / 169.5 | -89.6% |
| `09-sorting-and-searching/04-merge-sorted-vectors.clj` | 0.13 / 0.38 | -65.8% | 0.12 / 0.90 | -86.7% | 16.7 / 120.5 | -86.1% |
| `09-sorting-and-searching/05-rotated-search.clj` | 0.14 / 0.37 | -62.2% | 0.13 / 0.80 | -83.8% | 1.5 / 115.9 | -98.7% |
| `09-sorting-and-searching/06-frequency-table.clj` | 0.10 / 0.39 | -74.4% | 0.10 / 0.89 | -88.8% | 9.1 / 117.4 | -92.2% |
| `10-moderate-problems/01-maximum-subarray.clj` | 0.54 / 0.35 | +54.3% | 0.54 / 0.78 | -30.8% | 10.1 / 95.0 | -89.3% |
| `10-moderate-problems/02-pair-sum-count.clj` | 0.19 / 0.36 | -47.2% | 0.19 / 0.79 | -75.9% | 17.8 / 94.9 | -81.3% |
| `10-moderate-problems/03-mastermind-score.clj` | 0.31 / 0.42 | -26.2% | 0.30 / 0.99 | -69.7% | 11.1 / 130.6 | -91.5% |
| `10-moderate-problems/04-peak-population.clj` | 0.24 / 0.38 | -36.8% | 0.24 / 0.85 | -71.8% | 6.0 / 112.9 | -94.7% |
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
`a0fb120d758259ffe4148169e6dc662e4037189806ad0ebb1a7355001adcadf2`.

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
