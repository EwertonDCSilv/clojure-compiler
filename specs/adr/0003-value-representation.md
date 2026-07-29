# ADR-0003 — Representação de valores

- **Status:** Aceito e implementado (fixnums, Char/Float, Bytes e ponteiros tagged em produção)
- **Contexto:** todo valor Clojure em runtime precisa de uma representação uniforme que
  suporte imediatos rápidos (nil/bool/int/float/char), objetos de heap gerenciados pela GC,
  igualdade/hash consistentes, metadata e pattern-matching no runtime Rust. Ver
  [RUNTIME_SPEC.md](../RUNTIME_SPEC.md#representação-atual).

## Alternativas

| Opção | Prós | Contras |
| --- | --- | --- |
| **enum `Value`** (imediatos + `Gc<T>`) | seguro (sem bit-hacking), pattern-match ergonômico, casa com GC precisa | 16 bytes/valor, boxing de heap |
| Tagged pointers (low-bit) | 8 bytes, imediatos rápidos, comum em Lisps | `unsafe` de tagging, cuidado com GC/alinhamento |
| NaN boxing | 8 bytes, floats/immediates rápidos | `unsafe` denso, difícil com GC móvel, portabilidade de payload |
| Boxed trait objects (`Box<dyn>`) | flexível | alocação sempre, vtables, hash/eq trabalhosos |
| Handles do GC p/ tudo | uniform | imediatos pagam heap; lento |

## Decisão

**MVP: enum `Value` em Rust** com imediatos inline (`Nil, Bool, Int(i64), Float(f64),
Char, Keyword(id), Symbol(id)`) e `Obj(Gc<Obj>)` para o resto no heap gerenciado.
**Todo acesso passa por uma API** (`is_*`/`as_*`/construtores) para manter a representação
**trocável**.

### Justificativa
- **Segurança e velocidade de desenvolvimento** primeiro (start_spec §30: não começar por
  otimização): sem `unsafe` de tagging no MVP.
- **Pattern-matching** direto simplifica runtime, interpretador e testes.
- **Integração limpa com a GC precisa** (ADR-0002): `Obj` tem cabeçalho e tracer; imediatos
  não são heap.

### Trade-off e evolução (R19)
- Custo: 16 bytes/valor e boxing de heap. **Migração planejada** para tagged pointers (ou
  NaN boxing) quando benchmarks (Criterion) justificarem — viabilizada pela API de acesso
  que esconde a representação. Registrado como risco (migração invasiva) e adiado até haver
  dados.

## Consequências
- `clojure-value` define `Value`, `Obj`, igualdade/hash (invariante `= ⇒ hash igual`,
  testado por `proptest`), metadata em `Obj`.
- Protótipo #4 mede tamanho/custo e valida a ergonomia antes de congelar.
