# Arquitetura do clojure-native

O projeto é organizado como um workspace Cargo com crates especializados em cada parte da pilha.

## Crates principais

- `clojure-native-cli`
  - CLI de usuário para `read`, `eval`, `run` e `build`
  - Comando `build` compila um programa Clojure para binário nativo via Cranelift e depois invoca o linker C
- `clojure-reader`
  - Leitor Clojure que converte texto em AST, com suporte a spans para diagnósticos
- `clojure-syntax`
  - Definição de AST e estruturas sintáticas do compilador
- `clojure-analyzer`
  - Analisa forms Clojure, resolve vars, valida `recur`, e produz um `Program` para o codegen
- `clojure-codegen`
  - Gera um objeto nativo Cranelift do `Program`
  - Importa runtime C para operações semânticas e GC
- `clojure-interp`
  - Interpretador usado durante bootstrap e testes de validação
- `clojure-value`
  - Representação de valores Clojure em Rust
- `clojure-diagnostics`
  - Diagnósticos e mensagens de erro legíveis
- `clojure-span`
  - Gerenciamento de spans de fonte

## Fluxo de compilação

1. O CLI lê o arquivo-fonte e o `core` compilável.
2. O leitor (`clojure-reader`) converte o texto em forms Clojure.
3. O analisador (`clojure-analyzer`) transforma forms em `Program` com AST analisado.
4. O codegen (`clojure-codegen`) converte o `Program` para objeto nativo Cranelift.
5. O runtime C embutido é gravado e o linker C produz o executável final.

## Runtime e GC

- Valores são representados como `i64` tagueado para fixnums, com ponteiros para strings e listas.
- Operações semânticas são implementadas em runtime C e chamadas pelo código gerado.
- O GC atual usa shadow-stack de roots via chamadas `gc_enter`, `gc_leave`, `gc_push`, `gc_popn` e `gc_set`.
- `loop/recur` é compilado como salto para o bloco de loop/fn, evitando crescimento da pilha.
