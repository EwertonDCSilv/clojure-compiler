# STANDARD_LIBRARY_SCOPE.md

Define quais namespaces/funções entram no MVP e **onde** são implementados. Princípio
(start_spec §15/§16): **não** reescrever toda a stdlib em Rust; só primitivas em Rust e o
resto progressivamente em Clojure (bootstrap — ver
[adr/0005-bootstrap-strategy.md](adr/0005-bootstrap-strategy.md)).

## Classificação de implementação

Cada função de `clojure.core` cai em uma categoria:

- **P — Primitiva do compilador**: forma especial ou operação com suporte direto no
  codegen (ex.: `if`, `let*`, `fn*`, `def`, `loop*/recur`, `quote`, `var`, `throw`, `try`).
- **R — Runtime (Rust)**: em `clojure-core-native`. Operações que precisam tocar
  representação/GC/IO ou de performance (ex.: `+`, `conj`, `get`, `=`, `println`, `apply`,
  aritmética, acesso a coleções, `seq`, `first`, `rest`).
- **C — Clojure**: em `clojure-core-clj` (fonte `.clj` embutida, compilada pelo próprio
  compilador). Funções deriváveis das primitivas (ex.: `map`, `filter`, `reduce` [parte],
  `when`, `cond`, `->`, `->>`, `partial`, `comp`, `complement`, `map?`, `every?`).
- **M — Macro**: em Rust (bootstrap) e depois migrada p/ Clojure (ex.: `defn`, `when`,
  `cond`, `and`, `or`, `->`, `if-let`, `when-let`, `doseq`, `dotimes`, `lazy-seq`).
- **A — Adiada** `[FUTURO]`.
- **X — Incompatível** (depende da JVM) → erro (Compat).

## Namespaces avaliados

| Namespace | MVP | Observações |
| --- | :-: | --- |
| `clojure.core` | ✅ (subconjunto) | núcleo; tabela abaixo |
| `clojure.string` | ✅ (subconjunto) | `join split trim upper-case lower-case replace starts-with? …` (sem regex no MVP → `split` por string literal) |
| `clojure.set` | ✅ | `union intersection difference subset? …` (puro Clojure sobre sets) |
| `clojure.walk` | ✅ (parcial) | `walk postwalk prewalk macroexpand-all` |
| `clojure.edn` | ✅ (parcial) | `read-string` de EDN (reusa reader, sem eval) |
| `clojure.data` | `[FUTURO]` | `diff` |
| `clojure.zip` | `[FUTURO]` | |
| `clojure.test` | ✅ (mínimo) | `deftest is are testing run-tests` — necessário p/ próprios testes |

## `clojure.core` — subconjunto MVP (classificado)

Matriz por função (colunas: **MVP** · **Impl** · **Deps** · **Compat**). Lista
representativa do MVP; o conjunto exato é congelado ao entrar na Fase 7 e vira suite de
conformidade.

| Função | MVP | Impl | Deps | Compat |
| --- | :-: | :-: | --- | --- |
| `if do let fn def quote var loop recur throw try` | ✅ | P | — | B |
| `let letfn when when-not if-let when-let cond condp case` | ✅ | M/P | let*/if | B |
| `and or not` | ✅ | M/R | if | B |
| `defn defn- defmacro fn` | ✅ | M | def/fn* | B |
| `+ - * inc dec` | ✅ | R | aritmética i64/f64 (checada) | B (sem BigInt) |
| `/ quot rem mod` | ✅ | R | — | B* (`/` sem ratio → erro; ver LANGUAGE_SCOPE) |
| `= not= == < <= > >= compare` | ✅ | R | eq/ord | B |
| `min max abs` | ✅ | R/C | — | B |
| `bit-and bit-or bit-xor bit-shift-*` | ✅ | R | i64 | B |
| `str pr prn print println pr-str print-str` | ✅ | R/C | IO, print | B |
| `first rest next cons seq empty? count nth` | ✅ | R | ISeq | B |
| `conj into assoc dissoc get get-in assoc-in update update-in` | ✅ | R/C | coleções | B |
| `keys vals contains? find select-keys merge` | ✅ | R/C | maps | B |
| `list vector hash-map hash-set sorted-map?` | ✅ (sorted A) | R | coleções | B |
| `vec set list* hash-set` | ✅ | R/C | — | B |
| `map filter remove keep mapcat` | ✅ | C | lazy-seq, seq | B |
| `reduce reduce-kv reductions` | ✅ | R+C | seq | B |
| `map-indexed take drop take-while drop-while` | ✅ | C | lazy-seq | B |
| `range iterate repeat repeatedly cycle` | ✅ | R/C | lazy-seq | B |
| `partition partition-all interleave interpose` | ✅ | C | seq | B |
| `apply partial comp complement juxt identity constantly` | ✅ | R/C | fn model | B |
| `every? some not-any? not-every?` | ✅ | C | seq | B |
| `sort sort-by group-by frequencies` | ✅ | R/C | compare | B |
| `nil? some? true? false? zero? pos? neg? even? odd?` | ✅ | R/C | — | B |
| `int? string? keyword? symbol? map? vector? seq? coll? fn?` | ✅ | R | type preds | B |
| `name namespace keyword symbol gensym` | ✅ | R | interner | B |
| `atom deref reset! swap! compare-and-set! volatile! vswap! vreset!` | ✅ | R | GC/CAS | B |
| `delay force realized?` | ✅ | R | — | B |
| `meta with-meta vary-meta` | ✅ | R | metadata | B |
| `assert` | ✅ | M | throw | B |
| `doseq dotimes while` | ✅ | M | recur/seq | B |
| `lazy-seq` | ✅ | M/R | LazySeq | B |
| `ex-info ex-data ex-message throw` | ✅ | R | exceções | B |
| `read-string` (EDN) | ✅ | R | reader | B (sem eval) |
| `format` | ✅ (parcial) | R | — | B (subconjunto de specifiers) |
| `re-* regex` | ❌ | X/A | engine regex | flag `regex` `[FUTURO]` |
| `bigint biginteger bigdec rationalize` | ❌ | A | bignum | `[FUTURO]` |
| `future promise deliver pmap` | ❌ | A | threads | `[FUTURO]` |
| `defprotocol defrecord defmulti reify deftype extend*` | ❌ | A | Fase 10 | `[FUTURO]` |
| `proxy gen-class bean` | ❌ | X | JVM | erro (Compat) |
| `eval load-string require`(dinâmico runtime) | parcial | — | `require` só build-time (AOT) | B/`[FUTURO]` |
| `slurp spit` (IO arquivo) | ✅ (parcial) | R | FS nativo | B (API pode diferir) |
| `*in* *out* *err*` streams | ✅ | R | IO | B |

`* B` com asterisco = compatível **exceto** a divergência já catalogada em
[COMPATIBILITY_SPEC.md](COMPATIBILITY_SPEC.md).

## Estratégia de bootstrap progressivo

1. Primitivas **P** existem no codegen desde a Fase 5.
2. Núcleo **R** (aritmética, coleções, seq, print, IO, atom) em `clojure-core-native`
   (Fases 4–7).
3. Macros essenciais **M** primeiro em Rust (`defn`, `let`, `when`, `cond`, `->`, `and`,
   `or`, `lazy-seq`) — Fase 6.
4. Funções **C** escritas em Clojure em `clojure-core-clj`, compiladas pelo próprio
   compilador — Fase 7 em diante. **Meta:** parcela crescente de `clojure.core` em Clojure.
5. Ao final do MVP, `clojure.string`/`set`/`walk` majoritariamente em Clojure.

Isto realiza o objetivo do start_spec: **bootstrap progressivo**, com o dialeto suportado
escrevendo cada vez mais da própria biblioteca — caminho para self-hosting parcial
`[FUTURO]` (ver ADR-0005).
