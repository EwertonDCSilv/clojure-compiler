# NATIVE_INTEROP.md

Interoperabilidade **nativa** (não-Java). Substitui o interop Java da JVM por FFI **C
ABI**. Crate `clojure-ffi`. `[FUTURO — Fase 11]` (não é MVP), mas a API é planejada agora
para não engessar a arquitetura.

## Princípios

- **C ABI** é a fronteira universal (`extern "C"`). Rust ABI só **interno** ao projeto.
- **Sem** interop Java: `.method`, `new`, `Class/field` continuam erros de compilação
  (Compat). O caminho para "chamar código externo" é FFI C.
- **Memory safety primeiro**: FFI é `unsafe` por baixo; a superfície Clojure é segura por
  construção, com política de `unsafe` (ver Segurança).
- Integração com GC: valores Clojure passados a C são **rooteados/pinados** durante a
  chamada; ponteiros crus vindos de C **não** são gerenciados pela GC (o usuário/lib
  controla o ciclo de vida). Ver [MEMORY_MODEL.md](MEMORY_MODEL.md).

## Duas direções

### A) Clojure → biblioteca C (carregar `.so`/`.dll`/`.dylib` e chamar símbolos)

API Clojure proposta (ilustrativa, a consolidar):

```clojure
(ns example
  (:require [cljn.ffi :as ffi]))

(def libc (ffi/load-library "c"))         ; resolve libc do sistema

(def c-strlen
  (ffi/defcfn libc "strlen"
    {:args   [:pointer]
     :return :usize}))

(c-strlen (ffi/c-string "hello"))         ; => 5
```

- **Tipos FFI** (mapa Clojure ↔ C): `:i8 :u8 :i16 :u16 :i32 :u32 :i64 :u64 :usize :isize
  :f32 :f64 :bool :pointer :c-string :void`. Structs/callbacks abaixo.
- **Carregamento**: `load-library` resolve por nome/caminho (respeitando plataforma).
  No modo AOT, bibliotecas podem ser **linkadas estaticamente** (declaradas no manifesto)
  ou carregadas **dinamicamente** em runtime (`dlopen`/`LoadLibrary`) — decisão por
  biblioteca. Preferir linkagem estática para manter o binário autônomo quando possível.
- **Marshalling**:
  - strings: `c-string` copia p/ buffer NUL-terminado gerenciado; leitura de `char*` de C
    copia p/ `Str` Clojure.
  - buffers: `ffi/alloc`/`ffi/buffer` para blocos de memória crus (fora da GC), com
    `free` explícito ou escopo `with-ffi`.
  - ponteiros: opacos (`:pointer`), envoltos em um `Value` `ForeignPtr` não-traçado.
  - structs: descritos por layout (`ffi/defstruct` com campos e tipos), com alinhamento/
    offset calculados; leitura/escrita por acessores.

### B) Rust/C → expor funções para Clojure

- Uma lib Rust do usuário pode expor funções `extern "C"` que o runtime registra como
  Vars/fns nativas via uma **tabela de registro** (`cljn_register_fn(name, arity, ptr)`),
  descrita em um módulo de binding. Isso permite escrever partes de performance em Rust e
  chamá-las de Clojure sem passar por FFI dinâmico.
- Convenção de assinatura: recebem/retornam `Value` (ABI C do runtime), responsáveis por
  rooting conforme o contrato do GC.

## Callbacks (Clojure → C que chama de volta Clojure)

- Um `Fn` Clojure pode ser exposto como ponteiro de função C via **trampolim**
  (`ffi/callback fn {:args [...] :return ...}`). O trampolim converte args C→Value, chama
  o fn (com rooting), converte retorno Value→C.
- Restrições: o callback deve viver enquanto a lib o retiver (o usuário mantém referência;
  registrada como root). Reentrância no GC single-thread do MVP é `[HIPÓTESE]` a validar.

## Erros e async

- **Erros**: convenções C (código de retorno/`errno`/out-param) são traduzidas para
  exceções Clojure por wrappers definidos pelo usuário; a camada FFI não inventa
  semântica de erro.
- **async / thread safety**: chamadas FFI bloqueantes rodam na thread atual (single-thread
  no MVP). FFI a partir de múltiplas threads e callbacks concorrentes ⇒ `[FUTURO]` (junto
  com o modelo multi-thread do runtime).

## Segurança e política de `unsafe`

- Todo o `unsafe` de FFI concentra-se em `clojure-ffi`, com invariantes documentadas,
  testes e (quando aplicável) verificação **Miri** nas partes puramente Rust.
- Carregar bibliotecas dinâmicas e chamar símbolos arbitrários é poderoso e perigoso:
  sujeito à política de segurança (ver seção Segurança no
  [RISK_REGISTER.md](RISK_REGISTER.md) e regras do start_spec §25). No MVP, FFI está fora
  do escopo; quando entrar, `load-library`/símbolos externos exigem declaração explícita
  no manifesto (sem carregamento implícito de caminhos não confiáveis).

## Protótipo obrigatório (start_spec §27 #12)

Antes de projetar a API final, um protótipo descartável valida **uma** chamada FFI C real
(ex.: `strlen`/`cos`), incluindo marshalling de string e rooting durante a chamada.
