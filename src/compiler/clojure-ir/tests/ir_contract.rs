//! Public model, verifier, optimizer, printer, and GC-plan contracts.

use clojure_ir::{
    print_module, run_safe_pipeline, verify_function, BinaryOp, BlockId, Constant, Effects,
    FunctionBuilder, InstructionKind, Module, Representation, Terminator, UnaryOp, ValueId,
    VerifyOptions,
};
use std::collections::BTreeMap;

fn scalar_add() -> clojure_ir::Function {
    let mut builder = FunctionBuilder::new("answer");
    let left = builder.append(
        InstructionKind::Constant(Constant::Fixnum(20)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let right = builder.append(
        InstructionKind::Constant(Constant::Fixnum(22)),
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
fn valid_function_verifies_and_prints_deterministically() {
    let function = scalar_add();
    verify_function(&function, VerifyOptions::default()).unwrap();
    let module = Module {
        functions: vec![function],
    };
    let expected = "fn answer {\n  b0():\n    v0:fixnum = const.fixnum 20\n    v1:fixnum = const.fixnum 22\n    v2:fixnum = add.checked v0, v1\n    return v2\n}\n";
    assert_eq!(print_module(&module), expected);
    assert_eq!(print_module(&module), expected);
}

#[test]
fn representations_join_conservatively_and_effects_remain_orthogonal() {
    assert_eq!(
        Representation::FixnumUnboxed.join(Representation::FixnumTagged),
        Representation::FixnumTagged
    );
    assert_eq!(
        Representation::HeapReference.join(Representation::NilImmediate),
        Representation::UnknownTagged
    );
    let effects = Effects::MAY_ALLOCATE
        .union(Effects::MAY_SAFEPOINT)
        .union(Effects::MAY_THROW);
    assert!(effects.may_safepoint());
    assert!(!effects.is_pure());
    assert_eq!(
        effects.bits(),
        Effects::MAY_ALLOCATE.bits() | Effects::MAY_SAFEPOINT.bits() | Effects::MAY_THROW.bits()
    );
}

#[test]
fn builder_assigns_stable_monotonic_block_and_value_ids() {
    let mut builder = FunctionBuilder::new("ids");
    let first = builder.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    let target = builder.create_block(&[Representation::UnknownTagged]);
    assert_eq!(builder.entry().0, 0);
    assert_eq!(target.0, 1);
    let function = builder.finish();
    assert_eq!(first.0, 0);
    assert_eq!(function.blocks[1].parameters[0].value.0, 1);
}

#[test]
fn model_queries_and_operand_rewrites_cover_every_cfg_shape() {
    let mut function = scalar_add();
    assert_eq!(
        function.block(BlockId(0)).map(|block| block.id),
        Some(BlockId(0))
    );
    function.block_mut(BlockId(0)).expect("entry block");

    let replacements = BTreeMap::from([(ValueId(1), ValueId(0))]);
    let mut instruction = InstructionKind::RootStore {
        value: ValueId(1),
        slot: 0,
    };
    assert_eq!(instruction.operands(), vec![ValueId(1)]);
    instruction.rewrite_operands(&replacements);
    assert_eq!(instruction.operands(), vec![ValueId(0)]);

    let mut branch = Terminator::CondBranch {
        condition: ValueId(1),
        then_target: BlockId(1),
        then_arguments: vec![ValueId(1)],
        else_target: BlockId(2),
        else_arguments: vec![ValueId(0)],
    };
    assert_eq!(branch.successors(), vec![BlockId(1), BlockId(2)]);
    assert_eq!(branch.operands(), vec![ValueId(1), ValueId(1), ValueId(0)]);
    branch.rewrite_operands(&replacements);
    assert_eq!(branch.operands(), vec![ValueId(0), ValueId(0), ValueId(0)]);
}

#[test]
fn verifier_rejects_use_before_definition_without_panicking() {
    let mut function = scalar_add();
    function.blocks[0].instructions[0].kind = InstructionKind::Copy(ValueId(2));
    let error = verify_function(&function, VerifyOptions::default()).unwrap_err();
    assert!(error.to_string().contains("antes da definição"));
}

#[test]
fn verifier_rejects_wrong_block_argument_count() {
    let mut builder = FunctionBuilder::new("edge");
    let target = builder.create_block(&[Representation::UnknownTagged]);
    builder.terminate(Terminator::Branch {
        target,
        arguments: Vec::new(),
    });
    builder.switch_to_block(target);
    let result = builder.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(result));
    let error = verify_function(&builder.finish(), VerifyOptions::default()).unwrap_err();
    assert!(error.to_string().contains("passa 0 valores para 1"));
}

#[test]
fn verifier_rejects_an_unboxed_value_at_the_public_abi() {
    let mut builder = FunctionBuilder::new("abi");
    let value = builder.append(
        InstructionKind::Constant(Constant::Fixnum(1)),
        Representation::FixnumUnboxed,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(value));
    let error = verify_function(&builder.finish(), VerifyOptions::default()).unwrap_err();
    assert!(error.to_string().contains("ABI tagged"));
}

#[test]
fn safe_pipeline_folds_constants_and_is_idempotent() {
    let mut function = scalar_add();
    let first = run_safe_pipeline(&mut function).unwrap();
    assert!(first.changed_passes.contains(&"checked-constant-folding"));
    let text = print_module(&Module {
        functions: vec![function.clone()],
    });
    assert!(text.contains("const.fixnum 42"));
    assert!(!text.contains("add.checked"));

    let second = run_safe_pipeline(&mut function).unwrap();
    assert!(second.changed_passes.is_empty());
}

#[test]
fn checked_folding_preserves_fixnum_overflow_operation() {
    let mut builder = FunctionBuilder::new("overflow");
    let max = builder.append(
        InstructionKind::Constant(Constant::Fixnum((1_i64 << 62) - 1)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    let result = builder.append(
        InstructionKind::Unary {
            op: UnaryOp::Inc,
            value: max,
        },
        Representation::FixnumTagged,
        Effects::MAY_THROW,
        None,
    );
    builder.terminate(Terminator::Return(result));
    let mut function = builder.finish();
    run_safe_pipeline(&mut function).unwrap();
    assert!(matches!(
        function.blocks[0].instructions[1].kind,
        InstructionKind::Unary {
            op: UnaryOp::Inc,
            ..
        }
    ));
}

#[test]
fn dce_keeps_unused_io_and_removes_unused_pure_values() {
    let mut builder = FunctionBuilder::new("effects");
    let unused = builder.append(
        InstructionKind::Constant(Constant::Fixnum(7)),
        Representation::FixnumTagged,
        Effects::none(),
        None,
    );
    builder.append_effect(
        InstructionKind::RuntimeCall {
            symbol: "cljn_print".into(),
            arguments: vec![unused],
        },
        Effects::MAY_PERFORM_IO,
        None,
    );
    let nil = builder.append(
        InstructionKind::Constant(Constant::Nil),
        Representation::NilImmediate,
        Effects::none(),
        None,
    );
    builder.terminate(Terminator::Return(nil));
    let mut function = builder.finish();
    run_safe_pipeline(&mut function).unwrap();
    assert!(function.blocks[0]
        .instructions
        .iter()
        .any(|instruction| matches!(instruction.kind, InstructionKind::RuntimeCall { .. })));
}

#[test]
fn root_planner_covers_heap_values_live_at_safepoints() {
    let mut builder = FunctionBuilder::new("roots");
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
    let mut function = builder.finish();
    run_safe_pipeline(&mut function).unwrap();
    assert!(function.root_slots.contains_key(&heap));
    verify_function(
        &function,
        VerifyOptions {
            require_root_plan: true,
        },
    )
    .unwrap();
}
