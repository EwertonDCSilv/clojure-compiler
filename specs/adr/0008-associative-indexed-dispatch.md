# ADR-0008 — Dispatch por capacidades para `assoc` e `nth`

- Status: **proposta**
- Data: 2026-07-27
- Relacionadas: [ADR-0002](0002-memory-management.md),
  [ADR-0003](0003-value-representation.md),
  [ADR-0005](0005-bootstrap-strategy.md),
  [ADR-0006](0006-codegen-optimization.md) e
  [ASSOCIATIVE_INDEXED_SPEC](../ASSOCIATIVE_INDEXED_SPEC.md)

## Contexto

`assoc` e `nth` existem nos dois caminhos de execução, mas não possuem hoje um contrato
único.

No interpretador, `assoc` aceita vários pares e `nth` aceita `not-found` e string. No
analyzer/codegen, ambos são primitivas de aridade fixa. No runtime C, `cljn_assoc` e
`cljn_nth` fazem dispatch por um `switch` de tags: acrescentar um tipo exige editar as
operações centrais, e um tipo definido pelo programa não pode participar.

O problema não é a representação das coleções. Vetores já usam trie bitmap e
path-copying; mapas usam array-map/HAMT; sorted-maps usam LLRB; records delegam ao mapa
interno. Substituir essas estruturas perderia trabalho correto. O problema é a fronteira
fechada e a duplicação da semântica.

Clojure/JVM delega a atualização unitária à interface `Associative`, usa `Indexed` para
o caminho direto de `nth` e mantém fallback para tipos sequenciais. ClojureScript mostra
que `IAssociative` e `IIndexed` também funcionam como abstrações portáteis sem depender
de classes Java.

## Forças de decisão

- manter acesso de vetor e atualização de coleções embutidas sem overhead desnecessário;
- permitir records e tipos futuros sem ampliar todo `switch` de operações;
- preservar a semântica de `nil`, múltiplos pares e `not-found`;
- não misturar `assoc` persistente com `assoc!` transient;
- centralizar o contrato entre interpretador e executável;
- manter roots corretos quando dispatch ou fallback executar código Clojure;
- permitir especialização futura pelo analyzer sem alterar comportamento;
- não expor interfaces JVM como requisito do runtime nativo.

## Alternativas consideradas

| Alternativa | Vantagens | Desvantagens | Veredito |
| --- | --- | --- | --- |
| Continuar ampliando `match`/`switch` por operação | simples e rápido para os tipos atuais | conjunto fechado, duplicação Rust/C e nenhuma extensão pelo programa | rejeitada |
| Implementar `nth` apenas como `first`/`rest` | uma única abstração de sequência | transforma vetor O(log32 n) em O(n), perde `not-found` especializado | rejeitada |
| Enviar toda chamada ao mecanismo geral de protocolos | máxima uniformidade e extensão | aloca/prepara argv e faz lookup mesmo para vetor/HAMT conhecidos | rejeitada como caminho único |
| Criar vtables C diferentes para cada coleção | dispatch eficiente | duplica o mecanismo de protocolos, complica tipos do usuário e bootstrap | rejeitada como API independente |
| Fast path embutido + capability dispatch + fallback sequencial | preserva custo atual, abre extensão e mantém semântica portátil | exige IDs reservados, efeitos conservadores e dois níveis de dispatch | **escolhida** |

## Decisão

Adotar três operações internas de core:

```text
AssocOne(coll, key, value)
Nth(coll, index)
NthOr(coll, index, not-found)
```

Elas formam capacidades conceitualmente equivalentes a
`IAssociative/-assoc` e `IIndexed/-nth`. Os IDs dessas operações pertencem a um espaço
reservado e estável dentro do artefato compilado; não são numerados conforme a ordem das
formas do programa e não colidem com protocolos ou multimétodos do usuário.

### `assoc` em duas camadas

`assoc` público valida a aridade e dobra os pares da esquerda para a direita sobre
`AssocOne`. A operação unitária:

1. trata `nil` como mapa vazio;
2. usa a implementação nativa quando a tag tiver capacidade associativa embutida;
3. consulta a implementação registrada para um tipo nominal;
4. falha com receptor sem suporte.

O fast path embutido continua chamando as rotinas atuais de vetor, array-map/HAMT,
sorted-map e record. Portanto, promoção de mapa, comparador e path-copying continuam
responsabilidade de cada estrutura, não do dispatch genérico.

Todos os argumentos da chamada pública são avaliados antes da dobra. Um lowering que
intercale avaliação e `AssocOne` é incorreto, ainda que produza o mesmo resultado em
programas puros.

### `nth` em três camadas

`nth` público valida o índice e então:

1. usa a operação indexada da tag embutida;
2. consulta `Nth` ou `NthOr` para tipo nominal registrado;
3. percorre incrementalmente um valor sequencial;
4. falha se o receptor não for indexado nem sequencial.

Depois de validar o índice, `nil` é tratado antes do dispatch: devolve `nil` na aridade
2 e `not-found` na aridade 3. A aridade 3 só absorve índice fora dos limites; ela não
absorve erro de tipo ou receptor sem suporte.

As duas aridades usam operações internas distintas. Isso permite implementar o gate sem
depender imediatamente de overload de métodos no protocolo inicial e evita expor um
sentinela interno ao código Clojure.

### Precedência e extensão

Implementações embutidas têm precedência e não podem ser substituídas por `extend-type`.
Isso corresponde ao fato de que a semântica dos tipos fundamentais faz parte do core.

Tipos sem implementação embutida podem registrar capacidades usando a infraestrutura de
protocolos já existente, ampliada com:

- namespace de IDs separado para operações de core;
- registro de mais de uma operação por capacidade;
- lookup por `type_key` sem depender de ordem de análise;
- erro específico quando uma das operações requeridas estiver ausente.

A superfície pública para declarar essas capacidades será nativa, não uma simulação das
interfaces `clojure.lang.Associative` e `clojure.lang.Indexed`. Seus nomes só serão
congelados quando `deftype`, impls inline e namespaces compilados estiverem prontos. A
ABI interna acima é a decisão estável desta ADR.

### Primitivas e core compilado

O analyzer deixa de tratar as aridades públicas atuais como o contrato do runtime.
Primitivas internas representam `AssocOne`, `Nth` e `NthOr`; o comportamento derivável
de múltiplos pares e fallback sequencial pertence ao core compilado ou a um lowering
equivalente, validado pelos mesmos testes.

O interpretador implementa as mesmas operações conceituais. Não é necessário compartilhar
a representação Rust/C, mas resultados, ordem de avaliação e categorias de erro devem
ser os mesmos.

### Efeitos e GC

`AssocOne`, `Nth` e `NthOr` são `MaySafepoint` no dispatch genérico. `assoc` aloca
normalmente; `nth` pode chamar uma implementação do usuário ou realizar uma lazy seq.

O codegen pode marcar um fast path embutido específico como `NoSafepoint` apenas quando
isso for provado para aquela implementação. Antes de qualquer caminho genérico, todos os
valores vivos devem estar na shadow stack conforme a ADR-0006.

Em `assoc` com vários pares, o acumulador tem um único root atualizado depois de cada
operação. Os pares ainda não consumidos continuam rooteados. Em `nth` com `not-found`,
o default permanece vivo durante dispatch e travessia.

## Consequências

### Positivas

- Novos tipos entram em `assoc`/`nth` sem editar o dispatch central.
- Vetor, HAMT e LLRB mantêm suas implementações e complexidades.
- Aritades públicas deixam de estar acopladas à assinatura da ABI C.
- Interpretador e executável passam a compartilhar um contrato testável.
- `nth` pode crescer para string, `Bytes`, map entry e lazy seq sem perder o caminho
  indexado.
- Especialização futura continua possível quando o analyzer provar a tag.

### Custos

- O runtime passa a ter IDs reservados de operações de core.
- Chamadas de tipo desconhecido são conservadoramente `MaySafepoint`.
- A infraestrutura de protocolo precisa distinguir built-in, extensão ausente e receptor
  sem suporte.
- `nth` em sequência permanece O(n) e pode realizar efeitos de uma lazy seq.
- Congelar uma superfície pública de capabilities depende de trabalho ainda pendente em
  tipos e namespaces.

### Riscos e mitigação

- **Drift entre fast path e protocolo:** rodar o mesmo corpus contra tags embutidas e
  wrappers extensíveis.
- **Colisão de IDs:** usar enum/espaço tipado de core, nunca contador compartilhado.
- **Use-after-free no dispatch:** classificar o caminho genérico como `MaySafepoint` e
  executar a matriz sob GC stress e sanitizers.
- **Cópia acidental O(n):** testes internos de endereços compartilhados e contagem de
  alocações.
- **Fallback sequencial não terminante:** proibir precontagem/materialização e testar
  sequência infinita com índice finito quando lazy seq estiver disponível.
- **Divergência de string:** indexar por valor escalar Unicode e registrar explicitamente
  a incompatibilidade com UTF-16 da JVM.

## Verificação

A decisão é aceita quando:

1. `assoc` aceita um ou mais pares nos caminhos interpretado e compilado;
2. `nth` aceita as aridades 2 e 3 e distingue limites de tipo sem suporte;
3. um tipo nominal sem tag embutida implementa cada capability sem alteração no
   `switch` central;
4. vetores, HAMTs, LLRBs e records passam os testes de compartilhamento estrutural;
5. argumentos heap sobrevivem a dispatch, promoção e fallback sob
   `CLJN_GC_STRESS=1`;
6. o caminho conhecido de vetor não apresenta regressão material contra o baseline;
7. a matriz definida na
   [ASSOCIATIVE_INDEXED_SPEC](../ASSOCIATIVE_INDEXED_SPEC.md) está ativa para todos os
   tipos já entregues.

Uma futura troca da tabela de capabilities por vtables, PICs ou dispatch de IR não exige
nova ADR se mantiver precedência, contrato e capacidade de extensão. Remover a extensão
por tipo, alterar a ordem semântica ou fundir `assoc` com transients exige reabrir esta
decisão.
