# ADR-0009 — Estudo da performance do executável nativo nos benchmarks Cormen

- Status: **estudo** (informativo; orienta decisões futuras, não altera comportamento)
- Data: 2026-07-27
- Medições no commit `8012102`, escala 25×, `--opt-level none`
- Relacionadas: [ADR-0002](0002-memory-management.md),
  [ADR-0003](0003-value-representation.md),
  [ADR-0006](0006-codegen-optimization.md),
  [ADR-0008](0008-associative-indexed-dispatch.md),
  [ADR-0010](0010-interprocedural-ephemeral-vectors.md) e
  [results/README](../../benchmarks/cormen/results/README.md)

## 1. Pergunta

Por que, na suíte Cormen/CLRS (30 casos), o executável nativo é mais lento em **tempo
de parede** que Clojure/JVM em 22 dos 30 casos (mediana `wall_speedup` 0,637×), apesar
de o próprio projeto compilar AOT, sem JVM, com fixnums tagueados e fast paths? Este
documento isola as causas com medições, não com hipóteses.

## 2. Metodologia

- Máquina: AMD Ryzen 5 8600G (6c/12t), 30 GiB, Linux 6.14, GCC 14.2, OpenJDK 21,
  Clojure 1.12.5 AOT. Fonte: [results/README](../../benchmarks/cormen/results/README.md).
- Fonte primária: `benchmarks/cormen/results/extreme.csv` (nativo re-medido no commit
  atual; colunas `clojure_*` preservadas da rodada AOT anterior).
- Instrumentação: contadores temporários em `obj_alloc`/`gc_collect` (revertidos após a
  coleta) para contar **alocações**, **bytes** e **coletas** por caso, na escala 1× de
  cada script.
- Atribuição: reexecução com `CLJN_GC_OFF=1` para separar o custo de **alocar** do custo
  de **coletar**; leitura de `clojure_cpu_percent` para medir o paralelismo da JVM.
- Cada tempo relatado é o **mínimo de 3 execuções** com `ulimit -v` (política do projeto
  para binários compilados).

## 3. Dados

### 3.1 Panorama (escala 25×, cumulativo)

| Métrica | Nativo | Clojure/JVM | Razão |
| --- | --- | --- | --- |
| Tempo de parede acumulado | 39,04 s | 16,91 s | 2,31× (nativo pior) |
| Tempo de **CPU** acumulado | 38,87 s | 32,61 s | 1,19× (nativo pior) |
| Mediana `wall_speedup` | — | — | 0,637× |
| Mediana `cpu_speedup` | — | — | **1,31× a favor do nativo** |
| RSS (mediana da razão JVM/nativo) | — | — | 19,7× menor no nativo |
| `clojure_cpu_percent` (típico) | ~99% | **130–223%** | — |

### 3.2 Por caso (ordenado por alocação; escala 1× de cada script)

| Caso | allocs | MB | wall_spd | cpu_spd | jvm_cpu% |
| --- | ---: | ---: | ---: | ---: | ---: |
| binary-exponentiation | 0 | 0 | 6,33 | 13,67 | 214% |
| iterative-binary-search | 43 | 0 | 2,86 | 6,07 | 213% |
| activity-selection | 50 | 0 | 8,50 | 19,00 | 223% |
| rabin-karp | 58 | 0 | 3,26 | 6,00 | 182% |
| naive-string-matching | 59 | 0 | 1,03 | 2,18 | 212% |
| knuth-morris-pratt | 270 054 | 44 | 1,03 | 2,24 | 212% |
| bellman-ford | 273 135 | 39 | 0,92 | 1,92 | 206% |
| floyd-warshall | 242 578 | 31 | 0,79 | 1,63 | 206% |
| selection-sort | 355 000 | 57 | 0,94 | 1,96 | 204% |
| build-max-heap | 399 000 | 65 | 1,07 | 2,26 | 210% |
| prefix-range-sums | 480 000 | 79 | 0,86 | 1,84 | 211% |
| circular-queue | 497 000 | 80 | 1,05 | 1,98 | 188% |
| horner-polynomial | 540 000 | 90 | 0,62 | 1,37 | 215% |
| chained-hash-table | 720 025 | 87 | 0,84 | 1,70 | 198% |
| insertion-sort | 738 000 | 119 | 0,65 | 1,25 | 190% |
| disjoint-set-union | 765 000 | 105 | 0,45 | 0,92 | 204% |
| counting-sort | 952 000 | 155 | 0,53 | 1,03 | 189% |
| breadth-first-search | 990 083 | 168 | 0,39 | 0,81 | 205% |
| quickselect | 1 080 031 | 35 | 0,67 | 1,39 | 207% |
| matrix-chain-order | 1 210 000 | 147 | 0,36 | 0,68 | 192% |
| depth-first-search | 1 215 069 | 209 | 0,35 | 0,77 | 216% |
| merge-sort | 1 274 000 | 219 | 0,42 | 0,88 | 209% |
| maximum-subarray-divide | 1 384 035 | 47 | 0,35 | 0,73 | 208% |
| topological-sort | 1 384 063 | 231 | 0,36 | 0,72 | 198% |
| binary-search-tree | 1 836 025 | 316 | 0,20 | 0,43 | 210% |
| extended-euclid | 2 016 000 | 347 | 0,21 | 0,41 | 202% |
| rod-cutting | 2 261 025 | 139 | 0,27 | 0,50 | 189% |
| longest-common-subsequence | 3 231 000 | 432 | 0,20 | 0,34 | 168% |
| zero-one-knapsack | 3 990 000 | 446 | 0,19 | 0,32 | 171% |
| sieve-of-eratosthenes | 6 101 550 | 1 097 | 0,31 | 0,41 | 134% |

## 4. Achados

### Achado 1 — A alocação explica quase tudo (correlação de Spearman −0,943)

O `wall_speedup` correlaciona-se com o número de alocações com ρ de Spearman = **−0,943**
(quanto mais aloca, mais lento relativo à JVM). Os cinco casos onde o nativo mais **ganha**
(binary-exp, iterative-binary-search, activity-selection, rabin-karp, naive-string-matching)
fazem **0–59 alocações** no total. Os cinco onde mais **perde** (knapsack, LCS, rod-cutting,
euclid, BST) fazem **2–4 milhões**. Não há caso alocação-pesada em que o nativo ganhe, nem
caso alocação-zero em que perca.

A vazão de alocação do nativo é aproximadamente constante em **~20 M objetos/s**
(intervalo 8–28 M/s). Ou seja, o executável tem um **teto de throughput de alocação**, e o
tempo dos casos pesados é essencialmente `nº_de_alocações / 20M`.

### Achado 2 — O custo é **alocar**, não **coletar**

Com `CLJN_GC_OFF=1` (sem nenhuma coleta), os casos pesados ficam **mais lentos**, não mais
rápidos:

| Caso | GC ligado | GC desligado |
| --- | ---: | ---: |
| zero-one-knapsack | 0,17 s | 0,28 s |
| longest-common-subsequence | 0,17 s | 0,29 s |
| extended-euclid | 0,09 s | 0,19 s |

O coletor mark-sweep com o alocador slab+free-list (ADR-0006/2026) **recicla** memória, o
que mantém o working-set pequeno e cache-quente; desligá-lo faz a memória crescer sem
limite e degrada por cache/TLB. Portanto **a pausa de GC não é o gargalo** — o gargalo é o
trabalho de criar cada objeto (bump/free-list + header + inserção na lista global + a
maquinaria da operação persistente ao redor).

### Achado 3 — Metade do gap de parede é **paralelismo da JVM**

`clojure_cpu_percent` fica entre 130% e 223% (≈2 núcleos): a JVM usa threads de **JIT** e
de **GC paralelo** concorrentes com a thread principal. O nativo é single-thread (~99%),
então parede ≈ CPU. Decompondo o cumulativo:

```
gap_parede (2,31×) = gap_CPU (1,19×) × paralelismo_JVM (~1,93×)
```

Em **tempo de CPU** — o trabalho real — o nativo tem **mediana 1,31× a favor** e vence em
17 dos 30 casos. O gap de parede é inflado pelo fato de a JVM ter runtime multi-thread e o
nativo não.

### Achado 4 — Memória: o nativo usa ~20× menos

O nativo teve RSS menor nos 30 casos (mediana 19,7×). No sieve: 20,7 MiB (nativo) contra
1.024 MiB (JVM). É a contrapartida direta do modelo não-móvel + working-set reciclado.

### Achado 5 — Onde o nativo domina

Loops numéricos/array sem alocação: binary-exp (6,3×), activity-selection (8,5×),
rabin-karp (3,3×), iterative-binary-search (2,9×). Aqui os fast paths de fixnum
(ADR-0006) + AOT + zero pressão de GC batem o JIT com folga, inclusive em parede.

## 5. Causa-raiz

Os casos lentos são **DP/estruturas de dados que alocam vetores pequenos e efêmeros em
volume**, e o padrão exato importa:

- **Vetores como tuplas de retorno** — `extended-euclid` devolve `[a 1 0]` a cada chamada
  recursiva e lê com `nth`. A JVM, via **escape analysis + scalar replacement**, prova que
  a tupla não escapa e **elimina a alocação inteiramente** (vira registradores). O nativo
  não tem essa análise: aloca um PVec + VNode (~320 B) no heap a cada chamada.
- **Tabelas de DP** — `zero-one-knapsack`, `rod-cutting`, `LCS`, `matrix-chain` fazem
  `(assoc tabela i v)` em loop apertado (path-copying persistente). A JVM inlina a máquina
  do PersistentVector e aloca via TLAB; frequentemente usa a implementação transiente
  interna de `into`/`reduce`.
- **Nós de árvore como vetores** — `binary-search-tree` representa nós como `[val esq dir]`
  e reconstrói o caminho a cada inserção.

Do lado da JVM, quatro mecanismos que o nativo hoje **não** tem tornam isso barato:

1. **Escape analysis / scalar replacement** — remove a alocação de valores que não escapam.
2. **TLAB (bump-pointer por thread)** — alocação de ~1–2 instruções.
3. **GC geracional paralelo** — objetos de vida curta morrem quase de graça na young-gen,
   e a coleta roda em outros núcleos.
4. **JIT** — inlina `nth`/`conj`/`assoc` em acesso direto a array e especializa o hot path.

Do lado nativo, o custo por operação é: chamada C cross-ABI + dispatch por tag + alocação
(bump/free-list + header + lista global) + rooting na shadow stack + cópia de nó de 32
slots — multiplicado por milhões. Nenhum item isolado é caro; o produto é.

## 6. Recomendações (priorizadas por alavancagem × risco)

1. **Escape analysis / scalar replacement para vetores curtos não-escapantes.** É a arma
   principal da JVM nos piores casos (euclid, BST, DP como tupla). Detectar literais de
   vetor pequenos que não escapam (retorno lido só por `nth` com índice constante) e
   passá-los desmontados (por registradores / múltiplos valores), sem heap. Maior impacto
   nos casos de menor `wall_speedup`. Risco: análise de escape correta (exige ADR própria).
2. **Auto-transient por linearidade** — para tabelas de DP e acumuladores lineares
   (`assoc`/`conj` numa var de loop que não escapa). Agora viável: o vetor transiente já é
   **estrutural** (`transient`/`persistent!` O(1), commit `8012102`). Impacto direto em
   knapsack, rod-cutting, LCS, sieve. Risco: correção da análise de não-escape.
3. **Inline dos fast paths persistentes (`conj`/`assoc`/`nth`) quando a tag for provada**
   (ADR-0006/0008 já preveem). Reduz o custo por operação do hot path. `nth` já foi inlinado;
   `conj`/`assoc` exigem inlinar alocação (expor o alocador). Ganho médio, risco médio.
4. **Nursery com alocação bump dedicada** — acelera o caminho de alocação sem GC móvel.
   Ganho limitado (o alocador slab já recicla; o Achado 2 mostra que a coleta não é o custo),
   mas o caminho de alocar por si pode encurtar.

## 7. O que os dados dizem para NÃO perseguir

- **Tunar/reduzir a frequência de GC** — o Achado 2 mostra que desligar o GC piora; a coleta
  não é o gargalo.
- **Trocar o alocador por arena pura** — já testado (ADR-0006/2026): a arena sem reciclagem
  fica mais lenta por cache. O slab+free-list atual é o ponto adequado no modelo não-móvel.
- **GC móvel/geracional-copiador só pela performance** — conflita com o rooting não-móvel da
  ADR-0006 (exigiria reabri-la) e o Achado 2 indica retorno baixo para o custo.
- **Perseguir o gap de parede via paralelismo** — cerca de metade do gap é a JVM usar 2
  núcleos (JIT+GC). Replicar isso (GC/JIT em background) é desproporcional ao ganho e ao
  escopo atual single-thread.

## 8. Conclusão

O executável nativo **não é lento de forma difusa**: ele tem um teto de vazão de alocação
(~20 M/s) e é penalizado exatamente na proporção em que o algoritmo aloca vetores efêmeros —
correlação de Spearman −0,943. Em **tempo de CPU** o nativo já é competitivo (mediana 1,31×
a favor) e usa ~20× menos memória; o gap de **parede** é ~metade throughput de alocação e
~metade paralelismo da JVM. As duas alavancas de maior retorno são **eliminar alocações**
(escape analysis para tuplas; auto-transient para acumuladores lineares), não mexer no
coletor. Ambas merecem ADR de decisão própria antes da implementação, pela exigência de
uma análise de escape/linearidade correta.

## 9. Acompanhamento após os ganhos intraprocedurais

As recomendações intraprocedurais foram medidas novamente no commit de código
`663d2d4`: auto-transient de acumuladores de `loop` e `mapv`/`into` construindo por
transiente estrutural. A coleta usou a mesma escala 25× e `--opt-level none`, preservou
integralmente as colunas JVM e publicou a execução mediana de três rodadas completas.

| Métrica Cormen | Baseline `8012102` | `663d2d4` | Variação |
| --- | ---: | ---: | ---: |
| Tempo de parede acumulado | 39,04 s | 36,21 s | -7,25% |
| Tempo de CPU acumulado | 38,87 s | 36,07 s | -7,20% |
| Mediana `wall_speedup` | 0,637× | 0,726× | +14,0% |
| Mediana `cpu_speedup` | 1,310× | 1,515× | +15,6% |
| Mediana da razão RSS JVM/nativo | 19,709× | 20,297× | +3,0% |

O ganho não elimina a causa-raiz: o total nativo de parede ainda é 2,14× o JVM e os
piores padrões continuam atravessando fronteiras de função. Ele confirma, porém, que
reduzir operações persistentes e alocações intermediárias é uma alavanca real: 18 dos
30 casos Cormen melhoraram, incluindo `counting-sort` (-43,9%), `merge-sort` (-23,3%),
`prefix-range-sums` (-34,0%) e o crivo de Eratóstenes (-15,9%). A ADR-0010 permanece
necessária para os vetores efêmeros interprocedurais restantes. A fotografia completa,
as três repetições e os valores por caso estão no
[relatório Cormen](../../benchmarks/cormen/results/README.md).

## 10. Acompanhamento após linearidade interprocedural e hoisting de literais

Os commits `e87456e` e `1ca1d79` implementaram, respectivamente, o primeiro padrão de
acumulador linear interprocedural da ADR-0010 e o cache de vetores literais constantes
compostos apenas por imediatos. A nova coleta manteve escala 25×, `--opt-level none`,
as referências JVM anteriores e o protocolo de três rodadas completas.

| Métrica Cormen | `663d2d4` | `1ca1d79` | Variação |
| --- | ---: | ---: | ---: |
| Tempo de parede acumulado | 36,21 s | 29,45 s | -18,67% |
| Tempo de CPU acumulado | 36,07 s | 29,30 s | -18,77% |
| Mediana `wall_speedup` | 0,726× | 0,958× | +32,0% |
| Mediana `cpu_speedup` | 1,515× | 1,920× | +26,7% |
| Mediana da razão RSS JVM/nativo | 20,297× | 26,296× | +29,6% |

O resultado muda qualitativamente a leitura de CPU: o nativo passa de 36,07 s contra
32,61 s da JVM para **29,30 s**, 10,1% menos CPU acumulada, e vence 19 dos 30 casos
nessa métrica. Em parede ainda perde no agregado (29,45 s contra 16,91 s), mas divide
as vitórias por caso em 15/15 e leva a mediana para perto da paridade.

O ganho aparece nas duas frentes previstas pelo estudo: `zero-one-knapsack`, que
atravessa a fronteira de função com acumulador linear, cai 48,4%; e literais constantes
reduzem `horner-polynomial` em 91,8%, `prefix-range-sums` em 81,8% e `counting-sort` em
65,5%. A regressão agregada de 7,7% no capítulo de grafos, concentrada em BFS/DFS,
permanece como sinal para investigação, sem desfazer o ganho dos outros cinco capítulos.

## 11. Revalidação integral em 2026-07-28

O benchmark foi executado novamente no compilador `1dc69b5`, ainda em escala 25× e
`--opt-level none`. Diferentemente dos acompanhamentos anteriores, esta rodada refez
também o lado Clojure 1.12.5/JVM; portanto, variações contra a rodada `1ca1d79` não
isolam uma mudança específica do compilador.

| Métrica Cormen | Native | Clojure/JVM |
| --- | ---: | ---: |
| Tempo de parede acumulado | 26,08 s | 16,39 s |
| Tempo de CPU acumulado | 25,97 s | 31,35 s |
| RSS mediano | 13,2 MiB | 273,0 MiB |
| Vitórias por tempo de parede | 13 | 16 |
| Vitórias por CPU | 22 | 8 |
| Vitórias por RSS | 30 | 0 |

Os 30 checksums coincidiram. O nativo ainda perde 1,59× no tempo de parede acumulado,
mas consome 17,2% menos CPU no agregado e preserva vantagem de memória em todos os
casos. O CSV e a tabela por caso permanecem no
[relatório Cormen](../../benchmarks/cormen/results/README.md).
