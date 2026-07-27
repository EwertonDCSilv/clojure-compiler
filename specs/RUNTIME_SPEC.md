# Runtime nativo

Este documento separa o runtime executável atual do modelo de longo prazo. No caminho
compilado, o runtime é C embutido em `clojure-codegen` e exposto ao objeto Cranelift por
ABI C. Os crates Rust `clojure-value` e `clojure-interp` mantêm a representação do
interpretador de bootstrap; eles não são a representação ABI do executável nativo.

## Representação atual

O tipo ABI `Value` é uma palavra do tamanho de ponteiro (`intptr_t`):

- fixnums são inteiros tagueados;
- `nil`, `false`, `true` e a lista vazia usam valores imediatos reservados;
- strings, keywords, cons cells, funções, vetores, mapas, sets, records e nós internos
  são ponteiros para objetos com cabeçalho de GC.

O intervalo seguro do fixnum é `[-2^62, 2^62 - 1]` nas plataformas de 64 bits atuais.
Fast paths gerados validam tag e overflow antes de devolver o valor retagueado; tipos
inválidos e overflow seguem para o slow path do runtime.

O reader reconhece `f64`, mas a representação numérica compilada ainda é somente
fixnum. Float, BigInt, ratio e BigDecimal no runtime nativo são trabalho futuro.

### Objetos rastreados

O runtime atual possui tags para:

- string e keyword;
- cons;
- closure/função;
- vetor persistente e seus nós;
- array-map e HAMT, incluindo nós e colisões;
- array-set e HAMT set;
- record;
- tipos internos usados pelo runtime.

Símbolos são necessários durante leitura/análise, mas não são objetos de primeira classe
do runtime compilado atual. Metadata em valores compilados, Vars dinâmicas, atoms,
delays e exceções também permanecem futuros.

## Igualdade e hash

- `=` usa igualdade estrutural para strings, listas, vetores, maps, sets e records.
- Maps e sets com representações físicas diferentes continuam iguais quando seus
  elementos são iguais.
- O hash interno precisa ser consistente com igualdade para navegação HAMT.
- Metadata não participa de igualdade ou hash quando esse suporte for introduzido.
- Não existe compromisso de reproduzir valores numéricos de hash da JVM.

## Coleções persistentes

| Coleção | Implementação atual | Estado |
| --- | --- | :-: |
| Lista/Cons | lista ligada imutável | ✅ |
| Vetor | trie bitmap de 32 vias, tail e path-copying | ✅ |
| Mapa pequeno | array-map em ordem de inserção, até 8 entradas | ✅ |
| Mapa grande | HAMT bitmap de 32 vias com nós de colisão | ✅ |
| Set pequeno | array-set compacto | ✅ |
| Set grande | HAMT de 32 vias | ✅ |
| Record | nome nominal + mapa persistente de campos | ✅ |
| Queue | duas sequências | futuro |
| Sorted map/set | árvore balanceada | futuro |
| Transients | edição controlada | futuro |
| CHAMP | evolução do HAMT para localidade/iteração | futuro |

Mapas e sets promovem automaticamente da representação pequena para HAMT. As operações
persistentes retornam novos roots e preservam os caminhos não alterados.

## Modelo de funções

Todas as funções nativas geradas usam uma convenção uniforme:

```text
(self, argc, argv) -> Value
```

- `self` aponta para a closure e seu vetor de valores capturados;
- `argc` informa a aridade efetiva;
- `argv` aponta para os argumentos já protegidos na shadow stack;
- dispatch de multi-aridade e variádico ocorre a partir de `argc`;
- chamadas indiretas permitem HOF e closures;
- `apply` materializa os argumentos na shadow stack e usa a mesma convenção;
- `loop/recur` é um backedge, não uma chamada.

Chamadas em tail que não usam `recur` ainda consomem a pilha nativa. TCO geral,
stack traces de linguagem e conversão portátil de stack overflow permanecem futuros.

## Protocolos e records

O subconjunto atual implementa:

- `defrecord` com construtor `->Nome`, acesso por keyword, `assoc`, `keys`, `count`,
  igualdade e impressão nominal;
- `defprotocol`, gerando funções de dispatch;
- `extend-type` para records e tipos embutidos;
- lookup por `(method_id, type_key(primeiro-argumento))`;
- tabela de métodos mantida como root permanente do GC.

Impls inline em `defrecord`, `extend-protocol`, `deftype`, `reify`, multimethods,
hierarquias e devirtualização continuam futuros.

## Sequences

Listas e vetores participam das operações de sequência suportadas (`first`, `rest`,
`count`, `empty?` e as funções eager do core compilado). O core atual não implementa
`LazySeq`, chunked seqs ou sequências infinitas. O `range` atual recebe um limite e
constrói um resultado finito.

O alvo futuro continua sendo um protocolo `ISeq`, lazy seq memoizada e suporte seguro a
fontes infinitas, acompanhado de testes de retenção.

## GC e roots

O coletor atual é tracing mark-sweep preciso, não móvel e single-thread. Ele não
escaneia conservadoramente a pilha nativa: o codegen mantém referências vivas em uma
shadow stack.

- Entrada e saída de função delimitam o frame de roots.
- Slots locais e temporários são atualizados por loads/stores diretos no código gerado.
- `gc_push`, `gc_popn` e `gc_set` continuam no runtime por compatibilidade, mas não são
  importados pelo novo objeto para o caminho comum.
- Operações multi-alocação do runtime usam uma zona sem GC e executam um safepoint na
  entrada.
- `CLJN_GC_STRESS=1` força coleta em toda alocação permitida.

O rooting ainda é eager. O próximo refinamento planejado usa liveness para manter roots
somente ao redor de operações que podem alocar.

## Erros

O runtime atual reporta erros de tipo, aridade, overflow, divisão por zero e índices
inválidos como diagnósticos fatais do programa nativo. Uma hierarquia capturável de
exceções com `throw`, `try/catch/finally`, `ex-info` e stack trace de fonte ainda não
foi implementada.

## Estado futuro

O modelo arquitetural preserva espaço para:

- números de precisão arbitrária e ponto flutuante;
- metadata e Vars;
- lazy seqs;
- exceções capturáveis;
- atoms, volatiles e delays;
- namespaces e carregamento AOT multi-arquivo;
- threads e um coletor apropriado a concorrência.

Esses itens não devem ser tratados como recursos disponíveis até aparecerem como casos
`active` na suíte de conformidade.
