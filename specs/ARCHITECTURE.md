# Arquitetura do compilador

O `clojure-compiler` é um workspace Cargo modular. Seu binário é `clojure-native`.
Esta página descreve a árvore executável atual; componentes planejados aparecem em uma
seção separada.

## Fluxo atual

```text
Source (.clj, UTF-8)
  │
  ▼
clojure-reader ──► forms + spans
  │
  ▼
expansão de macros conhecidas + clojure-analyzer
  │                  └──► scopes, closures, aridades, recur, records, protocols
  ▼
Program / Expr analisado
  │
  ▼
clojure-codegen ──► Cranelift IR ──► objeto do host
  │
  ▼
cc + runtime C embutido
  │
  ▼
executável autônomo, sem JVM e sem .class
```

O interpretador de bootstrap é um caminho paralelo usado pelos comandos `eval` e `run`.
O core compilável em `crates/clojure-native-cli/src/core_compiled.clj` é analisado junto
com o programa do usuário em todo `build`.

## Crates atuais

| Crate | Responsabilidade |
| --- | --- |
| `clojure-span` | `SourceId`, posições e spans |
| `clojure-diagnostics` | diagnósticos estruturados e renderização |
| `clojure-syntax` | forms, símbolos, keywords e metadata de leitura |
| `clojure-reader` | tokenizer, reader macros e parser |
| `clojure-value` | representação de valores do interpretador |
| `clojure-interp` | interpretador tree-walking de bootstrap |
| `clojure-analyzer` | resolução léxica, AST, macros conhecidas e validações |
| `clojure-codegen` | codegen Cranelift e runtime C embutido |
| `clojure-native-cli` | comandos `read`, `eval`, `run` e `build` |
| `clojure-test-support` | conformidade, schema, checksums, oracle e relatórios |

## Dependências e fronteiras

- `span`, `diagnostics` e `syntax` são crates de dados.
- O reader não depende do analyzer nem do codegen.
- O interpretador usa `clojure-value`; ele não compartilha a ABI de `Value` do runtime C.
- O analyzer recebe forms e produz a representação consumida pelo codegen.
- O codegen incorpora os fragmentos de `clojure-codegen/runtime/` na ordem declarada,
  formando uma única unidade C, compila-a com o compilador do host e liga esse objeto ao
  objeto Cranelift. `clojure-codegen/runtime.c` permanece como entrada amalgamada
  compatível para compilação e ferramentas C diretas.
- A CLI fica no topo e orquestra os caminhos de leitura, interpretação e build.
- `clojure-test-support` executa o produto pela interface pública da CLI.

## Runtime ABI

O código gerado chama o runtime por ABI C. `Value` nativo é uma palavra do tamanho de
ponteiro, com fixnums tagueados e ponteiros para objetos GC. Funções Clojure usam a
convenção uniforme `(self, argc, argv)`.

O runtime fornece:

- alocação e GC mark-sweep;
- strings, cons, vetores, maps, sets e records;
- closures, aridade, `apply` e dispatch de protocolos;
- operações de coleção e o baseline de stdout por `print`/`println`;
- slow paths de tipo, overflow e divisão.

Fast paths numéricos e operações da shadow stack são gerados diretamente sempre que
possível.

O gate proposto em [IO_SPEC](IO_SPEC.md) preserva essa fronteira: handles, buffers e
syscalls ficarão atrás da ABI C, enquanto opções, macros de lifecycle e readers
derivados serão implementados em Clojure. A [ADR-0007](adr/0007-native-io-and-runtime-reader.md)
registra as alternativas. Essa arquitetura ainda não é uma capacidade executável,
exceto pelo baseline de saída.

## Persistência e memória

Vetores usam trie bitmap de 32 vias. Mapas e sets começam compactos e promovem para
HAMT. Todas essas coleções são imutáveis e preservam estrutura não alterada por
path-copying.

O GC é preciso, não móvel e single-thread. Cada função abre um frame de shadow stack com
`cljn_gc_enter` e o encerra com `cljn_gc_leave`; slots intermediários são atualizados por
load/store direto. O coletor não escaneia a pilha nativa.

## Componentes planejados

Os nomes a seguir aparecem em documentos históricos, mas ainda não são crates do
workspace:

- `clojure-ir`: HIR/LIR e passes próprios;
- `clojure-macroexpander`: macros de usuário em build-time;
- `clojure-loader` e `clojure-project`: namespaces e projetos multi-arquivo;
- `clojure-ffi`: interop C;
- primitivas ABI para streams, arquivos e filesystem descritas em
  [IO_SPEC](IO_SPEC.md);
- crates Rust separados para GC, runtime e coleções persistentes;
- backend C alternativo e linker dedicado.

Eles só devem ser introduzidos quando a separação trouxer uma fronteira testável melhor
que o corte atual.

## Plataformas

O build atual gera objeto e executável para o host suportado por Cranelift e pelo
compilador C disponível. Linux x86_64 é o ambiente exercitado no repositório. Windows,
macOS, ARM64 e cross-compilation permanecem metas de matriz; não são afirmados como
plataformas validadas até existirem jobs e fixtures ativos.

O gate de I/O é ainda mais estreito: Linux x86_64 é sua primeira plataforma
bloqueante, com operação síncrona, bloqueante e UTF-8 estrito.
