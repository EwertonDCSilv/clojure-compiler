//! Unit tests for lib.rs.

use super::*;

#[test]
fn embedded_runtime_is_the_ordered_concatenation_of_unique_modules() {
    let mut concatenated = String::new();
    let mut names = std::collections::BTreeSet::new();
    for (name, source) in RUNTIME_MODULES {
        assert!(names.insert(*name), "módulo duplicado: {name}");
        assert!(!source.is_empty(), "módulo vazio: {name}");
        concatenated.push_str(source);
    }
    assert_eq!(RUNTIME_C, concatenated);
}

#[test]
fn collects_strings_from_every_nested_ast_container() {
    let ast = Ast::Do(vec![
        Ast::Str("plain".into()),
        Ast::Keyword("keyword".into()),
        Ast::VecLit(vec![Ast::Str("vector".into())]),
        Ast::SetLit(vec![Ast::Str("set".into())]),
        Ast::MapLit(vec![(Ast::Str("key".into()), Ast::Str("value".into()))]),
        Ast::If(
            Box::new(Ast::Str("test".into())),
            Box::new(Ast::Str("then".into())),
            Box::new(Ast::Str("else".into())),
        ),
        Ast::Recur(vec![Ast::Str("recur".into())]),
        Ast::MakeFn {
            lambda: "lambda".into(),
            arity: 1,
            captures: vec![Ast::Str("capture".into())],
        },
        Ast::CallValue {
            f: Box::new(Ast::Str("callable".into())),
            args: vec![Ast::Str("argument".into())],
        },
        Ast::Apply {
            f: Box::new(Ast::Str("apply".into())),
            fixed: vec![Ast::Str("fixed".into())],
            coll: Box::new(Ast::Str("collection".into())),
        },
        Ast::MakeRecord {
            type_name: "Point".into(),
            fields: vec![("x".into(), Ast::Str("field-value".into()))],
        },
        Ast::RegisterMethod {
            method_id: 1,
            key: Box::new(Ast::Str("dispatch-key".into())),
            impl_fn: Box::new(Ast::Str("implementation".into())),
        },
        Ast::Loop {
            slots: vec![(0, Ast::Str("loop-init".into()))],
            body: Box::new(Ast::Str("loop-body".into())),
        },
        Ast::Let {
            slots: vec![(1, Ast::Str("let-init".into()))],
            body: Box::new(Ast::Str("let-body".into())),
        },
        Ast::Call {
            callee: Callee::Prim(Prim::Print),
            args: vec![Ast::Str("call-arg".into())],
        },
    ]);

    let mut strings = Vec::new();
    collect_strings(&ast, &mut strings);
    assert!(strings.contains(&"plain".to_string()));
    assert!(strings.contains(&"keyword".to_string()));
    assert!(strings.contains(&"Point".to_string()));
    assert!(strings.contains(&"x".to_string()));
    assert!(strings.contains(&"implementation".to_string()));
    assert!(strings.contains(&"call-arg".to_string()));
    assert_eq!(strings.len(), 26);
}

#[test]
fn compiles_minimal_program_to_nonempty_object() {
    let program = Program {
        functions: vec![],
        main_body: vec![Ast::Int(42), Ast::Str("done".into())],
        main_local_count: 0,
        global_count: 0,
    };
    let object = compile_object(&program).expect("minimal program should compile");
    assert!(object.len() > 100);
    assert!(object.iter().any(|byte| *byte != 0));
}

#[test]
fn compiles_with_every_supported_optimization_level() {
    let program = Program {
        functions: vec![],
        main_body: vec![Ast::Int(42)],
        main_local_count: 0,
        global_count: 0,
    };

    for optimization_level in [
        OptimizationLevel::None,
        OptimizationLevel::Speed,
        OptimizationLevel::SpeedAndSize,
    ] {
        let object = compile_object_with_options(
            &program,
            CodegenOptions {
                optimization_level,
                ..CodegenOptions::default()
            },
        )
        .expect("all supported optimization levels should compile");
        assert!(object.len() > 100);
    }
}

#[test]
fn parses_public_optimization_level_names() {
    assert_eq!(
        "none".parse::<OptimizationLevel>(),
        Ok(OptimizationLevel::None)
    );
    assert_eq!(
        "speed".parse::<OptimizationLevel>(),
        Ok(OptimizationLevel::Speed)
    );
    assert_eq!(
        "speed-and-size".parse::<OptimizationLevel>(),
        Ok(OptimizationLevel::SpeedAndSize)
    );
    assert!("fast".parse::<OptimizationLevel>().is_err());
    assert_eq!("adr15".parse::<IrExperiment>(), Ok(IrExperiment::Adr15));
    assert!("future".parse::<IrExperiment>().is_err());
}

#[test]
fn adr15_reports_a_specialized_call_without_generic_argv_spills() {
    let function_name = "test/add".to_string();
    let program = Program {
        functions: vec![clojure_analyzer::Function {
            name: function_name.clone(),
            methods: vec![FnMethod {
                params: vec!["left".into(), "right".into()],
                rest: None,
                body: Ast::Call {
                    callee: Callee::Prim(Prim::Add),
                    args: vec![Ast::Local(0), Ast::Local(1)],
                },
                optimization: Default::default(),
            }],
            local_count: 2,
            is_lambda: false,
            dispatch: Dispatch::None,
        }],
        main_body: vec![Ast::Call {
            callee: Callee::Fn(function_name),
            args: vec![Ast::Int(20), Ast::Int(22)],
        }],
        main_local_count: 0,
        global_count: 0,
    };

    let (_, stats) = compile_object_with_options_and_stats(
        &program,
        CodegenOptions {
            ir_optimization: IrOptimizationMode::Safe,
            ir_experiment: IrExperiment::Adr15,
            ..CodegenOptions::default()
        },
    )
    .expect("ADR-0015 candidate should compile");

    assert_eq!(stats.specialized_entries, 1);
    assert_eq!(stats.specialized_direct_calls, 1);
    assert_eq!(stats.generic_direct_calls, 0);
    assert_eq!(stats.generic_argv_spills, 0);
    assert!(stats.raw_fixnum_bindings >= 2);
    assert!(stats
        .to_json()
        .contains("\"schema\": \"clojure-compiler-optimization-stats-v1\""));
}

#[test]
fn adr15_compacts_fixnum_locals_but_keeps_an_escaped_generic_boundary() {
    let function_name = "test/escaped".to_string();
    let program = Program {
        functions: vec![clojure_analyzer::Function {
            name: function_name.clone(),
            methods: vec![FnMethod {
                params: vec!["unknown".into()],
                rest: None,
                body: Ast::Let {
                    slots: vec![(1, Ast::Int(41))],
                    body: Box::new(Ast::Local(0)),
                },
                optimization: Default::default(),
            }],
            local_count: 2,
            is_lambda: false,
            dispatch: Dispatch::None,
        }],
        main_body: vec![Ast::FnRef(function_name)],
        main_local_count: 0,
        global_count: 0,
    };
    let (_, control) = compile_object_with_options_and_stats(
        &program,
        CodegenOptions {
            ir_optimization: IrOptimizationMode::Safe,
            ..CodegenOptions::default()
        },
    )
    .expect("safe control should compile");
    let (_, candidate) = compile_object_with_options_and_stats(
        &program,
        CodegenOptions {
            ir_optimization: IrOptimizationMode::Safe,
            ir_experiment: IrExperiment::Adr15,
            ..CodegenOptions::default()
        },
    )
    .expect("ADR-0015 candidate should compile");

    assert_eq!(candidate.specialized_entries, 0);
    assert_eq!(candidate.root_slots, 1);
    assert_eq!(control.root_slots, 2);
}

#[test]
fn adr15_compiles_every_raw_fixnum_operation_and_loop_slot() {
    let call = |primitive, args| Ast::Call {
        callee: Callee::Prim(primitive),
        args,
    };
    let binary = |primitive| call(primitive, vec![Ast::Local(0), Ast::Local(1)]);
    let unary = |primitive| call(primitive, vec![Ast::Local(0)]);
    let loop_body = Ast::Loop {
        slots: vec![(2, Ast::Local(0))],
        body: Box::new(Ast::If(
            Box::new(call(Prim::Lt, vec![Ast::Local(2), Ast::Local(1)])),
            Box::new(Ast::Recur(vec![call(Prim::Inc, vec![Ast::Local(2)])])),
            Box::new(Ast::Local(2)),
        )),
    };
    let body = Ast::Do(vec![
        binary(Prim::Add),
        unary(Prim::Sub),
        binary(Prim::Sub),
        binary(Prim::Mul),
        binary(Prim::Quot),
        binary(Prim::Mod),
        unary(Prim::Inc),
        unary(Prim::Dec),
        binary(Prim::Eq),
        binary(Prim::Lt),
        binary(Prim::Le),
        binary(Prim::Gt),
        binary(Prim::Ge),
        Ast::Let {
            slots: vec![(3, call(Prim::Add, vec![Ast::Local(0), Ast::Int(1)]))],
            body: Box::new(Ast::Local(3)),
        },
        loop_body,
    ]);
    let function_name = "test/raw-operations".to_string();
    let program = Program {
        functions: vec![clojure_analyzer::Function {
            name: function_name.clone(),
            methods: vec![FnMethod {
                params: vec!["left".into(), "right".into()],
                rest: None,
                body,
                optimization: Default::default(),
            }],
            local_count: 4,
            is_lambda: false,
            dispatch: Dispatch::None,
        }],
        main_body: vec![Ast::Call {
            callee: Callee::Fn(function_name),
            args: vec![Ast::Int(4), Ast::Int(9)],
        }],
        main_local_count: 0,
        global_count: 0,
    };

    let (object, stats) = compile_object_with_options_and_stats(
        &program,
        CodegenOptions {
            ir_optimization: IrOptimizationMode::Safe,
            ir_experiment: IrExperiment::Adr15,
            ..CodegenOptions::default()
        },
    )
    .expect("all ADR-0015 raw operations should lower");

    assert!(!object.is_empty());
    assert_eq!(stats.specialized_entries, 1);
    assert_eq!(stats.specialized_direct_calls, 1);
    assert!(stats.raw_fixnum_bindings >= 6);
    assert!(stats.runtime_abi_calls > 0, "slow paths must stay imported");
}

#[test]
fn default_optimization_remains_unoptimized_until_the_speed_gate_passes() {
    assert_eq!(CodegenOptions::default(), CodegenOptions::unoptimized());
    assert_eq!(
        CodegenOptions::optimized_for_speed().optimization_level,
        OptimizationLevel::Speed
    );
}

#[test]
fn adr18_d1_gc_sp_flushed_once_per_call_not_per_push() {
    // Without D1 each gc_push_val stores gc_sp to the global, so 3 pushes
    // before a call produce ≥ 3 stores.  With D1 a single flush before each
    // call site reduces stores to at most 1 per call regardless of push count.
    let function_name = "test/three-arg-assoc".to_string();
    let program = Program {
        functions: vec![clojure_analyzer::Function {
            name: function_name.clone(),
            methods: vec![FnMethod {
                params: vec!["coll".into(), "k".into(), "v".into()],
                rest: None,
                body: Ast::Call {
                    callee: Callee::Prim(Prim::Assoc),
                    args: vec![Ast::Local(0), Ast::Local(1), Ast::Local(2)],
                },
                optimization: Default::default(),
            }],
            local_count: 3,
            is_lambda: false,
            dispatch: Dispatch::None,
        }],
        main_body: vec![Ast::Call {
            callee: Callee::Fn(function_name),
            args: vec![Ast::Str("key".into()), Ast::Int(1), Ast::Int(2)],
        }],
        main_local_count: 0,
        global_count: 0,
    };
    let (_, stats) = compile_object_with_options_and_stats(&program, CodegenOptions::default())
        .expect("should compile");
    // D1: 3 consecutive pushes before a call must not each store gc_sp
    // separately; the cached variable absorbs them and only every runtime
    // call (`call1`/`call2`/.../`gc_enter`) flushes it. The two functions
    // here (main body, `test/three-arg-assoc`) have 2 `gc_enter` flushes,
    // and the one user-function call site flushes twice (once to compute
    // `argv`, once immediately before the call itself), plus the `assoc`
    // primitive's own runtime call: 7 total, well under the 3-per-push
    // count D1 replaces (9+ pre-D1 for this program). The budget check is
    // deliberately loose -- it is not a substitute for the end-to-end
    // `gc_correctness_under_stress`/`native_connector_bodies_and_gc_stress`
    // tests in clojure-native-cli, which exercise real GC collection and
    // are the tests that actually catch a stale/desynchronized `gc_sp`.
    assert!(
        stats.gc_sp_global_stores <= 8,
        "D1 (ADR-0018): flush count should track call sites, not push count; got {}",
        stats.gc_sp_global_stores
    );
}

#[test]
fn codegen_diagnostics_use_stable_code() {
    let one = single("failure");
    assert_eq!(one.items[0].code, "E0120");
    assert_eq!(one.items[0].message, "failure");
    assert_eq!(single_d("other").code, "E0120");
}
