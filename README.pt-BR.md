# clojure-compiler

[English](README.md) · [Português (Brasil)](README.pt-BR.md)

Compilação nativa de Clojure sem JVM, construída com Rust, Cranelift e um runtime C
compacto. O repositório se chama `clojure-compiler`; seu binário de linha de comando se
chama `clojure-native`.

> Projeto experimental em desenvolvimento ativo. Ele implementa um subconjunto
> documentado de Clojure e ainda não está pronto para produção.

## Visão geral

O `clojure-native` lê, interpreta e compila antecipadamente código-fonte Clojure para
executáveis nativos autônomos. O programa gerado não precisa de JVM em tempo de
execução: o Cranelift produz um objeto nativo, que é linkado com o runtime C embutido.

O repositório funciona tanto como implementação quanto como registro arquitetural.
Especificações, limites de compatibilidade, planos de implementação e decisões
arquiteturais ficam em [`specs/`](specs/README.md).

## Recursos atuais

- Reader com spans de código-fonte, macros de leitura, suporte a Unicode e diagnósticos
  determinísticos.
- Interpretador de bootstrap usado por `eval`, scripts e infraestrutura de macros.
- Geração AOT com Cranelift para executáveis nativos autônomos.
- Funções, closures, funções de ordem superior, aridades fixas, variádicas e múltiplas,
  além de `apply`.
- `if`, `do`, `let`, `loop/recur`, recursão direta e expansão das macros de core
  atualmente suportadas.
- Fixnums tagueados com fast paths nativos verificados para `+`, `-`, `*`, `quot`,
  `mod`, `inc`, `dec` e comparações inteiras.
- Strings, listas, keywords, vetores persistentes e mapas/sets persistentes híbridos.
  Vetores usam trie bitmap de 32 vias; mapas/sets pequenos promovem para HAMT de 32 vias.
- Mapas e sets ordenados apoiados em árvore rubro-negra persistente inclinada à esquerda.
- Records e dispatch de protocolos com `defrecord`, `defprotocol` e `extend-type`.
- Subconjunto compilado de `clojure.core` com 26 funções, incluindo `map`, `filter`,
  `reduce`, `range`, `into`, `mapv`, `take`, `drop` e `comp`.
- GC mark-sweep preciso, não móvel e single-thread com shadow stack de roots gerado.
- Loads/stores diretos na pilha de roots no código gerado, retirando chamadas auxiliares
  dos caminhos quentes.

O estado detalhado está em [`specs/README.md`](specs/README.md). O roteiro de otimização
e sua decisão arquitetural estão em [`specs/optime.md`](specs/optime.md) e na
[`ADR-0006`](specs/adr/0006-codegen-optimization.md).
O gate proposto de I/O nativo está separado na
[`IO_SPEC`](specs/IO_SPEC.md) e na
[`ADR-0007`](specs/adr/0007-native-io-and-runtime-reader.md); hoje só estão entregues
os casos de conformidade marcados como `active`.

## Requisitos

- Rust 1.74 ou mais recente e Cargo.
- Um compilador C disponível como `cc` ou configurado pela variável de ambiente `CC`.
- Uma plataforma host suportada pela configuração atual do Cranelift e do linker nativo.

## Compilar o compilador

```bash
cargo build --release -p clojure-native-cli
./target/release/clojure-native --help
```

## Compilar e executar um programa nativo

```bash
./target/release/clojure-native build examples/hello.clj -o hello-native
./hello-native
```

Saída esperada:

```text
Hello from native Clojure
```

A otimização do Cranelift pode ser escolhida com `--opt-level none`, `speed` ou
`speed-and-size`. O padrão atual é `none`; os modos otimizados continuam explícitos
enquanto suas regressões nos benchmarks são investigadas.

## Outros comandos da CLI

```bash
# Imprime as forms produzidas pelo reader
./target/release/clojure-native read examples/hello.clj

# Avalia uma expressão com o interpretador de bootstrap
./target/release/clojure-native eval '(reduce + 0 (range 10))'

# Executa um arquivo-fonte pelo interpretador
./target/release/clojure-native run examples/demo.clj
```

## Testes e validação

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
scripts/coverage.sh
scripts/conformance.sh verify
```

A matriz executável de compatibilidade contém atualmente 431 casos nos níveis A–E:
160 ativos, 239 falhas esperadas e 32 itens pendentes de inventário. Os níveis D e E
agora incluem recortes executáveis de bibliotecas puras e aplicações autônomas, além de
lacunas esperadas concretas e inventário de projetos, incluindo uma API HTTP Hello
World em Pedestal. A matriz também inventaria toda a superfície proposta de I/O como
falhas esperadas, sem afirmar que ela está disponível. A verificação roda
offline e sem JVM, confere a integridade das fixtures e grava relatórios em
`target/conformance/`.

```bash
scripts/conformance.sh list --level A
scripts/conformance.sh list --namespace clojure.core
scripts/conformance.sh verify
```

Consulte [`specs/conformance/README.md`](specs/conformance/README.md) para filtros,
checksums, relatórios e o oracle manual opcional com Clojure/JVM 1.12.5.

## Benchmarks

As duas suítes têm um runner nativo e um runner de comparação. Seus CSVs registram
tempo de parede, tempo de CPU e pico de memória:

```bash
benchmarks/cracking/run.sh
benchmarks/cracking/compare-clojure.sh --csv benchmarks/cracking/results/comparison.csv
benchmarks/cormen/run.sh
benchmarks/cormen/compare-clojure.sh --csv benchmarks/cormen/results/comparison.csv
```

- [`benchmarks/cracking`](benchmarks/cracking/README.md): 60 casos organizados por
  capítulo, inspirados em *Cracking the Coding Interview*.
- [`benchmarks/cormen`](benchmarks/cormen/README.md): 30 casos de algoritmos no estilo
  CLRS, com validação por checksum.

## Estrutura do projeto

| Caminho | Responsabilidade |
| --- | --- |
| `crates/clojure-reader` | Reader e parser |
| `crates/clojure-interp` | Interpretador de bootstrap |
| `crates/clojure-analyzer` | Análise, expansão de macros, closures, records e protocolos |
| `crates/clojure-codegen` | Codegen Cranelift e runtime C embutido |
| `crates/clojure-native-cli` | Comandos `read`, `eval`, `run` e `build` |
| `crates/clojure-test-support` | Runner, schema, oracle e relatórios de conformidade |
| [`tests/conformance`](tests/conformance) | Fixtures executáveis de compatibilidade A–E |
| `examples` | Exemplos Clojure e cargas de desempenho |
| `specs` | Escopo, modelo de runtime, planos, riscos e ADRs |
| `docs` | Guias breves de uso, visão geral e arquitetura |

## Limitações conhecidas

- Este é um subconjunto de Clojure, não um substituto direto para Clojure/JVM.
- O reader aceita literais de ponto flutuante, mas a execução numérica compilada
  nativamente ainda é limitada a fixnums. Bignums, ratios e BigDecimal não existem.
- Macros definidas pelo usuário, sequências lazy/infinitas, exceções, carregamento
  dinâmico de namespaces e compilação de projetos com múltiplos arquivos não estão
  disponíveis no caminho nativo.
- Stdin geral, arquivos, operações de filesystem, reader EDN em runtime e
  redirecionamento de streams estão especificados, mas não implementados; a saída
  nativa atual limita-se ao baseline ativo de `print`/`println`.
- A compilação nativa usa o host e invoca um linker C do sistema.
- O GC é single-thread e não móvel. O rooting ainda é eager; uma fase planejada usará
  liveness para posicionar roots nos safepoints de alocação.
- Mapas/sets CHAMP e transients continuam como trabalho futuro.
- A interoperabilidade Java e bibliotecas do ecossistema JVM estão fora do runtime
  nativo atual.
