# Comparação extrema de referência

Arquivo: [`extreme.csv`](extreme.csv)

Execução realizada em 2026-07-27 com:

```bash
benchmarks/cracking/compare-clojure.sh --scale 25 --opt-level none \
  --compiler target/release/clojure-native \
  --csv benchmarks/cracking/results/extreme.csv
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

- 60 casos comparados, 60 com status `OK` e checksums idênticos.
- O nativo teve menor tempo de parede em 46 casos; Clojure/JVM em 12; houve 2 empates.
- Mediana de `wall_speedup_vs_clojure`: 2,081× a favor do nativo.
- Tempos de parede acumulados: nativo 17,87 s; Clojure/JVM 24,96 s.
- Tempos de CPU acumulados: nativo 17,68 s; Clojure/JVM 53,88 s.
- Mediana de `cpu_speedup_vs_clojure`: 4,542× a favor do nativo.
- O nativo apresentou RSS menor em 59 dos 60 casos.
- Mediana de `rss_ratio_clojure_over_native`: 19,906×.
- Maior RSS nativo: 232.196 KiB em
  `07-object-oriented-design/06-record-updates.clj`.
- Maior RSS Clojure/JVM: 525.512 KiB em
  `06-math-and-logic/05-integer-square-root.clj`.
- Compilação acumulada: 7.246 ms no nativo e 33.114 ms no Clojure/JVM AOT.

Em relação ao CSV de referência versionado anterior, o tempo acumulado do nativo caiu
17,95%, enquanto o do Clojure/JVM subiu 7,59%. A rodada indica melhora do nativo nesta
máquina, mas não isola causalidade sem uma série de repetições.

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

Os valores são uma fotografia desta máquina; frequência dinâmica, carga do sistema,
toolchain, JIT e sistema operacional afetam o resultado. Para conclusões estatísticas,
repita as medições no mesmo ambiente e compare distribuições, não apenas uma execução.

O compilador release foi reconstruído imediatamente antes da rodada. O `runtime.c`
medido tinha SHA-256
`b9cd80d252c5763722b7d860bebc0e688ec9413bf60d714b08c0054b7bdaa958`.

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
