# Comparação extrema de referência

Arquivo: [`extreme.csv`](extreme.csv)

Execução realizada em 2026-07-26 com:

```bash
benchmarks/cracking/compare-clojure.sh --scale 25 \
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
- O nativo teve menor tempo de parede em 38 casos; Clojure/JVM em 21; houve 1 empate.
- Mediana de `wall_speedup_vs_clojure`: 1,450× a favor do nativo.
- Tempos de parede acumulados: nativo 21,78 s; Clojure/JVM 23,20 s.
- Tempos de CPU acumulados: nativo 21,57 s; Clojure/JVM 50,77 s.
- Mediana de `cpu_speedup_vs_clojure`: 3,277× a favor do nativo.
- O nativo apresentou RSS menor em 59 dos 60 casos.
- Mediana de `rss_ratio_clojure_over_native`: 19,599×.
- Maior RSS nativo: 232.324 KiB em
  `07-object-oriented-design/06-record-updates.clj`.
- Maior RSS Clojure/JVM: 529.336 KiB em
  `06-math-and-logic/05-integer-square-root.clj`.

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
