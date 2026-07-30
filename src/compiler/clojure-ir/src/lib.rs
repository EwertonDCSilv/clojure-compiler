//! Backend-neutral control-flow IR for optional compiler-owned optimization.
//!
//! The crate receives semantically analyzed operations through [`FunctionBuilder`],
//! verifies explicit control flow, representations, effects, and safepoint root
//! plans, then applies deterministic conservative passes. It contains no Cranelift
//! types; final machine lowering remains owned by `clojure-codegen`.

mod analysis;
mod builder;
mod model;
mod passes;
mod print;
mod verify;

pub use analysis::{compute_liveness, plan_roots, Liveness};
pub use builder::FunctionBuilder;
pub use model::{
    BinaryOp, Block, BlockId, BlockParam, Constant, Effects, Function, GuardKind, Instruction,
    InstructionKind, Module, Representation, Terminator, UnaryOp, ValueId,
};
pub use passes::{run_safe_pipeline, PassReport};
pub use print::print_module;
pub use verify::{verify_function, VerifyError, VerifyOptions};
