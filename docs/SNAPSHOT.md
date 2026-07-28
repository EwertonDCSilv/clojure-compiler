# Snapshot documentado

[Índice da documentação](README.md) ·
[Visão geral](overview.md) ·
[Resultados dos benchmarks](../benchmarks/README.md)

Esta documentação descreve o repositório no **HEAD documentado
[`476aefd`](https://github.com/EwertonDCSilv/clojure-compiler/commit/476aefd47bd01c4dca8b11f3e8009fbf2cd78d3c)**
de 2026-07-27.

O marcador é o commit de código e resultados que serviu de entrada para a revisão
documental. O commit que grava esta própria revisão será necessariamente posterior;
isso evita tentar manter uma referência Git autorreferente, cujo hash mudaria ao
escrevê-lo no próprio commit. Toda atualização de estado deve avançar este marcador
para o HEAD que foi efetivamente auditado.

## Mudanças incluídas

| Commit | Estado documentado |
| --- | --- |
| [`e87456e`](https://github.com/EwertonDCSilv/clojure-compiler/commit/e87456e9be1e5bd759c96f47a323b99fb4f8d88f) | primeiro subconjunto conservador de linearidade interprocedural para acumuladores de vetor |
| [`1ca1d79`](https://github.com/EwertonDCSilv/clojure-compiler/commit/1ca1d799a02ead388a1ffcae33b760fe0743d8d9) | hoisting e cache por site de vetores literais constantes formados por imediatos |
| [`476aefd`](https://github.com/EwertonDCSilv/clojure-compiler/commit/476aefd47bd01c4dca8b11f3e8009fbf2cd78d3c) | CSVs, relatórios, gráficos, página e ADR-0009 atualizados |

O runtime C já está fisicamente separado por subsistema em
[`crates/clojure-codegen/runtime/`](../crates/clojure-codegen/runtime/), mas continua
compilado como uma única unidade de tradução. A modularização dos grandes arquivos Rust
e a expansão sistemática de seus testes são propostas separadas nas ADRs
[0012](../specs/adr/0012-rust-crate-modularization.md) e
[0011](../specs/adr/0011-rust-crate-unit-testing-strategy.md); elas não são apresentadas
como concluídas neste snapshot.

## Benchmark de referência

As medições nativas usam o compilador do commit
[`1ca1d79`](https://github.com/EwertonDCSilv/clojure-compiler/commit/1ca1d799a02ead388a1ffcae33b760fe0743d8d9),
escala 25× e Cranelift `--opt-level none`. A execução mediana integral de três rodadas
foi publicada; as colunas Clojure/JVM anteriores foram preservadas.

| Suíte | Parede nativo/JVM | CPU nativo/JVM | RSS mediano nativo/JVM |
| --- | ---: | ---: | ---: |
| Cracking | 7,77 / 24,96 s | 7,61 / 53,88 s | 4,7 / 117,5 MiB |
| Cormen/CLRS | 29,45 / 16,91 s | 29,30 / 32,61 s | 13,4 / 271,1 MiB |

Todos os 90 casos terminaram com status `OK` e checksums nativo/JVM equivalentes. Os
detalhes por caso estão nos relatórios
[Cracking](../benchmarks/cracking/results/README.md) e
[Cormen](../benchmarks/cormen/results/README.md).

## Como conferir o marcador

```bash
git show --stat 476aefd
git diff 476aefd..HEAD
```

O primeiro comando mostra o snapshot auditado. O segundo evidencia alterações
posteriores que ainda precisam ser incorporadas numa futura revisão documental.
