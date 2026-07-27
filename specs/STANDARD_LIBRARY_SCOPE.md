# Escopo da biblioteca padrão

O projeto mantém primitivas próximas da representação nativa e escreve funções
deriváveis no subconjunto Clojure compilável. Esta página distingue a biblioteca que
entra hoje em `clojure-native build` do alvo de compatibilidade.

## Categorias

- **P — Forma/primitiva de compilador:** reconhecida pelo analyzer/codegen.
- **R — Runtime C:** toca representação, GC, coleções ou IO.
- **C — Core em Clojure:** definido em `crates/clojure-native-cli/src/core_compiled.clj`.
- **M — Macro conhecida:** expandida antes da análise.
- **A — Adiada:** planejada, não executável.
- **X — Incompatível:** dependente da JVM.

## Snapshot executável

### Formas, primitivas e macros

| Grupo | Itens atuais |
| --- | --- |
| Controle | `if`, `do`, `let`, `loop`, `recur`, `fn`, `defn`, `throw`, `try`/`catch`/`finally` |
| Aritmética | `+`, `-`, `*`, `quot`, `mod`, `inc`, `dec` |
| Comparação/predicados | `=`, `<`, `<=`, `>`, `>=`, `not`, `nil?`, `empty?` |
| Sequências | `cons`, `first`, `rest`, `count`, `list` |
| Coleções | `get`, `nth`, `assoc`, `dissoc`, `contains?`, `keys`, `vals`, `conj`, `transient`, `persistent!`, `conj!`, `assoc!`, `dissoc!` |
| Strings/IO | `str`, `print`, `println` |
| Funções | closures, aridades múltiplas/variádicas e `apply` |
| Macros conhecidas | `when`, `when-not`, `if-not`, `cond`, `and`, `or`, `->`, `->>` |
| Tipos/dispatch | `defrecord`, `defprotocol`, `extend-type`, `defmulti`, `defmethod` |

Isso não implica todas as aridades e coerções de Clojure/JVM. O contrato exato é dado
pelos casos `active` e `xfail` da suíte de conformidade.

### `clojure.core` compilado

O arquivo de bootstrap define 26 funções:

| Família | Funções |
| --- | --- |
| Numéricas | `zero?`, `pos?`, `neg?`, `even?`, `odd?`, `max`, `min` |
| Transformação | `map`, `filter`, `remove`, `mapv`, `mapcat` |
| Redução/construção | `reduce`, `into`, `count-if` |
| Sequência | `reverse`, `take`, `drop`, `range`, `second`, `last`, `concat` |
| Predicados/HOF | `every?`, `some`, `comp`, `identity` |

Cada função tem na matriz executável cenários normal, limite e alternativo, além de um
caso separado de aridade inválida.

### Namespaces

| Namespace | Estado no caminho compilado | Conformidade |
| --- | --- | --- |
| `clojure.core` | subconjunto acima, pré-carregado em `build` | ativo |
| `clojure.string` | ainda não carregável | pending |
| `clojure.set` | ainda não carregável | pending |
| `clojure.walk` | ainda não carregável | pending |
| `clojure.edn` | ainda não carregável | pending |
| `clojure.test` | ainda não carregável | pending |

As pastas pending tornam o escopo futuro auditável, mas não contam como suporte.

O inventário de I/O amplia essa regra: `clojure.core`, `clojure.edn`, `cljn.io` e
`cljn.process` possuem casos `xfail` normais, de limite e de erro conforme
[IO_SPEC](IO_SPEC.md). Esses casos tornam o contrato auditável, mas não promovem os
namespaces nem as funções a suporte atual.

## Alvo progressivo

O objetivo é mover para Clojure tudo que possa ser expresso eficientemente sobre as
primitivas e manter no runtime apenas operações que dependem da representação:

1. ampliar `clojure.core` com seqs, predicados, composição e coleções;
2. introduzir namespaces e carregamento AOT multi-arquivo;
3. ativar `clojure.string`, `clojure.set`, `clojure.walk`, `clojure.edn` e um
   `clojure.test` mínimo;
4. adicionar lazy seqs, exceções tipadas, metadata e estado;
5. avaliar bibliotecas puras dos níveis D–E.

Funções dependentes da JVM (`proxy`, `gen-class`, `bean` e interop Java) permanecem
fora do runtime nativo.

## Recursos ainda não entregues

- `/` com ratios, `rem`, bignums, BigDecimal e ponto flutuante compilado;
- `ex-info`, hierarquia tipada/múltiplos catches e tradução de falhas fatais do runtime;
- `defmacro` de usuário, syntax-quote executável e `macroexpand`;
- `lazy-seq`, sequências infinitas e chunked seqs;
- atoms, Vars dinâmicas, delays e futures;
- `deftype`, `reify`, `extend-protocol` e hierarquias de multimétodos;
- `disj!`, `pop!`, edit tokens e invalidação completa de transients;
- require dinâmico, classpath/JAR e bibliotecas JVM.
- streams gerais, arquivos, filesystem, `clojure.edn` em runtime, `cljn.io` e
  `cljn.process` (ver [IO_SPEC](IO_SPEC.md)).

Uma função só deve sair desta lista quando houver implementação e caso de conformidade
ativo.
