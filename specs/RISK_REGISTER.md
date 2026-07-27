# RISK_REGISTER.md

Matriz de riscos. Prob./Impacto em Baixo/Médio/Alto. "Detecção" = como perceberemos cedo.
Cada risco liga-se a decisões/fases relevantes.

| # | Risco | Prob. | Impacto | Detecção | Mitigação |
| --: | --- | :--: | :--: | --- | --- |
| R1 | Complexidade excessiva da semântica de Clojure (escopo real maior que o previsto) | Alta | Alto | tarefas da Fase 3/7 estouram estimativa | escopo travado em LANGUAGE_SCOPE; fora-de-escopo vira **erro**, não silêncio; cortar por níveis A/B/C |
| R2 | Execução de macros (determinismo, isolamento, ordem de carga) | Alta | Alto | flakiness em `conformance/macros` | interpretador dedicado + sandbox + cache por hash (ADR-0004); macros base em Rust primeiro |
| R3 | Gerenciamento de memória (precisão de roots, vazamentos; ciclos quando lazy-seq chegar) | Alta | Alto | GC stress, retenção e checksums | shadow-stack precisa já entregue; mark-sweep não-móvel; avançar rooting por liveness (ADR-0002/0006) |
| R4 | Performance de chamadas dinâmicas / dispatch | Média | Médio | benchmarks Criterion | chamada direta quando estático; dispatch de protocol direto `[FUTURO]`; não otimizar cedo |
| R5 | Compatibilidade com bibliotecas (Nível D) menor que o esperado | Média | Médio | tentar compilar libs puras reais na Fase 12 | metas honestas (D é `[FUTURO]`); política de incompatibilidade clara |
| R6 | **Crescimento descontrolado de escopo** | Alta | Alto | backlog inchando; fases sem fim | "menor caminho até o binário" como norte; congelar subconjuntos por fase; ADRs travam decisões |
| R7 | Portabilidade / cross-compilation | Média | Médio | matriz CI | Linux oficial + Windows 1ª classe desde já; protótipo #11 |
| R8 | Suporte a Windows (MSVC/lld, linker, unwinding) | Média | Alto | protótipo #11 e CI Windows | Windows na matriz desde a Fase 0; lld-link; testar cedo, não tardiamente |
| R9 | Stack traces ao nível de fonte insuficientes | Média | Médio | casos de `conformance/errors` | frames lógicos no runtime (independe de DWARF); source maps do reader |
| R10 | Bootstrap (dependências circulares core↔compilador; ordem de estágios) | Média | Alto | Fase 6/7 travando | estágios explícitos (ADR-0005); JVM só como oracle; macros base em Rust |
| R11 | Diferenças semânticas difíceis de detectar (acidente vs. spec) | Média | Alto | differential testing + normalização | oracle + classificação spec/oficial/acidente; expected-diff declarados |
| R12 | Implementação incorreta de coleções persistentes | Média | Alto | property/fuzz vs. modelo de referência | não usar `Vec`/`HashMap` como substituto; testar contra modelo; protótipos #7/#8 |
| R13 | Bugs de concorrência | Baixa (MVP) | Médio | Loom onde há CAS | MVP single-thread; concorrência ampla `[FUTURO]` com Loom/safepoints |
| R14 | Uso excessivo/inseguro de `unsafe` | Média | Alto | revisão + Miri + auditoria | política de `unsafe` (abaixo); concentrar em `gc`/`value`/`ffi` |
| R15 | Dependência excessiva de **Cranelift** (churn de API, features faltando) | Média | Médio | quebras ao atualizar; recursos ausentes | backend-C fallback especificado (ADR-0001); LIR desacoplado do backend |
| R16 | Tempo de compilação do próprio compilador/projetos do usuário | Média | Médio | tempo de build no CI | Cranelift é rápido; medir; caching/incremental `[FUTURO]` |
| R17 | Tamanho dos executáveis | Baixa | Médio | métrica de tamanho no CI | link estático enxuto; medir; strip/otimização `[FUTURO]` |
| R18 | Manutenção por equipe pequena | Alta | Alto | velocity | módulos com fronteiras claras; specs vivas; decisões em ADR; testes como contrato |
| R19 | Divergência entre o `Value` Rust do interpretador e o `Value` tagged da ABI nativa | Média | Médio | diferenças interpreter/native | manter fronteiras explícitas e conformidade em ambos os caminhos |
| R20 | Panics do Rust vazando como "erros" da linguagem | Média | Médio | fuzz/asserts | fronteira captura panic → "internal compiler error"; nunca como exceção normal |
| R21 | Sandbox de macro insuficiente (I/O/rede em build-time) | Média | Médio | auditoria de FFI/IO no interp | negar por padrão; permitir só caminhos do projeto declarados |
| R22 | Oracle manual indisponível ou atualizado sem controle | Baixa | Médio | revisão de `oracle --check` | fixar Clojure 1.12.5; manter snapshots/checksums; CI nativa não depende da JVM |

## Política de `unsafe` (start_spec §25)

Todo bloco `unsafe` deve ter, sem exceção:
- **justificativa** (por que não dá para ser seguro);
- **invariantes** documentadas no comentário `// SAFETY:`;
- **testes** que exercitam o caminho (incluindo **Miri** quando aplicável);
- **revisão** por segundo par de olhos;
- **documentação** na API pública se a invariante vaza para o chamador.

No corte atual, a parte mais sensível fica no runtime C embutido em `clojure-codegen`.
Crates futuros de GC/FFI devem concentrar `unsafe`; os demais devem usar
`#![forbid(unsafe_code)]` quando possível. Miri se aplica ao Rust `unsafe`; ASan/UBSan
são necessários para o runtime C quando esses jobs forem adicionados.

## Segurança de supply chain e build

- Macros executam código em build-time ⇒ sandbox (R21); dependências de terceiros ⇒
  fixadas por lockfile + hash; builds reprodutíveis como meta; `cargo-deny`/audit no CI;
  não carregar bibliotecas dinâmicas de caminhos não confiáveis (FFI declarado no
  manifesto).
