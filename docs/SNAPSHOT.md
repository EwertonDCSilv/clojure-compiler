# Snapshot documentado

[Índice da documentação](README.md) ·
[Visão geral](overview.md) ·
[Resultados dos benchmarks](../benchmarks/README.md)

Esta documentação descreve o repositório no **HEAD documentado
[`1dc69b5`](https://github.com/EwertonDCSilv/clojure-compiler/commit/1dc69b5b126c193c30e9f24fdddd549abb7ce4cb)**
de 2026-07-28.

O marcador é o commit de código e resultados que serviu de entrada para a revisão
documental. O commit que grava esta própria revisão será necessariamente posterior;
isso evita tentar manter uma referência Git autorreferente, cujo hash mudaria ao
escrevê-lo no próprio commit. Toda atualização de estado deve avançar este marcador
para o HEAD que foi efetivamente auditado.

## Mudanças incluídas

| Commit | Estado documentado |
| --- | --- |
| [`7607bef`](https://github.com/EwertonDCSilv/clojure-compiler/commit/7607bef9f951b25711307f5f7c936053bc34baf8)–[`2d9f0db`](https://github.com/EwertonDCSilv/clojure-compiler/commit/2d9f0db) | implementação incremental de I/O nativo: streams dinâmicos, arquivos, filesystem, reader de runtime, argv, `Char`, `Bytes` e `Path` |
| [`8521e2c`](https://github.com/EwertonDCSilv/clojure-compiler/commit/8521e2c)–[`f923f28`](https://github.com/EwertonDCSilv/clojure-compiler/commit/f923f28) | corpus Exercism fixado, suíte de oito benchmarks e separação entre inventários de desempenho e conformidade |
| [`12dc120`](https://github.com/EwertonDCSilv/clojure-compiler/commit/12dc120)–[`5ab131a`](https://github.com/EwertonDCSilv/clojure-compiler/commit/5ab131a) | `Float` nativo boxeado, aritmética numérica mista e promoção dos casos de conformidade |
| [`4c10f69`](https://github.com/EwertonDCSilv/clojure-compiler/commit/4c10f69) | promoção de 13 casos estáveis de I/O de `xfail` para `active` |
| [`1dc69b5`](https://github.com/EwertonDCSilv/clojure-compiler/commit/1dc69b5b126c193c30e9f24fdddd549abb7ce4cb) | gates de documentação e fluxo TDD em todo o repositório |

O runtime C já está fisicamente separado por subsistema em
[`crates/clojure-codegen/runtime/`](../crates/clojure-codegen/runtime/), mas continua
compilado como uma única unidade de tradução. A modularização dos grandes arquivos Rust
e a expansão sistemática de seus testes são propostas separadas nas ADRs
[0012](../specs/adr/0012-rust-crate-modularization.md) e
[0011](../specs/adr/0011-rust-crate-unit-testing-strategy.md); elas não são apresentadas
como concluídas neste snapshot.

## Benchmark de referência

As medições Native e Clojure/JVM foram refeitas no commit
[`1dc69b5`](https://github.com/EwertonDCSilv/clojure-compiler/commit/1dc69b5b126c193c30e9f24fdddd549abb7ce4cb).
Cracking e Cormen usam escala 25×; Exercism usa escala 5×. O compilador nativo usa
Cranelift `--opt-level none`, enquanto o outro caminho usa Clojure 1.12.5 sobre Java
21. O snapshot registra uma execução completa de cada suíte: serve para triagem de
regressões, não como afirmação estatística controlada.

| Suíte | Parede nativo/JVM | CPU nativo/JVM | RSS mediano nativo/JVM |
| --- | ---: | ---: | ---: |
| Cracking | 7,71 / 22,27 s | 7,58 / 48,66 s | 4,6 / 120,8 MiB |
| Cormen/CLRS | 26,08 / 16,39 s | 25,97 / 31,35 s | 13,2 / 273,0 MiB |
| Exercism | 6,68 / 4,22 s | 6,66 / 8,00 s | 7,8 / 249,6 MiB |

Todos os 98 casos terminaram com status `OK` e checksums Native/JVM equivalentes. Os
detalhes por caso estão nos relatórios
[Cracking](../benchmarks/cracking/results/README.md),
[Cormen](../benchmarks/cormen/results/README.md) e
[Exercism](../benchmarks/exercism/results/README.md).

## Como conferir o marcador

```bash
git show --stat 1dc69b5
git diff 1dc69b5..HEAD
```

O primeiro comando mostra o snapshot auditado. O segundo evidencia alterações
posteriores que ainda precisam ser incorporadas numa futura revisão documental.
