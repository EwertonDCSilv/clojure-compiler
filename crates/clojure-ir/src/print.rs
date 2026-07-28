//! Deterministic textual IR printer used by structural tests and diagnostics.

use crate::{
    BinaryOp, Constant, Function, InstructionKind, Module, Representation, Terminator, UnaryOp,
};
use std::fmt::Write;

/// Prints a module without pointer addresses or host-specific paths.
pub fn print_module(module: &Module) -> String {
    let mut output = String::new();
    for function in &module.functions {
        print_function(&mut output, function);
    }
    output
}

fn print_function(output: &mut String, function: &Function) {
    let _ = writeln!(output, "fn {} {{", function.name);
    for block in &function.blocks {
        let _ = write!(output, "  b{}(", block.id.0);
        for (index, parameter) in block.parameters.iter().enumerate() {
            if index != 0 {
                output.push_str(", ");
            }
            let _ = write!(
                output,
                "v{}:{}",
                parameter.value.0,
                representation_name(parameter.representation)
            );
        }
        output.push_str("):\n");
        for instruction in &block.instructions {
            output.push_str("    ");
            if let Some(result) = instruction.result {
                let _ = write!(
                    output,
                    "v{}:{} = ",
                    result.0,
                    representation_name(instruction.representation.expect("result repr"))
                );
            }
            print_instruction(output, &instruction.kind);
            if !instruction.effects.is_pure() {
                let _ = write!(output, " effects=0x{:02x}", instruction.effects.bits());
            }
            if let Some(span) = instruction.span {
                let _ = write!(output, " @{}:{}..{}", span.source, span.start, span.end);
            }
            output.push('\n');
        }
        output.push_str("    ");
        print_terminator(output, &block.terminator);
        output.push('\n');
    }
    if !function.root_slots.is_empty() {
        output.push_str("  roots");
        for (value, slot) in &function.root_slots {
            let _ = write!(output, " v{}=r{}", value.0, slot);
        }
        output.push('\n');
    }
    output.push_str("}\n");
}

fn print_instruction(output: &mut String, instruction: &InstructionKind) {
    match instruction {
        InstructionKind::Constant(Constant::Fixnum(value)) => {
            let _ = write!(output, "const.fixnum {value}");
        }
        InstructionKind::Constant(Constant::Boolean(value)) => {
            let _ = write!(output, "const.bool {value}");
        }
        InstructionKind::Constant(Constant::Nil) => output.push_str("const.nil"),
        InstructionKind::Copy(value) => {
            let _ = write!(output, "copy v{}", value.0);
        }
        InstructionKind::Unary { op, value } => {
            let _ = write!(output, "{} v{}", unary_name(*op), value.0);
        }
        InstructionKind::Binary { op, left, right } => {
            let _ = write!(output, "{} v{}, v{}", binary_name(*op), left.0, right.0);
        }
        InstructionKind::Guard { kind, value } => {
            let _ = write!(output, "guard.{kind:?} v{}", value.0);
        }
        InstructionKind::RuntimeCall { symbol, arguments } => {
            let _ = write!(output, "call {symbol}(");
            for (index, value) in arguments.iter().enumerate() {
                if index != 0 {
                    output.push_str(", ");
                }
                let _ = write!(output, "v{}", value.0);
            }
            output.push(')');
        }
        InstructionKind::RootStore { value, slot } => {
            let _ = write!(output, "root.store r{slot}, v{}", value.0);
        }
    }
}

fn print_terminator(output: &mut String, terminator: &Terminator) {
    match terminator {
        Terminator::Branch { target, arguments } => {
            let _ = write!(output, "br b{}(", target.0);
            print_values(output, arguments);
            output.push(')');
        }
        Terminator::CondBranch {
            condition,
            then_target,
            then_arguments,
            else_target,
            else_arguments,
        } => {
            let _ = write!(output, "brif v{} b{}(", condition.0, then_target.0);
            print_values(output, then_arguments);
            let _ = write!(output, ") b{}(", else_target.0);
            print_values(output, else_arguments);
            output.push(')');
        }
        Terminator::Return(value) => {
            let _ = write!(output, "return v{}", value.0);
        }
        Terminator::Throw(value) => {
            let _ = write!(output, "throw v{}", value.0);
        }
        Terminator::Unreachable => output.push_str("unreachable"),
    }
}

fn print_values(output: &mut String, values: &[crate::ValueId]) {
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            output.push_str(", ");
        }
        let _ = write!(output, "v{}", value.0);
    }
}

fn representation_name(representation: Representation) -> &'static str {
    match representation {
        Representation::UnknownTagged => "tagged",
        Representation::FixnumTagged => "fixnum",
        Representation::FixnumUnboxed => "i63",
        Representation::BooleanImmediate => "bool",
        Representation::NilImmediate => "nil",
        Representation::HeapReference => "heap",
        Representation::CallableTagged => "callable",
    }
}

fn unary_name(operation: UnaryOp) -> &'static str {
    match operation {
        UnaryOp::Inc => "inc.checked",
        UnaryOp::Dec => "dec.checked",
        UnaryOp::Not => "not",
    }
}

fn binary_name(operation: BinaryOp) -> &'static str {
    match operation {
        BinaryOp::Add => "add.checked",
        BinaryOp::Sub => "sub.checked",
        BinaryOp::Mul => "mul.checked",
        BinaryOp::Equal => "eq",
        BinaryOp::LessThan => "lt",
        BinaryOp::LessThanOrEqual => "le",
        BinaryOp::GreaterThan => "gt",
        BinaryOp::GreaterThanOrEqual => "ge",
    }
}
