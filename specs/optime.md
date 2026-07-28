# Plano de otimização do codegen numérico e do rooting de GC

Status: **parcialmente implementado**

Escopo principal: `clojure-codegen`, ABI do runtime compilado e testes e2e.

Decisão arquitetural: [ADR-0006](adr/0006-codegen-optimization.md).

Problemas-alvo originais:

1. `+`, `-`, `*`, `quot`, `mod`, `<`, `<=`, `>`, `>=`, `inc` e `dec` atravessavam a
   ABI C mesmo quando os operandos eram fixnums.
2. O contrato de `expr` fazia cada atualização da shadow stack por chamadas
   `cljn_gc_set`, `cljn_gc_push` e `cljn_gc_popn`.

Fast paths para essas operações e loads/stores diretos de roots já foram entregues. O
rooting continua eager; as seções de liveness/safepoints descrevem o principal trabalho
restante. `CodegenOptions` também já expõe `none`, `speed` e `speed-and-size`, mas
`none` permanece padrão após regressão medida no gate Cormen.

Ganhos posteriores, medidos no `HEAD 476aefd`, ampliaram o escopo original:

- `mapv` e `into` constroem vetores por transiente estrutural;
- acumuladores frescos de `loop` podem ser promovidos automaticamente, incluindo o
  primeiro sumário conservador de parâmetro linear interprocedural (`e87456e`);
- vetores literais constantes de imediatos são cacheados por site e registrados como
  roots permanentes (`1ca1d79`).

Essas entregas reduziram o Cormen de 36,21 s para 29,45 s de parede e de 36,07 s para
29,30 s de CPU na mesma referência JVM preservada. Elas não implementam rooting por
liveness, tuplas de retorno sem heap nem uma IR própria. A análise completa está na
[ADR-0009](adr/0009-benchmark-performance-study.md), e a decisão interprocedural na
[ADR-0010](adr/0010-interprocedural-ephemeral-vectors.md).

Este plano mantém a representação tagged e o coletor mark-sweep preciso, não-móvel e
single-thread. Não troca o GC nem muda a semântica da linguagem.

---

## 1. Resultado esperado

Para um loop numérico como:

```clojure
(loop [i 0 acc 0]
  (if (<= i n)
    (recur (inc i) (+ acc i))
    acc))
```

o corpo quente deve ser reduzido a guardas estritamente necessárias, operações inteiras,
comparação, seleção do boolean tagged e o backedge. Depois de provados os tipos do loop,
não deve haver no corpo:

- chamadas a `cljn_add`, `cljn_le`, `cljn_inc` ou equivalentes;
- chamadas a `cljn_gc_set`, `cljn_gc_push` ou `cljn_gc_popn`;
- escrita no shadow-stack quando não existe safepoint no caminho.

`cljn_gc_enter`/`cljn_gc_leave` podem permanecer uma vez por entrada/saída de função. Uma
função sem nenhum valor heap vivo em safepoint poderá, numa etapa posterior, omitir até
esse frame.

Metas verificáveis:

- zero calls de primitivas numéricas no bloco quente de `examples/loop-benchmark.clj`;
- zero calls ou stores de rooting por iteração em loops sem alocação;
- saída e erros iguais aos atuais, inclusive tipo inválido, overflow e divisão por zero;
- toda a suíte e2e verde com `CLJN_GC_STRESS=1`;
- ganho mínimo inicial de **2x** na mediana do benchmark de loop contra o baseline
  registrado na mesma máquina, sem regressão maior que 5% nos benchmarks alocadores. O
  critério estrutural de zero calls no loop é bloqueante; tempo absoluto não deve ser
  bloqueante em CI compartilhada.

---

## 2. Invariantes que não podem ser quebradas

### Valores numéricos

- Um fixnum continua sendo `(n << 1) | 1`.
- O fast path deve validar `(value & 1) == 1` quando o operando não estiver
  comprovadamente fixnum.
- O intervalo válido precisa ser declarado em um único lugar:
  `FIXNUM_MIN = -(1 << 62)` e `FIXNUM_MAX = (1 << 62) - 1` em alvos de 64 bits.
- `+`, `-`, `*`, `inc` e `dec` precisam detectar resultado fora desse intervalo **antes**
  do retagging. Não basta detectar apenas overflow de `i64`, pois o fixnum tem 63 bits.
- A codificação/decodificação em C deve operar pelos equivalentes unsigned (`uintptr_t`)
  para não executar left shift de inteiro signed negativo, que é comportamento indefinido.
- Falha de guarda ou overflow segue para um slow path de runtime. No escopo atual ele
  produz o mesmo erro; no futuro poderá promover para bigint sem reescrever o fast path.
- Comparações retornam exatamente `TRUEV` ou `FALSEV`.

### GC

- O coletor só pode executar em safepoints conhecidos. No runtime atual eles são as
  chamadas que podem alcançar `obj_alloc`/`maybe_gc`.
- Imediatos (`nil`, boolean, fixnum e outros tags não-ponteiro) nunca precisam ser roots.
- Antes de cada safepoint, todo valor **possivelmente heap** e **vivo depois ou durante**
  o safepoint deve estar em um slot visível ao coletor.
- Como o GC é não-móvel, o valor pode continuar em registrador depois da coleta; não é
  necessário recarregá-lo do root slot.
- Não existe GC assíncrono no modelo single-thread. Entre safepoints, registradores e
  variáveis Cranelift não precisam ser espelhados.
- Argumentos de uma chamada que pode alocar durante a avaliação de argumentos seguintes
  precisam ser rooteados enquanto vivos. A remoção de pushes não pode alterar a ordem de
  avaliação.
- Todo caminho normal, `if`, `recur`, retorno e erro deve manter o mesmo estado lógico de
  roots nos pontos de junção.

Essas invariantes substituem o contrato atual “todo `Flow::Val` está uma vez no topo do
shadow-stack”.

---

## 3. Desenho proposto

### 3.1 Metadados de efeito

Centralizar em `clojure-codegen` a descrição de cada operação:

```text
CallEffect = NoSafepoint | MaySafepoint
ValueKind  = Fixnum | Immediate | MaybeHeap
```

- `NoSafepoint`: aritmética, comparação, truthiness, acessos que não alocam e helpers
  comprovadamente não-alocadores.
- `MaySafepoint`: construtores, concatenação/conversão que aloca, operações persistentes
  que copiam, criação de closure e chamadas de função Clojure diretas ou indiretas.
- `ValueKind` começa conservador. Literais e resultados de fast paths têm tipo conhecido;
  parâmetros, capturas, chamadas dinâmicas e operações gerais começam como `MaybeHeap`.

Nenhuma call nova deve ser emitida sem declarar seu efeito. Uma mudança no runtime que
faça uma operação `NoSafepoint` alocar exige mudar o metadado no mesmo commit.

### 3.2 Lowering direto de fixnums

Criar helpers de codegen específicos, sem reutilizar os helpers genéricos `call1` e
`call2`:

```text
emit_fixnum_guard(value)
emit_fixnum_unbox(value)       ; sshr_imm 1
emit_fixnum_retag(raw)         ; ishl_imm 1 + bor 1
emit_fixnum_range_guard(raw)
emit_numeric_slow_path(op, ...)
```

Lowering inicial:

| Primitiva | Fast path Cranelift |
| --- | --- |
| `+` | `iadd` nos valores sem tag, checagem do intervalo fixnum e retag |
| `-` | `isub` nos valores sem tag, checagem do intervalo fixnum e retag |
| `*` | `smul_overflow`, checagem de overflow e do intervalo fixnum, retag |
| `inc`/`dec` | soma/subtração de 1 no valor sem tag, checagem de intervalo e retag |
| `< <= > >=` | `icmp` signed nos valores sem tag e `select(TRUEV, FALSEV)` |
| `=` | identidade tagged como fast path; igualdade estrutural continua no runtime |

Para soma e subtração de dois fixnums, o resultado intermediário cabe em `i64`, mas ainda
pode exceder o intervalo de fixnum; a checagem de limites continua obrigatória.

`quot` e `mod` ficam na segunda leva: além das guardas de tipo, precisam preservar divisão
por zero, o caso mínimo dividido por `-1` e a regra atual de sinal de `mod`.

O primeiro corte usa guardas locais e slow path. Depois, fatos de tipo eliminam guardas
redundantes:

- literal inteiro e resultado de operação validada são `Fixnum`;
- `let` propaga o fato para o slot;
- `if` faz a interseção dos fatos dos ramos;
- `loop`/`recur` calcula ponto fixo para os valores carregados pelo backedge;
- valores invariantes no loop têm a guarda movida para o preheader;
- parâmetros desconhecidos continuam guardados antes do primeiro uso numérico.

Unboxing permanente de locais de loop é uma otimização posterior e condicionada a
benchmark. O primeiro marco já deve remover as calls de runtime mantendo os valores
tagged entre operações.

### 3.3 Frame fixo de roots

Substituir `push/popn/set` por slots fixos planejados por função:

```text
RootPlan {
  frame_slots,
  local_to_root_slot,
  temp_intervals,
  safepoints
}
```

Um prepass sobre a AST calcula o máximo de valores `MaybeHeap` simultaneamente vivos em
safepoints e atribui slots reutilizáveis. A implementação pode começar conservadora
(`local_count + máximo de temporários vivos`) e compactar slots depois.

Alterar a ABI interna do frame para que `cljn_gc_enter(frame_slots)` devolva um ponteiro
para a área reservada. O código gerado escreve diretamente com `store` Cranelift em
`base + slot * sizeof(Value)`. `cljn_gc_leave(base)` restaura o stack pointer. Durante a
migração, `cljn_gc_push/popn/set` permanecem no runtime para compatibilidade, mas o novo
codegen deixa de importá-los.

Benefícios desse passo, mesmo antes da liveness completa:

- elimina uma chamada C por `gc_set`, `gc_push` e `gc_popn`;
- torna explícito e inspecionável o layout de roots por função;
- permite que o passo seguinte mova stores para safepoints sem alterar novamente a ABI.

### 3.4 Rooting orientado a safepoint e liveness

Adicionar ao `FnGen` um `RootTracker` com:

- valores vivos e seu `ValueKind`;
- slot atribuído, se houver;
- estado `Clean`, `Dirty` ou `Dead`;
- profundidade/estado esperado em cada bloco e alvo de `recur`.

Comportamento:

1. Produzir um valor não gera store automaticamente.
2. Vincular/religar um local atualiza a variável Cranelift e marca sua root como `Dirty`;
   não escreve imediatamente.
3. Antes de uma operação `MaySafepoint`, `flush_roots()` grava somente valores
   `MaybeHeap`, vivos e `Dirty`.
4. Valores mortos liberam o slot logicamente. A limpeza física pode ser adiada até o
   próximo safepoint, evitando store de `NIL` por expressão.
5. Operações `NoSafepoint`, inclusive todo fast path numérico, não fazem flush.
6. Junções de `if` e backedges de `recur` reconciliam o mapa de slots de forma
   determinística. Se a análise não provar um estado, usa o estado conservador.
7. O valor retornado por uma call `MaySafepoint` só é rooteado se ficar vivo até outro
   safepoint.

Para chamadas Clojure, o contrato será:

- `gc_enter` ocorre no prólogo; os parâmetros entram no `RootTracker` e são flushados, no
  máximo, imediatamente antes da primeira operação `MaySafepoint` do callee;
- não há coleta na transição caller/callee;
- argumentos anteriores só são rooteados no caller se a avaliação de argumentos
  posteriores puder atingir safepoint;
- outros valores do caller vivos depois da call são flushados antes da call.

O analisador pode continuar emitindo `local_count`; `RootPlan` é responsabilidade do
codegen neste corte. Se a liveness crescer além de um prepass local simples, ela deve
migrar para a LIR/ANF prevista em `COMPILER_PIPELINE.md`, sem duplicar uma SSA própria no
backend.

### 3.5 Configuração do Cranelift

O codegen originalmente fixava `opt_level = "none"`. `CodegenOptions` e a CLI já
expõem:

```text
Debug   -> opt_level none
Release -> opt_level speed
```

Implementado: `CodegenOptions` e a CLI aceitam
`--opt-level none|speed|speed-and-size`. O nível `speed` ativa o pipeline de otimização
do Cranelift, mas não habilita inlining por si só; a política de inlining e o acesso aos
corpos das funções continuam sendo responsabilidade do frontend.

O gate Cormen de 2026-07-26 rejeitou temporariamente a promoção de `speed` a padrão:
97,74 s contra 93,08 s de `none`, com regressão em 25/30 casos. O padrão continua
`none` até reduzir os spills e o crescimento de frames observado no IR atual.

Os testes estruturais devem passar nos dois modos; os benchmarks registram o nível
explicitamente. A
correção não pode depender de o otimizador remover calls ou stores que o frontend emitiu.

---

## 4. Fases de implementação

### Fase 0 — Baseline e observabilidade

**Objetivo:** provar onde está o custo e impedir que ele volte.

Tarefas:

- criar um benchmark curto e determinístico derivado de
  `examples/loop-benchmark.clj`, separando:
  - loop inteiro sem alocação;
  - loop com call de função;
  - loop que aloca `cons`;
- registrar mediana, dispersão e contadores das calls `cljn_*`;
- adicionar uma forma de obter o CLIF gerado em teste (`compile_ir_for_test` ou dump
  controlado por opção), sem depender de `objdump` específico de plataforma;
- criar goldens/asserts que contem calls a primitivas e helpers de GC dentro do bloco
  quente;
- opcionalmente adicionar `CLJN_RUNTIME_STATS=1`, compilado/desligado por padrão, para
  imprimir contadores de primitivas, roots, alocações e coleções.

**Aceite:** baseline versionado com comando de reprodução e um teste capaz de falhar se
`cljn_add` ou `cljn_gc_push` reaparecer no loop otimizado.

### Fase 1 — Efeitos e limites numéricos explícitos

**Objetivo:** preparar uma fronteira segura para as duas otimizações.

Tarefas:

- definir `CallEffect`, `ValueKind` e a tabela única de efeitos;
- declarar os limites de fixnum em Rust e C e testar que coincidem;
- endurecer os slow paths de `runtime.c` para checar o intervalo de fixnum antes de
  `MK_FIX`, evitando shift/overflow indefinido em C;
- classificar todas as funções atualmente importadas por `Runtime`;
- documentar quais helpers podem alocar.

**Testes:** limites mínimo/máximo, um passo além dos limites, tipo não-numérico, soma,
subtração e multiplicação com overflow; teste que cada import tem efeito declarado.

**Aceite:** sem mudança de saída válida; erros atuais preservados; sanitizers sem UB nos
casos-limite.

### Fase 2 — Fast paths inteiros no Cranelift

**Objetivo:** retirar a ABI C do caminho numérico comum.

Ordem:

1. `inc`, `dec`, `<`, `<=`, `>`, `>=`;
2. `+`, `-` e folds variádicos;
3. `*`;
4. identidade rápida de `=`;
5. `quot` e `mod`, depois que os casos excepcionais estiverem cobertos.

Cada operação entra com:

- fast path guardado;
- slow path compartilhado por função/operação para evitar duplicação excessiva;
- propagação básica de `ValueKind`;
- teste de CLIF e teste e2e.

**Aceite:** o loop de referência não chama helpers numéricos no caminho quente; todos os
testes de erro e `CLJN_GC_STRESS=1` continuam verdes.

### Fase 3 — ABI de frame fixo e stores diretos

**Objetivo:** remover as calls de manutenção do shadow-stack antes de mudar sua
frequência.

Tarefas:

- implementar `RootPlan` conservador;
- fazer `gc_enter` devolver a base endereçável do frame;
- emitir stores Cranelift para slots;
- migrar locals e temporários;
- manter asserts de overflow/underflow do shadow-stack no runtime;
- parar de declarar/importar `cljn_gc_set`, `cljn_gc_push` e `cljn_gc_popn` no novo
  objeto.

**Aceite:** nenhum desses três símbolos aparece no objeto gerado; a suíte sob GC-stress
passa antes de qualquer eliminação de store.

### Fase 4 — Spill somente em safepoints

**Objetivo:** chegar a zero tráfego de roots em regiões não-alocadoras.

Tarefas:

- implementar `RootTracker`, dirty tracking e `flush_roots`;
- não reservar root para `Fixnum`/`Immediate`;
- marcar e flushar argumentos/temporários somente quando atravessam `MaySafepoint`;
- reconciliar roots em `if`, `loop` e `recur`;
- limpar roots mortos de forma lazy antes do próximo safepoint;
- permitir frame com zero slots quando a função não precisa de roots;
- adicionar modo de diagnóstico que coleta em toda alocação e valida limites/estado do
  frame.

**Aceite:** zero stores no shadow-stack dentro do loop inteiro; testes alocadores e de
closures passam sob `CLJN_GC_STRESS=1`, ASan e UBSan.

### Fase 5 — Eliminação de guardas e otimização de loop

**Objetivo:** reduzir o fast path a operações essenciais.

Tarefas:

- propagar `ValueKind` por `let`;
- calcular fatos de `loop`/`recur` por ponto fixo;
- mover guardas invariantes para o preheader;
- juntar slow paths por operação;
- avaliar, por benchmark, manter loop-carried fixnums sem tag e retaggear apenas em
  fronteiras ABI/heap.

**Aceite:** CLIF do benchmark sem guardas redundantes no backedge; qualquer unboxing
adotado tem teste diferencial e ganho mensurável próprio.

### Fase 6 — Estabilização e rollout

**Objetivo:** tornar a melhoria sustentável.

Tarefas:

- rodar suíte normal, `CLJN_GC_STRESS=1`, sanitizers, Linux e Windows;
- comparar benchmark numérico e benchmarks alocadores com o baseline da Fase 0;
- atualizar `MEMORY_MODEL.md`, `COMPILER_PIPELINE.md` e o estado em `specs/README.md`;
- remover os helpers antigos de push/pop/set somente depois de não haver objetos antigos
  ou outro consumidor da ABI;
- documentar o procedimento para classificar uma nova primitiva/runtime call.

**Aceite:** metas da seção 1 satisfeitas e resultados registrados no PR/relatório de
benchmark.

---

## 5. Matriz mínima de testes

| Área | Casos |
| --- | --- |
| Aritmética válida | positivos, negativos, zero, folds com vários argumentos |
| Limites | `FIXNUM_MIN`, `FIXNUM_MAX`, overflow nas duas direções, multiplicação grande |
| Tipos dinâmicos | string, lista, `nil` e boolean usados em cada operação numérica |
| Comparação | todos os operadores, igualdade e limites |
| Controle | fast path em `if`, `let`, `loop/recur`, ramos divergentes |
| GC de locals | local heap vivo através de construção/alocação |
| GC de temporários | primeiro argumento heap vivo enquanto o segundo argumento aloca |
| Closures | captura viva durante criação e chamada direta/indireta |
| Coleções | `assoc`, `conj`, `str`, listas e literais sob coleta a cada alocação |
| Retenção | root morto não mantém objetos alcançáveis indefinidamente |
| Estrutural | contagem de calls/stores no CLIF e ausência de símbolos antigos no objeto |
| Performance | loop puro, loop com call, loop alocador; debug e release separados |

Casos sentinela de rooting:

```clojure
;; `x` precisa sobreviver à alocação feita ao avaliar o outro operando/argumento.
(let [x (list 1 2 3)]
  (cons x (list 4 5 6)))

;; `acc` é loop-carried e heap; `i` é imediato e não precisa de root.
(loop [i 0 acc (list)]
  (if (< i 1000)
    (recur (inc i) (cons i acc))
    acc))
```

Além da saída, os testes de GC devem rodar com heap pequeno/threshold baixo para provocar
coleta nos pontos mais desfavoráveis.

---

## 6. Riscos e mitigação

| Risco | Mitigação |
| --- | --- |
| Valor heap vivo apenas em registrador durante coleta | tabela conservadora de efeitos, flush antes de todo `MaySafepoint`, GC-stress e casos sentinela |
| Runtime `NoSafepoint` passar a alocar | metadado obrigatório no mesmo local da declaração e revisão/teste de contrato |
| Overflow errado por confundir `i64` com fixnum de 63 bits | limites explícitos, slow path e testes nos quatro extremos |
| Explosão de blocos por guardas/slow paths | slow blocks compartilhados e eliminação de guardas por fatos de tipo |
| Estado de roots diferente nos ramos/backedge | `RootTracker` com merge conservador e asserts no modo diagnóstico |
| Retenção por slots mortos não limpos | limpeza lazy antes do próximo safepoint e teste de RSS/reclamação |
| Otimização dependente de x86_64 | usar instruções Cranelift portáveis e manter matriz Linux/Windows; não testar por assembly específico |
| Mudança ampla demais no AST codegen recursivo | entregar frame fixo primeiro; migrar liveness para LIR/ANF se o prepass deixar de ser local e verificável |

---

## 7. Sequência de entrega recomendada

Cada item deve caber em um PR revisável:

1. benchmark + dump/teste de CLIF;
2. limites de fixnum + hardening do runtime;
3. `CallEffect`/`ValueKind`;
4. comparação e `inc`/`dec` diretos;
5. `+`/`-`/`*` diretos;
6. `RootPlan` e ABI de frame fixo;
7. stores diretos e retirada das calls `push/popn/set`;
8. rooting por safepoint para temporários;
9. rooting por safepoint para locals e `recur`;
10. propagação de fatos/hoisting de guardas;
11. `quot`/`mod`, estabilização cross-platform e documentação final.

Não misturar a troca do lowering numérico com a troca completa do protocolo de roots no
mesmo PR: ambos mexem no hot path e, separados, permitem localizar regressões sem abrir
mão do objetivo conjunto.
