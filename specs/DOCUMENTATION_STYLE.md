# Compiler Documentation Style

## Audience and language

Write for contributors who understand systems programming but may not yet know
this compiler. Technical documentation and comments use concise English.
User-facing diagnostics, CLI messages, and snapshot output remain in Portuguese.

Describe current implementation rather than intent inferred from names. Mark
unimplemented design explicitly as **Planned** and link the relevant ADR or
specification.

## What to document

Every crate, Rust module, and executable starts with inner documentation (`//!`)
that answers:

1. What responsibility does this component own?
2. What does it receive and produce?
3. Which invariants or side effects cross its boundary?
4. Where does it sit in the reader-to-native pipeline?

Every public Rust item, field, and variant has `///` documentation. Private code
needs comments when its correctness depends on a non-obvious algorithm,
invariant, ownership or lifetime rule, GC root, ABI layout, span calculation,
recursive termination argument, complexity characteristic, or external effect.
Do not restate obvious syntax.

Every C runtime fragment has a subsystem header. Each exported `cljn_*` function
has an immediately preceding contract comment. Non-trivial static functions
receive the same treatment when they manage allocation, ownership, rooting,
errors, or algorithmic complexity.

Every `defn` in `core/core_compiled.clj` has a docstring describing its supported
semantics, evaluation strategy, limits, and any observable difference from the
JVM implementation.

## Contract markers

Use these exact prefixes when a short comment communicates a critical contract:

- `INVARIANT:` a property that must hold before and after an operation.
- `SAFETY:` the justification for pointer use, unchecked indexing, casts, or an
  otherwise implicit memory-safety assumption.
- `ABI:` a calling convention, tag, layout, symbol, or cross-language constraint.
- `GC:` a rooting, allocation, reachability, or safepoint requirement.

Place cross-language markers on both sides of a Rust/C contract when practical,
and name the counterpart file or symbol.

## Rust templates

### Crate or module

```rust
//! Reads UTF-8 Clojure source into spanned syntax forms.
//!
//! The reader receives a source name and UTF-8 text and returns [`Form`] values
//! whose byte ranges refer to the original [`SourceMap`]. It performs no name
//! resolution; that boundary belongs to `clojure-analyzer`.
```

### Public API

```rust
/// Parses every form in `source`, preserving source byte ranges.
///
/// # Errors
///
/// Returns a diagnostic when a token or collection is malformed. The first
/// error stops the current parse.
///
/// # Examples
///
/// ```
/// # use clojure_reader::read_all;
/// let forms = read_all("example.clj", "(+ 1 2)").unwrap();
/// assert_eq!(forms.len(), 1);
/// ```
pub fn read_all(/* ... */) -> Result</* ... */> {
    // ...
}
```

Only add `# Panics` for reachable, intentional panic conditions. Only add
`# Safety` to unsafe APIs or where an unsafe contract is exposed to the caller.
Examples should be executable unless explicitly marked `text` or `ignore` with a
reason.

### Private invariant

```rust
// INVARIANT: `slots` follows lexical binding order, so a local's numeric slot is
// stable for the complete analysis of this function body.
```

Use intra-doc links when they make navigation better. Prefer links to types and
methods over bare file names, and keep links valid under `cargo doc --no-deps`.

## C templates

### Fragment header

```c
/*
 * Persistent vector subsystem.
 *
 * Implements tree traversal and path-copying updates for vector Values.
 * ABI: tag and field offsets must match clojure-codegen/src/lib.rs.
 * GC: any heap Value live across allocation must be rooted.
 */
```

### Exported ABI function

```c
/*
 * Return `vector[index]`, or terminate through the runtime error path.
 *
 * ABI: `vector` and the return value use the tagged 64-bit Value encoding.
 * GC: does not allocate and therefore does not require a temporary root.
 * Complexity: O(log32 n).
 */
Value cljn_nth(Value vector, Value index);
```

State whether the function allocates, whether inputs must already be rooted,
whether returned buffers transfer ownership, and whether failure returns a value,
sets runtime state, calls a fatal path, or performs `longjmp`.

## Clojure template

```clojure
(defn map
  "Returns an eager sequence containing `(f item)` for each item in `coll`.

  Unlike clojure.core/map on the JVM, this bootstrap implementation realizes the
  complete result and currently accepts one collection."
  [f coll]
  ;; ...
)
```

Avoid claiming laziness, chunking, metadata preservation, numeric widening, or
JVM exception identity unless the current implementation provides it.

## Tests, fixtures, and generators

Document a test module's intent and the contract its cases protect. Prefer one
module overview and comments around unusual setup or oracle logic to repetitive
comments above individual assertions.

For conformance fixtures, describe the schema and status transition rules in the
support crate. Fixture-local comments should explain only unusual process,
filesystem, binary-stream, or oracle expectations.

Generators must document inputs, deterministic ordering, overwritten outputs,
and validation expectations. Never imply that generated output should be edited
by hand when the generator owns it.

## Review checklist

- The comment explains behavior that can be verified in current code.
- Public parameters, outputs, errors, and side effects are unambiguous.
- Critical layouts and root lifetimes have `ABI:` or `GC:` markers.
- Source offsets say whether they are bytes, Unicode scalar values, or display
  columns.
- Complexity is stated only where meaningful and accurate.
- Rust examples compile as doctests.
- No user-facing Portuguese text was translated.
- No behavior, symbol, public API, layout, checksum, or expected result changed.
