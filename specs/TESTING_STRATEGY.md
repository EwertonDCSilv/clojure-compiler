# Estratégia de testes

[Índice das especificações](README.md) ·
[Guia de uso](../docs/usage.md) ·
[Contrato de conformidade](conformance/README.md)

A estratégia combina testes Rust, execução nativa end-to-end, conformidade versionada,
GC stress, cobertura e benchmarks. Clojure/JVM é apenas um oracle manual: não é
dependência da CI, do runner `verify` nem do binário produzido.

## Camadas

| Camada | Entrada recomendada | Implementação | Contrato |
| --- | --- | --- | --- |
| Formatação | `make fmt-check` | `cargo fmt --all --check` | nenhuma divergência |
| Lints | `make lint` | Clippy + `clj-kondo` | zero erros e warnings |
| Unitários/integração | `make test` | `cargo test --workspace` | todos verdes |
| Runtime C | `make test-runtime` | harnesses C dedicados | invariantes, ABI, erros e GC stress |
| Cobertura | `make coverage` | `cargo-llvm-cov` | gates globais e por arquivo |
| Conformidade | `make compatibility` | runner A–E offline | ativos/xfail/checksums corretos |
| Cobertura de sintaxe | `make reader-syntax-coverage` | catálogo congelado do Reader 1.12.5 | denominador, fixtures e issues rastreáveis |
| GC | casos com `CLJN_GC_STRESS=1` | runtime e conformidade | coleta a cada alocação sem corrupção |
| Benchmarks | `make benchmarks` | runners Cracking, Cormen e Exercism | checksum e métricas comparáveis |
| Corpus externo | `make exercism-compatibility` | 101 práticas, 13 conceitos e 493 arquivos pinados | transições PASS/FAIL rastreáveis |
| Oracle JVM | `make compatibility-oracle` | Clojure 1.12.5 local | operação exclusivamente manual |

Benchmarks respondem quanto custa executar a mesma carga em Native e JVM, medindo
tempo, CPU e memória após validar o checksum. Conformidade responde quais formas da
linguagem e APIs são aceitas e semanticamente corretas, usando estados
`active`/`xfail`/`pending`. Um caso de conformidade só vira benchmark quando também
oferece uma carga de desempenho determinística e útil.

Os agregadores são:

```bash
make quality  # formato, lints e testes
make all      # quality + cobertura + conformidade + 98 benchmarks
make ci       # comandos usados pelos jobs do GitHub Actions
```

Property testing, fuzzing, Miri, sanitizers e uma matriz multiplataforma mais ampla
continuam recomendados para as fases que introduzirem novas superfícies de `unsafe`,
concorrência e FFI.

O runtime C possui harnesses próprios em `src/compiler/clojure-codegen/tests/c/`. O harness
unitário inclui `runtime.c` na mesma unidade de tradução para observar invariantes
internos sem ampliar a ABI de produção. O harness de integração é compilado e linkado
separadamente, consumindo somente os símbolos públicos; os contratos fatais são
executados em subprocessos para validar status e `stderr`. As suítes rodam normalmente e
com `CLJN_GC_STRESS=1`. Para executar também ASan e UBSan:

```bash
make test-runtime-sanitize
```

## Suíte de conformidade

As fixtures executáveis vivem em [`tests/conformance/`](../tests/conformance):

```text
tests/conformance/
├── level-a-syntax/
│   ├── literals/
│   ├── collections/
│   ├── reader-macros/
│   ├── metadata/
│   ├── trivia/
│   └── diagnostics/
├── level-b-semantics/
│   ├── arithmetic/
│   ├── control-flow/
│   ├── functions/
│   ├── closures/
│   ├── macros/
│   ├── collections/
│   ├── records-protocols/
│   ├── exceptions/
│   ├── multimethods/
│   ├── collections/transients/
│   ├── errors/
│   ├── gc/
│   └── io/
├── level-c-stdlib/
│   ├── clojure-core/
│   ├── cljn-io/
│   ├── cljn-process/
│   ├── clojure-string/
│   ├── clojure-set/
│   ├── clojure-walk/
│   ├── clojure-edn/
│   └── clojure-test/
├── level-d-pure-libraries/
└── level-e-ecosystem/
```

O inventário atual contém 461 casos:

- 225 `active`: executados e bloqueantes;
- 204 `xfail`: precisam falhar pela razão declarada; um passe inesperado também bloqueia;
- 32 `pending`: schema e checksum são validados, mas o caso não é executado.

Níveis A–C classificam a sintaxe, a semântica e a biblioteca realmente executáveis.
O nível D executa bibliotecas puras autocontidas, incluindo recuperação por exceção, e
registra lacunas de macros, namespaces, lazy seqs e metadata. O nível E executa aplicações autônomas de
arquivo único e transforma dependências, JARs, Java, carregamento dinâmico e concorrência
em `xfail` concretos. Projetos que exigem loader/packaging continuam `pending`.
Entre eles está uma API HTTP Hello World em Pedestal, com `deps.edn`, fonte, requisição
e resposta esperada versionados como contrato do alvo.

Cada caso é autocontido e tem `case.toml`, `input.clj` e a expectativa aplicável. O
manifesto registra `status`, `class`, `target`, `oracle`, timeout, modo GC stress, razão
e tracking. Casos `build-run` podem ainda declarar `[run]` com argv, ambiente, stdin,
exit code, plataformas e symlinks. Cada execução usa um diretório temporário:
`work.before/` define o estado inicial e `work.after/` o snapshot exato esperado.
Streams `.bin` são comparados byte a byte; mapas/sets são comparados estruturalmente;
newlines e caminhos temporários de expectativas textuais são normalizados.

A quantidade de sintaxe do Reader ainda ausente não é calculada dividindo casos
`active` pelo inventário A existente. O denominador independente está em
[`conformance/clojure-1.12.5-reader.toml`](conformance/clojure-1.12.5-reader.toml), e
as fórmulas estão em
[`conformance/READER_SYNTAX_COVERAGE.md`](conformance/READER_SYNTAX_COVERAGE.md).
`make compatibility` valida esse catálogo antes de executar as fixtures.

A matriz de I/O descrita em [IO_SPEC](IO_SPEC.md) acrescenta cenários normal, limite e
erro para core/EDN e `cljn.*`. Há casos ativos para output, flush, redirecionamento,
texto em arquivo, `read-string` e streams de string; as lacunas restantes continuam
`xfail` e não afirmam implementação.

`verify` compila o CLI release uma vez, reutiliza o artefato, executa no máximo quatro
casos em paralelo e grava:

- `target/conformance/report.json`;
- `target/conformance/report-summary.txt`.

Os checksums em `tests/conformance/checksums.sha256` tornam alterações acidentais nas
fixtures bloqueantes, incluindo os arquivos internos de projetos. O contrato completo
está em
[`conformance/README.md`](conformance/README.md).

## Oracle Clojure/JVM

O oracle é fixado em Clojure 1.12.5, não faz downloads e só roda quando o mantenedor
fornece `CLOJURE_CLASSPATH`.

```bash
CLOJURE_CLASSPATH=/caminho/clojure-1.12.5.jar:/caminho/spec.alpha.jar:/caminho/core.specs.alpha.jar \
  make compatibility-oracle
```

`oracle --bless` altera expectativas somente quando solicitado explicitamente, apenas
em casos `oracle = "equal"`, e atualiza os checksums. Divergências declaradas nunca são
sobrescritas pelo resultado da JVM.

Na comparação diferencial:

- maps e sets são comparados estruturalmente, sem depender de ordem de hash;
- erros são comparados por categoria e fragmento estável;
- diferenças documentadas usam `expected-diff`;
- acidentes de implementação da JVM não são promovidos a contrato.

## Gates de cobertura

Each `make coverage` run also writes `target/coverage/coverage.json` and a readable
crate/module summary. `config/coverage-baseline.json` is a versioned ratchet for every
crate and the high-risk modules identified in ADR-0011: a recorded line, function, or
region percentage cannot decline. Changed executable Rust lines require 90% coverage;
comments, whitespace, and lines without an LLVM coverage segment are outside the diff
denominator.

`make coverage` chama `scripts/coverage.sh`, que usa `cargo-llvm-cov` e aplica:

- no mínimo 82% de regiões;
- no mínimo 82% de funções;
- no mínimo 82% de linhas;
- no mínimo 30% de linhas em cada arquivo medido.

No último resultado registrado, o workspace atingiu 84,08% de regiões, 82,66% de
funções e 83,45% de linhas.

## Benchmarks

Os benchmarks validam o resultado antes de aceitar métricas:

- [`benchmarks/cracking/`](../benchmarks/cracking): 60 casos por capítulo;
- [`benchmarks/cormen/`](../benchmarks/cormen): 30 casos CLRS.

Os CSVs registram, por caso e implementação, tempo de parede, CPU e pico de memória. A
comparação JVM deve manter versões, warmup, repetição e escala registrados para evitar
comparar Cranelift frio com HotSpot aquecido sem contexto.

`make benchmark-page-refresh` is the publication gate for the comparison CSVs and
Pages assets. It requires ten complete Native × JVM rounds, verifies that case
identity, scale, runtime version, checksum, and status are stable across the sample
set, then publishes the per-case median for every measured field. Ratios are
recomputed from those medians, and the ten raw CSVs are retained under `target/` for
audit. A failed or incomplete round must leave the published artifacts unchanged.

The optional optimization IR has an additional executable native A/B gate defined by
[OPTIMIZATION_IR_SPEC](OPTIMIZATION_IR_SPEC.md#13-blocking-cormen-non-regression-gate).
It compares `--ir-opt none` with `--ir-opt safe` on the same commit for at least seven
paired Cormen repetitions. A checksum difference, aggregate regression, confirmed
chapter regression, or confirmed per-case regression blocks pass promotion. The JVM
columns remain contextual and are not rewritten by that native A/B gate.

`scripts/conformance.sh verify --status active --ir-opt safe` executes the active
compiled corpus through the opt-in IR adapter. The dedicated performance gate is
`make benchmarks-cormen-ir`; passing ordinary conformance alone does not promote the
profile. The current seven-pair, scale-25 result passes at 0.9568 wall and 0.9565 CPU
candidate/control while preserving every Cormen checksum.

ADR-0015 candidates use the same paired runner with an explicit control:

```bash
benchmarks/cormen/compare-ir.sh \
  --control-ir-opt safe \
  --candidate-ir-opt safe \
  --candidate-experiment adr15 \
  --repetitions 7 --scale 25
```

The first ADR-0015 run was correctly rejected at 1.0066 wall and 1.0054 CPU because it
regressed in aggregate and exceeded the per-case median point-estimate ceiling.

## Aceite

Uma mudança está pronta quando:

1. `make quality` passa;
2. os gates de cobertura permanecem acima dos limites;
3. `make compatibility` passa sem rede e sem JVM;
4. casos novos têm status, razão, tracking e checksum;
5. mudanças de desempenho relevantes incluem checksum e metodologia reproduzível.

An IR optimization is ready only after the additional Cormen gate reports zero
aggregate, chapter, and confirmed per-case execution-time regressions.

Para promover o gate de I/O, também são obrigatórios zero handles vazados, GC stress,
sanitizers, leitura/escrita em blocos e os snapshots isolados definidos na
[IO_SPEC](IO_SPEC.md).
