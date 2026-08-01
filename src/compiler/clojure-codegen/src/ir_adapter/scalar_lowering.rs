//! Pure scalar constant folding through `clojure-ir`: lowers a self-contained
//! scalar expression (fixnum/boolean/nil literals and arithmetic/comparison
//! primitives over them) into a one-block IR function, runs the verified
//! local optimization profile, and reads back a proven constant when the
//! whole expression folded to one. Any expression `lower_scalar` cannot
//! represent is left untouched by the caller.

use clojure_analyzer::{Ast, Callee, Prim};
use clojure_diagnostics::Diagnostic;
use clojure_ir::{
    run_safe_pipeline, BinaryOp, Constant, Effects, FunctionBuilder, InstructionKind,
    Representation, Terminator, UnaryOp, ValueId,
};

pub(crate) fn optimize_ast(ast: Ast) -> Result<Ast, Diagnostic> {
    let rebuilt = match ast {
        Ast::VecLit(items) => Ast::VecLit(optimize_many(items)?),
        Ast::SetLit(items) => Ast::SetLit(optimize_many(items)?),
        Ast::MapLit(pairs) => Ast::MapLit(
            pairs
                .into_iter()
                .map(|(key, value)| Ok((optimize_ast(key)?, optimize_ast(value)?)))
                .collect::<Result<_, Diagnostic>>()?,
        ),
        Ast::DefGlobal { index, value } => Ast::DefGlobal {
            index,
            value: Box::new(optimize_ast(*value)?),
        },
        Ast::MakeFn {
            lambda,
            arity,
            captures,
        } => Ast::MakeFn {
            lambda,
            arity,
            captures: optimize_many(captures)?,
        },
        Ast::If(test, then, otherwise) => {
            let test = optimize_ast(*test)?;
            let then = optimize_ast(*then)?;
            let otherwise = optimize_ast(*otherwise)?;
            match test {
                Ast::Bool(true) | Ast::Int(_) => then,
                Ast::Bool(false) | Ast::Nil => otherwise,
                test => Ast::If(Box::new(test), Box::new(then), Box::new(otherwise)),
            }
        }
        Ast::Do(expressions) => Ast::Do(optimize_many(expressions)?),
        Ast::Let { slots, body } => Ast::Let {
            slots: slots
                .into_iter()
                .map(|(slot, value)| Ok((slot, optimize_ast(value)?)))
                .collect::<Result<_, Diagnostic>>()?,
            body: Box::new(optimize_ast(*body)?),
        },
        Ast::Loop { slots, body } => Ast::Loop {
            slots: slots
                .into_iter()
                .map(|(slot, value)| Ok((slot, optimize_ast(value)?)))
                .collect::<Result<_, Diagnostic>>()?,
            body: Box::new(optimize_ast(*body)?),
        },
        Ast::Recur(arguments) => Ast::Recur(optimize_many(arguments)?),
        Ast::Call { callee, args } => Ast::Call {
            callee,
            args: optimize_many(args)?,
        },
        Ast::CallValue { f, args } => Ast::CallValue {
            f: Box::new(optimize_ast(*f)?),
            args: optimize_many(args)?,
        },
        Ast::Apply { f, fixed, coll } => Ast::Apply {
            f: Box::new(optimize_ast(*f)?),
            fixed: optimize_many(fixed)?,
            coll: Box::new(optimize_ast(*coll)?),
        },
        Ast::MakeRecord { type_name, fields } => Ast::MakeRecord {
            type_name,
            fields: fields
                .into_iter()
                .map(|(name, value)| Ok((name, optimize_ast(value)?)))
                .collect::<Result<_, Diagnostic>>()?,
        },
        Ast::RegisterMethod {
            method_id,
            key,
            impl_fn,
        } => Ast::RegisterMethod {
            method_id,
            key: Box::new(optimize_ast(*key)?),
            impl_fn: Box::new(optimize_ast(*impl_fn)?),
        },
        Ast::RegisterMulti {
            method_id,
            dispatch_fn,
        } => Ast::RegisterMulti {
            method_id,
            dispatch_fn: Box::new(optimize_ast(*dispatch_fn)?),
        },
        scalar => scalar,
    };
    optimize_scalar(rebuilt)
}

fn optimize_many(expressions: Vec<Ast>) -> Result<Vec<Ast>, Diagnostic> {
    expressions.into_iter().map(optimize_ast).collect()
}

fn optimize_scalar(ast: Ast) -> Result<Ast, Diagnostic> {
    let mut builder = FunctionBuilder::new("scalar");
    let Some(result) = lower_scalar(&ast, &mut builder) else {
        return Ok(ast);
    };
    builder.terminate(Terminator::Return(result));
    let mut function = builder.finish();
    run_safe_pipeline(&mut function).map_err(|error| {
        Diagnostic::error("E0120", format!("IR de otimização inválida: {error}"))
    })?;
    let Terminator::Return(result) = function.blocks[0].terminator else {
        return Ok(ast);
    };
    let constant = function.blocks[0]
        .instructions
        .iter()
        .find_map(
            |instruction| match (instruction.result, &instruction.kind) {
                (Some(value), InstructionKind::Constant(constant)) if value == result => {
                    Some(constant)
                }
                _ => None,
            },
        );
    Ok(match constant {
        Some(Constant::Fixnum(value)) => Ast::Int(*value),
        Some(Constant::Boolean(value)) => Ast::Bool(*value),
        Some(Constant::Nil) => Ast::Nil,
        None => ast,
    })
}

fn lower_scalar(ast: &Ast, builder: &mut FunctionBuilder) -> Option<ValueId> {
    match ast {
        Ast::Int(value) => Some(builder.append(
            InstructionKind::Constant(Constant::Fixnum(*value)),
            Representation::FixnumTagged,
            Effects::none(),
            None,
        )),
        Ast::Bool(value) => Some(builder.append(
            InstructionKind::Constant(Constant::Boolean(*value)),
            Representation::BooleanImmediate,
            Effects::none(),
            None,
        )),
        Ast::Nil => Some(builder.append(
            InstructionKind::Constant(Constant::Nil),
            Representation::NilImmediate,
            Effects::none(),
            None,
        )),
        Ast::Call {
            callee: Callee::Prim(primitive) | Callee::ProvenFixnumPrim(primitive),
            args,
        } => lower_primitive(*primitive, args, builder),
        _ => None,
    }
}

fn lower_primitive(
    primitive: Prim,
    arguments: &[Ast],
    builder: &mut FunctionBuilder,
) -> Option<ValueId> {
    let unary = match primitive {
        Prim::Inc => Some(UnaryOp::Inc),
        Prim::Dec => Some(UnaryOp::Dec),
        Prim::Not => Some(UnaryOp::Not),
        _ => None,
    };
    if let Some(operation) = unary {
        let [argument] = arguments else {
            return None;
        };
        let value = lower_scalar(argument, builder)?;
        let representation = if operation == UnaryOp::Not {
            Representation::BooleanImmediate
        } else {
            Representation::FixnumTagged
        };
        return Some(builder.append(
            InstructionKind::Unary {
                op: operation,
                value,
            },
            representation,
            Effects::MAY_THROW,
            None,
        ));
    }
    let operation = match primitive {
        Prim::Add => BinaryOp::Add,
        Prim::Sub => BinaryOp::Sub,
        Prim::Mul => BinaryOp::Mul,
        Prim::Eq => BinaryOp::Equal,
        Prim::Lt => BinaryOp::LessThan,
        Prim::Le => BinaryOp::LessThanOrEqual,
        Prim::Gt => BinaryOp::GreaterThan,
        Prim::Ge => BinaryOp::GreaterThanOrEqual,
        _ => return None,
    };
    let [left, right] = arguments else {
        return None;
    };
    let left = lower_scalar(left, builder)?;
    let right = lower_scalar(right, builder)?;
    let representation = if matches!(
        operation,
        BinaryOp::Equal
            | BinaryOp::LessThan
            | BinaryOp::LessThanOrEqual
            | BinaryOp::GreaterThan
            | BinaryOp::GreaterThanOrEqual
    ) {
        Representation::BooleanImmediate
    } else {
        Representation::FixnumTagged
    };
    Some(builder.append(
        InstructionKind::Binary {
            op: operation,
            left,
            right,
        },
        representation,
        Effects::MAY_THROW,
        None,
    ))
}
