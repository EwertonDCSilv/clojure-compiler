# Especificações — `clojure-compiler`

Este diretório registra a arquitetura, o escopo e as decisões do compilador nativo. O
nome do produto e do binário é `clojure-native`; o repositório se chama
`clojure-compiler`.

## Estado executável

Em 2026-07-26, o workspace já possui um corte vertical funcional:

- Reader com spans, reader macros e diagnósticos com arquivo, linha e coluna.
- Interpretador de bootstrap para `eval`, `run` e infraestrutura de macros.
- Analyzer e codegen Cranelift capazes de gerar executáveis nativos sem JVM.
- Fixnums tagueados, strings, listas, keywords, vetores, mapas, sets, closures e
  records rastreados pelo runtime nativo.
- `loop/recur` como backedge nativo, closures transitivas, HOF, aridades
  fixas/múltiplas/variádicas e `apply`.
- Expansão compilada de `when`, `when-not`, `if-not`, `cond`, `and`, `or`, `->` e
  `->>`.
- Vetores persistentes em trie bitmap de 32 vias. Mapas e sets começam em representação
  compacta e promovem para HAMT de 32 vias.
- `defrecord`, `defprotocol` e `extend-type`, com dispatch para records e tipos
  embutidos.
- `clojure.core` compilável com 26 funções pré-carregadas em todo `build`.
- Fast paths nativos verificados para `+`, `-`, `*`, `quot`, `mod`, `inc`, `dec`,
  `<`, `<=`, `>` e `>=`; igualdade estrutural permanece no runtime.
- GC mark-sweep preciso, não móvel e single-thread, validado com
  `CLJN_GC_STRESS=1`.
- Operações diretas na shadow stack substituem `gc_push`, `gc_popn` e `gc_set` no
  caminho quente. O rooting ainda é eager; liveness em safepoints é a próxima etapa.

A suíte Rust possui 106 testes. A matriz em [`tests/conformance/`](../tests/conformance)
possui 205 casos: 154 ativos, 20 falhas esperadas e 31 itens pendentes. Os níveis D e E
agora combinam recortes executáveis com lacunas `xfail` e projetos `pending`. O gate de
cobertura exige 82% globais para linhas, funções e regiões, além de 30% de linhas por
arquivo.

O benchmark numérico de 100 milhões de iterações caiu de 3,02 s para 0,66 s após os
fast paths e os stores diretos de roots. Os resultados completos e as ressalvas de
medição estão na [ADR-0006](adr/0006-codegen-optimization.md).

## Limites atuais

As specs descrevem tanto o que existe quanto o alvo futuro; marcações de fase e
`[FUTURO]` não devem ser lidas como funcionalidade entregue. O caminho nativo ainda não
oferece bignums, ratios, ponto flutuante compilado, macros definidas pelo usuário,
lazy-seq, exceções, namespaces dinâmicos, projetos multi-arquivo ou interop Java.

## Como ler estes documentos

Ordem sugerida:

1. [VISION.md](VISION.md) — problema, proposta de valor e métricas de sucesso.
2. [LANGUAGE_SCOPE.md](LANGUAGE_SCOPE.md) — matriz do subconjunto da linguagem.
3. [COMPATIBILITY_SPEC.md](COMPATIBILITY_SPEC.md) — níveis A–E e incompatibilidades.
4. [ARCHITECTURE.md](ARCHITECTURE.md) — fronteiras e fluxo de dados planejados.
5. [COMPILER_PIPELINE.md](COMPILER_PIPELINE.md) — reader, expansão, análise e codegen.
6. [RUNTIME_SPEC.md](RUNTIME_SPEC.md) — implementação atual e modelo futuro do runtime.
7. [MEMORY_MODEL.md](MEMORY_MODEL.md) — GC, roots e ownership.
8. [STANDARD_LIBRARY_SCOPE.md](STANDARD_LIBRARY_SCOPE.md) — biblioteca entregue e alvo.
9. [NATIVE_INTEROP.md](NATIVE_INTEROP.md) — FFI em ABI C.
10. [TESTING_STRATEGY.md](TESTING_STRATEGY.md) — testes, cobertura e oracle manual.
11. [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) — fases incrementais.
12. [RISK_REGISTER.md](RISK_REGISTER.md) — riscos e mitigações.

Documentos operacionais:

- [conformance/README.md](conformance/README.md) — suíte executável A–E.
- [optime.md](optime.md) — plano de otimização.
- [adr/0006-codegen-optimization.md](adr/0006-codegen-optimization.md) — decisão e
  resultados de otimização.

## ADRs

- [0001](adr/0001-code-generation-backend.md) — Cranelift como backend primário.
- [0002](adr/0002-memory-management.md) — GC tracing mark-sweep com shadow stack.
- [0003](adr/0003-value-representation.md) — modelo inicial de representação de valores.
- [0004](adr/0004-macro-execution.md) — interpretador de bootstrap para macros.
- [0005](adr/0005-bootstrap-strategy.md) — primitivas + core progressivo em Clojure.
- [0006](adr/0006-codegen-optimization.md) — fast paths de fixnum, roots e opt-level.

ADRs aceitas não são reescritas para representar o estado posterior. Uma mudança
fundamental deve criar uma nova ADR que substitua explicitamente a anterior.

## Resumo das decisões

| Área | Implementação atual | Alvo |
| --- | --- | --- |
| Backend | Cranelift AOT + link pelo `cc` do sistema | backend C opcional |
| Macros | expansão de macros de core conhecidas | macros de usuário no bootstrap |
| Valor nativo | fixnums tagueados + ponteiros para objetos GC | especialização/unboxing medidos |
| Memória | mark-sweep preciso, não móvel, single-thread, shadow stack | rooting por liveness; GC geracional futuro |
| Bootstrap | primitivas no runtime + core compilado em Clojure | self-hosting parcial |
| Otimização | fast paths inteiros e stores diretos de roots | safepoints e otimização do IR |
| Coleções | lista, trie vetorial, array-map/set com promoção HAMT | CHAMP, sorted e transients |
| Plataforma | compilação e link para o host | matriz multiplataforma ampliada |

## Convenções

- `[FATO]`: observado ou verificável.
- `[JVM]`: comportamento da implementação oficial.
- `[DECISÃO]`: escolha arquitetural do projeto.
- `[HIPÓTESE]`: item ainda a validar.
- `[RISCO]`: risco registrado.
- `[FUTURO]`: fora do caminho executável atual.

Estas specs são vivas. A classificação executável da compatibilidade é sempre baseada
no código e nas fixtures atuais, não em itens aspiracionais.
