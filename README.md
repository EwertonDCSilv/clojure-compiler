# clojure-native

`clojure-native` é um projeto de implementação nativa de Clojure em Rust. Ele compila arquivos Clojure para binários nativos autônomos usando Cranelift e um runtime C integrado.

## Estrutura do projeto

- `Cargo.toml` - workspace Rust principal
- `crates/` - crates do compilador e runtime
- `examples/` - exemplos e benchmarks em Clojure
- `specs/` - especificações, ADRs e planejamento
- `docs/` - documentação gerada do projeto

## Como construir

```bash
cargo build -p clojure-native-cli
```

## Comandos principais

```bash
cargo run -p clojure-native-cli -- read <arquivo.clj>
cargo run -p clojure-native-cli -- eval "(println \"Olá\")"
cargo run -p clojure-native-cli -- run <arquivo.clj> [--main]
cargo run -p clojure-native-cli -- build <arquivo.clj> -o <binário>
```

## Exemplo

```bash
cargo run -p clojure-native-cli -- build examples/loop-benchmark.clj -o loop-benchmark-native
./loop-benchmark-native
```

## Status atual

- Projeto em desenvolvimento
- Suporte atual: `defn`, `if`, `let`, `do`, `loop/recur`, recursão direta, primitivas inteiras e operações básicas de lista/string
- O compilador ainda depende de runtime para muitas operações e o conjunto de linguagem é limitado

## Documentação

Veja também:

- `docs/overview.md`
- `docs/architecture.md`
- `docs/usage.md`
- `specs/README.md`

## Limitações conhecidas

- Não há suporte a bignums nem floats
- Operações primitivas ainda usam chamadas de runtime C
- Coleções avançadas e macros complexas ainda não estão totalmente implementadas

## Requisitos

- Rust + Cargo
- Compilador C (`cc`)

## Contato

Este projeto é destinado a explorar a compilação nativa de Clojure sem JVM e documentar a evolução incremental da implementação.
