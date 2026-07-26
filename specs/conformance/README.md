# Suíte de conformidade

Programas Clojure versionados que servem de **contrato executável** do subconjunto
suportado. Cada caso é validado por **differential testing** contra o oracle
(Clojure/JVM) — ver [../TESTING_STRATEGY.md](../TESTING_STRATEGY.md).

> Esta pasta contém, por ora, **apenas a especificação da estrutura**. Os arquivos de
> caso `.clj` + esperados são criados junto com cada fase do
> [../IMPLEMENTATION_PLAN.md](../IMPLEMENTATION_PLAN.md) (não são código de produção do
> compilador; são fixtures de teste).

## Estrutura pretendida

```text
conformance/
├── reader/          # literais, coleções, quoting, metadata, discard, syntax-quote
├── arithmetic/      # + - * / inc dec ; erros declarados de ratio/overflow
├── control-flow/    # if do when cond case loop/recur try/catch/finally
├── functions/       # multi-aridade, variádica, apply, HOF, destructuring
├── closures/        # captura de variáveis, escopo léxico
├── recur/           # tail loops, contadores grandes, limites
├── macros/          # defmacro, syntax-quote, gensym, macroexpand/-1
├── namespaces/      # ns/require/refer/:as/privados/ordem de carga
├── collections/     # vector/map/set/list; assoc/conj/get/update; equality/hash
├── sequences/       # lazy, infinite, take/map/filter/reduce, retenção de memória
├── metadata/        # meta/with-meta/vary-meta
└── errors/          # mensagens com arquivo:linha:coluna; categorias de exceção
```

## Formato de um caso

Cada caso tem (proposta — a firmar na Fase 1 quando o runner for escrito):

```text
casos/<n>/
├── input.clj        # fragmento a compilar/rodar
├── expected.edn     # { :value ... :stdout "..." :error nil :class :spec }
└── notes.md         # opcional: por que existe, referência à spec
```

Campo `:class` classifica o caso (start_spec §30, TESTING_STRATEGY):
- `:spec` — comportamento especificado; **deve** casar com o oracle.
- `:official` — comportamento da implementação oficial que adotamos.
- `:accidental` — acidente da JVM que **não** replicamos (comparação normalizada).
- `:expected-diff` — divergência **declarada** (ver [../COMPATIBILITY_SPEC.md](../COMPATIBILITY_SPEC.md));
  o teste afirma a diferença.

## Runner

O harness (`clojure-test-support`) roda cada caso no oracle e no alvo nativo (ou no
interpretador nas fases iniciais), normaliza acidentes (hash/ordem de hash-map/formatação
de exceção) e compara valor/tipo/stdout/erro/metadata/ordem/efeitos.

Cobertura de 100% dos casos `:spec`/`:official` para o subconjunto atual é **critério de
aceite do MVP #15**.
