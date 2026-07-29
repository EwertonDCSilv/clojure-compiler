# Especificações — `clojure-compiler`

[README do projeto](../README.pt-BR.md) ·
[Documentação operacional](../docs/README.md) ·
[Matriz de conformidade](conformance/README.md) ·
[Benchmarks](../benchmarks/README.md)

Este diretório registra a arquitetura, o escopo e as decisões do compilador nativo. O
nome do produto e do binário é `clojure-native`; o repositório se chama
`clojure-compiler`.

> Snapshot documentado: [`HEAD 424ba20`](https://github.com/EwertonDCSilv/clojure-compiler/commit/424ba20e88fd91a641675e4d9d9bf111c63fc164)
> em 2026-07-28. A política e o benchmark de referência estão em
> [`docs/SNAPSHOT.md`](../docs/SNAPSHOT.md).

## Estado executável

Em 2026-07-28, o workspace já possui um corte vertical funcional:

- Reader com spans, reader macros e diagnósticos com arquivo, linha e coluna.
- Interpretador de bootstrap para `eval`, `run` e infraestrutura de macros.
- Analyzer e codegen Cranelift capazes de gerar executáveis nativos sem JVM.
- Fixnums tagueados, doubles IEEE-754 boxeados, strings, listas, keywords, vetores,
  mapas, sets, closures e records rastreados pelo runtime nativo.
- `loop/recur` como backedge nativo, closures transitivas, HOF, aridades
  fixas/múltiplas/variádicas e `apply`.
- Expansão compilada de `when`, `when-not`, `if-not`, `cond`, `and`, `or`, `->` e
  `->>`.
- Vetores persistentes em trie bitmap de 32 vias. Mapas e sets começam em representação
  compacta e promovem para HAMT de 32 vias.
- `defrecord`, `defprotocol` e `extend-type`, com dispatch para records e tipos
  embutidos.
- `throw` e `try`/`catch`/`finally` nativos com unwind aninhado e restauração da
  shadow stack.
- `defmulti`/`defmethod` com dispatch por igualdade de valor e fallback `:default`.
- Transients para vetores, mapas e sets por meio de `transient`, `persistent!`,
  `conj!`, `assoc!` e `dissoc!`.
- Construção de `mapv`/`into` por vetor transient estrutural e promoção conservadora de
  acumuladores lineares de `loop`, inclusive no primeiro subconjunto interprocedural.
- Hoisting por site de vetores literais constantes formados somente por imediatos, com
  cache registrado como root permanente do GC.
- `clojure.core` compilável com 26 funções pré-carregadas em todo `build`.
- Fast paths nativos verificados para `+`, `-`, `*`, `quot`, `mod`, `inc`, `dec`,
  `<`, `<=`, `>` e `>=`; igualdade estrutural permanece no runtime.
- GC mark-sweep preciso, não móvel e single-thread, validado com
  `CLJN_GC_STRESS=1`.
- Operações diretas na shadow stack substituem `gc_push`, `gc_popn` e `gc_set` no
  caminho quente. O rooting ainda é eager; liveness em safepoints é a próxima etapa.
- Runtime C separado fisicamente por subsistema, mas amalgamado como uma única unidade
  de tradução e uma única ABI.
- I/O textual nativo com streams dinâmicos, `slurp`/`spit`, `read-string`, flush e
  redirecionamento cobertos pela matriz executável.

O workspace possui uma suíte Rust bloqueante. A matriz em
[`tests/conformance/`](../tests/conformance) possui 461 casos: 225 ativos, 204 falhas
esperadas e 32 itens pendentes. Os níveis D e E
agora combinam recortes executáveis com lacunas `xfail` e projetos `pending`. O gate de
cobertura exige 82% globais para linhas, funções e regiões, além de 30% de linhas por
arquivo.

O benchmark numérico de 100 milhões de iterações caiu de 3,02 s para 0,66 s após os
fast paths e os stores diretos de roots. No snapshot mais recente, Cracking acumula
8,05 s nativos contra 23,02 s na JVM; Cormen acumula 27,23 s contra 16,95 s de parede,
mas 27,09 s contra 32,08 s de CPU. Os 98 checksums são equivalentes. A queda de 9,4%
na parede nativa do Cormen contra o snapshot anterior é positiva, mas ainda exige
medições pareadas antes de ser atribuída ao compilador. O corpus
externo Exercism audita 101 soluções práticas e 13 exemplares conceituais oficiais:
10 compilam e 104 registram o primeiro bloqueador. Oito cargas adequadas formam uma
suíte de desempenho Native × JVM separada; os demais casos pertencem ao relatório de
conformidade. A varredura literal do checkout cobre 493 arquivos Clojure, dos quais
117 compilam isoladamente.
A metodologia,
ressalvas e evolução estão na [ADR-0009](adr/0009-benchmark-performance-study.md), em
[`benchmarks/exercism/`](../benchmarks/exercism) e em
[`docs/SNAPSHOT.md`](../docs/SNAPSHOT.md).

## Limites atuais

As specs descrevem tanto o que existe quanto o alvo futuro; marcações de fase e
`[FUTURO]` não devem ser lidas como funcionalidade entregue. O caminho nativo ainda não
oferece bignums, ratios, BigDecimal, macros definidas pelo usuário, lazy-seq, namespaces
dinâmicos, projetos multi-arquivo ou interop Java. Exceções ainda não têm hierarquia
tipada e multimétodos ainda não usam hierarquias. O runtime já expõe valores e
primitivas para streams, arquivos, bytes, paths, filesystem, contexto de processo e
leitura de dados. O gate completo de I/O continua aberto porque APIs derivadas, opções
EDN completas e vários contratos de lifecycle/erro ainda não possuem evidência ativa
de conformidade.

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
9. [IO_SPEC.md](IO_SPEC.md) — gate proposto de streams, arquivos, processo e readers.
10. [PEDESTAL_NATIVE_CONNECTOR_SPEC.md](PEDESTAL_NATIVE_CONNECTOR_SPEC.md) — **Planejado**:
    connector HTTP nativo e subconjunto de aplicação compatível com Pedestal.
11. [OPTIMIZATION_IR_SPEC.md](OPTIMIZATION_IR_SPEC.md) — **Partially implemented**:
    optional verified scalar IR; the current scalar profile passed its Cormen gate,
    while whole-function lowering and root-plan consumption remain open.
12. [ASSOCIATIVE_INDEXED_SPEC.md](ASSOCIATIVE_INDEXED_SPEC.md) — contrato proposto de
    `assoc` persistente e `nth` genérico.
13. [NATIVE_INTEROP.md](NATIVE_INTEROP.md) — FFI em ABI C.
14. [TESTING_STRATEGY.md](TESTING_STRATEGY.md) — testes, cobertura e oracle manual.
15. [TDD_WORKFLOW.md](TDD_WORKFLOW.md) — evolução Red–Green–Refactor e contratos de
    regressão.
16. [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) — fases incrementais.
17. [RISK_REGISTER.md](RISK_REGISTER.md) — riscos e mitigações.

Documentos operacionais:

- [../docs/README.md](../docs/README.md) — índice dos guias correntes.
- [../docs/usage.md](../docs/usage.md) — CLI, Makefile, instalação e gates.
- [conformance/README.md](conformance/README.md) — suíte executável A–E.
- [conformance/READER_SYNTAX_COVERAGE.md](conformance/READER_SYNTAX_COVERAGE.md) —
  denominador e métricas objetivas da sintaxe do Reader Clojure 1.12.5.
- [../benchmarks/README.md](../benchmarks/README.md) — catálogo das suítes de desempenho.
- [optime.md](optime.md) — plano de otimização.
- [adr/0006-codegen-optimization.md](adr/0006-codegen-optimization.md) — decisão e
  resultados de otimização.
- [adr/0014-optional-optimization-ir.md](adr/0014-optional-optimization-ir.md) —
  optional IR decision and its mandatory Cormen performance gate.
- [adr/0015-internal-value-root-and-abi-specialization.md](adr/0015-internal-value-root-and-abi-specialization.md)
  — internal value, shadow-stack, and call-boundary specialization with stricter
  structural and Cormen admission gates.

Para verificar o estado executável sem confundir itens futuros com recursos entregues:

```bash
make quality
make compatibility
make benchmarks
```

O [`Makefile`](../Makefile) é a interface operacional pública. Scripts individuais
continuam documentados somente quando são necessários para manutenção de baixo nível ou
para reproduzir uma medição histórica.

## ADRs

- [0001](adr/0001-code-generation-backend.md) — Cranelift como backend primário.
- [0002](adr/0002-memory-management.md) — GC tracing mark-sweep com shadow stack.
- [0003](adr/0003-value-representation.md) — modelo inicial de representação de valores.
- [0004](adr/0004-macro-execution.md) — interpretador de bootstrap para macros.
- [0005](adr/0005-bootstrap-strategy.md) — primitivas + core progressivo em Clojure.
- [0006](adr/0006-codegen-optimization.md) — fast paths de fixnum, roots e opt-level.
- [0007](adr/0007-native-io-and-runtime-reader.md) — I/O atrás da ABI C e reader de
  runtime em Clojure (proposta).
- [0008](adr/0008-associative-indexed-dispatch.md) — capabilities com fast paths para
  `assoc` persistente e `nth` genérico (proposta).
- [0009](adr/0009-benchmark-performance-study.md) — estudo da performance nativa nos
  benchmarks Cormen e prioridades de otimização.
- [0010](adr/0010-interprocedural-ephemeral-vectors.md) — análise de
  escape/unicidade para vetores efêmeros interprocedurais (parcialmente implementada).
- [0011](adr/0011-rust-crate-unit-testing-strategy.md) — estratégia de testes
  unitários, integração e gates de cobertura dos crates Rust (proposta).
- [0012](adr/0012-rust-crate-modularization.md) — modularização incremental dos crates
  Rust e controle da dívida de arquivos gigantes (proposta).
- [0013](adr/0013-compiled-clojure-pedestal-native-connector.md) — connector
  Pedestal-compatible escrito em Clojure compilado sobre provider HTTP C (proposta).
- [0014](adr/0014-optional-optimization-ir.md) — optional, backend-neutral
  optimization IR that remains disabled by default (accepted; implementation in
  progress).
- [0015](adr/0015-internal-value-root-and-abi-specialization.md) — internal unboxed
  representations, virtual roots, specialized direct calls, and classified runtime
  boundaries (accepted; experimental bundle not admitted to `safe`).

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
| Otimização | fast paths inteiros, stores diretos, auto-transient, hoisting e IR escalar `safe` opt-in | whole-function IR, roots virtuais e ABI interna especializada admitidos somente pelos gates Cormen |
| Coleções | lista, trie vetorial, HAMT, sorted map/set por LLRB e transients iniciais | CHAMP e transients com edit tokens |
| Operações de coleção | dispatch fechado por tag para `assoc`/`nth` | capabilities extensíveis com fast paths nativos |
| I/O | output, flush, redirecionamento, texto em arquivo e streams de string | filesystem amplo, binários e reader/EDN completo conforme IO_SPEC |
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
