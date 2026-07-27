# VISION.md

> Este documento descreve a visão de longo prazo. Para não confundir meta com suporte
> atual, consulte o snapshot executável em [README.md](README.md) e a matriz em
> [tests/conformance](../tests/conformance). Macros de usuário, namespaces
> multi-arquivo e FFI nativa citados abaixo ainda não foram entregues.

## Problema

Clojure hoje depende da JVM. Isso traz três custos que limitam categorias inteiras
de uso:

- **Startup** `[JVM]`: mesmo programas triviais pagam centenas de ms de boot da JVM
  (mitigado, mas não eliminado, por AppCDS/Babashka/GraalVM).
- **Distribuição** `[FATO]`: entregar uma aplicação Clojure significa entregar (ou exigir)
  um runtime Java; não há um único executável estático trivial.
- **Footprint**: memória residente e tamanho de imagem altos para CLIs e funções curtas.

Existem paliativos (Babashka para scripting via SCI; GraalVM Native Image), mas:
Babashka é interpretado e limitado ao seu conjunto embutido; GraalVM ainda parte do
bytecode JVM e de todo o toolchain Java. **Nenhum é um compilador nativo de Clojure
escrito de forma independente da JVM.**

## Proposta de valor

`clojure-native` compila um **subconjunto explicitamente definido** de Clojure
diretamente para código nativo, produzindo **um executável autônomo** que:

- inicia em poucos milissegundos;
- não requer JVM, JRE, `.class`, bytecode, Clojure instalado, nem GraalVM;
- distribui-se como arquivo único;
- preserva a semântica de Clojure dentro do escopo documentado (ver
  [LANGUAGE_SCOPE.md](LANGUAGE_SCOPE.md));
- oferece estruturas de dados persistentes reais e GC; no alvo completo, inclui macros
  de usuário e FFI nativa.

Não é uma transpilação superficial: é uma implementação real de linguagem com reader,
analisador, expansão de macros conhecidas, runtime, coleções persistentes e build
próprios. Namespaces multi-arquivo e macroexpansão de usuário continuam no roteiro (ver
[ARCHITECTURE.md](ARCHITECTURE.md)).

## Casos de uso prioritários (MVP)

1. **Ferramentas de linha de comando** — startup rápido, binário único.
2. **Automações / scripting** — substituto compilado de shell/Babashka para lógica pesada.
3. **Processamento de dados** batch de pequeno/médio porte.
4. **Funções serverless** — cold start baixo, imagem pequena.
5. **Pequenas aplicações server-side** sem dependência de frameworks JVM.
6. **Clojure em ambientes sem JVM** (containers mínimos, edge, dispositivos restritos).

## Fora de escopo (inicial, e alguns permanentes)

Não priorizados agora `[DECISÃO]`:

- Compatibilidade completa com bibliotecas Java / interop Java arbitrário.
- Reflexão Java, `proxy`, `gen-class`, `deftype` acoplado a interfaces Java.
- Frameworks fortemente acoplados à JVM (Spring-like, servlets, etc.).
- Swing/AWT/JavaFX e qualquer GUI dependente da JVM.
- Compatibilidade binária com bytecode Java / classpath / JARs / Maven como runtime.
- `core.async`, STM (`refs`/`dosync`), `agents` — adiados (ver
  [LANGUAGE_SCOPE.md](LANGUAGE_SCOPE.md)).
- REPL/`eval`/hot-reload completos — a arquitetura não os impede, mas não são MVP
  (ver [COMPILER_PIPELINE.md](COMPILER_PIPELINE.md) e RUNTIME_SPEC).

Permanentemente fora (por definição do projeto): empacotar/inicializar uma JVM embutida.

## Nível de compatibilidade desejado

Alvo do **MVP**: compatibilidade **Nível A + B + C parcial** (ver
[COMPATIBILITY_SPEC.md](COMPATIBILITY_SPEC.md)):

- **A (sintática)**: o reader lê Clojure idiomático puro.
- **B (semântica)**: as formas suportadas produzem comportamento equivalente ao `[JVM]`
  para valores/tipos/output/erros observáveis no subconjunto.
- **C (biblioteca)**: um subconjunto documentado de `clojure.core`, `clojure.string`,
  `clojure.set`, `clojure.walk`, `clojure.edn`.

Alvo de **longo prazo**: Nível D (bibliotecas Clojure puras compiláveis com pouca ou
nenhuma alteração). Nível E (ecossistema amplo) é aspiracional, **não** um compromisso.

## Diferenças inevitáveis em relação à JVM (declaradas desde já)

`[FATO]`/`[DECISÃO]`:

- **Inteiros**: sem semântica de `long`/`int`/overflow-para-`BigInt` idêntica de saída
  a menos que implementemos `BigInt` (ver LANGUAGE_SCOPE — `BigInt`/`BigDecimal` são
  pós-MVP). No MVP, inteiro = `i64` com política de overflow **explícita e documentada**.
- **Interop**: `(new java.util.Date)`, `.method`, `Class/staticField` **não existem**;
  geram erro de compilação claro com sugestão (ver
  [COMPATIBILITY_SPEC.md](COMPATIBILITY_SPEC.md)).
- **Exceções**: hierarquia de exceções é nativa do runtime, não `java.lang.Throwable`.
  `catch` casa por tipos do runtime, não por classes Java.
- **Threads/concorrência**: modelo de threads nativo (SO), não `java.llang.Thread`;
  STM/agents adiados.
- **Ordem de hash / iteração de mapas** pode divergir de detalhes acidentais da JVM;
  garantimos apenas o que a semântica especifica (ex.: `array-map` preserva ordem de
  inserção; `hash-map` não garante ordem).
- **`eval`/reflexão em runtime**: ausentes no MVP (AOT).

Todas as divergências conhecidas são catalogadas em
[COMPATIBILITY_SPEC.md](COMPATIBILITY_SPEC.md) — requisito de aceite do MVP (#18).

## Como o sucesso será medido

Métricas objetivas (linha de base a estabelecer nos protótipos da Fase 0):

| Métrica | Alvo MVP | Referência de comparação |
| --- | --- | --- |
| Startup do "hello world" nativo | < 10 ms | Clojure/JVM cold ~1 s `[JVM]`; Babashka ~10–20 ms |
| Tamanho do binário "hello world" | < 5 MB estático | — |
| Build limpo de projeto pequeno | segundos, não minutos | — |
| Conformidade differential (subconjunto MVP) | 100% dos casos declarados passam | oracle Clojure/JVM |
| Plataformas com binário verde no CI | Linux x86_64 **e** Windows x86_64 | — |
| Diagnósticos de erro | 100% com arquivo:linha:coluna | — |

Métrica qualitativa: outra equipe consegue iniciar a implementação a partir destas
specs **sem redescobrir** as decisões fundamentais (objetivo declarado no start_spec §31).

Critérios formais de conclusão do MVP: ver
[IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md#criterios-de-aceite-do-mvp) (18 critérios).
