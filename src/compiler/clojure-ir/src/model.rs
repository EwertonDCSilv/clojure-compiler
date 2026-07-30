//! Core identifiers, effects, values, instructions, and CFG containers.

use clojure_span::Span;
use std::collections::BTreeMap;

/// Stable function-local basic-block identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlockId(pub u32);

/// Stable function-local single-definition value identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ValueId(pub u32);

/// Conservative runtime representation evidence attached to a value.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Representation {
    /// A tagged value whose more precise representation is unknown.
    UnknownTagged,
    /// A tagged fixnum.
    FixnumTagged,
    /// An unboxed signed fixnum payload, legal only inside a function.
    FixnumUnboxed,
    /// One of the immediate tagged booleans.
    BooleanImmediate,
    /// The immediate tagged `nil` value.
    NilImmediate,
    /// A managed heap reference.
    HeapReference,
    /// A tagged callable value.
    CallableTagged,
}

impl Representation {
    /// Returns whether the value may need a shadow-stack root at a safepoint.
    pub fn is_heap_capable(self) -> bool {
        matches!(
            self,
            Self::UnknownTagged | Self::HeapReference | Self::CallableTagged
        )
    }

    /// Computes the conservative representation accepted by a CFG merge.
    pub fn join(self, other: Self) -> Self {
        if self == other {
            self
        } else if matches!(self, Self::FixnumTagged | Self::FixnumUnboxed)
            && matches!(other, Self::FixnumTagged | Self::FixnumUnboxed)
        {
            Self::FixnumTagged
        } else {
            Self::UnknownTagged
        }
    }
}

/// Orthogonal instruction-effect flags.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Effects(u8);

impl Effects {
    /// Reads mutable runtime or dynamic state.
    pub const READS_STATE: Self = Self(1 << 0);
    /// Writes mutable runtime or dynamic state.
    pub const WRITES_STATE: Self = Self(1 << 1);
    /// May allocate a managed object.
    pub const MAY_ALLOCATE: Self = Self(1 << 2);
    /// May invoke the collector.
    pub const MAY_SAFEPOINT: Self = Self(1 << 3);
    /// May transfer control through the runtime exception path.
    pub const MAY_THROW: Self = Self(1 << 4);
    /// May perform externally observable I/O.
    pub const MAY_PERFORM_IO: Self = Self(1 << 5);

    /// No observable effects.
    pub const fn none() -> Self {
        Self(0)
    }

    /// Combines two effect sets.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns whether every flag is absent.
    pub const fn is_pure(self) -> bool {
        self.0 == 0
    }

    /// Returns whether this instruction can trigger a GC safepoint.
    pub const fn may_safepoint(self) -> bool {
        self.0 & Self::MAY_SAFEPOINT.0 != 0
    }

    /// Returns the stable raw flag bits for deterministic diagnostics.
    pub const fn bits(self) -> u8 {
        self.0
    }
}

/// Literal value represented directly in optimization IR.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Constant {
    /// Signed untagged fixnum payload.
    Fixnum(i64),
    /// Boolean immediate.
    Boolean(bool),
    /// Nil immediate.
    Nil,
}

impl Constant {
    /// Returns the representation implied by this literal.
    pub fn representation(&self) -> Representation {
        match self {
            Self::Fixnum(_) => Representation::FixnumTagged,
            Self::Boolean(_) => Representation::BooleanImmediate,
            Self::Nil => Representation::NilImmediate,
        }
    }
}

/// Pure unary scalar operations accepted by the initial optimizer.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnaryOp {
    /// Checked fixnum increment.
    Inc,
    /// Checked fixnum decrement.
    Dec,
    /// Clojure truthiness negation.
    Not,
}

/// Pure scalar binary operations accepted by the initial optimizer.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BinaryOp {
    /// Checked fixnum addition.
    Add,
    /// Checked fixnum subtraction.
    Sub,
    /// Checked fixnum multiplication.
    Mul,
    /// Structural equality for immediate constants.
    Equal,
    /// Signed fixnum comparison.
    LessThan,
    /// Signed fixnum comparison.
    LessThanOrEqual,
    /// Signed fixnum comparison.
    GreaterThan,
    /// Signed fixnum comparison.
    GreaterThanOrEqual,
}

/// Runtime guard kinds whose slow path remains observable.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GuardKind {
    /// Require a tagged fixnum.
    IsFixnum,
    /// Require a callable tagged value.
    IsCallable,
}

/// Operation payload independent of result, representation, effects, and span.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InstructionKind {
    /// Produce a literal.
    Constant(Constant),
    /// Copy a value.
    Copy(ValueId),
    /// Apply a unary scalar operation.
    Unary {
        /// Selected operation.
        op: UnaryOp,
        /// Operand.
        value: ValueId,
    },
    /// Apply a binary scalar operation.
    Binary {
        /// Selected operation.
        op: BinaryOp,
        /// Left operand.
        left: ValueId,
        /// Right operand.
        right: ValueId,
    },
    /// Check a runtime representation before a preserved slow path.
    Guard {
        /// Guard condition.
        kind: GuardKind,
        /// Value being checked.
        value: ValueId,
    },
    /// Call a runtime ABI symbol with explicitly declared effects.
    RuntimeCall {
        /// Stable imported symbol.
        symbol: String,
        /// Eager argument values.
        arguments: Vec<ValueId>,
    },
    /// Store a heap-capable live value in an assigned root slot.
    RootStore {
        /// Value made visible to the collector.
        value: ValueId,
        /// Fixed function-local root slot.
        slot: u32,
    },
}

impl InstructionKind {
    /// Visits every operand in deterministic source order.
    pub fn operands(&self) -> Vec<ValueId> {
        match self {
            Self::Constant(_) => Vec::new(),
            Self::Copy(value) => vec![*value],
            Self::Unary { value, .. } | Self::Guard { value, .. } => vec![*value],
            Self::Binary { left, right, .. } => vec![*left, *right],
            Self::RuntimeCall { arguments, .. } => arguments.clone(),
            Self::RootStore { value, .. } => vec![*value],
        }
    }

    /// Rewrites operands according to a canonical replacement map.
    pub fn rewrite_operands(&mut self, replacements: &BTreeMap<ValueId, ValueId>) {
        let resolve = |mut value: ValueId| {
            while let Some(next) = replacements.get(&value) {
                if *next == value {
                    break;
                }
                value = *next;
            }
            value
        };
        match self {
            Self::Constant(_) => {}
            Self::Copy(value) | Self::Unary { value, .. } | Self::Guard { value, .. } => {
                *value = resolve(*value);
            }
            Self::Binary { left, right, .. } => {
                *left = resolve(*left);
                *right = resolve(*right);
            }
            Self::RuntimeCall { arguments, .. } => {
                for value in arguments {
                    *value = resolve(*value);
                }
            }
            Self::RootStore { value, .. } => *value = resolve(*value),
        }
    }
}

/// One ordered instruction in a basic block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instruction {
    /// Optional produced SSA value.
    pub result: Option<ValueId>,
    /// Operation payload.
    pub kind: InstructionKind,
    /// Result representation, or `None` for result-less operations.
    pub representation: Option<Representation>,
    /// Explicit orthogonal effects.
    pub effects: Effects,
    /// Source span when failure or result is source-observable.
    pub span: Option<Span>,
}

/// Value accepted by a basic block from every predecessor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlockParam {
    /// SSA value defined at block entry.
    pub value: ValueId,
    /// Required merged representation.
    pub representation: Representation,
}

/// Explicit basic-block terminator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Terminator {
    /// Transfer to one successor with block arguments.
    Branch {
        /// Successor block.
        target: BlockId,
        /// Values passed to successor parameters.
        arguments: Vec<ValueId>,
    },
    /// Conditional transfer with explicit arguments on both edges.
    CondBranch {
        /// Immediate or tagged condition.
        condition: ValueId,
        /// True successor.
        then_target: BlockId,
        /// Values passed on the true edge.
        then_arguments: Vec<ValueId>,
        /// False successor.
        else_target: BlockId,
        /// Values passed on the false edge.
        else_arguments: Vec<ValueId>,
    },
    /// Return a tagged ABI value.
    Return(ValueId),
    /// Throw a tagged value through the native exception mechanism.
    Throw(ValueId),
    /// Mark a path that cannot continue.
    Unreachable,
}

impl Terminator {
    /// Returns successor blocks in deterministic edge order.
    pub fn successors(&self) -> Vec<BlockId> {
        match self {
            Self::Branch { target, .. } => vec![*target],
            Self::CondBranch {
                then_target,
                else_target,
                ..
            } => vec![*then_target, *else_target],
            Self::Return(_) | Self::Throw(_) | Self::Unreachable => Vec::new(),
        }
    }

    /// Returns every value used by this terminator.
    pub fn operands(&self) -> Vec<ValueId> {
        match self {
            Self::Branch { arguments, .. } => arguments.clone(),
            Self::CondBranch {
                condition,
                then_arguments,
                else_arguments,
                ..
            } => {
                let mut values = vec![*condition];
                values.extend(then_arguments);
                values.extend(else_arguments);
                values
            }
            Self::Return(value) | Self::Throw(value) => vec![*value],
            Self::Unreachable => Vec::new(),
        }
    }

    /// Rewrites every operand using canonical replacements.
    pub fn rewrite_operands(&mut self, replacements: &BTreeMap<ValueId, ValueId>) {
        let resolve = |mut value: ValueId| {
            while let Some(next) = replacements.get(&value) {
                if *next == value {
                    break;
                }
                value = *next;
            }
            value
        };
        match self {
            Self::Branch { arguments, .. } => {
                for value in arguments {
                    *value = resolve(*value);
                }
            }
            Self::CondBranch {
                condition,
                then_arguments,
                else_arguments,
                ..
            } => {
                *condition = resolve(*condition);
                for value in then_arguments.iter_mut().chain(else_arguments) {
                    *value = resolve(*value);
                }
            }
            Self::Return(value) | Self::Throw(value) => *value = resolve(*value),
            Self::Unreachable => {}
        }
    }
}

/// One basic block with no implicit fallthrough.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    /// Stable block identifier.
    pub id: BlockId,
    /// Values defined at block entry.
    pub parameters: Vec<BlockParam>,
    /// Ordered operations.
    pub instructions: Vec<Instruction>,
    /// Required explicit terminator.
    pub terminator: Terminator,
}

/// One backend-neutral function CFG.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    /// Stable linkage/debug name.
    pub name: String,
    /// Function entry block.
    pub entry: BlockId,
    /// Blocks in deterministic creation order.
    pub blocks: Vec<Block>,
    /// Representation of every defined value.
    pub representations: BTreeMap<ValueId, Representation>,
    /// Fixed root slot assigned to heap-capable values live across safepoints.
    pub root_slots: BTreeMap<ValueId, u32>,
}

impl Function {
    /// Returns a block by ID.
    pub fn block(&self, id: BlockId) -> Option<&Block> {
        self.blocks.iter().find(|block| block.id == id)
    }

    /// Returns a mutable block by ID.
    pub fn block_mut(&mut self, id: BlockId) -> Option<&mut Block> {
        self.blocks.iter_mut().find(|block| block.id == id)
    }
}

/// Complete optimization module.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Module {
    /// Functions in deterministic analyzer order.
    pub functions: Vec<Function>,
}
