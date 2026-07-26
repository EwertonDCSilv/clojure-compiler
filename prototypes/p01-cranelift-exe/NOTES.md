# Protótipo #1 — Cranelift → executável nativo (DESCARTÁVEL)

**Status: ✅ pergunta respondida — SIM.** Confirma ADR-0001 (Cranelift como backend AOT).

## Pergunta arquitetural
Conseguimos emitir um objeto com Cranelift e linká-lo em um executável nativo
autônomo que roda **sem JVM**?

## Resultado (Linux x86_64, 2026-07-26)
- Cranelift `0.124.3` compila e expõe a API de `ObjectModule`/`ObjectBuilder`.
- Geramos `main() -> i32` que chama `puts` (libc) e retorna 0.
- Objeto emitido: `proto.o` (904 bytes).
- Link via `cc` (clang 20) → `proto`.
- Execução:
  ```
  $ ./proto
  Hello from Cranelift (native, no JVM)
  $ echo $?
  0
  ```
- `file proto` → `ELF 64-bit LSB pie executable, dynamically linked`.
- `ldd proto` → `libc.so.6`, `ld-linux-x86-64.so.2`. **Nenhuma `libjvm`.**

## Aprendizados p/ produção
- Usar `symbol_value` (não `global_value`) para obter o ponteiro de um dado com PIC.
- `is_pic=true` + `cc` produz PIE — bom default.
- Ponteiros como `module.target_config().pointer_type()`.
- Próximos protótipos de backend: chamada indireta (#2), closures (#3) e link no
  Windows via `lld-link` (#11) — pendentes antes de codegen de produção (Fase 5).

## Não-produtivo
Este crate é isolado do workspace (`[workspace]` próprio; `exclude` na raiz) e **não**
deve virar código de produção. O codegen real (Fase 5) reusa os aprendizados, não o
código.
