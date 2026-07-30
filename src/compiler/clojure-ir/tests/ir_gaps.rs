//! Closes coverage gaps left in `passes.rs`, `verify.rs`, and `print.rs` (issue #117):
//! a positive/negative matrix per pass, a wider verifier diagnostic matrix, and
//! printer snapshots for every instruction and terminator kind. This file adds no
//! new pass, optimization, or output; it only characterizes existing behavior.

use clojure_ir::{
    print_module, run_safe_pipeline, verify_function, BinaryOp, Constant, Effects, Function,
    FunctionBuilder, GuardKind, InstructionKind, Module, Representation, Terminator, UnaryOp,
    ValueId, VerifyOptions,
};
use clojure_span::Span;

// ---------------------------------------------------------------------------
// Pass matrix: one transforming and one non-transforming case per safe pass.
// ---------------------------------------------------------------------------

fn entry_returns_and_leaves_a_block_unreachable() -> Function {
    let mut builder = FunctionBuilder::new("dead_block");
    let value = builder.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(value));
    builder.create_block(&[]); // never branched to; keeps the default `unreachable` terminator.
    builder.finish()
}

#[test]
fn simplify_cfg_removes_a_provably_unreachable_block() {
    let mut function = entry_returns_and_leaves_a_block_unreachable();
    assert_eq!(function.blocks.len(), 2);
    let report = run_safe_pipeline(&mut function).unwrap();
    assert!(report.changed_passes.contains(&"simplify-cfg"));
    assert_eq!(report.removed_blocks, 1);
    assert_eq!(function.blocks.len(), 1);
}

fn diamond_with_runtime_condition() -> Function {
    let mut builder = FunctionBuilder::new("diamond");
    let condition = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_truthy".into(),
            arguments: Vec::new(),
        },
        Representation::UnknownTagged,
        Effects::none(),
        None,
    );
    let then_block = builder.create_block(&[]);
    let else_block = builder.create_block(&[]);
    builder.terminate(Terminator::CondBranch {
        condition,
        then_target: then_block,
        then_arguments: Vec::new(),
        else_target: else_block,
        else_arguments: Vec::new(),
    });
    builder.switch_to_block(then_block);
    let then_value = builder.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(then_value));
    builder.switch_to_block(else_block);
    let else_value = builder.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(else_value));
    builder.finish()
}

#[test]
fn simplify_cfg_leaves_a_fully_reachable_diamond_untouched() {
    let mut function = diamond_with_runtime_condition();
    let before = function.blocks.len();
    let report = run_safe_pipeline(&mut function).unwrap();
    assert!(report.changed_passes.is_empty());
    assert_eq!(function.blocks.len(), before);
}

fn value_defined_then_copied_and_used_via_copy() -> Function {
    let mut builder = FunctionBuilder::new("copy_prop");
    let original = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_identity_allocating".into(),
            arguments: Vec::new(),
        },
        Representation::HeapReference,
        Effects::none(),
        None,
    );
    let copied = builder.append(
        InstructionKind::Copy(original),
        Representation::HeapReference,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(copied));
    builder.finish()
}

#[test]
fn copy_propagation_removes_a_pure_copy_and_rewrites_its_use() {
    let mut function = value_defined_then_copied_and_used_via_copy();
    let report = run_safe_pipeline(&mut function).unwrap();
    assert!(report.changed_passes.contains(&"copy-propagation"));
    assert_eq!(function.blocks[0].instructions.len(), 1);
    match &function.blocks[0].terminator {
        Terminator::Return(value) => assert_eq!(*value, ValueId(0)),
        other => panic!("unexpected terminator: {other:?}"),
    }
}

fn two_constants_added() -> Function {
    let mut builder = FunctionBuilder::new("plain_add");
    let left = builder.append(
        InstructionKind::Constant(Constant::Fixnum(3)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let right = builder.append(
        InstructionKind::Constant(Constant::Fixnum(4)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let sum = builder.append(
        InstructionKind::Binary {
            op: BinaryOp::Add,
            left,
            right,
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(sum));
    builder.finish()
}

#[test]
fn copy_propagation_is_a_no_op_without_copy_instructions() {
    let mut function = two_constants_added();
    let report = run_safe_pipeline(&mut function).unwrap();
    assert!(!report.changed_passes.contains(&"copy-propagation"));
}

fn duplicate_pure_binary_expressions() -> Function {
    let mut builder = FunctionBuilder::new("cse");
    let a = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_opaque_a".into(),
            arguments: Vec::new(),
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let b = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_opaque_b".into(),
            arguments: Vec::new(),
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let first = builder.append(
        InstructionKind::Binary {
            op: BinaryOp::Add,
            left: a,
            right: b,
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let second = builder.append(
        InstructionKind::Binary {
            op: BinaryOp::Add,
            left: a,
            right: b,
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let sum = builder.append(
        InstructionKind::Binary {
            op: BinaryOp::Add,
            left: first,
            right: second,
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(sum));
    builder.finish()
}

#[test]
fn local_cse_merges_a_duplicate_pure_binary_expression() {
    let mut function = duplicate_pure_binary_expressions();
    let before = function.blocks[0].instructions.len();
    let report = run_safe_pipeline(&mut function).unwrap();
    assert!(report.changed_passes.contains(&"local-cse"));
    assert_eq!(function.blocks[0].instructions.len(), before - 1);
}

fn duplicate_impure_runtime_calls() -> Function {
    let mut builder = FunctionBuilder::new("cse_impure");
    let first = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_print_marker".into(),
            arguments: Vec::new(),
        },
        Representation::NilImmediate,
        Effects::MAY_PERFORM_IO,
        None,
    );
    let second = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_print_marker".into(),
            arguments: Vec::new(),
        },
        Representation::NilImmediate,
        Effects::MAY_PERFORM_IO,
        None,
    );
    let result = builder.append(
        InstructionKind::Binary {
            op: BinaryOp::Equal,
            left: first,
            right: second,
        },
        Representation::BooleanImmediate,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(result));
    builder.finish()
}

#[test]
fn local_cse_never_merges_impure_instructions_even_when_syntactically_identical() {
    let mut function = duplicate_impure_runtime_calls();
    let before = function.blocks[0].instructions.len();
    let report = run_safe_pipeline(&mut function).unwrap();
    assert!(!report.changed_passes.contains(&"local-cse"));
    assert_eq!(function.blocks[0].instructions.len(), before);
}

fn cond_branch_on_constant(condition_value: bool) -> Function {
    let mut builder = FunctionBuilder::new("const_branch");
    let condition = builder.append(
        InstructionKind::Constant(Constant::Boolean(condition_value)),
        Representation::BooleanImmediate,
        Effects::none(),
        None,
    );
    let then_block = builder.create_block(&[]);
    let else_block = builder.create_block(&[]);
    builder.terminate(Terminator::CondBranch {
        condition,
        then_target: then_block,
        then_arguments: Vec::new(),
        else_target: else_block,
        else_arguments: Vec::new(),
    });
    builder.switch_to_block(then_block);
    let then_value = builder.append(
        InstructionKind::Constant(Constant::Fixnum(1)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(then_value));
    builder.switch_to_block(else_block);
    let else_value = builder.append(
        InstructionKind::Constant(Constant::Fixnum(2)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(else_value));
    builder.finish()
}

#[test]
fn branch_simplification_folds_a_true_constant_condition_to_the_then_edge() {
    let mut function = cond_branch_on_constant(true);
    let report = run_safe_pipeline(&mut function).unwrap();
    assert!(report.changed_passes.contains(&"branch-simplification"));
    assert_eq!(function.blocks.len(), 2);
    let text = print_module(&Module {
        functions: vec![function.clone()],
    });
    assert!(text.contains("const.fixnum 1"));
    assert!(!text.contains("const.fixnum 2"));
}

#[test]
fn branch_simplification_folds_a_false_constant_condition_to_the_else_edge() {
    let mut function = cond_branch_on_constant(false);
    let report = run_safe_pipeline(&mut function).unwrap();
    assert!(report.changed_passes.contains(&"branch-simplification"));
    assert_eq!(function.blocks.len(), 2);
    let text = print_module(&Module {
        functions: vec![function.clone()],
    });
    assert!(text.contains("const.fixnum 2"));
    assert!(!text.contains("const.fixnum 1"));
}

fn cond_branch_on_runtime_condition() -> Function {
    let mut builder = FunctionBuilder::new("dynamic_branch");
    let condition = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_truthy".into(),
            arguments: Vec::new(),
        },
        Representation::UnknownTagged,
        Effects::none(),
        None,
    );
    let then_block = builder.create_block(&[]);
    let else_block = builder.create_block(&[]);
    builder.terminate(Terminator::CondBranch {
        condition,
        then_target: then_block,
        then_arguments: Vec::new(),
        else_target: else_block,
        else_arguments: Vec::new(),
    });
    builder.switch_to_block(then_block);
    let then_value = builder.append(
        InstructionKind::Constant(Constant::Fixnum(1)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(then_value));
    builder.switch_to_block(else_block);
    let else_value = builder.append(
        InstructionKind::Constant(Constant::Fixnum(2)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(else_value));
    builder.finish()
}

#[test]
fn branch_simplification_leaves_a_dynamic_condition_untouched() {
    let mut function = cond_branch_on_runtime_condition();
    let report = run_safe_pipeline(&mut function).unwrap();
    assert!(report.changed_passes.is_empty());
    assert_eq!(function.blocks.len(), 3);
}

fn chain_of_dead_pure_values() -> Function {
    let mut builder = FunctionBuilder::new("dead_chain");
    let base = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_opaque".into(),
            arguments: Vec::new(),
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    builder.append(
        InstructionKind::Unary {
            op: UnaryOp::Inc,
            value: base,
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let nil = builder.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(nil));
    builder.finish()
}

#[test]
fn effect_aware_dce_iteratively_removes_a_chain_of_dead_pure_values() {
    let mut function = chain_of_dead_pure_values();
    let report = run_safe_pipeline(&mut function).unwrap();
    assert!(report.changed_passes.contains(&"effect-aware-dce"));
    assert_eq!(report.removed_instructions, 2);
    assert_eq!(function.blocks[0].instructions.len(), 1);
}

fn no_dead_values_present() -> Function {
    let mut builder = FunctionBuilder::new("all_live");
    let value = builder.append(
        InstructionKind::Constant(Constant::Fixnum(5)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(value));
    builder.finish()
}

#[test]
fn effect_aware_dce_is_a_no_op_when_every_pure_value_is_used() {
    let mut function = no_dead_values_present();
    let report = run_safe_pipeline(&mut function).unwrap();
    assert!(!report.changed_passes.contains(&"effect-aware-dce"));
    assert_eq!(function.blocks[0].instructions.len(), 1);
}

// ---------------------------------------------------------------------------
// Verifier invariants: one negative case per structural, dominance, ABI, and
// GC-root rule, plus a positive loop/join case exercising every CFG shape.
// ---------------------------------------------------------------------------

#[test]
fn verifier_rejects_a_duplicated_block_id() {
    let mut function = two_constants_added();
    let duplicate = function.blocks[0].clone();
    function.blocks.push(duplicate);
    let error = verify_function(&function, VerifyOptions::default()).unwrap_err();
    assert!(error.to_string().contains("bloco definido mais de uma vez"));
}

#[test]
fn verifier_rejects_a_missing_entry_block() {
    let mut function = two_constants_added();
    function.entry = clojure_ir::BlockId(99);
    let error = verify_function(&function, VerifyOptions::default()).unwrap_err();
    assert!(error.to_string().contains("bloco de entrada inexistente"));
}

#[test]
fn verifier_rejects_a_value_defined_more_than_once() {
    let mut function = two_constants_added();
    function.blocks[0].instructions[1].result = function.blocks[0].instructions[0].result;
    let error = verify_function(&function, VerifyOptions::default()).unwrap_err();
    assert!(error
        .to_string()
        .contains("valor v0 definido mais de uma vez"));
}

#[test]
fn verifier_rejects_a_result_without_a_representation() {
    let mut function = two_constants_added();
    function.blocks[0].instructions[0].representation = None;
    let error = verify_function(&function, VerifyOptions::default()).unwrap_err();
    assert!(error
        .to_string()
        .contains("resultado e representação divergentes"));
}

#[test]
fn verifier_rejects_a_runtime_call_with_an_empty_symbol() {
    let mut builder = FunctionBuilder::new("empty_symbol");
    let value = builder.append(
        InstructionKind::RuntimeCall {
            symbol: String::new(),
            arguments: Vec::new(),
        },
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(value));
    let error = verify_function(&builder.finish(), VerifyOptions::default()).unwrap_err();
    assert!(error.to_string().contains("runtime call sem símbolo"));
}

#[test]
fn verifier_rejects_a_branch_to_a_nonexistent_block() {
    let mut builder = FunctionBuilder::new("bad_target");
    builder.terminate(Terminator::Branch {
        target: clojure_ir::BlockId(42),
        arguments: Vec::new(),
    });
    let error = verify_function(&builder.finish(), VerifyOptions::default()).unwrap_err();
    assert!(error.to_string().contains("referência a bloco b42"));
}

#[test]
fn verifier_rejects_incompatible_representations_across_a_branch_edge() {
    let mut builder = FunctionBuilder::new("mismatch_edge");
    let target = builder.create_block(&[Representation::FixnumTagged]);
    let nil = builder.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Branch {
        target,
        arguments: vec![nil],
    });
    builder.switch_to_block(target);
    let placeholder = builder.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(placeholder));
    let error = verify_function(&builder.finish(), VerifyOptions::default()).unwrap_err();
    assert!(error.to_string().contains("mistura"));
}

#[test]
fn verifier_rejects_a_duplicated_root_slot() {
    let mut builder = FunctionBuilder::new("dup_slot");
    let first = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_alloc_a".into(),
            arguments: Vec::new(),
        },
        Representation::HeapReference,
        Effects::MAY_ALLOCATE.union(Effects::MAY_SAFEPOINT),
        None,
    );
    let second = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_alloc_b".into(),
            arguments: Vec::new(),
        },
        Representation::HeapReference,
        Effects::MAY_ALLOCATE.union(Effects::MAY_SAFEPOINT),
        None,
    );
    builder.terminate(Terminator::Return(second));
    let mut function = builder.finish();
    function.root_slots.insert(first, 0);
    function.root_slots.insert(second, 0);
    let error = verify_function(
        &function,
        VerifyOptions {
            require_root_plan: true,
        },
    )
    .unwrap_err();
    assert!(error.to_string().contains("duplicado"));
}

#[test]
fn verifier_rejects_a_root_slot_assigned_to_an_immediate_value() {
    let mut builder = FunctionBuilder::new("immediate_root");
    let value = builder.append(
        InstructionKind::Constant(Constant::Fixnum(1)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(value));
    let mut function = builder.finish();
    function.root_slots.insert(value, 0);
    let error = verify_function(
        &function,
        VerifyOptions {
            require_root_plan: true,
        },
    )
    .unwrap_err();
    assert!(error
        .to_string()
        .contains("root slot atribuído a v0 imediato"));
}

#[test]
fn verifier_rejects_a_missing_root_for_a_value_live_at_a_safepoint() {
    let mut builder = FunctionBuilder::new("missing_root");
    let heap = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_str_from".into(),
            arguments: Vec::new(),
        },
        Representation::HeapReference,
        Effects::MAY_ALLOCATE.union(Effects::MAY_SAFEPOINT),
        None,
    );
    let call = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_identity_allocating".into(),
            arguments: vec![heap],
        },
        Representation::HeapReference,
        Effects::MAY_ALLOCATE.union(Effects::MAY_SAFEPOINT),
        None,
    );
    builder.terminate(Terminator::Return(call));
    let function = builder.finish();
    let error = verify_function(
        &function,
        VerifyOptions {
            require_root_plan: true,
        },
    )
    .unwrap_err();
    assert!(error.to_string().contains("sem root"));
}

#[test]
fn verifier_rejects_a_use_not_dominated_by_its_definition() {
    let mut builder = FunctionBuilder::new("no_dominance");
    let condition = builder.append(
        InstructionKind::Constant(Constant::Boolean(true)),
        Representation::BooleanImmediate,
        Effects::none(),
        None,
    );
    let then_block = builder.create_block(&[]);
    let else_block = builder.create_block(&[]);
    builder.terminate(Terminator::CondBranch {
        condition,
        then_target: then_block,
        then_arguments: Vec::new(),
        else_target: else_block,
        else_arguments: Vec::new(),
    });
    builder.switch_to_block(then_block);
    let defined_in_then = builder.append(
        InstructionKind::Constant(Constant::Fixnum(1)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(defined_in_then));
    builder.switch_to_block(else_block);
    // Illegally reuses a value from a sibling, non-dominating block.
    builder.terminate(Terminator::Return(defined_in_then));
    let error = verify_function(&builder.finish(), VerifyOptions::default()).unwrap_err();
    assert!(error.to_string().contains("não domina"));
}

/// A loop header block receiving a counter through its parameter from two
/// predecessors (the preheader and its own back edge), exercising loops,
/// phi-like joins, and dominance of a back edge in one shape.
fn loop_with_join_and_back_edge() -> Function {
    let mut builder = FunctionBuilder::new("loop");
    let zero = builder.append(
        InstructionKind::Constant(Constant::Fixnum(0)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let header = builder.create_block(&[Representation::FixnumTagged]);
    // `builder_assigns_stable_monotonic_block_and_value_ids` (ir_contract.rs) pins
    // this allocation order: the header's sole parameter is the second value.
    let counter = ValueId(1);
    builder.terminate(Terminator::Branch {
        target: header,
        arguments: vec![zero],
    });
    builder.switch_to_block(header);
    let condition = builder.append(
        InstructionKind::Binary {
            op: BinaryOp::LessThan,
            left: counter,
            right: zero,
        },
        Representation::BooleanImmediate,
        Effects::none(),
        None,
    );
    let incremented = builder.append(
        InstructionKind::Unary {
            op: UnaryOp::Inc,
            value: counter,
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let exit = builder.create_block(&[]);
    builder.terminate(Terminator::CondBranch {
        condition,
        then_target: header,
        then_arguments: vec![incremented],
        else_target: exit,
        else_arguments: Vec::new(),
    });
    builder.switch_to_block(exit);
    let done = builder.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(done));
    builder.finish()
}

#[test]
fn verifier_accepts_a_self_looping_block_with_a_block_parameter() {
    let function = loop_with_join_and_back_edge();
    verify_function(&function, VerifyOptions::default()).unwrap();
}

// ---------------------------------------------------------------------------
// Printer snapshots: every instruction and terminator kind, effects/span
// suffixes, the root-plan trailer, block-parameter joins, and module order.
// ---------------------------------------------------------------------------

#[test]
fn printer_renders_every_instruction_kind_deterministically() {
    let mut builder = FunctionBuilder::new("kitchen_sink");
    let fixnum = builder.append(
        InstructionKind::Constant(Constant::Fixnum(7)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let boolean = builder.append(
        InstructionKind::Constant(Constant::Boolean(true)),
        Representation::BooleanImmediate,
        Effects::none(),
        None,
    );
    builder.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    let copied = builder.append(
        InstructionKind::Copy(fixnum),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let incremented = builder.append(
        InstructionKind::Unary {
            op: UnaryOp::Inc,
            value: copied,
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let decremented = builder.append(
        InstructionKind::Unary {
            op: UnaryOp::Dec,
            value: incremented,
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    builder.append(
        InstructionKind::Unary {
            op: UnaryOp::Not,
            value: boolean,
        },
        Representation::BooleanImmediate,
        Effects::none(),
        None,
    );
    let sum = builder.append(
        InstructionKind::Binary {
            op: BinaryOp::Add,
            left: fixnum,
            right: decremented,
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let guarded = builder.append(
        InstructionKind::Guard {
            kind: GuardKind::IsFixnum,
            value: sum,
        },
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let called = builder.append(
        InstructionKind::RuntimeCall {
            symbol: "cljn_add2".into(),
            arguments: vec![sum, guarded],
        },
        Representation::HeapReference,
        Effects::MAY_ALLOCATE.union(Effects::MAY_PERFORM_IO),
        Some(Span::new(3, 10, 14)),
    );
    builder.append_effect(
        InstructionKind::RootStore {
            value: sum,
            slot: 1,
        },
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(called));
    let mut function = builder.finish();
    function.root_slots.insert(called, 0);

    let expected = "fn kitchen_sink {\n\
\x20 b0():\n\
\x20   v0:fixnum = const.fixnum 7\n\
\x20   v1:bool = const.bool true\n\
\x20   v2:nil = const.nil\n\
\x20   v3:fixnum = copy v0\n\
\x20   v4:fixnum = inc.checked v3\n\
\x20   v5:fixnum = dec.checked v4\n\
\x20   v6:bool = not v1\n\
\x20   v7:fixnum = add.checked v0, v5\n\
\x20   v8:fixnum = guard.IsFixnum v7\n\
\x20   v9:heap = call cljn_add2(v7, v8) effects=0x24 @3:10..14\n\
\x20   root.store r1, v7\n\
\x20   return v9\n\
\x20 roots v9=r0\n\
}\n";
    assert_eq!(
        print_module(&Module {
            functions: vec![function.clone()]
        }),
        expected
    );
    verify_function(&function, VerifyOptions::default()).unwrap();
}

#[test]
fn printer_renders_a_throw_terminator() {
    let mut builder = FunctionBuilder::new("thrown");
    let value = builder.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Throw(value));
    let module = Module {
        functions: vec![builder.finish()],
    };
    let expected = "fn thrown {\n  b0():\n    v0:nil = const.nil\n    throw v0\n}\n";
    assert_eq!(print_module(&module), expected);
}

#[test]
fn printer_renders_an_unreachable_terminator_by_default() {
    let builder = FunctionBuilder::new("stub");
    let module = Module {
        functions: vec![builder.finish()],
    };
    let expected = "fn stub {\n  b0():\n    unreachable\n}\n";
    assert_eq!(print_module(&module), expected);
}

#[test]
fn printer_renders_block_parameters_and_a_branch_join() {
    let function = loop_with_join_and_back_edge();
    let expected = "fn loop {\n\
\x20 b0():\n\
\x20   v0:fixnum = const.fixnum 0\n\
\x20   br b1(v0)\n\
\x20 b1(v1:fixnum):\n\
\x20   v2:bool = lt v1, v0\n\
\x20   v3:fixnum = inc.checked v1\n\
\x20   brif v2 b1(v3) b2()\n\
\x20 b2():\n\
\x20   v4:nil = const.nil\n\
\x20   return v4\n\
}\n";
    assert_eq!(
        print_module(&Module {
            functions: vec![function]
        }),
        expected
    );
}

#[test]
fn printer_concatenates_multiple_functions_in_module_order() {
    let mut first = FunctionBuilder::new("first");
    let a = first.append(
        InstructionKind::Constant(Constant::Fixnum(1)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    first.terminate(Terminator::Return(a));

    let mut second = FunctionBuilder::new("second");
    let b = second.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    second.terminate(Terminator::Return(b));

    let module = Module {
        functions: vec![first.finish(), second.finish()],
    };
    let expected = "fn first {\n  b0():\n    v0:fixnum = const.fixnum 1\n    return v0\n}\n\
fn second {\n  b0():\n    v0:nil = const.nil\n    return v0\n}\n";
    assert_eq!(print_module(&module), expected);
}
