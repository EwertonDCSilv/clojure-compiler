# ADR-0004 — Execução de macros

- **Status:** Proposto
- **Contexto:** macros de Clojure executam **código arbitrário em tempo de compilação**
  (não são substituição sintática — start_spec §30). Precisamos de um mecanismo para rodar
  `defmacro`, syntax-quote, `macroexpand`, e o código de topo necessário ao bootstrap,
  **sem JVM** e sem acoplar ao backend nativo.

## Alternativas (start_spec §7)
1. **Interpretador interno** (tree-walking) para expansão.
2. Compilação incremental das macros para nativo.
3. VM interna dedicada a macros.
4. Bootstrap de parte de `clojure.core` no mesmo motor.
5. Macros essenciais implementadas primeiro em Rust.
6. Abordagem híbrida.

## Decisão

**Interpretador tree-walking em build-time** (`clojure-interp`) para expansão de macros e
avaliação de topo — **(1)+(4)+(5)** combinados:
- macros essenciais de bootstrap (`defn`, `let`, `when`, `cond`, `and`, `or`, `->`,
  `lazy-seq`…) **primeiro em Rust**, migrando para Clojure conforme o core cresce (ADR-0005);
- `clojure.core` mínimo roda no **mesmo** interpretador (as macros do usuário chamam core);
- **sem** compilar macros para nativo no MVP (rejeita (2)/(3): complexidade e lentidão de
  build sem ganho necessário).

### Propriedades garantidas
- **Determinismo:** expansão independe de horário/PID/rede/ordem de hash não-determinística.
- **Isolamento/segurança:** sandbox nega I/O/rede/ambiente por padrão em build-time;
  filesystem só a caminhos do projeto declarados (R21).
- **Ordem de carga:** macro deve estar definida/avaliada antes do uso; namespaces em ordem
  topológica; dentro do arquivo, de cima para baixo.
- **Cache:** expansões/avaliações determinísticas cacheáveis por hash(fonte+versão+flags).
- **Cross-compilation:** sem host-interop ⇒ expansão independe do target.
- **`&form`/`&env`:** `&form` completo; `&env` mínimo no MVP (locais visíveis), documentado.

### Justificativa
- Simples, portátil, sem dependência do backend nem da JVM; fácil de sandboxar e tornar
  determinístico. É também a base natural para `eval`/REPL `[FUTURO]`.

### Riscos (R2, R10)
- Não-terminância de expansão → limite de profundidade + erro diagnóstico.
- Divergência entre semântica do interp e do código nativo → **mesmo** runtime/coleções
  por baixo (Fase 4) + differential testing.

## Consequências
- `clojure-macroexpander` orquestra o loop de expansão usando `clojure-interp`.
- Performance de builds macro-pesados: aceitável no MVP; **JIT Cranelift** para acelerar é
  `[FUTURO]`, reusando o mesmo backend (ADR-0001) — não comprometer o MVP com JIT.
