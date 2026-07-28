# Comparação extrema de referência — Cormen/CLRS

[Catálogo dos benchmarks](../../README.md) ·
[Guia da suíte](../README.md)

Arquivo: [`extreme.csv`](extreme.csv)

Snapshot do relatório:
[`HEAD 3e71bc1`](https://github.com/EwertonDCSilv/clojure-compiler/commit/3e71bc1996b689233c80516b4b4aff52259c2cdf).

Medições Native × Clojure/JVM refeitas em 2026-07-28 no commit `3e71bc1` com:

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
- O nativo teve menor tempo de parede em 13 casos; Clojure/JVM em 17.
- Mediana de `wall_speedup_vs_clojure`: 0,766×.
- Tempos de parede acumulados: nativo 30,06 s; Clojure/JVM 17,01 s.
- Tempos de CPU acumulados: nativo 29,95 s; Clojure/JVM 32,66 s.
- O nativo teve menor tempo de CPU em 20 casos; Clojure/JVM em 10.
- Mediana de `cpu_speedup_vs_clojure`: 1,537×; o total de CPU ainda favorece o nativo.
- O nativo apresentou RSS menor nos 30 casos.
- Mediana de `rss_ratio_clojure_over_native`: 26,753×.
- Maior RSS nativo: 21,3 MiB em
  `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj`.
- Maior RSS Clojure/JVM: 989,9 MiB no mesmo caso.
- Compilação acumulada: 5.830 ms no nativo e 15.165 ms no Clojure/JVM AOT.

Esta é uma execução completa única. Em relação ao snapshot anterior, o agregado nativo
subiu de 26,08 para 30,06 s de parede (+15,3%) e de 25,97 para 29,95 s de CPU
(+15,3%). A JVM também foi medida novamente e subiu menos; portanto, a rodada é um
sinal de regressão que precisa do controle pareado definido pela ADR-0014 antes de ser
atribuído a uma mudança específica.

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
| `01-foundations-and-divide-conquer/01-binary-exponentiation.clj` | 0.06 / 0.38 | -84.2% | 0.06 / 0.84 | -92.9% | 1.4 / 148.5 | -99.0% |
| `01-foundations-and-divide-conquer/02-horner-polynomial.clj` | 0.06 / 0.39 | -84.6% | 0.06 / 0.84 | -92.9% | 1.4 / 169.3 | -99.1% |
| `01-foundations-and-divide-conquer/03-prefix-range-sums.clj` | 0.07 / 0.45 | -84.4% | 0.07 / 0.92 | -92.4% | 11.8 / 232.5 | -94.9% |
| `01-foundations-and-divide-conquer/04-iterative-binary-search.clj` | 0.17 / 0.40 | -57.5% | 0.17 / 0.83 | -79.5% | 1.6 / 115.9 | -98.6% |
| `01-foundations-and-divide-conquer/05-maximum-subarray-divide.clj` | 1.42 / 0.48 | +195.8% | 1.42 / 0.98 | +44.9% | 5.2 / 197.3 | -97.4% |
| `02-sorting-and-order-statistics/01-insertion-sort.clj` | 0.63 / 0.52 | +21.2% | 0.62 / 0.98 | -36.7% | 18.1 / 362.7 | -95.0% |
| `02-sorting-and-order-statistics/02-selection-sort.clj` | 0.36 / 0.45 | -20.0% | 0.36 / 0.93 | -61.3% | 18.2 / 231.8 | -92.1% |
| `02-sorting-and-order-statistics/03-counting-sort.clj` | 0.23 / 0.53 | -56.6% | 0.23 / 1.03 | -77.7% | 14.6 / 373.1 | -96.1% |
| `02-sorting-and-order-statistics/04-merge-sort.clj` | 1.02 / 0.61 | +67.2% | 1.01 / 1.31 | -22.9% | 16.0 / 395.8 | -96.0% |
| `02-sorting-and-order-statistics/05-quickselect.clj` | 1.37 / 0.96 | +42.7% | 1.36 / 2.03 | -33.0% | 4.7 / 530.9 | -99.1% |
| `03-data-structures/01-build-max-heap.clj` | 0.31 / 0.46 | -32.6% | 0.30 / 0.94 | -68.1% | 18.2 / 233.5 | -92.2% |
| `03-data-structures/02-disjoint-set-union.clj` | 0.79 / 0.50 | +58.0% | 0.78 / 0.99 | -21.2% | 13.4 / 232.0 | -94.2% |
| `03-data-structures/03-chained-hash-table.clj` | 0.53 / 0.59 | -10.2% | 0.53 / 1.17 | -54.7% | 12.8 / 364.8 | -96.5% |
| `03-data-structures/04-circular-queue.clj` | 0.22 / 0.56 | -60.7% | 0.21 / 1.04 | -79.8% | 13.1 / 364.7 | -96.4% |
| `03-data-structures/05-binary-search-tree.clj` | 2.07 / 0.46 | +350.0% | 2.06 / 0.99 | +108.1% | 19.4 / 266.4 | -92.7% |
| `04-dynamic-programming-and-greedy/01-rod-cutting.clj` | 1.89 / 0.53 | +256.6% | 1.89 / 0.98 | +92.9% | 7.4 / 255.2 | -97.1% |
| `04-dynamic-programming-and-greedy/02-matrix-chain-order.clj` | 1.30 / 0.57 | +128.1% | 1.30 / 1.11 | +17.1% | 12.8 / 365.7 | -96.5% |
| `04-dynamic-programming-and-greedy/03-longest-common-subsequence.clj` | 3.52 / 0.82 | +329.3% | 3.51 / 1.40 | +150.7% | 14.7 / 471.1 | -96.9% |
| `04-dynamic-programming-and-greedy/04-zero-one-knapsack.clj` | 2.10 / 0.71 | +195.8% | 2.10 / 1.24 | +69.4% | 5.4 / 472.4 | -98.8% |
| `04-dynamic-programming-and-greedy/05-activity-selection.clj` | 0.05 / 0.35 | -85.7% | 0.05 / 0.79 | -93.7% | 1.4 / 119.7 | -98.8% |
| `05-graph-algorithms/01-breadth-first-search.clj` | 1.16 / 0.49 | +136.7% | 1.15 / 1.01 | +13.9% | 19.3 / 288.2 | -93.3% |
| `05-graph-algorithms/02-depth-first-search.clj` | 0.86 / 0.47 | +83.0% | 0.86 / 1.00 | -14.0% | 19.6 / 281.5 | -93.0% |
| `05-graph-algorithms/03-topological-sort.clj` | 1.43 / 0.56 | +155.4% | 1.42 / 1.08 | +31.5% | 18.8 / 368.4 | -94.9% |
| `05-graph-algorithms/04-bellman-ford.clj` | 0.40 / 0.45 | -11.1% | 0.40 / 0.90 | -55.6% | 14.8 / 173.1 | -91.4% |
| `05-graph-algorithms/05-floyd-warshall.clj` | 0.72 / 0.51 | +41.2% | 0.71 / 1.01 | -29.7% | 14.8 / 233.3 | -93.6% |
| `06-number-theory-and-string-matching/01-extended-euclid.clj` | 2.12 / 0.54 | +292.6% | 2.12 / 1.01 | +109.9% | 19.4 / 362.4 | -94.6% |
| `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj` | 4.28 / 1.82 | +135.2% | 4.28 / 2.45 | +74.7% | 21.3 / 989.9 | -97.8% |
| `06-number-theory-and-string-matching/03-naive-string-matching.clj` | 0.47 / 0.41 | +14.6% | 0.47 / 0.87 | -46.0% | 1.4 / 115.6 | -98.8% |
| `06-number-theory-and-string-matching/04-rabin-karp.clj` | 0.27 / 0.63 | -57.1% | 0.27 / 1.15 | -76.5% | 1.4 / 365.1 | -99.6% |
| `06-number-theory-and-string-matching/05-knuth-morris-pratt.clj` | 0.18 / 0.41 | -56.1% | 0.18 / 0.84 | -78.6% | 11.7 / 144.1 | -91.9% |

## Resultado por capítulo

| Capítulo | Casos | Parede nativo | Parede Clojure | Razão Clojure/nativo |
| --- | ---: | ---: | ---: | ---: |
| Fundamentos e divisão e conquista | 5 | 1,78 s | 2,10 s | 1,180× |
| Ordenação e estatísticas de ordem | 5 | 3,61 s | 3,07 s | 0,850× |
| Estruturas de dados | 5 | 3,92 s | 2,57 s | 0,656× |
| Programação dinâmica e gulosa | 5 | 8,86 s | 2,98 s | 0,336× |
| Algoritmos de grafos | 5 | 4,57 s | 2,48 s | 0,543× |
| Teoria dos números e casamento de strings | 5 | 7,32 s | 3,81 s | 0,520× |

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

O total de parede é 1,77× o Clojure/JVM, enquanto o nativo usa 8,3% menos CPU acumulada
e substancialmente menos memória. A maior concentração restante está em
programação dinâmica e estruturas de dados que ainda alocam ou atravessam fronteiras
de função. Essa relação é uma observação dos dados, não uma prova isolada de causalidade.

Os valores são uma fotografia desta máquina; frequência dinâmica, carga do sistema,
toolchain, JIT e sistema operacional afetam o resultado. Para conclusões estatísticas,
repita as medições no mesmo ambiente e compare distribuições, não apenas uma execução.

O compilador release foi reconstruído imediatamente antes da rodada no commit
`3e71bc1`.

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
