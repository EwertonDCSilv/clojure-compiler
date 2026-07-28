# Cracking-style benchmark suite

[Catálogo central de benchmarks](../README.md) ·
[Suíte Cormen/CLRS](../cormen/README.md) ·
[README do projeto](../../README.pt-BR.md)

Suíte de benchmarks algorítmicos para o `clojure-compiler`, inspirada nas áreas de estudo
popularizadas por livros de entrevistas técnicas.

Os 60 programas são implementações originais. Nenhum enunciado, texto ou solução de
livro foi reproduzido. Os capítulos funcionam apenas como categorias temáticas.

## Organização

Cada arquivo `.clj` é um programa autônomo que:

1. implementa um algoritmo;
2. executa uma carga determinística várias vezes;
3. imprime um único checksum;
4. pode ser compilado diretamente pelo comando `build`.

| Capítulo | Casos | Foco |
| --- | ---: | --- |
| `01-arrays-and-strings` | 6 | soma, reversão, rotação, compactação, matriz e rolling hash |
| `02-linked-lists` | 6 | remoção, k-ésimo reverso, partição, dígitos, palíndromo e merge |
| `03-stacks-and-queues` | 6 | stack, mínimo, fila com duas pilhas, balanceamento, spans e round-robin |
| `04-trees-and-graphs` | 6 | altura, BST, níveis, balanceamento, alcance e componentes |
| `05-bit-manipulation` | 6 | popcount, paridade, Hamming, reversão, potência de dois e campo de bits |
| `06-math-and-logic` | 6 | MDC, MMC, primos, zeros fatoriais, raiz inteira e potência modular |
| `07-object-oriented-design` | 6 | records, protocolos, dispatch polimórfico e atualizações imutáveis |
| `08-recursion-and-dp` | 6 | Fibonacci, escadas, grid, moedas, LIS e subset sum |
| `09-sorting-and-searching` | 6 | busca binária/rotacionada, insertion, bubble, merge e frequências |
| `10-moderate-problems` | 6 | subarray, pares, score, população, swap e comprimentos |
| **Total** | **60** | |

Como o subconjunto compilado ainda não expõe primitivas bitwise nem indexação de
caracteres, o capítulo 5 usa operações aritméticas equivalentes e strings algorítmicas
são representadas por vetores de códigos inteiros. Isso mantém os casos executáveis hoje
e torna explícito o que deverá ser substituído quando essas primitivas entrarem.

## Uso rápido

Na raiz do repositório:

```bash
make benchmarks-cracking
```

O runner imprime CSV:

```text
benchmark,mode,scale,compile_wall_ms,wall_time_s,cpu_user_s,cpu_system_s,cpu_total_s,cpu_percent,max_rss_kb,checksum,expected,status
```

Exemplos:

```bash
# Executar apenas árvores e grafos
make benchmarks-cracking CRACKING_ARGS="--chapter 04"

# Exercitar o rooting com coleta em toda alocação
make benchmarks-cracking CRACKING_ARGS="--chapter 07 --gc-stress"

# Multiplicar a carga interna de todos os casos por 25 e gravar métricas nativas
make benchmarks-cracking \
  CRACKING_ARGS="--extreme --csv /tmp/native-extreme.csv"

# Comparar o nativo com Clojure/JVM na mesma carga extrema
make benchmarks-compare-cracking \
  CRACKING_COMPARISON_CSV=benchmarks/cracking/results/extreme.csv

# Atualizar só o nativo, preservando as medições JVM do CSV comparativo
benchmarks/refresh-native-comparison.sh --suite cracking

# Renderizar a tabela Markdown compacta por caso
benchmarks/render-benchmark-summary.sh \
  benchmarks/cracking/results/extreme.csv

# Regenerar os gráficos de tempo, CPU e memória das suítes
make benchmarks-charts

# Escolher outro multiplicador de carga
make benchmarks-cracking CRACKING_ARGS="--scale 10 --csv /tmp/scale-10.csv"

# Usar outro binário do compilador
make benchmarks-cracking \
  CRACKING_ARGS="--compiler /caminho/para/clojure-native"

# Apenas listar os casos selecionados
make benchmarks-list
```

Os scripts continuam disponíveis como interface de baixo nível. O
[`Makefile`](../../Makefile) é a entrada recomendada para build, testes, compatibilidade
e execução conjunta das suítes.

`expected.tsv` contém o checksum esperado de cada programa. Qualquer erro de build,
execução ou divergência faz o runner terminar com status diferente de zero.

O resultado extremo comparativo de referência desta árvore está em
[`results/extreme.csv`](results/extreme.csv), acompanhado pela descrição da máquina e da
metodologia em [`results/README.md`](results/README.md).

### Colunas de métricas

| Coluna | Unidade | Significado |
| --- | --- | --- |
| `compile_wall_ms` | ms | tempo de parede gasto pelo comando `build` |
| `wall_time_s` | s | tempo de parede do executável gerado |
| `cpu_user_s` | s | CPU executando código em user space |
| `cpu_system_s` | s | CPU gasta no kernel |
| `cpu_total_s` | s | soma de CPU user e system |
| `cpu_percent` | % | utilização de CPU reportada pelo GNU `time` |
| `max_rss_kb` | KiB | pico de resident set size do processo |

Esses nomes são usados pelo runner exclusivamente nativo. No CSV comparativo,
`compare-clojure.sh` usa prefixos explícitos:

| Coluna comparativa | Significado |
| --- | --- |
| `native_*` | medição do binário gerado pelo `clojure-compiler` |
| `clojure_*` | medição da referência Clojure/JVM AOT |
| `wall_speedup_vs_clojure` | `clojure_wall_time_s / native_wall_time_s` |
| `cpu_speedup_vs_clojure` | `clojure_cpu_total_s / native_cpu_total_s` |
| `rss_ratio_clojure_over_native` | `clojure_max_rss_kb / native_max_rss_kb` |
| `native_checksum`, `clojure_checksum` | resultados usados para validar equivalência |
| `status` | `OK` somente quando ambos executam e os checksums coincidem |

Nas três razões, um valor maior que `1` favorece o nativo: ele levou menos tempo ou
usou menos memória que Clojure/JVM. Um valor menor que `1` favorece Clojure/JVM.

O resumo Markdown por caso usa `N/J` para mostrar os valores absolutos
nativo/Clojure. Suas colunas de delta calculam `(nativo - Clojure) / Clojure`; portanto,
um delta negativo favorece o nativo e um delta positivo favorece Clojure/JVM.

`--extreme` não edita os programas versionados. O runner cria uma fonte temporária em
que apenas a quantidade de rodadas passada a `benchmark` é multiplicada por 25. Use
`--scale N` para controlar esse multiplicador. Em modo extremo, `expected` aparece como
`not-recorded`; o checksum medido continua no CSV e a execução ainda falha para saída
inválida ou status não-zero.

## Executar um caso manualmente

```bash
./target/release/clojure-native \
  build benchmarks/cracking/08-recursion-and-dp/04-coin-change.clj \
  -o /tmp/coin-change

/tmp/coin-change
```

## Metodologia

- Os tempos de compilação e execução são medidos separadamente.
- CPU e memória são coletadas pelo GNU `time`; `max_rss_kb` é o pico do processo, não
  uma medição de alocações individuais do GC.
- A comparação usa Clojure 1.12.5 em Java 21. Cada namespace Clojure é compilado AOT
  antes da medição; `clojure_compile_wall_ms` registra esse custo separadamente.
- `clojure_wall_time_s` ainda inclui a inicialização do processo JVM, assim como
  `native_wall_time_s` inclui a inicialização do processo nativo.
- Na primeira comparação, os JARs oficiais fixados de Clojure e `spec.alpha` são
  baixados do Maven Central para `target/benchmark-clojure`, que não é versionado.
- Cada programa repete internamente a operação para reduzir ruído de processos muito
  curtos.
- A saída é consumida como checksum, impedindo que o resultado seja ignorado.
- As entradas são fixas para tornar execuções comparáveis.
- O runner é um detector simples de regressão. Para medições estatísticas formais, rode
  o executável produzido várias vezes com uma ferramenta como `hyperfine`, mantendo
  máquina, perfil e toolchain constantes.
- Os valores absolutos não devem ser comparados entre máquinas diferentes.

## Adicionar um benchmark

1. Coloque um `.clj` autônomo no capítulo adequado.
2. Use apenas o subconjunto documentado em `specs/LANGUAGE_SCOPE.md`.
3. Faça o programa imprimir um único checksum determinístico.
4. Compile e execute o caso.
5. Registre o checksum em `expected.tsv`.
6. Atualize o [catálogo central](../README.md).
7. Rode `make benchmarks-cracking` e `make test`.
