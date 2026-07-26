# Resultado extremo de referência

Arquivo: [`extreme.csv`](extreme.csv)

Execução realizada em 2026-07-26 com:

```bash
benchmarks/cracking/run.sh --extreme \
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
| Build do compilador | Cargo `--release` |
| Multiplicador interno | 25× |
| GC | configuração normal |

## Resumo desta execução

- 60 casos executados, 60 com status `OK`.
- Tempo de parede acumulado dos executáveis: 21,82 s.
- Tempo de CPU acumulado: 21,57 s.
- Caso mais demorado: `05-bit-manipulation/02-bit-parity.clj`, 1,47 s.
- Maior pico de RSS: `07-object-oriented-design/06-record-updates.clj`, 232.196 KiB.

Os tempos de compilação aparecem por caso e não fazem parte do `wall_time_s` de
execução. Os valores são uma fotografia desta máquina; frequência dinâmica, carga do
sistema, toolchain e sistema operacional afetam o resultado. Compare regressões usando
o mesmo ambiente e várias execuções.
