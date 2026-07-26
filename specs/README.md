# Especificações — `clojure-native`

Implementação nativa de Clojure em Rust: compila código-fonte Clojure para **binários nativos autônomos**, sem JVM em tempo de execução, sem bytecode `.class`, sem GraalVM como solução principal.

> **Status atual do repositório (fato observado, 2026-07-26):** greenfield.
> Conteúdo existente: `start_spec.md` (briefing) e diretórios de config vazios
> `.clj-kondo/`, `.lsp/`. **Não há** código, crates, git ou testes. Toolchain
> local detectado: Java 21 (usável só como *oracle* de teste), clang 20 / LLVM 20;
> **Rust/Cargo ainda não instalados**. Nada a reaproveitar além do briefing.

Estamos na **Fase 0 — Pesquisa e Especificação**. Nenhum código de produção deve
ser escrito antes que estas specs estejam aprovadas. Protótipos descartáveis são
permitidos e devem ser marcados como não-produtivos.

## Como ler estes documentos

Ordem sugerida de leitura:

1. [VISION.md](VISION.md) — problema, proposta de valor, escopo, métricas de sucesso.
2. [LANGUAGE_SCOPE.md](LANGUAGE_SCOPE.md) — matriz precisa do subconjunto da linguagem (MVP / depois / fora).
3. [COMPATIBILITY_SPEC.md](COMPATIBILITY_SPEC.md) — níveis A–E de compatibilidade e política de incompatibilidades.
4. [ARCHITECTURE.md](ARCHITECTURE.md) — crates, fronteiras e fluxo de dados.
5. [COMPILER_PIPELINE.md](COMPILER_PIPELINE.md) — reader → macroexpand → analyzer → IR → codegen.
6. [RUNTIME_SPEC.md](RUNTIME_SPEC.md) — representação de valores, funções, Vars, protocols, seqs.
7. [MEMORY_MODEL.md](MEMORY_MODEL.md) — modelo de GC, roots, ownership no runtime.
8. [STANDARD_LIBRARY_SCOPE.md](STANDARD_LIBRARY_SCOPE.md) — quais namespaces/funções e onde são implementados.
9. [NATIVE_INTEROP.md](NATIVE_INTEROP.md) — FFI C ABI.
10. [TESTING_STRATEGY.md](TESTING_STRATEGY.md) — differential testing contra Clojure/JVM como oracle.
11. [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) — plano incremental verificável, Fases 0–12.
12. [RISK_REGISTER.md](RISK_REGISTER.md) — riscos, probabilidade, impacto, mitigação.

Decisões arquiteturais fundamentais (imutáveis sem novo ADR):

- [adr/0001-code-generation-backend.md](adr/0001-code-generation-backend.md) — **Cranelift** (AOT) + backend-C fallback.
- [adr/0002-memory-management.md](adr/0002-memory-management.md) — **GC tracing precisa mark-sweep** com shadow-stack roots.
- [adr/0003-value-representation.md](adr/0003-value-representation.md) — **enum `Value`** com imediatos + `Gc<T>`.
- [adr/0004-macro-execution.md](adr/0004-macro-execution.md) — **interpretador tree-walking** em tempo de compilação.
- [adr/0005-bootstrap-strategy.md](adr/0005-bootstrap-strategy.md) — primitivas em Rust + `clojure.core` progressivo em Clojure.

Testes de conformidade: [conformance/README.md](conformance/README.md).

## Convenções

- **Marcações de confiança** usadas em todo o texto:
  `[FATO]` observado/verificável · `[JVM]` comportamento da implementação oficial ·
  `[DECISÃO]` escolha proposta neste projeto · `[HIPÓTESE]` a validar ·
  `[RISCO]` · `[FUTURO]` fora do MVP.
- Nome do produto/binário: `clojure-native`.
- Estas specs são **vivas**: cada fase do [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)
  pode refiná-las. Toda mudança de decisão fundamental exige um novo ADR (nunca editar
  um ADR aceito — cria-se um sucessor que o marca como *Superseded*).

## Resumo executivo das decisões

| Área | Decisão MVP | Alvo de longo prazo | ADR |
| --- | --- | --- | --- |
| Backend de codegen | Cranelift (objeto + link via `lld`) | + backend C p/ portabilidade/otimização | 0001 |
| Execução de macros | Interpretador tree-walking em build-time | JIT Cranelift opcional | 0004 |
| Representação de valor | `enum Value` (imediatos) + `Gc<T>` | tagged pointers / NaN-boxing | 0003 |
| Memória | Mark-sweep precisa, não-móvel, shadow-stack roots, single-thread | Geracional/móvel (MMTk) | 0002 |
| Bootstrap | Primitivas Rust + core em Clojure, staged | Self-hosting parcial | 0005 |
| Coleções | array-map + HAMT + bitmapped vector trie | CHAMP + sorted + transients | RUNTIME_SPEC |
| Plataforma inicial | Linux x86_64 (oficial) + Windows x86_64 (1ª classe) | + arm64, macOS | ARCHITECTURE |

O **menor caminho até o primeiro binário nativo** está descrito no fim de
[IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md#menor-caminho-ate-o-primeiro-binario).
