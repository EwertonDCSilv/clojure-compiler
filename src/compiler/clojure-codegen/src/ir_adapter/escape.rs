//! Escape analysis: collects the set of top-level function names that are
//! ever taken as a value (referenced as `FnRef` or captured by a closure)
//! rather than only called directly by name. `facts` treats an escaped
//! function's parameter representations as unknown, since indirect callers
//! are not visible to the interprocedural fact-collection walk.

use clojure_analyzer::{Ast, Program};
use std::collections::HashSet;

pub(crate) fn escaped_functions(program: &Program) -> HashSet<String> {
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
