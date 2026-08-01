//! Interprocedural fixnum-parameter fact collection: a fixed-point walk over
//! every direct call and `recur` site that proves, for each eligible method
//! (non-multimethod, non-lambda, non-variadic, never taken as a value), which
//! parameter slots are always called with a `FixnumTagged` argument. `escape`
//! supplies the set of functions this analysis must treat conservatively;
//! `specialization::infer_representation` supplies the per-expression
//! representation judgments the fixed point converges on.

use super::escape::escaped_functions;
use super::specialization::{infer_representation, loop_representations};
use clojure_analyzer::{Ast, Callee, Dispatch, Program};
use clojure_ir::Representation;
use std::collections::{HashMap, HashSet};

pub(crate) type MethodId = (String, usize);

#[derive(Clone)]
struct IncomingFacts {
    arguments: Vec<Representation>,
    direct_call_seen: bool,
}

pub(crate) fn infer_parameter_facts(
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

pub(crate) fn method_environment(
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
