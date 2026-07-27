# Comparação extrema de referência — Cormen/CLRS

Arquivo: [`extreme.csv`](extreme.csv)

Execução realizada em 2026-07-27 com:

```bash
benchmarks/cormen/compare-clojure.sh --scale 25 --opt-level none \
  --compiler target/release/clojure-native \
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
- O nativo teve menor tempo de parede em 4 casos; Clojure/JVM em 26.
- Mediana de `wall_speedup_vs_clojure`: 0,254×.
- Tempos de parede acumulados: nativo 102,60 s; Clojure/JVM 16,91 s.
- Tempos de CPU acumulados: nativo 102,41 s; Clojure/JVM 32,61 s.
- Mediana de `cpu_speedup_vs_clojure`: 0,502×.
- O nativo apresentou RSS menor nos 30 casos.
- Mediana de `rss_ratio_clojure_over_native`: 19,918×.
- Maior RSS nativo: 20.740 KiB em
  `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj`.
- Maior RSS Clojure/JVM: 1.024.412 KiB no mesmo caso.
- Compilação acumulada: 3.677 ms no nativo e 15.272 ms no Clojure/JVM AOT.

Em relação ao CSV de referência versionado anterior, os tempos acumulados subiram
18,76% no nativo e 1,32% no Clojure/JVM. A rodada aponta regressão do nativo nesta
máquina, mas não isola causalidade sem uma série de repetições.

## Resultado por capítulo

| Capítulo | Casos | Parede nativo | Parede Clojure | Razão Clojure/nativo |
| --- | ---: | ---: | ---: | ---: |
| Fundamentos e divisão e conquista | 5 | 5,44 s | 2,07 s | 0,381× |
| Ordenação e estatísticas de ordem | 5 | 11,94 s | 3,02 s | 0,253× |
| Estruturas de dados | 5 | 14,85 s | 2,60 s | 0,175× |
| Programação dinâmica e gulosa | 5 | 33,69 s | 2,96 s | 0,088× |
| Algoritmos de grafos | 5 | 17,59 s | 2,50 s | 0,142× |
| Teoria dos números e casamento de strings | 5 | 19,09 s | 3,76 s | 0,197× |

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

Mesmo depois dos fast paths inteiros e dos stores diretos na shadow stack, os casos
algorítmicos continuam mais lentos que Clojure/JVM, apesar do consumo de memória
substancialmente menor. Essa relação é uma observação dos dados, não uma prova isolada
de causalidade; ela orienta as próximas etapas de liveness, IR e safepoints da ADR 0006.

Os valores são uma fotografia desta máquina; frequência dinâmica, carga do sistema,
toolchain, JIT e sistema operacional afetam o resultado. Para conclusões estatísticas,
repita as medições no mesmo ambiente e compare distribuições, não apenas uma execução.

O compilador release foi reconstruído imediatamente antes da rodada. O `runtime.c`
medido tinha SHA-256
`b9cd80d252c5763722b7d860bebc0e688ec9413bf60d714b08c0054b7bdaa958`.

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
