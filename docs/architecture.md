# Arquitetura

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

## Runtime e GC

O runtime C embutido fornece alocação, coleções, strings, impressão, exceções,
dispatch de protocolos/multimétodos e slow paths. O coletor é mark-sweep preciso, não
móvel e single-thread.

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
