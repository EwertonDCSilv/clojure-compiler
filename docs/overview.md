# Visão geral

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
- `defrecord`, `defprotocol` e `extend-type`, inclusive dispatch sobre tipos embutidos.
- Um core compilado com 26 funções, entre elas `map`, `filter`, `reduce`, `range`,
  `into`, `mapv`, `take`, `drop`, `comp`, `concat` e `mapcat`.

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
Ela possui 181 casos: 141 ativos, 9 falhas esperadas e 31 itens pendentes. A verificação
é offline, não depende de JVM e gera relatórios em `target/conformance/`.

As suítes de algoritmos ficam em [`benchmarks/cracking/`](../benchmarks/cracking) e
[`benchmarks/cormen/`](../benchmarks/cormen). Ambas exportam CSV com tempo de parede,
CPU e pico de memória para o nativo e para Clojure/JVM.

## Limites importantes

O compilador ainda não é uma implementação completa de Clojure. A execução nativa
numérica é limitada a fixnums; não há bignums, ratios nem BigDecimal. Literais
floating-point são reconhecidos pelo reader, mas ainda não chegam ao caminho compilado.
Também permanecem fora do caminho nativo: macros definidas pelo usuário, sequências
lazy/infinitas, exceções, namespaces dinâmicos, projetos multi-arquivo e interop Java.

Consulte [`specs/README.md`](../specs/README.md) para o estado detalhado e
[`README.pt-BR.md`](../README.pt-BR.md) para instruções de uso.
