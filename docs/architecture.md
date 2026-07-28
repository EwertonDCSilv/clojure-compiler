# Arquitetura

[Índice da documentação](README.md) · [Visão geral](overview.md) ·
[Uso](usage.md) · [Especificações](../specs/README.md)

> Arquitetura auditada no [`HEAD 476aefd`](https://github.com/EwertonDCSilv/clojure-compiler/commit/476aefd47bd01c4dca8b11f3e8009fbf2cd78d3c).
> Consulte [SNAPSHOT.md](SNAPSHOT.md).

O `clojure-compiler` é um workspace Cargo. O executável entregue pelo workspace se chama
`clojure-native`.

## Crates principais

| Crate | Responsabilidade |
| --- | --- |
| `clojure-span` | Posições e intervalos de código-fonte |
| `clojure-diagnostics` | Erros determinísticos e renderização de diagnósticos |
| `clojure-syntax` | Forms e estruturas sintáticas |
| `clojure-reader` | Tokenização, reader macros e parsing |
| `clojure-value` | Valores usados pelo interpretador |
| `clojure-interp` | Interpretador de bootstrap |
| `clojure-analyzer` | Resolução, macroexpansão, closures, `recur`, records e protocolos |
| `clojure-codegen` | IR Cranelift, objeto nativo e runtime C embutido |
| `clojure-native-cli` | Comandos `read`, `eval`, `run` e `build` |
| `clojure-test-support` | Runner, schema, checksums, oracle e relatórios de conformidade |

## Fluxo de compilação

```text
fonte .clj
   │
   ▼
reader ──► forms com spans
   │
   ▼
macroexpansão + analyzer ──► programa analisado
   │
   ▼
codegen Cranelift ──► objeto nativo
   │
   ▼
cc + runtime C embutido ──► executável do host
```

O CLI carrega primeiro o subconjunto compilável de `clojure.core`, analisa o core e o
programa do usuário como uma unidade, gera o objeto e invoca o linker C do sistema.

Depois da análise semântica, um pós-passe conservador identifica acumuladores frescos de
vetor usados linearmente. O passe cobre loops locais e o primeiro padrão de parâmetro
linear entre funções de topo; ele rebaixa para o caminho persistente quando encontra
captura, alias ou chamada não reconhecida. O resultado continua sendo a mesma AST
consumida pelo codegen — ainda não existe uma IR própria separada.

## Modelo de valores e chamadas

No código compilado, fixnums são valores tagueados e os demais valores são ponteiros
para objetos rastreados pelo GC. As operações inteiras mais frequentes (`+`, `-`, `*`,
`quot`, `mod`, `inc`, `dec` e comparações) têm fast paths no código gerado, com guards
de tipo e overflow. Casos inválidos seguem para funções verificadas do runtime.

Todas as funções compiladas usam a convenção uniforme `(self, argc, argv)`. Isso permite
aridades múltiplas, variádicas, closures, chamadas indiretas e `apply`. `loop/recur`
vira um backedge nativo e não cresce a pilha.

## Coleções

- Lista: células cons ligadas.
- Vetor: trie bitmap persistente de 32 vias.
- Mapa: array-map pequeno que promove para HAMT de 32 vias.
- Set: array-set pequeno que promove para a representação HAMT.
- Mapa/set ordenado: árvore LLRB persistente.
- Record: nome nominal e campos com semântica associativa.
- Transient: buffer mutável para vetor e caixa sobre o valor persistente para map/set.

As estruturas persistentes usam path-copying e compartilhamento estrutural. CHAMP,
edit tokens e transients com mutação in-place em nós de map/set continuam planejados.
O core compilado usa o vetor transient estrutural em `mapv` e `into`; o analyzer também
o seleciona para os acumuladores cuja unicidade consegue provar.

## Runtime e GC

O runtime C embutido fornece alocação, coleções, strings, impressão, exceções,
dispatch de protocolos/multimétodos e slow paths. O coletor é mark-sweep preciso, não
móvel e single-thread.

Fisicamente, o runtime está dividido em fragmentos ordenados por subsistema em
`crates/clojure-codegen/runtime/`. O codegen os concatena com `include_str!` e o
compilador C ainda recebe uma única unidade de tradução; portanto a modularização não
cria bibliotecas, ABIs ou estados duplicados.

Vetores literais não vazios compostos somente por fixnums, booleanos e `nil` recebem um
ID de site. No primeiro uso, o codegen constrói o vetor e o registra no cache do runtime;
usos seguintes carregam o mesmo valor. O cache é uma raiz permanente visitada pelo GC.
Literais com elementos dinâmicos continuam seguindo a construção normal.

Streams gerais, arquivos e readers de runtime ainda são alvo futuro. A fronteira
proposta mantém handles e buffers atrás da ABI C, conforme
[`IO_SPEC`](../specs/IO_SPEC.md) e
[`ADR-0007`](../specs/adr/0007-native-io-and-runtime-reader.md).

Cada função compilada abre um frame na shadow stack. O codegen faz loads/stores diretos
nos slots de roots e no stack pointer, em vez de chamar `gc_push`, `gc_popn` ou `gc_set`
para cada expressão. A entrada e a saída do frame continuam delimitadas pelo runtime.
O modo `CLJN_GC_STRESS=1` força coleta em toda alocação para validar rooting.

## Testes

Além dos testes de cada crate, `clojure-test-support` descobre e executa os casos de
[`tests/conformance/`](../tests/conformance). Casos `active` bloqueiam em divergências;
`xfail` bloqueia se passar inesperadamente; `pending` valida schema e checksum sem
executar. O runner reutiliza um único CLI release e limita a concorrência a quatro casos.

O [`Makefile`](../Makefile) mantém uma entrada estável sobre essas camadas:

```text
make quality
   ├── rustfmt
   ├── clippy
   ├── clj-kondo
   └── cargo test --workspace

make all
   ├── quality
   ├── coverage
   ├── compatibility
   └── benchmarks
```

Os scripts em `scripts/` e nos diretórios de benchmark continuam sendo as interfaces
de baixo nível. A CI usa os mesmos alvos públicos do Makefile, divididos em jobs de
qualidade, cobertura, conformidade e checksums de benchmarks.

A estratégia para ampliar testes unitários por crate está na
[`ADR-0011`](../specs/adr/0011-rust-crate-unit-testing-strategy.md), e a redução dos
grandes arquivos Rust está na
[`ADR-0012`](../specs/adr/0012-rust-crate-modularization.md). Ambas são propostas; a
separação física já concluída do runtime C é um trabalho anterior e distinto.

Detalhes normativos ficam nas
[`especificações`](../specs/README.md); comandos e requisitos estão no
[guia de uso](usage.md).
