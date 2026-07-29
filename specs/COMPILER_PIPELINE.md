# Compiler Pipeline

The current executable path is:

```text
Reader -> known expansion -> Analyzer -> Cranelift codegen -> object -> C runtime link
```

This page indexes the implemented stages. The optional optimization IR specified by
[`OPTIMIZATION_IR_SPEC.md`](OPTIMIZATION_IR_SPEC.md) and a general macroexpander are
**Planned**, not implicit stages in the current workspace.

## 1. Reader

[`clojure-reader`](../crates/clojure-reader/src/lib.rs) accepts UTF-8 text and
produces [`clojure-syntax`](../crates/clojure-syntax/src/lib.rs) forms carrying
byte spans into a [`SourceMap`](../crates/clojure-span/src/lib.rs).

- Whitespace, commas, comments, and a leading shebang are trivia.
- Literals cover nil, booleans, integers, floats, strings, symbols, and keywords.
- Lists, vectors, maps, and sets may nest.
- Reader macros cover quote, syntax quote, unquote, dereference, var quote,
  anonymous functions, metadata, and discard.
- Invalid delimiters, escapes, and tokens produce deterministic diagnostics.

Reader recognition does not imply native execution support. A form may parse
successfully and still be rejected by analysis or lowering.

## 2. Known expansion

The analyzer first expands the implemented core macro set:

```text
when  when-not  if-not  cond  and  or  ->  ->>
```

Expansion produces special forms before semantic analysis. `and` and `or`
preserve short-circuiting and single evaluation. **Planned:** user `defmacro`,
`&form`/`&env`, cross-namespace expansion, and arbitrary bootstrap macro
execution are tracked by [ADR-0004](adr/0004-macro-execution.md).

## 3. Analyzer

[`clojure-analyzer`](../crates/clojure-analyzer/src/lib.rs):

- resolves local slots and direct or transitive lexical captures;
- validates fixed, multiple, and variadic arities;
- validates `recur` target, tail position, and arity;
- distinguishes primitives, direct calls, and indirect calls;
- analyzes collection literals and synthesizes closures;
- represents primitive functions when they flow as values;
- records records, protocols, extensions, and multimethod declarations;
- computes conservative summaries for linear top-level parameters;
- promotes fresh, uniquely used vector loop accumulators to transients; and
- rejects forms outside the compiled subset with spanned diagnostics.

The result is the documented `Program`/`Expr` AST consumed directly by codegen by
default. An optional `clojure-ir` boundary now lowers pure scalar islands, verifies and
optimizes them, and materializes proven constants before direct codegen. It does not
yet lower complete functions or serve as the final Cranelift input. The
interprocedural transform covers
the chained-accumulator pattern in
[ADR-0010](adr/0010-interprocedural-ephemeral-vectors.md). General escape
analysis, tuple out-slots, scalar replacement, and complete function-level IR lowering
remain **Planned** under
[ADR-0014](adr/0014-optional-optimization-ir.md).

## 4. Code generation

[`clojure-codegen`](../crates/clojure-codegen/src/lib.rs) lowers analyzed
functions to Cranelift IR and emits one host object.

- Every function has the `(self, argc, argv)` ABI.
- `loop`/`recur` becomes a branch to the target block.
- Closures carry a code pointer, arity information, and captured `Value`s.
- Higher-order calls use indirect-call signatures.
- Common integer arithmetic and comparisons have guarded fast paths.
- General or invalid cases call checked C ABI slow paths.
- Functions with fixed root slots or an unbalanced heap result own a shadow-stack
  frame; proven balanced zero-slot functions omit it according to
  [ADR-0017](adr/0017-selective-zero-slot-gc-frames.md).
- Heap values live across allocation remain in root slots.
- Immediate-only vector literals use a permanently rooted site cache.

`CodegenOptions` independently accepts Cranelift levels `none`, `speed`, and
`speed-and-size`, plus compiler IR modes `none` and experimental `safe`. Both dimensions
default to `none`; benchmark methodology records them explicitly.

## 5. Runtime and link

The fragments in
[`crates/clojure-codegen/runtime/`](../crates/clojure-codegen/runtime/) are
amalgamated in the order documented by
[`runtime_all.c`](../crates/clojure-codegen/runtime/runtime_all.c). The C driver
selected by `CC` (or `cc`) compiles that translation unit and links it with the
Cranelift object. [`runtime.c`](../crates/clojure-codegen/runtime.c) exposes the
same amalgamation to direct C runtime tests.

The runtime implements:

- tagged values and the uniform call ABI;
- precise mark-sweep collection with a shadow stack;
- closures, arity checks, `apply`, and protocol dispatch;
- strings, printing, and structural equality;
- persistent and transient collections, records, and sorted collections;
- exceptions, multimethods, and fatal runtime checks;
- dynamic I/O, string/file readers and writers, byte arrays, filesystem
  operations, command arguments, file metadata, and runtime form reading; and
- permanent roots for cached constants.

`ABI:` and `GC:` comments in the Rust codegen and C fragments are the
authoritative cross-language contracts. The final executable contains no JVM or
`.class` bytecode.

## 6. Compiled core

Before user code, the CLI loads
[`core_compiled.clj`](../crates/clojure-native-cli/src/core_compiled.clj).
Its 26 documented functions use only the subset accepted by this pipeline.
Collection transforms are eager rather than Clojure/JVM lazy sequences, and
their docstrings state current arity and semantic limits.

## 7. Diagnostics and failure boundaries

The reader and analyzer use source spans to render file, line, and display
column through
[`clojure-diagnostics`](../crates/clojure-diagnostics/src/lib.rs). Build errors
include malformed forms, unavailable symbols or primitives, invalid `recur`,
and host linker failures. CLI-visible messages remain Portuguese.

Explicitly thrown values cross native `try`/`catch`/`finally`. Several runtime
type, arity, arithmetic, and bounds violations still use fatal process errors.
Typed catch hierarchies, source stack traces, and conversion of all fatal
runtime checks into catchable values are **Planned**; see
[`RUNTIME_SPEC.md`](RUNTIME_SPEC.md#erros).

## Planned evolution

1. Extend the optional verified scalar IR from
   [`OPTIMIZATION_IR_SPEC.md`](OPTIMIZATION_IR_SPEC.md) to complete function CFGs.
2. Lower verified IR directly to Cranelift and consume its root plan at safepoints.
3. Specialize internal values, root state, and closed direct calls according to
   [ADR-0015](adr/0015-internal-value-root-and-abi-specialization.md).
4. Promote compiler-owned passes only after their blocking Cormen non-regression gate.
5. Extend escape and uniqueness analysis beyond current vector patterns.
6. Execute user macros deterministically through the bootstrap path.

Documentation requirements for every stage are defined in
[`DOCUMENTATION_STYLE.md`](DOCUMENTATION_STYLE.md), and the delivery baseline is
recorded in
[`COMPILER_DOCUMENTATION_PLAN.md`](COMPILER_DOCUMENTATION_PLAN.md).
