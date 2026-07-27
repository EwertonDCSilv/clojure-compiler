# Pipeline do compilador

O caminho executável atual é:

```text
Reader → expansão conhecida → Analyzer → Codegen Cranelift → objeto → link com runtime C
```

HIR/LIR próprios, loader multi-arquivo e um macroexpander completo continuam sendo
evoluções planejadas, não etapas presentes no workspace.

## 1. Reader

O `clojure-reader` recebe UTF-8 e produz forms com spans.

- Espaços, vírgulas, comentários e shebang são tratados como trivia.
- Há literais de nil, boolean, inteiro, float, string, símbolo e keyword.
- Listas, vetores, maps e sets podem ser aninhados.
- Reader macros cobrem quote, syntax-quote, unquote, deref, var-quote, função anônima,
  metadata e discard.
- Delimitadores, escapes e tokens inválidos produzem diagnósticos determinísticos.

O reconhecimento pelo reader não implica execução nativa: floats, por exemplo, são
lidos, mas ainda não fazem parte da representação numérica compilada.

## 2. Expansão

O analyzer executa um pré-passo para o conjunto de macros de core conhecido:

```text
when  when-not  if-not  cond  and  or  ->  ->>
```

As expansões viram formas especiais antes da análise. `and` e `or` preservam
short-circuit e evitam avaliação dupla. `defmacro` de usuário, ambiente `&form`/`&env`,
macroexpansão entre namespaces e execução arbitrária de macros no interpretador ainda
não estão no caminho compilado.

## 3. Analyzer

O `clojure-analyzer`:

- resolve slots locais e capturas léxicas, inclusive capturas transitivas;
- valida aridades fixas, múltiplas e variádicas;
- valida `recur` em tail position e sua aridade;
- reconhece primitivas, chamadas diretas e indiretas;
- analisa literais de coleção;
- sintetiza closures e wrappers de primitivas como valores;
- registra records, protocolos e implementações de `extend-type`;
- rejeita forms fora do subconjunto com diagnóstico.

A saída atual é `Program`/`Expr` analisado consumido diretamente pelo codegen. Uma
camada HIR/LIR ANF permanece desejável quando otimizações de liveness, DCE, inlining ou
especialização exigirem uma representação intermediária estável.

## 4. Codegen

O `clojure-codegen` traduz cada função para Cranelift IR e emite um objeto nativo.

- Convenção de função: `(self, argc, argv)`.
- `loop/recur` vira branch para o bloco alvo.
- Closures carregam ponteiro de código, aridade e capturas.
- Chamadas HOF usam `call_indirect`.
- `+`, `-`, `*`, `quot`, `mod`, `inc`, `dec` e comparações inteiras possuem fast paths
  com guard de tag e checagem de overflow.
- Casos inválidos usam slow paths ABI C.
- Loads/stores de roots são emitidos diretamente na shadow stack.

`CodegenOptions` aceita `none`, `speed` e `speed-and-size`. A CLI mantém `none` como
padrão porque o gate Cormen registrou regressão para `speed` no IR atual. O nível
selecionado é parte da metodologia dos benchmarks.

## 5. Runtime e link

Os módulos em `crates/clojure-codegen/runtime/` são amalgamados, na ordem declarada pelo
codegen, e compilados como uma única unidade pelo compilador definido em `CC` ou por
`cc`. `crates/clojure-codegen/runtime.c` oferece a mesma unidade para ferramentas C
diretas. A CLI liga o objeto C ao objeto Cranelift e produz um executável do host.

O runtime implementa:

- ABI de valores e chamadas;
- GC mark-sweep com shadow stack;
- strings e impressão;
- listas, vetores, maps, sets e records;
- closures, `apply` e dispatch de protocolos;
- operações de coleção e slow paths numéricos.

O executável final não contém JVM nem bytecode `.class`.

## 6. Core compilado

Antes do código do usuário, o CLI carrega
`crates/clojure-native-cli/src/core_compiled.clj`. As 26 funções desse arquivo usam
somente o subconjunto que o próprio compilador aceita, exercitando o bootstrap
progressivo.

## 7. Diagnósticos

Reader e analyzer usam spans para reportar arquivo, linha e coluna. Erros de build
incluem forms malformadas, símbolo/primitiva indisponível, `recur` inválido e falha do
linker.

O runtime nativo atualmente encerra o programa com mensagem para erro de tipo, aridade,
overflow, divisão ou índice. Valores lançados explicitamente já atravessam
`try`/`catch`/`finally`; catches tipados, tradução desses erros fatais e stack traces de
fonte permanecem futuros. Consulte [RUNTIME_SPEC.md](RUNTIME_SPEC.md#erros).

## Evolução planejada

1. introduzir liveness e rooting somente nos safepoints;
2. criar uma IR explícita quando isso reduzir complexidade no codegen;
3. executar macros de usuário de forma determinística no bootstrap;
4. adicionar loader e grafo de namespaces multi-arquivo;
5. ampliar otimizações próprias antes de depender do Cranelift para remover trabalho
   desnecessário.
