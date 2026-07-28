# Comparação extrema de referência — Cormen/CLRS

[Catálogo dos benchmarks](../../README.md) ·
[Guia da suíte](../README.md)

Arquivo: [`extreme.csv`](extreme.csv)

Snapshot do relatório:
[`HEAD 1dc69b5`](https://github.com/EwertonDCSilv/clojure-compiler/commit/1dc69b5b126c193c30e9f24fdddd549abb7ce4cb).

Medições Native × Clojure/JVM refeitas em 2026-07-28 no commit `1dc69b5` com:

```bash
benchmarks/cormen/compare-clojure.sh --scale 25 \
  --csv benchmarks/cormen/results/extreme.csv
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

- 30 casos comparados, todos com status `OK` e checksums idênticos.
- O nativo teve menor tempo de parede em 13 casos; Clojure/JVM em 16; houve um empate.
- Mediana de `wall_speedup_vs_clojure`: 0,776×.
- Tempos de parede acumulados: nativo 26,08 s; Clojure/JVM 16,39 s.
- Tempos de CPU acumulados: nativo 25,97 s; Clojure/JVM 31,35 s.
- O nativo teve menor tempo de CPU em 22 casos; Clojure/JVM em 8.
- Mediana de `cpu_speedup_vs_clojure`: 1,688×; o total de CPU favorece o nativo.
- O nativo apresentou RSS menor nos 30 casos.
- Mediana de `rss_ratio_clojure_over_native`: 26,912×.
- Maior RSS nativo: 21,3 MiB em
  `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj`.
- Maior RSS Clojure/JVM: 1009,9 MiB no mesmo caso.
- Compilação acumulada: 5.304 ms no nativo e 14.234 ms no Clojure/JVM AOT.

Esta é uma execução completa única. Em relação ao snapshot anterior, o agregado nativo
caiu de 29,45 para 26,08 s de parede e de 29,30 para 25,97 s de CPU. A melhora é
coerente no agregado, mas a confirmação estatística ainda exige várias repetições na
mesma revisão e máquina ociosa.

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
| `01-foundations-and-divide-conquer/01-binary-exponentiation.clj` | 0.06 / 0.36 | -83.3% | 0.06 / 0.78 | -92.3% | 1.4 / 142.1 | -99.0% |
| `01-foundations-and-divide-conquer/02-horner-polynomial.clj` | 0.06 / 0.37 | -83.8% | 0.06 / 0.75 | -92.0% | 1.4 / 162.0 | -99.1% |
| `01-foundations-and-divide-conquer/03-prefix-range-sums.clj` | 0.07 / 0.43 | -83.7% | 0.07 / 0.86 | -91.9% | 11.6 / 229.7 | -95.0% |
| `01-foundations-and-divide-conquer/04-iterative-binary-search.clj` | 0.17 / 0.38 | -55.3% | 0.16 / 0.79 | -79.7% | 1.4 / 116.1 | -98.8% |
| `01-foundations-and-divide-conquer/05-maximum-subarray-divide.clj` | 1.38 / 0.45 | +206.7% | 1.38 / 0.93 | +48.4% | 5.2 / 197.4 | -97.4% |
| `02-sorting-and-order-statistics/01-insertion-sort.clj` | 0.51 / 0.51 | +0.0% | 0.51 / 0.97 | -47.4% | 18.2 / 364.7 | -95.0% |
| `02-sorting-and-order-statistics/02-selection-sort.clj` | 0.31 / 0.43 | -27.9% | 0.31 / 0.89 | -65.2% | 18.2 / 233.3 | -92.2% |
| `02-sorting-and-order-statistics/03-counting-sort.clj` | 0.21 / 0.50 | -58.0% | 0.21 / 0.99 | -78.8% | 14.4 / 371.9 | -96.1% |
| `02-sorting-and-order-statistics/04-merge-sort.clj` | 0.85 / 0.57 | +49.1% | 0.84 / 1.21 | -30.6% | 15.9 / 397.7 | -96.0% |
| `02-sorting-and-order-statistics/05-quickselect.clj` | 1.31 / 0.92 | +42.4% | 1.31 / 2.06 | -36.4% | 4.7 / 531.8 | -99.1% |
| `03-data-structures/01-build-max-heap.clj` | 0.31 / 0.42 | -26.2% | 0.30 / 0.90 | -66.7% | 18.2 / 235.8 | -92.3% |
| `03-data-structures/02-disjoint-set-union.clj` | 0.73 / 0.48 | +52.1% | 0.73 / 0.93 | -21.5% | 13.3 / 233.9 | -94.3% |
| `03-data-structures/03-chained-hash-table.clj` | 0.47 / 0.57 | -17.5% | 0.47 / 1.12 | -58.0% | 12.8 / 368.3 | -96.5% |
| `03-data-structures/04-circular-queue.clj` | 0.22 / 0.57 | -61.4% | 0.22 / 1.04 | -78.8% | 13.1 / 367.0 | -96.4% |
| `03-data-structures/05-binary-search-tree.clj` | 1.77 / 0.45 | +293.3% | 1.77 / 0.91 | +94.5% | 19.4 / 259.7 | -92.5% |
| `04-dynamic-programming-and-greedy/01-rod-cutting.clj` | 1.85 / 0.51 | +262.7% | 1.84 / 0.97 | +89.7% | 7.4 / 264.0 | -97.2% |
| `04-dynamic-programming-and-greedy/02-matrix-chain-order.clj` | 1.12 / 0.56 | +100.0% | 1.11 / 1.08 | +2.8% | 12.7 / 371.1 | -96.6% |
| `04-dynamic-programming-and-greedy/03-longest-common-subsequence.clj` | 2.93 / 0.81 | +261.7% | 2.92 / 1.38 | +111.6% | 14.9 / 474.1 | -96.8% |
| `04-dynamic-programming-and-greedy/04-zero-one-knapsack.clj` | 1.95 / 0.69 | +182.6% | 1.95 / 1.15 | +69.6% | 5.4 / 485.3 | -98.9% |
| `04-dynamic-programming-and-greedy/05-activity-selection.clj` | 0.05 / 0.34 | -85.3% | 0.05 / 0.73 | -93.2% | 1.4 / 114.8 | -98.7% |
| `05-graph-algorithms/01-breadth-first-search.clj` | 0.85 / 0.49 | +73.5% | 0.85 / 0.96 | -11.5% | 19.3 / 286.8 | -93.3% |
| `05-graph-algorithms/02-depth-first-search.clj` | 0.75 / 0.45 | +66.7% | 0.74 / 0.97 | -23.7% | 19.6 / 282.1 | -93.1% |
| `05-graph-algorithms/03-topological-sort.clj` | 1.04 / 0.54 | +92.6% | 1.03 / 1.04 | -1.0% | 18.8 / 364.5 | -94.8% |
| `05-graph-algorithms/04-bellman-ford.clj` | 0.36 / 0.44 | -18.2% | 0.36 / 0.88 | -59.1% | 14.8 / 178.9 | -91.7% |
| `05-graph-algorithms/05-floyd-warshall.clj` | 0.66 / 0.48 | +37.5% | 0.65 / 0.95 | -31.6% | 14.7 / 231.3 | -93.6% |
| `06-number-theory-and-string-matching/01-extended-euclid.clj` | 1.70 / 0.49 | +246.9% | 1.69 / 0.96 | +76.0% | 19.3 / 364.9 | -94.7% |
| `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj` | 3.50 / 1.80 | +94.4% | 3.50 / 2.41 | +45.2% | 21.3 / 1009.9 | -97.9% |
| `06-number-theory-and-string-matching/03-naive-string-matching.clj` | 0.46 / 0.38 | +21.1% | 0.46 / 0.83 | -44.6% | 1.5 / 121.9 | -98.8% |
| `06-number-theory-and-string-matching/04-rabin-karp.clj` | 0.25 / 0.61 | -59.0% | 0.25 / 1.11 | -77.5% | 1.5 / 368.6 | -99.6% |
| `06-number-theory-and-string-matching/05-knuth-morris-pratt.clj` | 0.18 / 0.39 | -53.8% | 0.17 / 0.80 | -78.8% | 11.8 / 150.0 | -92.1% |

## Resultado por capítulo

| Capítulo | Casos | Parede nativo | Parede Clojure | Razão Clojure/nativo |
| --- | ---: | ---: | ---: | ---: |
| Fundamentos e divisão e conquista | 5 | 1,74 s | 1,99 s | 1,144× |
| Ordenação e estatísticas de ordem | 5 | 3,19 s | 2,93 s | 0,918× |
| Estruturas de dados | 5 | 3,50 s | 2,49 s | 0,711× |
| Programação dinâmica e gulosa | 5 | 7,90 s | 2,91 s | 0,368× |
| Algoritmos de grafos | 5 | 3,66 s | 2,40 s | 0,656× |
| Teoria dos números e casamento de strings | 5 | 6,09 s | 3,67 s | 0,603× |

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

O total de parede ainda é 1,59× o Clojure/JVM, enquanto o nativo usa 17,2% menos CPU
acumulada e substancialmente menos memória. A maior concentração restante está em
programação dinâmica e estruturas de dados que ainda alocam ou atravessam fronteiras
de função. Essa relação é uma observação dos dados, não uma prova isolada de causalidade.

Os valores são uma fotografia desta máquina; frequência dinâmica, carga do sistema,
toolchain, JIT e sistema operacional afetam o resultado. Para conclusões estatísticas,
repita as medições no mesmo ambiente e compare distribuições, não apenas uma execução.

O compilador release foi reconstruído imediatamente antes da rodada no commit
`1dc69b5`.

## Experimento Cranelift `none` contra `speed`

Arquivos:

- [`cranelift-none-control.csv`](cranelift-none-control.csv)
- [`cranelift-speed.csv`](cranelift-speed.csv)

Os dois níveis foram executados pelo mesmo compilador release, na mesma máquina e com a
mesma escala:

```bash
benchmarks/cormen/compare-clojure.sh --scale 25 --opt-level none \
  --csv benchmarks/cormen/results/cranelift-none-control.csv

benchmarks/cormen/compare-clojure.sh --scale 25 --opt-level speed \
  --csv benchmarks/cormen/results/cranelift-speed.csv
```

| Métrica agregada | `none` | `speed` | Razão `none / speed` |
| --- | ---: | ---: | ---: |
| Tempo de parede | 93,08 s | 97,74 s | 0,952× |
| Tempo de CPU | 92,84 s | 97,49 s | 0,952× |

`speed` foi mais rápido em 5 casos e `none` em 25. A mediana da razão
`none / speed` foi 0,944× tanto para parede quanto para CPU, isto é, a configuração
`speed` apresentou regressão mediana de aproximadamente 5,9%.

A inspeção do KMP mostrou um efeito coerente com pressão de registradores: o frame de
`kmp-count` passou de 96 para 1.312 bytes e o símbolo cresceu de 5.024 para 7.208 bytes
(aproximadamente 44%). O assembly contém spills adicionais. Por isso, o gate rejeita
`speed` como padrão neste estágio; o nível continua disponível de forma explícita para
investigar e corrigir o IR do frontend.

## Validação do fast path de multiplicação

Comparação do baseline `18f58cc` com o commit candidato
`ADR-0006: fast path de fixnum para *`, ambos construídos em release e executados com
`--scale 25 --opt-level none`:

- [`mul-fastpath-before.csv`](mul-fastpath-before.csv)
- [`mul-fastpath-after.csv`](mul-fastpath-after.csv)

| Métrica | Antes | Depois | Variação |
| --- | ---: | ---: | ---: |
| Tempo de parede acumulado | 91,01 s | 91,93 s | +1,01% |
| Tempo de CPU acumulado | 90,78 s | 91,64 s | +0,95% |
| Compilação acumulada | 2.638 ms | 2.659 ms | +0,80% |

O resultado global ficou ligeiramente mais lento: 13 casos melhoraram, 15 pioraram e 2
empataram. A mediana por caso variou +0,37% em parede e 0,00% em CPU, portanto o delta
agregado de 1% deve ser tratado como pequeno e sujeito a ruído. O caso diretamente
favorável mais claro foi `binary-exponentiation`, de 0,20 s para 0,17 s (-15%).

Somando as duas suítes, o tempo de parede passou de 113,58 s para 114,43 s (+0,75%) e o
tempo de CPU de 113,15 s para 113,86 s (+0,63%). O fast path ajuda cargas concentradas
em multiplicação, mas não produziu ganho global mensurável nesta rodada.
