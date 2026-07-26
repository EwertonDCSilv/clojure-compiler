//! Pré-passo de expansão de macros para o caminho **compilado** (ADR-0004).
//!
//! O caminho de compilação AOT não roda o interpretador de macros geral; aqui
//! expandimos o conjunto de macros de `clojure.core` que o analyzer não trata como
//! forma especial: `when when-not if-not cond and or -> ->>`. Formas especiais
//! (`if do let* fn* def defn loop* recur quote ns`) e `let`/`defn` (reconhecidos
//! diretamente pelo analyzer) são preservadas. `(quote ...)` não é expandido.
//!
//! As expansões espelham a semântica do interpretador de bootstrap
//! (`clojure-interp`), mantendo os crates desacoplados.

use clojure_span::{Span, Spanned};
use clojure_syntax::{Form, SForm};

/// Expande as macros suportadas em todas as forms de topo.
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
                            return f; // não expande dados citados
                        }
                        let args = items[1..].to_vec();
                        if let Some(expanded) = self.try_expand(&n.name, &args, f.span) {
                            return self.expand(expanded); // re-expande o resultado
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

    /// Uma etapa de expansão para as macros suportadas. `None` se não for macro.
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
                    return None; // deixa o analyzer reportar erro estrutural
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
            _ => None,
        }
    }
}

/// Insere `expr` como 1º (`->`) ou último (`->>`) argumento de `step`.
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
mod tests {
    use super::*;

    fn expanded(source: &str) -> String {
        let forms = clojure_reader::read_all(0, source).expect("reader");
        expand_all(&forms)
            .iter()
            .map(|form| form.node.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn expands_conditional_macros() {
        assert_eq!(
            expanded("(when ready (print 1) (print 2))"),
            "(if ready (do (print 1) (print 2)))"
        );
        assert_eq!(expanded("(when-not ready 1)"), "(if ready nil (do 1))");
        assert_eq!(expanded("(if-not ready 1 2)"), "(if ready 2 1)");
        assert_eq!(expanded("(if-not ready 1)"), "(if ready nil 1)");
        assert_eq!(expanded("(cond a 1 b 2 :else 3)"), "(if a 1 (if b 2 3))");
    }

    #[test]
    fn expands_boolean_macros_without_double_evaluation() {
        assert_eq!(expanded("(and)"), "true");
        assert_eq!(expanded("(and x)"), "x");
        assert!(expanded("(and (f) (g))").contains("__cljn_and_1"));
        assert_eq!(expanded("(or)"), "nil");
        assert_eq!(expanded("(or x)"), "x");
        assert!(expanded("(or (f) (g))").contains("__cljn_or_1"));
    }

    #[test]
    fn expands_both_threading_directions_and_symbol_steps() {
        assert_eq!(expanded("(-> 5 inc (- 2))"), "(- (inc 5) 2)");
        assert_eq!(
            expanded("(->> xs (map f) (reduce + 0))"),
            "(reduce + 0 (map f xs))"
        );
    }

    #[test]
    fn preserves_quote_and_expands_inside_collections_and_metadata() {
        assert_eq!(expanded("'(when true 1)"), "(quote (when true 1))");
        assert_eq!(expanded("[(when true 1)]"), "[(if true (do 1))]");
        assert_eq!(expanded("#{(if-not false 1 2)}"), "#{(if false 2 1)}");
        assert_eq!(expanded("{:x (when true 1)}"), "{:x (if true (do 1))}");
        assert_eq!(expanded("^:m (when true 1)"), "^:m (if true (do 1))");
    }

    #[test]
    fn malformed_or_unknown_forms_remain_for_analyzer_diagnostics() {
        assert_eq!(expanded("(cond true)"), "(cond true)");
        assert_eq!(
            expanded("(unknown (when true 1))"),
            "(unknown (if true (do 1)))"
        );
    }
}
