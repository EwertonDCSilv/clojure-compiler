//! Structural, dominance, representation, and GC-root verification.

use crate::{compute_liveness, BlockId, Function, Representation, Terminator, ValueId};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Controls stage-specific verification requirements.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VerifyOptions {
    /// Require a complete root plan for every safepoint.
    pub require_root_plan: bool,
}

/// Stable verifier failure returned as a compiler diagnostic by callers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifyError {
    message: String,
}

impl VerifyError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for VerifyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for VerifyError {}

/// Verifies one function without panicking on malformed user-produced IR.
pub fn verify_function(function: &Function, options: VerifyOptions) -> Result<(), VerifyError> {
    let blocks: BTreeMap<BlockId, _> = function
        .blocks
        .iter()
        .map(|block| (block.id, block))
        .collect();
    if blocks.len() != function.blocks.len() {
        return Err(VerifyError::new("bloco definido mais de uma vez"));
    }
    if !blocks.contains_key(&function.entry) {
        return Err(VerifyError::new("bloco de entrada inexistente"));
    }

    let mut definitions = BTreeMap::<ValueId, (BlockId, Option<usize>)>::new();
    for block in &function.blocks {
        for param in &block.parameters {
            if definitions.insert(param.value, (block.id, None)).is_some() {
                return Err(VerifyError::new(format!(
                    "valor v{} definido mais de uma vez",
                    param.value.0
                )));
            }
            require_representation(function, param.value, param.representation)?;
        }
        for (index, instruction) in block.instructions.iter().enumerate() {
            if instruction.result.is_some() != instruction.representation.is_some() {
                return Err(VerifyError::new(format!(
                    "resultado e representação divergentes em b{} i{}",
                    block.id.0, index
                )));
            }
            if let Some(result) = instruction.result {
                if definitions
                    .insert(result, (block.id, Some(index)))
                    .is_some()
                {
                    return Err(VerifyError::new(format!(
                        "valor v{} definido mais de uma vez",
                        result.0
                    )));
                }
                require_representation(
                    function,
                    result,
                    instruction.representation.expect("checked above"),
                )?;
            }
            if let crate::InstructionKind::RuntimeCall { symbol, .. } = &instruction.kind {
                if symbol.is_empty() {
                    return Err(VerifyError::new("runtime call sem símbolo"));
                }
            }
        }
    }
    if definitions.len() != function.representations.len() {
        return Err(VerifyError::new(
            "tabela de representações contém valor sem definição",
        ));
    }

    let reachable = reachable_blocks(function, &blocks)?;
    let predecessors = predecessors(function, &blocks)?;
    let dominators = dominators(function.entry, &reachable, &predecessors);
    for block in &function.blocks {
        for (index, instruction) in block.instructions.iter().enumerate() {
            for operand in instruction.kind.operands() {
                verify_use(operand, block.id, Some(index), &definitions, &dominators)?;
            }
        }
        for operand in block.terminator.operands() {
            verify_use(operand, block.id, None, &definitions, &dominators)?;
        }
        verify_edges(function, block.id, &block.terminator, &blocks)?;
        match &block.terminator {
            Terminator::Return(value) | Terminator::Throw(value)
                if function.representations[value] == Representation::FixnumUnboxed =>
            {
                return Err(VerifyError::new(format!(
                    "v{} cruza ABI tagged ainda unboxed",
                    value.0
                )));
            }
            _ => {}
        }
    }

    if options.require_root_plan {
        verify_roots(function)?;
    }
    Ok(())
}

fn require_representation(
    function: &Function,
    value: ValueId,
    expected: Representation,
) -> Result<(), VerifyError> {
    match function.representations.get(&value) {
        Some(actual) if *actual == expected => Ok(()),
        Some(actual) => Err(VerifyError::new(format!(
            "representação de v{} diverge: {:?} != {:?}",
            value.0, actual, expected
        ))),
        None => Err(VerifyError::new(format!(
            "v{} não possui representação",
            value.0
        ))),
    }
}

fn reachable_blocks(
    function: &Function,
    blocks: &BTreeMap<BlockId, &crate::Block>,
) -> Result<BTreeSet<BlockId>, VerifyError> {
    let mut reachable = BTreeSet::new();
    let mut pending = vec![function.entry];
    while let Some(block) = pending.pop() {
        if !reachable.insert(block) {
            continue;
        }
        let Some(body) = blocks.get(&block) else {
            return Err(VerifyError::new(format!("referência a bloco b{}", block.0)));
        };
        pending.extend(body.terminator.successors());
    }
    Ok(reachable)
}

fn predecessors(
    function: &Function,
    blocks: &BTreeMap<BlockId, &crate::Block>,
) -> Result<BTreeMap<BlockId, BTreeSet<BlockId>>, VerifyError> {
    let mut predecessors: BTreeMap<_, _> = blocks
        .keys()
        .map(|block| (*block, BTreeSet::new()))
        .collect();
    for block in &function.blocks {
        for successor in block.terminator.successors() {
            let Some(entries) = predecessors.get_mut(&successor) else {
                return Err(VerifyError::new(format!(
                    "b{} salta para bloco inexistente b{}",
                    block.id.0, successor.0
                )));
            };
            entries.insert(block.id);
        }
    }
    Ok(predecessors)
}

fn dominators(
    entry: BlockId,
    reachable: &BTreeSet<BlockId>,
    predecessors: &BTreeMap<BlockId, BTreeSet<BlockId>>,
) -> BTreeMap<BlockId, BTreeSet<BlockId>> {
    let mut result = BTreeMap::new();
    for block in reachable {
        result.insert(
            *block,
            if *block == entry {
                BTreeSet::from([entry])
            } else {
                reachable.clone()
            },
        );
    }
    loop {
        let mut changed = false;
        for block in reachable.iter().copied().filter(|block| *block != entry) {
            let incoming: Vec<_> = predecessors[&block]
                .iter()
                .filter(|pred| reachable.contains(pred))
                .collect();
            let mut next = if let Some(first) = incoming.first() {
                result[first].clone()
            } else {
                BTreeSet::new()
            };
            for predecessor in incoming.iter().skip(1) {
                next = next.intersection(&result[predecessor]).copied().collect();
            }
            next.insert(block);
            if result[&block] != next {
                result.insert(block, next);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    result
}

fn verify_use(
    value: ValueId,
    use_block: BlockId,
    use_index: Option<usize>,
    definitions: &BTreeMap<ValueId, (BlockId, Option<usize>)>,
    dominators: &BTreeMap<BlockId, BTreeSet<BlockId>>,
) -> Result<(), VerifyError> {
    let Some((def_block, def_index)) = definitions.get(&value).copied() else {
        return Err(VerifyError::new(format!(
            "uso de v{} sem definição",
            value.0
        )));
    };
    if def_block == use_block {
        if let (Some(def), Some(used)) = (def_index, use_index) {
            if def >= used {
                return Err(VerifyError::new(format!(
                    "uso de v{} antes da definição",
                    value.0
                )));
            }
        }
        return Ok(());
    }
    if !dominators
        .get(&use_block)
        .is_some_and(|set| set.contains(&def_block))
    {
        return Err(VerifyError::new(format!(
            "definição de v{} não domina uso em b{}",
            value.0, use_block.0
        )));
    }
    Ok(())
}

fn verify_edges(
    function: &Function,
    source: BlockId,
    terminator: &Terminator,
    blocks: &BTreeMap<BlockId, &crate::Block>,
) -> Result<(), VerifyError> {
    let edges: Vec<(BlockId, &[ValueId])> = match terminator {
        Terminator::Branch { target, arguments } => vec![(*target, arguments)],
        Terminator::CondBranch {
            then_target,
            then_arguments,
            else_target,
            else_arguments,
            ..
        } => vec![
            (*then_target, then_arguments),
            (*else_target, else_arguments),
        ],
        Terminator::Return(_) | Terminator::Throw(_) | Terminator::Unreachable => Vec::new(),
    };
    for (target, arguments) in edges {
        let Some(block) = blocks.get(&target) else {
            return Err(VerifyError::new(format!(
                "b{} salta para bloco inexistente b{}",
                source.0, target.0
            )));
        };
        if arguments.len() != block.parameters.len() {
            return Err(VerifyError::new(format!(
                "aresta b{} -> b{} passa {} valores para {} parâmetros",
                source.0,
                target.0,
                arguments.len(),
                block.parameters.len()
            )));
        }
        for (argument, parameter) in arguments.iter().zip(&block.parameters) {
            let actual = function.representations[argument];
            let compatible = actual == parameter.representation
                || (parameter.representation == Representation::UnknownTagged
                    && actual != Representation::FixnumUnboxed);
            if !compatible {
                return Err(VerifyError::new(format!(
                    "aresta b{} -> b{} mistura {:?} com {:?}",
                    source.0, target.0, actual, parameter.representation
                )));
            }
        }
    }
    Ok(())
}

fn verify_roots(function: &Function) -> Result<(), VerifyError> {
    let liveness = compute_liveness(function);
    let mut used_slots = BTreeMap::<u32, ValueId>::new();
    for (value, slot) in &function.root_slots {
        if let Some(previous) = used_slots.insert(*slot, *value) {
            return Err(VerifyError::new(format!(
                "root slot {} duplicado para v{} e v{}",
                slot, previous.0, value.0
            )));
        }
        if !function
            .representations
            .get(value)
            .is_some_and(|representation| representation.is_heap_capable())
        {
            return Err(VerifyError::new(format!(
                "root slot atribuído a v{} imediato",
                value.0
            )));
        }
    }
    for block in &function.blocks {
        for (index, instruction) in block.instructions.iter().enumerate() {
            if !instruction.effects.may_safepoint() {
                continue;
            }
            let mut live = liveness.live_after_instruction[&(block.id, index)].clone();
            live.extend(instruction.kind.operands());
            for value in live {
                if function.representations[&value].is_heap_capable()
                    && !function.root_slots.contains_key(&value)
                {
                    return Err(VerifyError::new(format!(
                        "v{} vivo no safepoint b{} i{} sem root",
                        value.0, block.id.0, index
                    )));
                }
            }
        }
    }
    Ok(())
}
