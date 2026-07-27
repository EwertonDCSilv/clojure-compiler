# Comparação extrema de referência — Cormen/CLRS

[Catálogo dos benchmarks](../../README.md) ·
[Guia da suíte](../README.md)

Arquivo: [`extreme.csv`](extreme.csv)

Medições nativas atualizadas em 2026-07-27 no commit `8012102` com:

```bash
benchmarks/cormen/run.sh --scale 25 --opt-level none \
  --compiler target/release/clojure-native \
  --csv /tmp/clojure-compiler-cormen-native.csv
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

- 30 casos comparados, todos com status `OK` e checksums idênticos.
- O nativo teve menor tempo de parede em 8 casos; Clojure/JVM em 22.
- Mediana de `wall_speedup_vs_clojure`: 0,637×.
- Tempos de parede acumulados: nativo 39,04 s; Clojure/JVM preservado 16,91 s.
- Tempos de CPU acumulados: nativo 38,87 s; Clojure/JVM preservado 32,61 s.
- O nativo teve menor tempo de CPU em 17 casos; Clojure/JVM em 13.
- Mediana de `cpu_speedup_vs_clojure`: 1,310× a favor do nativo, embora os casos mais
  lentos mantenham o total de CPU nativo maior.
- O nativo apresentou RSS menor nos 30 casos.
- Mediana de `rss_ratio_clojure_over_native`: 19,709×.
- Maior RSS nativo: 20.740 KiB em
  `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj`.
- Maior RSS Clojure/JVM: 1.024.412 KiB no mesmo caso.
- Compilação acumulada: 4.034 ms no nativo e 15.272 ms preservados no Clojure/JVM AOT.

Em relação ao CSV versionado imediatamente anterior, os tempos acumulados de parede e
CPU variaram respectivamente -0,23% e -0,26%, ou seja, permaneceram estáveis.

Antes da publicação foram feitas três repetições consecutivas adicionais, com tempos
de parede de 39,31 s, 39,75 s e 39,63 s. Incluindo a rodada publicada de 39,04 s, a
mediana foi 39,47 s, apenas 0,87% acima da referência anterior de 39,13 s. A execução
isolada de 51,74 s observada durante a triagem não se repetiu. O `extreme.csv` guarda a
rodada publicada completa, em vez de combinar medianas de métricas de processos
distintos.

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
| `01-foundations-and-divide-conquer/01-binary-exponentiation.clj` | 0.06 / 0.38 | -84.2% | 0.06 / 0.82 | -92.7% | 1.5 / 148.5 | -99.0% |
| `01-foundations-and-divide-conquer/02-horner-polynomial.clj` | 0.61 / 0.38 | +60.5% | 0.60 / 0.82 | -26.8% | 19.0 / 164.4 | -88.4% |
| `01-foundations-and-divide-conquer/03-prefix-range-sums.clj` | 0.50 / 0.43 | +16.3% | 0.49 / 0.90 | -45.6% | 18.6 / 231.8 | -92.0% |
| `01-foundations-and-divide-conquer/04-iterative-binary-search.clj` | 0.14 / 0.40 | -65.0% | 0.14 / 0.85 | -83.5% | 1.5 / 117.8 | -98.8% |
| `01-foundations-and-divide-conquer/05-maximum-subarray-divide.clj` | 1.36 / 0.48 | +183.3% | 1.36 / 0.99 | +37.4% | 5.2 / 197.5 | -97.3% |
| `02-sorting-and-order-statistics/01-insertion-sort.clj` | 0.80 / 0.52 | +53.8% | 0.79 / 0.99 | -20.2% | 18.4 / 362.4 | -94.9% |
| `02-sorting-and-order-statistics/02-selection-sort.clj` | 0.48 / 0.45 | +6.7% | 0.47 / 0.92 | -48.9% | 18.5 / 230.6 | -92.0% |
| `02-sorting-and-order-statistics/03-counting-sort.clj` | 0.98 / 0.52 | +88.5% | 0.97 / 1.00 | -3.0% | 18.6 / 363.8 | -94.9% |
| `02-sorting-and-order-statistics/04-merge-sort.clj` | 1.46 / 0.61 | +139.3% | 1.46 / 1.28 | +14.1% | 19.4 / 396.1 | -95.1% |
| `02-sorting-and-order-statistics/05-quickselect.clj` | 1.37 / 0.92 | +48.9% | 1.37 / 1.91 | -28.3% | 4.8 / 521.5 | -99.1% |
| `03-data-structures/01-build-max-heap.clj` | 0.42 / 0.45 | -6.7% | 0.42 / 0.95 | -55.8% | 18.5 / 233.7 | -92.1% |
| `03-data-structures/02-disjoint-set-union.clj` | 1.14 / 0.51 | +123.5% | 1.13 / 1.04 | +8.7% | 15.9 / 241.9 | -93.4% |
| `03-data-structures/03-chained-hash-table.clj` | 0.71 / 0.60 | +18.3% | 0.70 / 1.19 | -41.2% | 14.1 / 366.6 | -96.1% |
| `03-data-structures/04-circular-queue.clj` | 0.55 / 0.58 | -5.2% | 0.55 / 1.09 | -49.5% | 18.5 / 364.5 | -94.9% |
| `03-data-structures/05-binary-search-tree.clj` | 2.25 / 0.46 | +389.1% | 2.24 / 0.96 | +133.3% | 19.5 / 258.0 | -92.4% |
| `04-dynamic-programming-and-greedy/01-rod-cutting.clj` | 2.00 / 0.53 | +277.4% | 1.99 / 1.00 | +99.0% | 8.1 / 255.2 | -96.8% |
| `04-dynamic-programming-and-greedy/02-matrix-chain-order.clj` | 1.60 / 0.57 | +180.7% | 1.59 / 1.08 | +47.2% | 14.3 / 366.2 | -96.1% |
| `04-dynamic-programming-and-greedy/03-longest-common-subsequence.clj` | 4.04 / 0.81 | +398.8% | 4.03 / 1.37 | +194.2% | 15.5 / 477.4 | -96.8% |
| `04-dynamic-programming-and-greedy/04-zero-one-knapsack.clj` | 3.76 / 0.71 | +429.6% | 3.75 / 1.21 | +209.9% | 13.2 / 509.1 | -97.4% |
| `04-dynamic-programming-and-greedy/05-activity-selection.clj` | 0.04 / 0.34 | -88.2% | 0.04 / 0.76 | -94.7% | 1.5 / 117.1 | -98.8% |
| `05-graph-algorithms/01-breadth-first-search.clj` | 1.28 / 0.50 | +156.0% | 1.27 / 1.03 | +23.3% | 19.4 / 288.7 | -93.3% |
| `05-graph-algorithms/02-depth-first-search.clj` | 1.38 / 0.49 | +181.6% | 1.37 / 1.05 | +30.5% | 19.4 / 284.3 | -93.2% |
| `05-graph-algorithms/03-topological-sort.clj` | 1.56 / 0.56 | +178.6% | 1.55 / 1.11 | +39.6% | 19.0 / 372.4 | -94.9% |
| `05-graph-algorithms/04-bellman-ford.clj` | 0.48 / 0.44 | +9.1% | 0.47 / 0.90 | -47.8% | 16.4 / 171.6 | -90.5% |
| `05-graph-algorithms/05-floyd-warshall.clj` | 0.65 / 0.51 | +27.5% | 0.65 / 1.06 | -38.7% | 14.8 / 234.9 | -93.7% |
| `06-number-theory-and-string-matching/01-extended-euclid.clj` | 2.53 / 0.52 | +386.5% | 2.53 / 1.05 | +141.0% | 19.4 / 372.3 | -94.8% |
| `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj` | 5.93 / 1.83 | +224.0% | 5.93 / 2.46 | +141.1% | 20.3 / 1000.4 | -98.0% |
| `06-number-theory-and-string-matching/03-naive-string-matching.clj` | 0.38 / 0.39 | -2.6% | 0.38 / 0.83 | -54.2% | 1.5 / 116.8 | -98.7% |
| `06-number-theory-and-string-matching/04-rabin-karp.clj` | 0.19 / 0.62 | -69.4% | 0.19 / 1.14 | -83.3% | 1.5 / 370.7 | -99.6% |
| `06-number-theory-and-string-matching/05-knuth-morris-pratt.clj` | 0.39 / 0.40 | -2.5% | 0.38 / 0.85 | -55.3% | 18.7 / 143.3 | -86.9% |

## Resultado por capítulo

| Capítulo | Casos | Parede nativo | Parede Clojure | Razão Clojure/nativo |
| --- | ---: | ---: | ---: | ---: |
| Fundamentos e divisão e conquista | 5 | 2,67 s | 2,07 s | 0,775× |
| Ordenação e estatísticas de ordem | 5 | 5,09 s | 3,02 s | 0,593× |
| Estruturas de dados | 5 | 5,07 s | 2,60 s | 0,513× |
| Programação dinâmica e gulosa | 5 | 11,44 s | 2,96 s | 0,259× |
| Algoritmos de grafos | 5 | 5,35 s | 2,50 s | 0,467× |
| Teoria dos números e casamento de strings | 5 | 9,42 s | 3,76 s | 0,399× |

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

Mesmo depois dos fast paths inteiros, dos stores diretos na shadow stack e do fast path
de `nth`, o total de parede dos casos algorítmicos continua maior que no Clojure/JVM,
apesar da mediana de CPU agora favorecer o nativo e do consumo de memória ser
substancialmente menor. Essa relação é uma observação dos dados, não uma prova isolada
de causalidade; ela orienta as próximas etapas de liveness, IR e safepoints da ADR 0006.

Os valores são uma fotografia desta máquina; frequência dinâmica, carga do sistema,
toolchain, JIT e sistema operacional afetam o resultado. Para conclusões estatísticas,
repita as medições no mesmo ambiente e compare distribuições, não apenas uma execução.

O compilador release foi reconstruído imediatamente antes da rodada. O `runtime.c`
medido tinha SHA-256
`160fb7fd7fe2478177f9f3ef5cd95a7b7fbf987e7e2c61dd51b11bdeb38d14c6`.

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
