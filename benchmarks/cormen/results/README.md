# Comparação extrema de referência — Cormen/CLRS

[Catálogo dos benchmarks](../../README.md) ·
[Guia da suíte](../README.md)

Arquivo: [`extreme.csv`](extreme.csv)

Medições nativas atualizadas em 2026-07-27 no commit `663d2d4` com:

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
- Mediana de `wall_speedup_vs_clojure`: 0,726×.
- Tempos de parede acumulados: nativo 36,21 s; Clojure/JVM preservado 16,91 s.
- Tempos de CPU acumulados: nativo 36,07 s; Clojure/JVM preservado 32,61 s.
- O nativo teve menor tempo de CPU em 18 casos; Clojure/JVM em 12.
- Mediana de `cpu_speedup_vs_clojure`: 1,515× a favor do nativo, embora os casos mais
  lentos mantenham o total de CPU nativo maior.
- O nativo apresentou RSS menor nos 30 casos.
- Mediana de `rss_ratio_clojure_over_native`: 20,297×.
- Maior RSS nativo: 21.888 KiB em
  `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj`.
- Maior RSS Clojure/JVM: 1.024.412 KiB no mesmo caso.
- Compilação acumulada: 3.959 ms no nativo e 15.272 ms preservados no Clojure/JVM AOT.

Foram feitas três execuções completas, com tempos acumulados de parede de 35,96 s,
41,04 s e 36,21 s e CPU de 35,77 s, 40,89 s e 36,07 s. O `extreme.csv` guarda
integralmente a execução mediana pelo tempo de parede, de 36,21 s, sem combinar
métricas de processos distintos.

Em relação ao CSV versionado imediatamente anterior, a execução publicada melhorou
7,25% em parede e 7,20% em CPU; 18 casos melhoraram, 5 empataram e 7 pioraram. O crivo
de Eratóstenes caiu de 5,93 s para 4,99 s (-15,9%), `counting-sort` de 0,98 s para
0,55 s (-43,9%), `merge-sort` de 1,46 s para 1,12 s (-23,3%) e `prefix-range-sums`
de 0,50 s para 0,33 s (-34,0%). O maior movimento contrário foi `bellman-ford`, de
0,48 s para 0,60 s. O ganho agregado persistiu em duas das três repetições; a segunda,
de 41,04 s, foi tratada como outlier e não foi publicada.

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
| `01-foundations-and-divide-conquer/01-binary-exponentiation.clj` | 0.06 / 0.38 | -84.2% | 0.06 / 0.82 | -92.7% | 1.4 / 148.5 | -99.1% |
| `01-foundations-and-divide-conquer/02-horner-polynomial.clj` | 0.49 / 0.38 | +28.9% | 0.48 / 0.82 | -41.5% | 19.0 / 164.4 | -88.4% |
| `01-foundations-and-divide-conquer/03-prefix-range-sums.clj` | 0.33 / 0.43 | -23.3% | 0.32 / 0.90 | -64.4% | 18.1 / 231.8 | -92.2% |
| `01-foundations-and-divide-conquer/04-iterative-binary-search.clj` | 0.14 / 0.40 | -65.0% | 0.14 / 0.85 | -83.5% | 1.5 / 117.8 | -98.8% |
| `01-foundations-and-divide-conquer/05-maximum-subarray-divide.clj` | 1.39 / 0.48 | +189.6% | 1.39 / 0.99 | +40.4% | 5.3 / 197.5 | -97.3% |
| `02-sorting-and-order-statistics/01-insertion-sort.clj` | 0.80 / 0.52 | +53.8% | 0.80 / 0.99 | -19.2% | 18.4 / 362.4 | -94.9% |
| `02-sorting-and-order-statistics/02-selection-sort.clj` | 0.49 / 0.45 | +8.9% | 0.49 / 0.92 | -46.7% | 18.5 / 230.6 | -92.0% |
| `02-sorting-and-order-statistics/03-counting-sort.clj` | 0.55 / 0.52 | +5.8% | 0.54 / 1.00 | -46.0% | 18.0 / 363.8 | -95.1% |
| `02-sorting-and-order-statistics/04-merge-sort.clj` | 1.12 / 0.61 | +83.6% | 1.11 / 1.28 | -13.3% | 16.3 / 396.1 | -95.9% |
| `02-sorting-and-order-statistics/05-quickselect.clj` | 1.28 / 0.92 | +39.1% | 1.27 / 1.91 | -33.5% | 4.8 / 521.5 | -99.1% |
| `03-data-structures/01-build-max-heap.clj` | 0.48 / 0.45 | +6.7% | 0.48 / 0.95 | -49.5% | 18.5 / 233.7 | -92.1% |
| `03-data-structures/02-disjoint-set-union.clj` | 1.06 / 0.51 | +107.8% | 1.05 / 1.04 | +1.0% | 15.9 / 241.9 | -93.4% |
| `03-data-structures/03-chained-hash-table.clj` | 0.68 / 0.60 | +13.3% | 0.68 / 1.19 | -42.9% | 14.1 / 366.6 | -96.1% |
| `03-data-structures/04-circular-queue.clj` | 0.41 / 0.58 | -29.3% | 0.40 / 1.09 | -63.3% | 17.9 / 364.5 | -95.1% |
| `03-data-structures/05-binary-search-tree.clj` | 2.20 / 0.46 | +378.3% | 2.19 / 0.96 | +128.1% | 19.5 / 258.0 | -92.4% |
| `04-dynamic-programming-and-greedy/01-rod-cutting.clj` | 1.94 / 0.53 | +266.0% | 1.94 / 1.00 | +94.0% | 7.9 / 255.2 | -96.9% |
| `04-dynamic-programming-and-greedy/02-matrix-chain-order.clj` | 1.43 / 0.57 | +150.9% | 1.42 / 1.08 | +31.5% | 14.3 / 366.2 | -96.1% |
| `04-dynamic-programming-and-greedy/03-longest-common-subsequence.clj` | 3.86 / 0.81 | +376.5% | 3.85 / 1.37 | +181.0% | 15.6 / 477.4 | -96.7% |
| `04-dynamic-programming-and-greedy/04-zero-one-knapsack.clj` | 3.70 / 0.71 | +421.1% | 3.70 / 1.21 | +205.8% | 13.2 / 509.1 | -97.4% |
| `04-dynamic-programming-and-greedy/05-activity-selection.clj` | 0.04 / 0.34 | -88.2% | 0.04 / 0.76 | -94.7% | 1.5 / 117.1 | -98.7% |
| `05-graph-algorithms/01-breadth-first-search.clj` | 1.22 / 0.50 | +144.0% | 1.22 / 1.03 | +18.4% | 19.2 / 288.7 | -93.3% |
| `05-graph-algorithms/02-depth-first-search.clj` | 1.39 / 0.49 | +183.7% | 1.38 / 1.05 | +31.4% | 19.5 / 284.3 | -93.1% |
| `05-graph-algorithms/03-topological-sort.clj` | 1.59 / 0.56 | +183.9% | 1.59 / 1.11 | +43.2% | 18.9 / 372.4 | -94.9% |
| `05-graph-algorithms/04-bellman-ford.clj` | 0.60 / 0.44 | +36.4% | 0.59 / 0.90 | -34.4% | 16.4 / 171.6 | -90.5% |
| `05-graph-algorithms/05-floyd-warshall.clj` | 0.69 / 0.51 | +35.3% | 0.68 / 1.06 | -35.8% | 14.9 / 234.9 | -93.7% |
| `06-number-theory-and-string-matching/01-extended-euclid.clj` | 2.39 / 0.52 | +359.6% | 2.38 / 1.05 | +126.7% | 19.5 / 372.3 | -94.8% |
| `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj` | 4.99 / 1.83 | +172.7% | 4.99 / 2.46 | +102.8% | 21.4 / 1000.4 | -97.9% |
| `06-number-theory-and-string-matching/03-naive-string-matching.clj` | 0.37 / 0.39 | -5.1% | 0.37 / 0.83 | -55.4% | 1.5 / 116.8 | -98.7% |
| `06-number-theory-and-string-matching/04-rabin-karp.clj` | 0.19 / 0.62 | -69.4% | 0.19 / 1.14 | -83.3% | 1.5 / 370.7 | -99.6% |
| `06-number-theory-and-string-matching/05-knuth-morris-pratt.clj` | 0.33 / 0.40 | -17.5% | 0.33 / 0.85 | -61.2% | 17.6 / 143.3 | -87.7% |
## Resultado por capítulo

| Capítulo | Casos | Parede nativo | Parede Clojure | Razão Clojure/nativo |
| --- | ---: | ---: | ---: | ---: |
| Fundamentos e divisão e conquista | 5 | 2,41 s | 2,07 s | 0,859× |
| Ordenação e estatísticas de ordem | 5 | 4,24 s | 3,02 s | 0,712× |
| Estruturas de dados | 5 | 4,83 s | 2,60 s | 0,538× |
| Programação dinâmica e gulosa | 5 | 10,97 s | 2,96 s | 0,270× |
| Algoritmos de grafos | 5 | 5,49 s | 2,50 s | 0,455× |
| Teoria dos números e casamento de strings | 5 | 8,27 s | 3,76 s | 0,455× |

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
