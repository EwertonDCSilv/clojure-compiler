# ARCHITECTURE.md

Arquitetura modular em Rust (workspace Cargo). Cada crate tem responsabilidade única,
API pública mínima e dependências direcionadas (DAG, **sem ciclos**).

## Fluxo de dados (fronteiras do pipeline)

```text
Source (.clj, UTF-8)
  │  clojure-reader
  ▼
Forms + source spans            (dados de leitura: listas/vetores/símbolos/…)
  │  clojure-macroexpander  (usa clojure-interp p/ rodar macros em build-time)
  ▼
Expanded Forms
  │  clojure-analyzer
  ▼
Analyzed AST  (scopes, Vars resolvidas, aridades, recur targets, tail-pos)
  │  lowering (clojure-ir)
  ▼
HIR (high-level IR)
  │  lowering + optimizações obrigatórias
  ▼
LIR (low-level IR, SSA-ish / A-normal form)
  │  clojure-codegen  (Cranelift → objeto)  [ADR-0001]
  ▼
Object files (.o/.obj)
  │  clojure-linker (lld) + runtime estático (clojure-runtime, clojure-core-native)
  ▼
Standalone executable   (sem JVM, sem .class)
```

Fronteira paralela **build-time**: `clojure-interp` interpreta HIR/AST para executar
macros e código de topo necessário à compilação (ADR-0004), sem tocar o backend nativo.

## Crates do workspace

```text
crates/
├── clojure-native-cli/     # binário `clojure-native`: parse de args, orquestra build
├── clojure-span/           # SourceId, Span, posições; usado por todos
├── clojure-diagnostics/    # diagnósticos estruturados, render de erros, códigos Exxxx
├── clojure-reader/         # tokenizer + parser → Form
├── clojure-syntax/         # tipo Form, Symbol/Keyword interning, metadata de leitura
├── clojure-interp/         # interpretador tree-walking (macros + bootstrap) [ADR-0004]
├── clojure-macroexpander/  # loop de expansão de macros; ambiente &form/&env
├── clojure-analyzer/       # resolução, scopes, validação de formas especiais → AST
├── clojure-ir/             # HIR e LIR + passes de lowering/otimização
├── clojure-codegen/        # AST/LIR → Cranelift IR → objeto (backend AOT) [ADR-0001]
├── clojure-linker/         # invoca lld/link.exe, junta runtime, produz executável
├── clojure-value/          # enum Value, imediatos, Gc<T> handles [ADR-0003]
├── clojure-gc/             # coletor mark-sweep, shadow-stack roots [ADR-0002]
├── clojure-persistent/     # coleções persistentes (vector trie, HAMT, list, ...)
├── clojure-runtime/        # runtime nativo: fns, Vars, namespaces, seqs, exceções, atoms
├── clojure-core-native/    # primitivas de clojure.core escritas em Rust
├── clojure-core-clj/       # parte de clojure.core escrita em Clojure (fonte, embutida)
├── clojure-loader/         # grafo de deps entre namespaces, ordem de carga, cache
├── clojure-ffi/            # FFI C ABI (NATIVE_INTEROP)
├── clojure-project/        # manifesto de projeto, profiles, feature flags, lockfile
└── clojure-test-support/   # harness de differential/golden/conformance testing
```

> A estrutura é referência; ajustar conforme a implementação. Regra: **um crate = uma
> responsabilidade**; crates de "dados" (span, syntax, value) não dependem de crates de
> "processo" (analyzer, codegen).

## Camadas e dependências permitidas

Camadas (uma camada só depende das inferiores):

```text
L0  span, diagnostics, syntax, value          (fundamentos; sem deps entre si além de span)
L1  reader, persistent, gc                     (dependem de L0)
L2  interp, runtime                            (dependem de L0,L1)
L3  macroexpander, analyzer                    (dependem de L0,L1,L2)
L4  ir, loader                                 (dependem de L0–L3)
L5  codegen, linker, ffi, project              (dependem de L0–L4)
L6  cli                                         (topo; orquestra tudo)
core-native (L2) e core-clj (fonte) fornecem a stdlib.
```

Proibições explícitas (para impedir ciclos):
- `clojure-value`, `clojure-syntax`, `clojure-span` **não** dependem de nada acima de L0.
- `clojure-reader` **não** depende de `analyzer`/`ir`/`codegen`.
- `clojure-codegen` **não** depende de `reader`/`macroexpander` (recebe LIR pronto).
- `clojure-runtime` **não** depende de `codegen` (o código gerado chama o runtime, não o
  contrário — ligação por C ABI / símbolos exportados; ver Runtime ABI).
- Nada depende de `clojure-native-cli` exceto testes de integração.

## Contrato por crate (responsabilidade · API · invariantes · testes)

Formato resumido; detalhes de tipos em COMPILER_PIPELINE/RUNTIME_SPEC.

- **clojure-span** — `SourceId`, `Pos`, `Span`, `Spanned<T>`. Invariante: todo `Form` e
  nó de AST carrega `Span` rastreável ao byte de origem. Testes: unit.
- **clojure-diagnostics** — `Diagnostic { code, severity, primary_span, labels, notes,
  help }`, renderer estilo rustc (com trecho, `^^^`, sugestão). Invariante: nenhum erro
  de usuário sem span. Testes: golden de mensagens.
- **clojure-syntax** — `Form` (Nil, Bool, Int, Float, Str, Char, Symbol, Keyword, List,
  Vector, Map, Set, Meta), *interner* de `Symbol`/`Keyword` (índices, não `String`).
  Invariante: símbolos iguais ⇒ mesmo id. Testes: interning, igualdade.
- **clojure-reader** — `read_all(src) -> Result<Vec<Form>, Vec<Diagnostic>>`. Invariante:
  posições preservadas; erros recuperáveis (continua após erro quando possível). Testes:
  unit + fuzz (`cargo-fuzz`) + golden.
- **clojure-value** — `Value` (ADR-0003), `Gc<T>`, igualdade/hash. Invariante: `=` ⇒ mesmo
  `hash`. Testes: property (`proptest`) para leis de `Eq`/`Hash`.
- **clojure-persistent** — coleções persistentes + transients `[FUTURO]`. Invariante:
  imutabilidade + structural sharing; nunca `Vec/HashMap` mutável como substituto
  silencioso. Testes: property (equivalência a modelo de referência) + fuzz.
- **clojure-gc** — `Heap`, `Gc<T>`, `RootSet`/shadow-stack, `collect()`. Invariante:
  precisão (todo ponteiro vivo é raiz alcançável). Testes: stress + ciclos + Miri no
  código `unsafe`.
- **clojure-interp** — avalia AST/HIR. Invariante: determinístico; sandbox de I/O
  (ADR-0004). Testes: differential contra oracle.
- **clojure-runtime** — Vars, namespaces, fn objects, seqs, exceções, atoms, dynamic
  binding. Exposto ao código gerado via ABI estável. Testes: unit + differential.
- **clojure-analyzer** — `analyze(forms, ns_env) -> Result<Ast, Diagnostics>`. Invariante:
  toda Var resolvida ou erro; recur em tail-position validado. Testes: unit + golden AST.
- **clojure-ir / codegen / linker / ffi / loader / project / cli** — ver
  [COMPILER_PIPELINE.md](COMPILER_PIPELINE.md) e docs específicos.

## Runtime ABI (fronteira código-gerado ↔ runtime)

`[DECISÃO]` O código nativo gerado interage com o runtime por uma **ABI C estável**
(`extern "C"`), não pela API Rust interna. Isso:
- desacopla versão do runtime da versão do codegen;
- viabiliza o backend-C fallback (ADR-0001) sem duplicar contratos;
- facilita o rooting de GC (funções de runtime empurram/retiram frames do shadow-stack).

O runtime é linkado **estaticamente** ao executável final (sem libs dinâmicas do projeto),
garantindo binário autônomo.

## Plataformas (ver §Compatibilidade de plataformas no start_spec)

`[DECISÃO]`:
- **Oficial inicial:** Linux `x86_64` (GNU). CI verde obrigatório.
- **Primeira classe desde o início:** Windows `x86_64` (MSVC ABI; link via `lld-link` ou
  `link.exe`). Não é adaptação tardia — entra na matriz de CI e nos protótipos (protótipo
  #11: linking no Windows).
- `[FUTURO]`: Linux arm64, macOS x86_64, macOS Apple Silicon.

Detalhes de toolchain/linker/libc por plataforma em COMPILER_PIPELINE e RISK_REGISTER.

## Anti-ciclo e verificação

- CI roda `cargo-deny`/checagem de grafo para garantir o DAG de camadas.
- Cada crate declara em `Cargo.toml` apenas as deps de camada inferior.
- Um teste de arquitetura (`clojure-test-support`) falha o build se detectar dependência
  proibida (ex.: `runtime` → `codegen`).
