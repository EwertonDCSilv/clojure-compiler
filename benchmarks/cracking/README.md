# Cracking-style benchmark suite

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
cargo build --release -p clojure-native-cli
benchmarks/cracking/run.sh
```

O runner imprime CSV:

```text
benchmark,compile_ms,run_ms,checksum,expected,status
```

Exemplos:

```bash
# Executar apenas árvores e grafos
benchmarks/cracking/run.sh --chapter 04

# Exercitar o rooting com coleta em toda alocação
benchmarks/cracking/run.sh --chapter 07 --gc-stress

# Usar outro binário do compilador
benchmarks/cracking/run.sh --compiler /caminho/para/clojure-native

# Apenas listar os casos selecionados
benchmarks/cracking/run.sh --list
```

`expected.tsv` contém o checksum esperado de cada programa. Qualquer erro de build,
execução ou divergência faz o runner terminar com status diferente de zero.

## Executar um caso manualmente

```bash
./target/release/clojure-native \
  build benchmarks/cracking/08-recursion-and-dp/04-coin-change.clj \
  -o /tmp/coin-change

/tmp/coin-change
```

## Metodologia

- Os tempos de compilação e execução são medidos separadamente.
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
6. Rode `benchmarks/cracking/run.sh` e `cargo test --workspace`.
