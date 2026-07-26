# ADR-0005 — Estratégia de bootstrap

- **Status:** Proposto
- **Contexto:** precisamos de um caminho para ter `clojure.core` e o compilador
  funcionando sem cair em dependência circular nem reescrever toda a stdlib em Rust
  (start_spec §16/§30). A JVM só pode ser usada como **oracle de teste**, nunca no
  executável nem como parte obrigatória do bootstrap.

## Alternativas (start_spec §16)
1. `clojure.core` inteiro em Rust.
2. Só primitivas em Rust, o resto em Clojure.
3. Interpretador temporário.
4. Compilar a stdlib por estágios.
5. Implementação oficial só como oracle.
6. JVM só no desenvolvimento, nunca no binário.
7. Self-hosting progressivo.

## Decisão

**(2)+(3)+(4)+(5)+(7 progressivo)**: primitivas e núcleo de performance em Rust
(`clojure-core-native`); o máximo possível de `clojure.core` escrito em **Clojure**
(`clojure-core-clj`) e compilado pelo próprio compilador; um **interpretador de bootstrap**
(ADR-0004) cobre macros e código de topo; a JVM é **apenas oracle** (start_spec §22).

## Estágios (start_spec §16)

```text
Stage 0: reader + runtime mínimo em Rust                     (Fases 1,4)
Stage 1: formas especiais + funções primitivas (P/R)          (Fases 3,4)
Stage 2: macro evaluator (interpretador)                      (Fases 2,6)
Stage 3: clojure.core mínimo (macros base em Rust; C em .clj) (Fases 6,7)
Stage 4: compilador compila parte de si / da stdlib .clj       (Fase 7+)
Stage 5: stdlib majoritariamente em Clojure                    (Fases 7,8,12)
Stage 6: self-hosting parcial                                  [FUTURO]
```

### Regras anti-ciclo
- As macros/funcões **base** (necessárias para compilar `.clj`) existem primeiro em Rust;
  só depois são reescritas em Clojure, quando já há infraestrutura para compilá-las.
- `clojure-core-clj` é compilado em ordem: primeiro o que só depende de primitivas, depois
  camadas superiores (grafo de dependências — `clojure-loader`, Fase 9).

### Justificativa
- Evita o custo de reescrever a stdlib inteira em Rust e concretiza o **bootstrap
  progressivo** pedido no start_spec, movendo cada vez mais código para o próprio dialeto.
- Não propõe self-hosting imediato (start_spec §30) — é meta de longo prazo.

### Riscos (R10)
- Dependência circular core↔compilador → estágios explícitos + base em Rust.
- Divergência interp vs. nativo → mesmo runtime por baixo + differential testing.

## Consequências
- Dois crates de core: `clojure-core-native` (Rust) e `clojure-core-clj` (fonte `.clj`
  embutida). A fronteira entre eles **move-se** ao longo dos estágios (mais `.clj` com o
  tempo).
- O oracle (Clojure/JVM) é dependência **só de dev/CI** (ver TESTING_STRATEGY); o binário
  final não o toca.
