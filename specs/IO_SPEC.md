# Gate nativo de I/O

Status: **parcialmente implementado**. Este documento define o contrato necessário
para declarar o compilador `I/O-ready`; ele não afirma que toda a superfície descrita
já está implementada. A classificação executável continua sendo a matriz em
[`tests/conformance/`](../tests/conformance): output, flush, redirecionamento,
`slurp`/`spit`, `read-string` e streams de string possuem casos `active`, enquanto as
lacunas com caminho de teste permanecem `xfail`.

## Escopo do primeiro gate

O primeiro gate é Linux x86_64, síncrono, bloqueante e com texto UTF-8 estrito. Ele
inclui streams padrão, contexto do processo, arquivos textuais e binários, paths,
filesystem, lifecycle explícito e reader em runtime.

Rede, sockets, URLs, subprocessos, terminal interativo, file watchers, `load-file`,
`eval` e carregamento dinâmico ficam fora. Programas compilados recebem as permissões
normais do processo. Macros continuam sem I/O por padrão, conforme a
[ADR-0004](adr/0004-macro-execution.md).

As referências semânticas são as APIs oficiais de
[`clojure.core`](https://clojure.github.io/clojure/branch-master/clojure.core-api.html),
[`clojure.edn`](https://clojure.github.io/clojure/clojure.edn-api.html),
[`clojure.java.io`](https://clojure.github.io/clojure/branch-clojure-1.8.0/clojure.java.io-api.html)
e o contrato de
[`clojure.main`](https://clojure.org/reference/repl_and_main). Onde objetos Java
seriam necessários, esta spec define uma API nativa deliberadamente diferente.

## Dependências bloqueantes

O gate é dividido em seis marcos ordenados. Um marco não pode ser declarado completo
sem os anteriores.

| Marco | Dependência | Critério mínimo |
| --- | --- | --- |
| IO-0 | Exceções capturáveis, `try/catch/finally`, Vars dinâmicas e `binding` | exceção atravessa frames, `finally` sempre executa e bindings são restaurados |
| IO-1 | `Char`, `Float`, `Symbol`, metadata, `Bytes`, `Path` e handles | valores têm igualdade, impressão, rooting e erros de tipo definidos |
| IO-2 | stdin/stdout/stderr, argv, ambiente e redirecionamento | streams dinâmicos funcionam com arquivo, memória e pipe |
| IO-3 | arquivos textuais/binários, buffering, seek e lifecycle | leitura/escrita parcial é repetida; nenhum handle vaza |
| IO-4 | filesystem, metadata, symlinks e operações recursivas | sandbox de teste e regras de segurança são observáveis |
| IO-5 | reader EDN e reader de código em runtime | corpus diferencial é igual ao reader do compilador no subconjunto ativo |

IO-0 é uma dependência arquitetural, não apenas uma conveniência de API. Sem unwind
capturável não é possível garantir fechamento em `with-open`, restauração de bindings
ou erros de I/O categorizados.

O snapshot atual satisfaz apenas a parte genérica de unwind de IO-0: `throw` e
`try`/`catch`/`finally` atravessam frames e executam `finally`. O marco continua aberto
porque Vars dinâmicas, `binding`, tipos de exceção e `ex-data` ainda não existem no
caminho compilado.

## Modelo de valores e ABI

`Bytes` é um valor imutável, compacto e indexado por bytes sem sinal de 0 a 255.
Conversões de/para string validam UTF-8. Uma fatia lida pode compartilhar armazenamento
internamente, mas mutação nunca é observável pela linguagem.

`Path` guarda a representação nativa do SO. Strings fornecidas a construtores de I/O
sempre representam paths; texto em memória exige `string-reader`. Um `Path` pode
continuar válido mesmo se seus bytes não forem representáveis em UTF-8. Nesse caso,
qualquer conversão textual falha com `:invalid-encoding`, sem alterar o path.

Handles de reader, writer, input stream e output stream são objetos GC que apontam para
recursos externos mantidos atrás da ABI C. Enquanto abertos, ficam registrados como
recursos externos vivos. `close!` remove o registro e é idempotente; operações
posteriores falham com `:closed`. Streams padrão não podem ser fechados.

Toda leitura e escrita usa buffers internos. Uma chamada de ABI por byte ou caractere
é uma falha do gate. Escritas parciais e `EINTR` são tratadas em loop; arquivos grandes
são transmitidos em blocos, sem `slurp` implícito nas APIs de streaming.

## Standard streams and process context

### `clojure.core`

O runtime fornece Vars dinâmicas:

- `*in*`, `*out*` e `*err*`;
- `*command-line-args*`;
- `*flush-on-newline*`, inicialmente `true`.

As funções `print`, `println`, `pr`, `prn`, `newline`, `flush` e `read-line` usam os
bindings correntes. `print` produz representação legível por humanos; `pr` produz
representação de dados quando houver uma. As variantes com `n` acrescentam newline.
`println`/`newline` fazem flush somente quando `*flush-on-newline*` for verdadeiro.

`with-in-str` vincula `*in*` a um reader em memória. `with-out-str` vincula `*out*` a
um writer em memória, fecha o recurso e retorna seu conteúdo mesmo com zero writes.
`with-open` avalia os inicializadores da esquerda para a direita e fecha os handles na
ordem inversa, inclusive durante unwind. Se corpo e fechamento falharem, a exceção do
corpo é primária e as falhas de fechamento são anexadas aos dados da exceção.

### Launcher e processo

`cljn.process/getenv` retorna string ou `nil`; `environment` retorna um mapa imutável
do snapshot corrente; `cwd` retorna `Path`. Nomes/valores de ambiente inválidos para o
SO geram exceção categorizada.

O launcher preenche `*command-line-args*` sem incluir o nome do executável. Um
namespace principal só é invocado automaticamente quando escolhido por
`build --main ns` ou pelo manifesto. Nesse modo, `ns/-main` recebe os mesmos argumentos,
como em `clojure -M -m`. No modo script, os argumentos são vinculados, mas nenhuma
função `-main` é descoberta implicitamente.

## Encoding and line rules

Streams textuais, `slurp` e `spit` aceitam somente UTF-8 no primeiro gate. Decoding é
estrito e incremental: uma sequência inválida ou incompleta gera
`:invalid-encoding`, inclusive quando cruza a fronteira entre buffers.

`read-line`:

1. remove um terminador `LF` ou `CRLF`;
2. preserva outros caracteres, inclusive `CR` isolado;
3. retorna a última linha quando EOF ocorre sem terminador;
4. retorna `nil` quando EOF ocorre antes de qualquer caractere.

`read-char` retorna `Char` ou `nil` no EOF. `unread-char` mantém no mínimo um caractere
de pushback e rejeita overflow do buffer. As APIs binárias nunca fazem normalização de
newline ou transcodificação.

## API de `clojure.core`

Além do baseline de saída, o gate ativa:

- `slurp`, `spit`, `read` e `read-string`;
- `with-open`, `with-in-str` e `with-out-str`;
- as Vars e funções de stream definidas acima.

`slurp` e `spit` aceitam string ou `Path`. `spit` trunca por padrão; append e overwrite
são opções explícitas. Nenhuma dessas funções substitui as APIs de streaming para
arquivos grandes.

## API de `clojure.edn`

`clojure.edn/read` e `read-string` aceitam:

- `:eof`, valor retornado quando nenhum formulário existe;
- `:readers`, mapa de símbolos de tag para funções;
- `:default`, função chamada para tags não registradas.

O reader EDN rejeita forms exclusivas de código, símbolos inválidos e `#=`. Ele lê um
form por chamada, preservando o restante do stream. Erros incluem localização em
linha, coluna e offset de byte nos dados da exceção.

## API nativa `cljn.io`

`cljn.io` substitui as partes portáveis de `clojure.java.io` sem expor classes Java.

### Paths e bytes

- Paths: `path`, `join`, `parent`, `file-name`, `normalize`, `real-path`, `absolute?`.
- Bytes: `bytes`, `bytes?`, `byte-count`, `bytes->vector`, `bytes->string`,
  `string->bytes`.

`normalize` é lexical; `real-path` consulta o filesystem e resolve symlinks.
`bytes` valida cada elemento como inteiro de 0 a 255.

### Recursos

- Arquivo: `reader`, `writer`, `input-stream`, `output-stream`.
- Memória: `string-reader`, `string-writer`, `byte-input-stream`,
  `byte-output-stream`.
- Resultado em memória: `writer-string`, `output-bytes`.
- Lifecycle: `close!`, `closed?`, `flush!`.

Opções de criação, append e overwrite são keywords nomeadas. Construtores que recebem
string interpretam-na como path; nunca como conteúdo.
`writer-string` e `output-bytes` devolvem snapshots imutáveis do conteúdo acumulado;
não fecham nem resetam o handle.

### Texto, binário e posição

- Texto: `read-char`, `unread-char`, `read-line`, `read-block!`, `write!`.
- Binário: `read-bytes`, `write-bytes!`.
- Arquivo: `seek!`, `position`, `truncate!`, `copy!`.

`read-block!` e `read-bytes` podem retornar menos que o tamanho solicitado e retornam
um valor vazio somente para pedido de tamanho zero; no EOF retornam `nil`.
`seek!` usa offsets não negativos no primeiro gate.

### Filesystem and recursive safety

- Inspeção: `exists?`, `file?`, `directory?`, `symlink?`, `attributes`, `list`.
- Criação: `create-directory!`, `create-directories!`, `create-symlink!`.
- Mudança: `copy!`, `move!`, `delete!`, `read-link`.
- Árvores: `copy-tree!`, `delete-tree!`.

`attributes` retorna pelo menos tipo, tamanho, permissões e timestamps disponíveis.
Listagens têm ordem determinística por `Path` para tornar builds e testes reproduzíveis.

Operações recursivas nunca seguem symlinks. Overwrite requer opção explícita. A
remoção da raiz do filesystem é sempre recusada, mesmo se o processo tiver permissão.
O implementador deve usar operações relativas a descritores quando disponíveis para
reduzir corridas entre validação e uso; a API não promete atomicidade para árvores.

## Erros

Falhas de I/O lançam `cljn.io/IOException`. `ex-data` contém:

```clojure
{:kind      :not-found
 :operation :open-reader
 :path      #cljn/path "missing.txt"
 :os-code   2
 :message   "No such file or directory"}
```

Campos não aplicáveis podem ser `nil`, mas as chaves permanecem. As categorias mínimas
são `:not-found`, `:permission-denied`, `:already-exists`, `:invalid-encoding`,
`:eof`, `:closed`, `:incompatible-operation`, `:invalid-input`, `:would-block` e
`:other`. A mensagem não é oracle estável; `:kind` e `:operation` são.

Erros de tipo puros continuam usando a hierarquia de exceções da linguagem. Códigos do
SO não escapam como números de retorno.

## Reader de runtime

O parser de runtime é escrito em Clojure sobre primitivas bufferizadas de caracteres.
Há dois modos:

- `:edn`, conforme `clojure.edn`;
- `:clojure`, exatamente o subconjunto `active` do nível A da conformidade.

O parser Rust do compilador continua sendo a fonte do reader de build. As duas
implementações compartilham o mesmo corpus diferencial, diagnósticos equivalentes e
casos de leitura sequencial, mas não compartilham código. `#=` é rejeitado em ambos os
modos. Esta fase não introduz `load-file`, `eval` nem `require` dinâmico.

## Contrato da conformidade

Casos `build-run` podem incluir:

```toml
[run]
args = ["first", "ação"]
env = { APP_MODE = "test" }
stdin = "stdin.bin"
expected_exit = 0
platforms = ["linux"]
setup_symlinks = [{ path = "link", target = "target.txt" }]
expected_symlinks = [{ path = "copy/link", target = "target.txt" }]
```

Cada processo roda em diretório temporário isolado. `work.before/` é copiado antes da
execução; `work.after/` é o snapshot exato esperado. Paths de fixtures não podem ser
absolutos, conter `..` nem escapar do temporário. `expected.stdout.bin`,
`expected.stderr.bin` e fixtures `.bin` são comparados byte a byte; expectativas
textuais mantêm normalização de newline.

O oracle JVM é manual e só se aplica a APIs compatíveis de core/EDN. `cljn.*` usa
`oracle = "not-applicable"` ou uma divergência documentada.

## Gate de aceite

O gate completo exige simultaneamente:

1. todos os casos previstos promovidos a `active`, sem `xfail` de I/O;
2. `make compatibility` verde sem rede e sem JVM;
3. zero handles abertos no fim de cada caso, inclusive sob GC stress e exceção;
4. arquivos grandes processados em blocos, com testes que detectem buffering ausente;
5. sanitizers e gates de cobertura existentes preservados;
6. stdin/stdout/stderr comparados como bytes e filesystem comparado exatamente;
7. corpus EDN/código diferencial verde contra o parser do compilador;
8. riscos de leak, symlink race, remoção recursiva, encoding, bloqueio e escrita parcial
   cobertos por testes explícitos.
