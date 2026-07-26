# Especificações — `clojure-native`

Implementação nativa de Clojure em Rust: compila código-fonte Clojure para **binários nativos autônomos**, sem JVM em tempo de execução, sem bytecode `.class`, sem GraalVM como solução principal.

> **Status atual do repositório (fato observado, 2026-07-26):** greenfield.
> Conteúdo existente: `start_spec.md` (briefing) e diretórios de config vazios
> `.clj-kondo/`, `.lsp/`. **Não há** código, crates, git ou testes. Toolchain
> local detectado: Java 21 (usável só como *oracle* de teste), clang 20 / LLVM 20;
> **Rust/Cargo ainda não instalados**. Nada a reaproveitar além do briefing.

> **PROGRESSO (2026-07-26).** A implementação já saiu do papel:
> - **Fase 0** ✅ workspace Cargo + git + toolchain; **protótipo #1 (Cranelift→exe)** validado (portão passou).
> - **Fase 1 — Reader** ✅ tokenizer/parser, spans, desugar de reader-macros, diagnósticos `arquivo:linha:coluna`.
> - **Fase 2 — Interpretador de bootstrap** ✅ formas especiais, closures, `recur`, macros base em Rust, primitivas + `core.clj` embutido; roda programas via `clojure-native run`.
> - **Fase 3+5 (corte vertical)** ✅ analyzer + **codegen Cranelift** → **primeiro binário nativo compilado** (`clojure-native build`, sem JVM).
> - **Fase 4 (representação de valor, parcial)** ✅ **valores tagged** no código nativo (int, nil, bool, **string**, **lista**) via runtime C (ABI C); primitivas `+ - * quot mod inc dec = < <= > >= not nil? empty? cons first rest count list str println print`; strings e listas de primeira classe (concat, `count`, igualdade estrutural). Alocação por `malloc` **sem coletor** ainda (GC mark-sweep é a próxima etapa — ver [MEMORY_MODEL.md](MEMORY_MODEL.md#estado-da-implementacao-2026-07-26)).
> - **`loop`/`recur` compilado** ✅ backedge nativo com validação de tail-position (rastreio de divergência `Flow` no codegen). `(conta 1000000 0)` roda sem estourar a pilha — prova que é loop, não recursão.
> - **GC mark-sweep preciso** ✅ (fecha a Fase 4) coletor tracing não-móvel single-thread com **shadow-stack de roots** gerado pelo codegen (`enter/leave/push/popn/set`); sem escanear a pilha nativa. Validado sob `CLJN_GC_STRESS=1` (coleta a cada alocação, saída correta) e reclamação medida: loop de 10M cons → ~6 MB com GC vs. ~470 MB sem. Ver [MEMORY_MODEL.md](MEMORY_MODEL.md#estado-da-implementacao-2026-07-26).
> - **Macros no caminho compilado** ✅ (ADR-0004) pré-passo de expansão no analyzer: `when when-not if-not cond and or -> ->>` funcionam em `build` (expandidos para as formas especiais antes da análise; `and`/`or` com gensym, sem duplo-eval).
> - **Closures / funções de 1ª classe** ✅ (protótipos #2/#3) `fn` com captura léxica (incl. **transitiva/aninhada**), HOF (`my-map`/`my-reduce` em Clojure compilado), fn de topo como valor (`FnRef`), **chamada indireta** (`call_indirect`) com checagem de aridade em runtime. Closures são objetos GC-traçados (`free[]` marcado). Convenção: toda fn recebe `self` (a closure) como 1º arg.
> - **Coleções: vetores, mapas, sets, keywords** ✅ literais `[]`/`{}`/`#{}`, keywords (`:k`, `(:k m)`), imutáveis com semântica de valor correta (array-map/vetor imutável copy-on-write; a versão com structural sharing — vector trie/HAMT — é otimização posterior). Ops: `get nth assoc dissoc conj contains? keys vals count first rest empty? =`. GC rastreia todos; validado sob `CLJN_GC_STRESS=1`.
> - **`clojure.core` compilável (bootstrap, ADR-0005)** ✅ um `core.clj` no subconjunto compilável é pré-carregado em todo `build`: `map filter reduce remove reverse take drop range into mapv every? some comp identity second last zero? pos? neg? even? odd? max min` — **sem** o usuário defini-los. **Primitivas como valor** (`(map inc ...)`, `(reduce + 0 ...)`) via wrapper sintetizado. Validado normal + GC-stress.
> - ~50 testes verdes (e2e de closures/HOF, coleções e stdlib, normal + GC-stress).
>
> Próximo: structural sharing (vector trie/HAMT) p/ perf; `defprotocol`/`defrecord`;
> variádicos/multi-aridade; `eval`/REPL sobre o interpretador.

O planejamento original abaixo permanece a fonte de verdade das decisões. Protótipos
descartáveis são permitidos e devem ser marcados como não-produtivos.

## Como ler estes documentos

Ordem sugerida de leitura:

1. [VISION.md](VISION.md) — problema, proposta de valor, escopo, métricas de sucesso.
2. [LANGUAGE_SCOPE.md](LANGUAGE_SCOPE.md) — matriz precisa do subconjunto da linguagem (MVP / depois / fora).
3. [COMPATIBILITY_SPEC.md](COMPATIBILITY_SPEC.md) — níveis A–E de compatibilidade e política de incompatibilidades.
4. [ARCHITECTURE.md](ARCHITECTURE.md) — crates, fronteiras e fluxo de dados.
5. [COMPILER_PIPELINE.md](COMPILER_PIPELINE.md) — reader → macroexpand → analyzer → IR → codegen.
6. [RUNTIME_SPEC.md](RUNTIME_SPEC.md) — representação de valores, funções, Vars, protocols, seqs.
7. [MEMORY_MODEL.md](MEMORY_MODEL.md) — modelo de GC, roots, ownership no runtime.
8. [STANDARD_LIBRARY_SCOPE.md](STANDARD_LIBRARY_SCOPE.md) — quais namespaces/funções e onde são implementados.
9. [NATIVE_INTEROP.md](NATIVE_INTEROP.md) — FFI C ABI.
10. [TESTING_STRATEGY.md](TESTING_STRATEGY.md) — differential testing contra Clojure/JVM como oracle.
11. [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) — plano incremental verificável, Fases 0–12.
12. [RISK_REGISTER.md](RISK_REGISTER.md) — riscos, probabilidade, impacto, mitigação.

Decisões arquiteturais fundamentais (imutáveis sem novo ADR):

- [adr/0001-code-generation-backend.md](adr/0001-code-generation-backend.md) — **Cranelift** (AOT) + backend-C fallback.
- [adr/0002-memory-management.md](adr/0002-memory-management.md) — **GC tracing precisa mark-sweep** com shadow-stack roots.
- [adr/0003-value-representation.md](adr/0003-value-representation.md) — **enum `Value`** com imediatos + `Gc<T>`.
- [adr/0004-macro-execution.md](adr/0004-macro-execution.md) — **interpretador tree-walking** em tempo de compilação.
- [adr/0005-bootstrap-strategy.md](adr/0005-bootstrap-strategy.md) — primitivas em Rust + `clojure.core` progressivo em Clojure.

Testes de conformidade: [conformance/README.md](conformance/README.md).

## Convenções

- **Marcações de confiança** usadas em todo o texto:
  `[FATO]` observado/verificável · `[JVM]` comportamento da implementação oficial ·
  `[DECISÃO]` escolha proposta neste projeto · `[HIPÓTESE]` a validar ·
  `[RISCO]` · `[FUTURO]` fora do MVP.
- Nome do produto/binário: `clojure-native`.
- Estas specs são **vivas**: cada fase do [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)
  pode refiná-las. Toda mudança de decisão fundamental exige um novo ADR (nunca editar
  um ADR aceito — cria-se um sucessor que o marca como *Superseded*).

## Resumo executivo das decisões

| Área | Decisão MVP | Alvo de longo prazo | ADR |
| --- | --- | --- | --- |
| Backend de codegen | Cranelift (objeto + link via `lld`) | + backend C p/ portabilidade/otimização | 0001 |
| Execução de macros | Interpretador tree-walking em build-time | JIT Cranelift opcional | 0004 |
| Representação de valor | `enum Value` (imediatos) + `Gc<T>` | tagged pointers / NaN-boxing | 0003 |
| Memória | Mark-sweep precisa, não-móvel, shadow-stack roots, single-thread | Geracional/móvel (MMTk) | 0002 |
| Bootstrap | Primitivas Rust + core em Clojure, staged | Self-hosting parcial | 0005 |
| Coleções | array-map + HAMT + bitmapped vector trie | CHAMP + sorted + transients | RUNTIME_SPEC |
| Plataforma inicial | Linux x86_64 (oficial) + Windows x86_64 (1ª classe) | + arm64, macOS | ARCHITECTURE |

O **menor caminho até o primeiro binário nativo** está descrito no fim de
[IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md#menor-caminho-ate-o-primeiro-binario).
