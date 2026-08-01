//! Semantic analysis and closure conversion for the compilable subset.
//!
//! [`analyze`] expands supported core macros, resolves lexical locals, captures,
//! top-level functions, protocols, records, and multimethods, validates call and
//! `recur` arities, and produces a backend-oriented [`Program`]. Closure
//! conversion assigns numeric frame slots and explicit capture vectors.
//! Unsupported source constructs become stable diagnostics; code generation
//! never needs to recover source-level binding rules.
//!
//! This crate is split by responsibility: `ast` (the backend-oriented AST and
//! primitive table), `top_level` (top-level form recognition: `def`/`defn`/
//! `defrecord`/`defprotocol`/`extend-type`/`defmulti`/`defmethod`), `analysis`
//! (scopes, captures, and the expression analyzer -- the cohesive core, not
//! split further), `optimizations` (the transient-accumulator rewrite), and
//! `primitives` (the primitive name table and arity checks). Submodules share
//! crate-internal helpers freely; only the historical public surface below is
//! re-exported outside the crate.

mod expand;

pub use expand::expand_all;

mod analysis;
mod ast;
mod optimizations;
mod primitives;
mod top_level;

pub use ast::{Ast, Callee, Dispatch, FnMethod, Function, MethodOptimization, Prim, Program};
pub use top_level::analyze;

#[cfg(test)]
pub(crate) use clojure_diagnostics::Diagnostics;

#[cfg(test)]
#[path = "../tests/unit/lib/mod.rs"]
mod tests;
