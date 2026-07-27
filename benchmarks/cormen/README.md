# CLRS-style algorithm benchmark suite

[Catálogo dos 90 benchmarks](../README.md) ·
[Suíte Cracking](../cracking/README.md) ·
[README do projeto](../../README.pt-BR.md)

Suíte de cargas determinísticas para o `clojure-compiler`, organizada pelas grandes
famílias de algoritmos ensinadas em *Introduction to Algorithms* (CLRS/Cormen).

Os programas são implementações originais para benchmarking. Nenhum enunciado, trecho
ou solução do livro foi reproduzido. Os nomes dos capítulos são apenas categorias
temáticas amplamente conhecidas.

## Organização

| Capítulo | Casos | Foco |
| --- | ---: | --- |
| `01-foundations-and-divide-conquer` | 5 | potência, Horner, prefixos, busca e subarray |
| `02-sorting-and-order-statistics` | 5 | insertion, selection, counting, merge e seleção |
| `03-data-structures` | 5 | heap, conjuntos disjuntos, hash, fila e árvore |
| `04-dynamic-programming-and-greedy` | 5 | rod cutting, matrix chain, LCS, knapsack e atividades |
| `05-graph-algorithms` | 5 | BFS, DFS, topological sort, Bellman-Ford e Floyd-Warshall |
| `06-number-theory-and-string-matching` | 5 | Euclides estendido, sieve, matching, Rabin-Karp e KMP |
| **Total** | **30** | |

Cada arquivo imprime um único checksum determinístico. `expected.tsv` fixa o resultado
correto e o runner falha em qualquer erro de compilação, execução ou divergência.

## Uso rápido

Na raiz do repositório:

```bash
# Compilar e validar os 30 casos
make benchmarks-cormen

# Executar apenas algoritmos de grafos
make benchmarks-cormen CORMEN_ARGS="--chapter 05"

# Forçar uma carga 25 vezes maior e coletar CPU, memória e tempo
make benchmarks-cormen \
  CORMEN_ARGS="--extreme --csv benchmarks/cormen/results/native-extreme.csv"

# Comparar nativo com Clojure/JVM AOT
make benchmarks-compare-cormen \
  CORMEN_COMPARISON_CSV=benchmarks/cormen/results/extreme.csv

# Atualizar só o nativo, preservando as medições JVM do CSV comparativo
benchmarks/refresh-native-comparison.sh --suite cormen

# Renderizar a tabela Markdown compacta por caso
benchmarks/render-benchmark-summary.sh \
  benchmarks/cormen/results/extreme.csv

# Regenerar os gráficos de tempo, CPU e memória das duas suítes
make benchmarks-charts

# Repetir a comparação com um nível Cranelift explícito
make benchmarks-compare-cormen \
  CORMEN_COMPARE_ARGS="--opt-level speed" \
  CORMEN_COMPARISON_CSV=benchmarks/cormen/results/cranelift-speed.csv
```

Os scripts continuam disponíveis como interface de baixo nível. O
[`Makefile`](../../Makefile) é a entrada recomendada para build, testes, compatibilidade
e execução conjunta das suítes.

As opções, colunas CSV e razões de comparação são as mesmas documentadas na
[suíte cracking](../cracking/README.md).

## Resultados de referência

A execução extrema versionada (`--scale 25`) está em
[`results/extreme.csv`](results/extreme.csv), acompanhada do
[ambiente, metodologia e resumo](results/README.md). Os 30 casos foram comparados
com Clojure/JVM AOT e tiveram checksums idênticos.

## Adicionar um benchmark

1. Coloque um programa `.clj` autônomo na família adequada.
2. Use apenas o subconjunto de [`LANGUAGE_SCOPE`](../../specs/LANGUAGE_SCOPE.md).
3. Faça o programa imprimir exatamente um checksum determinístico.
4. Compile e execute o caso, depois registre o resultado em `expected.tsv`.
5. Atualize este README e o [catálogo central](../README.md).
6. Rode `make benchmarks-cormen` e `make test`.
