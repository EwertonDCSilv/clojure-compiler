//! Fixnum representation inference and AST specialization: given a slot
//! environment of proven [`Representation`]s, rewrites primitive calls whose
//! arguments are all provably `FixnumTagged` to use [`Callee::ProvenFixnumPrim`],
//! letting codegen skip the generic dispatch and boxing paths. `infer_representation`
//! is the single source of truth for what representation an expression produces;
//! `facts` and `scalar_lowering`'s caller both build on it.

use clojure_analyzer::{Ast, Callee, Prim};
use clojure_ir::Representation;
use std::collections::HashMap;

pub(crate) fn specialize_fixnums(ast: Ast, environment: &HashMap<u32, Representation>) -> Ast {
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

pub(crate) fn infer_representation(
    ast: &Ast,
    environment: &HashMap<u32, Representation>,
) -> Representation {
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

pub(crate) fn loop_representations(
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
