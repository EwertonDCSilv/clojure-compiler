# clojure-compiler

[English](README.md) · [Português (Brasil)](README.pt-BR.md)

[Site do projeto](https://ewertondcsilv.github.io/clojure-compiler/) ·
[Código-fonte](https://github.com/EwertonDCSilv/clojure-compiler)

Compilação nativa de Clojure sem JVM, construída com Rust, Cranelift e um runtime C
compacto. O repositório se chama `clojure-compiler`; seu binário de linha de comando se
chama `clojure-native`.

> Projeto experimental em desenvolvimento ativo. Ele implementa um subconjunto
> documentado de Clojure e ainda não está pronto para produção.

> Snapshot documentado e compilador medido:
> [`HEAD 1dc69b5`](https://github.com/EwertonDCSilv/clojure-compiler/commit/1dc69b5b126c193c30e9f24fdddd549abb7ce4cb)
> (2026-07-28).
> Consulte a [política do snapshot e as medições atuais](docs/SNAPSHOT.md).

## Visão geral

O `clojure-native` lê, interpreta e compila antecipadamente código-fonte Clojure para
executáveis nativos autônomos. O programa gerado não precisa de JVM em tempo de
execução: o Cranelift produz um objeto nativo, que é linkado com o runtime C embutido.

O repositório funciona tanto como implementação quanto como registro arquitetural.
Especificações, limites de compatibilidade, planos de implementação e decisões
arquiteturais ficam em [`specs/`](specs/README.md).

## Documentação

| Guia | Conteúdo |
| --- | --- |
| [`docs/README.md`](docs/README.md) | Índice documental e política de fonte de verdade |
| [`docs/overview.md`](docs/overview.md) | Recursos e limitações atuais |
| [`docs/usage.md`](docs/usage.md) | CLI, Makefile, instalação, testes e benchmarks |
| [`docs/architecture.md`](docs/architecture.md) | Crates, pipeline AOT, runtime e GC |
| [`docs/SNAPSHOT.md`](docs/SNAPSHOT.md) | HEAD auditado, commit medido e resultados atuais |
| [`specs/conformance/README.md`](specs/conformance/README.md) | Contrato executável de compatibilidade A–E |
| [`specs/PEDESTAL_NATIVE_CONNECTOR_SPEC.md`](specs/PEDESTAL_NATIVE_CONNECTOR_SPEC.md) | Connector HTTP nativo planejado e subconjunto compatível com Pedestal |
| [`benchmarks/README.md`](benchmarks/README.md) | Catálogo e metodologia de 98 cargas de desempenho Native × JVM |

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
- Vetores transientes com construção mutável em lote, além de wrappers transientes de
  map/set com `transient`, `persistent!`, `conj!`, `assoc!` e `dissoc!`.
- `mapv` e `into` constroem por transientes estruturais. Um passe conservador do
  analyzer também promove acumuladores frescos de vetor em loops, incluindo o primeiro
  padrão suportado de parâmetro linear interprocedural.
- Vetores literais constantes formados somente por imediatos são construídos uma vez
  por site compilado, mantidos em cache e registrados como roots permanentes do GC.
- Records e dispatch de protocolos com `defrecord`, `defprotocol` e `extend-type`.
- `throw` e `try`/`catch`/`finally` nativos, incluindo unwind aninhado e capturas
  léxicas seguras sob GC.
- Dispatch de multimétodos por valor com `defmulti`, `defmethod` e fallback `:default`.
- Subconjunto compilado de `clojure.core` com 26 funções, incluindo `map`, `filter`,
  `reduce`, `range`, `into`, `mapv`, `take`, `drop` e `comp`.
- GC mark-sweep preciso, não móvel e single-thread com shadow stack de roots gerado.
- Loads/stores diretos na pilha de roots no código gerado, retirando chamadas auxiliares
  dos caminhos quentes.
- O runtime C está separado por subsistema para manutenção, mas continua compilado como
  uma única unidade de tradução com a mesma ABI.

O estado detalhado está em [`specs/README.md`](specs/README.md). O roteiro de otimização
e sua decisão arquitetural estão em [`specs/optime.md`](specs/optime.md) e na
[`ADR-0006`](specs/adr/0006-codegen-optimization.md). O estudo de alocação e a decisão
interprocedural parcialmente implementada estão na
[`ADR-0009`](specs/adr/0009-benchmark-performance-study.md) e na
[`ADR-0010`](specs/adr/0010-interprocedural-ephemeral-vectors.md).
O gate proposto de I/O nativo está separado na
[`IO_SPEC`](specs/IO_SPEC.md) e na
[`ADR-0007`](specs/adr/0007-native-io-and-runtime-reader.md); hoje só estão entregues
os casos de conformidade marcados como `active`.

## Requisitos

- Rust 1.74 ou mais recente e Cargo.
- GNU Make.
- Um compilador C disponível como `cc` ou configurado pela variável de ambiente `CC`.
- GNU `time` em `/usr/bin/time` para os benchmarks.
- `cargo-llvm-cov` e o componente Rust `llvm-tools-preview` para `make coverage`,
  `make all` e `make ci`.
- Java para a comparação opcional `make benchmarks-compare` com Clojure/JVM. Os
  relatórios de referência usam Java 21; `curl` e rede são necessários somente para
  preencher o cache de artefatos na primeira execução.
- Uma plataforma host suportada pela configuração atual do Cranelift e do linker nativo.

## Compilar o compilador

```bash
make release
./target/release/clojure-native --help
```

Use `make help` para listar os alvos de build, qualidade, testes, compatibilidade,
benchmarks e instalação.

## Instalar no Linux

```bash
make install
~/.local/bin/clojure-native --help
```

O destino padrão é `~/.local/bin`. Garanta que ele esteja no `PATH` ou altere o destino
com `PREFIX=/usr/local`, `BINDIR=/outro/bin` e, para empacotamento, `DESTDIR`. Uma
instalação global pode usar `sudo make install PREFIX=/usr/local`.

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
make quality
make coverage
make compatibility
make benchmarks

# Executa todos os gates locais acima
make all

# Reproduz os comandos usados pelo GitHub Actions
make ci
```

O gate de lint Clojure usa uma versão de `clj-kondo` fixada e validada por checksum. No
Linux x86_64, o script instala a ferramenta em `target/tools/` quando necessário; em
outras plataformas, instale `clj-kondo` e exponha o executável no `PATH` ou em
`CLJ_KONDO_BIN`. Fixtures de conformidade deliberadamente inválidas ficam fora, mas os
dois cores de bootstrap, exemplos, benchmarks de algoritmos e o oracle JVM são
verificados com warnings tratados como erro.

A matriz executável de compatibilidade contém atualmente 460 casos nos níveis A–E:
185 ativos, 243 falhas esperadas e 32 itens pendentes de inventário. Os níveis D e E
agora incluem recortes executáveis de bibliotecas puras e aplicações autônomas, além de
lacunas esperadas concretas e inventário de projetos, incluindo uma API HTTP Hello
World em Pedestal e 13 exemplares conceituais oficiais do Exercism. A matriz também
abrange toda a superfície proposta de I/O: os recortes implementados estão ativos e as
lacunas executáveis restantes continuam como falhas esperadas. A verificação roda
offline e sem JVM, confere a integridade das fixtures e grava relatórios em
`target/conformance/`.

```bash
make compatibility-list CONFORMANCE_ARGS="--level A"
make compatibility-list CONFORMANCE_ARGS="--namespace clojure.core"
make compatibility
```

Consulte [`specs/conformance/README.md`](specs/conformance/README.md) para filtros,
checksums, relatórios e o oracle manual opcional com Clojure/JVM 1.12.5.

## Benchmarks

As três suítes têm um runner nativo e um runner de comparação. Seus CSVs registram
tempo de parede, tempo de CPU e pico de memória:

Comece pelo [catálogo central de benchmarks](benchmarks/README.md) para consultar a
metodologia, as métricas, os relatórios e os links diretos para os 98 casos de desempenho.

```bash
make benchmarks
make benchmarks-compare
make benchmarks-charts

# Filtra uma suíte mantendo a mesma entrada
make benchmarks-cracking CRACKING_ARGS="--chapter 08 --scale 10"
```

- [`benchmarks/cracking`](benchmarks/cracking/README.md): 60 casos organizados por
  capítulo, inspirados em *Cracking the Coding Interview*.
- [`benchmarks/cormen`](benchmarks/cormen/README.md): 30 casos de algoritmos no estilo
  CLRS, com validação por checksum.
- [`benchmarks/exercism`](benchmarks/exercism/README.md): oito soluções públicas com
  cargas determinísticas Native × JVM. O inventário mais amplo de suporte às 114
  soluções é mantido separadamente pela suíte de conformidade.

Os snapshots ficam fixados no relatório de cada suíte. As três suítes usam o compilador
`1dc69b5`; Cracking e Cormen rodam em escala 25×, enquanto Exercism usa o snapshot
upstream `4a4c4fd` e escala 5×:

| Suíte | Parede nativo/JVM | CPU nativo/JVM | RSS mediano nativo/JVM |
| --- | ---: | ---: | ---: |
| Cracking | 7,71 / 22,27 s | 7,58 / 48,66 s | 4,6 / 120,8 MiB |
| Cormen/CLRS | 26,08 / 16,39 s | 25,97 / 31,35 s | 13,2 / 273,0 MiB |
| Exercism (escala 5×) | 6,68 / 4,22 s | 6,66 / 8,00 s | 7,8 / 249,6 MiB |

Os 98 casos de benchmark possuem checksums nativo/JVM equivalentes. No Cormen, o nativo
usa 17,2% menos CPU acumulada que a JVM, embora o tempo de parede agregado ainda seja
maior. No corpus externo, 8 das 114 soluções upstream completas compilam; as outras 106
possuem classificação versionada do primeiro bloqueador.

## Estrutura do projeto

| Caminho | Responsabilidade |
| --- | --- |
| [`Makefile`](Makefile) | Build, qualidade, testes, compatibilidade, benchmarks e instalação Linux |
| `crates/clojure-reader` | Reader e parser |
| `crates/clojure-interp` | Interpretador de bootstrap |
| `crates/clojure-analyzer` | Análise, expansão de macros, closures, records e protocolos |
| `crates/clojure-codegen` | Codegen Cranelift e runtime C embutido |
| `crates/clojure-native-cli` | Comandos `read`, `eval`, `run` e `build` |
| `crates/clojure-test-support` | Runner, schema, oracle e relatórios de conformidade |
| [`tests/conformance`](tests/conformance) | Fixtures executáveis de compatibilidade A–E |
| `examples` | Exemplos Clojure e cargas de desempenho |
| `specs` | Escopo, modelo de runtime, planos, riscos e ADRs |
| [`docs`](docs/README.md) | Índice, snapshot auditado, site, guias de uso, visão geral e arquitetura |

## Limitações conhecidas

- Este é um subconjunto de Clojure, não um substituto direto para Clojure/JVM.
- A execução nativa compilada aceita doubles IEEE-754 boxeados e aritmética mista entre
  fixnums e floats. Bignums, ratios e BigDecimal não existem.
- Macros definidas pelo usuário, sequências lazy/infinitas, carregamento dinâmico de
  namespaces e compilação de projetos com múltiplos arquivos não estão disponíveis no
  caminho nativo.
- Os catches ainda são catch-all: hierarquia tipada, múltiplas cláusulas de catch,
  `ex-info` e conversão de falhas fatais do runtime em valores capturáveis permanecem
  incompletos.
- Multimétodos exigem uma função de dispatch explícita e suportam igualdade mais
  `:default`; dispatch por hierarquia com `derive`/`isa?` ainda não existe.
- O runtime nativo inclui streams padrão e em memória, arquivos textuais e binários,
  paths, bytes, primitivas de filesystem, contexto do processo e leitura de dados em
  runtime. O gate completo de I/O continua aberto porque APIs derivadas, opções EDN
  completas e vários contratos de lifecycle/erro ainda estão incompletos.
- A compilação nativa usa o host e invoca um linker C do sistema.
- O GC é single-thread e não móvel. O rooting ainda é eager; uma fase planejada usará
  liveness para posicionar roots nos safepoints de alocação.
- Mapas/sets CHAMP, edit tokens de transients, `disj!`, `pop!` e invalidação depois de
  `persistent!` continuam como trabalho futuro.
- A interoperabilidade Java e bibliotecas do ecossistema JVM estão fora do runtime
  nativo atual.
