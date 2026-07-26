# Comparação extrema de referência — Cormen/CLRS

Arquivo: [`extreme.csv`](extreme.csv)

Execução realizada em 2026-07-26 com:

```bash
benchmarks/cormen/compare-clojure.sh --scale 25 \
  --csv benchmarks/cormen/results/extreme.csv
```

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
- O nativo teve menor tempo de parede em 2 casos; Clojure/JVM em 28.
- Mediana de `wall_speedup_vs_clojure`: 0,266×.
- Tempos de parede acumulados: nativo 86,39 s; Clojure/JVM 16,69 s.
- Tempos de CPU acumulados: nativo 86,25 s; Clojure/JVM 31,85 s.
- Mediana de `cpu_speedup_vs_clojure`: 0,527×.
- O nativo apresentou RSS menor nos 30 casos.
- Mediana de `rss_ratio_clojure_over_native`: 19,766×.
- Maior RSS nativo: 20.692 KiB em
  `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj`.
- Maior RSS Clojure/JVM: 968.184 KiB no mesmo caso.
- Compilação acumulada: 2.604 ms no nativo e 15.128 ms no Clojure/JVM AOT.

## Resultado por capítulo

| Capítulo | Casos | Parede nativo | Parede Clojure | Razão Clojure/nativo |
| --- | ---: | ---: | ---: | ---: |
| Fundamentos e divisão e conquista | 5 | 5,82 s | 2,07 s | 0,356× |
| Ordenação e estatísticas de ordem | 5 | 11,47 s | 2,89 s | 0,252× |
| Estruturas de dados | 5 | 10,93 s | 2,57 s | 0,235× |
| Programação dinâmica e gulosa | 5 | 22,72 s | 2,98 s | 0,131× |
| Algoritmos de grafos | 5 | 13,54 s | 2,45 s | 0,181× |
| Teoria dos números e casamento de strings | 5 | 21,91 s | 3,73 s | 0,170× |

## Como ler a comparação

- `wall_speedup_vs_clojure` é tempo Clojure dividido pelo tempo nativo.
- `cpu_speedup_vs_clojure` é CPU Clojure dividida pela CPU nativa.
- `rss_ratio_clojure_over_native` é RSS Clojure dividido pelo RSS nativo.
- Nas três colunas, valores maiores que `1` favorecem o nativo; valores menores que `1`
  favorecem Clojure/JVM.

Os dois caminhos são compilados antes da execução medida: o binário pelo
`clojure-compiler` e o namespace JVM por AOT. Os custos aparecem separadamente em
`native_compile_wall_ms` e `clojure_compile_wall_ms`. A medição de execução inclui a
inicialização de cada processo, inclusive a JVM.

Esta execução evidencia o custo das chamadas de runtime e da manutenção da shadow
stack no backend atual: os casos algorítmicos intensivos em operações inteiras ficam
mais lentos que o Clojure/JVM, apesar do consumo de memória substancialmente menor.
Essa relação é uma observação dos dados, não uma prova isolada de causalidade; ela
serve como linha de base para medir as otimizações propostas na ADR 0006.

Os valores são uma fotografia desta máquina; frequência dinâmica, carga do sistema,
toolchain, JIT e sistema operacional afetam o resultado. Para conclusões estatísticas,
repita as medições no mesmo ambiente e compare distribuições, não apenas uma execução.

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
