//! Deterministic conservative passes admitted to the initial local profile.

use crate::{
    plan_roots, verify_function, BinaryOp, Constant, Function, InstructionKind, Representation,
    Terminator, UnaryOp, ValueId, VerifyOptions,
};
use std::collections::{BTreeMap, BTreeSet, HashMap};

const FIXNUM_MIN: i64 = -(1_i64 << 62);
const FIXNUM_MAX: i64 = (1_i64 << 62) - 1;

/// Structural changes made by the safe local pipeline.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PassReport {
    /// Names of passes that changed the function, in execution order.
    pub changed_passes: Vec<&'static str>,
    /// Total removed instructions.
    pub removed_instructions: usize,
    /// Total removed unreachable blocks.
    pub removed_blocks: usize,
}

/// Runs verified local optimizations and plans roots after transformations.
///
/// The initial profile deliberately excludes LICM and guard elimination until
/// their independent semantic and Cormen gates are available.
pub fn run_safe_pipeline(function: &mut Function) -> Result<PassReport, crate::VerifyError> {
    verify_function(function, VerifyOptions::default())?;
    let mut report = PassReport::default();

    let removed_blocks = remove_unreachable_blocks(function);
    if removed_blocks != 0 {
        report.changed_passes.push("simplify-cfg");
        report.removed_blocks += removed_blocks;
        verify_function(function, VerifyOptions::default())?;
    }

    if fold_constants(function) {
        report.changed_passes.push("checked-constant-folding");
        verify_function(function, VerifyOptions::default())?;
    }

    let copies = propagate_copies(function);
    if copies != 0 {
        report.changed_passes.push("copy-propagation");
        report.removed_instructions += copies;
        verify_function(function, VerifyOptions::default())?;
    }

    let common = eliminate_common_subexpressions(function);
    if common != 0 {
        report.changed_passes.push("local-cse");
        report.removed_instructions += common;
        verify_function(function, VerifyOptions::default())?;
    }

    if simplify_constant_branches(function) {
        report.changed_passes.push("branch-simplification");
        let additionally_removed = remove_unreachable_blocks(function);
        report.removed_blocks += additionally_removed;
        verify_function(function, VerifyOptions::default())?;
    }

    let dead = eliminate_dead_code(function);
    if dead != 0 {
        report.changed_passes.push("effect-aware-dce");
        report.removed_instructions += dead;
        verify_function(function, VerifyOptions::default())?;
    }

    plan_roots(function);
    verify_function(
        function,
        VerifyOptions {
            require_root_plan: true,
        },
    )?;
    Ok(report)
}

fn remove_unreachable_blocks(function: &mut Function) -> usize {
    let mut reachable = BTreeSet::new();
    let mut pending = vec![function.entry];
    while let Some(block) = pending.pop() {
        if !reachable.insert(block) {
            continue;
        }
        if let Some(body) = function.block(block) {
            pending.extend(body.terminator.successors());
        }
    }
    let before = function.blocks.len();
    function
        .blocks
        .retain(|block| reachable.contains(&block.id));
    let defined: BTreeSet<_> = function
        .blocks
        .iter()
        .flat_map(|block| {
            block
                .parameters
                .iter()
                .map(|param| param.value)
                .chain(block.instructions.iter().filter_map(|inst| inst.result))
        })
        .collect();
    function
        .representations
        .retain(|value, _| defined.contains(value));
    function
        .root_slots
        .retain(|value, _| defined.contains(value));
    before - function.blocks.len()
}

fn fold_constants(function: &mut Function) -> bool {
    let mut constants = BTreeMap::<ValueId, Constant>::new();
    let mut changed = false;
    for block in &mut function.blocks {
        for instruction in &mut block.instructions {
            let folded = match &instruction.kind {
                InstructionKind::Constant(value) => {
                    if let Some(result) = instruction.result {
                        constants.insert(result, value.clone());
                    }
                    None
                }
                InstructionKind::Copy(value) => constants.get(value).cloned(),
                InstructionKind::Unary { op, value } => constants
                    .get(value)
                    .and_then(|value| fold_unary(*op, value)),
                InstructionKind::Binary { op, left, right } => constants
                    .get(left)
                    .zip(constants.get(right))
                    .and_then(|(left, right)| fold_binary(*op, left, right)),
                _ => None,
            };
            if let (Some(result), Some(constant)) = (instruction.result, folded) {
                instruction.kind = InstructionKind::Constant(constant.clone());
                instruction.representation = Some(constant.representation());
                instruction.effects = crate::Effects::none();
                function
                    .representations
                    .insert(result, constant.representation());
                constants.insert(result, constant);
                changed = true;
            }
        }
    }
    changed
}

fn fold_unary(operation: UnaryOp, value: &Constant) -> Option<Constant> {
    match (operation, value) {
        (UnaryOp::Inc, Constant::Fixnum(value)) if *value < FIXNUM_MAX => {
            Some(Constant::Fixnum(value + 1))
        }
        (UnaryOp::Dec, Constant::Fixnum(value)) if *value > FIXNUM_MIN => {
            Some(Constant::Fixnum(value - 1))
        }
        (UnaryOp::Not, Constant::Boolean(value)) => Some(Constant::Boolean(!value)),
        (UnaryOp::Not, Constant::Nil) => Some(Constant::Boolean(true)),
        (UnaryOp::Not, Constant::Fixnum(_)) => Some(Constant::Boolean(false)),
        _ => None,
    }
}

fn fold_binary(operation: BinaryOp, left: &Constant, right: &Constant) -> Option<Constant> {
    let checked_fixnum = |value: i128| {
        if (FIXNUM_MIN as i128..=FIXNUM_MAX as i128).contains(&value) {
            Some(Constant::Fixnum(value as i64))
        } else {
            None
        }
    };
    match (operation, left, right) {
        (BinaryOp::Add, Constant::Fixnum(left), Constant::Fixnum(right)) => {
            checked_fixnum(*left as i128 + *right as i128)
        }
        (BinaryOp::Sub, Constant::Fixnum(left), Constant::Fixnum(right)) => {
            checked_fixnum(*left as i128 - *right as i128)
        }
        (BinaryOp::Mul, Constant::Fixnum(left), Constant::Fixnum(right)) => {
            checked_fixnum(*left as i128 * *right as i128)
        }
        (BinaryOp::Equal, left, right) => Some(Constant::Boolean(left == right)),
        (BinaryOp::LessThan, Constant::Fixnum(left), Constant::Fixnum(right)) => {
            Some(Constant::Boolean(left < right))
        }
        (BinaryOp::LessThanOrEqual, Constant::Fixnum(left), Constant::Fixnum(right)) => {
            Some(Constant::Boolean(left <= right))
        }
        (BinaryOp::GreaterThan, Constant::Fixnum(left), Constant::Fixnum(right)) => {
            Some(Constant::Boolean(left > right))
        }
        (BinaryOp::GreaterThanOrEqual, Constant::Fixnum(left), Constant::Fixnum(right)) => {
            Some(Constant::Boolean(left >= right))
        }
        _ => None,
    }
}

fn propagate_copies(function: &mut Function) -> usize {
    let mut replacements = BTreeMap::new();
    for block in &function.blocks {
        for instruction in &block.instructions {
            if let (Some(result), InstructionKind::Copy(value)) =
                (instruction.result, &instruction.kind)
            {
                replacements.insert(result, *value);
            }
        }
    }
    if replacements.is_empty() {
        return 0;
    }
    rewrite_uses(function, &replacements);
    let mut removed = 0;
    for block in &mut function.blocks {
        block.instructions.retain(|instruction| {
            let remove = matches!(instruction.kind, InstructionKind::Copy(_))
                && instruction.effects.is_pure();
            removed += remove as usize;
            !remove
        });
    }
    for value in replacements.keys() {
        function.representations.remove(value);
        function.root_slots.remove(value);
    }
    removed
}

fn eliminate_common_subexpressions(function: &mut Function) -> usize {
    let mut replacements = BTreeMap::new();
    for block in &function.blocks {
        let mut available = HashMap::<(InstructionKind, Option<Representation>), ValueId>::new();
        for instruction in &block.instructions {
            let Some(result) = instruction.result else {
                continue;
            };
            if !instruction.effects.is_pure()
                || matches!(
                    instruction.kind,
                    InstructionKind::Constant(_) | InstructionKind::RootStore { .. }
                )
            {
                continue;
            }
            let key = (instruction.kind.clone(), instruction.representation);
            if let Some(previous) = available.get(&key) {
                replacements.insert(result, *previous);
            } else {
                available.insert(key, result);
            }
        }
    }
    if replacements.is_empty() {
        return 0;
    }
    rewrite_uses(function, &replacements);
    let mut removed = 0;
    for block in &mut function.blocks {
        block.instructions.retain(|instruction| {
            let remove = instruction
                .result
                .is_some_and(|result| replacements.contains_key(&result));
            removed += remove as usize;
            !remove
        });
    }
    for value in replacements.keys() {
        function.representations.remove(value);
        function.root_slots.remove(value);
    }
    removed
}

fn simplify_constant_branches(function: &mut Function) -> bool {
    let constants: BTreeMap<_, _> = function
        .blocks
        .iter()
        .flat_map(|block| &block.instructions)
        .filter_map(
            |instruction| match (instruction.result, &instruction.kind) {
                (Some(result), InstructionKind::Constant(constant)) => {
                    Some((result, constant.clone()))
                }
                _ => None,
            },
        )
        .collect();
    let mut changed = false;
    for block in &mut function.blocks {
        let replacement = match &block.terminator {
            Terminator::CondBranch {
                condition,
                then_target,
                then_arguments,
                else_target,
                else_arguments,
            } => match constants.get(condition) {
                Some(Constant::Boolean(false) | Constant::Nil) => Some(Terminator::Branch {
                    target: *else_target,
                    arguments: else_arguments.clone(),
                }),
                Some(Constant::Boolean(true) | Constant::Fixnum(_)) => Some(Terminator::Branch {
                    target: *then_target,
                    arguments: then_arguments.clone(),
                }),
                None => None,
            },
            _ => None,
        };
        if let Some(replacement) = replacement {
            block.terminator = replacement;
            changed = true;
        }
    }
    changed
}

fn eliminate_dead_code(function: &mut Function) -> usize {
    let mut used = BTreeSet::new();
    for block in &function.blocks {
        used.extend(block.terminator.operands());
        for instruction in &block.instructions {
            used.extend(instruction.kind.operands());
        }
    }
    let mut removed = 0;
    loop {
        let mut changed = false;
        for block in &mut function.blocks {
            block.instructions.retain(|instruction| {
                let remove = instruction.effects.is_pure()
                    && instruction
                        .result
                        .is_some_and(|result| !used.contains(&result));
                if remove {
                    if let Some(result) = instruction.result {
                        function.representations.remove(&result);
                        function.root_slots.remove(&result);
                    }
                    removed += 1;
                    changed = true;
                }
                !remove
            });
        }
        if !changed {
            break;
        }
        used.clear();
        for block in &function.blocks {
            used.extend(block.terminator.operands());
            for instruction in &block.instructions {
                used.extend(instruction.kind.operands());
            }
        }
    }
    removed
}

fn rewrite_uses(function: &mut Function, replacements: &BTreeMap<ValueId, ValueId>) {
    for block in &mut function.blocks {
        for instruction in &mut block.instructions {
            instruction.kind.rewrite_operands(replacements);
        }
        block.terminator.rewrite_operands(replacements);
    }
}
