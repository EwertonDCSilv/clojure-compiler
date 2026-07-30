# ADR-0011 — Estratégia de testes unitários para os crates Rust

- Status: **accepted; implementada** (harnesses de unidade por crate em uso)
- Data: 2026-07-27
- Relacionadas: [ADR-0004](0004-macro-execution.md),
  [ADR-0006](0006-codegen-optimization.md),
  [ADR-0007](0007-native-io-and-runtime-reader.md),
  [ADR-0008](0008-associative-indexed-dispatch.md) e
  [ADR-0012](0012-rust-crate-modularization.md)

## Contexto

O workspace possui testes unitários, testes de integração da CLI, testes do runtime C,
conformidade A–E e benchmarks com checksum. Essa combinação protege o comportamento
externo do compilador, mas não torna explícito quais funções e invariantes internas de
cada crate estão protegidas. Uma regressão pequena pode aparecer somente no fim do
pipeline, com diagnóstico distante da causa.

O snapshot de 2026-07-27 contém 125 funções marcadas com `#[test]`, sem contar doctests
ou os casos gerados da conformidade:

| Crate | Linhas Rust | Testes `#[test]` | Observação |
| --- | ---: | ---: | --- |
| `clojure-span` | 255 | 6 | tipos de posição e fonte |
| `clojure-diagnostics` | 207 | 4 | renderização e ordenação |
| `clojure-syntax` | 278 | 4 | formas e metadata |
| `clojure-reader` | 801 | 13 | reader concentrado em um arquivo |
| `clojure-value` | 538 | 9 | valores e coleções do interpretador |
| `clojure-interp` | 1.722 | 18 | avaliação, primitivas e bootstrap |
| `clojure-analyzer` | 2.032 | 11 | análise e otimizações de AST |
| `clojure-codegen` | 2.649 | 10 | codegen Rust e integração do runtime C |
| `clojure-native-cli` | 1.035 | 28 | principalmente testes end-to-end |
| `clojure-test-support` | 5.115 | 22 | runner, oracle e gerador de fixtures |

Contagem bruta não mede qualidade: um único teste pode atravessar muitas funções sem
exercitar seus limites, enquanto getters triviais não justificam um teste exclusivo.
Também é possível aumentar cobertura executando linhas sem verificar o resultado.

## Forças de decisão

- localizar regressões no crate e na função responsáveis;
- cobrir caminhos de sucesso, limites e falhas, não apenas linhas executadas;
- preservar testes de integração e conformidade como contratos externos;
- permitir a modularização da [ADR-0012](0012-rust-crate-modularization.md) com uma rede
  de caracterização anterior às extrações;
- manter testes determinísticos, rápidos e independentes de rede ou JVM;
- não ampliar APIs públicas apenas para torná-las testáveis;
- não incentivar testes sem valor para satisfazer uma porcentagem;
- proteger especialmente parsing, lowering, otimizações, rooting e fronteiras unsafe.

## Alternativas consideradas

| Alternativa | Vantagens | Desvantagens | Veredito |
| --- | --- | --- | --- |
| Confiar apenas em E2E e conformidade | contrato externo forte | falhas lentas e pouco localizadas; combina muitos componentes | insuficiente |
| Exigir um teste exclusivo para toda função | regra simples de contar | testa detalhes triviais, força APIs artificiais e não garante assertivas úteis | rejeitada |
| Cobertura global como único gate | automatização já existente | crates fortes escondem módulos fracos; linha coberta não implica comportamento verificado | insuficiente |
| Unitários por responsabilidade + integração nas fronteiras + cobertura com ratchet | falha localizada e contrato externo preservado | exige inventário e manutenção de fixtures | **escolhida** |

## Decisão

Toda função de produção **não trivial** deve ter seu comportamento coberto de uma das
seguintes formas:

1. teste unitário direto no módulo que contém a função; ou
2. teste do contrato público do módulo que atravesse a função e verifique sua saída ou
   efeito observável.

Não é obrigatório criar um teste por getter, conversão mecânica ou delegação sem lógica.
Nesses casos, a cobertura pelo contrato do tipo é suficiente. A revisão deve conseguir
relacionar cada branch relevante da função a pelo menos um cenário; a mera execução da
linha não é evidência bastante.

### Categorias mínimas de casos

Aplicam-se conforme a natureza da função:

| Categoria | Casos exigidos |
| --- | --- |
| Função pura | partição normal, valores de fronteira e entradas vazias |
| Função falível | sucesso e ao menos um caso para cada categoria de erro alcançável |
| Parser/decoder | entrada válida mínima e composta, EOF/truncamento, token inválido e posição do erro |
| Estado mutável | estado inicial, transição válida, repetição/idempotência quando prometida e transição recusada |
| Coleção/algoritmo | vazio, um elemento, crescimento, colisão/duplicata e invariantes após a operação |
| Transformação de AST/IR | caso que transforma, caso conservador que não transforma e equivalência semântica |
| Aritmética/índice | zero, limites, overflow/underflow e valores inválidos |
| Processo/filesystem | sucesso, exit code/erro, timeout e cleanup do diretório temporário |
| Unsafe/FFI/ABI | sucesso, nulidade/tamanho inválido, ownership/lifecycle e sanitizers quando aplicável |

Uma correção de bug começa com um teste que falha pelo motivo correto e permanece como
regressão. Uma otimização precisa testar tanto o padrão reconhecido quanto um padrão
parecido que **não** pode ser transformado. Para transforms semânticos, checksum de
benchmark complementa, mas não substitui, os testes negativos.

### Localização dos testes

- Os corpos dos testes unitários ficam fora da implementação, em
  `src/compiler/<crate>/tests/unit/<módulo>/mod.rs`, espelhando o caminho de `src/<módulo>.rs`.
  O arquivo de produção mantém apenas uma declaração `#[cfg(test)]` com `#[path]`,
  para que o teste continue sendo um módulo filho e preserve acesso a invariantes
  privados sem ampliar a API.
- Contratos públicos entre crates ficam em `src/compiler/<crate>/tests/*.rs`; suítes de
  integração existentes não devem ser misturadas com `tests/unit/`.
- Testes que compilam e executam um programa pertencem à integração do codegen/CLI, não
  ao módulo de parsing que fornece a entrada.
- Doctests servem para exemplos curtos de API pública; não carregam matrizes extensas.
- Fixtures compartilhadas ficam em um módulo `test_support` privado ou no crate
  `clojure-test-support`; não entram na API de produção por conveniência.
- Nenhum item muda para `pub` somente para um teste. Quando necessário, o teste reside
  no módulo filho ou usa visibilidade `pub(crate)` justificada pela arquitetura real.

### Forma dos testes

Os nomes seguem `operacao_cenario_resultado`, por exemplo
`parse_params_ampersand_without_name_reports_error`. Cada teste deixa distinguíveis
arranjo, execução e verificação, mesmo sem comentários formais.

Casos homogêneos usam tabelas locais para reduzir repetição, mas cada entrada inclui um
rótulo que aparece na falha. Assertivas comparam o valor estrutural completo e os
campos relevantes do erro; `is_err()` sozinho é insuficiente quando a categoria ou span
faz parte do contrato.

Snapshots são aceitos para diagnósticos, AST/IR estável e relatórios extensos, desde que:

- o formato seja determinístico;
- valores voláteis sejam normalizados;
- a revisão mostre a alteração do snapshot;
- exista uma assertiva estrutural separada para invariantes que não podem desaparecer
  numa grande diferença textual.

### Isolamento e determinismo

Testes unitários não acessam a rede, não exigem JVM e não dependem da ordem de execução.
Relógio, ambiente, CWD, processos e filesystem entram por uma fronteira controlável ou
por diretório temporário exclusivo. Variáveis de ambiente modificadas são restauradas
mesmo após falha; testes que inevitavelmente alterem estado global são serializados e
documentam o motivo.

Randomização usa seed fixa exibida na falha. Testes não usam `sleep` como sincronização;
processos usam timeout explícito. Um teste ignorado exige motivo e tracking; `#[ignore]`
não é uma forma permanente de aceitar regressão.

### Estratégia por crate

- `clojure-span`, `clojure-diagnostics` e `clojure-syntax`: invariantes de tipos,
  ordenação, spans, metadata e renderização.
- `clojure-reader`: unidades do cursor/token, delimitadores, reader macros, Unicode e
  diagnósticos localizados; o corpus de conformidade continua como diferencial externo.
- `clojure-value`: igualdade, hash, impressão, sequências, aridade e persistência das
  estruturas usadas pelo interpretador.
- `clojure-interp`: ambiente léxico, controle (`recur`/erro), cada forma especial,
  dispatch de primitivas, macroexpansão e bootstrap.
- `clojure-analyzer`: reconhecimento de formas, slots/capturas, aridades, lowering de
  primitivas e testes positivo/negativo para cada otimização.
- `clojure-codegen`: opções, declaração da ABI, classificação de valores e lowering
  inspecionável em unidade; execução do objeto gerado e runtime C em integração.
- `clojure-native-cli`: parsing de argumentos e seleção de comandos em unidade; build,
  link e execução em E2E.
- `clojure-test-support`: schema, segurança de paths, filtros, processos, comparação,
  snapshots, checksums e oracle; o gerador valida unicidade dos IDs e completude.

### Cobertura e gates

`scripts/coverage.sh` writes `target/coverage/coverage.json` and
`target/coverage/summary.json` for every run. The versioned
[`config/coverage-baseline.json`](../../config/coverage-baseline.json) records all
crate metrics plus the initially high-risk modules. A report parser rejects a lower
line, function, or region metric for every recorded entry.

The existing aggregate gates remain blocking:

- linhas globais: 82%;
- funções globais: 82%;
- regiões globais: 82%;
- linhas por arquivo: 30%.

Those values are a floor, not the final objective. New or modified executable Rust
lines must reach 90% line coverage in the Git diff. Comments, blank lines, and lines
without an LLVM coverage segment do not count in that denominator. Defensive or
platform branches remain executable and must be tested; they are not a silent
exclusion.

Exclusões não são silenciosas: cada uma informa arquivo, razão, plataforma e issue. Não
se adicionam testes sem assertiva relevante nem se duplica o mesmo caminho apenas para
recuperar porcentagem.

### Pirâmide de execução

1. Durante desenvolvimento: `cargo test -p <crate>`.
2. Antes do commit: `make quality`.
3. Gate bloqueante: `make coverage`.
4. Mudanças de reader, analyzer ou semântica: `make compatibility`.
5. Mudanças de analyzer/codegen/runtime: checksums dos benchmarks e testes do runtime;
   medições completas somente quando a execução ou performance puder mudar.

Testes unitários devem continuar sendo a camada mais numerosa e rápida. Integração
valida fronteiras reais; conformidade valida a linguagem; benchmark valida checksum e
performance. Uma camada não substitui a anterior.

## Plano de adoção

1. Gerar inventário por crate de funções, branches e testes existentes.
2. Congelar comportamento dos módulos a extrair na ADR-0012 com testes de
   caracterização.
3. Cobrir primeiro analyzer, codegen e test-support, onde tamanho e risco são maiores.
4. Separar testes monolíticos por responsabilidade junto com a modularização, sem mudar
   as assertivas.
5. Publish coverage by crate and establish ratchet baselines. **Completed:** issue #108.
6. Add the diff-coverage gate after the baseline is stable. **Completed:** issue #108.

Cada etapa contém apenas testes, fixtures ou refatorações necessárias à testabilidade.
Novas features são entregas separadas.

## Critérios de aceite

- `make quality`, `make coverage` e `make compatibility` verdes;
- todo módulo de produção possui unidade correspondente ou justificativa de cobertura
  integral pela API pública;
- funções falíveis cobrem suas categorias de erro alcançáveis;
- cada otimização tem caso positivo, negativo e verificação semântica;
- testes não dependem de rede/JVM, ordem, CWD compartilhado ou sleeps;
- nenhuma API pública é ampliada somente para teste;
- cobertura por crate é publicada e não regride abaixo do ratchet;
- alterações de testes não introduzem features do compilador.

## Consequências

Positivas:

- regressões passam a falhar perto da causa;
- modularizações podem ser feitas com segurança;
- gaps de analyzer, codegen e infraestrutura deixam de ser mascarados pelo total global;
- cobertura mede cenários e invariantes, não apenas execução acidental.

Custos:

- manutenção de builders, fixtures e snapshots;
- tempo inicial para inventário e caracterização;
- alguns comportamentos continuarão exigindo integração por dependerem de Cranelift,
  linker, processos ou runtime C.
