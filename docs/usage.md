# Uso do clojure-native

## Compilar o CLI

```bash
cargo build -p clojure-native-cli
```

## Comandos suportados

- `clojure-native read <arquivo.clj>`
  - Lê e imprime as forms do arquivo
- `clojure-native eval <expr>`
  - Avalia uma expressão em modo interpretado
- `clojure-native run <arquivo.clj> [--main]`
  - Executa um script Clojure via interpretador
- `clojure-native build <arquivo.clj> [-o saída]`
  - Compila um arquivo Clojure para binário nativo

## Exemplos

### Rodar um script

```bash
cargo run -p clojure-native-cli -- run examples/loop-benchmark.clj
```

### Compilar um programa

```bash
cargo run -p clojure-native-cli -- build examples/loop-benchmark.clj -o loop-benchmark-native
```

### Executar o binário compilado

```bash
./loop-benchmark-native
```

## Observações

- O comando `build` depende de um compilador C (`cc`) no sistema para linkar o runtime C e o objeto gerado.
- O binário resultante é um executável nativo que não requer JVM em tempo de execução.
