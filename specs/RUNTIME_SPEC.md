# RUNTIME_SPEC.md

Runtime nativo em Rust (`clojure-runtime` + `clojure-value` + `clojure-persistent` +
`clojure-gc`). Linkado estaticamente; exposto ao código gerado por ABI C
(ver [ARCHITECTURE.md](ARCHITECTURE.md#runtime-abi)).

---

## Representação de valores (ADR-0003)

`[DECISÃO MVP]` `Value` é um **enum Rust** com imediatos inline e heap via `Gc<T>`:

```rust
#[derive(Clone, Copy)]  // 16 bytes; Copy p/ imediatos, Gc é ponteiro Copy
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),            // fixnum
    Float(f64),
    Char(char),
    Keyword(KwId),       // interned (índice)
    Symbol(SymId),       // interned (índice)
    Obj(Gc<Obj>),        // tudo o resto no heap gerenciado
}
```

`Obj` (no heap, com cabeçalho de GC) cobre: `Str`, `PersistentList`, `Cons`, `LazySeq`,
`PersistentVector`, `ArrayMap`, `HashMap`, `HashSet`, `Fn` (closure), `Var`, `Atom`,
`Volatile`, `Delay`, `ExceptionInfo`, `Namespace`, e `[FUTURO]` `Record`, `Ratio`,
`BigInt`, `Protocol`, sorted collections, transients.

**Justificativa da escolha (vs. NaN-boxing / tagged pointers):** segurança (sem `unsafe`
de bit-tagging no MVP), pattern-matching ergonômico, integração direta com a GC precisa.
Custo: 16 bytes/valor e boxing de heap. **Migração planejada** para tagged pointers /
NaN-boxing quando perf exigir — por isso todo acesso passa por uma API (`Value::is_*`,
`Value::as_*`, construtores), mantendo a representação **trocável**. Registrado como
`[RISCO]` no [RISK_REGISTER.md](RISK_REGISTER.md) (migração de representação é invasiva).

### Igualdade, hash, identidade
- `=` (`clojure_eq`): igualdade **estrutural** de valor. `nil`/`false` distintos.
  Categorias numéricas: `(= 1 1.0)` → `false` (como `[JVM]`).
- `identical?`: identidade (mesmo imediato ou mesmo `Gc` handle).
- `hash`: consistente com `=` (**invariante testado por property**: `a = b ⇒ hash a =
  hash b`). Algoritmo **próprio** (não replica o `[JVM]`; ver Compat).
- Metadata **não** participa de `=`/`hash` `[JVM]`.

### Metadata
- Anexável a: símbolos, coleções persistentes, Vars, fns `[parcial]`, records `[FUTURO]`.
- Guardada no `Obj` (campo `meta: Option<Gc<Map>>`), copy-on-write (`with-meta`/`vary-meta`
  retornam novo valor com mesma estrutura + nova meta). Imediatos não carregam meta.

### Thread safety
- Imediatos: `Copy`, triviais.
- Coleções persistentes: imutáveis ⇒ compartilháveis entre threads sem lock `[FUTURO
  quando threads]`.
- `Atom`: sincronização por CAS. `Var` dinâmica: binding thread-local.
- No MVP single-thread, o coletor é single-thread; multithreading e GC concorrente são
  `[FUTURO]` (ver [MEMORY_MODEL.md](MEMORY_MODEL.md)).

---

## Coleções persistentes

`[DECISÃO]` Implementações **reais**, nunca `Vec`/`HashMap` mutáveis disfarçados
(start_spec §10/§30).

| Coleção | Algoritmo | MVP |
| --- | --- | :-: |
| PersistentList / Cons | lista ligada + contagem | ✅ |
| PersistentVector | **bitmapped vector trie** (branching 32, tail otimizada) | ✅ |
| ArrayMap | array ordenado por inserção (≤ ~8 entradas) | ✅ |
| HashMap | **HAMT** (32-way) — migrar p/ **CHAMP** depois | ✅ (HAMT) |
| HashSet | sobre HashMap | ✅ |
| PersistentQueue | duas listas/vector | `[FUTURO]` |
| Sorted map/set | red-black tree | `[FUTURO]` |
| Transients | versão mutável-controlada de vector/map | `[FUTURO]` |

Requisitos comuns: igualdade estrutural, hash consistente, metadata, `ISeq`/`seqable`,
iteradores nativos (Rust `Iterator` interno p/ o runtime), structural sharing por
path-copying, e (futuro) transients para construção em massa. `array-map` **promove**
para `hash-map` ao crescer.

Algoritmos analisados (ver start_spec §10): bitmapped vector trie (vector), HAMT/CHAMP
(map/set), red-black (sorted), path copying / structural sharing (todos). CHAMP escolhido
como alvo por melhor iteração/localidade; HAMT primeiro por simplicidade.

---

## Sequences (seqs) e lazy seqs

- `ISeq`: abstração `first`/`rest`/`next`/`cons`. `seq` sobre coleções produz um seq.
- **Lazy seq** (`LazySeq`): thunk `Gc<Fn>` realizado sob demanda, **memoizado** (realiza
  uma vez, guarda o resultado; thread-safe `[FUTURO]` via lock/once).
- Infinitas: `(range)`, `(iterate f x)`, `(repeat x)` etc. suportadas.
- `[FUTURO]` **chunked seqs** para throughput (realizar em blocos de 32).
- **Propagação de erro:** exceção durante realização propaga ao consumidor no ponto de
  força.
- **Retenção de memória / "head holding":** documentar padrão; GC coleta a cauda quando o
  head não é retido. Testes de retenção no [TESTING_STRATEGY.md](TESTING_STRATEGY.md).

Exemplos que são **critérios de teste** (do start_spec §14):
```clojure
(take 10 (map inc (range)))          ; lazy + infinito, sem estourar
(def xs (lazy-seq (cons 1 xs)))      ; auto-referência: 1 1 1 … (retido) — não vazar
(reduce + (take 1000000 (range)))    ; sem stack overflow, memória O(1) na cauda
```

---

## Modelo de funções

Especifica como uma `defn` multi-aridade vira código (start_spec §11):

```clojure
(defn sum
  ([a b] (+ a b))
  ([a b & more] (reduce + (+ a b) more)))
```

`[DECISÃO]`:
- Cada **aridade** vira uma função nativa distinta (`sum__2`, `sum__variadic`).
- Um **Fn object** (`Gc<Fn>`) guarda: ponteiro(s) de código por aridade, aridade mínima,
  flag variádica, e o **environment capturado** (slice de `Value` das variáveis livres).
- **Invocation protocol:** chamada por um trampolim de aridade —
  `cljn_invoke(fn, args_ptr, n)` (ABI C) despacha para a função nativa correta por `n`;
  aridade inválida ⇒ `ArityException`.
- **Chamadas diretas:** quando o alvo (Var de fn) e a aridade são estáticos, o codegen
  emite chamada direta à função nativa (sem trampolim) — otimização obrigatória barata.
- **Chamadas indiretas:** valor-função desconhecido (HOF) usa o trampolim.
- **`apply`:** materializa args e chama o trampolim.
- **`partial`/`comp`:** funções de core que criam closures.
- **`recur`:** compilado como backedge de loop (não é chamada) — recursão em tail sem
  crescer a pilha nativa.
- **Tail position geral (não-`recur`):** **não** há TCO geral no MVP → chamadas em tail
  usam a pilha nativa. Recursão profunda não-`recur` pode estourar; documentado.
  `trampoline` (`[FUTURO]`) cobre recursão mútua.
- **Stack overflow:** o runtime instala guarda de pilha e converte estouro em exceção
  `StackOverflow` da linguagem quando possível (não em crash silencioso). `[HIPÓTESE]` a
  validar por plataforma (protótipo).
- **Native stack vs. runtime stack:** usamos a pilha nativa para chamadas; o runtime
  mantém, além disso, um **stack de frames lógicos** para produzir stack traces
  ao nível de fonte (ver Erros/Exceções) e para o shadow-stack de GC roots.

---

## Vars e namespaces

`[DECISÃO]`:
- **Var**: célula com root binding + metadata + flag `dynamic` + (thread-local) pilha de
  bindings dinâmicos. `def` cria/atualiza a Var no namespace atual.
- **Namespace registry**: mapa global `sym → Namespace`; cada `Namespace` tem mapa
  `sym → Var`, aliases (`:as`), refers, e o conjunto de Vars públicas/privadas.
- **root binding / dynamic binding:** `binding` empurra valores na pilha thread-local de
  Vars `^:dynamic`; `set!` de Var dinâmica altera o topo. `deref` de Var lê topo ou root.
- **Private Vars:** acessíveis só do próprio ns (erro ao referenciar de fora).
- **Loading order:** ordem **topológica** do grafo de `require` (`clojure-loader`);
  dependência circular entre namespaces ⇒ erro diagnóstico (como `[JVM]` proíbe ciclos).
- **AOT / inicialização determinística:** o `__cljn_init` gerado registra namespaces e
  avalia inits de topo em ordem estável; startup determinístico (sem corrida).
- **Metadata de Var** (`:doc`, `:arglists`, `:private`, `:dynamic`, `:macro`, linha/arquivo).

Capacidades adiadas, **sem** fechar a porta na arquitetura (start_spec §12):
- Redefinição de Vars em runtime / `eval` / REPL / hot-reload ⇒ `[FUTURO]`. O modelo de
  Var (célula mutável endereçável) **permite** ligá-los depois via interpretador/JIT; o
  MVP simplesmente não expõe `eval` no binário.

---

## Protocols, records, multimethods `[FUTURO — Fase 10]`

Especificação para quando entrarem (não MVP):
- **Protocol**: conjunto de assinaturas; dispatch por **tipo do 1º arg**. Tabela de
  métodos por tipo (`type → impl`) + cache de dispatch; `extend`/`extend-protocol`/
  `extend-type` populam. Otimização AOT: dispatch direto quando o tipo é estático
  (devirtualization) — `[FUTURO]`.
- **Record** (`defrecord`): tipo nominal com campos posicionais + acesso rápido; também é
  um mapa (implementa protocolos de associação); metadata; igualdade por valor de campos.
- **deftype/reify**: tipos/instâncias sem semântica de mapa; só protocols nativos (sem
  interfaces Java).
- **Multimethod**: `defmulti` com fn de dispatch + tabela `dispatch-val → method`,
  hierarquia (`derive`/`isa?`/`prefer-method`), cache com invalidação ao alterar hierarquia
  ou métodos.
Equilíbrio semântica-dinâmica × perf × AOT documentado no ADR de dispatch `[FUTURO]`.

---

## Estado

- **atom**: `swap!` (retry via CAS), `reset!`, `compare-and-set!`, `deref`. Watches/
  validators `[FUTURO próximo]`.
- **volatile!**: `vswap!`/`vreset!`/`deref`, sem CAS (uso single-thread rápido).
- **delay**: computa uma vez sob `force`/`deref`, memoiza.
- **dynamic Vars/binding**: acima.
- **promise/future/ref/agent/core.async**: `[FUTURO]` (ver LANGUAGE_SCOPE — dependem de
  threads/scheduler/STM).

---

## Erros e exceções

`[DECISÃO]` Hierarquia de exceções **nativa do runtime** (não `java.lang.Throwable`):
- Raiz `Throwable` (runtime) → `Exception` → específicas: `ArithmeticException`,
  `ArityException`, `ClassCastException`(→ `TypeError` nativo), `NullPointer`-equivalente,
  `IndexOutOfBounds`, `IllegalArgument`, `IllegalState`, `StackOverflow`, e
  `ExceptionInfo` (`ex-info`/`ex-data`).
- `throw` lança um valor de exceção; `try/catch/finally` casa `catch` por **tipo do
  runtime** (não classe Java); `finally` sempre executa.
- **Stack traces ao nível de fonte:** o runtime mantém pilha de frames lógicos
  (fn + ns + arquivo:linha) e anexa à exceção — independe de unwinding nativo/DWARF.
- Panics do Rust **não** são o mecanismo de exceção da linguagem: são bugs do runtime;
  capturados na fronteira e reportados como erro interno (start_spec §18/§30).
- Diagnósticos de exceção em runtime imprimem: tipo, mensagem, `ex-data` (se houver),
  e stack trace de fonte (arquivo:linha por frame).
