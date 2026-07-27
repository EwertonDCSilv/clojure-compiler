# COMPATIBILITY_SPEC.md

Compatibilidade é declarada em **níveis** e por **política de incompatibilidade**.
Nunca afirmamos "compatível com Clojure" sem qualificar o nível e o subconjunto.

## Níveis de compatibilidade

### Nível A — Sintática
O reader de `clojure-native` lê o texto-fonte e produz as mesmas formas que o reader
oficial produziria (mesma estrutura de dados de leitura), para o subconjunto de
sintaxe suportado (ver Reader em [LANGUAGE_SCOPE.md](LANGUAGE_SCOPE.md)).
**Critério de teste:** para um corpus de `.clj` puro, o dump de formas do nosso reader
é estruturalmente igual ao de `(read-string ...)`/tools.reader do oracle, ignorando
apenas sintaxe fora do escopo (que deve gerar erro localizado, não silêncio).

### Nível B — Semântica
Cada forma suportada produz **comportamento observável equivalente** ao `[JVM]`:
mesmo valor, mesmo tipo lógico, mesmo output, mesmo *tipo* de erro, mesma ordem/efeitos.
Verificado por **differential testing** (ver [TESTING_STRATEGY.md](TESTING_STRATEGY.md)).
Divergências deliberadas (números, interop, etc.) são catalogadas abaixo.

### Nível C — Biblioteca padrão (subconjunto)
Funções selecionadas de `clojure.core` e alguns namespaces auxiliares estão disponíveis
com semântica de Nível B. O conjunto executável exato é o inventário `active` de
[`tests/conformance/level-c-stdlib`](../tests/conformance/level-c-stdlib); o alvo
progressivo está em [STANDARD_LIBRARY_SCOPE.md](STANDARD_LIBRARY_SCOPE.md).

### Nível D — Bibliotecas Clojure puras `[FUTURO]`
Bibliotecas de terceiros **sem** dependência de Java compilam com pouca ou nenhuma
alteração. Requer boa cobertura de `clojure.core` e do reader (reader conditionals,
regex, etc.). Não é meta do MVP.

### Nível E — Ecossistema amplo `[FUTURO/aspiracional]`
Grande parte do ecossistema. **Não é um compromisso**; serve de norte de longo prazo.

**Meta declarada do MVP: A + B + C (subconjunto).** A situação atual por recurso é
`active`, `xfail` ou `pending` na suíte executável. Ver
[VISION.md](VISION.md#nivel-de-compatibilidade-desejado).

---

## Divergências conhecidas em relação à JVM (catálogo)

Cada linha declara o comportamento oficial `[JVM]` e a decisão `[DECISÃO]` do projeto,
mais a **política** aplicada (ver seção seguinte).

| Área | `[JVM]` | `clojure-native` MVP | Política |
| --- | --- | --- | --- |
| Classes Java (`java.util.Date`, etc.) | disponíveis | inexistentes | **erro de compilação** + sugestão |
| Reflexão Java | disponível | inexistente | erro de compilação |
| Type hints Java (`^long`, `^String`, `^Foo`) | afetam boxing/reflexão | ignorados/erro se referem classe Java | warning (hint numérico) / erro (classe) |
| Java interop `.m` `Foo/bar` `(new ...)` | suportado | não suportado | erro de compilação + sugestão de API nativa |
| `proxy` | cria classe anônima | não suportado | erro de compilação |
| `gen-class` | gera `.class` | não suportado | erro de compilação |
| `deftype` c/ interfaces Java | suportado | só protocols nativos | erro se lista interface Java |
| Exceções Java (`catch IOException`) | classes JVM | tipos de exceção nativos | erro/mapeamento; ver Exceções |
| Threads da JVM (`Thread.`, `Thread/sleep`) | JVM | threads do SO via API nativa | substituição por API nativa |
| Classpath / JARs / Maven | runtime | não usados no runtime | N/A (só source deps; ver deps) |
| Carregamento dinâmico de classes | suportado | não suportado | erro |
| Inteiros: overflow de `long` | promove p/ exceção; `'`+ p/ BigInt | `i64` checado; sem BigInt no MVP | **feature flag futura** (`bigint`) |
| `(/ 1 2)` | `1/2` (Ratio) | erro "ratio não suportado" | erro (evita divergência silenciosa) |
| `(= 1 1.0)` | `false` | `false` (igual) | compatível |
| Ordem de iteração de `hash-map` | não especificada | pode diferir | compatível (não garantido dos dois lados) |
| Hash de valores (`hash`) | algoritmo específico `[JVM]` | algoritmo próprio | **divergente por design**; `=` preservado, `hash` numérico pode diferir |
| `eval` / REPL em runtime | presente | ausente (AOT) | erro / recurso `[FUTURO]` |

> Nota `[JVM]` vs. `[FATO acidental]`: alguns comportamentos oficiais (ordem exata de
> hash-map, valores de `hash`, formato de `toString` de coleções) são **acidentes da
> implementação**, não semântica especificada. Não os replicamos; garantimos apenas o
> que a semântica define. Isso deve ser destacado nos testes (não tratar acidente como
> spec — ver [TESTING_STRATEGY.md](TESTING_STRATEGY.md)).

---

## Política para incompatibilidades

Toda construção fora do escopo cai em **uma** destas categorias, nesta ordem de preferência:

1. **Erro de compilação (padrão)** — quando não há semântica nativa segura. Deve incluir
   arquivo:linha:coluna, o trecho, a causa e, quando possível, **sugestão** de alternativa
   nativa. Ex.: `(new java.util.Date)` → "interop Java não é suportado; use
   `clojure-native`'s `now` de `cljn.time` [FUTURO]".
2. **Warning** — quando a construção é inofensiva/ignorável sem mudar semântica.
   Ex.: type hint numérico `^long` (podemos aproveitar ou ignorar com aviso).
3. **Feature flag** — quando o recurso é implementável mas custoso e opcional.
   Ex.: `--feature bigint`, `--feature regex`. Desligado por padrão no MVP.
4. **Fallback** — quando há caminho degradado correto. Ex.: `array-map` grande promove
   automaticamente para `hash-map` (isto é comportamento, não incompatibilidade).
5. **Substituição por API nativa** — expor equivalente idiomático (ex.: threads do SO,
   tempo, I/O) em namespaces `cljn.*`, documentando a diferença de API.

Regra de ouro `[DECISÃO]`: **nunca silenciar** uma incompatibilidade produzindo um
resultado plausível-porém-diferente. Preferir erro alto e claro.

## Mecanismo técnico

- O analisador mantém uma tabela de "construções não suportadas" com a política e a
  mensagem associada; ao encontrá-las emite diagnóstico estruturado (ver
  [COMPILER_PIPELINE.md](COMPILER_PIPELINE.md#tratamento-de-erros) e a seção de erros).
- Feature flags entram no manifesto do projeto e na CLI (`clojure-native build
  --feature regex`), afetando reader/analyzer/stdlib disponíveis.
- Um documento gerado `COMPAT_REPORT` (saída de `clojure-native check`) lista todas as
  incompatibilidades detectadas no código do usuário — requisito de aceite do MVP (#18:
  "todas as incompatibilidades conhecidas documentadas").
