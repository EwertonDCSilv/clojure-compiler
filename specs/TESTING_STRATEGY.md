# TESTING_STRATEGY.md

Estratégia baseada em **equivalência semântica** contra a implementação oficial de
Clojure/JVM usada **apenas como oracle de desenvolvimento** — nunca como dependência do
binário final (start_spec §22).

> **Fato local**: Java 21 está disponível no ambiente; a CLI `clojure`/`clj` **não**.
> Para usar o oracle é preciso instalar Clojure (ou usar um jar do `clojure.main`) no
> ambiente de dev/CI. O binário produzido **não** depende disso.

## Diferencial (oracle) — o método central

Para cada fragmento de código no subconjunto suportado:

1. Executa no **oracle** (Clojure/JVM) → captura valor (`pr-str`), tipo, stdout/stderr,
   erro (classe+mensagem), metadata, ordem, efeitos.
2. Executa no **binário nativo** (ou no interpretador de bootstrap) → captura o mesmo.
3. **Compara**, com **normalização** que ignora *acidentes* da JVM (start_spec §30):
   - ignorar diferenças de `hash` numérico e ordem de iteração de `hash-map` **não
     especificada** (comparar como conjuntos/mapas, não como texto);
   - ignorar formatação de exceção específica da JVM; comparar **categoria** de erro e
     mensagem essencial;
   - divergências **declaradas** em [COMPATIBILITY_SPEC.md](COMPATIBILITY_SPEC.md) são
     marcadas como *expected-diff* (o teste afirma a divergência, não a igualdade).

Distinguir sempre: **spec** (deve casar) × **comportamento oficial** × **acidente**
(não casar exige justificativa) — cada caso de teste declara em qual categoria está.

## Tipos de teste e ferramentas Rust (com justificativa)

| Teste | Ferramenta | Por quê |
| --- | --- | --- |
| Unitário / integração | `cargo test` | base |
| Parser / golden de forms e AST | `insta` (snapshots) | detecta regressão de estrutura/erros com revisão fácil |
| Golden de mensagens de erro | `insta` | diagnósticos são contrato de UX |
| Property-based (leis de `=`/`hash`, coleções persistentes vs. modelo) | `proptest` | cobre espaço de entrada que exemplos não cobrem |
| (alternativa/《complemento》 property) | `quickcheck` | avaliar; provavelmente **só `proptest`** p/ não duplicar — decisão: `proptest` |
| Differential (oracle) | harness próprio em `clojure-test-support` | equivalência semântica |
| Fuzzing do reader | `cargo-fuzz` | reader é superfície de entrada não confiável; achar panics/hangs |
| Fuzzing de coleções persistentes | `cargo-fuzz` + modelo | sequências aleatórias de ops vs. `Vec`/`BTreeMap` de referência |
| GC: stress, ciclos, retenção | testes próprios | correção do coletor e ausência de vazamento |
| Segurança de memória em `unsafe` (GC, value, ffi) | **Miri** | detecta UB em código `unsafe` |
| Sanitizers (ASan/UBSan) no binário nativo | build flags | UB no código gerado/runtime linkado |
| Concorrência (`atom`/CAS; futuro multi-thread) | **Loom** | modela intercalações; só onde há concorrência (MVP: mínimo) |
| Benchmarks | **Criterion** | medir startup/dispatch/coleções/GC (PERF) |
| ABI / FFI | testes de integração C | contratos `extern "C"` estáveis |
| End-to-end / build limpo | scripts de CI | `clojure-native build` em ambiente limpo |
| Cross-platform | matriz de CI | Linux **e** Windows verdes (requisito de aceite) |

Justificativa de exclusão: `quickcheck` redundante com `proptest` (adotar um só). Loom só
onde há real concorrência (evitar custo sem concorrência no MVP).

## Suíte de conformidade

Programas `.clj` versionados que servem de contrato executável (ver
[conformance/README.md](conformance/README.md)):

```text
specs/conformance/
├── reader/          # literais, coleções, quoting, metadata, discard
├── arithmetic/      # + - * / (erros de ratio/overflow declarados)
├── control-flow/    # if do when cond case loop/recur try/catch/finally
├── functions/       # multi-aridade, variádica, apply, HOF
├── closures/        # captura, escopo léxico
├── recur/           # tail loops, limites
├── macros/          # defmacro, syntax-quote, gensym, macroexpand
├── namespaces/      # ns/require/refer/alias/privados
├── collections/     # vector/map/set/list, assoc/conj/get, equality/hash
├── sequences/       # lazy, infinite, take/map/filter/reduce, retenção
├── metadata/        # meta/with-meta/vary-meta
└── errors/          # mensagens: arquivo:linha:coluna, categorias de exceção
```

Cada caso tem: entrada `.clj`, resultado esperado (do oracle) e classificação
(spec/oficial/acidente/expected-diff). O runner roda oracle + nativo e compara.

## Portões de qualidade (CI)

- Todo PR: `cargo test`, `clippy` sem warnings, `fmt`, Miri nos crates `unsafe`, suíte de
  conformidade (subconjunto atual), golden snapshots.
- Cobertura Rust bloqueante com `cargo-llvm-cov`: mínimo global de **82%** para linhas,
  funções e regiões, além de **30% de linhas por arquivo**. O comando local
  `scripts/coverage.sh` aplica os mesmos limites do CI.
- Matriz de plataformas: Linux x86_64 (bloqueante) + Windows x86_64 (bloqueante).
- Fuzz/bench: jobs periódicos (não bloqueantes por corrida, mas monitorados).
- Cobertura de conformidade é métrica de sucesso (VISION): 100% dos casos declarados
  passam para o subconjunto suportado (critério de aceite #15).

## Faseamento dos testes (acompanha IMPLEMENTATION_PLAN)

- Fase 1: golden do reader + fuzz do reader.
- Fase 2: differential do interpretador (literais/`if`/`let`/`fn`/calls).
- Fase 4: property de `=`/`hash` e coleções persistentes vs. modelo.
- Fase 5: e2e do primeiro binário (`hello`), build limpo.
- Fase 6+: conformidade de macros; depois collections/sequences/namespaces/errors.
