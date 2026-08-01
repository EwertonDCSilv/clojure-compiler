# ADR-0012 — Modularização dos crates Rust e controle de arquivos gigantes

- Status: **parcialmente implementada** (gate de tamanho de arquivo entregue em #109/#124 com baselines *grandfathered*; a extração/split dos módulos gigantes como `analyzer/lib.rs` e `codegen/lib.rs` continua pendente)
- Data: 2026-07-27
- Relacionadas: [ADR-0001](0001-code-generation-backend.md),
  [ADR-0004](0004-macro-execution.md),
  [ADR-0006](0006-codegen-optimization.md),
  [ADR-0010](0010-interprocedural-ephemeral-vectors.md) e
  [ADR-0011](0011-rust-crate-unit-testing-strategy.md)

## Contexto

O corte vertical do compilador cresceu primeiro em poucos arquivos. Isso reduziu o
custo para entregar reader, interpretador, analyzer, codegen e conformidade, mas alguns
arquivos agora misturam modelo de dados, parsing, transformação, integração com
processos e testes. Em 2026-07-27, os maiores são:

| Arquivo | Linhas | Responsabilidades hoje |
| --- | ---: | --- |
| `clojure-test-support/examples/generate_suite.rs` | 2.561 | DSL de fixtures e inventário A–E/I/O |
| `clojure-codegen/src/lib.rs` | 2.473 | opções, ABI, lowering, rooting e otimizações |
| `clojure-test-support/src/lib.rs` | 2.396 | schema, descoberta, execução, comparação, relatório e oracle |
| `clojure-analyzer/src/lib.rs` | 1.774 | AST, formas de topo, análise, primitivas e auto-transient |
| `clojure-interp/src/lib.rs` | 815 | estado, avaliação, funções e conversões |
| `clojure-reader/src/lib.rs` | 801 | cursor, tokens, coleções e reader macros |
| `clojure-native-cli/tests/e2e.rs` | 748 | vários contratos de CLI no mesmo arquivo |
| `clojure-interp/src/primitives.rs` | 689 | dispatch de primitivas |

Arquivos grandes não são um defeito por si só. O problema aparece quando uma alteração
exige conhecer responsabilidades não relacionadas, conflitos de merge se concentram,
testes ficam distantes da unidade e fronteiras de dependência permanecem implícitas.
Dividir apenas por quantidade de linhas, porém, cria módulos artificiais como
`part1.rs`/`part2.rs` e piora a navegação.

## Forças de decisão

- reduzir carga cognitiva e raio de mudança;
- tornar ownership, dependências e invariantes visíveis;
- preservar API pública, semântica, checksums e performance;
- permitir testes unitários próximos ao código conforme a ADR-0011;
- evitar ciclos e um número excessivo de crates;
- manter mudanças revisáveis e reversíveis;
- separar refatoração estrutural de novas features;
- impedir que os novos módulos voltem a crescer sem controle.

## Alternativas consideradas

| Alternativa | Vantagens | Desvantagens | Veredito |
| --- | --- | --- | --- |
| Manter arquivos monolíticos | nenhum churn imediato | custo e conflitos continuam crescendo | rejeitada |
| Dividir automaticamente por número de linhas | rápido | fronteiras arbitrárias, imports cruzados e baixa coesão | rejeitada |
| Criar um crate para cada componente interno | isolamento forte | grafo Cargo excessivo, API prematura e builds mais complexos | rejeitada |
| Reescrever cada crate na arquitetura final | desenho limpo no papel | alto risco, diff não revisável e regressões difíceis de localizar | rejeitada |
| Extrair módulos coesos dentro do crate, incrementalmente | preserva API e permite validação por etapa | exige ordem e testes de caracterização | **escolhida** |

## Decisão

Modularizar primeiro **dentro de cada crate**, por responsabilidade e direção de
dependência. Um novo crate só é criado quando existir pelo menos uma fronteira real:

- reutilização independente por dois ou mais crates;
- dependências externas diferentes e pesadas;
- ownership/lifecycle isolável;
- necessidade de compilação ou feature flag independente;
- API estável que faça sentido fora do crate original.

`lib.rs` torna-se a fachada: documentação do crate, declarações de módulo e `pub use` da
API suportada. `main.rs` faz composição e delega lógica. Tipos e funções continuam com
a menor visibilidade possível; mover código não é motivo para torná-lo público.

### Regras de fronteira

Os módulos seguem estas camadas, adaptadas a cada crate:

```text
modelo/tipos
    ↓
operações puras e validação
    ↓
transformações/pipeline
    ↓
adapters (Cranelift, processo, filesystem, CLI)
```

- Camadas inferiores não importam adapters.
- Tipos compartilhados vivem no módulo proprietário, não em um `utils.rs` genérico.
- Um módulo tem um motivo principal para mudar e nome de domínio; são proibidos
  `misc`, `common`, `helpers` amplo, `part1` e nomes equivalentes.
- Dependências cíclicas indicam fronteira incorreta. A solução preferida é mover o tipo
  estável para uma camada inferior ou introduzir uma interface estreita, não usar
  reexports para ocultar o ciclo.
- Reexports preservam caminhos públicos existentes durante a migração. Mudança pública
  incompatível exige decisão própria e não é parte desta ADR.
- Test helpers não entram em produção; ficam em
  `src/compiler/<crate>/tests/unit/<módulo>/mod.rs` ou no suporte compartilhado de testes.

### Limites de tamanho

Linhas são um sinal de revisão, não uma medida absoluta de qualidade. Adotam-se limites
graduais:

| Tipo de arquivo | Alvo | Revisão obrigatória | Limite para código novo |
| --- | ---: | ---: | ---: |
| módulo de produção | até 400 | acima de 400 | 800 |
| `lib.rs`/`main.rs` de fachada | até 250 | acima de 250 | 500 |
| teste de unidade/integração | até 600 | acima de 600 | 1.000 |
| gerador declarativo/fixtures | até 800 | acima de 800 | 1.200 |

Um arquivo novo não ultrapassa o limite sem justificativa arquitetural registrada na
revisão. Arquivos existentes acima dele são dívida **grandfathered**: não quebram a CI
imediatamente, mas não podem crescer em saldo líquido, salvo correção urgente acompanhada
de tracking para extração.

`scripts/check-rust-file-size.sh` verifies Rust source-file sizes using the versioned
`config/rust-file-size-baseline.json` allowlist. The gate fails when:

- um arquivo novo excede seu limite;
- um arquivo em dívida cresce além do baseline;
- uma exceção perde seu tracking;
- um módulo extraído viola a direção de dependência definida.

Comentários, testes e código claro não devem ser comprimidos para reduzir linhas. Se um
arquivo coeso ultrapassar o alvo, a revisão avalia complexidade, número de
responsabilidades e frequência de mudança antes de exigir divisão.

## Mapa inicial de módulos

Os nomes abaixo orientam a primeira extração; podem ser refinados sem alterar os
princípios desta ADR.

### `clojure-analyzer`

```text
src/
├── lib.rs              # fachada: analyze e reexports
├── ast.rs              # Ast, Prim, Function, Program e Dispatch
├── top_level.rs        # defn, defrecord, protocol, multimethod e ns
├── analyzer.rs         # Frame, slots, capturas e análise de expressões
├── arity.rs            # parâmetros, aridades e validação de chamadas
├── primitives.rs       # resolução e aridade das primitivas
├── optimizations/
│   ├── mod.rs
│   └── transients.rs   # análise linear e auto-transient
└── expand.rs           # expansão já isolada
```

O modelo da AST não depende do analyzer. Otimizações consomem e produzem AST, mas não
participam do reconhecimento das formas de topo.

### `clojure-codegen`

```text
src/
├── lib.rs              # compile_object* e tipos públicos
├── options.rs          # OptimizationLevel e CodegenOptions
├── value.rs            # tags, layouts e classificação VKind
├── runtime_abi.rs      # IDs e declarações das funções do runtime
├── module.rs           # criação/finalização do ObjectModule
├── function.rs         # FnGen, Flow e lowering de função
├── expression.rs       # lowering de Ast
├── roots.rs            # frames, slots e safepoints
├── primitives.rs       # fast/slow paths de Prim
└── constants.rs        # coleta e emissão de dados constantes
```

As constantes que precisam casar com `runtime.c` ficam numa única fronteira, com testes
de contrato. Detalhes Cranelift não vazam para o analyzer.

#### `ir_adapter` — extraído (issue #118)

```text
src/ir_adapter/
├── (mod.rs = src/ir_adapter.rs)  # optimize_program: orquestra as 4 etapas
├── escape.rs         # funções tomadas por valor (FnRef/MakeFn)
├── facts.rs           # ponto fixo interprocedural de fatos de fixnum
├── scalar_lowering.rs  # dobra constante de ilhas escalares via clojure-ir
└── specialization.rs   # infer_representation e especialização de Ast
```

`facts` depende de `escape` (função escapada é tratada de forma conservadora) e de
`specialization::infer_representation`/`loop_representations`. `scalar_lowering` e
`specialization` são independentes entre si e de `facts`/`escape`. A ordem de passes
em `optimize_program` (dobra escalar → fatos → especialização) é fixa e documentada no
`//!` do módulo-fachada.

### `clojure-test-support`

```text
src/
├── lib.rs              # fachada pública
├── manifest.rs         # schema, parsing e validação
├── discovery.rs        # descoberta e filtros
├── execution.rs        # processos, timeout, stdin/out/err
├── workspace.rs        # fixtures, symlinks e snapshots
├── comparison.rs       # texto, EDN e categorias de erro
├── checksum.rs
├── report.rs
└── oracle.rs

examples/generate_suite/
├── main.rs
├── fixture.rs          # DSL e escrita segura
├── level_a.rs
├── level_b.rs
├── level_c.rs
├── level_d.rs
├── level_e.rs
└── io.rs
```

Os casos permanecem declarativos. IDs, paths e manifestos são validados por uma única
camada, evitando que cada arquivo de nível replique regras de escrita.

### Demais crates

- `clojure-reader`: separar cursor/token, coleções, macros de reader e diagnósticos
  somente quando os testes de caracterização estiverem completos.
- `clojure-interp`: separar ambiente/estado, avaliação, funções/closures e conversões;
  `primitives.rs` pode ser dividido por família quando ultrapassar o limite.
- `clojure-native-cli`: manter parsing/configuração separado dos handlers; dividir E2E
  por `read`, `eval`, `build` e otimizações.
- crates pequenos (`span`, `diagnostics`, `syntax`, `value`) permanecem coesos até que
  responsabilidades reais justifiquem módulos; não serão fragmentados para uniformidade.

## Processo de extração

Cada extração é uma mudança mecânica e independente:

1. escrever ou confirmar testes de caracterização conforme ADR-0011;
2. escolher uma responsabilidade e seu conjunto mínimo de dependências;
3. mover código preservando nomes e visibilidade;
4. reexportar somente a API pública anterior;
5. executar formatação, testes do crate e cobertura;
6. executar integração proporcional ao componente;
7. comparar checksums e, quando o hot path puder mudar, performance;
8. só então iniciar a extração seguinte.

Não se combinam no mesmo commit:

- extração estrutural e nova feature;
- renomeação ampla e mudança semântica;
- alteração de API pública e movimentação interna;
- otimização e reorganização não necessária para ela.

Pequenos ajustes exigidos pelo borrow checker são aceitos quando preservam comportamento
e ficam evidentes no diff. Alteração de ownership, alocação, ordem de avaliação ou
tratamento de erro deixa de ser “somente modularização” e recebe teste e commit próprios.

## Ordem de adoção

1. Criar os testes de caracterização de analyzer, codegen e test-support.
2. Separar a DSL e os níveis do gerador da conformidade; é o maior arquivo e tem
   fronteiras declarativas claras.
3. Separar schema, execução, comparação e oracle de `clojure-test-support`.
4. Extrair AST, formas de topo e auto-transient de `clojure-analyzer`.
5. Extrair opções, ABI, valores/rooting e lowering de `clojure-codegen`.
   **Feito em parte** (issue #118): `ir_adapter` dividido em `escape`/`facts`/
   `scalar_lowering`/`specialization`. `options`/`value`/`runtime_abi`/`module`/
   `function`/`expression`/`roots`/`primitives`/`constants` seguem pendentes.
6. Reavaliar `reader`, `interp` e testes E2E usando métricas após as primeiras etapas.
7. Ativar o gate de tamanho com os baselines restantes na allowlist.

Essa ordem não implica mudança funcional. Se uma etapa revelar acoplamento sem teste,
ela pausa e volta à ADR-0011 antes de continuar.

## Validação

Toda extração executa:

```text
cargo test -p <crate>
make fmt-check
make lint
make coverage
```

Além disso:

- reader/analyzer/interp: `make compatibility`;
- analyzer/codegen: testes E2E e checksums dos benchmarks;
- codegen/runtime ABI: `make test-runtime` e sanitizers quando a ABI for tocada;
- test-support/gerador: regeneração da suíte sem diff inesperado e
  `scripts/conformance.sh verify`.

O tempo de compilação e o tamanho do binário são observados para evitar que a
modularização introduza dependências ou monomorfização acidental. Não se espera ganho de
runtime; checksums e performance devem permanecer equivalentes dentro do ruído.

## Critérios de aceite

- APIs públicas e caminhos documentados permanecem compatíveis;
- nenhuma mudança de feature ou semântica é misturada às extrações;
- módulos possuem responsabilidade nomeável e dependência acíclica;
- `lib.rs` e `main.rs` funcionam como fachadas/composição;
- arquivos novos respeitam os limites e dívida existente não cresce;
- testes da ADR-0011 antecedem módulos críticos extraídos;
- qualidade, cobertura, conformidade e integrações proporcionais permanecem verdes;
- checksums dos benchmarks não mudam; qualquer variação de performance é investigada;
- a allowlist de arquivos grandes diminui progressivamente.

## Consequências

Positivas:

- mudanças menores e mais localizadas;
- testes próximos às invariantes que protegem;
- menos conflitos em arquivos centrais;
- fronteiras de analyzer, codegen e infraestrutura tornam-se explícitas;
- dívida de tamanho passa a ser mensurável sem quebrar o projeto imediatamente.

Custos:

- churn inicial de paths e imports;
- possíveis conflitos com branches que alterem simultaneamente os arquivos monolíticos;
- manutenção do gate e da allowlist;
- modularização interna não resolve, por si só, APIs conceitualmente acopladas.
