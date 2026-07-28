# CLRS-style algorithm benchmark suite

[Catálogo central de benchmarks](../README.md) ·
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

# Regenerar os gráficos de tempo, CPU e memória das suítes
make benchmarks-charts

# Repetir a comparação com um nível Cranelift explícito
make benchmarks-compare-cormen \
  CORMEN_COMPARE_ARGS="--opt-level speed" \
  CORMEN_COMPARISON_CSV=benchmarks/cormen/results/cranelift-speed.csv

# Comparar IR direta e experimental em 7 pares alternados, sem alterar a JVM
make benchmarks-cormen-ir \
  CORMEN_IR_ARGS="--repetitions 7 --scale 25"
```

O gate da IR grava amostras brutas em `results/ir-ab-raw.csv`, metadados do ambiente em
`results/ir-ab-raw.metadata.txt` e o resumo estatístico em `results/ir-ab-report.md`.
Uma execução por capítulo ou com menos de sete pares é apenas diagnóstica e nunca
promove o perfil.

O resultado versionado atual passou o gate completo: `safe/none` foi 0,9568 em wall e
0,9565 em CPU, ou melhorias medianas de 4,32% e 4,35%, respectivamente. Os 30 checksums
foram preservados e não houve regressão confirmada por caso ou capítulo. Isso promove a
evidência do perfil opt-in; não altera o padrão `--ir-opt none`. Consulte o
[relatório A/B](results/ir-ab-report.md) para as amostras e os intervalos.

Para comparar um bundle ainda não admitido com o `safe` atual sem reescrever as
medições JVM:

```bash
benchmarks/cormen/compare-ir.sh \
  --control-ir-opt safe \
  --candidate-ir-opt safe \
  --candidate-experiment adr15 \
  --repetitions 7 --scale 25 \
  --raw target/adr15-ir-ab-raw.csv \
  --report target/adr15-ir-ab-report.md
```

O primeiro resultado completo da ADR-0015 foi rejeitado: 1,0066 wall e 1,0054 CPU no
agregado, em vez do ganho mínimo de 3%, e estimativas pontuais medianas acima do teto
por caso. O experimento não foi incorporado a `safe`.

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
