# Snapshot documentado

[Índice da documentação](README.md) ·
[Visão geral](overview.md) ·
[Resultados dos benchmarks](../benchmarks/README.md)

Esta documentação descreve o repositório no **HEAD documentado
[`3e71bc1`](https://github.com/EwertonDCSilv/clojure-compiler/commit/3e71bc1996b689233c80516b4b4aff52259c2cdf)**
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
| [`2df79df`](https://github.com/EwertonDCSilv/clojure-compiler/commit/2df79df)–[`fdd319c`](https://github.com/EwertonDCSilv/clojure-compiler/commit/fdd319c) | globais permanentes de topo, loader estático multi-arquivo e isolamento de símbolos por namespace |
| [`29c0822`](https://github.com/EwertonDCSilv/clojure-compiler/commit/29c0822)–[`ab1af4d`](https://github.com/EwertonDCSilv/clojure-compiler/commit/ab1af4d) | primeiro corte executável do connector Pedestal em memória: request/response, cadeia de interceptors, roteamento e lifecycle síncrono |
| [`02a7141`](https://github.com/EwertonDCSilv/clojure-compiler/commit/02a7141)–[`3e71bc1`](https://github.com/EwertonDCSilv/clojure-compiler/commit/3e71bc1996b689233c80516b4b4aff52259c2cdf) | parser HTTP/1.x estrito e serializador de respostas HTTP/1.1 |

O runtime C já está fisicamente separado por subsistema em
[`crates/clojure-codegen/runtime/`](../crates/clojure-codegen/runtime/), mas continua
compilado como uma única unidade de tradução. A modularização dos grandes arquivos Rust
e a expansão sistemática de seus testes são propostas separadas nas ADRs
[0012](../specs/adr/0012-rust-crate-modularization.md) e
[0011](../specs/adr/0011-rust-crate-unit-testing-strategy.md); elas não são apresentadas
como concluídas neste snapshot.

## Benchmark de referência

As medições Native e Clojure/JVM foram refeitas no commit
[`3e71bc1`](https://github.com/EwertonDCSilv/clojure-compiler/commit/3e71bc1996b689233c80516b4b4aff52259c2cdf).
Cracking e Cormen usam escala 25×; Exercism usa escala 5×. O compilador nativo usa
Cranelift `--opt-level none`, enquanto o outro caminho usa Clojure 1.12.5 sobre Java
21. O snapshot registra uma execução completa de cada suíte: serve para triagem de
regressões, não como afirmação estatística controlada.

| Suíte | Parede nativo/JVM | CPU nativo/JVM | RSS mediano nativo/JVM |
| --- | ---: | ---: | ---: |
| Cracking | 8,16 / 23,22 s | 8,06 / 50,93 s | 4,6 / 119,9 MiB |
| Cormen/CLRS | 30,06 / 17,01 s | 29,95 / 32,66 s | 13,3 / 273,9 MiB |
| Exercism | 7,08 / 4,22 s | 7,05 / 8,04 s | 7,8 / 242,2 MiB |

Todos os 98 casos terminaram com status `OK` e checksums Native/JVM equivalentes. Os
detalhes por caso estão nos relatórios
[Cracking](../benchmarks/cracking/results/README.md),
[Cormen](../benchmarks/cormen/results/README.md) e
[Exercism](../benchmarks/exercism/results/README.md).
O total de parede nativo do Cormen ficou 15,3% acima do snapshot imediatamente
anterior. Uma rodada única não separa ruído ambiental de regressão real; por isso o
dado fica registrado como sinal para o gate pareado definido na ADR-0014.

## Como conferir o marcador

```bash
git show --stat 3e71bc1
git diff 3e71bc1..HEAD
```

O primeiro comando mostra o snapshot auditado. O segundo evidencia alterações
posteriores que ainda precisam ser incorporadas numa futura revisão documental.
