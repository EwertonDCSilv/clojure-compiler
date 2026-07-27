# LANGUAGE_SCOPE.md

Define **precisamente** o subconjunto suportado. Legenda das colunas:

> As colunas abaixo congelam o **alvo do MVP**, não o estado de cada commit. A fonte de
> verdade executável é [`tests/conformance/`](../tests/conformance): `active` existe,
> `xfail` registra uma lacuna conhecida e `pending` é somente inventário. O snapshot
> legível fica em [README.md](README.md).

- **MVP** — deve funcionar no primeiro release.
- **Depois** — planejado, pós-MVP.
- **Fora (inicial)** — não no roadmap de curto prazo (pode entrar depois).
- Marcações: `[JVM]` = detalhe da implementação oficial; `[DECISÃO]` = escolha nossa.

> Regra geral `[DECISÃO]`: nada aqui é "suportar Clojure" genérico. Só está no MVP o
> que tem semântica definida, teste de conformidade previsto e caminho de implementação
> claro no [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md).

---

## Reader

| Recurso | MVP | Depois | Fora (inicial) | Observações |
| --- | :-: | :-: | :-: | --- |
| Inteiros (`i64`) | ✅ | | | Overflow: política explícita (ver Números abaixo) |
| Floats (`f64`) | ✅ | | | `Double` semantics |
| Ratios (`22/7`) | | ✅ | | Exige aritmética racional + BigInt |
| BigInt (`7N`) | | ✅ | | Depende de crate bignum |
| BigDecimal (`1.5M`) | | ✅ | | idem |
| Strings `"..."` + escapes | ✅ | | | Unicode; escapes padrão |
| Caracteres `\a \newline \uXXXX` | ✅ | | | |
| Símbolos | ✅ | | | com namespace (`foo/bar`) |
| Keywords `:k` `:ns/k` `::k` | ✅ | | | `::k` resolve alias do ns atual |
| Listas `( )` | ✅ | | | |
| Vetores `[ ]` | ✅ | | | |
| Mapas `{ }` | ✅ | | | |
| Sets `#{ }` | ✅ | | | |
| Metadata `^{...}` `^:kw` `^Sym` | ✅ | | | Sem *type hints Java* (ver Compat) |
| Quote `'` | ✅ | | | |
| Syntax-quote `` ` `` | ✅ | | | com resolução de símbolo p/ ns |
| Unquote `~` / splicing `~@` | ✅ | | | |
| Reader conditionals `#?` `#?@` | | ✅ | | precisa de `:features`; MVP usa feature única `:cljn` |
| Tagged literals `#inst` `#uuid` | | ✅ | | infra de data readers pós-MVP |
| Tagged literals custom `#foo/bar` | | ✅ | | |
| Anon fn `#(...)` `%` `%1` `%&` | ✅ | | | desugar p/ `fn` |
| Regex `#"..."` | | ✅ | | precisa de engine regex (crate) |
| Discard `#_` | ✅ | | | |
| Namespaced map `#:ns{...}` | | ✅ | | |
| Deref `@` | ✅ | | | desugar p/ `(deref x)` |
| Var-quote `#'x` | ✅ | | | necessário para macros/Vars |
| Comment `;` e `#!` shebang | ✅ | | | shebang p/ scripts |

---

## Formas especiais

| Forma | MVP | Depois | Fora (inicial) | Observações |
| --- | :-: | :-: | :-: | --- |
| `if` | ✅ | | | |
| `do` | ✅ | | | |
| `let*` / `let` | ✅ | | | `let` = macro sobre `let*` |
| `letfn` | ✅ | | | funções mutuamente recursivas |
| `fn*` / `fn` | ✅ | | | closures, multi-aridade, variádica |
| `def` | ✅ | | | cria/atualiza Var |
| `quote` | ✅ | | | |
| `var` | ✅ | | | |
| `set!` (de Var dinâmica/campo) | | ✅ | | MVP só para `set!` de Var dinâmica em `binding` |
| `loop*`/`recur` | ✅ | | | recur em tail-position de fn/loop |
| `throw` | ✅ | | | lança valor de exceção nativo |
| `try`/`catch`/`finally` | ✅ | | | catch por tipo do runtime, não classe Java |
| `new` / `.` / `..` interop | | | ✅ | sem equivalente nativo → erro claro (Compat) |
| `monitor-enter`/`monitor-exit` | | | ✅ | sem modelo de lock de objeto no MVP |
| `case*` | ✅ | | | `case` como forma otimizada (const dispatch) |
| `deftype*`/`reify*` | | ✅ | | base p/ protocols/records |

---

## Funções

| Recurso | MVP | Depois | Fora | Observações |
| --- | :-: | :-: | :-: | --- |
| Funções de 1ª classe / closures | ✅ | | | |
| Múltiplas aridades | ✅ | | | dispatch por aridade em runtime + direto quando estático |
| Variádicas (`& rest`) | ✅ | | | |
| Destructuring (seq e map) | ✅ | | | `:keys :as :or` e vetorial `[a b & r]` |
| `apply` | ✅ | | | |
| `partial`, `comp`, `complement` | ✅ | | | em `clojure.core` (Clojure) |
| Higher-order (`map`/`filter`/…) | ✅ | | | ver STANDARD_LIBRARY_SCOPE |
| Tail recursion via `recur` | ✅ | | | garantida; sem TCO geral (ver RUNTIME_SPEC) |
| `trampoline` | | ✅ | | p/ recursão mútua em tail |

---

## Namespaces

| Recurso | MVP | Depois | Fora | Observações |
| --- | :-: | :-: | :-: | --- |
| `ns` (forma básica) | ✅ | | | `:require`, `:refer`, `:as` |
| `require` / alias `:as` | ✅ | | | |
| `refer` / `:refer [..]` / `:refer :all` | ✅ | | | `:all` suportado mas desencorajado |
| `use` | | ✅ | | tratado como `require :refer :all` |
| Resolução de símbolos | ✅ | | | ver COMPILER_PIPELINE |
| Vars privadas (`^:private`/`defn-`) | ✅ | | | |
| Carregamento de módulos multi-arquivo | ✅ | | | grafo de dependências (Fase 9) |
| Compilação separada / incremental | | ✅ | | cache por unidade |
| `in-ns` dinâmico | | | ✅ | fora do modelo AOT do MVP |

---

## Macros

| Recurso | MVP | Depois | Fora | Observações |
| --- | :-: | :-: | :-: | --- |
| `defmacro` | ✅ | | | executadas em build-time pelo interpretador (ADR-0004) |
| Expansão de macros | ✅ | | | |
| `macroexpand` / `-1` / `macroexpand-all` | ✅ | | `-all` em `clojure.walk` | |
| Syntax-quote / gensym (`x#`) | ✅ | | | |
| Macros do usuário entre namespaces | ✅ | | | macro deve estar compilada antes do uso |
| `&form` / `&env` | parcial | ✅ | | `&form` sim; `&env` mínimo (locais visíveis) no MVP |
| Macros que fazem I/O em build-time | | | ✅ | sandbox restringe (ver Segurança / ADR-0004) |

---

## Estruturas de dados

| Recurso | MVP | Depois | Fora | Observações |
| --- | :-: | :-: | :-: | --- |
| Persistent list / cons | ✅ | | | |
| Persistent vector (bitmapped trie) | ✅ | | | 32-way (ver RUNTIME_SPEC) |
| Persistent array-map (pequeno) | ✅ | | | ordem de inserção; promove p/ hash-map |
| Persistent hash-map (HAMT) | ✅ | | | migrar p/ CHAMP depois |
| Persistent hash-set | ✅ | | | sobre hash-map |
| `seq` / `ISeq` | ✅ | | | abstração central |
| Lazy seq | ✅ | | | realização preguiçosa + memoização |
| Chunked seq | | ✅ | | otimização de throughput |
| Queue (`PersistentQueue`) | | ✅ | | |
| Sorted map / set (red-black) | | ✅ | | |
| Transients | | ✅ | | otimização de construção em massa |

---

## Abstrações

| Recurso | MVP | Depois | Fora | Observações |
| --- | :-: | :-: | :-: | --- |
| `defprotocol` / `extend`/`extend-protocol` | | ✅ | | Fase 10 |
| `defrecord` | | ✅ | | Fase 10 |
| `deftype` | | ✅ | | sem interfaces Java |
| `reify` | | ✅ | | |
| Metadata em valores | ✅ | | | maps/vectors/symbols/collections |
| Multimethods (`defmulti`/`defmethod`) | | ✅ | | hierarquias `derive`/`isa?` depois |
| Hierarquias (`derive`,`isa?`,`parents`) | | ✅ | | |
| Interfaces Java equivalentes | | | ✅ | não existem; usar protocols |

---

## Estado e concorrência

| Recurso | MVP | Depois | Fora | Observações |
| --- | :-: | :-: | :-: | --- |
| `atom` (`swap!`,`reset!`,`compare-and-set!`) | ✅ | | | thread-safe via CAS |
| `deref`/`@` | ✅ | | | |
| `volatile!` | ✅ | | | |
| `delay` / `force` | ✅ | | | |
| `binding` / dynamic Vars | ✅ | | | thread-local; base de macros de I/O |
| `promise` / `deliver` | | ✅ | | precisa de threads/bloqueio |
| `future` | | ✅ | | precisa de pool de threads |
| Thread-local bindings além de `binding` | | ✅ | | |
| `locking` | | | ✅ | depende de `monitor-*` |
| `ref` / STM / `dosync` | | | ✅ | complexidade alta; adiado deliberadamente |
| `agent` | | | ✅ | |
| `core.async` / channels | | | ✅ | requer scheduler/go-blocks; muito pós-MVP |

As marcas nesta tabela descrevem o escopo-alvo original, não o snapshot executável:
Vars dinâmicas e `binding` ainda são bloqueantes para o gate de I/O e permanecem
`xfail`. O contrato de implementação, streams, arquivos e reader em runtime está em
[IO_SPEC](IO_SPEC.md).

---

## Números — política explícita `[DECISÃO]`

MVP:

- Inteiro literal → `i64` ("fixnum"). **Sem** promoção automática para `BigInt`.
- Overflow: operações `+ - *` **checam overflow** e lançam `ArithmeticException`
  nativo (equivalente ao `+'` seria o não-checado; MVP mapeia `+` para checado, como
  `[JVM]` faz por padrão desde Clojure 1.3). `unchecked-*` fica `[FUTURO]`.
- Ponto flutuante → `f64`, semântica IEEE-754.
- `/` sobre inteiros sem divisão exata → **erro** ou `f64`? `[DECISÃO]`: enquanto não
  houver `Ratio`, `(/ 1 2)` **lança** erro "ratios não suportados no MVP" em vez de
  retornar float silenciosamente (evita divergência semântica escondida). Documentado
  como incompatibilidade conhecida (COMPATIBILITY_SPEC).
- `Ratio`, `BigInt`, `BigDecimal`: `[FUTURO]`, habilitam `/`, `quot`/`rem` exatos e
  literais `N`/`M`.

---

## `nil`, verdade e igualdade `[JVM→DECISÃO]`

- `nil` e `false` são os únicos valores falsos; todo o resto é verdadeiro.
- `nil` ≠ `false` em `=` (`(= nil false)` → `false`).
- `=` é igualdade de valor estrutural; `identical?` é identidade de referência.
- Igualdade numérica entre categorias segue `[JVM]` no que estiver no escopo (só `i64`
  e `f64` no MVP): `(= 1 1.0)` → `false` (como Clojure). `==` faz comparação numérica.

---

## Resumo do que **não** está no MVP (para evitar ambiguidade)

Interop Java (todas as formas), `proxy`, `gen-class`, regex, ratios/bignum,
reader conditionals multi-plataforma, tagged literals, multimethods,
transients, chunked seqs, STM/agents/core.async, `future`/`promise`,
`eval`/REPL/hot-reload, `monitor-*`/`locking`.

Cada item acima que alguém tente usar deve produzir **erro de compilação diagnosticável**
(arquivo:linha:coluna + sugestão), nunca comportamento silencioso divergente — ver
política em [COMPATIBILITY_SPEC.md](COMPATIBILITY_SPEC.md).
