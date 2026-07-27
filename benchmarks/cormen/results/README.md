# Comparação extrema de referência — Cormen/CLRS

[Catálogo dos benchmarks](../../README.md) ·
[Guia da suíte](../README.md)

Arquivo: [`extreme.csv`](extreme.csv)

Medições nativas atualizadas em 2026-07-27 com:

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
- O nativo teve menor tempo de parede em 7 casos; Clojure/JVM em 21; houve 2 empates
  na resolução de 0,01 s do runner.
- Mediana de `wall_speedup_vs_clojure`: 0,686×.
- Tempos de parede acumulados: nativo 39,13 s; Clojure/JVM preservado 16,91 s.
- Tempos de CPU acumulados: nativo 38,97 s; Clojure/JVM preservado 32,61 s.
- Mediana de `cpu_speedup_vs_clojure`: 1,395× a favor do nativo, embora os casos mais
  lentos mantenham o total de CPU nativo maior.
- O nativo apresentou RSS menor nos 30 casos.
- Mediana de `rss_ratio_clojure_over_native`: 20,618×.
- Maior RSS nativo: 19.844 KiB em
  `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj`.
- Maior RSS Clojure/JVM: 1.024.412 KiB no mesmo caso.
- Compilação acumulada: 3.975 ms no nativo e 15.272 ms preservados no Clojure/JVM AOT.

Em relação à medição nativa imediatamente anterior, os tempos acumulados de parede e
CPU caíram respectivamente 50,22% e 50,29%. A rodada isolada aponta melhora expressiva,
mas não permite atribuir o delta a código sem uma série de repetições controladas.

## Resumo por teste

`N/J` mostra os valores absolutos nativo/Clojure. O delta é
`(nativo - Clojure) / Clojure`: negativo favorece o nativo; positivo favorece a JVM.

| Caso | Tempo N/J (s) | Δ tempo | CPU N/J (s) | Δ CPU | RSS N/J (MiB) | Δ RSS |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `01-foundations-and-divide-conquer/01-binary-exponentiation.clj` | 0.06 / 0.38 | -84.2% | 0.06 / 0.82 | -92.7% | 1.5 / 148.5 | -99.0% |
| `01-foundations-and-divide-conquer/02-horner-polynomial.clj` | 0.55 / 0.38 | +44.7% | 0.54 / 0.82 | -34.1% | 18.2 / 164.4 | -88.9% |
| `01-foundations-and-divide-conquer/03-prefix-range-sums.clj` | 0.50 / 0.43 | +16.3% | 0.49 / 0.90 | -45.6% | 17.9 / 231.8 | -92.3% |
| `01-foundations-and-divide-conquer/04-iterative-binary-search.clj` | 0.14 / 0.40 | -65.0% | 0.14 / 0.85 | -83.5% | 1.5 / 117.8 | -98.8% |
| `01-foundations-and-divide-conquer/05-maximum-subarray-divide.clj` | 1.40 / 0.48 | +191.7% | 1.39 / 0.99 | +40.4% | 5.2 / 197.5 | -97.4% |
| `02-sorting-and-order-statistics/01-insertion-sort.clj` | 0.73 / 0.52 | +40.4% | 0.72 / 0.99 | -27.3% | 17.6 / 362.4 | -95.1% |
| `02-sorting-and-order-statistics/02-selection-sort.clj` | 0.45 / 0.45 | +0.0% | 0.44 / 0.92 | -52.2% | 17.8 / 230.6 | -92.3% |
| `02-sorting-and-order-statistics/03-counting-sort.clj` | 0.93 / 0.52 | +78.8% | 0.92 / 1.00 | -8.0% | 17.8 / 363.8 | -95.1% |
| `02-sorting-and-order-statistics/04-merge-sort.clj` | 1.60 / 0.61 | +162.3% | 1.60 / 1.28 | +25.0% | 18.6 / 396.1 | -95.3% |
| `02-sorting-and-order-statistics/05-quickselect.clj` | 1.35 / 0.92 | +46.7% | 1.35 / 1.91 | -29.3% | 4.8 / 521.5 | -99.1% |
| `03-data-structures/01-build-max-heap.clj` | 0.42 / 0.45 | -6.7% | 0.41 / 0.95 | -56.8% | 17.8 / 233.7 | -92.4% |
| `03-data-structures/02-disjoint-set-union.clj` | 1.19 / 0.51 | +133.3% | 1.19 / 1.04 | +14.4% | 15.2 / 241.9 | -93.7% |
| `03-data-structures/03-chained-hash-table.clj` | 0.69 / 0.60 | +15.0% | 0.68 / 1.19 | -42.9% | 13.6 / 366.6 | -96.3% |
| `03-data-structures/04-circular-queue.clj` | 0.58 / 0.58 | +0.0% | 0.58 / 1.09 | -46.8% | 17.6 / 364.5 | -95.2% |
| `03-data-structures/05-binary-search-tree.clj` | 2.17 / 0.46 | +371.7% | 2.17 / 0.96 | +126.0% | 18.5 / 258.0 | -92.8% |
| `04-dynamic-programming-and-greedy/01-rod-cutting.clj` | 1.94 / 0.53 | +266.0% | 1.94 / 1.00 | +94.0% | 8.0 / 255.2 | -96.9% |
| `04-dynamic-programming-and-greedy/02-matrix-chain-order.clj` | 1.52 / 0.57 | +166.7% | 1.52 / 1.08 | +40.7% | 13.6 / 366.2 | -96.3% |
| `04-dynamic-programming-and-greedy/03-longest-common-subsequence.clj` | 4.32 / 0.81 | +433.3% | 4.31 / 1.37 | +214.6% | 14.8 / 477.4 | -96.9% |
| `04-dynamic-programming-and-greedy/04-zero-one-knapsack.clj` | 3.86 / 0.71 | +443.7% | 3.85 / 1.21 | +218.2% | 12.8 / 509.1 | -97.5% |
| `04-dynamic-programming-and-greedy/05-activity-selection.clj` | 0.04 / 0.34 | -88.2% | 0.04 / 0.76 | -94.7% | 1.5 / 117.1 | -98.7% |
| `05-graph-algorithms/01-breadth-first-search.clj` | 1.29 / 0.50 | +158.0% | 1.28 / 1.03 | +24.3% | 18.5 / 288.7 | -93.6% |
| `05-graph-algorithms/02-depth-first-search.clj` | 1.39 / 0.49 | +183.7% | 1.39 / 1.05 | +32.4% | 18.4 / 284.3 | -93.5% |
| `05-graph-algorithms/03-topological-sort.clj` | 1.51 / 0.56 | +169.6% | 1.50 / 1.11 | +35.1% | 18.3 / 372.4 | -95.1% |
| `05-graph-algorithms/04-bellman-ford.clj` | 0.49 / 0.44 | +11.4% | 0.49 / 0.90 | -45.6% | 15.9 / 171.6 | -90.7% |
| `05-graph-algorithms/05-floyd-warshall.clj` | 0.64 / 0.51 | +25.5% | 0.63 / 1.06 | -40.6% | 14.4 / 234.9 | -93.9% |
| `06-number-theory-and-string-matching/01-extended-euclid.clj` | 2.15 / 0.52 | +313.5% | 2.14 / 1.05 | +103.8% | 18.6 / 372.3 | -95.0% |
| `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj` | 6.27 / 1.83 | +242.6% | 6.26 / 2.46 | +154.5% | 19.4 / 1000.4 | -98.1% |
| `06-number-theory-and-string-matching/03-naive-string-matching.clj` | 0.38 / 0.39 | -2.6% | 0.38 / 0.83 | -54.2% | 1.5 / 116.8 | -98.7% |
| `06-number-theory-and-string-matching/04-rabin-karp.clj` | 0.19 / 0.62 | -69.4% | 0.19 / 1.14 | -83.3% | 1.5 / 370.7 | -99.6% |
| `06-number-theory-and-string-matching/05-knuth-morris-pratt.clj` | 0.38 / 0.40 | -5.0% | 0.37 / 0.85 | -56.5% | 17.9 / 143.3 | -87.5% |

## Resultado por capítulo

| Capítulo | Casos | Parede nativo | Parede Clojure | Razão Clojure/nativo |
| --- | ---: | ---: | ---: | ---: |
| Fundamentos e divisão e conquista | 5 | 2,65 s | 2,07 s | 0,781× |
| Ordenação e estatísticas de ordem | 5 | 5,06 s | 3,02 s | 0,597× |
| Estruturas de dados | 5 | 5,05 s | 2,60 s | 0,515× |
| Programação dinâmica e gulosa | 5 | 11,68 s | 2,96 s | 0,253× |
| Algoritmos de grafos | 5 | 5,32 s | 2,50 s | 0,470× |
| Teoria dos números e casamento de strings | 5 | 9,37 s | 3,76 s | 0,401× |

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
`a0fb120d758259ffe4148169e6dc662e4037189806ad0ebb1a7355001adcadf2`.

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
