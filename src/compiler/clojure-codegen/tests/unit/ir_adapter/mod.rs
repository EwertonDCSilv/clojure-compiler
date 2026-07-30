//! Unit tests for ir_adapter.rs.

use super::*;
use clojure_analyzer::{FnMethod, Function};

fn binary(primitive: Prim, left: Ast, right: Ast) -> Ast {
    Ast::Call {
        callee: Callee::Prim(primitive),
        args: vec![left, right],
    }
}

#[test]
fn folds_a_nested_checked_scalar_island() {
    let expression = binary(
        Prim::Mul,
        binary(Prim::Add, Ast::Int(2), Ast::Int(3)),
        Ast::Int(4),
    );
    let optimized = optimize_ast(expression).expect("valid IR");
    assert!(matches!(optimized, Ast::Int(20)));
}

#[test]
fn preserves_the_runtime_slow_path_when_fixnum_folding_would_overflow() {
    let expression = binary(Prim::Add, Ast::Int((1_i64 << 62) - 1), Ast::Int(1));
    let optimized = optimize_ast(expression).expect("valid IR");
    assert!(matches!(
        optimized,
        Ast::Call {
            callee: Callee::Prim(Prim::Add),
            ..
        }
    ));
}

#[test]
fn simplifies_only_a_proven_constant_branch() {
    let optimized = optimize_ast(Ast::If(
        Box::new(Ast::Bool(false)),
        Box::new(Ast::Int(1)),
        Box::new(binary(Prim::Sub, Ast::Int(8), Ast::Int(3))),
    ))
    .expect("valid IR");
    assert!(matches!(optimized, Ast::Int(5)));
}

#[test]
fn marks_fixnum_operations_closed_over_a_loop_fixed_point() {
    let loop_body = Ast::Loop {
        slots: vec![(0, Ast::Int(0))],
        body: Box::new(Ast::If(
            Box::new(binary(Prim::Lt, Ast::Local(0), Ast::Int(10))),
            Box::new(Ast::Recur(vec![Ast::Call {
                callee: Callee::Prim(Prim::Inc),
                args: vec![Ast::Local(0)],
            }])),
            Box::new(Ast::Local(0)),
        )),
    };
    let program = Program {
        functions: Vec::new(),
        main_body: vec![loop_body],
        main_local_count: 1,
        global_count: 0,
    };
    let optimized = optimize_program(&program).expect("valid IR");
    let Ast::Loop { body, .. } = &optimized.main_body[0] else {
        panic!("expected loop");
    };
    let Ast::If(test, then, _) = body.as_ref() else {
        panic!("expected conditional");
    };
    assert!(matches!(
        test.as_ref(),
        Ast::Call {
            callee: Callee::ProvenFixnumPrim(Prim::Lt),
            ..
        }
    ));
    let Ast::Recur(arguments) = then.as_ref() else {
        panic!("expected recur");
    };
    assert!(matches!(
        &arguments[0],
        Ast::Call {
            callee: Callee::ProvenFixnumPrim(Prim::Inc),
            ..
        }
    ));
}

#[test]
fn leaves_unknown_function_parameters_on_the_guarded_path() {
    let program = Program {
        functions: Vec::new(),
        main_body: vec![binary(Prim::Add, Ast::Local(0), Ast::Int(1))],
        main_local_count: 1,
        global_count: 0,
    };
    let optimized = optimize_program(&program).expect("valid IR");
    assert!(matches!(
        &optimized.main_body[0],
        Ast::Call {
            callee: Callee::Prim(Prim::Add),
            ..
        }
    ));
}

#[test]
fn propagates_fixnum_facts_through_a_direct_function_call() {
    let program = Program {
        functions: vec![Function {
            name: "bench/add-one".to_owned(),
            methods: vec![FnMethod {
                params: vec!["value".to_owned()],
                rest: None,
                body: binary(Prim::Add, Ast::Local(0), Ast::Int(1)),
                optimization: Default::default(),
            }],
            local_count: 1,
            is_lambda: false,
            dispatch: Dispatch::None,
        }],
        main_body: vec![Ast::Call {
            callee: Callee::Fn("bench/add-one".to_owned()),
            args: vec![Ast::Int(41)],
        }],
        main_local_count: 0,
        global_count: 0,
    };
    let optimized = optimize_program(&program).expect("valid IR");
    assert!(matches!(
        &optimized.functions[0].methods[0].body,
        Ast::Call {
            callee: Callee::ProvenFixnumPrim(Prim::Add),
            ..
        }
    ));
    assert_eq!(
        optimized.functions[0].methods[0]
            .optimization
            .proven_fixnum_params,
        vec![true]
    );
    assert!(
        optimized.functions[0].methods[0]
            .optimization
            .proven_fixnum_return
    );
    assert!(
        optimized.functions[0].methods[0]
            .optimization
            .specialized_fixnum_abi
    );
}

#[test]
fn does_not_specialize_parameters_of_an_escaped_function() {
    let program = Program {
        functions: vec![Function {
            name: "bench/add-one".to_owned(),
            methods: vec![FnMethod {
                params: vec!["value".to_owned()],
                rest: None,
                body: binary(Prim::Add, Ast::Local(0), Ast::Int(1)),
                optimization: Default::default(),
            }],
            local_count: 1,
            is_lambda: false,
            dispatch: Dispatch::None,
        }],
        main_body: vec![
            Ast::FnRef("bench/add-one".to_owned()),
            Ast::Call {
                callee: Callee::Fn("bench/add-one".to_owned()),
                args: vec![Ast::Int(41)],
            },
        ],
        main_local_count: 0,
        global_count: 0,
    };
    let optimized = optimize_program(&program).expect("valid IR");
    assert!(matches!(
        &optimized.functions[0].methods[0].body,
        Ast::Call {
            callee: Callee::Prim(Prim::Add),
            ..
        }
    ));
    assert!(
        !optimized.functions[0].methods[0]
            .optimization
            .specialized_fixnum_abi
    );
}

#[test]
fn joins_every_direct_call_before_specializing_a_parameter() {
    let program = Program {
        functions: vec![Function {
            name: "bench/add-one".to_owned(),
            methods: vec![FnMethod {
                params: vec!["value".to_owned()],
                rest: None,
                body: binary(Prim::Add, Ast::Local(0), Ast::Int(1)),
                optimization: Default::default(),
            }],
            local_count: 1,
            is_lambda: false,
            dispatch: Dispatch::None,
        }],
        main_body: vec![
            Ast::Call {
                callee: Callee::Fn("bench/add-one".to_owned()),
                args: vec![Ast::Int(41)],
            },
            Ast::Call {
                callee: Callee::Fn("bench/add-one".to_owned()),
                args: vec![Ast::Float(1.5)],
            },
        ],
        main_local_count: 0,
        global_count: 0,
    };
    let optimized = optimize_program(&program).expect("valid IR");
    assert!(matches!(
        &optimized.functions[0].methods[0].body,
        Ast::Call {
            callee: Callee::Prim(Prim::Add),
            ..
        }
    ));
}

#[test]
fn uses_intrinsic_fixnum_results_to_remove_downstream_guards() {
    let expression = binary(
        Prim::Lt,
        Ast::Int(0),
        Ast::Call {
            callee: Callee::Prim(Prim::Count),
            args: vec![Ast::Local(0)],
        },
    );
    let optimized = specialize_fixnums(expression, &HashMap::new());
    assert!(matches!(
        optimized,
        Ast::Call {
            callee: Callee::ProvenFixnumPrim(Prim::Lt),
            ..
        }
    ));
}
