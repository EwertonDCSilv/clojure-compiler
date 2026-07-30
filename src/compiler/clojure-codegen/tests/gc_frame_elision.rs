//! Structural contracts for generated shadow-stack frame elision.

use clojure_analyzer::{Ast, Callee, Dispatch, FnMethod, Function, Prim, Program};
use clojure_codegen::{compile_object_with_options_and_stats, CodegenOptions, OptimizationStats};

fn compile_stats(functions: Vec<Function>) -> OptimizationStats {
    let program = Program {
        functions,
        main_body: vec![Ast::Int(0)],
        main_local_count: 0,
        global_count: 0,
    };

    compile_object_with_options_and_stats(&program, CodegenOptions::default())
        .expect("frame sentinel program should compile")
        .1
}

fn rootless_function(name: &str, body: Ast) -> Function {
    Function {
        name: name.to_owned(),
        methods: vec![FnMethod {
            params: vec![],
            rest: None,
            body,
            optimization: Default::default(),
        }],
        local_count: 0,
        is_lambda: false,
        dispatch: Dispatch::None,
    }
}

#[test]
fn elides_only_the_rootless_function_that_cannot_leave_a_temporary_root() {
    let baseline = compile_stats(vec![]);
    let pure = compile_stats(vec![rootless_function("test/pure", Ast::Int(42))]);
    let balanced_allocation = compile_stats(vec![rootless_function(
        "test/balanced-allocation",
        Ast::Call {
            callee: Callee::Prim(Prim::Eq),
            args: vec![Ast::Str("same".to_owned()), Ast::Str("same".to_owned())],
        },
    )]);
    let allocating = compile_stats(vec![rootless_function(
        "test/allocating",
        Ast::Str("rooted result".to_owned()),
    )]);

    assert_eq!(
        pure.root_frame_entries, baseline.root_frame_entries,
        "a proven immediate result must not add a zero-slot frame"
    );
    assert_eq!(
        balanced_allocation.root_frame_entries, baseline.root_frame_entries,
        "balanced temporary roots must not force a zero-slot frame"
    );
    assert_eq!(
        allocating.root_frame_entries,
        baseline.root_frame_entries + 1,
        "a heap result still needs a frame to restore its temporary root"
    );
}
