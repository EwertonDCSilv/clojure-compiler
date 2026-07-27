# Estratégia de testes

A estratégia combina testes Rust, execução nativa end-to-end, conformidade versionada,
GC stress, cobertura e benchmarks. Clojure/JVM é apenas um oracle manual: não é
dependência da CI, do runner `verify` nem do binário produzido.

## Camadas

| Camada | Comando ou ferramenta | Contrato |
| --- | --- | --- |
| Formatação | `cargo fmt --all --check` | nenhuma divergência |
| Lints | `cargo clippy --workspace --all-targets -- -D warnings` | zero warnings |
| Unitários/integração | `cargo test --workspace` | todos verdes |
| Cobertura | `scripts/coverage.sh` | gates globais e por arquivo |
| Conformidade | `scripts/conformance.sh verify` | ativos/xfail/checksums corretos |
| GC | casos com `CLJN_GC_STRESS=1` | coleta a cada alocação sem corrupção |
| Benchmarks | runners Cracking e Cormen | checksum e métricas comparáveis |
| Oracle JVM | `scripts/conformance.sh oracle ...` | operação exclusivamente manual |

Property testing, fuzzing, Miri, sanitizers e uma matriz multiplataforma mais ampla
continuam recomendados para as fases que introduzirem novas superfícies de `unsafe`,
concorrência e FFI.

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
│   ├── errors/
│   └── gc/
├── level-c-stdlib/
│   ├── clojure-core/
│   ├── clojure-string/
│   ├── clojure-set/
│   ├── clojure-walk/
│   ├── clojure-edn/
│   └── clojure-test/
├── level-d-pure-libraries/
└── level-e-ecosystem/
```

O inventário atual contém 206 casos:

- 154 `active`: executados e bloqueantes;
- 20 `xfail`: precisam falhar pela razão declarada; um passe inesperado também bloqueia;
- 32 `pending`: schema e checksum são validados, mas o caso não é executado.

Níveis A–C classificam a sintaxe, a semântica e a biblioteca realmente executáveis.
O nível D executa bibliotecas puras autocontidas e registra lacunas de macros,
namespaces, lazy seqs, metadata e exceções. O nível E executa aplicações autônomas de
arquivo único e transforma dependências, JARs, Java, carregamento dinâmico e concorrência
em `xfail` concretos. Projetos que exigem loader/packaging continuam `pending`.
Entre eles está uma API HTTP Hello World em Pedestal, com `deps.edn`, fonte, requisição
e resposta esperada versionados como contrato do alvo.

Cada caso é autocontido e tem `case.toml`, `input.clj` e a expectativa aplicável. O
manifesto registra `status`, `class`, `target`, `oracle`, timeout, modo GC stress, razão
e tracking. Mapas e sets são comparados estruturalmente; newlines e caminhos temporários
são normalizados.

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
  scripts/conformance.sh oracle --check
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

`scripts/coverage.sh` usa `cargo-llvm-cov` e aplica:

- no mínimo 82% de regiões;
- no mínimo 82% de funções;
- no mínimo 82% de linhas;
- no mínimo 30% de linhas em cada arquivo medido.

No último resultado registrado, o workspace atingiu 85,86% de regiões, 84,98% de
funções e 85,22% de linhas.

## Benchmarks

Os benchmarks validam o resultado antes de aceitar métricas:

- [`benchmarks/cracking/`](../benchmarks/cracking): 60 casos por capítulo;
- [`benchmarks/cormen/`](../benchmarks/cormen): 30 casos CLRS.

Os CSVs registram, por caso e implementação, tempo de parede, CPU e pico de memória. A
comparação JVM deve manter versões, warmup, repetição e escala registrados para evitar
comparar Cranelift frio com HotSpot aquecido sem contexto.

## Aceite

Uma mudança está pronta quando:

1. `fmt`, `clippy` e testes Rust passam;
2. os gates de cobertura permanecem acima dos limites;
3. `scripts/conformance.sh verify` passa sem rede e sem JVM;
4. casos novos têm status, razão, tracking e checksum;
5. mudanças de desempenho relevantes incluem checksum e metodologia reproduzível.
