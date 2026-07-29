# ADR-0002 — Gerenciamento de memória

- **Status:** Aceito e implementado (GC mark-sweep preciso e não-movente com shadow-stack em produção)
- **Contexto:** Clojure exige gerenciamento **automático** de memória com closures, lazy
  seqs (potencialmente cíclicas), estruturas persistentes com muito compartilhamento,
  Vars de longa duração e muito garbage temporário de sequências. Detalhes em
  [MEMORY_MODEL.md](../MEMORY_MODEL.md).

## Alternativas
Ver tabela completa em [MEMORY_MODEL.md](../MEMORY_MODEL.md#comparação-de-abordagens).
Pontos decisivos:
- **Refcount puro** vaza ciclos (lazy-seq auto-referente, closures via atom/Var) e sofre
  churn no sharing ⇒ **insuficiente** como modelo do runtime.
- **Boehm conservativo** é impreciso (vaza, falsos ponteiros) ⇒ rejeitado.
- **Geracional/incremental/MMTk** são o alvo de longo prazo, mas exigem write barriers e/ou
  integração de object-model/roots pesada ⇒ **`[FUTURO]`**.

## Decisão

**Coletor tracing mark-sweep, preciso, não-móvel, single-thread**, com **shadow-stack de
roots** (rooting explícito por handles), em `clojure-gc`. O **interpretador de bootstrap**
pode usar `Rc` para andar rápido (não é o runtime de produção).

### Justificativa
- **Preciso** evita vazamentos por falsos ponteiros; controlamos layout (`Obj` com
  cabeçalho + tracer por tipo) e roots.
- **Não-móvel** dispensa write barriers e atualização de ponteiros ⇒ integração simples com
  código gerado (Cranelift) e FFI.
- **Shadow-stack** evita escanear a pilha nativa (frágil/dependente de backend); troca por
  disciplina de rooting no runtime/codegen — portável e preciso (protótipo #6).
- **Single-thread** mantém o coletor simples; multi-thread/concorrente é `[FUTURO]` junto
  com `future`/threads.
- Correto para **ciclos** (lazy-seq/closures), ao contrário de refcount.

### Riscos e mitigação (R3, R14)
- Precisão de roots → protótipo #6 + testes de ciclo/retenção; Miri no `unsafe`.
- Pausas stop-the-world → medir (Criterion); geracional/concorrente `[FUTURO]`.
- Fragmentação (não-móvel) → free-lists por tamanho; compactação `[FUTURO]`.

## Consequências
- ABI de runtime inclui `cljn_gc_push_frame`/`pop_frame`/`cljn_alloc`; safepoints em
  alocação.
- Evolução: geracional + write barriers → móvel/compactação → multi-thread, possivelmente
  **MMTk**, mantendo a fronteira de rooting estável (não reescrever codegen).
- Finalizers evitados; recursos externos por fechamento explícito.
