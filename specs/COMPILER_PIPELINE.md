# COMPILER_PIPELINE.md

Pipeline: **Reader → Macroexpand → Analyzer → HIR → LIR → Codegen → Link**.
Camada build-time paralela: **Interpreter** (macros / avaliação de topo). Ver
[ARCHITECTURE.md](ARCHITECTURE.md).

---

## 1. Reader

### Tokenizer
- Entrada: bytes UTF-8 com `SourceId`. Saída: stream de tokens com `Span`.
- Tokens: whitespace/vírgula (ignorados, vírgula = espaço), `; comment`, delimitadores
  `() [] {} #{}`, string, char, número, símbolo/keyword, macros de leitura (`' ` `` ` ``
  `~ ~@ @ ^ #_ #' #(` ...).
- Números: distinguir int/float; ratios/bignum ⇒ `[FUTURO]` (por ora erro claro).

### Parser
- Constrói `Form` (ver `clojure-syntax`) com `Span` em **cada** nó.
- Macros de leitura desugaram já no reader:
  `'x`→`(quote x)`, `@x`→`(deref x)`, `#'x`→`(var x)`, `#(...)`→`(fn* [%1 ...] ...)`,
  syntax-quote expande para código de construção (`seq`/`concat`/`list`/`apply vector`…)
  com resolução de símbolo para o namespace atual e `gensym` para `x#`.
- `#_` descarta a próxima form. `^meta form` anexa metadata à próxima form.

### Requisitos
- **Source spans** preservados até diagnósticos e stack traces (source maps).
- **Recuperação de erro:** ao encontrar delimitador não-balanceado etc., emitir
  diagnóstico e tentar continuar (para reportar múltiplos erros num `check`).
- **Interning** de símbolos e keywords (ids inteiros; igualdade O(1)).
- **Metadata** de leitura preservada (linha/coluna e `^{...}` do usuário).
- **Feature flags** afetam o reader (`regex`, `bigint`, reader conditionals) — desligadas
  no MVP.

### Critério de aceite (Fase 1)
`clojure-native read examples/basic.clj` produz dump estruturado **determinístico** das
forms (ordem estável, spans corretos). Base de golden tests + fuzz do reader.

---

## 2. Macro expansion  (ADR-0004)

`[DECISÃO]` Macros executam em **build-time** via **interpretador tree-walking**
(`clojure-interp`), não via backend nativo e não via JVM.

### Estratégia (comparação — decisão em [adr/0004-macro-execution.md](adr/0004-macro-execution.md))
1. Interpretador interno — **escolhido** (simples, determinístico, sandboxável, sem dep de
   backend).
2. Compilação incremental de macros para nativo — rejeitado no MVP (complexo, lento p/
   build).
3. VM interna dedicada — desnecessário dado o interpretador.
4. Bootstrap de parte de `clojure.core` — sim, complementar (as macros do usuário chamam
   core; core mínimo roda no mesmo interpretador). Ver ADR-0005.
5. Macros iniciais em Rust — sim para as *essenciais* do bootstrap (`defn`, `let`, `when`,
   `cond`, `->`, `fn` sugar) até `clojure.core` conseguir defini-las em Clojure.
6. Híbrido — resultado final: (1)+(4)+(5).

### Semântica de expansão
- Loop `macroexpand-1` até ponto fixo por posição de chamada; símbolo em posição de
  operador que resolve a uma Var com `^:macro` ⇒ chama a função-macro com `(&form, &env,
  &args)`.
- `&form` = a form original (com metadata/span). `&env` = mapa de locais visíveis (MVP:
  nomes → placeholder; valor semântico de `&env` reduzido, documentado).
- Ordem de carregamento: uma macro precisa estar **definida e avaliada** antes de ser
  usada (compilação em ordem topológica de namespaces; dentro do arquivo, de cima p/
  baixo). Macros entre namespaces exigem `require` do ns que a define.

### Propriedades exigidas
- **Determinismo:** mesma entrada ⇒ mesma expansão (sem depender de horário, PID, ordem de
  hash não-determinística, rede).
- **Isolamento/segurança:** sandbox restringe I/O/rede/ambiente em build-time por padrão
  (ver Segurança / ADR-0004). Acesso a filesystem só a caminhos do projeto declarados.
- **Cache:** expansões e avaliação de topo determinísticas podem ser cacheadas por hash de
  (fonte + versão do compilador + flags).
- **Cross-compilation:** como não há host-interop no MVP, macros não dependem da
  plataforma-alvo; expansão é independente de target.
- `macroexpand`/`macroexpand-1` expostos como funções de `clojure.core` (usam o mesmo
  motor). `macroexpand-all` via `clojure.walk` `[FUTURO próximo]`.

---

## 3. Analisador semântico

Entrada: forms expandidas. Saída: **Analyzed AST** tipada por variante + diagnósticos.

Responsabilidades:
- **Lexical scope** e resolução de **locais** (índices de slot por frame).
- **Resolução de Vars**: símbolo → Var de namespace (respeitando aliases, `refer`,
  privados). Não resolvido ⇒ erro com sugestão ("did you mean…", ns não `require`d).
- **Namespaces**: aplica `ns`/`require`/`refer`/`:as`; mantém `NsEnv`.
- **Captures** de closures: variáveis livres capturadas por cada `fn`.
- **Aridades**: valida assinaturas de `fn` multi-aridade; no máximo uma variádica; sem
  sobreposição de aridade fixa.
- **recur targets**: `recur` só em tail-position de `fn`/`loop`; aridade do `recur` casa
  com o alvo; erro claro caso contrário.
- **Validação de tail position** (para `recur` e futura TCO).
- **Type hints**: numéricos aproveitados/avisados; hints de classe Java ⇒ erro (Compat).
- **Metadata semântica**: `^:private`, `^:dynamic`, `^:const`, `^:macro`.
- **Checagem das formas especiais** (aridade/estrutura de `if`, `let*`, `fn*`, `def`,
  `try/catch/finally`, `loop*/recur`, `quote`, `var`, `case*`).
- **Desugaring**: destructuring (`let`/`fn` params) → `let*` + acessos; `and`/`or`/`when`/
  `cond`/`->`/`->>`/`if-let` são **macros** (em core), não formas especiais.

Nós da AST (esboço):
```text
Ast =
  Const(value) | LocalRef(slot) | VarRef(var)
  | If(test, then, else) | Do(stmts, ret)
  | Let(bindings: [(slot, Ast)], body) | Loop(bindings, body)
  | Recur(args, target)
  | Fn(name?, methods: [FnMethod{params, variadic?, body, captures}])
  | Def(var, init?, meta) | Quote(form) | VarSpecial(var)
  | Invoke(fn: Ast, args: [Ast])
  | Try(body, catches: [(type, slot, body)], finally?)
  | Throw(expr) | Case(expr, clauses, default)
  | New/Interop  -> erro diagnóstico (fora de escopo)
```

---

## 4. Representação intermediária (IR)

`[DECISÃO]` Múltiplos níveis, do alto ao baixo:

```text
Parsed Forms → Expanded Forms → Analyzed AST → HIR → LIR → Backend IR (Cranelift)
```

Comparação de estilos (escolha justificada):
- AST anotada: bom p/ análise, ruim p/ codegen direto.
- **HIR** = AST desugarada com nós ainda "Clojure-aware" (Invoke, Closure, SeqOp), locais
  em slots, closures com lista de capturas explícita. **Adotado.**
- **LIR** = forma **A-normal form (ANF)**: toda subexpressão composta nomeada em um
  temporário; controle de fluxo explícito (blocos, branches, calls). Facilita TCO,
  alocação e tradução direta para Cranelift IR (que já é SSA por blocos). **Adotado** em
  vez de SSA/CPS completos (menor complexidade, suficiente).
- CPS / SSA puro / MIR-like: `[FUTURO]` se surgir necessidade de otimizações avançadas.

Tipos centrais:
- **HIR**: `HExpr` espelha `Ast` porém com destructuring já resolvido, `Invoke` anotado com
  aridade conhecida quando estática, `Closure { captures, method }`.
- **LIR**: `Fn { blocks: [Block] }`, `Block { params, insts, terminator }`,
  `Inst = Const | CallStatic | CallDynamic | MakeClosure | AllocSeq | ...`,
  `Terminator = Ret | Br | CondBr | TailCall(recur)`.

### Passes / otimizações
Obrigatórias no MVP (baratas e necessárias à correção/tamanho):
- **recur → loop** (backedge no bloco; é a "TCO" garantida da linguagem).
- **constant folding** de literais e `case` constante.
- **dead code elimination** trivial (código após `throw`/`recur`, ramos mortos de `if`
  constante).
- resolução de **chamada direta** quando o alvo (Var de fn) e a aridade são conhecidos em
  tempo de compilação (evita dispatch dinâmico).

Futuras `[FUTURO]` (fora do caminho crítico do MVP — justificar antes de adotar):
inlining, unboxing, escape analysis, monomorphization, devirtualization, dispatch direto
de protocol, especialização de coleções persistentes, eliminação de seqs intermediárias,
eliminação de alocação de closures.

> Regra do start_spec: **não começar pela otimização.** MVP faz o mínimo p/ correção e
> tamanho razoável.

---

## 5. Codegen (ADR-0001)

- **Backend primário: Cranelift.** LIR → Cranelift IR → objeto (`.o`/`.obj`) por função.
- Chamadas ao runtime via **ABI C** (símbolos `extern "C"` do `clojure-runtime`).
- Cada `fn` Clojure vira uma ou mais funções nativas (uma por aridade) + um "fn object"
  com ponteiros e capturas (ver [RUNTIME_SPEC.md](RUNTIME_SPEC.md#modelo-de-funcoes)).
- Inicialização global determinística: um `__cljn_init` gerado registra namespaces/Vars na
  ordem topológica antes de chamar `-main` (ver RUNTIME_SPEC — Vars/namespaces).
- **Backend-C fallback** `[FUTURO]`: emite C equivalente; útil p/ portabilidade e debug.

---

## 6. Link

- `clojure-linker` invoca o linker da plataforma:
  - Linux: `ld.lld`/`cc` (estático quando possível; libc GNU).
  - Windows: `lld-link`/`link.exe` (MSVC ABI); tratar como 1ª classe.
- Linka objetos do usuário + `clojure-runtime` + `clojure-core-native` + GC, **estático**.
- Produz **executável autônomo** (sem JVM, `.class`, libs dinâmicas do projeto).

---

## 7. Tratamento de erros (pipeline)

Cada estágio produz `Diagnostic` estruturado (nunca `panic!` como erro normal — ver
Segurança / start_spec §18). Categorias e requisitos: ver documento dedicado abaixo.

| Estágio | Erros típicos |
| --- | --- |
| Reader | delimitador não-fechado, char/número inválido, EOF em string |
| Macroexpand | macro lança, expansão não-terminante (limite), símbolo não resolvido em syntax-quote |
| Analyzer | Var não resolvida, `recur` fora de tail-pos/aridade errada, forma especial malformada, construção fora de escopo (interop) |
| IR/Codegen | invariante interna (bug do compilador → erro "internal compiler error" com pedido de report), símbolo externo ausente |
| Linker | símbolo indefinido, toolchain ausente |
| Runtime | exceções da linguagem (ver RUNTIME_SPEC — Exceções) |

**Toda** mensagem ao usuário: arquivo, linha, coluna, trecho do código, causa, contexto,
sugestão quando possível. Códigos estáveis `E0001…` para documentação. Panics do Rust são
bugs do compilador, capturados e reportados como "internal error", nunca como erro de
usuário. `[DECISÃO]` — detalhes em [RUNTIME_SPEC.md](RUNTIME_SPEC.md#erros-e-excecoes).
