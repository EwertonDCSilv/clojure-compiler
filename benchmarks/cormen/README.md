# CLRS-style algorithm benchmark suite

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
benchmarks/cormen/run.sh

# Executar apenas algoritmos de grafos
benchmarks/cormen/run.sh --chapter 05

# Forçar uma carga 25 vezes maior e coletar CPU, memória e tempo
benchmarks/cormen/run.sh --extreme \
  --csv benchmarks/cormen/results/native-extreme.csv

# Comparar nativo com Clojure/JVM AOT
benchmarks/cormen/compare-clojure.sh \
  --csv benchmarks/cormen/results/extreme.csv

# Repetir a comparação com um nível Cranelift explícito
benchmarks/cormen/compare-clojure.sh --opt-level speed \
  --csv benchmarks/cormen/results/cranelift-speed.csv
```

As opções, colunas CSV e razões de comparação são as mesmas documentadas na
[suíte cracking](../cracking/README.md).

## Resultados de referência

A execução extrema versionada (`--scale 25`) está em
[`results/extreme.csv`](results/extreme.csv), acompanhada do
[ambiente, metodologia e resumo](results/README.md). Os 30 casos foram comparados
com Clojure/JVM AOT e tiveram checksums idênticos.
