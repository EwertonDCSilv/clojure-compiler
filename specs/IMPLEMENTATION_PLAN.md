# IMPLEMENTATION_PLAN.md

Plano incremental e **verificável**. Cada tarefa declara: **objetivo · crates · deps ·
implementação esperada · testes · riscos · critério de aceite · resultado observável**
(start_spec §26). Nada de roadmap genérico.

> **Leitura em 2026-07-26:** este é o plano histórico de fases, não uma lista do que
> está ausente. O corte vertical já entrega reader, interpretador, analyzer, Cranelift,
> runtime C com GC, coleções persistentes, core compilado, closures, aridades,
> records/protocols e a suíte A–E. Consulte [README.md](README.md) para o snapshot
> executável. Nomes de crates ainda não existentes abaixo representam a arquitetura
> originalmente planejada, não dependências atuais.

Regras: não implementar nada de produção antes das specs aprovadas; não começar por
otimização; primeiro binário o quanto antes; protótipos descartáveis marcados como
não-produtivos.

Legenda de dependência: "→ Fase N" = depende da fase N.

---

## Fase 0 — Pesquisa e Especificação (**entregue no corte inicial**)

- **0.1 Specs** — *Objetivo:* estas specs. *Resultado:* `specs/*` aprovado. **Feito** (este documento).
- **0.2 Setup do workspace** — *Objetivo:* instalar Rust/Cargo, criar `git init`, workspace Cargo vazio com crates-esqueleto (só `lib.rs` + `//! doc`), CI (Linux+Windows) rodando `cargo build`/`test`/`clippy`/`fmt`. *Crates:* todos (stubs). *Deps:* —. *Testes:* CI verde vazio. *Risco:* toolchain Windows (MSVC/lld). *Aceite:* `cargo test` verde nas 2 plataformas. *Resultado:* workspace compila.
- **0.3 Protótipos descartáveis** (pasta `prototypes/`, **não-produtivo**, um por pergunta — start_spec §27):
  1. gerar executável mínimo com **Cranelift** (emitir `main` que retorna 0) → valida ADR-0001;
  2. chamada **indireta** de função (ponteiro) via Cranelift;
  3. **closure** (função + ambiente capturado) chamada indiretamente;
  4. **value representation** (enum `Value`) — tamanho, pattern-match, custo;
  5. **GC** alocação + mark-sweep básico;
  6. **shadow-stack roots** capturando vivos (teste de ciclo);
  7. **persistent vector** (bitmapped trie) mínimo + property;
  8. **persistent hash map** (HAMT) mínimo + property;
  9. **macro** simples expandida por interpretador tree-walking;
  10. **stack trace ao nível de fonte** a partir de frames lógicos;
  11. **linking no Windows** (lld-link) de objeto Cranelift → `.exe`;
  12. **FFI** chamando `strlen`/`cos` de libc.
  *Aceite:* cada protótipo responde sua pergunta com um relatório curto em `prototypes/<n>/NOTES.md`; decisões confirmadas ou ADR revisado. *Resultado:* riscos arquiteturais retirados antes da implementação real.

> Portão: só avançar para Fase 1 com protótipos #1, #4, #5, #6, #11 bem-sucedidos
> (backend, valor, GC, roots, Windows). Se algum falhar, reabrir o ADR correspondente.

---

## Fase 1 — Reader e formulários

- **1.1 Tipos base** — *Crates:* `clojure-span`, `clojure-syntax`. *Impl:* `Span`, `Form`, interner de `Symbol`/`Keyword`, metadata de leitura. *Testes:* unit + interning. *Aceite:* igualdade/identidade de símbolos O(1).
- **1.2 Tokenizer + Parser** — *Crates:* `clojure-reader`, `clojure-diagnostics`. *Deps:* 1.1. *Impl:* tokens, parser recursivo, desugar de macros de leitura (`' @ #' #( ^ #_` e syntax-quote), spans em todo nó, recuperação de erro. *Testes:* golden (`insta`) + **fuzz** (`cargo-fuzz`). *Risco:* syntax-quote correto (gensym/resolução). *Aceite:* `clojure-native read examples/basic.clj` dá dump determinístico; erros com arquivo:linha:coluna.
- **Resultado observável (Fase 1):** comando `read` funcionando; nível A em
  [`tests/conformance/`](../tests/conformance) verde para os casos ativos.

---

## Fase 2 — Interpretador de bootstrap (evaluator)

- **2.1 Núcleo do interpretador** — *Crates:* `clojure-interp`, `clojure-value`,
  `clojure-runtime` (nome planejado). *Deps:* Fase 1. *Impl:* avaliar literais, locais,
  `if`, `do`, `let*`, `fn*`, chamadas de função, closures, operações primitivas
  (aritmética, comparação, coleções básicas), Vars/`def`. *Testes:* differential contra
  oracle quando disponível. *Risco:* semântica de truthiness/igualdade. *Aceite:* casos
  correspondentes do nível B em [`tests/conformance/`](../tests/conformance).
- **Papel:** este interpretador existe para **bootstrap e macro expansion** (ADR-0004), não é o produto. Pode usar `Rc` internamente (ADR-0002) para andar rápido.
- **Resultado observável:** `clojure-native eval '(let [a 1 b 2] (+ a b))'` → `3`.

---

## Fase 3 — Analisador e IR

- **3.1 Analyzer** — *Crates:* `clojure-analyzer`. *Deps:* Fase 2. *Impl:* resolução de símbolos/Vars/namespaces, scopes/locais em slots, capturas, aridades, validação de `recur`/tail-position, checagem de formas especiais, desugar de destructuring, detecção de construções fora de escopo (interop → erro). *Testes:* golden de AST + erros. *Aceite:* AST correta p/ o subconjunto; erros diagnósticos p/ interop e `recur` inválido.
- **3.2 HIR/LIR + lowering** — *Crates:* `clojure-ir`. *Deps:* 3.1. *Impl:* AST→HIR→LIR (ANF), `recur`→loop, constant folding, DCE trivial, resolução de chamada direta. *Testes:* golden de IR; property (semântica preservada via interp sobre IR). *Aceite:* IR determinística e semanticamente equivalente ao interp.

---

## Fase 4 — Runtime mínimo

- **4.1 Valores + coleções essenciais** — *Crates:* `clojure-value`, `clojure-persistent`. *Impl:* `Value` final, `Str`, `PersistentList/Cons`, `PersistentVector` (trie), `ArrayMap`, `HashMap` (HAMT), `HashSet`, `=`/`hash`/`compare`, `seq`/`first`/`rest`, metadata. *Testes:* **property** (leis + vs. modelo) + fuzz. *Aceite:* `conformance/collections` (subconjunto) verde.
- **4.2 GC** — *Crates:* `clojure-gc`. *Deps:* 4.1. *Impl:* heap, `Gc<T>`, mark-sweep não-móvel, shadow-stack roots, `alloc`/`collect`, safepoints em alocação. *Testes:* stress/ciclos/retenção + **Miri**. *Risco:* precisão de roots (todo vivo alcançável). *Aceite:* `(def xs (lazy-seq (cons 1 xs)))` e `(reduce + (take 1e6 (range)))` sem vazar/estourar (via interp).
- **4.3 Runtime nativo** — *Crates:* `clojure-runtime`, `clojure-core-native`. *Impl:* Vars/namespaces registry, fn objects + trampolim de aridade (ABI C), exceções nativas + `try/catch/finally`, atoms/volatile/delay, dynamic binding, IO (`println`/`pr`/`str`), stack de frames lógicos p/ traces. *Testes:* unit + differential. *Aceite:* interp usa o runtime real (não mais `Rc` ad-hoc) para valores/coleções.
- **4.4 Gate completo de I/O `[FUTURO]`** — *Deps:* exceções capturáveis, Vars
  dinâmicas e valores de runtime de 4.3. *Impl:* seguir os marcos IO-0–IO-5 de
  [IO_SPEC](IO_SPEC.md): handles/buffers/syscalls atrás da ABI C, `cljn.io`,
  `cljn.process`, streams dinâmicos, arquivos/filesystem e readers de runtime.
  *Testes:* matriz isolada com stdin/argv/env, bytes, `work.before/after`, symlinks,
  GC stress e sanitizers. *Riscos:* R23–R28. *Aceite:* todos os casos de I/O ativos,
  zero handles vazados e streaming em blocos. A [ADR-0007](adr/0007-native-io-and-runtime-reader.md)
  registra a fronteira; o item ainda não está entregue.

---

## Fase 5 — Primeiro executável nativo (**marco crítico**)

- **5.1 Codegen** — *Crates:* `clojure-codegen`. *Deps:* Fases 3–4. *Impl:* LIR→Cranelift IR→objeto; chamadas ao runtime via ABI C; fn objects; `__cljn_init` de inicialização determinística. *Testes:* e2e. *Risco:* ABI/rooting no código gerado. *Aceite:* compila um subconjunto (literais, `if/do/let/fn/def/loop/recur`, chamadas, print).
- **5.2 Linker + runtime estático** — *Crates:* `clojure-linker`, `clojure-native-cli`. *Impl:* linkar objetos + runtime + GC estático; `clojure-native build src/main.clj --output prog`. *Risco:* Windows (lld-link/MSVC). *Aceite:* build limpo em Linux **e** Windows.
- **Programa-alvo:**
  ```clojure
  (ns hello.core)
  (defn -main [] (println "Hello from native Clojure"))
  (-main)
  ```
- **Resultado observável (Fase 5):** binário autônomo roda **sem JVM**, imprime a linha, sai 0; verificado com `ldd`/depende-só-de-libc; testes de build limpo em Linux e Windows. *Este é o alvo do "menor caminho" (abaixo).*

---

## Fase 6 — Macros

- *Crates:* `clojure-macroexpander`, `clojure-interp`, `clojure-core-native` (macros base). *Deps:* Fases 2–5. *Impl:* `defmacro`, ambiente `&form`/`&env` (parcial), syntax-quote/gensym, `macroexpand`/`-1`, ordem de carga, cache, sandbox de I/O em build-time. Macros essenciais primeiro em Rust (`defn`,`let`,`when`,`cond`,`and`,`or`,`->`,`->>`,`lazy-seq`,`doseq`). *Testes:* `conformance/macros` + differential de expansão. *Risco:* determinismo/isolamento (ADR-0004). *Aceite:* macro do usuário compila e roda no binário.

---

## Fase 7 — `clojure.core` mínimo

- *Crates:* `clojure-core-clj` (Clojure) + `clojure-core-native` (primitivas). *Deps:* Fase 6. *Impl:* congelar o subconjunto de [STANDARD_LIBRARY_SCOPE.md](STANDARD_LIBRARY_SCOPE.md); escrever em Clojure o que for derivável (`map/filter/reduce/take/drop/range/every?/sort/...`). *Testes:* conformidade `sequences`/`collections`/`functions`. *Aceite:* programas úteis (CLI de exemplo) compilam e rodam; 100% dos casos declarados passam.

---

## Fase 8 — Estruturas persistentes completas

- *Crates:* `clojure-persistent`. *Impl:* vector trie completo (subvec, transients), HAMT→**CHAMP**, hash-set sobre CHAMP, **transients**, structural sharing verificado, sorted (red-black) `[opcional]`. *Testes:* property/fuzz mais duros + bench (Criterion). *Aceite:* paridade de resultados com oracle em `collections`; perf medida (não regressão).

---

## Fase 9 — Namespaces e dependências (multi-arquivo)

- *Crates:* `clojure-loader`, `clojure-project`, `clojure-analyzer`. *Impl:* múltiplos arquivos, grafo de `require`, ordem topológica, aliases, detecção de ciclos (erro), cache de compilação, manifesto de projeto (ver CLI abaixo). *Testes:* `conformance/namespaces` + projetos multi-módulo. *Aceite:* projeto com vários `ns` compila em ordem correta; ciclo ⇒ erro claro.

---

## Fase 10 — Protocols e records (**subconjunto entregue**)

- *Entregue:* `defprotocol`, `defrecord` e `extend-type`, dispatch pelo tipo do primeiro
  argumento, suporte a records e builtins e casos de conformidade/GC stress.
- *Restante:* `extend-protocol`, impls inline, `deftype`, `reify`,
  `defmulti`/`defmethod`, hierarquias, cache e devirtualização.
- *Aceite final:* protocols, records e multimethods equivalentes ao oracle no
  subconjunto declarado.

---

## Fase 11 — FFI nativa `[FUTURO]`

- *Crates:* `clojure-ffi`. *Impl:* [NATIVE_INTEROP.md](NATIVE_INTEROP.md) — `load-library`, `defcfn`, tipos, marshalling, callbacks, registro Rust→Clojure. *Testes:* integração C + Miri. *Aceite:* chamar `strlen`/lib C real de Clojure; expor fn Rust a Clojure.

---

## Fase 12 — Conformidade e distribuição

- *Impl:* suíte de conformidade completa; builds cross-platform e artefatos de release; documentação de usuário; distribuição de pacotes/deps (source deps — ver abaixo). *Testes:* matriz CI completa; e2e de build limpo. *Aceite:* releases assinados p/ Linux+Windows; `COMPAT_REPORT` gerado por `check`.

---

## CLI e build (planejado desde já; MVP implementa um subconjunto)

Comandos (start_spec §19): `new check build run test fmt repl clean`. **MVP:** `build`,
`run`, `check`, `read`, `eval` (dev), `test`. `repl`/`fmt`/`new` `[FUTURO]`.

Manifesto de projeto (formato próprio — **não** copiar `deps.edn`/Cargo/Lein; TOML por
simplicidade de tooling):
```toml
[project]
name = "example"
version = "0.1.0"
main = "example.core"

[build]
target = "native"
optimization = 1        # 0 debug, 1 padrão, 2 release
features = []           # ex.: ["regex","bigint"]  (desligadas no MVP)
```
Inclui: resolução de fontes, cache de compilação, diretório de build, profiles
debug/release, feature flags, lockfile p/ reprodutibilidade, compilação incremental
`[FUTURO]`.

## Dependências Clojure (distribuição — start_spec §20) `[FUTURO]`

MVP: **source dependencies** (Git/caminho) de bibliotecas **Clojure puras**. Política por
tipo de lib: puro ⇒ compila; Java interop ⇒ erro (Compat); usa macros ⇒ compila se as
macros forem puras; recursos/gen-class ⇒ não suportado. Repositório Maven/binários
pré-compilados ⇒ `[FUTURO]`.

## REPL/`eval` (start_spec §21)

AOT vive sem REPL no MVP. Um **interpretador** já existe (bootstrap/macros) e é o caminho
natural para `eval`/REPL/hot-reload `[FUTURO]`; JIT (Cranelift) só se a performance de
`eval` exigir. Não comprometer o MVP com JIT completo.

---

## <a id="criterios-de-aceite-do-mvp"></a>Critérios de aceite do MVP (start_spec §29)

1. Compila Clojure → executável nativo. 2. Roda sem JVM. 3. Sem JVM embutida.
4. Reader próprio. 5. Analisador próprio. 6. Runtime nativo. 7. Funções e closures.
8. `if do let fn def loop recur`. 9. Vetores/listas/mapas/sets persistentes básicos.
10. Macros do usuário (no escopo documentado). 11. Namespaces básicos.
12. Subconjunto documentado de `clojure.core`. 13. Build em ambiente limpo.
14. Binário roda em **Linux e Windows**. 15. Testes diferenciais passam no subconjunto.
16. Erros com arquivo:linha:coluna. 17. Runtime não depende de Java.
18. Incompatibilidades conhecidas documentadas (`COMPAT_REPORT` + COMPATIBILITY_SPEC).

Mapeamento: 1–3 → Fase 5/12; 4 → Fase 1; 5 → Fase 3; 6 → Fase 4; 7–8 → Fases 3–5;
9 → Fases 4/8; 10 → Fase 6; 11 → Fase 9; 12 → Fase 7; 13–14 → Fases 5/12; 15 → contínuo;
16 → Fases 1/3; 17 → Fase 4; 18 → contínuo (Compat).

---

## <a id="menor-caminho-ate-o-primeiro-binario"></a>Menor caminho até o primeiro binário nativo

Sequência mínima (ignora tudo que não bloqueia o `hello.core`):

1. **0.2** workspace + CI. **0.3** protótipos #1 (Cranelift→exe), #4 (Value), #5 (GC),
   #6 (roots), #11 (Windows link) — só o suficiente para confiar no backend/valor/GC.
2. **1.1/1.2** reader mínimo (o bastante para ler `hello.core`).
3. **2.1** interpretador mínimo (necessário para macros; e `defn`/`println` no bootstrap).
4. **3.1/3.2** analyzer + IR para: `ns`, `def`, `fn*`, `do`, chamada, literais string.
5. **4.1/4.2/4.3** runtime mínimo: `Str`, fn object, `println` (via runtime IO), GC básico,
   registry de Vars/namespace, `__cljn_init`.
6. **5.1/5.2** codegen Cranelift + link estático (Linux e Windows).

Alvo compilável do caminho mínimo:
```clojure
(ns hello.core)
(defn -main [] (println "Hello from native Clojure"))
(-main)
```
**Marco:** `clojure-native build src/hello.clj --output hello && ./hello` imprime a linha,
sem JVM, em Linux e Windows. Tudo além disso (macros ricas, coleções completas, protocols,
FFI) vem **depois** desse binário existir.
