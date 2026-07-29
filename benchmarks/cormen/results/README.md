# Comparação extrema de referência — Cormen/CLRS

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

- 30 casos comparados, todos com status `OK` e checksums idênticos.
- O nativo teve menor tempo de parede em 13 casos; Clojure/JVM em 17.
- Mediana de `wall_speedup_vs_clojure`: 0,773×.
- Tempos de parede acumulados: nativo 30,60 s; Clojure/JVM 16,74 s.
- Tempos de CPU acumulados: nativo 30,38 s; Clojure/JVM 31,82 s.
- O nativo teve menor tempo de CPU em 20 casos; Clojure/JVM em 10.
- Mediana de `cpu_speedup_vs_clojure`: 1,603×; o total de CPU ainda favorece o nativo.
- O nativo apresentou RSS menor nos 30 casos.
- Mediana de `rss_ratio_clojure_over_native`: 26,570×.
- Maior RSS nativo: 21,4 MiB em
  `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj`.
- Maior RSS Clojure/JVM: 908,6 MiB no mesmo caso.
- Compilação acumulada: 6.580 ms no nativo e 15.013 ms no Clojure/JVM AOT.

Em relação ao artefato publicado imediatamente anterior, a mediana reduziu o agregado
nativo de 34,67 para 30,60 s de parede (-11,7%) e de 34,33 para 30,38 s de CPU
(-11,5%). O snapshot de uma rodada em `424ba20` registrava 27,23 s; a diferença para
esse ponto antigo não prova regressão causal e continua subordinada ao controle
pareado definido pela ADR-0014.

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
| `01-foundations-and-divide-conquer/01-binary-exponentiation.clj` | 0.07 / 0.39 | -82.1% | 0.06 / 0.81 | -92.6% | 1.4 / 145.4 | -99.0% |
| `01-foundations-and-divide-conquer/02-horner-polynomial.clj` | 0.06 / 0.38 | -84.2% | 0.06 / 0.79 | -92.4% | 1.4 / 162.4 | -99.1% |
| `01-foundations-and-divide-conquer/03-prefix-range-sums.clj` | 0.07 / 0.43 | -83.7% | 0.07 / 0.86 | -91.9% | 11.7 / 232.1 | -95.0% |
| `01-foundations-and-divide-conquer/04-iterative-binary-search.clj` | 0.17 / 0.40 | -57.5% | 0.17 / 0.82 | -79.4% | 1.4 / 117.7 | -98.8% |
| `01-foundations-and-divide-conquer/05-maximum-subarray-divide.clj` | 1.47 / 0.47 | +212.8% | 1.47 / 0.96 | +51.8% | 5.1 / 197.6 | -97.4% |
| `02-sorting-and-order-statistics/01-insertion-sort.clj` | 0.56 / 0.53 | +6.7% | 0.56 / 0.98 | -42.9% | 18.1 / 364.7 | -95.0% |
| `02-sorting-and-order-statistics/02-selection-sort.clj` | 0.34 / 0.46 | -25.0% | 0.34 / 0.93 | -63.4% | 18.2 / 232.4 | -92.2% |
| `02-sorting-and-order-statistics/03-counting-sort.clj` | 0.23 / 0.52 | -55.8% | 0.22 / 0.97 | -77.3% | 14.5 / 365.0 | -96.0% |
| `02-sorting-and-order-statistics/04-merge-sort.clj` | 1.03 / 0.59 | +75.4% | 1.02 / 1.27 | -19.7% | 16.1 / 395.3 | -95.9% |
| `02-sorting-and-order-statistics/05-quickselect.clj` | 1.36 / 0.91 | +50.3% | 1.35 / 1.94 | -30.0% | 4.6 / 524.2 | -99.1% |
| `03-data-structures/01-build-max-heap.clj` | 0.30 / 0.44 | -31.8% | 0.29 / 0.92 | -68.5% | 18.3 / 234.3 | -92.2% |
| `03-data-structures/02-disjoint-set-union.clj` | 0.83 / 0.50 | +66.0% | 0.81 / 0.98 | -16.8% | 13.3 / 232.2 | -94.3% |
| `03-data-structures/03-chained-hash-table.clj` | 0.55 / 0.59 | -7.6% | 0.54 / 1.15 | -53.5% | 12.9 / 367.7 | -96.5% |
| `03-data-structures/04-circular-queue.clj` | 0.23 / 0.56 | -58.6% | 0.22 / 1.02 | -78.4% | 13.1 / 364.7 | -96.4% |
| `03-data-structures/05-binary-search-tree.clj` | 2.05 / 0.46 | +345.7% | 2.04 / 0.94 | +117.6% | 19.4 / 258.8 | -92.5% |
| `04-dynamic-programming-and-greedy/01-rod-cutting.clj` | 1.93 / 0.53 | +263.2% | 1.92 / 0.98 | +94.9% | 7.4 / 257.0 | -97.1% |
| `04-dynamic-programming-and-greedy/02-matrix-chain-order.clj` | 1.32 / 0.57 | +129.6% | 1.31 / 1.09 | +20.2% | 12.8 / 367.4 | -96.5% |
| `04-dynamic-programming-and-greedy/03-longest-common-subsequence.clj` | 3.58 / 0.81 | +344.1% | 3.57 / 1.36 | +162.5% | 15.0 / 466.3 | -96.8% |
| `04-dynamic-programming-and-greedy/04-zero-one-knapsack.clj` | 2.05 / 0.69 | +195.0% | 2.05 / 1.19 | +72.3% | 5.5 / 472.4 | -98.8% |
| `04-dynamic-programming-and-greedy/05-activity-selection.clj` | 0.06 / 0.35 | -82.9% | 0.05 / 0.78 | -93.5% | 1.4 / 115.6 | -98.8% |
| `05-graph-algorithms/01-breadth-first-search.clj` | 1.17 / 0.50 | +134.0% | 1.17 / 0.99 | +17.7% | 19.4 / 288.1 | -93.3% |
| `05-graph-algorithms/02-depth-first-search.clj` | 0.88 / 0.48 | +80.4% | 0.87 / 1.00 | -13.0% | 19.5 / 281.6 | -93.1% |
| `05-graph-algorithms/03-topological-sort.clj` | 1.29 / 0.56 | +130.4% | 1.28 / 1.05 | +21.9% | 18.9 / 364.3 | -94.8% |
| `05-graph-algorithms/04-bellman-ford.clj` | 0.41 / 0.44 | -6.8% | 0.40 / 0.89 | -54.8% | 14.9 / 173.3 | -91.4% |
| `05-graph-algorithms/05-floyd-warshall.clj` | 0.70 / 0.50 | +40.0% | 0.69 / 1.00 | -31.3% | 14.8 / 237.7 | -93.8% |
| `06-number-theory-and-string-matching/01-extended-euclid.clj` | 2.37 / 0.51 | +364.7% | 2.36 / 0.99 | +138.4% | 19.4 / 368.5 | -94.7% |
| `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj` | 4.60 / 1.78 | +158.4% | 4.58 / 2.40 | +90.6% | 21.4 / 908.6 | -97.6% |
| `06-number-theory-and-string-matching/03-naive-string-matching.clj` | 0.48 / 0.40 | +20.0% | 0.47 / 0.82 | -43.0% | 1.4 / 115.5 | -98.8% |
| `06-number-theory-and-string-matching/04-rabin-karp.clj` | 0.26 / 0.61 | -57.4% | 0.26 / 1.11 | -76.6% | 1.5 / 364.9 | -99.6% |
| `06-number-theory-and-string-matching/05-knuth-morris-pratt.clj` | 0.19 / 0.39 | -51.3% | 0.18 / 0.82 | -77.6% | 11.7 / 143.9 | -91.8% |
## Resultado por capítulo

| Capítulo | Casos | Parede nativo | Parede Clojure | Razão Clojure/nativo |
| --- | ---: | ---: | ---: | ---: |
| Fundamentos e divisão e conquista | 5 | 1,84 s | 2,07 s | 1,125× |
| Ordenação e estatísticas de ordem | 5 | 3,53 s | 3,00 s | 0,850× |
| Estruturas de dados | 5 | 3,95 s | 2,54 s | 0,643× |
| Programação dinâmica e gulosa | 5 | 8,93 s | 2,96 s | 0,331× |
| Algoritmos de grafos | 5 | 4,45 s | 2,48 s | 0,559× |
| Teoria dos números e casamento de strings | 5 | 7,90 s | 3,69 s | 0,467× |

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

O total de parede é 1,83× o Clojure/JVM, enquanto o nativo usa 4,5% menos CPU acumulada
e substancialmente menos memória. A maior concentração restante está em
programação dinâmica e estruturas de dados que ainda alocam ou atravessam fronteiras
de função. Essa relação é uma observação dos dados, não uma prova isolada de causalidade.

Os valores são uma fotografia desta máquina; frequência dinâmica, carga do sistema,
toolchain, JIT e sistema operacional afetam o resultado. Para conclusões estatísticas,
repita as medições no mesmo ambiente e compare distribuições, não apenas uma execução.

O compilador release foi reconstruído antes das medições no commit `a1ecebd`.

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
