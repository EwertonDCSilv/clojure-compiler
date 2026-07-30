//! Analyzer-AST adapter for the optional compiler-owned optimization IR.
//!
//! The first delivered slice lowers pure scalar islands to `clojure-ir`, runs
//! the verified local profile, and materializes proven constants back into the
//! analyzer program before the existing Cranelift lowering. Every AST variant is
//! traversed, while unsupported or effectful regions remain byte-for-byte
//! equivalent to the direct path. Full function CFG lowering remains tracked by
//! ADR-0014.

use clojure_analyzer::{Ast, Callee, Dispatch, Prim, Program};
use clojure_diagnostics::Diagnostic;
use clojure_ir::{
    run_safe_pipeline, BinaryOp, Constant, Effects, FunctionBuilder, InstructionKind,
    Representation, Terminator, UnaryOp, ValueId,
};
use std::collections::{HashMap, HashSet};

type MethodId = (String, usize);

#[derive(Clone)]
struct IncomingFacts {
    arguments: Vec<Representation>,
    direct_call_seen: bool,
}

/// Clones and conservatively optimizes every expression in a program.
pub(super) fn optimize_program(program: &Program) -> Result<Program, Diagnostic> {
    let mut optimized = program.clone();
    for function in &mut optimized.functions {
        for method in &mut function.methods {
            method.body = optimize_ast(method.body.clone())?;
        }
    }
    optimized.main_body = optimized
        .main_body
        .into_iter()
        .map(optimize_ast)
        .collect::<Result<_, _>>()?;

    let (parameter_facts, directly_called_methods) = infer_parameter_facts(&optimized);
    for function in &mut optimized.functions {
        for (method_index, method) in function.methods.iter_mut().enumerate() {
            let method_id = (function.name.clone(), method_index);
            let environment = method_environment(&parameter_facts, &function.name, method_index);
            method.optimization.proven_fixnum_params = environment
                .iter()
                .filter_map(|(slot, representation)| {
                    (*slot < method.params.len() as u32)
                        .then_some((*slot as usize, *representation))
                })
                .fold(
                    vec![false; method.params.len()],
                    |mut parameters, (slot, representation)| {
                        parameters[slot] = representation == Representation::FixnumTagged;
                        parameters
                    },
                );
            method.body = specialize_fixnums(method.body.clone(), &environment);
            method.optimization.proven_fixnum_return =
                infer_representation(&method.body, &environment) == Representation::FixnumTagged;
            method.optimization.specialized_fixnum_abi = method.rest.is_none()
                && directly_called_methods.contains(&method_id)
                && method
                    .optimization
                    .proven_fixnum_params
                    .iter()
                    .all(|proven| *proven)
                && method.optimization.proven_fixnum_return;
        }
    }
    optimized.main_body = optimized
        .main_body
        .into_iter()
        .map(|expression| specialize_fixnums(expression, &HashMap::new()))
        .collect();
    Ok(optimized)
}

fn infer_parameter_facts(
    program: &Program,
) -> (HashMap<MethodId, Vec<Representation>>, HashSet<MethodId>) {
    let escaped = escaped_functions(program);
    let signatures = direct_method_signatures(program);
    let eligible_methods = program
        .functions
        .iter()
        .flat_map(|function| {
            let escaped = &escaped;
            function
                .methods
                .iter()
                .enumerate()
                .filter(move |(_, method)| {
                    function.dispatch == Dispatch::None
                        && !function.is_lambda
                        && method.rest.is_none()
                        && !escaped.contains(&function.name)
                })
                .map(move |(method_index, _)| (function.name.clone(), method_index))
        })
        .collect::<HashSet<_>>();
    let mut facts = HashMap::new();
    for function in &program.functions {
        for (method_index, method) in function.methods.iter().enumerate() {
            let eligible = eligible_methods.contains(&(function.name.clone(), method_index));
            facts.insert(
                (function.name.clone(), method_index),
                vec![
                    if eligible {
                        Representation::FixnumTagged
                    } else {
                        Representation::UnknownTagged
                    };
                    method.params.len()
                ],
            );
        }
    }

    loop {
        let mut incoming = facts
            .iter()
            .map(|(method, arguments)| {
                (
                    method.clone(),
                    IncomingFacts {
                        arguments: vec![Representation::FixnumTagged; arguments.len()],
                        direct_call_seen: false,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        for function in &program.functions {
            for (method_index, method) in function.methods.iter().enumerate() {
                let method_id = (function.name.clone(), method_index);
                let environment = method_environment(&facts, &function.name, method_index);
                collect_incoming_facts(
                    &method.body,
                    &environment,
                    Some(&method_id),
                    true,
                    &signatures,
                    &mut incoming,
                );
            }
        }
        for expression in &program.main_body {
            collect_incoming_facts(
                expression,
                &HashMap::new(),
                None,
                false,
                &signatures,
                &mut incoming,
            );
        }

        let mut changed = false;
        for function in &program.functions {
            for (method_index, method) in function.methods.iter().enumerate() {
                let method_id = (function.name.clone(), method_index);
                let eligible = eligible_methods.contains(&method_id);
                let observed = &incoming[&method_id];
                let next = if eligible && observed.direct_call_seen {
                    observed.arguments.clone()
                } else {
                    vec![Representation::UnknownTagged; method.params.len()]
                };
                if facts.get(&method_id) != Some(&next) {
                    facts.insert(method_id, next);
                    changed = true;
                }
            }
        }
        if !changed {
            let directly_called = incoming
                .into_iter()
                .filter_map(|(method, observed)| {
                    (observed.direct_call_seen && eligible_methods.contains(&method))
                        .then_some(method)
                })
                .collect();
            return (facts, directly_called);
        }
    }
}

fn method_environment(
    facts: &HashMap<MethodId, Vec<Representation>>,
    function: &str,
    method_index: usize,
) -> HashMap<u32, Representation> {
    facts
        .get(&(function.to_owned(), method_index))
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(slot, representation)| (slot as u32, *representation))
        .collect()
}

fn direct_method_signatures(program: &Program) -> HashMap<(String, usize), MethodId> {
    let mut signatures = HashMap::new();
    for function in &program.functions {
        if function.dispatch != Dispatch::None {
            continue;
        }
        for (method_index, method) in function.methods.iter().enumerate() {
            if method.rest.is_none() {
                signatures.insert(
                    (function.name.clone(), method.params.len()),
                    (function.name.clone(), method_index),
                );
            }
        }
    }
    signatures
}

fn escaped_functions(program: &Program) -> HashSet<String> {
    let mut escaped = HashSet::new();
    for function in &program.functions {
        for method in &function.methods {
            collect_escaped_functions(&method.body, &mut escaped);
        }
    }
    for expression in &program.main_body {
        collect_escaped_functions(expression, &mut escaped);
    }
    escaped
}

fn collect_escaped_functions(ast: &Ast, escaped: &mut HashSet<String>) {
    match ast {
        Ast::FnRef(function) => {
            escaped.insert(function.clone());
        }
        Ast::MakeFn {
            lambda, captures, ..
        } => {
            escaped.insert(lambda.clone());
            for capture in captures {
                collect_escaped_functions(capture, escaped);
            }
        }
        Ast::VecLit(items) | Ast::SetLit(items) | Ast::Do(items) | Ast::Recur(items) => {
            for item in items {
                collect_escaped_functions(item, escaped);
            }
        }
        Ast::MapLit(pairs) => {
            for (key, value) in pairs {
                collect_escaped_functions(key, escaped);
                collect_escaped_functions(value, escaped);
            }
        }
        Ast::DefGlobal { value, .. } => collect_escaped_functions(value, escaped),
        Ast::If(test, then, otherwise) => {
            collect_escaped_functions(test, escaped);
            collect_escaped_functions(then, escaped);
            collect_escaped_functions(otherwise, escaped);
        }
        Ast::Let { slots, body } | Ast::Loop { slots, body } => {
            for (_, initializer) in slots {
                collect_escaped_functions(initializer, escaped);
            }
            collect_escaped_functions(body, escaped);
        }
        Ast::Call { args, .. } => {
            for argument in args {
                collect_escaped_functions(argument, escaped);
            }
        }
        Ast::CallValue { f, args } => {
            collect_escaped_functions(f, escaped);
            for argument in args {
                collect_escaped_functions(argument, escaped);
            }
        }
        Ast::Apply { f, fixed, coll } => {
            collect_escaped_functions(f, escaped);
            for argument in fixed {
                collect_escaped_functions(argument, escaped);
            }
            collect_escaped_functions(coll, escaped);
        }
        Ast::MakeRecord { fields, .. } => {
            for (_, value) in fields {
                collect_escaped_functions(value, escaped);
            }
        }
        Ast::RegisterMethod { key, impl_fn, .. } => {
            collect_escaped_functions(key, escaped);
            collect_escaped_functions(impl_fn, escaped);
        }
        Ast::RegisterMulti { dispatch_fn, .. } => {
            collect_escaped_functions(dispatch_fn, escaped);
        }
        Ast::Nil
        | Ast::Bool(_)
        | Ast::Int(_)
        | Ast::Float(_)
        | Ast::Str(_)
        | Ast::Keyword(_)
        | Ast::GlobalRef(_)
        | Ast::Local(_)
        | Ast::Capture(_) => {}
    }
}

fn collect_incoming_facts(
    ast: &Ast,
    environment: &HashMap<u32, Representation>,
    current_method: Option<&MethodId>,
    method_recur_allowed: bool,
    signatures: &HashMap<(String, usize), MethodId>,
    incoming: &mut HashMap<MethodId, IncomingFacts>,
) {
    match ast {
        Ast::Call { callee, args } => {
            for argument in args {
                collect_incoming_facts(
                    argument,
                    environment,
                    current_method,
                    method_recur_allowed,
                    signatures,
                    incoming,
                );
            }
            if let Callee::Fn(function) = callee {
                if let Some(method_id) = signatures.get(&(function.clone(), args.len())) {
                    let observed = incoming
                        .get_mut(method_id)
                        .expect("every direct signature has incoming facts");
                    observed.direct_call_seen = true;
                    join_arguments(&mut observed.arguments, args, environment);
                }
            }
        }
        Ast::Recur(args) if method_recur_allowed => {
            for argument in args {
                collect_incoming_facts(
                    argument,
                    environment,
                    current_method,
                    method_recur_allowed,
                    signatures,
                    incoming,
                );
            }
            if let Some(method_id) = current_method {
                let observed = incoming
                    .get_mut(method_id)
                    .expect("current method has incoming facts");
                join_arguments(&mut observed.arguments, args, environment);
            }
        }
        Ast::Let { slots, body } => {
            let mut local = environment.clone();
            for (slot, initializer) in slots {
                collect_incoming_facts(
                    initializer,
                    &local,
                    current_method,
                    method_recur_allowed,
                    signatures,
                    incoming,
                );
                local.insert(*slot, infer_representation(initializer, &local));
            }
            collect_incoming_facts(
                body,
                &local,
                current_method,
                method_recur_allowed,
                signatures,
                incoming,
            );
        }
        Ast::Loop { slots, body } => {
            let mut local = environment.clone();
            for (slot, initializer) in slots {
                collect_incoming_facts(
                    initializer,
                    &local,
                    current_method,
                    method_recur_allowed,
                    signatures,
                    incoming,
                );
                local.insert(*slot, infer_representation(initializer, &local));
            }
            let loop_environment = loop_representations(slots, body, environment);
            collect_incoming_facts(
                body,
                &loop_environment,
                current_method,
                false,
                signatures,
                incoming,
            );
        }
        Ast::If(test, then, otherwise) => {
            for expression in [test.as_ref(), then.as_ref(), otherwise.as_ref()] {
                collect_incoming_facts(
                    expression,
                    environment,
                    current_method,
                    method_recur_allowed,
                    signatures,
                    incoming,
                );
            }
        }
        Ast::VecLit(items) | Ast::SetLit(items) | Ast::Do(items) | Ast::Recur(items) => {
            for item in items {
                collect_incoming_facts(
                    item,
                    environment,
                    current_method,
                    method_recur_allowed,
                    signatures,
                    incoming,
                );
            }
        }
        Ast::MapLit(pairs) => {
            for (key, value) in pairs {
                for expression in [key, value] {
                    collect_incoming_facts(
                        expression,
                        environment,
                        current_method,
                        method_recur_allowed,
                        signatures,
                        incoming,
                    );
                }
            }
        }
        Ast::DefGlobal { value, .. } => collect_incoming_facts(
            value,
            environment,
            current_method,
            method_recur_allowed,
            signatures,
            incoming,
        ),
        Ast::MakeFn { captures, .. } => {
            for capture in captures {
                collect_incoming_facts(
                    capture,
                    environment,
                    current_method,
                    method_recur_allowed,
                    signatures,
                    incoming,
                );
            }
        }
        Ast::CallValue { f, args } => {
            collect_incoming_facts(
                f,
                environment,
                current_method,
                method_recur_allowed,
                signatures,
                incoming,
            );
            for argument in args {
                collect_incoming_facts(
                    argument,
                    environment,
                    current_method,
                    method_recur_allowed,
                    signatures,
                    incoming,
                );
            }
        }
        Ast::Apply { f, fixed, coll } => {
            collect_incoming_facts(
                f,
                environment,
                current_method,
                method_recur_allowed,
                signatures,
                incoming,
            );
            for argument in fixed {
                collect_incoming_facts(
                    argument,
                    environment,
                    current_method,
                    method_recur_allowed,
                    signatures,
                    incoming,
                );
            }
            collect_incoming_facts(
                coll,
                environment,
                current_method,
                method_recur_allowed,
                signatures,
                incoming,
            );
        }
        Ast::MakeRecord { fields, .. } => {
            for (_, value) in fields {
                collect_incoming_facts(
                    value,
                    environment,
                    current_method,
                    method_recur_allowed,
                    signatures,
                    incoming,
                );
            }
        }
        Ast::RegisterMethod { key, impl_fn, .. } => {
            for expression in [key.as_ref(), impl_fn.as_ref()] {
                collect_incoming_facts(
                    expression,
                    environment,
                    current_method,
                    method_recur_allowed,
                    signatures,
                    incoming,
                );
            }
        }
        Ast::RegisterMulti { dispatch_fn, .. } => collect_incoming_facts(
            dispatch_fn,
            environment,
            current_method,
            method_recur_allowed,
            signatures,
            incoming,
        ),
        Ast::Nil
        | Ast::Bool(_)
        | Ast::Int(_)
        | Ast::Float(_)
        | Ast::Str(_)
        | Ast::Keyword(_)
        | Ast::GlobalRef(_)
        | Ast::Local(_)
        | Ast::Capture(_)
        | Ast::FnRef(_) => {}
    }
}

fn join_arguments(
    accumulated: &mut [Representation],
    arguments: &[Ast],
    environment: &HashMap<u32, Representation>,
) {
    for (fact, argument) in accumulated.iter_mut().zip(arguments) {
        *fact = fact.join(infer_representation(argument, environment));
    }
}

fn optimize_ast(ast: Ast) -> Result<Ast, Diagnostic> {
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

fn specialize_fixnums(ast: Ast, environment: &HashMap<u32, Representation>) -> Ast {
    match ast {
        Ast::VecLit(items) => Ast::VecLit(
            items
                .into_iter()
                .map(|item| specialize_fixnums(item, environment))
                .collect(),
        ),
        Ast::SetLit(items) => Ast::SetLit(
            items
                .into_iter()
                .map(|item| specialize_fixnums(item, environment))
                .collect(),
        ),
        Ast::MapLit(pairs) => Ast::MapLit(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    (
                        specialize_fixnums(key, environment),
                        specialize_fixnums(value, environment),
                    )
                })
                .collect(),
        ),
        Ast::DefGlobal { index, value } => Ast::DefGlobal {
            index,
            value: Box::new(specialize_fixnums(*value, environment)),
        },
        Ast::MakeFn {
            lambda,
            arity,
            captures,
        } => Ast::MakeFn {
            lambda,
            arity,
            captures: captures
                .into_iter()
                .map(|capture| specialize_fixnums(capture, environment))
                .collect(),
        },
        Ast::If(test, then, otherwise) => Ast::If(
            Box::new(specialize_fixnums(*test, environment)),
            Box::new(specialize_fixnums(*then, environment)),
            Box::new(specialize_fixnums(*otherwise, environment)),
        ),
        Ast::Do(expressions) => Ast::Do(
            expressions
                .into_iter()
                .map(|expression| specialize_fixnums(expression, environment))
                .collect(),
        ),
        Ast::Let { slots, body } => {
            let mut local = environment.clone();
            let mut specialized_slots = Vec::with_capacity(slots.len());
            for (slot, initializer) in slots {
                let initializer = specialize_fixnums(initializer, &local);
                local.insert(slot, infer_representation(&initializer, &local));
                specialized_slots.push((slot, initializer));
            }
            Ast::Let {
                slots: specialized_slots,
                body: Box::new(specialize_fixnums(*body, &local)),
            }
        }
        Ast::Loop { slots, body } => {
            let mut initializer_environment = environment.clone();
            let mut specialized_slots = Vec::with_capacity(slots.len());
            for (slot, initializer) in slots {
                let initializer = specialize_fixnums(initializer, &initializer_environment);
                initializer_environment.insert(
                    slot,
                    infer_representation(&initializer, &initializer_environment),
                );
                specialized_slots.push((slot, initializer));
            }
            let loop_environment = loop_representations(&specialized_slots, &body, environment);
            Ast::Loop {
                slots: specialized_slots,
                body: Box::new(specialize_fixnums(*body, &loop_environment)),
            }
        }
        Ast::Recur(arguments) => Ast::Recur(
            arguments
                .into_iter()
                .map(|argument| specialize_fixnums(argument, environment))
                .collect(),
        ),
        Ast::Call { callee, args } => {
            let args = args
                .into_iter()
                .map(|argument| specialize_fixnums(argument, environment))
                .collect::<Vec<_>>();
            let callee = match callee {
                Callee::Prim(primitive)
                    if fixnum_specializable(primitive, &args)
                        && args.iter().all(|argument| {
                            infer_representation(argument, environment)
                                == Representation::FixnumTagged
                        }) =>
                {
                    Callee::ProvenFixnumPrim(primitive)
                }
                other => other,
            };
            Ast::Call { callee, args }
        }
        Ast::CallValue { f, args } => Ast::CallValue {
            f: Box::new(specialize_fixnums(*f, environment)),
            args: args
                .into_iter()
                .map(|argument| specialize_fixnums(argument, environment))
                .collect(),
        },
        Ast::Apply { f, fixed, coll } => Ast::Apply {
            f: Box::new(specialize_fixnums(*f, environment)),
            fixed: fixed
                .into_iter()
                .map(|argument| specialize_fixnums(argument, environment))
                .collect(),
            coll: Box::new(specialize_fixnums(*coll, environment)),
        },
        Ast::MakeRecord { type_name, fields } => Ast::MakeRecord {
            type_name,
            fields: fields
                .into_iter()
                .map(|(name, value)| (name, specialize_fixnums(value, environment)))
                .collect(),
        },
        Ast::RegisterMethod {
            method_id,
            key,
            impl_fn,
        } => Ast::RegisterMethod {
            method_id,
            key: Box::new(specialize_fixnums(*key, environment)),
            impl_fn: Box::new(specialize_fixnums(*impl_fn, environment)),
        },
        Ast::RegisterMulti {
            method_id,
            dispatch_fn,
        } => Ast::RegisterMulti {
            method_id,
            dispatch_fn: Box::new(specialize_fixnums(*dispatch_fn, environment)),
        },
        scalar => scalar,
    }
}

fn infer_representation(ast: &Ast, environment: &HashMap<u32, Representation>) -> Representation {
    match ast {
        Ast::Int(_) => Representation::FixnumTagged,
        Ast::Local(slot) => environment
            .get(slot)
            .copied()
            .unwrap_or(Representation::UnknownTagged),
        Ast::Do(expressions) => expressions
            .last()
            .map(|expression| infer_representation(expression, environment))
            .unwrap_or(Representation::UnknownTagged),
        Ast::Let { slots, body } => {
            let mut local = environment.clone();
            for (slot, initializer) in slots {
                local.insert(*slot, infer_representation(initializer, &local));
            }
            infer_representation(body, &local)
        }
        Ast::Loop { slots, body } => {
            let local = loop_representations(slots, body, environment);
            infer_representation(body, &local)
        }
        Ast::If(_, then, otherwise) if always_diverges(then) => {
            infer_representation(otherwise, environment)
        }
        Ast::If(_, then, otherwise) if always_diverges(otherwise) => {
            infer_representation(then, environment)
        }
        Ast::If(_, then, otherwise) => infer_representation(then, environment)
            .join(infer_representation(otherwise, environment)),
        Ast::Call { callee, args }
            if matches!(
                callee,
                Callee::Prim(
                    Prim::Add
                        | Prim::Sub
                        | Prim::Mul
                        | Prim::Quot
                        | Prim::Mod
                        | Prim::Inc
                        | Prim::Dec
                ) | Callee::ProvenFixnumPrim(
                    Prim::Add
                        | Prim::Sub
                        | Prim::Mul
                        | Prim::Quot
                        | Prim::Mod
                        | Prim::Inc
                        | Prim::Dec
                )
            ) && fixnum_specializable(
                match callee {
                    Callee::Prim(primitive) | Callee::ProvenFixnumPrim(primitive) => *primitive,
                    Callee::Fn(_) => unreachable!(),
                },
                args,
            ) && args.iter().all(|argument| {
                infer_representation(argument, environment) == Representation::FixnumTagged
            }) =>
        {
            Representation::FixnumTagged
        }
        Ast::Call {
            callee:
                Callee::Prim(
                    Prim::Quot
                    | Prim::Mod
                    | Prim::Count
                    | Prim::Compare
                    | Prim::IntOf
                    | Prim::Bget
                    | Prim::FileSize
                    | Prim::FileModified,
                )
                | Callee::ProvenFixnumPrim(Prim::Quot | Prim::Mod),
            ..
        } => Representation::FixnumTagged,
        _ => Representation::UnknownTagged,
    }
}

/// Returns whether an expression has no normal value-producing path.
fn always_diverges(ast: &Ast) -> bool {
    match ast {
        Ast::Recur(_) => true,
        Ast::Do(expressions) => expressions.last().is_some_and(always_diverges),
        Ast::Let { body, .. } | Ast::Loop { body, .. } => always_diverges(body),
        Ast::If(_, then, otherwise) => always_diverges(then) && always_diverges(otherwise),
        _ => false,
    }
}

fn loop_representations(
    slots: &[(u32, Ast)],
    body: &Ast,
    environment: &HashMap<u32, Representation>,
) -> HashMap<u32, Representation> {
    let mut local = environment.clone();
    let mut initial = Vec::with_capacity(slots.len());
    for (slot, initializer) in slots {
        let representation = infer_representation(initializer, &local);
        initial.push(representation);
        local.insert(*slot, representation);
    }
    loop {
        let mut recur = vec![Representation::FixnumTagged; slots.len()];
        let mut seen = false;
        collect_recur_representations(body, &local, &mut recur, &mut seen);
        let mut changed = false;
        for (index, (slot, _)) in slots.iter().enumerate() {
            let next = if seen {
                initial[index].join(recur[index])
            } else {
                initial[index]
            };
            if local.get(slot).copied() != Some(next) {
                local.insert(*slot, next);
                changed = true;
            }
        }
        if !changed {
            return local;
        }
    }
}

fn collect_recur_representations(
    ast: &Ast,
    environment: &HashMap<u32, Representation>,
    recur: &mut [Representation],
    seen: &mut bool,
) {
    match ast {
        Ast::Recur(arguments) => {
            *seen = true;
            for (index, argument) in arguments.iter().enumerate().take(recur.len()) {
                recur[index] = recur[index].join(infer_representation(argument, environment));
            }
        }
        Ast::Loop { .. } => {}
        Ast::If(_, then, otherwise) => {
            collect_recur_representations(then, environment, recur, seen);
            collect_recur_representations(otherwise, environment, recur, seen);
        }
        Ast::Let { slots, body } => {
            let mut local = environment.clone();
            for (slot, initializer) in slots {
                local.insert(*slot, infer_representation(initializer, &local));
            }
            collect_recur_representations(body, &local, recur, seen);
        }
        Ast::Do(expressions) => {
            for expression in expressions {
                collect_recur_representations(expression, environment, recur, seen);
            }
        }
        _ => {}
    }
}

fn fixnum_specializable(primitive: Prim, arguments: &[Ast]) -> bool {
    match primitive {
        Prim::Inc | Prim::Dec => arguments.len() == 1,
        Prim::Add | Prim::Sub | Prim::Mul => !arguments.is_empty(),
        Prim::Quot | Prim::Mod | Prim::Eq | Prim::Lt | Prim::Le | Prim::Gt | Prim::Ge => {
            arguments.len() == 2
        }
        _ => false,
    }
}

#[cfg(test)]
#[path = "../tests/unit/ir_adapter/mod.rs"]
mod tests;
