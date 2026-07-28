# Runtime nativo

Este documento separa o runtime executável atual do modelo de longo prazo. No caminho
compilado, o runtime é C embutido em `clojure-codegen` e exposto ao objeto Cranelift por
ABI C. Os crates Rust `clojure-value` e `clojure-interp` mantêm a representação do
interpretador de bootstrap; eles não são a representação ABI do executável nativo.

Os subsistemas C ficam em arquivos separados sob `clojure-codegen/runtime/`, mas são
concatenados na ordem declarada pelo codegen e compilados como uma única unidade de
tradução. `clojure-codegen/runtime.c` permanece como entrada compatível para ferramentas
C diretas.

## Representação atual

O tipo ABI `Value` é uma palavra do tamanho de ponteiro (`intptr_t`):

- fixnums são inteiros tagueados;
- `nil`, `false`, `true` e a lista vazia usam valores imediatos reservados;
- strings, keywords, cons cells, funções, vetores, mapas, sets, records e nós internos
  são ponteiros para objetos com cabeçalho de GC.

O intervalo seguro do fixnum é `[-2^62, 2^62 - 1]` nas plataformas de 64 bits atuais.
Fast paths gerados validam tag e overflow antes de devolver o valor retagueado; tipos
inválidos e overflow seguem para o slow path do runtime.

O caminho compilado representa `f64` como objeto `Float` boxeado e executa aritmética
mista fixnum/float. BigInt, ratio e BigDecimal permanecem trabalho futuro.

### Objetos rastreados

O runtime atual possui tags para:

- string, keyword, `Float` e `Bytes`;
- cons;
- closure/função;
- vetor persistente e seus nós;
- array-map e HAMT, incluindo nós e colisões;
- array-set e HAMT set;
- record;
- sorted map/set e nós LLRB;
- tipos internos usados pelo runtime.

Símbolos são necessários durante leitura/análise, mas não são objetos de primeira classe
do runtime compilado atual. Metadata em valores compilados, atoms, delays e objetos de
exceção com tipo/`ex-data` também permanecem futuros. Vars dinâmicas e `binding` já
restauram os valores anteriores inclusive durante unwind. Valores lançados
explicitamente são mantidos como roots enquanto atravessam handlers nativos.

### Cache de literais constantes

O codegen atribui IDs aos vetores literais não vazios cujos elementos são todos
imediatos (`Int`, `Bool` ou `Nil`). Cada site constrói o vetor apenas no primeiro uso e
registra o valor em `cljn_const_cache`; avaliações seguintes reutilizam o objeto
imutável.

O GC percorre as entradas registradas como roots permanentes antes do sweep. O cache
não é finalizer nem weak reference e não se aplica a literais com elementos dinâmicos.
O limite atual é 8.192 sites por objeto compilado; depois dele o codegen usa a construção
persistente normal.

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
| Sorted map/set | árvore LLRB persistente | ✅ |
| Transients | vetor com buffer mutável; map/set em caixa sobre valor persistente | ✅ parcial |
| CHAMP | evolução do HAMT para localidade/iteração | futuro |

Mapas e sets promovem automaticamente da representação pequena para HAMT. As operações
persistentes retornam novos roots e preservam os caminhos não alterados.

O subconjunto transient implementa `transient`, `persistent!`, `conj!`, `assoc!` e
`dissoc!`. Vetores usam mutação real e crescimento amortizado; mapas e sets atualizam
uma caixa com o novo valor persistente. Ainda faltam edit tokens, invalidação depois de
`persistent!`, `disj!`, `pop!` e mutação in-place nos nós HAMT.

O core compilado usa esse vetor transient em `mapv` e `into`. O analyzer também o
seleciona para acumuladores de `loop` comprovadamente lineares, inclusive no primeiro
subconjunto de chamadas interprocedurais. O valor é persistido na fronteira final; usos
ambíguos, capturas e chamadas não resumidas preservam o caminho imutável.

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

Impls inline em `defrecord`, `extend-protocol`, `deftype`, `reify`, hierarquias e
devirtualização continuam futuros.

O runtime também implementa `defmulti`/`defmethod`: a função de dispatch recebe os
argumentos originais, o resultado é comparado estruturalmente com os valores registrados
e `:default` é usado como fallback. A função de dispatch precisa ser uma `fn` explícita;
keywords invocáveis, preferências, hierarquias `derive`/`isa?` e cache de hierarquia
ainda não estão disponíveis.

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

`throw` e `try`/`catch`/`finally` já são capturáveis no caminho nativo. O unwind
restaura shadow stack e estado do GC, suporta nesting e executa `finally` nos caminhos
normal e capturado. O snapshot aceita uma única cláusula catch-all: o símbolo de classe
é sintaticamente aceito, mas ainda não participa do dispatch.

Erros de tipo, aridade, overflow, divisão por zero e índices inválidos continuam
diagnósticos fatais e não são convertidos em valores capturáveis. Hierarquia de
exceções, `ex-info`/`ex-data` e stack trace de fonte também permanecem futuros.

## I/O e recursos externos

O runtime executável atual oferece output, flush, redirecionamento dinâmico,
`slurp`/`spit`, `read-string` e streams de string. Handles e arquivos textuais já
atendem esses caminhos, mas stdin geral, binários, filesystem amplo, EDN completo e
parte dos erros/lifecycles ainda não estão cobertos pelo gate.

O contrato completo está em [IO_SPEC](IO_SPEC.md): handles abertos são objetos GC
ligados a um registro explícito de recursos externos; buffers e syscalls permanecerão
atrás da ABI C; `close!` remove o registro. Finalizers não são requisito de correção.
A base genérica de unwind, Vars dinâmicas e `binding` já existem; a matriz restante
continua registrando os contratos ainda não promovidos.

## Estado futuro

O modelo arquitetural preserva espaço para:

- números de precisão arbitrária;
- metadata;
- lazy seqs;
- exceções tipadas, `ex-data` e stack traces de fonte;
- atoms, volatiles e delays;
- namespaces e carregamento AOT multi-arquivo;
- threads e um coletor apropriado a concorrência.
- APIs públicas e contratos restantes de I/O conforme [IO_SPEC](IO_SPEC.md).

Esses itens não devem ser tratados como recursos disponíveis até aparecerem como casos
`active` na suíte de conformidade.
