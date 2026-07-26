# clojure-native

Native Clojure compilation without a JVM, built with Rust, Cranelift, and a small C
runtime.

Compilação nativa de Clojure sem JVM, construída com Rust, Cranelift e um pequeno runtime
em C.

[English](#english) · [Português](#portugues)

> Experimental project under active development. It implements a documented subset of
> Clojure and is not production-ready.
>
> Projeto experimental em desenvolvimento ativo. Ele implementa um subconjunto
> documentado de Clojure e ainda não está pronto para produção.

---

<a id="english"></a>

## English

### Overview

`clojure-native` reads, interprets, and AOT-compiles Clojure source code into standalone
native executables. The generated program does not require a JVM at runtime: Cranelift
emits a native object, which is linked with the project's embedded C runtime.

The repository is both an implementation and an architecture notebook. Specifications,
compatibility boundaries, implementation plans, and architectural decisions live under
[`specs/`](specs/README.md).

### Current capabilities

- Reader with source spans, reader macros, and deterministic diagnostics.
- Bootstrap interpreter used by `eval`, scripts, and macro infrastructure.
- Cranelift AOT code generation for standalone native executables.
- Functions, closures, higher-order functions, fixed/variadic/multiple arities, and
  `apply`.
- `if`, `do`, `let`, `loop/recur`, direct recursion, and core macro expansion.
- Tagged fixnums with native fast paths for `+`, `-`, `inc`, `dec`, and integer
  comparisons. Invalid types and overflow use checked runtime slow paths.
- Strings, lists, keywords, maps, sets, and persistent vectors backed by a 32-way
  bitmapped vector trie.
- Records and protocol dispatch through `defrecord`, `defprotocol`, and `extend-type`.
- A compiled core subset with functions such as `map`, `filter`, `reduce`, `range`,
  `into`, `mapv`, `take`, `drop`, and `comp`.
- Precise, non-moving, single-threaded mark-sweep GC with generated shadow-stack roots.
- Direct root-stack loads/stores in generated code, removing root helper calls from the
  hot path.

For the detailed implementation status, see
[`specs/README.md`](specs/README.md). The optimization roadmap and its decision record are
in [`specs/optime.md`](specs/optime.md) and
[`ADR-0006`](specs/adr/0006-codegen-optimization.md).

### Requirements

- Rust 1.74 or newer and Cargo.
- A C compiler available as `cc`, or through the `CC` environment variable.
- A host platform supported by the current Cranelift and native linker setup.

### Build the compiler

```bash
cargo build --release -p clojure-native-cli
./target/release/clojure-native --help
```

### Compile and run a native Clojure program

```bash
./target/release/clojure-native build examples/hello.clj -o hello-native
./hello-native
```

Expected output:

```text
Hello from native Clojure
```

### Other CLI commands

```bash
# Print the forms produced by the reader
./target/release/clojure-native read examples/hello.clj

# Evaluate an expression with the bootstrap interpreter
./target/release/clojure-native eval '(reduce + 0 (range 10))'

# Run a source file through the interpreter
./target/release/clojure-native run examples/demo.clj
```

### Test and validate

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The native end-to-end suite also exercises the collector with `CLJN_GC_STRESS=1`, which
forces collection at every allocation.

### Project layout

| Path | Purpose |
| --- | --- |
| `crates/clojure-reader` | Reader and parser |
| `crates/clojure-interp` | Bootstrap interpreter |
| `crates/clojure-analyzer` | Analysis, macro expansion, closures, records, and protocols |
| `crates/clojure-codegen` | Cranelift code generation and embedded C runtime |
| `crates/clojure-native-cli` | `read`, `eval`, `run`, and `build` commands |
| `benchmarks/cracking` | 60 chapter-organized algorithm benchmarks |
| `examples` | Clojure examples and performance workloads |
| `specs` | Language scope, runtime model, plans, risks, and ADRs |
| `docs` | Short usage and architecture guides |

### Known limitations

- This is a Clojure subset, not a drop-in replacement for Clojure/JVM.
- Bignums, ratios, and floating-point numbers are not implemented.
- Native compilation currently targets the host and invokes a system C linker.
- The GC is single-threaded and non-moving.
- Rooting is still eager; the next optimization phase moves root writes to safepoints
  using liveness information.
- Maps and sets still use simple representations; broader persistent collection work is
  ongoing.
- Java interop and the JVM ecosystem are outside the current native runtime.

---

<a id="portugues"></a>

## Português

### Visão geral

O `clojure-native` lê, interpreta e compila antecipadamente código-fonte Clojure para
executáveis nativos autônomos. O programa gerado não precisa de JVM em tempo de execução:
o Cranelift produz um objeto nativo, que é linkado com o runtime C embutido no projeto.

O repositório funciona tanto como implementação quanto como registro arquitetural.
Especificações, limites de compatibilidade, planos de implementação e decisões
arquiteturais ficam em [`specs/`](specs/README.md).

### Recursos atuais

- Reader com spans de código-fonte, macros de leitura e diagnósticos determinísticos.
- Interpretador de bootstrap usado por `eval`, scripts e infraestrutura de macros.
- Geração AOT com Cranelift para executáveis nativos autônomos.
- Funções, closures, funções de ordem superior, aridades fixas, variádicas e múltiplas,
  além de `apply`.
- `if`, `do`, `let`, `loop/recur`, recursão direta e expansão de macros do core.
- Fixnums tagged com fast paths nativos para `+`, `-`, `inc`, `dec` e comparações
  inteiras. Tipos inválidos e overflow seguem para slow paths verificados no runtime.
- Strings, listas, keywords, mapas, sets e vetores persistentes implementados com
  bitmapped vector trie 32-way.
- Records e dispatch de protocolos com `defrecord`, `defprotocol` e `extend-type`.
- Subconjunto compilado do core com funções como `map`, `filter`, `reduce`, `range`,
  `into`, `mapv`, `take`, `drop` e `comp`.
- GC mark-sweep preciso, não-móvel e single-thread com shadow-stack de roots gerado.
- Loads/stores diretos da pilha de roots no código gerado, removendo helpers de roots do
  caminho quente.

O estado detalhado está em [`specs/README.md`](specs/README.md). O plano de otimização e
sua decisão arquitetural estão em [`specs/optime.md`](specs/optime.md) e na
[`ADR-0006`](specs/adr/0006-codegen-optimization.md).

### Requisitos

- Rust 1.74 ou mais recente e Cargo.
- Um compilador C disponível como `cc` ou configurado pela variável de ambiente `CC`.
- Uma plataforma host suportada pela configuração atual do Cranelift e do linker nativo.

### Compilar o compilador

```bash
cargo build --release -p clojure-native-cli
./target/release/clojure-native --help
```

### Compilar e executar um programa Clojure nativo

```bash
./target/release/clojure-native build examples/hello.clj -o hello-native
./hello-native
```

Saída esperada:

```text
Hello from native Clojure
```

### Outros comandos da CLI

```bash
# Imprime as forms produzidas pelo reader
./target/release/clojure-native read examples/hello.clj

# Avalia uma expressão com o interpretador de bootstrap
./target/release/clojure-native eval '(reduce + 0 (range 10))'

# Executa um arquivo-fonte pelo interpretador
./target/release/clojure-native run examples/demo.clj
```

### Testes e validação

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

A suíte end-to-end nativa também exercita o coletor com `CLJN_GC_STRESS=1`, forçando uma
coleta em cada alocação.

### Estrutura do projeto

| Caminho | Responsabilidade |
| --- | --- |
| `crates/clojure-reader` | Reader e parser |
| `crates/clojure-interp` | Interpretador de bootstrap |
| `crates/clojure-analyzer` | Análise, expansão de macros, closures, records e protocolos |
| `crates/clojure-codegen` | Codegen Cranelift e runtime C embutido |
| `crates/clojure-native-cli` | Comandos `read`, `eval`, `run` e `build` |
| `benchmarks/cracking` | 60 benchmarks algorítmicos organizados por capítulo |
| `examples` | Exemplos Clojure e cargas de performance |
| `specs` | Escopo, modelo de runtime, planos, riscos e ADRs |
| `docs` | Guias breves de uso e arquitetura |

### Limitações conhecidas

- Este é um subconjunto de Clojure, não um substituto direto para Clojure/JVM.
- Bignums, ratios e números de ponto flutuante ainda não foram implementados.
- A compilação nativa atualmente usa o host e invoca um linker C do sistema.
- O GC é single-thread e não-móvel.
- O rooting ainda é eager; a próxima fase move as escritas de roots para safepoints
  usando informações de liveness.
- Mapas e sets ainda usam representações simples; o trabalho com coleções persistentes
  continua.
- A interoperabilidade Java e o ecossistema JVM estão fora do runtime nativo atual.
