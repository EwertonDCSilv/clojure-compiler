# MEMORY_MODEL.md

Modelo de gerenciamento automático de memória do runtime. Decisão registrada em
[adr/0002-memory-management.md](adr/0002-memory-management.md).

## Requisitos derivados da semântica de Clojure

Clojure produz padrões de alocação específicos (start_spec §9):
- **closures** e **lazy sequences** (podem formar **ciclos**: `(def xs (lazy-seq (cons 1
  xs)))`, closures que se referenciam via atoms/Vars);
- **estruturas persistentes** com **compartilhamento estrutural** intenso (muitos objetos
  imutáveis compartilhados — refcount teria contenção/churn alto);
- **metadata**, **caches** (dispatch/hash), **Vars** (raízes globais de longa duração);
- muitos **objetos temporários** em operações de sequência (garbage de curta vida).

Conclusão `[DECISÃO]`: **reference counting puro é insuficiente** (vaza ciclos de
lazy-seq/closure e sofre churn no sharing). É necessário um **coletor tracing**.

## Comparação de abordagens

| Abordagem | Prós | Contras | Veredito |
| --- | --- | --- | --- |
| Refcount (`Rc`/`Arc`) | simples, determinístico, casa com Rust | vaza ciclos; churn no sharing; contenção `Arc` | só no **interp de bootstrap** |
| Mark-sweep tracing precisa | correto p/ ciclos; sem barreiras (não-móvel) | pausas stop-the-world; fragmentação | **MVP (não-móvel)** |
| Generational | ótimo p/ garbage jovem (seqs) | write barriers, mais complexo | **[FUTURO]** |
| Incremental/concurrent | pausas menores | muito complexo | **[FUTURO]** |
| Boehm (conservativo) | fácil, sem rooting | impreciso, vaza, dep C | rejeitado (impreciso) |
| MMTk | produção, geracional, pluggável | integração pesada (object model + roots) | **alvo [FUTURO]** |
| Cycle collector sobre refcount | mantém refcount | complexo, ainda churn | rejeitado |
| Arenas | rápido p/ fases | não serve a lifetimes gerais de Clojure | uso pontual (compilador, não runtime) |
| GC próprio | controle total | trabalho/bugs | é o mark-sweep do MVP |

## Decisão MVP

**Coletor tracing mark-sweep, preciso, não-móvel, single-thread**, com **shadow-stack de
roots** (rooting explícito por handles), no crate `clojure-gc`.

Por que essas propriedades:
- **Preciso** (não conservativo): sem vazamentos por falsos ponteiros; requer conhecer os
  roots e o layout dos objetos — que controlamos (`Obj` tem cabeçalho + tracer por tipo).
- **Não-móvel** (mark-sweep, sem cópia/compactação): dispensa **write barriers** e
  atualização de ponteiros; simplifica a integração com o código gerado e o FFI. Custo:
  fragmentação (mitigável com free-lists por tamanho; compactação é `[FUTURO]`).
- **Shadow-stack de roots**: o código gerado e o runtime **registram** valores vivos em um
  stack de roots por frame (push na entrada, pop na saída), em vez de escanear a pilha
  nativa (que é dependente de backend/plataforma e frágil com Cranelift). Trade-off:
  disciplina de rooting no runtime/codegen, mas **portável e preciso**. Alternativa de
  stack-map preciso do backend é `[FUTURO]` (depende de suporte de Cranelift).
- **Single-thread**: coletor stop-the-world simples; multithreading + GC concorrente são
  `[FUTURO]` junto com `future`/threads.

## O que o documento fixa

- **Ownership no runtime:** o heap gerenciado é dono dos `Obj`; código Rust do runtime
  segura valores vivos via `Rooted<T>`/handles enquanto manipula. `Value` imediatos não
  são heap.
- **Roots:** (1) Vars/namespaces globais (raízes permanentes); (2) shadow-stack por frame
  de chamada; (3) registradores temporários rooteados durante operações do runtime;
  (4) o valor sendo retornado.
- **Stack scanning:** **não** escaneamos a pilha nativa no MVP (usamos shadow-stack).
- **Preciso vs. conservativo:** preciso.
- **Write barriers:** nenhuma no MVP (não-móvel, não-geracional). Entram com geracional
  `[FUTURO]`.
- **Finalização:** evitada por padrão (não-determinística). Recursos externos (FFI,
  arquivos) usam fechamento explícito / `with-open`-like, **não** finalizers. Finalizers
  fracos ⇒ `[FUTURO]` se necessário.
- **Weak references:** `[FUTURO]` (necessárias para caches de dispatch/intern que não
  devem reter). No MVP, caches são limpáveis ou de tamanho limitado.
- **Thread safety:** N/A no MVP (single-thread). Modelo multi-thread (safepoints, handshake
  de coleção) é `[FUTURO]`.
- **Pausas:** stop-the-world; medir no benchmark de GC pause (PERF). Objetivo MVP: pausas
  aceitáveis para CLIs/serverless; não otimizar prematuramente.
- **Integração com código compilado:** via ABI C — `cljn_gc_push_frame`/`pop_frame`,
  `cljn_alloc(type, size)`; alocação pode disparar coleta em safepoints (nas alocações).
- **Integração com FFI:** objetos passados a C são **pinados/rooteados** durante a chamada;
  ponteiros crus obtidos de C não são gerenciados (ver
  [NATIVE_INTEROP.md](NATIVE_INTEROP.md)).
- **Comportamento em panic:** panic do Rust (bug) faz unwinding controlado na fronteira
  ABI; o heap é abandonado com segurança (processo tende a abortar de forma limpa) — nunca
  corromper e continuar.
- **Destruição na saída do processo:** na saída normal **não** é preciso coletar tudo (o
  SO recupera a memória); só executar efeitos externos pendentes explícitos (flush de
  stdout). Sem dependência de finalizers para correção.

## Estratégia de validação (protótipos obrigatórios — start_spec §27)

Antes de produção, protótipos descartáveis respondem:
- #5 alocação + tracing básico funcionam;
- #6 shadow-stack roots capturam todos os vivos (teste de ciclos e de seqs longos);
- coleção de `(def xs (lazy-seq (cons 1 xs)))` sem vazar; `(reduce + (take 1e6 (range)))`
  em memória O(1).
Testes de stress/ciclos/retenção e uso de **Miri** nos blocos `unsafe` do GC — ver
[TESTING_STRATEGY.md](TESTING_STRATEGY.md).

## Estado da implementação (2026-07-26)

`[FATO]` O runtime **compilado** (`crates/clojure-codegen/runtime.c`) já implementa o
**coletor mark-sweep preciso, não-móvel, single-thread com shadow-stack de roots**
descrito acima:

- **Roots**: o código gerado (`clojure-codegen`) mantém um shadow-stack — cada função
  reserva `local_count` slots (locais, espelhados junto às variáveis Cranelift) via
  `cljn_gc_enter`/`leave`, e empurra/retira temporários (`cljn_gc_push`/`popn`) em volta
  de cada alocação. O coletor varre `[0, gc_sp)`; **nunca** escaneia a pilha nativa.
- **Objetos** têm header (`mark` + lista global) para o sweep; `mark` itera a cauda de
  listas (não recursa) para não estourar a pilha em listas longas.
- **Gatilho**: a cada `N` alocações; `CLJN_GC_STRESS=1` coleta a cada alocação
  (usado nos testes para validar o rooting); `CLJN_GC_OFF=1` desliga (diagnóstico).
- **Validação**: a suíte e2e roda sob `CLJN_GC_STRESS=1` (coleta a cada alocação) e
  mantém a saída correta — evidência de que o rooting é preciso (rooting incorreto
  liberaria valor vivo). Medida de reclamação: loop alocando 10M cons descartáveis →
  RSS ~6 MB **com** GC vs. ~470 MB **sem** (`CLJN_GC_OFF=1`).

Ainda `[FUTURO]` (conforme o caminho de evolução): geracional/write-barriers, móvel/
compactação, multi-thread/concorrente (possivelmente MMTk). O interpretador de bootstrap
segue com `Rc` (conforme previsto).

## Caminho de evolução

MVP (mark-sweep não-móvel, single-thread) → geracional + write barriers (garbage jovem de
seqs) → móvel/compactação → multi-thread + concorrente, possivelmente adotando **MMTk**
como implementação (mantendo a fronteira de rooting estável para não reescrever codegen).
