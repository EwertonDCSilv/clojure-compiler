//! Analyzer-AST adapter for the optional compiler-owned optimization IR.
//!
//! The first delivered slice lowers pure scalar islands to `clojure-ir`, runs
//! the verified local profile, and materializes proven constants back into the
//! analyzer program before the existing Cranelift lowering. Every AST variant is
//! traversed, while unsupported or effectful regions remain byte-for-byte
//! equivalent to the direct path. Full function CFG lowering remains tracked by
//! ADR-0014.
//!
//! [`optimize_program`] is the single entry point and orchestrates four
//! independent passes, split by responsibility and invariant: `scalar_lowering`
//! (constant-folds pure scalar islands through `clojure-ir`), `escape`
//! (finds functions taken as a value, for `facts` to treat conservatively),
//! `facts` (an interprocedural fixed point proving which method parameters
//! are always called with a fixnum), and `specialization` (rewrites AST
//! primitive calls proven fixnum-only to their specialized form). Pass order
//! is fixed: scalar folding first, then fact inference (which depends on
//! `escape`), then specialization (applied both per-method with its proven
//! parameter environment, and once more over `main_body` with an empty one).

use clojure_analyzer::Program;
use clojure_diagnostics::Diagnostic;
use clojure_ir::Representation;
use std::collections::HashMap;

mod escape;
mod facts;
mod scalar_lowering;
mod specialization;

use facts::{infer_parameter_facts, method_environment};
use scalar_lowering::optimize_ast;
use specialization::{infer_representation, specialize_fixnums};

#[cfg(test)]
use clojure_analyzer::{Ast, Callee, Dispatch, Prim};

/// Clones and conservatively optimizes every expression in a program.
pub(super) fn optimize_program(program: &Program) -> Result<Program, Diagnostic> {
    let mut optimized = program.clone();
    for function in &mut optimized.functions {
        for method in &mut function.methods {
            method.body = optimize_ast(method.body.clone())?;
        }
    }
    optimized.main_body = optimized
        .main_body
        .into_iter()
        .map(optimize_ast)
        .collect::<Result<_, _>>()?;

    let (parameter_facts, directly_called_methods) = infer_parameter_facts(&optimized);
    for function in &mut optimized.functions {
        for (method_index, method) in function.methods.iter_mut().enumerate() {
            let method_id = (function.name.clone(), method_index);
            let environment = method_environment(&parameter_facts, &function.name, method_index);
            method.optimization.proven_fixnum_params = environment
                .iter()
                .filter_map(|(slot, representation)| {
                    (*slot < method.params.len() as u32)
                        .then_some((*slot as usize, *representation))
                })
                .fold(
                    vec![false; method.params.len()],
                    |mut parameters, (slot, representation)| {
                        parameters[slot] = representation == Representation::FixnumTagged;
                        parameters
                    },
                );
            method.body = specialize_fixnums(method.body.clone(), &environment);
            method.optimization.proven_fixnum_return =
                infer_representation(&method.body, &environment) == Representation::FixnumTagged;
            method.optimization.specialized_fixnum_abi = method.rest.is_none()
                && directly_called_methods.contains(&method_id)
                && method
                    .optimization
                    .proven_fixnum_params
                    .iter()
                    .all(|proven| *proven)
                && method.optimization.proven_fixnum_return;
        }
    }
    optimized.main_body = optimized
        .main_body
        .into_iter()
        .map(|expression| specialize_fixnums(expression, &HashMap::new()))
        .collect();
    Ok(optimized)
}

#[cfg(test)]
#[path = "../tests/unit/ir_adapter/mod.rs"]
mod tests;
