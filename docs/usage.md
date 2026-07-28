# Uso do compilador

[Índice da documentação](README.md) · [Visão geral](overview.md) ·
[Arquitetura](architecture.md) · [README em português](../README.pt-BR.md)

> Comandos revisados no [`HEAD 3e71bc1`](https://github.com/EwertonDCSilv/clojure-compiler/commit/3e71bc1996b689233c80516b4b4aff52259c2cdf).
> Veja o [snapshot documentado](SNAPSHOT.md).

O repositório se chama `clojure-compiler`; os comandos abaixo usam o binário
`clojure-native`.

Todos os comandos devem ser executados na raiz do repositório.

## Requisitos

- Rust 1.74 ou mais recente, com Cargo;
- GNU Make;
- compilador C disponível como `cc` ou definido em `CC`;
- GNU `time` em `/usr/bin/time` para benchmarks;
- `cargo-llvm-cov` e `llvm-tools-preview` para cobertura;
- Java para a comparação opcional com Clojure/JVM; os relatórios versionados usam Java
  21, e `curl` com acesso à rede só é necessário para preencher o cache inicial.

## Preparar o CLI

```bash
make release
./target/release/clojure-native --help
```

O build nativo requer um compilador C disponível como `cc` ou definido em `CC`.
Use `make help` para consultar todas as rotinas disponíveis.

## Instalar no Linux

```bash
make install
~/.local/bin/clojure-native --help
```

O destino padrão é `~/.local/bin`. Ele pode ser alterado com `PREFIX`, `BINDIR` e
`DESTDIR`; o Makefile não modifica automaticamente o `PATH` nem arquivos de configuração
do shell.

No snapshot documentado, `make install` recompila o pacote `clojure-native-cli` em
release, cria o diretório de destino e instala `clojure-native` com modo `0755`.

Exemplos:

```bash
# Instalação global
sudo make install PREFIX=/usr/local

# Staging para empacotamento, sem escrever no prefixo real
make install PREFIX=/usr DESTDIR=/tmp/pacote

# Diretório final explícito
make install BINDIR=/opt/clojure-native/bin
```

## Rotinas do Makefile

`make help` é a referência curta e sempre acompanha os alvos implementados.

| Alvo | O que executa |
| --- | --- |
| `make build` | workspace completo em modo debug |
| `make release` | CLI otimizado em `target/release/clojure-native` |
| `make quality` | formato, lints Rust/Clojure e testes do workspace |
| `make coverage` | gates globais e por arquivo |
| `make compatibility` | matriz de conformidade A–E |
| `make benchmarks` | 60 casos Cracking, 30 Cormen e 8 Exercism |
| `make benchmarks-compare` | comparação nativo × Clojure/JVM AOT |
| `make all` | qualidade, cobertura, compatibilidade e benchmarks completos |
| `make ci` | mesmos comandos dos jobs do GitHub Actions |
| `make install` | build release e instalação do binário |

Alvos auxiliares:

| Alvo | Uso |
| --- | --- |
| `make check` | análise rápida de todos os targets sem produzir release |
| `make fmt` / `make fmt-check` | aplicar ou apenas validar `rustfmt` |
| `make lint` | Clippy com warnings bloqueantes e lint Clojure |
| `make test` | testes Rust do workspace |
| `make test-runtime` | harnesses dedicados do runtime C |
| `make test-runtime-sanitize` | runtime C com ASan e UBSan |
| `make compatibility-list` | inventário filtrável sem executar a matriz |
| `make compatibility-oracle` | comparação manual com Clojure/JVM |
| `make benchmarks-list` | lista os casos das três suítes |
| `make benchmarks-ci` | recorte de checksums usado na CI |
| `make benchmarks-charts` | regenera os SVGs dos relatórios e da página do projeto |
| `make benchmarks-compare-cracking` | comparação JVM somente da suíte Cracking |
| `make benchmarks-compare-cormen` | comparação JVM somente da suíte Cormen |
| `make benchmarks-compare-exercism` | comparação JVM do subconjunto Exercism promovido |
| `make exercism-compatibility` | compila as 101 referências e os 493 arquivos do checkout externo |

Variáveis frequentes:

| Variável | Exemplo |
| --- | --- |
| `CONFORMANCE_ARGS` | `--level A --status active` |
| `CRACKING_ARGS` | `--chapter 08 --scale 10` |
| `CORMEN_ARGS` | `--chapter 05` |
| `EXERCISM_ARGS` | `--scale 5` |
| `CRACKING_COMPARE_ARGS` | `--chapter 01 --scale 25` |
| `CORMEN_COMPARE_ARGS` | `--chapter 06 --scale 25` |
| `EXERCISM_COMPARE_ARGS` | `--scale 5` |
| `COVERAGE_ARGS` | `--html` |
| `PREFIX`, `BINDIR`, `DESTDIR` | destino da instalação |

Os aliases `compilar`, `testes`, `compatibilidade` e `instalar` também estão
disponíveis.

## Comandos

```bash
# Lê e imprime as forms
./target/release/clojure-native read arquivo.clj

# Avalia uma expressão no interpretador
./target/release/clojure-native eval '(+ 1 2)'

# Executa um script pelo interpretador
./target/release/clojure-native run arquivo.clj

# Executa as forms de topo e chama -main explicitamente ao final
./target/release/clojure-native run arquivo.clj --main

# Compila e linka um executável nativo
./target/release/clojure-native build arquivo.clj -o programa
./programa
```

`build` aceita `--opt-level none`, `speed` ou `speed-and-size`. O padrão atual é
`none`; os modos otimizados são opt-in enquanto as regressões observadas na suíte Cormen
são investigadas.

`--ir-opt none|safe` seleciona separadamente a IR mantida pelo compilador. `none`
continua sendo o padrão estável. `safe` é opt-in e a implementação continua parcial,
mas o perfil atual passou `make benchmarks-cormen-ir`: 0,9568 em wall e 0,9565 em CPU
contra o pipeline direto no gate de sete pares e escala 25.

Sem `-o`/`--output`, o nome de saída é o nome do arquivo sem a extensão. O binário
produzido não requer JVM em tempo de execução, mas o build invoca o compilador C do
sistema para o link final.

## Testes e gates

```bash
make quality
make coverage
make compatibility
make benchmarks

# Todos os gates locais
make all

# Recorte equivalente aos comandos da CI
make ci
```

`scripts/lint-clojure.sh` verifica com `clj-kondo` os cores de bootstrap, exemplos,
benchmarks e o oracle JVM. No Linux x86_64, a versão fixada é instalada automaticamente
em `target/tools/`; em outras plataformas, use `CLJ_KONDO_BIN` ou disponibilize
`clj-kondo` no `PATH`.

O gate de cobertura exige no mínimo 82% de linhas, funções e regiões no workspace e 30%
de linhas por arquivo. Ele requer `cargo-llvm-cov` e o componente
`llvm-tools-preview`. A suíte de conformidade roda offline e escreve:

- `target/conformance/report.json`;
- `target/conformance/report-summary.txt`.

Filtros úteis:

```bash
make compatibility-list CONFORMANCE_ARGS="--level A --status active"
make compatibility-list CONFORMANCE_ARGS="--area arithmetic"
make compatibility-list CONFORMANCE_ARGS="--namespace clojure.core"
```

O oracle Clojure/JVM 1.12.5 é exclusivamente manual. Os comandos `oracle --check` e
`oracle --bless`, incluindo a configuração de `CLOJURE_CLASSPATH`, estão documentados
em [`specs/conformance/README.md`](../specs/conformance/README.md).

## Benchmarks

```bash
make benchmarks
make benchmarks-compare
make benchmarks-charts
make benchmarks-cracking CRACKING_ARGS="--chapter 08 --scale 10"
```

Os runners preparam as variantes nativa e Clojure/JVM e geram resultados CSV com tempo
de parede, CPU e memória. `make benchmarks` não requer JVM; somente os alvos
`benchmarks-compare*` usam Java e os artefatos Clojure fixados. O alvo
`benchmarks-charts` usa os CSVs comparativos versionados para atualizar tanto os
relatórios quanto o painel de benchmarks da página do projeto.

Consulte o [catálogo central](../benchmarks/README.md) e os READMEs de
[`Cracking`](../benchmarks/cracking/README.md) e
[`Cormen`](../benchmarks/cormen/README.md) para filtros, repetições e caminhos de saída.
