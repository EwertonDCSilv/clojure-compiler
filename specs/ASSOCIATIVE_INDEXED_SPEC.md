# Operações associativas e indexadas

Status: **proposto**. Esta especificação define o gate para atualizações persistentes
com `assoc` e acesso posicional genérico com `nth`. Ela não afirma que todo o contrato
abaixo já está implementado.

A classificação executável continua sendo a matriz em
[`tests/conformance/`](../tests/conformance). O desenho de dispatch que sustenta este
contrato está na
[ADR-0008](adr/0008-associative-indexed-dispatch.md).

## Objetivos

O gate tem quatro objetivos:

1. tornar `assoc` uma operação persistente uniforme para toda coleção associativa;
2. tornar `nth` uniforme para valores indexados e sequenciais, incluindo a aridade com
   `not-found`;
3. permitir que novos tipos participem sem acrescentar mais um `case` a cada função do
   runtime;
4. preservar fast paths, compartilhamento estrutural e rooting correto no caminho
   nativo.

`get`, `contains?`, `find`, `assoc!` e invocação de coleções têm contratos relacionados,
mas não são redefinidos por esta página. Eles devem permanecer coerentes com as leis
descritas aqui.

## Baseline e lacunas atuais

Em 2026-07-27 há dois contratos diferentes:

| Operação | Interpretador de bootstrap | Executável nativo | Lacuna |
| --- | --- | --- | --- |
| `assoc` | mapa/`nil`, vetor e múltiplos pares | mapa, sorted-map, vetor, record e `nil`, somente um par | aridade e tipos divergem |
| `nth` | vetor, lista e string; aridades 2 e 3 | vetor, vetor transient e cons; somente aridade 2 | falta `not-found`, string e dispatch extensível |
| Dispatch | `match` sobre `Value` | `switch` sobre tag de objeto | conjunto fechado de tipos |
| Erros | `Result` textual | término fatal no runtime | ainda não há categoria capturável uniforme |

O runtime nativo já usa path-copying em vetor, HAMT e LLRB. Esta spec preserva essas
implementações; o trabalho é tornar o contrato completo, genérico e igual nos dois
caminhos.

## Vocabulário normativo

- **Persistente** significa que a operação devolve um novo valor lógico e que todas as
  versões anteriores continuam válidas e inalteradas.
- **Compartilhamento estrutural** significa que partes não modificadas podem ser
  reutilizadas entre versões. Esse compartilhamento nunca é observável como mutação.
- **Associativo** é o valor que aceita uma atualização chave/valor persistente.
- **Indexado** é o valor que oferece acesso posicional direto por índice.
- **Sequencial** é o valor que pode ser percorrido em ordem e, portanto, admite o
  fallback linear de `nth`.
- **Não encontrado** significa índice fora dos limites de um tipo suportado. Não
  significa tipo de coleção sem suporte nem índice de tipo inválido.

Os termos `DEVE`, `NÃO DEVE` e `PODE` são normativos.

## Contrato de `assoc`

### Formas e avaliação

```clojure
(assoc coll key value)
(assoc coll key value & key-values)
```

- A chamada DEVE conter ao menos um par chave/valor.
- Os argumentos DEVEM ser avaliados uma vez, da esquerda para a direita, antes de a
  primeira atualização ser observada.
- Os pares DEVEM ser aplicados da esquerda para a direita; cada passo recebe o resultado
  do passo anterior.
- Um número ímpar de itens depois de `coll` é erro de aridade. Em chamada direta ele
  PODE ser rejeitado pelo analyzer; via `apply`, o erro é de runtime.
- `assoc` NÃO DEVE modificar `coll`, mesmo quando não houver outra referência conhecida.
- `assoc` em transient é erro. Mutação controlada continua pertencendo a `assoc!`.

O encadeamento é semanticamente equivalente a:

```clojure
(assoc (assoc coll k1 v1) k2 v2)
```

Essa equivalência não autoriza intercalar a avaliação de `k2`/`v2` com a primeira
atualização.

### Tipos embutidos

| Receptor | Chave válida | Resultado | Complexidade alvo |
| --- | --- | --- | --- |
| `nil` | qualquer valor | novo array-map com o primeiro par; pares seguintes usam o mapa resultante | até 8 entradas, O(n) limitado |
| array-map | qualquer valor com igualdade/hash válidos | array-map ou promoção transparente para HAMT | O(n), com `n <= 8` |
| HAMT map | qualquer valor com igualdade/hash válidos | mapa persistente | O(log32 n) esperado |
| sorted-map | valor aceito pelo comparador | sorted-map com o mesmo comparador | O(log n) |
| vetor | fixnum inteiro `0 <= i <= count` | substituição se `i < count`; append se `i == count` | O(log32 n) |
| record | chave de campo ou chave de extensão | record do mesmo tipo nominal | custo do mapa interno + um wrapper |
| tipo extensível | definido pela capacidade associativa | valor persistente definido pela implementação | declarado pelo tipo |

Para mapas, atualizar uma chave existente NÃO DEVE alterar `count`. Adicionar uma chave
ausente DEVE incrementá-lo uma vez. O tipo físico de um mapa pequeno PODE mudar durante
promoção, mas sua família lógica deve ser preservada: mapas ordenados continuam
ordenados e records continuam records.

Para vetores:

- índice negativo, não inteiro ou maior que `count` é erro;
- depois de um append no índice `count`, o próximo par vê o novo `count`;
- o valor antigo no índice substituído continua alcançável pela versão anterior.

Uma implementação PODE devolver o próprio receptor quando a atualização não muda seu
valor lógico. Identidade de objeto não faz parte do contrato.

Quando metadata estiver disponível no caminho compilado, o resultado DEVE preservar a
metadata do receptor. A metadata não participa de igualdade, hash ou decisão de
compartilhamento. `assoc` em `nil` cria um mapa sem metadata.

### Leis

Para as coleções embutidas e todo tipo extensível que também ofereça as operações
companheiras:

```clojure
(= (get (assoc c k v) k) v)
(contains? (assoc c k v) k)
(= (count (assoc c k v))
   (if (contains? c k) (count c) (inc (count c))))
```

A lei de `count` não se aplica ao append de vetor da mesma forma que a mapas: no vetor,
o índice `count` aumenta o tamanho e um índice menor o preserva. Implementações
extensíveis devem documentar se também oferecem `get`, `contains?` e `count`; quando
oferecerem, essas operações DEVEM ser coerentes com `assoc`.

## Contrato de `nth`

### Formas e índice

```clojure
(nth coll index)
(nth coll index not-found)
```

- `index` DEVE ser um fixnum inteiro enquanto esse for o domínio inteiro do runtime.
- Índice negativo é fora dos limites, não erro de tipo.
- Na aridade 2, um índice fora dos limites em receptor não `nil` lança erro.
- Na aridade 3, um índice fora dos limites devolve exatamente `not-found`.
- `not-found` é um argumento normal: ele é avaliado antes da chamada mesmo quando o
  índice existe.
- Um receptor sem capacidade indexada ou sequencial é erro nas duas aridades. O terceiro
  argumento não transforma um tipo sem suporte em coleção vazia.

### Ordem de dispatch

`nth` DEVE seguir esta ordem semântica:

1. validar o tipo do índice;
2. tratar `nil`;
3. usar a implementação indexada embutida ou registrada;
4. usar travessia linear se o valor for sequencial;
5. reportar tipo sem suporte.

O passo 3 PODE ser especializado antes dos demais pelo codegen quando o tipo for
provado, desde que o resultado e os erros sejam idênticos.

### Tipos e resultados

| Receptor | Aridade 2 | Aridade 3 fora dos limites | Complexidade |
| --- | --- | --- | --- |
| `nil` | `nil` | `not-found` | O(1) |
| vetor persistente | elemento ou erro | `not-found` | O(log32 n) |
| vetor transient válido | elemento ou erro | `not-found` | O(1) |
| lista/cons e sequência | elemento ou erro | `not-found` | O(index) |
| lista vazia | erro | `not-found` | O(1) |
| map entry nativo | chave em 0, valor em 1, erro nos demais | `not-found` | O(1) |
| string | `Char` no índice ou erro | `not-found` | O(index), salvo índice auxiliar |
| `Bytes` | fixnum de 0 a 255 ou erro | `not-found` | O(1) |
| tipo extensível indexado | definido pela implementação | definido pela implementação | declarado pelo tipo |

O suporte a string fica bloqueado pela representação compilada de `Char`; `Bytes`
depende do marco IO-1 da [IO_SPEC](IO_SPEC.md). Arrays e `java.util.List` são detalhes
da plataforma JVM e não entram no runtime nativo. Regex matchers só entram quando
existir um tipo nativo correspondente.

Strings nativas são indexadas por valor escalar Unicode, não por unidade UTF-16. Assim,
caracteres fora do BMP ocupam uma posição. Esta é uma divergência deliberada da JVM e
deve ter caso de compatibilidade documentado.

O fallback sequencial:

- NÃO DEVE materializar toda a sequência;
- NÃO DEVE chamar `count` antes da travessia;
- DEVE consumir no máximo `index + 1` elementos;
- PODE realizar trabalho ou alocar ao realizar uma lazy seq futura;
- DEVE funcionar para um índice finito em uma sequência infinita.

Um vetor transient usado depois de `persistent!` deve produzir o erro de transient
inválido, inclusive através de `nth`. Enquanto a invalidação por edit token não estiver
implementada, esse caso permanece `xfail`.

## Capacidades extensíveis

O runtime reserva três operações de core, em um espaço de IDs separado dos protocolos e
multimétodos definidos pelo programa:

```text
AssocOne(coll, key, value)       -> novo valor persistente
Nth(coll, index)                 -> valor ou erro de limites
NthOr(coll, index, not-found)    -> valor ou not-found
```

Elas correspondem conceitualmente a `IAssociative/-assoc` e às duas aridades de
`IIndexed/-nth`. A ABI interna usa nomes distintos para as aridades de `nth`; isso evita
depender de overload no mecanismo inicial de protocolos.

Tipos embutidos têm precedência e não podem ser redefinidos por `extend-type`. Um tipo
sem implementação embutida PODE registrar as capacidades por tipo nominal. O dispatch
não pode depender da ordem em que `defprotocol`, `defmulti` ou arquivos do programa
foram analisados.

A superfície de linguagem para implementar essas capacidades fica atrás de um namespace
nativo de core até existirem `deftype`, impls inline e protocolos multi-aridade
completos. O gate inicial pode validar a extensão com records auxiliares e
`extend-type`, sem prometer compatibilidade com nomes de interfaces JVM.

## Rooting e efeitos

`AssocOne` é `MaySafepoint`: vetor, mapa, sorted-map, record e implementações do usuário
podem alocar.

`Nth` e `NthOr` também são `MaySafepoint` por padrão, pois uma implementação registrada
ou o fallback de uma lazy seq pode executar código e alocar. Uma operação embutida
provadamente não alocadora PODE receber fast path `NoSafepoint`.

Antes de um dispatch genérico:

- receptor, chave/índice, valor e `not-found` vivos devem estar rooteados;
- num `assoc` com vários pares, o acumulador deve substituir seu root depois de cada
  passo;
- argumentos já avaliados que serão usados por passos posteriores devem permanecer
  alcançáveis;
- uma falha ou unwind não pode deixar o shadow-stack desequilibrado.

Estas regras refinam a classificação de efeitos da
[ADR-0006](adr/0006-codegen-optimization.md).

## Compartilhamento estrutural

O gate de `assoc` NÃO aceita cópia integral das estruturas escaláveis no runtime nativo.

| Estrutura | Limite esperado por atualização |
| --- | --- |
| vetor, índice na tail | novo wrapper + nova tail; root compartilhado |
| vetor, índice na trie | novo wrapper + O(log32 n) nós; demais caminhos compartilhados |
| HAMT | novo wrapper + O(log32 n) nós esperados; ramos não percorridos compartilhados |
| LLRB | novo wrapper + O(log n) nós, incluindo rotações persistentes |
| record | novo record + custo do mapa interno |
| array-map | cópia integral permitida enquanto limitado a 8 entradas |

Testes internos devem comparar endereços de nós não alterados e contar alocações por
operação. Tempo de benchmark sozinho não prova persistência nem compartilhamento.

O interpretador de bootstrap pode manter uma representação mais simples, mas deve ter a
mesma semântica e deve registrar separadamente qualquer cópia O(n) que não seja aceitável
no runtime final.

## Erros

As categorias semânticas são:

| Categoria | Exemplos |
| --- | --- |
| aridade | `assoc` sem par, par incompleto, `nth` com aridade diferente de 2 ou 3 |
| tipo de índice/chave | `nth` com keyword; `assoc` de vetor com chave não inteira |
| limites | índice negativo ou além do limite permitido |
| receptor sem suporte | `assoc` em lista; `nth` em mapa ou set |
| transient inválido | `nth` após `persistent!`; `assoc` aplicado a transient |

Mensagens textuais não são oracle. Quando a hierarquia de exceções estiver disponível,
essas falhas devem ser capturáveis com categoria e operação estáveis. Até lá, a matriz
pode validar diagnóstico e status do processo, mas não deve confundir término fatal com
o contrato final.

## Matriz mínima de conformidade

### `assoc`

- substituição e inserção em array-map, HAMT e sorted-map;
- preservação do comparador e da família ordenada;
- substituição, append e erro de gap em vetor;
- dois ou mais pares, inclusive appends encadeados em vetor;
- `nil` promovido a mapa;
- field e extension key em record, preservando o tipo nominal;
- receptor antigo e novo usados depois da atualização, sob `CLJN_GC_STRESS=1`;
- chave e valor heap vivos durante promoção de array-map para HAMT;
- aridade inválida, chave de vetor inválida, limites e receptor sem suporte;
- dispatch de um tipo extensível quando essa superfície for ativada.

### `nth`

- vetor na tail, em cada nível de trie e nos limites 31/32, 1023/1024;
- lista/cons em zero, meio, último, negativo e além do fim;
- `nil` nas duas aridades;
- `not-found` igual a `nil`, `false` e a um objeto heap;
- tipo sem suporte com e sem `not-found`;
- índice de tipo inválido;
- string ASCII, multibyte e caractere fora do BMP quando `Char` estiver ativo;
- map entry e `Bytes` quando os tipos existirem;
- tipo extensível, incluindo `not-found`;
- sequência lazy sem precontagem quando lazy seqs existirem;
- todos os caminhos relevantes sob GC stress e sanitizers.

Casos de resultado devem ser comparados ao oracle Clojure/JVM 1.12.5. Divergências
intencionais, como indexação Unicode de string, usam expectativa nativa explícita e
justificativa no `case.toml`.

## Critérios de aceite

O gate é concluído quando:

1. interpretador e executável aceitam as mesmas aridades e produzem os mesmos resultados;
2. a matriz mínima aplicável aos tipos ativos está marcada `active`;
3. `assoc` variádico preserva ordem de avaliação, persistência e rooting;
4. `nth` implementa as duas aridades, `nil`, fallback sequencial e distinção entre
   `not-found` e tipo sem suporte;
5. um tipo não embutido participa por capability dispatch sem alterar o `switch` central;
6. testes internos comprovam os limites de path-copying;
7. GC stress, ASan e UBSan não encontram falhas;
8. benchmarks de leitura indexada e atualização não apresentam regressão material
   contra o baseline registrado.

## Referências semânticas

- [Clojure `assoc` e `nth`](https://clojure.github.io/clojure/clojure.core-api.html)
- [Estruturas de dados de Clojure](https://clojure.org/reference/data_structures)
- [Dispatch de `assoc` e `nth` no runtime JVM](https://github.com/clojure/clojure/blob/master/src/jvm/clojure/lang/RT.java)
- [Precedente portátil de `IAssociative`](https://cljs.github.io/api/cljs.core/IAssociative)
- [Precedente portátil de `IIndexed`](https://cljs.github.io/api/cljs.core/nth)
