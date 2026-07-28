//! Backward liveness and conservative fixed shadow-stack root planning.

use crate::{BlockId, Function, ValueId};
use std::collections::{BTreeMap, BTreeSet};

/// Block and instruction liveness sets for one verified function.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Liveness {
    /// Values live on entry to each block.
    pub live_in: BTreeMap<BlockId, BTreeSet<ValueId>>,
    /// Values live after each block.
    pub live_out: BTreeMap<BlockId, BTreeSet<ValueId>>,
    /// Values live immediately after `(block, instruction index)`.
    pub live_after_instruction: BTreeMap<(BlockId, usize), BTreeSet<ValueId>>,
}

/// Computes deterministic backward SSA liveness.
pub fn compute_liveness(function: &Function) -> Liveness {
    let mut uses = BTreeMap::<BlockId, BTreeSet<ValueId>>::new();
    let mut defs = BTreeMap::<BlockId, BTreeSet<ValueId>>::new();
    for block in &function.blocks {
        let mut block_defs: BTreeSet<ValueId> =
            block.parameters.iter().map(|param| param.value).collect();
        let mut block_uses = BTreeSet::new();
        for instruction in &block.instructions {
            for operand in instruction.kind.operands() {
                if !block_defs.contains(&operand) {
                    block_uses.insert(operand);
                }
            }
            if let Some(result) = instruction.result {
                block_defs.insert(result);
            }
        }
        for operand in block.terminator.operands() {
            if !block_defs.contains(&operand) {
                block_uses.insert(operand);
            }
        }
        uses.insert(block.id, block_uses);
        defs.insert(block.id, block_defs);
    }

    let mut live_in: BTreeMap<_, _> = function
        .blocks
        .iter()
        .map(|block| (block.id, BTreeSet::new()))
        .collect();
    let mut live_out = live_in.clone();
    loop {
        let mut changed = false;
        for block in function.blocks.iter().rev() {
            let mut out = BTreeSet::new();
            for successor in block.terminator.successors() {
                if let Some(values) = live_in.get(&successor) {
                    out.extend(values);
                }
            }
            let mut input = uses[&block.id].clone();
            input.extend(out.difference(&defs[&block.id]).copied());
            if live_out[&block.id] != out {
                live_out.insert(block.id, out);
                changed = true;
            }
            if live_in[&block.id] != input {
                live_in.insert(block.id, input);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let mut live_after_instruction = BTreeMap::new();
    for block in &function.blocks {
        let mut live = live_out[&block.id].clone();
        live.extend(block.terminator.operands());
        for (index, instruction) in block.instructions.iter().enumerate().rev() {
            live_after_instruction.insert((block.id, index), live.clone());
            if let Some(result) = instruction.result {
                live.remove(&result);
            }
            live.extend(instruction.kind.operands());
        }
    }
    Liveness {
        live_in,
        live_out,
        live_after_instruction,
    }
}

/// Assigns deterministic root slots to heap-capable values crossing safepoints.
///
/// GC: unknown tagged values are heap-capable. Slots are unique in this initial
/// implementation; later interval coloring may reuse them without changing the
/// public root-visibility invariant.
pub fn plan_roots(function: &mut Function) -> Liveness {
    let liveness = compute_liveness(function);
    let mut needs_root = BTreeSet::new();
    for block in &function.blocks {
        for (index, instruction) in block.instructions.iter().enumerate() {
            if !instruction.effects.may_safepoint() {
                continue;
            }
            let mut live = liveness
                .live_after_instruction
                .get(&(block.id, index))
                .cloned()
                .unwrap_or_default();
            live.extend(instruction.kind.operands());
            for value in live {
                if function
                    .representations
                    .get(&value)
                    .is_some_and(|representation| representation.is_heap_capable())
                {
                    needs_root.insert(value);
                }
            }
        }
    }
    function.root_slots = needs_root
        .into_iter()
        .enumerate()
        .map(|(slot, value)| (value, slot as u32))
        .collect();
    liveness
}
