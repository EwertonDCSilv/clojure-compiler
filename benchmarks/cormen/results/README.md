# Comparação extrema de referência — Cormen/CLRS

[Catálogo dos benchmarks](../../README.md) ·
[Guia da suíte](../README.md)

Arquivo: [`extreme.csv`](extreme.csv)

Snapshot do relatório:
[`HEAD 424ba20`](https://github.com/EwertonDCSilv/clojure-compiler/commit/424ba20e88fd91a641675e4d9d9bf111c63fc164).

Medições Native × Clojure/JVM refeitas em 2026-07-28 no commit `424ba20` com:

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
- O nativo teve menor tempo de parede em 14 casos; Clojure/JVM em 16.
- Mediana de `wall_speedup_vs_clojure`: 0,846×.
- Tempos de parede acumulados: nativo 27,23 s; Clojure/JVM 16,95 s.
- Tempos de CPU acumulados: nativo 27,09 s; Clojure/JVM 32,08 s.
- O nativo teve menor tempo de CPU em 21 casos; Clojure/JVM em 9.
- Mediana de `cpu_speedup_vs_clojure`: 1,685×; o total de CPU ainda favorece o nativo.
- O nativo apresentou RSS menor nos 30 casos.
- Mediana de `rss_ratio_clojure_over_native`: 27,018×.
- Maior RSS nativo: 21,4 MiB em
  `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj`.
- Maior RSS Clojure/JVM: 1.008,4 MiB no mesmo caso.
- Compilação acumulada: 5.781 ms no nativo e 15.171 ms no Clojure/JVM AOT.

Esta é uma execução completa única. Em relação ao snapshot anterior, o agregado nativo
caiu de 30,06 para 27,23 s de parede (-9,4%) e de 29,95 para 27,09 s de CPU
(-9,5%). A JVM também foi medida novamente e permaneceu próxima do snapshot anterior;
a rodada isolada não substitui o controle pareado definido pela ADR-0014.

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
| `01-foundations-and-divide-conquer/01-binary-exponentiation.clj` | 0.07 / 0.38 | -81.6% | 0.07 / 0.80 | -91.2% | 1.4 / 149.2 | -99.0% |
| `01-foundations-and-divide-conquer/02-horner-polynomial.clj` | 0.06 / 0.37 | -83.8% | 0.06 / 0.80 | -92.5% | 1.4 / 168.6 | -99.1% |
| `01-foundations-and-divide-conquer/03-prefix-range-sums.clj` | 0.09 / 0.40 | -77.5% | 0.08 / 0.82 | -90.2% | 11.7 / 229.2 | -94.9% |
| `01-foundations-and-divide-conquer/04-iterative-binary-search.clj` | 0.17 / 0.37 | -54.1% | 0.17 / 0.76 | -77.6% | 1.4 / 112.5 | -98.7% |
| `01-foundations-and-divide-conquer/05-maximum-subarray-divide.clj` | 1.43 / 0.45 | +217.8% | 1.43 / 0.89 | +60.7% | 5.2 / 196.7 | -97.4% |
| `02-sorting-and-order-statistics/01-insertion-sort.clj` | 0.52 / 0.53 | -1.9% | 0.51 / 0.89 | -42.7% | 18.3 / 359.4 | -94.9% |
| `02-sorting-and-order-statistics/02-selection-sort.clj` | 0.31 / 0.44 | -29.5% | 0.30 / 0.89 | -66.3% | 18.1 / 232.2 | -92.2% |
| `02-sorting-and-order-statistics/03-counting-sort.clj` | 0.23 / 0.57 | -59.6% | 0.23 / 1.10 | -79.1% | 14.4 / 372.0 | -96.1% |
| `02-sorting-and-order-statistics/04-merge-sort.clj` | 0.88 / 0.59 | +49.2% | 0.87 / 1.32 | -34.1% | 16.1 / 395.2 | -95.9% |
| `02-sorting-and-order-statistics/05-quickselect.clj` | 1.31 / 0.95 | +37.9% | 1.31 / 1.78 | -26.4% | 4.7 / 501.1 | -99.1% |
| `03-data-structures/01-build-max-heap.clj` | 0.28 / 0.45 | -37.8% | 0.28 / 0.93 | -69.9% | 18.2 / 234.0 | -92.2% |
| `03-data-structures/02-disjoint-set-union.clj` | 0.75 / 0.53 | +41.5% | 0.74 / 1.00 | -26.0% | 13.3 / 231.6 | -94.2% |
| `03-data-structures/03-chained-hash-table.clj` | 0.47 / 0.59 | -20.3% | 0.46 / 1.17 | -60.7% | 12.9 / 366.9 | -96.5% |
| `03-data-structures/04-circular-queue.clj` | 0.22 / 0.56 | -60.7% | 0.21 / 1.05 | -80.0% | 13.1 / 371.8 | -96.5% |
| `03-data-structures/05-binary-search-tree.clj` | 1.74 / 0.47 | +270.2% | 1.73 / 0.96 | +80.2% | 19.3 / 259.7 | -92.6% |
| `04-dynamic-programming-and-greedy/01-rod-cutting.clj` | 1.83 / 0.53 | +245.3% | 1.83 / 0.96 | +90.6% | 7.4 / 254.1 | -97.1% |
| `04-dynamic-programming-and-greedy/02-matrix-chain-order.clj` | 1.18 / 0.58 | +103.4% | 1.18 / 1.06 | +11.3% | 12.8 / 362.6 | -96.5% |
| `04-dynamic-programming-and-greedy/03-longest-common-subsequence.clj` | 3.16 / 0.81 | +290.1% | 3.15 / 1.34 | +135.1% | 14.8 / 465.4 | -96.8% |
| `04-dynamic-programming-and-greedy/04-zero-one-knapsack.clj` | 2.01 / 0.71 | +183.1% | 2.01 / 1.18 | +70.3% | 5.6 / 462.5 | -98.8% |
| `04-dynamic-programming-and-greedy/05-activity-selection.clj` | 0.05 / 0.35 | -85.7% | 0.05 / 0.79 | -93.7% | 1.5 / 115.3 | -98.7% |
| `05-graph-algorithms/01-breadth-first-search.clj` | 0.91 / 0.50 | +82.0% | 0.91 / 1.02 | -10.8% | 19.3 / 290.1 | -93.3% |
| `05-graph-algorithms/02-depth-first-search.clj` | 0.81 / 0.47 | +72.3% | 0.81 / 1.01 | -19.8% | 19.6 / 282.0 | -93.1% |
| `05-graph-algorithms/03-topological-sort.clj` | 1.13 / 0.56 | +101.8% | 1.12 / 1.09 | +2.8% | 18.8 / 366.1 | -94.9% |
| `05-graph-algorithms/04-bellman-ford.clj` | 0.38 / 0.45 | -15.6% | 0.37 / 0.95 | -61.1% | 14.9 / 181.4 | -91.8% |
| `05-graph-algorithms/05-floyd-warshall.clj` | 0.64 / 0.50 | +28.0% | 0.64 / 1.04 | -38.5% | 14.8 / 239.1 | -93.8% |
| `06-number-theory-and-string-matching/01-extended-euclid.clj` | 1.84 / 0.53 | +247.2% | 1.84 / 1.04 | +76.9% | 19.3 / 371.3 | -94.8% |
| `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj` | 3.87 / 1.87 | +107.0% | 3.86 / 2.55 | +51.4% | 21.4 / 1008.4 | -97.9% |
| `06-number-theory-and-string-matching/03-naive-string-matching.clj` | 0.45 / 0.41 | +9.8% | 0.44 / 0.87 | -49.4% | 1.4 / 117.3 | -98.8% |
| `06-number-theory-and-string-matching/04-rabin-karp.clj` | 0.26 / 0.62 | -58.1% | 0.26 / 1.15 | -77.4% | 1.5 / 369.4 | -99.6% |
| `06-number-theory-and-string-matching/05-knuth-morris-pratt.clj` | 0.18 / 0.41 | -56.1% | 0.17 / 0.87 | -80.5% | 11.7 / 150.9 | -92.2% |
## Resultado por capítulo

| Capítulo | Casos | Parede nativo | Parede Clojure | Razão Clojure/nativo |
| --- | ---: | ---: | ---: | ---: |
| Fundamentos e divisão e conquista | 5 | 1,82 s | 1,97 s | 1,082× |
| Ordenação e estatísticas de ordem | 5 | 3,25 s | 3,08 s | 0,948× |
| Estruturas de dados | 5 | 3,46 s | 2,60 s | 0,751× |
| Programação dinâmica e gulosa | 5 | 8,23 s | 2,98 s | 0,362× |
| Algoritmos de grafos | 5 | 3,87 s | 2,48 s | 0,641× |
| Teoria dos números e casamento de strings | 5 | 6,60 s | 3,84 s | 0,582× |

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

O total de parede é 1,61× o Clojure/JVM, enquanto o nativo usa 15,6% menos CPU acumulada
e substancialmente menos memória. A maior concentração restante está em
programação dinâmica e estruturas de dados que ainda alocam ou atravessam fronteiras
de função. Essa relação é uma observação dos dados, não uma prova isolada de causalidade.

Os valores são uma fotografia desta máquina; frequência dinâmica, carga do sistema,
toolchain, JIT e sistema operacional afetam o resultado. Para conclusões estatísticas,
repita as medições no mesmo ambiente e compare distribuições, não apenas uma execução.

O compilador release foi reconstruído imediatamente antes da rodada no commit
`424ba20`.

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
