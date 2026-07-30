//! Deterministic construction helpers for valid-by-construction identifiers.

use crate::{
    Block, BlockId, BlockParam, Effects, Function, Instruction, InstructionKind, Representation,
    Terminator, ValueId,
};
use clojure_span::Span;
use std::collections::BTreeMap;

/// Incremental builder for one function CFG.
pub struct FunctionBuilder {
    name: String,
    entry: BlockId,
    blocks: Vec<Block>,
    current: BlockId,
    next_block: u32,
    next_value: u32,
    representations: BTreeMap<ValueId, Representation>,
}

impl FunctionBuilder {
    /// Creates a function with an empty entry block ending in `unreachable`.
    pub fn new(name: impl Into<String>) -> Self {
        let entry = BlockId(0);
        Self {
            name: name.into(),
            entry,
            blocks: vec![Block {
                id: entry,
                parameters: Vec::new(),
                instructions: Vec::new(),
                terminator: Terminator::Unreachable,
            }],
            current: entry,
            next_block: 1,
            next_value: 0,
            representations: BTreeMap::new(),
        }
    }

    /// Returns the entry block.
    pub fn entry(&self) -> BlockId {
        self.entry
    }

    /// Allocates a new block with the requested parameter representations.
    pub fn create_block(&mut self, parameters: &[Representation]) -> BlockId {
        let id = BlockId(self.next_block);
        self.next_block += 1;
        let parameters = parameters
            .iter()
            .map(|representation| {
                let value = self.allocate_value(*representation);
                BlockParam {
                    value,
                    representation: *representation,
                }
            })
            .collect();
        self.blocks.push(Block {
            id,
            parameters,
            instructions: Vec::new(),
            terminator: Terminator::Unreachable,
        });
        id
    }

    /// Selects the block that receives subsequent instructions.
    pub fn switch_to_block(&mut self, block: BlockId) {
        assert!(
            self.blocks.iter().any(|candidate| candidate.id == block),
            "unknown block"
        );
        self.current = block;
    }

    /// Appends a value-producing instruction.
    pub fn append(
        &mut self,
        kind: InstructionKind,
        representation: Representation,
        effects: Effects,
        span: Option<Span>,
    ) -> ValueId {
        let result = self.allocate_value(representation);
        self.current_block_mut().instructions.push(Instruction {
            result: Some(result),
            kind,
            representation: Some(representation),
            effects,
            span,
        });
        result
    }

    /// Appends a result-less effectful instruction.
    pub fn append_effect(&mut self, kind: InstructionKind, effects: Effects, span: Option<Span>) {
        self.current_block_mut().instructions.push(Instruction {
            result: None,
            kind,
            representation: None,
            effects,
            span,
        });
    }

    /// Replaces the current block terminator.
    pub fn terminate(&mut self, terminator: Terminator) {
        self.current_block_mut().terminator = terminator;
    }

    /// Completes construction.
    pub fn finish(self) -> Function {
        Function {
            name: self.name,
            entry: self.entry,
            blocks: self.blocks,
            representations: self.representations,
            root_slots: BTreeMap::new(),
        }
    }

    fn allocate_value(&mut self, representation: Representation) -> ValueId {
        let value = ValueId(self.next_value);
        self.next_value += 1;
        self.representations.insert(value, representation);
        value
    }

    fn current_block_mut(&mut self) -> &mut Block {
        self.blocks
            .iter_mut()
            .find(|block| block.id == self.current)
            .expect("current block exists")
    }
}
