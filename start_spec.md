Quero projetar uma implementação nativa de Clojure escrita em Rust.

O objetivo é criar um compilador capaz de receber código-fonte Clojure e gerar executáveis binários nativos autônomos, sem depender da JVM durante a execução.

Neste momento, sua tarefa não é implementar o compilador. Sua tarefa é investigar o problema, definir o escopo da linguagem, elaborar a especificação técnica e produzir um plano de implementação incremental e verificável.

## Objetivo geral

Projetar uma implementação de Clojure que:

* tenha o compilador escrito predominantemente em Rust;
* compile código Clojure para binários nativos;
* não necessite da JVM em tempo de execução;
* não empacote nem inicialize uma JVM dentro do executável;
* preserve, dentro de um escopo explicitamente definido, a semântica de Clojure;
* ofereça estruturas de dados persistentes;
* suporte macros;
* implemente gerenciamento automático de memória;
* tenha interoperabilidade com bibliotecas nativas;
* possa evoluir progressivamente até suportar uma parcela relevante do ecossistema Clojure.

O artefato final esperado deve se comportar aproximadamente assim:

```bash
clojure-native build src/main.clj --output my-program
./my-program
```

No Windows:

```powershell
clojure-native.exe build src/main.clj --output my-program.exe
.\my-program.exe
```

O executável produzido não pode depender de:

* JVM;
* Java Runtime Environment;
* arquivos `.class`;
* bytecode JVM;
* inicialização de uma JVM embutida;
* instalação prévia de Clojure;
* GraalVM Native Image como implementação principal.

GraalVM pode ser analisado apenas como referência comparativa, não como solução para o projeto.

## Regra fundamental

Não trate este projeto como uma simples transpilerização superficial.

O projeto deve ser planejado como uma implementação real de linguagem, contendo:

* reader;
* representação de formulários;
* analisador semântico;
* expansão de macros;
* sistema de namespaces;
* compilador;
* runtime nativo;
* estruturas persistentes;
* gerenciamento de memória;
* modelo de chamada de funções;
* tratamento de erros;
* biblioteca padrão;
* sistema de build;
* distribuição de executáveis.

Não comece a escrever o compilador antes de concluir a especificação.

## Entregáveis

Crie os seguintes documentos:

```text
docs/
├── VISION.md
├── LANGUAGE_SCOPE.md
├── COMPATIBILITY_SPEC.md
├── ARCHITECTURE.md
├── COMPILER_PIPELINE.md
├── RUNTIME_SPEC.md
├── MEMORY_MODEL.md
├── STANDARD_LIBRARY_SCOPE.md
├── NATIVE_INTEROP.md
├── TESTING_STRATEGY.md
├── IMPLEMENTATION_PLAN.md
├── RISK_REGISTER.md
└── adr/
    ├── 0001-code-generation-backend.md
    ├── 0002-memory-management.md
    ├── 0003-value-representation.md
    ├── 0004-macro-execution.md
    └── 0005-bootstrap-strategy.md
```

Caso o repositório já possua uma estrutura documental, adapte os caminhos, mas preserve a separação conceitual.

---

# 1. Investigação inicial

Antes de escrever a especificação:

1. Inspecione completamente o repositório atual.
2. Identifique:

   * código existente;
   * experimentos anteriores;
   * decisões arquiteturais;
   * crates já utilizados;
   * targets suportados;
   * testes;
   * documentação;
   * código legado;
   * componentes reaproveitáveis.
3. Pesquise como Clojure implementa:

   * reader;
   * compiler;
   * macro expansion;
   * Vars;
   * namespaces;
   * persistent collections;
   * sequences;
   * multimethods;
   * protocols;
   * metadata;
   * dynamic binding;
   * lazy evaluation;
   * exceptions;
   * concurrency primitives.
4. Separe claramente:

   * comportamento especificado da linguagem;
   * comportamento específico da implementação JVM;
   * comportamento acidental da implementação oficial;
   * funcionalidades que dependem diretamente do ecossistema Java.

Não presuma que toda característica da implementação JVM precisa existir no primeiro release.

---

# 2. `VISION.md`

O documento deve explicar:

* qual problema o projeto resolve;
* qual é a proposta de valor;
* quais casos de uso são prioritários;
* quais casos de uso estão fora do escopo;
* qual nível de compatibilidade com Clojure é desejado;
* quais são as diferenças inevitáveis em relação à implementação JVM;
* como será medido o sucesso do projeto.

Considere como casos de uso iniciais:

* ferramentas de linha de comando;
* aplicações server-side pequenas;
* automações;
* processamento de dados;
* funções serverless;
* programas distribuídos como executável único;
* aplicações que precisam de startup rápido;
* uso de Clojure em ambientes sem JVM.

Não priorize inicialmente:

* compatibilidade completa com bibliotecas Java;
* aplicações Swing;
* bibliotecas dependentes de reflexão Java;
* frameworks fortemente acoplados à JVM;
* compatibilidade binária com bytecode Java.

---

# 3. `LANGUAGE_SCOPE.md`

Defina precisamente quais elementos da linguagem serão suportados.

Crie uma matriz com colunas:

| Recurso | MVP | Versão posterior | Fora do escopo inicial | Observações |
| ------- | --: | ---------------: | ---------------------: | ----------- |

Analise, no mínimo:

## Reader

* números inteiros;
* números de ponto flutuante;
* ratios;
* BigInt;
* BigDecimal;
* strings;
* caracteres;
* símbolos;
* keywords;
* listas;
* vetores;
* mapas;
* sets;
* metadata;
* quoting;
* syntax quote;
* unquote;
* unquote-splicing;
* reader conditionals;
* tagged literals;
* anonymous function literals;
* regex literals;
* discard forms;
* namespace map syntax.

## Formas especiais

* `if`;
* `do`;
* `let`;
* `letfn`;
* `fn`;
* `def`;
* `quote`;
* `var`;
* `set!`;
* `loop`;
* `recur`;
* `throw`;
* `try`;
* `catch`;
* `finally`;
* `new`, somente se existir equivalente nativo;
* `monitor-enter` e `monitor-exit`, provavelmente fora do MVP.

## Funções

* funções de primeira classe;
* closures;
* múltiplas aridades;
* funções variádicas;
* destructuring;
* apply;
* partial;
* comp;
* higher-order functions;
* tail recursion com `recur`.

## Namespaces

* `ns`;
* `require`;
* `use`, caso seja mantido;
* `refer`;
* aliases;
* resolução de símbolos;
* Vars privadas;
* carregamento de módulos;
* compilação separada.

## Macros

* `defmacro`;
* expansão de macros;
* `macroexpand`;
* `macroexpand-1`;
* syntax quote;
* gensym;
* macros definidas pelo usuário;
* macros entre namespaces;
* execução de macros durante a compilação.

## Estruturas de dados

* persistent vector;
* persistent hash map;
* persistent array map;
* persistent hash set;
* persistent list;
* cons;
* seq;
* lazy seq;
* queue;
* sorted map;
* sorted set;
* transients.

## Abstrações

* protocols;
* records;
* types;
* metadata;
* multimethods;
* hierarchies;
* interfaces nativas equivalentes, caso existam.

## Estado e concorrência

* atoms;
* volatile;
* delays;
* promises;
* futures;
* refs;
* STM;
* agents;
* dynamic Vars;
* thread-local bindings;
* locking;
* channels, caso core.async seja futuramente suportado.

Defina o que será MVP e o que será adiado.

---

# 4. `COMPATIBILITY_SPEC.md`

Defina níveis de compatibilidade.

Considere uma classificação como:

### Nível A — Compatibilidade sintática

O código pode ser lido pelo novo reader.

### Nível B — Compatibilidade semântica

As formas suportadas produzem comportamento equivalente.

### Nível C — Compatibilidade de biblioteca padrão

Funções selecionadas de `clojure.core` estão disponíveis.

### Nível D — Compatibilidade de bibliotecas Clojure puras

Bibliotecas sem dependência de Java podem ser compiladas com poucas ou nenhuma modificação.

### Nível E — Compatibilidade ampla com ecossistema

Objetivo de longo prazo, não necessário para o MVP.

Documente explicitamente as divergências relacionadas a:

* classes Java;
* reflexão;
* type hints Java;
* Java interop;
* `proxy`;
* `gen-class`;
* `deftype` associado a interfaces Java;
* exceções Java;
* threads da JVM;
* classpath;
* Maven;
* JARs;
* carregamento dinâmico de classes.

Crie uma política para incompatibilidades:

* erro de compilação;
* warning;
* feature flag;
* fallback;
* substituição por API nativa.

---

# 5. `ARCHITECTURE.md`

Proponha uma arquitetura modular em Rust.

Considere uma estrutura semelhante a:

```text
crates/
├── clojure-native-cli/
├── clojure-reader/
├── clojure-syntax/
├── clojure-analyzer/
├── clojure-macroexpander/
├── clojure-ir/
├── clojure-codegen/
├── clojure-runtime/
├── clojure-core/
├── clojure-persistent/
├── clojure-gc/
├── clojure-loader/
├── clojure-ffi/
├── clojure-diagnostics/
└── clojure-test-support/
```

A estrutura é apenas uma referência. Ajuste-a de acordo com a análise real.

Para cada crate, defina:

* responsabilidade;
* API pública;
* dependências permitidas;
* dependências proibidas;
* invariantes;
* tipos principais;
* estratégia de testes.

Evite dependências circulares.

Defina também as fronteiras entre:

```text
Source
  ↓
Reader
  ↓
Parsed Forms
  ↓
Macro Expansion
  ↓
Semantic Analysis
  ↓
Typed or Annotated IR
  ↓
Optimized IR
  ↓
Native Code Generation
  ↓
Linker
  ↓
Standalone Executable
```

---

# 6. Backend de geração de código

Compare detalhadamente estas alternativas:

* LLVM via `inkwell`;
* LLVM por geração direta de LLVM IR;
* Cranelift;
* geração de C intermediário;
* geração de Rust intermediário;
* WebAssembly com runtime nativo;
* bytecode próprio com máquina virtual;
* combinação entre AOT e runtime interpretado;
* backend próprio de machine code.

Crie uma tabela:

| Backend | Vantagens | Desvantagens | Build | Portabilidade | Otimização | Complexidade |
| ------- | --------- | ------------ | ----- | ------------- | ---------- | ------------ |

Avalie especialmente:

* tempo de compilação;
* qualidade do código nativo;
* portabilidade;
* debugging;
* stack traces;
* suporte a Windows;
* suporte a Linux;
* suporte a macOS;
* cross-compilation;
* tamanho dos binários;
* manutenção;
* geração de informações DWARF/PDB;
* suporte a JIT futuro;
* estabilidade das APIs em Rust.

Escolha um backend recomendado.

Registre a decisão em:

```text
docs/adr/0001-code-generation-backend.md
```

Não escolha LLVM ou Cranelift apenas por popularidade. Justifique com base nas necessidades específicas da linguagem.

---

# 7. `COMPILER_PIPELINE.md`

Especifique detalhadamente o pipeline do compilador.

## Reader

Defina:

* tokenizer;
* parser;
* source spans;
* preservação de posição;
* mensagens de erro;
* recuperação de erros;
* representação de símbolos e keywords;
* internamento de símbolos;
* metadata;
* tagged literals;
* feature flags.

## Macro expansion

Defina como macros serão executadas durante a compilação.

Compare:

1. interpretador interno para macro expansion;
2. compilação incremental das macros;
3. execução de macros em uma VM interna;
4. bootstrap de parte de `clojure.core`;
5. macros implementadas inicialmente em Rust;
6. abordagem híbrida.

Explique:

* ordem de carregamento;
* cache;
* isolamento;
* segurança;
* determinismo;
* acesso ao filesystem;
* acesso ao ambiente;
* versionamento;
* compilação cruzada;
* macros que dependem do host.

Registre a decisão em:

```text
docs/adr/0004-macro-execution.md
```

## Analisador semântico

Defina:

* lexical scope;
* resolução de Vars;
* namespaces;
* locals;
* captures;
* aridades;
* recur targets;
* validação de tail position;
* type hints;
* metadata semântica;
* checagem de formas especiais;
* desugaring.

## Representação intermediária

Compare:

* AST anotada;
* high-level IR;
* continuation-passing style;
* SSA;
* A-normal form;
* MIR inspirada no Rust;
* múltiplos níveis de IR.

Proponha ao menos:

```text
Parsed Forms
    ↓
Expanded Forms
    ↓
Analyzed AST
    ↓
High-Level IR
    ↓
Low-Level IR
    ↓
Backend IR
```

Defina os tipos centrais de cada IR.

## Otimizações

Separe otimizações obrigatórias e futuras:

* constant folding;
* dead code elimination;
* tail call optimization;
* specialization;
* inlining;
* unboxing;
* escape analysis;
* monomorphization;
* devirtualization;
* persistent collection specialization;
* removal of intermediate sequences;
* direct protocol dispatch;
* closure allocation elimination.

Não coloque otimizações avançadas no caminho crítico do MVP sem justificativa.

---

# 8. Representação de valores

Defina como valores Clojure serão representados em runtime.

Analise:

* tagged pointers;
* NaN boxing;
* enums Rust;
* boxed trait objects;
* handles gerenciados pelo GC;
* reference counting;
* arena allocation;
* hybrid value representation.

Os tipos mínimos incluem:

* nil;
* boolean;
* integer;
* floating point;
* big integer;
* ratio;
* decimal;
* character;
* string;
* symbol;
* keyword;
* list;
* vector;
* map;
* set;
* function;
* Var;
* atom;
* record;
* protocol object;
* exception;
* lazy sequence.

Avalie:

* custo de boxing;
* alinhamento;
* ABI;
* interoperabilidade;
* pattern matching;
* GC tracing;
* hash;
* equality;
* identity;
* metadata;
* thread safety.

Registre a decisão em:

```text
docs/adr/0003-value-representation.md
```

---

# 9. `MEMORY_MODEL.md`

Compare:

* garbage collector tracing;
* mark-and-sweep;
* generational GC;
* incremental GC;
* reference counting;
* `Arc`;
* cycle collector;
* arenas;
* Boehm GC;
* MMTk;
* GC próprio em Rust;
* abordagem híbrida.

Considere que Clojure possui:

* closures;
* lazy sequences;
* estruturas persistentes compartilhadas;
* referências cíclicas possíveis;
* metadata;
* caches;
* Vars;
* objetos de longa duração;
* objetos temporários em operações de sequência.

O documento deve definir:

* ownership no runtime;
* roots;
* stack scanning;
* precise GC versus conservative GC;
* write barriers;
* finalização;
* weak references;
* thread safety;
* pausas;
* integração com código compilado;
* integração com FFI;
* comportamento em panic;
* destruição na saída do processo.

Registre a decisão em:

```text
docs/adr/0002-memory-management.md
```

---

# 10. Estruturas persistentes

Planeje implementações nativas para:

* persistent vector;
* persistent hash map;
* persistent array map;
* persistent hash set;
* linked list;
* cons;
* queue;
* sorted collections;
* transients.

Analise algoritmos como:

* bitmapped vector trie;
* HAMT;
* CHAMP;
* red-black tree;
* structural sharing;
* path copying.

Defina requisitos de:

* igualdade estrutural;
* hashing;
* metadata;
* interfaces seqable;
* iteradores;
* thread safety;
* persistência;
* transients;
* interoperabilidade com coleções Rust.

Não use simplesmente `HashMap` e `Vec` mutáveis como substitutos silenciosos de coleções persistentes.

---

# 11. Modelo de funções

Especifique:

* representação de closures;
* ambientes capturados;
* múltiplas aridades;
* variadic functions;
* invocation protocol;
* direct calls;
* indirect calls;
* apply;
* partial application;
* recur;
* tail position;
* trampolining, caso necessário;
* stack overflow;
* native stack versus runtime stack.

Defina como será feita a chamada:

```clojure
(defn sum
  ([a b] (+ a b))
  ([a b & more] (reduce + (+ a b) more)))
```

Explique como aridades diferentes serão representadas no IR e no binário.

---

# 12. Vars e namespaces

Defina uma especificação para:

* internamento de Vars;
* root binding;
* dynamic binding;
* metadata;
* namespace registry;
* aliases;
* private Vars;
* unresolved Vars;
* loading order;
* circular dependencies;
* incremental compilation;
* ahead-of-time compilation;
* global initialization;
* deterministic startup.

Analise se o runtime deve permitir:

* redefinição de Vars;
* carregamento dinâmico;
* `eval`;
* REPL;
* hot reload.

Essas funcionalidades podem ser adiadas, mas a arquitetura não deve torná-las impossíveis sem necessidade.

---

# 13. Protocols, records e multimethods

Especifique:

* protocol dispatch;
* direct dispatch;
* lookup tables;
* dispatch por tipo;
* extension de tipos;
* records;
* field access;
* metadata;
* multimethod hierarchy;
* preference tables;
* method caches;
* invalidation de cache.

Avalie o equilíbrio entre:

* semântica dinâmica;
* performance;
* otimização AOT;
* compatibilidade.

---

# 14. Lazy sequences

Defina:

* representação de seqs;
* lazy realization;
* memoization;
* thread safety;
* chunked sequences;
* infinite sequences;
* error propagation;
* retenção de memória;
* interação com GC;
* cancelamento;
* debugging.

Crie exemplos semânticos usados como critérios de teste:

```clojure
(take 10 (map inc (range)))
```

```clojure
(def xs (lazy-seq (cons 1 xs)))
```

```clojure
(reduce + (take 1000000 (range)))
```

---

# 15. `STANDARD_LIBRARY_SCOPE.md`

Defina quais namespaces serão implementados no MVP.

Comece avaliando:

* `clojure.core`;
* `clojure.string`;
* `clojure.set`;
* `clojure.walk`;
* `clojure.edn`;
* `clojure.data`;
* `clojure.zip`;
* `clojure.test`.

Para cada função de `clojure.core`, classifique:

| Função | MVP | Implementação | Dependências | Compatibilidade |
| ------ | --: | ------------- | ------------ | --------------- |

Defina quais funções serão:

* primitivas do compilador;
* implementadas no runtime em Rust;
* implementadas em Clojure;
* macros;
* adiadas;
* incompatíveis por dependência da JVM.

Evite implementar toda a biblioteca padrão em Rust sem necessidade.

O plano deve buscar um bootstrap progressivo no qual uma parcela crescente da biblioteca padrão seja escrita no próprio dialeto Clojure suportado.

---

# 16. Estratégia de bootstrap

Compare:

1. implementar `clojure.core` inicialmente em Rust;
2. implementar apenas primitivas em Rust e o restante em Clojure;
3. usar um interpretador temporário;
4. compilar a biblioteca padrão por estágios;
5. usar a implementação oficial apenas como oracle de testes;
6. usar a JVM apenas durante o desenvolvimento inicial, nunca no executável final;
7. realizar self-hosting progressivo.

Defina estágios como:

```text
Stage 0: reader e runtime em Rust
Stage 1: formas especiais e funções primitivas
Stage 2: macro evaluator
Stage 3: clojure.core mínimo
Stage 4: compilador capaz de compilar parte de si mesmo
Stage 5: biblioteca padrão majoritariamente em Clojure
Stage 6: self-hosting parcial ou completo
```

Registre a estratégia em:

```text
docs/adr/0005-bootstrap-strategy.md
```

---

# 17. `NATIVE_INTEROP.md`

Defina uma estratégia de interoperabilidade nativa.

Considere:

* C ABI;
* Rust ABI somente internamente;
* geração de bindings;
* `extern "C"`;
* carregamento de bibliotecas compartilhadas;
* tipos FFI seguros;
* gerenciamento de ownership;
* strings;
* buffers;
* callbacks;
* erros;
* structs;
* ponteiros;
* async;
* thread safety.

Proponha uma API Clojure, por exemplo:

```clojure
(ns example
  (:require [native.ffi :as ffi]))

(def strlen
  (ffi/function
    {:library "c"
     :symbol "strlen"
     :args [:pointer]
     :return :usize}))
```

O exemplo é apenas ilustrativo. Defina uma API coerente com a arquitetura escolhida.

Especifique também como bibliotecas Rust poderão expor funções para o código Clojure.

---

# 18. Tratamento de erros

Defina:

* erros de reader;
* erros de macro expansion;
* erros semânticos;
* erros de compilação;
* linker errors;
* runtime exceptions;
* panic do Rust;
* falhas de FFI;
* stack traces;
* source maps;
* line and column information;
* causal chains;
* structured diagnostic output.

As mensagens devem possuir:

* arquivo;
* linha;
* coluna;
* trecho do código;
* causa;
* contexto;
* sugestão quando possível.

Evite expor diretamente `panic!` do Rust como mecanismo normal de erro da linguagem.

---

# 19. CLI e sistema de build

Especifique comandos como:

```bash
clojure-native new
clojure-native check
clojure-native build
clojure-native run
clojure-native test
clojure-native fmt
clojure-native repl
clojure-native clean
```

O MVP não precisa implementar todos, mas a interface deve ser planejada.

Defina:

* arquivo de projeto;
* resolução de fontes;
* cache de compilação;
* diretório de build;
* profiles;
* debug versus release;
* otimizações;
* feature flags;
* dependências;
* lockfile;
* reproducibilidade;
* incremental compilation.

Avalie um manifesto como:

```toml
[project]
name = "example"
version = "0.1.0"
main = "example.core"

[build]
target = "native"
optimization = 2
```

Não copie automaticamente o modelo de `deps.edn`, Cargo ou Leiningen. Escolha o formato com base nos requisitos do projeto.

---

# 20. Dependências Clojure

Planeje como bibliotecas serão distribuídas.

Compare:

* source dependencies;
* pacotes próprios;
* repositório compatível com Maven;
* adaptação parcial de `deps.edn`;
* manifesto próprio;
* Git dependencies;
* vendoring;
* pacote binário pré-compilado;
* compilação de dependências por source.

Defina uma política para bibliotecas que:

* usam apenas Clojure puro;
* usam Java interop;
* usam macros;
* dependem de recursos;
* dependem de geração de classes;
* executam código durante a compilação.

---

# 21. REPL e `eval`

Analise separadamente:

* compilação AOT;
* interpretador;
* JIT;
* REPL;
* `eval`;
* carregamento dinâmico.

O compilador AOT pode existir sem REPL no MVP.

Entretanto, defina se um interpretador ou JIT será futuramente necessário para:

* macro expansion;
* desenvolvimento interativo;
* `eval`;
* plugins;
* hot reload.

Evite comprometer o MVP com um JIT completo sem necessidade.

---

# 22. `TESTING_STRATEGY.md`

Crie uma estratégia de testes baseada em equivalência semântica.

Use a implementação oficial de Clojure na JVM como oracle durante o desenvolvimento, mas não como dependência do binário final.

Para cada fragmento suportado:

1. execute na implementação oficial;
2. execute na implementação nativa;
3. compare:

   * valor;
   * tipo;
   * output;
   * erro;
   * metadata;
   * ordem;
   * efeitos observáveis.

Inclua:

* testes unitários;
* parser tests;
* golden tests;
* snapshot tests;
* property-based testing;
* differential testing;
* fuzzing do reader;
* fuzzing de estruturas persistentes;
* testes de GC;
* testes de ciclos;
* testes de stress;
* testes de concorrência;
* testes de ABI;
* testes de integração;
* testes end-to-end;
* testes de build limpo;
* testes cross-platform.

Ferramentas Rust a avaliar:

* `cargo test`;
* `proptest`;
* `quickcheck`;
* `cargo-fuzz`;
* `insta`;
* Miri;
* sanitizers;
* Loom;
* Criterion.

Justifique cada ferramenta adicionada.

Crie um conjunto inicial de programas de conformidade:

```text
tests/conformance/
├── reader/
├── arithmetic/
├── control-flow/
├── functions/
├── closures/
├── recur/
├── macros/
├── namespaces/
├── collections/
├── sequences/
├── protocols/
├── metadata/
└── errors/
```

---

# 23. Performance

Defina benchmarks para:

* startup;
* tempo de compilação;
* tamanho do binário;
* alocação;
* GC pause;
* persistent vector;
* persistent map;
* seq traversal;
* lazy sequences;
* protocol dispatch;
* function calls;
* closures;
* recursion;
* string processing;
* startup de CLI.

Compare, quando aplicável:

* Clojure na JVM em cold start;
* Clojure após warm-up;
* Babashka;
* Rust nativo;
* executável produzido pelo novo compilador.

As comparações devem ser interpretadas com cautela. Não use benchmarks artificiais como argumento exclusivo de arquitetura.

---

# 24. Compatibilidade de plataformas

Planeje suporte para:

* Linux `x86_64`;
* Linux `arm64`;
* Windows `x86_64`;
* macOS `x86_64`;
* macOS Apple Silicon.

Escolha uma plataforma inicial oficial.

Explique:

* linker;
* toolchain;
* libc;
* MSVC versus GNU;
* debugging;
* stack unwinding;
* dynamic libraries;
* static linking;
* cross-compilation;
* CI matrix;
* release artifacts.

Windows deve ser tratado como plataforma de primeira classe, não como adaptação tardia.

---

# 25. Segurança

Analise:

* macros executando código no build;
* bibliotecas de terceiros;
* filesystem access;
* environment access;
* native FFI;
* memory safety;
* unsafe Rust;
* supply chain;
* reproducible builds;
* sandboxing;
* carregamento de bibliotecas dinâmicas;
* execução de código não confiável.

Crie uma política para uso de `unsafe`.

Todo bloco `unsafe` deve possuir:

* justificativa;
* invariantes;
* testes;
* revisão;
* documentação.

---

# 26. `IMPLEMENTATION_PLAN.md`

Crie um plano incremental.

Não produza um roadmap genérico. Cada item deve indicar:

* objetivo;
* arquivos ou crates envolvidos;
* dependências;
* implementação esperada;
* testes;
* riscos;
* critério de aceite;
* resultado observável.

Organize o trabalho em fases.

## Fase 0 — Pesquisa e especificação

* especificação da linguagem;
* decisões arquiteturais;
* protótipos descartáveis;
* validação do backend;
* validação do modelo de valores;
* validação do GC.

## Fase 1 — Reader e formulários

* tokenizer;
* parser;
* source spans;
* tipos básicos;
* collections literais;
* quoting;
* diagnostics.

Critério de aceite sugerido:

```bash
clojure-native read examples/basic.clj
```

Deve produzir uma representação estruturada e determinística dos formulários.

## Fase 2 — Interpretador mínimo ou evaluator de bootstrap

* literals;
* locals;
* `if`;
* `do`;
* `let`;
* `fn`;
* function calls;
* closures;
* primitive operations.

Esse componente pode existir somente para bootstrap e macro expansion.

## Fase 3 — Analisador e IR

* symbol resolution;
* scopes;
* closures;
* recur validation;
* HIR;
* lowering;
* diagnostics.

## Fase 4 — Runtime mínimo

* value representation;
* strings;
* symbols;
* keywords;
* functions;
* lists;
* vectors;
* maps;
* equality;
* hashing;
* memory management.

## Fase 5 — Primeiro executável nativo

Compile:

```clojure
(ns hello.core)

(defn -main []
  (println "Hello from native Clojure"))

(-main)
```

O resultado deve ser um binário independente da JVM.

## Fase 6 — Macros

* `defmacro`;
* macro environment;
* syntax quote;
* unquote;
* gensym;
* compile-time evaluation.

## Fase 7 — `clojure.core` mínimo

Implemente funções necessárias para programas úteis.

## Fase 8 — Estruturas persistentes completas

* vector trie;
* HAMT ou CHAMP;
* sets;
* transients;
* structural sharing.

## Fase 9 — Namespaces e dependências

* múltiplos arquivos;
* require;
* aliases;
* incremental compilation;
* dependency graph.

## Fase 10 — Protocols e records

* protocol dispatch;
* records;
* extension;
* optimization.

## Fase 11 — FFI nativa

* C ABI;
* bindings;
* libraries;
* callbacks.

## Fase 12 — Conformidade e distribuição

* test suite;
* cross-platform builds;
* release artifacts;
* documentation;
* package distribution.

Para cada fase, crie tarefas pequenas e ordenadas.

---

# 27. Protótipos obrigatórios antes da implementação principal

Planeje protótipos descartáveis para validar:

1. geração de um executável com o backend escolhido;
2. chamada indireta de funções;
3. closures;
4. tagged value representation;
5. alocação e tracing do GC;
6. stack roots;
7. persistent vector;
8. persistent hash map;
9. execução de uma macro simples;
10. source-level stack trace;
11. linking no Windows;
12. chamada FFI para uma função C.

Cada protótipo deve responder uma pergunta arquitetural específica.

Não transforme automaticamente os protótipos em código de produção.

---

# 28. `RISK_REGISTER.md`

Crie uma matriz:

| Risco | Probabilidade | Impacto | Detecção | Mitigação |
| ----- | ------------: | ------: | -------- | --------- |

Inclua no mínimo:

* complexidade excessiva da semântica Clojure;
* execução de macros;
* gerenciamento de memória;
* performance das chamadas dinâmicas;
* compatibilidade com bibliotecas;
* crescimento descontrolado do escopo;
* portabilidade;
* suporte ao Windows;
* stack traces;
* bootstrap;
* diferenças semânticas difíceis de detectar;
* implementação incorreta de coleções persistentes;
* bugs relacionados a concorrência;
* uso excessivo de `unsafe`;
* dependência excessiva de LLVM;
* tempo de compilação;
* tamanho dos executáveis;
* dificuldade de manutenção por equipe pequena.

---

# 29. Critérios de aceite do MVP

O MVP somente será considerado concluído quando:

1. Compilar código Clojure para um executável nativo.
2. O executável funcionar sem JVM.
3. O executável não carregar JVM de maneira embutida.
4. O projeto possuir reader próprio.
5. O projeto possuir analisador semântico próprio.
6. O projeto possuir runtime nativo.
7. Funções e closures funcionarem.
8. `if`, `do`, `let`, `fn`, `def`, `loop` e `recur` funcionarem.
9. Vetores, listas, mapas e sets persistentes básicos funcionarem.
10. Macros definidas pelo usuário funcionarem dentro do escopo documentado.
11. Namespaces básicos funcionarem.
12. Uma parte documentada de `clojure.core` estiver disponível.
13. O build funcionar em ambiente limpo.
14. O binário rodar em pelo menos Linux e Windows.
15. Os testes diferenciais passarem para o subconjunto suportado.
16. Erros de compilação mostrarem arquivo, linha e coluna.
17. O runtime não depender de Java.
18. Todas as incompatibilidades conhecidas estiverem documentadas.

---

# 30. Regras de qualidade

Durante a criação da especificação:

* não invente compatibilidade que ainda não existe;
* não use termos vagos como “suportar Clojure” sem definir o subconjunto;
* não esconda dependências da JVM;
* não trate GraalVM como compilador Rust;
* não use `HashMap` mutável como substituto silencioso de persistent map;
* não transforme panic do Rust em mecanismo comum de exceções;
* não presuma que macros são simples substituições sintáticas;
* não ignore dynamic Vars, metadata e namespaces;
* não coloque concurrency completa no MVP sem análise;
* não proponha self-hosting imediato;
* não comece pela otimização;
* não implemente todos os recursos antes do primeiro binário;
* não reescreva componentes existentes sem justificar;
* não aceite comentários `TODO` como especificação.

Diferencie claramente:

* fato observado;
* comportamento da implementação oficial;
* decisão proposta;
* hipótese;
* risco;
* funcionalidade futura.

---

# 31. Formato final da resposta

Ao terminar:

1. Crie ou atualize todos os documentos especificados.
2. Apresente um resumo das decisões.
3. Liste questões ainda abertas.
4. Liste os maiores riscos.
5. Mostre a ordem exata recomendada para implementação.
6. Identifique qual é o menor caminho até o primeiro binário nativo.
7. Não implemente o compilador nesta etapa.
8. Não altere código de produção, exceto caso sejam necessários protótipos explicitamente descartáveis.
9. Marque claramente qualquer protótipo como não produtivo.
10. Cite caminhos reais do repositório sempre que fizer referência ao código existente.

O resultado precisa ser detalhado o suficiente para que outra equipe consiga iniciar a implementação sem redescobrir as decisões fundamentais do compilador.
