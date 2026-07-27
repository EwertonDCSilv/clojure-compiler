# Uso do compilador

O repositório se chama `clojure-compiler`; os comandos abaixo usam o binário
`clojure-native`.

## Preparar o CLI

```bash
cargo build --release -p clojure-native-cli
./target/release/clojure-native --help
```

O build nativo requer um compilador C disponível como `cc` ou definido em `CC`.

## Comandos

```bash
# Lê e imprime as forms
./target/release/clojure-native read arquivo.clj

# Avalia uma expressão no interpretador
./target/release/clojure-native eval '(+ 1 2)'

# Executa um script pelo interpretador
./target/release/clojure-native run arquivo.clj

# Compila e linka um executável nativo
./target/release/clojure-native build arquivo.clj -o programa
./programa
```

`build` aceita `--opt-level none`, `speed` ou `speed-and-size`. O padrão atual é
`none`; os modos otimizados são opt-in enquanto as regressões observadas na suíte Cormen
são investigadas.

O binário produzido não requer JVM em tempo de execução.

## Testes

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
scripts/coverage.sh
scripts/conformance.sh verify
```

O gate de cobertura exige no mínimo 82% de linhas, funções e regiões no workspace e 30%
de linhas por arquivo. A suíte de conformidade roda offline e escreve:

- `target/conformance/report.json`;
- `target/conformance/report-summary.txt`.

Filtros úteis:

```bash
scripts/conformance.sh list --level A --status active
scripts/conformance.sh list --area arithmetic
scripts/conformance.sh list --namespace clojure.core
```

O oracle Clojure/JVM 1.12.5 é exclusivamente manual. Os comandos `oracle --check` e
`oracle --bless`, incluindo a configuração de `CLOJURE_CLASSPATH`, estão documentados
em [`specs/conformance/README.md`](../specs/conformance/README.md).

## Benchmarks

```bash
benchmarks/cracking/run.sh
benchmarks/cormen/run.sh
```

Os runners preparam as variantes nativa e Clojure/JVM e geram resultados CSV com tempo
de parede, CPU e memória. Consulte os READMEs de
[`Cracking`](../benchmarks/cracking/README.md) e
[`Cormen`](../benchmarks/cormen/README.md) para filtros, repetições e caminhos de saída.
