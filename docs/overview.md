# clojure-native

`clojure-native` é um projeto de implementação nativa de Clojure em Rust. O objetivo é compilar código-fonte Clojure para binários nativos autônomos, sem JVM em tempo de execução e sem bytecode `.class`.

## Visão geral

- Backend de compilação: Cranelift AOT
- Representação de valores: `i64` tagueado para inteiros, ponteiros para strings e listas, e objetos GC-traçados
- Runtime embutido: primitivas aritméticas, comparações, listas, strings, impressões, e GC
- Suporte de linguagem: subseto de Clojure com `defn`, `if`, `let`, `do`, `loop/recur`, recursão direta, chamadas de função, strings, listas e algumas primitivas

## Estrutura do workspace

- `crates/` - crates Rust do workspace
  - `clojure-native-cli` - CLI do projeto
  - `clojure-analyzer` - análise de AST, verificações, `recur`
  - `clojure-codegen` - geração de código Cranelift e runtime C
  - `clojure-reader` - parser e reader Clojure
  - `clojure-interp` - interpretador usado durante bootstrap e testes
  - `clojure-value` - representação de valores Clojure
  - `clojure-syntax` - AST e análise sintática
  - `clojure-span` - gerenciamento de spans/fonte
  - `clojure-diagnostics` - diagnósticos e renderização de erros
- `examples/` - exemplos e benchmarks
- `specs/` - especificações do projeto e ADRs

## Como usar

### Compilar o CLI

```bash
cargo build -p clojure-native-cli
```

### Executar um programa em modo interpretado

```bash
cargo run -p clojure-native-cli -- run examples/loop-benchmark.clj
```

### Compilar um programa para binário nativo

```bash
cargo run -p clojure-native-cli -- build examples/loop-benchmark.clj -o loop-benchmark-native
```

### Executar o binário nativo gerado

```bash
./loop-benchmark-native
```

## Subconjunto suportado (atual)

- `defn`, `if`, `do`, `let`
- `loop` / `recur`
- `+`, `-`, `*`, `quot`, `mod`, `inc`, `dec`, `=`, `<`, `<=`, `>`, `>=`, `not`, `nil?`, `empty?`, `first`, `rest`, `count`, `list`, `str`, `println`
- Estruturas imutáveis básicas: strings e listas cons

## Limitações conhecidas

- A representação atual é `i64` tagueado; não há bignums nem floats
- O garbage collector é um runtime C integrado
- A geração de código atual ainda depende de muitas chamadas de runtime para operações primitivas
- O compilador não suporta `lazy-seq`, macros complexas ou muitos recursos avançados de Clojure
