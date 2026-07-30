//! Deterministic core-macro expansion for the compiled pipeline.
//!
//! AOT compilation does not execute a general macro interpreter. This module
//! expands the supported `clojure.core` macros (`when`, `when-not`, `if-not`,
//! `cond`, `and`, `or`, `->`, `->>`, and `doto`) into forms handled directly by
//! semantic analysis. Special forms, `let`, and `defn` remain intact, and quoted
//! data is never traversed.
//!
//! Expansion mirrors the bootstrap interpreter without creating a dependency on
//! it. Generated symbols use a deterministic per-call counter.

use clojure_span::{Span, Spanned};
use clojure_syntax::{Form, SForm};

/// Recursively expands supported macros in all top-level forms.
///
/// Source spans are preserved or inherited from the enclosing macro call.
/// Malformed macro forms remain unexpanded so semantic analysis can issue the
/// appropriate source diagnostic.
pub fn expand_all(forms: &[SForm]) -> Vec<SForm> {
    let mut ex = Expander { g: 0 };
    forms.iter().cloned().map(|f| ex.expand(f)).collect()
}

struct Expander {
    g: u64,
}

impl Expander {
    fn gensym(&mut self, prefix: &str) -> String {
        self.g += 1;
        format!("__cljn_{prefix}_{}", self.g)
    }

    fn expand(&mut self, f: SForm) -> SForm {
        match &f.node {
            Form::List(items) if !items.is_empty() => {
                if let Form::Symbol(n) = items[0].node.strip_meta() {
                    if n.ns.is_none() {
                        if n.name == "quote" {
                            return f; // Quoted data is an expansion boundary.
                        }
                        let args = items[1..].to_vec();
                        if let Some(expanded) = self.try_expand(&n.name, &args, f.span) {
                            return self.expand(expanded); // Reach a fixed point.
                        }
                    }
                }
                let span = f.span;
                let new_items = items.iter().cloned().map(|it| self.expand(it)).collect();
                Spanned::new(Form::List(new_items), span)
            }
            Form::Vector(items) => {
                let span = f.span;
                let v = items.iter().cloned().map(|it| self.expand(it)).collect();
                Spanned::new(Form::Vector(v), span)
            }
            Form::Set(items) => {
                let span = f.span;
                let v = items.iter().cloned().map(|it| self.expand(it)).collect();
                Spanned::new(Form::Set(v), span)
            }
            Form::Map(pairs) => {
                let span = f.span;
                let v = pairs
                    .iter()
                    .cloned()
                    .map(|(k, val)| (self.expand(k), self.expand(val)))
                    .collect();
                Spanned::new(Form::Map(v), span)
            }
            Form::Meta { meta, form } => {
                let span = f.span;
                let form = Box::new(self.expand((**form).clone()));
                Spanned::new(
                    Form::Meta {
                        meta: meta.clone(),
                        form,
                    },
                    span,
                )
            }
            _ => f,
        }
    }

    /// Performs one supported macro rewrite, or returns `None`.
    fn try_expand(&mut self, name: &str, args: &[SForm], span: Span) -> Option<SForm> {
        let sym = |s: &str| Spanned::new(Form::sym(s), span);
        let list = |v: Vec<SForm>| Spanned::new(Form::List(v), span);
        let nilf = || Spanned::new(Form::Nil, span);

        match name {
            "when" => {
                let test = args.first()?.clone();
                let mut body = vec![sym("do")];
                body.extend(args[1..].iter().cloned());
                Some(list(vec![sym("if"), test, list(body)]))
            }
            "when-not" => {
                let test = args.first()?.clone();
                let mut body = vec![sym("do")];
                body.extend(args[1..].iter().cloned());
                Some(list(vec![sym("if"), test, nilf(), list(body)]))
            }
            "if-not" => {
                let test = args.first()?.clone();
                let then = args.get(1)?.clone();
                let els = args.get(2).cloned().unwrap_or_else(nilf);
                Some(list(vec![sym("if"), test, els, then]))
            }
            "cond" => {
                if args.len() & 1 != 0 {
                    return None; // Let semantic analysis report malformed cond.
                }
                let mut result = nilf();
                for pair in args.chunks_exact(2).rev() {
                    let test = pair[0].clone();
                    let expr = pair[1].clone();
                    let is_else = matches!(&test.node, Form::Keyword(n) if n.ns.is_none() && n.name == "else");
                    result = if is_else {
                        expr
                    } else {
                        list(vec![sym("if"), test, expr, result])
                    };
                }
                Some(result)
            }
            "and" => match args.len() {
                0 => Some(Spanned::new(Form::Bool(true), span)),
                1 => Some(args[0].clone()),
                _ => {
                    let g = self.gensym("and");
                    let gsym = Spanned::new(Form::sym(&g), span);
                    let mut rest = vec![sym("and")];
                    rest.extend(args[1..].iter().cloned());
                    let binding =
                        Spanned::new(Form::Vector(vec![gsym.clone(), args[0].clone()]), span);
                    Some(list(vec![
                        sym("let*"),
                        binding,
                        list(vec![sym("if"), gsym.clone(), list(rest), gsym]),
                    ]))
                }
            },
            "or" => match args.len() {
                0 => Some(nilf()),
                1 => Some(args[0].clone()),
                _ => {
                    let g = self.gensym("or");
                    let gsym = Spanned::new(Form::sym(&g), span);
                    let mut rest = vec![sym("or")];
                    rest.extend(args[1..].iter().cloned());
                    let binding =
                        Spanned::new(Form::Vector(vec![gsym.clone(), args[0].clone()]), span);
                    Some(list(vec![
                        sym("let*"),
                        binding,
                        list(vec![sym("if"), gsym.clone(), gsym, list(rest)]),
                    ]))
                }
            },
            "->" => {
                let mut expr = args.first()?.clone();
                for step in &args[1..] {
                    expr = thread_into(step, expr, true, span);
                }
                Some(expr)
            }
            "->>" => {
                let mut expr = args.first()?.clone();
                for step in &args[1..] {
                    expr = thread_into(step, expr, false, span);
                }
                Some(expr)
            }
            "doto" => {
                let obj = args.first()?.clone();
                let g = self.gensym("doto");
                let gsym = Spanned::new(Form::sym(&g), span);
                let binding = Spanned::new(Form::Vector(vec![gsym.clone(), obj]), span);
                let mut body = vec![sym("let*"), binding];
                for step in &args[1..] {
                    body.push(thread_into(step, gsym.clone(), true, span));
                }
                body.push(gsym.clone());
                Some(list(body))
            }
            _ => None,
        }
    }
}

/// Inserts `expr` first for `->` or last for `->>` in one threading step.
fn thread_into(step: &SForm, expr: SForm, first: bool, span: Span) -> SForm {
    match step.node.strip_meta() {
        Form::List(items) if !items.is_empty() => {
            let head = items[0].clone();
            let mut v = vec![head];
            if first {
                v.push(expr);
                v.extend(items[1..].iter().cloned());
            } else {
                v.extend(items[1..].iter().cloned());
                v.push(expr);
            }
            Spanned::new(Form::List(v), span)
        }
        _ => Spanned::new(Form::List(vec![step.clone(), expr]), span),
    }
}

#[cfg(test)]
#[path = "../tests/unit/expand/mod.rs"]
mod tests;
