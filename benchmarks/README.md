# Benchmarks

[README do projeto](../README.pt-BR.md) ·
[Documentação](../docs/README.md) ·
[Suíte Cracking](cracking/README.md) ·
[Suíte Cormen/CLRS](cormen/README.md) ·
[Corpus externo Exercism](exercism/README.md)

Catálogo central das cargas de desempenho do `clojure-compiler`.

São **98 programas Clojure autônomos**, distribuídos em três suítes. Cada caso executa
em Native e Clojure/JVM, aplica uma carga determinística e imprime o mesmo checksum.
O catálogo mede desempenho de execução; cobertura de sintaxe, semântica e biblioteca
padrão pertence à [suíte de conformidade](../specs/conformance/README.md).

> Os benchmarks são ferramentas de engenharia do compilador, não promessas universais
> de desempenho. Compare resultados somente no mesmo ambiente, com a mesma revisão,
> toolchain, escala e nível de otimização.

## Navegação

- [Escolha uma suíte](#escolha-uma-suíte)
- [Execute os benchmarks](#execute-os-benchmarks)
- [Entenda as métricas](#entenda-as-métricas)
- [Catálogo Cracking — 60 casos](#catálogo-cracking--60-casos)
- [Catálogo Cormen/CLRS — 30 casos](#catálogo-cormenclrs--30-casos)
- [Benchmark externo Exercism — 8 casos](#benchmark-externo-exercism--8-casos)
- [Consulte os resultados](#consulte-os-resultados)
- [Adicione um caso](#adicione-um-caso)

## Escolha uma suíte

| Suíte | Casos | Organização | Melhor ponto de partida para |
| --- | ---: | --- | --- |
| [Cracking](cracking/README.md) | 60 | 10 capítulos temáticos | estruturas da linguagem, coleções, records, protocols e algoritmos menores |
| [Cormen/CLRS](cormen/README.md) | 30 | 6 famílias de algoritmos | ordenação, estruturas de dados, programação dinâmica, grafos e string matching |
| [Exercism](exercism/README.md) | 8 | soluções públicas com adaptadores determinísticos | comparação independente Native × JVM |
| **Total** | **98** | **18 grupos** | correção, regressões e comparação Native × Clojure/JVM |

As implementações Cracking e Cormen são originais. A suíte Exercism preserva oito
soluções públicas MIT do snapshot documentado e adiciona cargas e checksums. A
auditoria das demais soluções não é benchmark: ela alimenta o catálogo de
conformidade. Origem, licença e mapeamento estão registrados em
[`exercism/UPSTREAM.md`](exercism/UPSTREAM.md).

## Execute os benchmarks

Os comandos abaixo partem da raiz do repositório.

### 1. Compile o compilador

```bash
make release
```

### 2. Valide uma suíte no caminho nativo

```bash
make benchmarks

# Ou execute apenas uma suíte
make benchmarks-cracking
make benchmarks-cormen
make benchmarks-exercism
```

Cada runner compila os casos selecionados, executa os binários e compara suas saídas
com os checksums versionados em
[`cracking/expected.tsv`](cracking/expected.tsv) e
[`cormen/expected.tsv`](cormen/expected.tsv), além de
[`exercism/expected.tsv`](exercism/expected.tsv). Build quebrado, processo com status
não-zero ou checksum divergente faz o comando falhar.

Para localizar um grupo antes de executá-lo:

```bash
make benchmarks-list
make benchmarks-cracking CRACKING_ARGS="--chapter 08"
make benchmarks-cormen CORMEN_ARGS="--chapter 05"
make exercism-compatibility
```

### 3. Compare com Clojure/JVM

```bash
make benchmarks-compare
```

A comparação compila o programa nos dois caminhos antes da medição:

1. binário nativo produzido pelo `clojure-compiler`;
2. namespace Clojure 1.12.5 compilado AOT sobre Java 21.

Os artefatos JVM fixados são baixados na primeira execução para
`target/benchmark-clojure/`, que não é versionado. Os CSVs são gravados por padrão em
`target/benchmarks/`; altere esse destino com `BENCHMARK_OUTPUT_DIR`.

The direct comparison target produces one diagnostic sample. To refresh the published
CSV files and Pages assets, use:

```bash
make benchmark-page-refresh
```

This workflow runs every Native × JVM suite ten times and publishes the per-case
median of each measurement. It accepts a sample set only when all ten rounds have the
same cases, scales, runtime version, checksums, and `OK` statuses. The raw CSV files
remain under the reported `target/benchmark-page-refresh.*` directory so that an
unexpected change can be audited before commit.

### 4. Aumente a carga ou investigue o runtime

```bash
# Multiplicador interno de 25×
make benchmarks-cracking CRACKING_ARGS="--extreme --csv /tmp/cracking-native.csv"

# Escala explícita
make benchmarks-cormen CORMEN_ARGS="--scale 10 --csv /tmp/cormen-native.csv"

# Coleta em toda alocação para exercitar o rooting
make benchmarks-cracking CRACKING_ARGS="--chapter 07 --gc-stress"

# Nível Cranelift explícito
make benchmarks-cormen CORMEN_ARGS="--opt-level speed"
```

`--extreme` e `--scale N` criam fontes temporárias; os arquivos `.clj` versionados não
são alterados.

### Execute um caso isolado

Todo arquivo do catálogo pode ser compilado diretamente. Por exemplo:

```bash
./target/release/clojure-native \
  build benchmarks/cracking/08-recursion-and-dp/04-coin-change.clj \
  -o /tmp/coin-change

/tmp/coin-change
```

## Entenda as métricas

O runner nativo separa o custo de compilação do custo de execução.

| Métrica | Unidade | O que representa |
| --- | --- | --- |
| `compile_wall_ms` | ms | tempo de parede do comando `build` |
| `wall_time_s` | s | tempo de parede do executável |
| `cpu_user_s` | s | CPU em espaço de usuário |
| `cpu_system_s` | s | CPU no kernel |
| `cpu_total_s` | s | soma de CPU user e system |
| `cpu_percent` | % | utilização de CPU reportada pelo GNU `time` |
| `max_rss_kb` | KiB | pico de memória residente do processo |
| `checksum` | inteiro | saída observada do caso |
| `status` | texto | `OK` somente após execução e validação corretas |

Nos CSVs comparativos, as métricas recebem os prefixos `native_*` e `clojure_*`. As
razões são calculadas como Clojure/JVM dividido pelo nativo:

| Razão | Leitura |
| --- | --- |
| `wall_speedup_vs_clojure` | vantagem em tempo de parede |
| `cpu_speedup_vs_clojure` | vantagem em tempo total de CPU |
| `rss_ratio_clojure_over_native` | vantagem em pico de memória |

Nas três razões, valores maiores que `1` favorecem o nativo e valores menores que `1`
favorecem Clojure/JVM. O checksum deve coincidir nos dois caminhos antes de o status ser
considerado `OK`.

## Catálogo Cracking — 60 casos

Guia completo: [`cracking/README.md`](cracking/README.md) · checksums:
[`cracking/expected.tsv`](cracking/expected.tsv) · resultados:
[`cracking/results/`](cracking/results/README.md)

### 01 · Arrays e strings

| Caso | O que exercita |
| --- | --- |
| [`01-vector-sum.clj`](cracking/01-arrays-and-strings/01-vector-sum.clj) | percurso indexado e soma de vetor |
| [`02-reverse-vector.clj`](cracking/01-arrays-and-strings/02-reverse-vector.clj) | construção de vetor em ordem reversa |
| [`03-rotate-left.clj`](cracking/01-arrays-and-strings/03-rotate-left.clj) | rotação circular de elementos |
| [`04-compact-adjacent.clj`](cracking/01-arrays-and-strings/04-compact-adjacent.clj) | remoção de duplicatas adjacentes |
| [`05-matrix-diagonals.clj`](cracking/01-arrays-and-strings/05-matrix-diagonals.clj) | acesso a matriz e soma de diagonais |
| [`06-rolling-hash.clj`](cracking/01-arrays-and-strings/06-rolling-hash.clj) | hash incremental sobre códigos inteiros |

### 02 · Listas ligadas

| Caso | O que exercita |
| --- | --- |
| [`01-remove-value.clj`](cracking/02-linked-lists/01-remove-value.clj) | remoção de ocorrências |
| [`02-kth-from-end.clj`](cracking/02-linked-lists/02-kth-from-end.clj) | busca do k-ésimo elemento a partir do fim |
| [`03-stable-partition.clj`](cracking/02-linked-lists/03-stable-partition.clj) | partição estável por pivô |
| [`04-reversed-digits.clj`](cracking/02-linked-lists/04-reversed-digits.clj) | soma de números representados por dígitos reversos |
| [`05-palindrome-list.clj`](cracking/02-linked-lists/05-palindrome-list.clj) | detecção de palíndromo |
| [`06-merge-sorted.clj`](cracking/02-linked-lists/06-merge-sorted.clj) | intercalação ordenada |

### 03 · Pilhas e filas

| Caso | O que exercita |
| --- | --- |
| [`01-stack-drain.clj`](cracking/03-stacks-and-queues/01-stack-drain.clj) | empilhamento e esvaziamento |
| [`02-min-stack.clj`](cracking/03-stacks-and-queues/02-min-stack.clj) | rastreamento do mínimo da pilha |
| [`03-queue-two-stacks.clj`](cracking/03-stacks-and-queues/03-queue-two-stacks.clj) | fila implementada com duas pilhas |
| [`04-balanced-tokens.clj`](cracking/03-stacks-and-queues/04-balanced-tokens.clj) | validação de delimitadores balanceados |
| [`05-monotonic-spans.clj`](cracking/03-stacks-and-queues/05-monotonic-spans.clj) | pilha monotônica e cálculo de spans |
| [`06-round-robin.clj`](cracking/03-stacks-and-queues/06-round-robin.clj) | escalonamento circular |

### 04 · Árvores e grafos

| Caso | O que exercita |
| --- | --- |
| [`01-tree-height.clj`](cracking/04-trees-and-graphs/01-tree-height.clj) | cálculo recursivo da altura |
| [`02-bst-search.clj`](cracking/04-trees-and-graphs/02-bst-search.clj) | busca em árvore binária |
| [`03-level-sums.clj`](cracking/04-trees-and-graphs/03-level-sums.clj) | agregação por nível |
| [`04-balanced-tree.clj`](cracking/04-trees-and-graphs/04-balanced-tree.clj) | verificação de balanceamento |
| [`05-graph-reachability.clj`](cracking/04-trees-and-graphs/05-graph-reachability.clj) | alcançabilidade em grafo |
| [`06-connected-components.clj`](cracking/04-trees-and-graphs/06-connected-components.clj) | contagem de componentes conexos |

### 05 · Manipulação de bits

O subconjunto compilado ainda não oferece primitivas bitwise. Estes casos usam
operações aritméticas equivalentes para manter as cargas executáveis.

| Caso | O que exercita |
| --- | --- |
| [`01-popcount.clj`](cracking/05-bit-manipulation/01-popcount.clj) | contagem de bits ativos |
| [`02-bit-parity.clj`](cracking/05-bit-manipulation/02-bit-parity.clj) | paridade binária |
| [`03-hamming-distance.clj`](cracking/05-bit-manipulation/03-hamming-distance.clj) | distância de Hamming |
| [`04-reverse-low-bits.clj`](cracking/05-bit-manipulation/04-reverse-low-bits.clj) | reversão dos bits menos significativos |
| [`05-power-of-two.clj`](cracking/05-bit-manipulation/05-power-of-two.clj) | identificação de potência de dois |
| [`06-insert-bit-field.clj`](cracking/05-bit-manipulation/06-insert-bit-field.clj) | inserção de campo binário |

### 06 · Matemática e lógica

| Caso | O que exercita |
| --- | --- |
| [`01-euclidean-gcd.clj`](cracking/06-math-and-logic/01-euclidean-gcd.clj) | máximo divisor comum |
| [`02-least-common-multiple.clj`](cracking/06-math-and-logic/02-least-common-multiple.clj) | mínimo múltiplo comum |
| [`03-prime-count.clj`](cracking/06-math-and-logic/03-prime-count.clj) | contagem de primos |
| [`04-factorial-trailing-zeros.clj`](cracking/06-math-and-logic/04-factorial-trailing-zeros.clj) | zeros finais de fatorial |
| [`05-integer-square-root.clj`](cracking/06-math-and-logic/05-integer-square-root.clj) | raiz quadrada inteira |
| [`06-modular-power.clj`](cracking/06-math-and-logic/06-modular-power.clj) | exponenciação modular |

### 07 · Records e protocols

| Caso | O que exercita |
| --- | --- |
| [`01-point-record.clj`](cracking/07-object-oriented-design/01-point-record.clj) | criação e leitura de record |
| [`02-shape-protocol.clj`](cracking/07-object-oriented-design/02-shape-protocol.clj) | dispatch de protocol entre formas |
| [`03-payroll-protocol.clj`](cracking/07-object-oriented-design/03-payroll-protocol.clj) | dispatch polimórfico em folha de pagamento |
| [`04-card-records.clj`](cracking/07-object-oriented-design/04-card-records.clj) | composição de records de cartas |
| [`05-file-tree-protocol.clj`](cracking/07-object-oriented-design/05-file-tree-protocol.clj) | protocol sobre árvore de arquivos |
| [`06-record-updates.clj`](cracking/07-object-oriented-design/06-record-updates.clj) | atualizações imutáveis de records |

### 08 · Recursão e programação dinâmica

| Caso | O que exercita |
| --- | --- |
| [`01-fibonacci.clj`](cracking/08-recursion-and-dp/01-fibonacci.clj) | Fibonacci iterativo |
| [`02-staircase-ways.clj`](cracking/08-recursion-and-dp/02-staircase-ways.clj) | contagem de maneiras de subir escadas |
| [`03-grid-paths.clj`](cracking/08-recursion-and-dp/03-grid-paths.clj) | caminhos em grade |
| [`04-coin-change.clj`](cracking/08-recursion-and-dp/04-coin-change.clj) | combinações para troca de moedas |
| [`05-longest-increasing-subsequence.clj`](cracking/08-recursion-and-dp/05-longest-increasing-subsequence.clj) | maior subsequência crescente |
| [`06-subset-sum.clj`](cracking/08-recursion-and-dp/06-subset-sum.clj) | soma de subconjunto |

### 09 · Ordenação e busca

| Caso | O que exercita |
| --- | --- |
| [`01-binary-search.clj`](cracking/09-sorting-and-searching/01-binary-search.clj) | busca binária |
| [`02-insertion-sort.clj`](cracking/09-sorting-and-searching/02-insertion-sort.clj) | insertion sort |
| [`03-bubble-sort-vector.clj`](cracking/09-sorting-and-searching/03-bubble-sort-vector.clj) | bubble sort sobre vetor |
| [`04-merge-sorted-vectors.clj`](cracking/09-sorting-and-searching/04-merge-sorted-vectors.clj) | intercalação de vetores ordenados |
| [`05-rotated-search.clj`](cracking/09-sorting-and-searching/05-rotated-search.clj) | busca em vetor rotacionado |
| [`06-frequency-table.clj`](cracking/09-sorting-and-searching/06-frequency-table.clj) | tabela de frequências |

### 10 · Problemas moderados

| Caso | O que exercita |
| --- | --- |
| [`01-maximum-subarray.clj`](cracking/10-moderate-problems/01-maximum-subarray.clj) | soma máxima de subarray |
| [`02-pair-sum-count.clj`](cracking/10-moderate-problems/02-pair-sum-count.clj) | contagem de pares com soma-alvo |
| [`03-mastermind-score.clj`](cracking/10-moderate-problems/03-mastermind-score.clj) | pontuação de acertos exatos e parciais |
| [`04-peak-population.clj`](cracking/10-moderate-problems/04-peak-population.clj) | ano de pico populacional |
| [`05-arithmetic-swap.clj`](cracking/10-moderate-problems/05-arithmetic-swap.clj) | troca aritmética sem variável auxiliar |
| [`06-board-lengths.clj`](cracking/10-moderate-problems/06-board-lengths.clj) | comprimentos possíveis com duas medidas |

## Catálogo Cormen/CLRS — 30 casos

Guia completo: [`cormen/README.md`](cormen/README.md) · checksums:
[`cormen/expected.tsv`](cormen/expected.tsv) · resultados:
[`cormen/results/`](cormen/results/README.md)

### 01 · Fundamentos e divisão e conquista

| Caso | O que exercita |
| --- | --- |
| [`01-binary-exponentiation.clj`](cormen/01-foundations-and-divide-conquer/01-binary-exponentiation.clj) | exponenciação binária |
| [`02-horner-polynomial.clj`](cormen/01-foundations-and-divide-conquer/02-horner-polynomial.clj) | avaliação polinomial pelo método de Horner |
| [`03-prefix-range-sums.clj`](cormen/01-foundations-and-divide-conquer/03-prefix-range-sums.clj) | prefixos e consultas de soma por intervalo |
| [`04-iterative-binary-search.clj`](cormen/01-foundations-and-divide-conquer/04-iterative-binary-search.clj) | busca binária iterativa |
| [`05-maximum-subarray-divide.clj`](cormen/01-foundations-and-divide-conquer/05-maximum-subarray-divide.clj) | subarray máximo por divisão e conquista |

### 02 · Ordenação e estatísticas de ordem

| Caso | O que exercita |
| --- | --- |
| [`01-insertion-sort.clj`](cormen/02-sorting-and-order-statistics/01-insertion-sort.clj) | insertion sort |
| [`02-selection-sort.clj`](cormen/02-sorting-and-order-statistics/02-selection-sort.clj) | selection sort |
| [`03-counting-sort.clj`](cormen/02-sorting-and-order-statistics/03-counting-sort.clj) | counting sort |
| [`04-merge-sort.clj`](cormen/02-sorting-and-order-statistics/04-merge-sort.clj) | merge sort |
| [`05-quickselect.clj`](cormen/02-sorting-and-order-statistics/05-quickselect.clj) | seleção do k-ésimo elemento |

### 03 · Estruturas de dados

| Caso | O que exercita |
| --- | --- |
| [`01-build-max-heap.clj`](cormen/03-data-structures/01-build-max-heap.clj) | construção de max-heap |
| [`02-disjoint-set-union.clj`](cormen/03-data-structures/02-disjoint-set-union.clj) | união e busca em conjuntos disjuntos |
| [`03-chained-hash-table.clj`](cormen/03-data-structures/03-chained-hash-table.clj) | tabela hash com encadeamento |
| [`04-circular-queue.clj`](cormen/03-data-structures/04-circular-queue.clj) | fila circular |
| [`05-binary-search-tree.clj`](cormen/03-data-structures/05-binary-search-tree.clj) | árvore binária de busca |

### 04 · Programação dinâmica e algoritmos gulosos

| Caso | O que exercita |
| --- | --- |
| [`01-rod-cutting.clj`](cormen/04-dynamic-programming-and-greedy/01-rod-cutting.clj) | corte de hastes |
| [`02-matrix-chain-order.clj`](cormen/04-dynamic-programming-and-greedy/02-matrix-chain-order.clj) | ordem ótima de multiplicação de matrizes |
| [`03-longest-common-subsequence.clj`](cormen/04-dynamic-programming-and-greedy/03-longest-common-subsequence.clj) | maior subsequência comum |
| [`04-zero-one-knapsack.clj`](cormen/04-dynamic-programming-and-greedy/04-zero-one-knapsack.clj) | mochila 0/1 |
| [`05-activity-selection.clj`](cormen/04-dynamic-programming-and-greedy/05-activity-selection.clj) | seleção gulosa de atividades |

### 05 · Algoritmos de grafos

| Caso | O que exercita |
| --- | --- |
| [`01-breadth-first-search.clj`](cormen/05-graph-algorithms/01-breadth-first-search.clj) | busca em largura |
| [`02-depth-first-search.clj`](cormen/05-graph-algorithms/02-depth-first-search.clj) | busca em profundidade |
| [`03-topological-sort.clj`](cormen/05-graph-algorithms/03-topological-sort.clj) | ordenação topológica |
| [`04-bellman-ford.clj`](cormen/05-graph-algorithms/04-bellman-ford.clj) | caminhos mínimos com Bellman–Ford |
| [`05-floyd-warshall.clj`](cormen/05-graph-algorithms/05-floyd-warshall.clj) | caminhos mínimos entre todos os pares |

### 06 · Teoria dos números e casamento de strings

| Caso | O que exercita |
| --- | --- |
| [`01-extended-euclid.clj`](cormen/06-number-theory-and-string-matching/01-extended-euclid.clj) | algoritmo de Euclides estendido |
| [`02-sieve-of-eratosthenes.clj`](cormen/06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj) | crivo de Eratóstenes |
| [`03-naive-string-matching.clj`](cormen/06-number-theory-and-string-matching/03-naive-string-matching.clj) | casamento ingênuo de sequências |
| [`04-rabin-karp.clj`](cormen/06-number-theory-and-string-matching/04-rabin-karp.clj) | busca Rabin–Karp |
| [`05-knuth-morris-pratt.clj`](cormen/06-number-theory-and-string-matching/05-knuth-morris-pratt.clj) | busca Knuth–Morris–Pratt |

## Benchmark externo Exercism — 8 casos

Guia completo: [`exercism/README.md`](exercism/README.md) · checksums:
[`exercism/expected.tsv`](exercism/expected.tsv) · resultados:
[`exercism/results/`](exercism/results/README.md).

Os oito casos compilam e executam nos dois caminhos. A auditoria das 114 soluções
oficiais e os 13 casos conceituais A–E ficam no
[catálogo de conformidade Exercism](../tests/conformance/level-d-pure-libraries/external/exercism/README.md).

```bash
make exercism-compatibility
make benchmarks-exercism
make benchmarks-compare-exercism EXERCISM_COMPARE_ARGS="--scale 5"
```

## Consulte os resultados

Snapshot medido em 2026-07-29 no compilador `a1ecebd`, com medianas por caso de dez
rodadas completas:

| Suíte | Parede Native/JVM | CPU Native/JVM | RSS mediano Native/JVM |
| --- | ---: | ---: | ---: |
| Cracking | 8,23 / 23,18 s | 8,06 / 49,68 s | 4,6 / 117,2 MiB |
| Cormen/CLRS | 30,60 / 16,74 s | 30,38 / 31,82 s | 13,2 / 270,2 MiB |
| Exercism | 35,05 / 8,48 s | 35,00 / 12,47 s | 7,7 / 430,9 MiB |

Os 98 casos terminaram com status `OK` e checksums equivalentes.
No Cormen, a mediana reduziu em 11,7% o pico do artefato anterior, mas ficou 12,4%
acima do snapshot de uma rodada em `424ba20`. Essa diferença continua subordinada ao
gate pareado da [ADR-0014](../specs/adr/0014-optional-optimization-ir.md), não como
causalidade atribuída.

| Suíte | Relatório comentado | CSV comparativo de referência |
| --- | --- | --- |
| Cracking | [`cracking/results/README.md`](cracking/results/README.md) | [`cracking/results/extreme.csv`](cracking/results/extreme.csv) |
| Cormen/CLRS | [`cormen/results/README.md`](cormen/results/README.md) | [`cormen/results/extreme.csv`](cormen/results/extreme.csv) |
| Exercism | [`exercism/results/README.md`](exercism/results/README.md) | [`exercism/results/extreme.csv`](exercism/results/extreme.csv) |

Para transformar qualquer CSV comparativo no mesmo formato de tabela Markdown usado
nos relatórios:

```bash
benchmarks/render-benchmark-summary.sh \
  benchmarks/cracking/results/extreme.csv
```

Os relatórios e a página do projeto também incorporam gráficos SVG de tempo, CPU e
memória. Para regenerar os três gráficos de cada destino a partir dos CSVs de referência:

```bash
make benchmarks-charts
```

O alvo atualiza `benchmarks/*/results/charts/`; as suítes exibidas na página também
atualizam `docs/assets/benchmarks/`. O programa Rust
[`render-benchmark-charts.rs`](render-benchmark-charts.rs) usa somente a biblioteca
padrão e aceita um CSV, um diretório de saída e um título, permitindo visualizar
resultados experimentais sem substituir os gráficos oficiais. O `Makefile` recompila o
binário apenas quando o fonte muda. Os SVGs usam somente identificadores neutros
(`CLJN` para o executável nativo, `JVM`, `WALL`, `CPU`, `RSS`, unidades e símbolos);
na página do projeto, títulos, legendas e explicações ficam no HTML e acompanham o
idioma selecionado.

Quando já existe uma referência JVM válida e apenas o compilador nativo mudou, é
possível atualizar somente as colunas nativas:

```bash
benchmarks/refresh-native-comparison.sh --suite cracking
benchmarks/refresh-native-comparison.sh --suite cormen
```

Esse fluxo preserva as medições `clojure_*`, exige a mesma escala e recalcula as razões
comparativas. A metodologia e o ambiente da rodada precisam continuar documentados no
README da respectiva pasta `results/`.

## Metodologia e limites

- Compilação e execução são medidas separadamente.
- CPU e pico de RSS são coletados pelo GNU `time`.
- Cada programa repete internamente sua operação para reduzir o ruído de processos
  muito curtos.
- A saída é consumida como checksum para impedir que o resultado seja descartado.
- As entradas são fixas e reprodutíveis.
- A execução JVM inclui a inicialização da JVM; a nativa inclui a inicialização do
  processo nativo.
- `make benchmarks-compare` remains a single-round diagnostic. The published
  `make benchmark-page-refresh` workflow requires ten complete rounds and records the
  per-case median while retaining every raw sample.
- Ten unpaired rounds reduce outlier sensitivity but do not prove causality. Compiler
  changes still require a same-environment paired A/B experiment such as the Cormen IR
  gate before attributing a performance difference to the implementation.
- O subconjunto nativo ainda possui limites documentados em
  [`specs/LANGUAGE_SCOPE.md`](../specs/LANGUAGE_SCOPE.md). Esses limites influenciam a
  forma de alguns casos.

## Adicione um caso

1. Escolha a suíte e o capítulo que melhor representam a carga.
2. Crie um programa `.clj` autônomo usando apenas o subconjunto documentado.
3. Mantenha entradas fixas e concentre o trabalho em uma função `benchmark`.
4. Imprima exatamente um checksum determinístico.
5. Compile e execute o arquivo manualmente.
6. Registre o checksum em `expected.tsv`.
7. Adicione o caso a este catálogo e ao README da suíte.
8. Execute `make benchmarks` e `make test`.

Use casos pequenos e intencionais. Um bom benchmark torna claro qual parte do
compilador ou runtime está sendo exercitada e permanece útil quando uma otimização
muda.
