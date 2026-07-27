# ADR-0010 — Eliminação interprocedural de vetores efêmeros (escape/uniqueness)

- Status: **proposta** (design; não altera comportamento até implementada)
- Data: 2026-07-27
- Relacionadas: [ADR-0002](0002-memory-management.md),
  [ADR-0006](0006-codegen-optimization.md),
  [ADR-0008](0008-associative-indexed-dispatch.md),
  [ADR-0009](0009-benchmark-performance-study.md) e o vetor transiente estrutural
  (commit `8012102`).

## 1. Contexto

A [ADR-0009](0009-benchmark-performance-study.md) mediu que o desempenho relativo do
executável nativo é previsto quase perfeitamente pelo volume de alocação (Spearman
−0,943), com um teto de vazão de ~20 M objetos/s. Os ganhos **intraprocedurais** já foram
entregues:

- auto-transient de acumulador de `loop` com init de vetor literal (ADR-0009 rec. 2);
- `mapv`/`into` no core compilado usando transiente estrutural.

Restam os **piores** casos da suíte Cormen, e todos têm o vetor efêmero atravessando
fronteiras de função — fora do alcance de qualquer análise intraprocedural:

- **Padrão A — tupla de retorno.** `extended-gcd` devolve `[a 1 0]` / `[gcd y1 …]` a cada
  chamada recursiva; o chamador liga o resultado e lê só por `(nth result <const>)`.
  `binary-search-tree` representa nós como `[val esq dir]`. A JVM elimina essas alocações
  com **escape analysis + scalar replacement**.
- **Padrão B — acumulador threaded.** `zero-one-knapsack` passa `best` (init literal) por
  `apply-item`, que o consome linearmente num loop de `assoc` e devolve a versão
  atualizada. `LCS`, `rod-cutting`, `matrix-chain` seguem o mesmo formato. A JVM usa
  TLAB + as implementações transientes internas de `reduce`/`into`.

Esta ADR decide **como** eliminar esses vetores com segurança. Ela **não** troca o
backend, a representação de valores, nem o coletor não-móvel.

## 2. Forças de decisão

- Ganho real nos casos alocação-pesados sem regredir os já rápidos.
- **Solidez acima de tudo:** uma classificação incorreta de "único/linear" causa mutação
  in-place observável — **corrupção silenciosa**, não um crash. Paridade de checksum é
  necessária mas **não suficiente** (um falso positivo pode não corromper na entrada
  testada). A análise DEVE ser conservadora e o argumento de correção, explícito.
- Preservar o rooting não-móvel da ADR-0006 (nada se move; slots endereçáveis).
- Manter o interpretador de bootstrap semanticamente equivalente (sem exigir a mesma
  representação — a otimização é transparente).
- Ser incremental e reversível: começar por um subconjunto sintático estreito e ampliar.

## 3. Alternativas consideradas

| Alternativa | Vantagens | Desvantagens | Veredito |
| --- | --- | --- | --- |
| Só intraprocedural (já feito) | simples, seguro | não alcança os piores casos (interprocedurais) | insuficiente |
| Inlining de funções + scalar replacement (como a JVM) | genérico | não elimina alocação em funções **recursivas** (euclid); grande e caro | parcial |
| ABI de múltiplos valores de retorno para toda função | uniforme | muda a convenção de chamada global; custo em quem não é tupla | rejeitada |
| Alocar a tupla no heap mas em pool dedicado | pouca análise | não remove a alocação, só a barateia; a ADR-0009 mostra que o custo é alocar | rejeitada |
| **Análise de unicidade/escape interprocedural + dois transforms guardados** | remove a alocação nos padrões reais; incremental; sã por construção | exige sumários de função por ponto fixo e metadados de efeito corretos | **escolhida** |

## 4. Decisão

Adotar uma **análise de unicidade (linearidade) interprocedural**, baseada em **sumários
de função** calculados por ponto fixo sobre o grafo de chamadas, que habilita dois
transforms guardados. Qualquer dúvida cancela o transform naquele ponto (fallback ao
caminho persistente) — **conservadora por construção**.

### 4.1 Reticulado de unicidade

Cada valor de vetor num ponto do programa é `Único` ou `Compartilhado`.

- Um literal de vetor (`[…]`, `(vector …)`) nasce **Único**.
- Um valor Único que é **consumido exatamente uma vez** por uma operação que o destrói
  produzindo um novo vetor (`conj`/`assoc`) continua a cadeia como Único; se for usado
  **de novo depois** desse consumo (aliasing), tanto o consumo quanto os usos posteriores
  o rebaixam a `Compartilhado`.
- Leituras não-retentivas (`nth`/`get`/`count`/`contains?`) **não** consomem: devolvem um
  elemento, não o vetor; o valor permanece Único para o próximo uso.
- Guardar num literal de coleção, capturar numa closure, ou passar a um parâmetro
  **não** marcado como linear → `Compartilhado`.

Mutar in-place é observacionalmente equivalente a copiar **somente** quando o valor é
Único (há exatamente uma referência alcançável).

### 4.2 Sumários de função (ponto fixo interprocedural)

Para cada função de topo `f(p1..pn)` computa-se, por ponto fixo (monótono; começa
otimista e rebaixa):

- `retorna_vetor_fresco(f) ∈ {Não, Sim(k)}` — **toda** saída em posição de cauda de `f` é
  um literal de vetor de aridade `k`, ou uma chamada a `g` com `retorna_vetor_fresco(g) =
  Sim(k)`, ou uma cadeia de `conj`/`assoc` sobre um valor Único (padrão A).
- Para cada parâmetro `pi`: `linear(f, pi) ∈ bool` — `pi` é usado linearmente no corpo de
  `f` (só `conj`/`assoc`/leituras, threaded por `loop`/`recur` do próprio slot e retornado),
  e o retorno de `f` é derivado de `pi`. Ou seja, passar um valor Único a `pi` preserva a
  unicidade do valor retornado (padrão B).

Recursão é tratada pelo ponto fixo (mutuamente recursivas convergem). Uma função cujo
corpo faz algo não reconhecido com `pi` (passa a outra função não-linear, captura, guarda)
tem `linear(f, pi) = falso`.

### 4.3 Transform A — tuplas de retorno sem heap

Se `retorna_vetor_fresco(f) = Sim(k)` **e** todos os call sites de `f` ligam o resultado e
o usam **só** por `(nth r <const>)`/`(get r <const>)` com `const < k` (nunca o vetor
inteiro), então:

- `f` passa a escrever os `k` componentes em **slots de saída no shadow-stack** fornecidos
  pelo chamador (out-params), em vez de alocar um `PersistentVector`. Como o shadow-stack é
  não-móvel e já é a raiz de GC (ADR-0006), cada frame reserva `k` slots para a tupla do
  seu callee; nada se move.
- Cada call site lê os `k` slots diretamente nas posições onde havia `(nth r i)`.

Prefere-se out-slots no shadow-stack a uma ABI de múltiplos retornos porque reusa a
infraestrutura de rooting existente e não altera a convenção das demais funções. Se algum
call site não destruturar (usar `r` inteiro), `f` **não** é convertida (fallback ao vetor).

### 4.4 Transform B — acumulador linear threaded vira transiente

Um valor de vetor Único cuja cadeia de consumo — incluindo passagens por parâmetros
`linear` — preserva a unicidade em todos os pontos é construído como **transiente
estrutural**: `conj`/`assoc` viram `conj!`/`assoc!`; funções `linear(f, pi)` recebem e
devolvem o transiente; `persistent!` é inserido na fronteira final (onde o valor deixa de
ser Único ou escapa). Reusa o vetor transiente estrutural já existente (mutação in-place
O(1), `persistent!` O(1)).

### 4.5 Efeitos, GC e rooting

- Os dois transforms permanecem compatíveis com a ADR-0006: valores vivos em safepoints
  estão na shadow-stack; os out-slots da tupla são slots de root normais.
- `conj!`/`assoc!` sobre transiente e a escrita em out-slots são `MaySafepoint` no caso
  geral (alocam nós internos), então o rooting segue as regras da ADR-0006.
- O modelo é seguro pelas propriedades da ADR-0002: coleta single-thread, não-móvel, só em
  safepoints; um valor Único mutado in-place nunca tem outra referência viva que o observe.

## 5. Argumento de solidez

A transformação é correta se, **em todo ponto onde se muta in-place ou se escreve num
out-slot, o valor tem exatamente uma referência alcançável** (é Único). O reticulado da
§4.1 e os sumários da §4.2 são um sistema de tipos de unicidade conservador:

1. Todo literal de vetor começa Único (recém-alocado, sem alias).
2. Cada regra de transição só mantém `Único` quando prova consumo-único; qualquer uso
   ambíguo rebaixa para `Compartilhado`, que **desliga** o transform naquele ponto.
3. Fronteiras de função só propagam unicidade por parâmetros comprovadamente `linear`, cujo
   sumário foi obtido por ponto fixo conservador (rebaixa na dúvida).
4. Portanto, um valor classificado Único nunca tem segunda referência viva; mutá-lo é
   indistinguível de path-copying.

A paridade de checksum e o oracle validam **a implementação** (que ela não regride
resultados nos casos testados), mas **não provam** a solidez da análise (um falso positivo
pode ser silencioso). Por isso a decisão exige, além dos testes: (a) a análise começar num
subconjunto sintático estreito e explícito; (b) o transform ser **gated por flag**,
desligável, com o caminho persistente sempre correto; (c) revisão do sistema de unicidade
como artefato próprio; (d) fuzzing diferencial (programas aleatórios no subconjunto,
comparando saída com/sem o transform e contra o interpretador).

## 6. Consequências

### Positivas

- Remove a alocação nos padrões que dominam os piores casos Cormen (euclid, BST, DP).
- Reusa o transiente estrutural e o shadow-stack não-móvel — sem novo mecanismo de memória.
- Incremental: o subconjunto sintático pode crescer sem reabrir a decisão.

### Custos

- O analyzer passa a manter sumários de função e um ponto fixo sobre o grafo de chamadas.
- Metadados de unicidade/linearidade por parâmetro; risco de complexidade.
- Transform A introduz uma convenção de out-slots para funções-tupla.

### Riscos e mitigação

| Risco | Mitigação |
| --- | --- |
| Unicidade incorreta → corrupção silenciosa | análise conservadora; subconjunto estreito; flag desligável; caminho persistente sempre correto; fuzzing diferencial + oracle + GC stress + ASan/UBSan |
| Ponto fixo não converge / recursão mútua | reticulado finito e monótono (Único→Compartilhado; Sim(k)→Não); iterar até estabilizar |
| Tupla escapa por um call site esquecido | exigir que **todos** os call sites destruturem; senão não converter |
| Divergência interp × executável | otimização transparente; teste diferencial contra o interpretador |

## 7. Critérios de validação

A decisão é considerada implementada quando:

1. `extended-euclid`, `binary-search-tree` (padrão A) e `zero-one-knapsack`/`LCS`/
   `rod-cutting` (padrão B) reduzem materialmente a contagem de alocação (teste estrutural
   de contagem, não só tempo);
2. paridade de checksum em toda a suíte Cormen e no oracle, com o transform ligado;
3. fuzzing diferencial (subconjunto) não encontra divergência entre ligado/desligado nem
   contra o interpretador;
4. GC stress, ASan e UBSan limpos nos casos transformados;
5. nenhum caso já rápido regride; o transform é desligável por flag.

## 8. Relação com outras decisões

- **ADR-0002:** depende de GC não-móvel/single-thread; a mutação de valores Únicos é segura
  só sob essas propriedades.
- **ADR-0006:** refina a classificação de efeitos e usa slots de root endereçáveis; os
  out-slots de tupla são roots normais.
- **ADR-0008:** `nth`/`get` por índice constante são o gatilho de destruturação do padrão A.
- **ADR-0009:** este documento implementa a **recomendação 1** daquele estudo; a
  recomendação 2 (auto-transient intraprocedural) já está entregue.
- **Transiente estrutural (`8012102`):** pré-requisito do transform B.

Uma futura coleta móvel/assíncrona ou a fusão desta análise com inlining de funções exige
reabrir esta ADR. Remover a exigência de subconjunto conservador ou o gate por flag também
exige reabri-la.
