//! Analisador do **subconjunto compilável** (Fase 3, corte vertical p/ Fase 5).
//!
//! Transforma `Form` em uma AST tipada por variante, com resolução de locais e
//! funções, checagem de aridade e detecção de construções fora do subconjunto
//! compilável (que viram diagnósticos, nunca comportamento silencioso —
//! specs/COMPATIBILITY_SPEC.md).
//!
//! Subconjunto compilável do MVP-slice (valores são inteiros `i64`; strings só
//! como argumento de `println`):
//! `ns`, `def`/`defn` (aridade fixa), `if`, `do`, `let`, chamadas diretas de
//! função (inclui recursão), primitivas `+ - * = < <= > >= println`, literais
//! inteiros/booleanos/`nil`/string.

use clojure_diagnostics::{Diagnostic, Diagnostics};
use clojure_span::Span;
use clojure_syntax::{Form, SForm};
use std::collections::HashMap;

mod expand;
pub use expand::expand_all;

/// Nó de AST do subconjunto compilável.
#[derive(Debug, Clone)]
pub enum Ast {
    Int(i64),
    Bool(bool),
    Nil,
    Str(String),
    /// Referência a um local por slot.
    Local(u32),
    If(Box<Ast>, Box<Ast>, Box<Ast>),
    Do(Vec<Ast>),
    /// `let*`: cada binding define o slot `first_slot + i` e é visível nos seguintes.
    Let { slots: Vec<(u32, Ast)>, body: Box<Ast> },
    /// `loop*`: alvo de `recur`. `slots` são as variáveis de loop.
    Loop { slots: Vec<(u32, Ast)>, body: Box<Ast> },
    /// `recur`: religa o alvo mais próximo (loop ou fn) e salta.
    Recur(Vec<Ast>),
    Call { callee: Callee, args: Vec<Ast> },
}

#[derive(Debug, Clone)]
pub enum Callee {
    Prim(Prim),
    /// Chamada direta a uma função definida no programa.
    Fn(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prim {
    Add,
    Sub,
    Mul,
    Quot,
    Mod,
    Inc,
    Dec,
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
    Not,
    NilP,
    EmptyP,
    Cons,
    First,
    Rest,
    Count,
    List,
    Str,
    Println,
    Print,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Ast,
    /// Número total de slots locais (params + lets).
    pub local_count: u32,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
    /// Formas de topo não-`defn` (modelo de script), avaliadas em `main`.
    pub main_body: Vec<Ast>,
    pub main_local_count: u32,
}

/// Analisa um conjunto de forms de topo no `Program` compilável.
///
/// Expande primeiro as macros de core suportadas (when/cond/and/or/->/->>; ADR-0004)
/// e então analisa o resultado.
pub fn analyze(forms: &[SForm]) -> Result<Program, Diagnostics> {
    let expanded = expand::expand_all(forms);
    analyze_expanded(&expanded)
}

fn analyze_expanded(forms: &[SForm]) -> Result<Program, Diagnostics> {
    let mut diags = Diagnostics::new();

    // Passo 1: coletar assinaturas de funções (para forward-refs / recursão).
    let mut sigs: HashMap<String, usize> = HashMap::new();
    for f in forms {
        if let Some((name, params, _)) = match_defn(f) {
            sigs.insert(name, params.len());
        }
    }

    // Passo 2: analisar corpos e o script de topo.
    let mut functions = Vec::new();
    let mut main_body = Vec::new();
    let mut main_scope = Scope::new(&sigs);

    for f in forms {
        if is_ns(f) {
            continue;
        }
        if let Some((name, params, body_forms)) = match_defn(f) {
            let mut scope = Scope::new(&sigs);
            for p in &params {
                scope.push_local(p.clone());
            }
            // A fn é um alvo de recur com a aridade dos seus parâmetros.
            scope.recur_arity.push(params.len());
            match scope.analyze_body(&body_forms, f.span, true) {
                Ok(body) => functions.push(Function {
                    name,
                    params,
                    body,
                    local_count: scope.max_slots,
                }),
                Err(d) => diags.push(d),
            }
        } else {
            // Forma de topo (ex.: `(-main)`) — não é posição de cauda de recur.
            match main_scope.analyze(f, false) {
                Ok(a) => main_body.push(a),
                Err(d) => diags.push(d),
            }
        }
    }

    if diags.has_errors() {
        return Err(diags);
    }
    Ok(Program {
        functions,
        main_body,
        main_local_count: main_scope.max_slots,
    })
}

fn is_ns(f: &SForm) -> bool {
    matches!(f.node.strip_meta(), Form::List(items)
        if matches!(items.first().map(|h| h.node.strip_meta()), Some(Form::Symbol(n)) if n.ns.is_none() && n.name == "ns"))
}

/// Reconhece `(defn nome [params] body...)` (aridade fixa, sem docstring/attrs).
fn match_defn(f: &SForm) -> Option<(String, Vec<String>, Vec<SForm>)> {
    let Form::List(items) = f.node.strip_meta() else {
        return None;
    };
    let head = items.first()?;
    let Form::Symbol(n) = head.node.strip_meta() else {
        return None;
    };
    if n.ns.is_some() || (n.name != "defn" && n.name != "defn-") {
        return None;
    }
    let name = match items.get(1).map(|f| f.node.strip_meta()) {
        Some(Form::Symbol(nm)) if nm.ns.is_none() => nm.name.clone(),
        _ => return None,
    };
    let params = match items.get(2).map(|f| f.node.strip_meta()) {
        Some(Form::Vector(ps)) => {
            let mut out = Vec::new();
            for p in ps {
                match p.node.strip_meta() {
                    Form::Symbol(pn) if pn.ns.is_none() => out.push(pn.name.clone()),
                    _ => return None, // destructuring/variádico fora do slice
                }
            }
            out
        }
        _ => return None,
    };
    let body = items[3..].to_vec();
    Some((name, params, body))
}

struct Scope<'a> {
    sigs: &'a HashMap<String, usize>,
    /// Pilha de locais visíveis: nome → slot.
    locals: Vec<(String, u32)>,
    next_slot: u32,
    max_slots: u32,
    /// Pilha de aridades de alvos de `recur` (fn/loop mais internos).
    recur_arity: Vec<usize>,
}

impl<'a> Scope<'a> {
    fn new(sigs: &'a HashMap<String, usize>) -> Self {
        Scope { sigs, locals: Vec::new(), next_slot: 0, max_slots: 0, recur_arity: Vec::new() }
    }

    fn push_local(&mut self, name: String) -> u32 {
        let slot = self.next_slot;
        self.next_slot += 1;
        self.max_slots = self.max_slots.max(self.next_slot);
        self.locals.push((name, slot));
        slot
    }

    fn lookup(&self, name: &str) -> Option<u32> {
        self.locals.iter().rev().find(|(n, _)| n == name).map(|(_, s)| *s)
    }

    /// Corpo (sequência implícita de `do`). `tail` propaga só para a última forma.
    fn analyze_body(&mut self, body: &[SForm], _span: Span, tail: bool) -> Result<Ast, Diagnostic> {
        if body.is_empty() {
            return Ok(Ast::Nil);
        }
        let last = body.len() - 1;
        let mut stmts = Vec::new();
        for (i, f) in body.iter().enumerate() {
            stmts.push(self.analyze(f, tail && i == last)?);
        }
        if stmts.len() == 1 {
            Ok(stmts.into_iter().next().unwrap())
        } else {
            Ok(Ast::Do(stmts))
        }
    }

    fn analyze(&mut self, f: &SForm, tail: bool) -> Result<Ast, Diagnostic> {
        match f.node.strip_meta() {
            Form::Nil => Ok(Ast::Nil),
            Form::Bool(b) => Ok(Ast::Bool(*b)),
            Form::Int(n) => Ok(Ast::Int(*n)),
            Form::Str(s) => Ok(Ast::Str(s.clone())),
            Form::Float(_) => Err(unsupported(
                "ponto flutuante ainda não é compilável (slice inteiro)",
                f.span,
            )),
            Form::Char(_) => Err(unsupported("char ainda não é compilável", f.span)),
            Form::Keyword(_) => Err(unsupported("keyword ainda não é compilável", f.span)),
            Form::Symbol(n) => {
                if n.ns.is_some() {
                    return Err(unsupported(format!("símbolo qualificado {n} fora do slice"), f.span));
                }
                match self.lookup(&n.name) {
                    Some(slot) => Ok(Ast::Local(slot)),
                    None => Err(Diagnostic::error("E0101", format!("símbolo não resolvido: {n}"))
                        .with_span(f.span)
                        .with_help("no subconjunto compilável só há locais e chamadas de função/primitiva")),
                }
            }
            Form::Vector(_) | Form::Map(_) | Form::Set(_) => {
                Err(unsupported("literais de coleção ainda não são compiláveis (slice inteiro)", f.span))
            }
            Form::List(items) => self.analyze_list(items, f.span, tail),
            Form::Meta { .. } => unreachable!(),
        }
    }

    fn analyze_list(&mut self, items: &[SForm], span: Span, tail: bool) -> Result<Ast, Diagnostic> {
        let Some((head, args)) = items.split_first() else {
            return Err(unsupported("lista vazia não é compilável", span));
        };
        let Form::Symbol(op) = head.node.strip_meta() else {
            return Err(unsupported("operador deve ser um símbolo no slice", head.span));
        };
        if op.ns.is_some() {
            return Err(unsupported(format!("operador qualificado {op} fora do slice"), head.span));
        }
        match op.name.as_str() {
            "if" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(Diagnostic::error("E0102", "if requer test, then e (opcional) else").with_span(span));
                }
                // test não é cauda; then/else herdam `tail`.
                let test = self.analyze(&args[0], false)?;
                let then = self.analyze(&args[1], tail)?;
                let els = match args.get(2) {
                    Some(e) => self.analyze(e, tail)?,
                    None => Ast::Nil,
                };
                Ok(Ast::If(Box::new(test), Box::new(then), Box::new(els)))
            }
            "do" => self.analyze_body(args, span, tail),
            "let" | "let*" => self.analyze_let(args, span, tail),
            "loop" | "loop*" => self.analyze_loop(args, span, tail),
            "recur" => self.analyze_recur(args, span, tail),
            "quote" => Err(unsupported("quote ainda não é compilável", span)),
            "fn" | "fn*" => Err(unsupported(
                "funções anônimas/closures ainda não são compiláveis (slice); defina com defn no topo",
                span,
            )),
            "def" | "defn" | "defn-" => Err(unsupported("def/defn só é permitido no nível de topo", span)),
            name => {
                // Primitiva ou chamada de função.
                let callee = if let Some(prim) = prim_of(name) {
                    check_prim_arity(prim, args.len(), span)?;
                    Callee::Prim(prim)
                } else if let Some(&arity) = self.sigs.get(name) {
                    if arity != args.len() {
                        return Err(Diagnostic::error(
                            "E0103",
                            format!("aridade errada ao chamar {name}: esperava {arity}, recebeu {}", args.len()),
                        )
                        .with_span(span));
                    }
                    Callee::Fn(name.to_string())
                } else if self.lookup(name).is_some() {
                    return Err(unsupported(
                        format!("`{name}` é um local; chamar valores-função ainda não é compilável no slice"),
                        span,
                    ));
                } else {
                    return Err(Diagnostic::error("E0101", format!("função não resolvida: {name}"))
                        .with_span(span)
                        .with_help("defina-a com defn no topo, ou use uma primitiva do slice"));
                };
                let arg_ast = self.analyze_seq(args)?;
                Ok(Ast::Call { callee, args: arg_ast })
            }
        }
    }

    /// Argumentos de chamada: nunca em posição de cauda.
    fn analyze_seq(&mut self, forms: &[SForm]) -> Result<Vec<Ast>, Diagnostic> {
        forms.iter().map(|f| self.analyze(f, false)).collect()
    }

    fn analyze_let(&mut self, args: &[SForm], span: Span, tail: bool) -> Result<Ast, Diagnostic> {
        let bindings = match args.first().map(|f| f.node.strip_meta()) {
            Some(Form::Vector(b)) => b,
            _ => return Err(Diagnostic::error("E0104", "let requer vetor de bindings").with_span(span)),
        };
        if bindings.len() % 2 != 0 {
            return Err(Diagnostic::error("E0104", "let: bindings em pares").with_span(span));
        }
        let saved_locals = self.locals.len();
        let saved_next = self.next_slot;
        let mut slots = Vec::new();
        for pair in bindings.chunks_exact(2) {
            let name = match pair[0].node.strip_meta() {
                Form::Symbol(n) if n.ns.is_none() => n.name.clone(),
                _ => return Err(unsupported("let: binding deve ser símbolo simples (sem destructuring no slice)", pair[0].span)),
            };
            let val = self.analyze(&pair[1], false)?;
            let slot = self.push_local(name);
            slots.push((slot, val));
        }
        let body = self.analyze_body(&args[1..], span, tail)?;
        self.locals.truncate(saved_locals);
        self.next_slot = saved_next;
        Ok(Ast::Let { slots, body: Box::new(body) })
    }

    fn analyze_loop(&mut self, args: &[SForm], span: Span, tail: bool) -> Result<Ast, Diagnostic> {
        let bindings = match args.first().map(|f| f.node.strip_meta()) {
            Some(Form::Vector(b)) => b,
            _ => return Err(Diagnostic::error("E0106", "loop requer vetor de bindings").with_span(span)),
        };
        if bindings.len() % 2 != 0 {
            return Err(Diagnostic::error("E0106", "loop: bindings em pares").with_span(span));
        }
        let saved_locals = self.locals.len();
        let saved_next = self.next_slot;
        let mut slots = Vec::new();
        for pair in bindings.chunks_exact(2) {
            let name = match pair[0].node.strip_meta() {
                Form::Symbol(n) if n.ns.is_none() => n.name.clone(),
                _ => return Err(unsupported("loop: binding deve ser símbolo simples (sem destructuring)", pair[0].span)),
            };
            let val = self.analyze(&pair[1], false)?;
            let slot = self.push_local(name);
            slots.push((slot, val));
        }
        // `loop` é o alvo de recur mais interno (mesmo dentro de uma fn).
        self.recur_arity.push(slots.len());
        let body = self.analyze_body(&args[1..], span, tail);
        self.recur_arity.pop();
        let body = body?;
        self.locals.truncate(saved_locals);
        self.next_slot = saved_next;
        Ok(Ast::Loop { slots, body: Box::new(body) })
    }

    fn analyze_recur(&mut self, args: &[SForm], span: Span, tail: bool) -> Result<Ast, Diagnostic> {
        let Some(&arity) = self.recur_arity.last() else {
            return Err(Diagnostic::error("E0107", "recur fora de loop/fn").with_span(span));
        };
        if !tail {
            return Err(Diagnostic::error("E0108", "recur não está em posição de cauda")
                .with_span(span)
                .with_help("recur só pode ser a última expressão de uma fn ou loop"));
        }
        if args.len() != arity {
            return Err(Diagnostic::error(
                "E0109",
                format!("recur: esperava {arity} argumentos, recebeu {}", args.len()),
            )
            .with_span(span));
        }
        Ok(Ast::Recur(self.analyze_seq(args)?))
    }
}

fn prim_of(name: &str) -> Option<Prim> {
    Some(match name {
        "+" => Prim::Add,
        "-" => Prim::Sub,
        "*" => Prim::Mul,
        "quot" => Prim::Quot,
        "mod" => Prim::Mod,
        "inc" => Prim::Inc,
        "dec" => Prim::Dec,
        "=" => Prim::Eq,
        "<" => Prim::Lt,
        "<=" => Prim::Le,
        ">" => Prim::Gt,
        ">=" => Prim::Ge,
        "not" => Prim::Not,
        "nil?" => Prim::NilP,
        "empty?" => Prim::EmptyP,
        "cons" => Prim::Cons,
        "first" => Prim::First,
        "rest" => Prim::Rest,
        "count" => Prim::Count,
        "list" => Prim::List,
        "str" => Prim::Str,
        "println" => Prim::Println,
        "print" => Prim::Print,
        _ => return None,
    })
}

fn check_prim_arity(prim: Prim, n: usize, span: Span) -> Result<(), Diagnostic> {
    let ok = match prim {
        Prim::Sub => n >= 1,
        Prim::Add | Prim::Mul => n >= 1,
        Prim::Quot | Prim::Mod | Prim::Cons => n == 2,
        Prim::Inc | Prim::Dec | Prim::Not | Prim::NilP | Prim::EmptyP | Prim::First
        | Prim::Rest | Prim::Count => n == 1,
        // Comparações binárias no slice (encadeamento é [FUTURO]).
        Prim::Eq | Prim::Lt | Prim::Le | Prim::Gt | Prim::Ge => n == 2,
        Prim::List | Prim::Str | Prim::Println | Prim::Print => true,
    };
    if ok {
        Ok(())
    } else {
        Err(Diagnostic::error("E0105", format!("aridade inválida para primitiva ({n} args)")).with_span(span))
    }
}

fn unsupported(msg: impl Into<String>, span: Span) -> Diagnostic {
    Diagnostic::error("E0100", msg)
        .with_span(span)
        .with_help("fora do subconjunto compilável atual; ver specs/LANGUAGE_SCOPE.md e IMPLEMENTATION_PLAN.md (Fase 5)")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prog(src: &str) -> Program {
        let forms = clojure_reader::read_all(0, src).expect("lê");
        analyze(&forms).expect("analisa")
    }

    #[test]
    fn analyzes_fn_and_main() {
        let p = prog("(ns h.core)\n(defn fib [n] (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))\n(defn -main [] (println \"fib\" (fib 10)))\n(-main)");
        assert_eq!(p.functions.len(), 2);
        assert_eq!(p.main_body.len(), 1);
        let fib = p.functions.iter().find(|f| f.name == "fib").unwrap();
        assert_eq!(fib.params, vec!["n"]);
        assert_eq!(fib.local_count, 1);
    }

    #[test]
    fn let_slots() {
        let p = prog("(defn f [a] (let [b (+ a 1) c (* b 2)] (+ b c)))");
        let f = &p.functions[0];
        assert_eq!(f.local_count, 3); // a, b, c
    }

    #[test]
    fn unresolved_is_error() {
        let forms = clojure_reader::read_all(0, "(defn f [] (nope 1))").unwrap();
        let e = analyze(&forms).unwrap_err();
        assert_eq!(e.items[0].code, "E0101");
    }

    #[test]
    fn unsupported_is_diagnosed() {
        let forms = clojure_reader::read_all(0, "(defn f [] (fn [x] x))").unwrap();
        assert_eq!(analyze(&forms).unwrap_err().items[0].code, "E0100");
    }
}
