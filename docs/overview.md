# Visão geral

[Índice da documentação](README.md) · [Uso](usage.md) ·
[Arquitetura](architecture.md) · [Especificações](../specs/README.md)

> Estado auditado no [`HEAD 424ba20`](https://github.com/EwertonDCSilv/clojure-compiler/commit/424ba20e88fd91a641675e4d9d9bf111c63fc164).
> Detalhes do snapshot: [SNAPSHOT.md](SNAPSHOT.md).

O repositório `clojure-compiler` implementa um compilador nativo experimental de
Clojure em Rust. O executável do projeto, `clojure-native`, lê, interpreta e compila
antecipadamente um subconjunto documentado da linguagem. O binário gerado não usa JVM
nem bytecode `.class` em tempo de execução.

## Caminhos de execução

- `read`: transforma texto em forms com spans e diagnósticos determinísticos.
- `eval` e `run`: executam forms no interpretador de bootstrap.
- `build`: analisa o programa, gera um objeto nativo com Cranelift e o linka ao runtime C
  embutido.

## Subconjunto executável atual

- Literais, strings, listas, keywords, vetores, mapas e sets.
- `defn`, `fn`, `if`, `do`, `let`, `loop/recur` e recursão direta.
- Closures, funções de ordem superior, aridades fixas/múltiplas/variádicas e `apply`.
- Macros de core suportadas: `when`, `when-not`, `if-not`, `cond`, `and`, `or`, `->` e
  `->>`.
- Fixnums tagueados com fast paths verificados para `+`, `-`, `*`, `quot`, `mod`,
  `inc`, `dec`, `<`, `<=`, `>` e `>=`; igualdade estrutural permanece no runtime.
- Vetores persistentes em trie bitmap de 32 vias e mapas/sets híbridos, com promoção de
  representação pequena para HAMT de 32 vias.
- Mapas e sets ordenados em árvore LLRB persistente.
- `defrecord`, `defprotocol` e `extend-type`, inclusive dispatch sobre tipos embutidos.
- Um core compilado com 26 funções, entre elas `map`, `filter`, `reduce`, `range`,
  `into`, `mapv`, `take`, `drop`, `comp`, `concat` e `mapcat`.
- Loader estático de fontes locais multi-arquivo, com `def`/`defn` isolados por
  namespace.
- Primeiro corte HTTP/Pedestal em memória, com request/response, cadeia síncrona de
  interceptors, roteador determinístico, parser HTTP/1.x e serializador HTTP/1.1.

## Otimizações entregues

- Fast paths Cranelift para fixnums retiram a ABI C do caminho inteiro comum.
- Loads e stores diretos substituem `gc_push`, `gc_popn` e `gc_set` no caminho gerado.
- `mapv` e `into` usam vetores transientes estruturais para construção em lote.
- O analyzer reconhece acumuladores de vetor frescos e lineares em `loop`; o primeiro
  subconjunto interprocedural propaga essa linearidade por parâmetros de funções de
  topo e mantém o fallback persistente quando não consegue provar unicidade.
- Vetores literais constantes com elementos imediatos são construídos no primeiro uso
  do site, reutilizados nas avaliações seguintes e marcados como roots permanentes.

A análise interprocedural entregue cobre o padrão de acumulador encadeado da
[`ADR-0010`](../specs/adr/0010-interprocedural-ephemeral-vectors.md). Tuplas de retorno
sem heap, análise geral de escape e rooting por liveness ainda são trabalho futuro.

## Runtime e memória

O caminho nativo representa fixnums como valores tagueados e objetos compostos no heap
gerenciado. O runtime C implementa coleções, strings, impressão, operações lentas e um
GC mark-sweep preciso, não móvel e single-thread.

O codegen mantém roots em uma shadow stack. Loads e stores da pilha de roots são gerados
diretamente; chamadas auxiliares não ficam no caminho quente de cada expressão. O
rooting ainda é eager, e a próxima etapa prevista é usar liveness para gravar roots
somente nos safepoints de alocação.

## Qualidade e compatibilidade

A suíte executável em [`tests/conformance/`](../tests/conformance) cobre os níveis A–E.
Ela possui 461 casos: 243 ativos, 186 falhas esperadas e 32 itens pendentes. D inclui
bibliotecas puras autocontidas; E inclui aplicações nativas integradas de arquivo único
e lacunas executáveis de ecossistema, além de um projeto-alvo de API HTTP Hello World
em Pedestal. A verificação é offline, não depende de JVM e gera relatórios em
`target/conformance/`.

O inventário também cobre o gate proposto de I/O. Já estão ativos output, flush,
redirecionamento, `slurp`/`spit`, `read-string`, streams de string e o primeiro recorte
de paths/filesystem em `cljn.io`; filesystem amplo, binários, EDN completo e vários
contratos de erro/lifecycle permanecem `xfail`. O
contrato está em [`specs/IO_SPEC.md`](../specs/IO_SPEC.md).

As suítes de algoritmos ficam em [`benchmarks/cracking/`](../benchmarks/cracking) e
[`benchmarks/cormen/`](../benchmarks/cormen). A suíte pública adicional em
[`benchmarks/exercism/`](../benchmarks/exercism) contém oito cargas executáveis nos dois
runtimes. A auditoria de 101 práticas, 13 conceitos e 493 arquivos alimenta a
conformidade, não os resultados de desempenho. As três suítes exportam CSV com tempo
de parede, CPU e pico de memória para o nativo e para Clojure/JVM.

No snapshot de referência, Cracking acumula 8,05 s nativos contra 23,02 s na JVM. Cormen
acumula 27,23 s nativos contra 16,95 s de parede na JVM, mas usa 27,09 s de CPU contra
32,08 s da JVM. No Exercism em escala 5×, o nativo soma 7,15 s de parede e 7,12 s de
CPU, contra 4,43 s e 8,44 s na JVM. Os 98 checksums coincidem. A parede nativa do
Cormen caiu 9,4% contra o snapshot anterior; repetições pareadas continuam necessárias
antes de atribuir essa variação ao compilador. Ambiente, repetição e valores por caso
estão nos relatórios das suítes.

Os gates correntes são expostos pelo [`Makefile`](../Makefile):

```bash
make quality        # formato, lints e testes
make coverage       # cobertura global e por arquivo
make compatibility  # matriz executável A–E
make benchmarks     # 98 casos nativos com checksum
make exercism-compatibility  # audita 101 práticas, 13 conceitos e 493 arquivos
```

`make all` executa o conjunto local completo. A comparação com Clojure/JVM é separada
em `make benchmarks-compare`, pois requer Java — os relatórios de referência usam Java
21 — e baixa artefatos fixados com `curl` na primeira execução.

## Limites importantes

O compilador ainda não é uma implementação completa de Clojure. Fixnums e floats
double-boxed já chegam ao caminho compilado, inclusive em aritmética mista; não há
bignums, ratios nem BigDecimal.
Também permanecem fora do caminho nativo: macros definidas pelo usuário, sequências
lazy/infinitas, namespaces dinâmicos, resolução geral de dependências e interop Java.
Fontes locais multi-arquivo podem ser carregadas estaticamente. Exceções
explícitas já são capturáveis, mas catches tipados e tradução de falhas fatais do
runtime ainda não estão disponíveis.
O runtime já possui streams, arquivos, paths, bytes, operações de filesystem e reader
de dados. O gate completo de I/O continua aberto porque parte das APIs derivadas,
opções EDN e contratos de lifecycle/erro ainda está marcada como `xfail`.

Consulte [`specs/README.md`](../specs/README.md) para o estado detalhado e
[`usage.md`](usage.md) para os comandos. O código e os casos de conformidade `active`
são a fonte de verdade quando uma especificação também descreve trabalho futuro.
