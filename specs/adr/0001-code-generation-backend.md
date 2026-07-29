# ADR-0001 — Backend de geração de código

- **Status:** Aceito e implementado (confirmado pelos protótipos #1 e #11; backend Cranelift AOT em produção)
- **Contexto:** precisamos gerar **código nativo autônomo** (sem JVM) para Linux e
  Windows, mantendo o projeto sustentável por equipe pequena e sem travar um futuro
  `eval`/REPL/JIT. Macros **não** dependem deste backend (rodam no interpretador —
  ADR-0004), então o backend serve à produção do binário final.

## Alternativas comparadas

| Backend | Vantagens | Desvantagens | Build | Portabilidade | Otimização | Complexidade |
| --- | --- | --- | --- | --- | --- | --- |
| **Cranelift** | pure-Rust, sem toolchain externo, compilação rápida, cross-compilation, serve p/ JIT futuro, API viva mas usável | otimização inferior ao LLVM, debuginfo (DWARF) em evolução, PDB fraco | rápido | boa (x64/arm64, Lin/Win/mac) | média | média |
| LLVM via `inkwell` | melhor codegen, DWARF/PDB maduros | dep pesada, churn de bindings, build lento/complexo, binário do compilador grande | lento | ótima | alta | alta |
| LLVM IR textual | controle total | frágil, verboso, mesmo peso do LLVM | lento | ótima | alta | alta |
| Geração de **C** | portátil, usa `cc` do sistema, ótimo DWARF/PDB, fácil de inspecionar/depurar, FFI trivial | exige toolchain C no build, TCO não garantida, build mais lento | médio/lento | excelente | alta (via cc) | média |
| Geração de **Rust** | reusa GC/coleções sem ABI | ciclo de build lento, ergonomia ruim p/ dinâmico, dep Rust no build | lento | ótima | alta | média/alta |
| WASM + runtime | sandbox, portátil | precisa runtime/embed, não é "nativo" direto, perf/startup | médio | boa | média | alta |
| VM/bytecode próprio | simples de emitir, portátil | não é nativo (interpretação), mais lento | rápido | ótima | baixa | média |
| Machine code próprio | zero deps | reinventar codegen p/ cada ISA | — | ruim | — | altíssima |

## Decisão

**Backend primário: Cranelift** (LIR → Cranelift IR → objeto; link via `lld`).
**Backend-C especificado como fallback/segundo alvo** para portabilidade máxima,
melhor debuginfo e otimização quando necessário.

### Justificativa (não por popularidade)
- A linguagem é **altamente dinâmica**; no MVP o gargalo é runtime/dispatch/coleções, **não**
  a qualidade do código aritmético — então a otimização superior do LLVM **não** está no
  caminho crítico (start_spec §6/§30: não escolher por otimização/popularidade).
- **Sustentabilidade p/ equipe pequena**: Cranelift é pure-Rust, sem gerenciar toolchain
  LLVM; build do compilador rápido e simples.
- **Cross-compilation** e **Windows 1ª classe** com objeto + `lld-link` (protótipo #11).
- **Futuro JIT/`eval`**: Cranelift é usado como JIT — o mesmo backend habilita
  desenvolvimento interativo depois, sem uma segunda tecnologia.
- **Stack traces** ao nível de fonte não dependem de DWARF (usamos frames lógicos do
  runtime — RUNTIME_SPEC), neutralizando a fraqueza de debuginfo do Cranelift no MVP.
- O **LIR é desacoplado** do backend, então adotar o backend-C (ou LLVM `[FUTURO]`) para
  release otimizado é evolução, não reescrita (mitiga R15).

### Riscos e mitigação
- Churn/limitações do Cranelift (R15) → backend-C fallback já especificado; abstração no
  limite do codegen.
- Debuginfo fraco → frames lógicos + source maps; DWARF melhora com o tempo.

## Consequências
- `clojure-codegen` depende de Cranelift; `clojure-linker` usa `lld`/link nativo.
- Protótipos #1 (exe mínimo) e #11 (link Windows) são **portão** para confirmar este ADR.
- Se #1/#11 falharem, reabrir com o backend-C como primário.
