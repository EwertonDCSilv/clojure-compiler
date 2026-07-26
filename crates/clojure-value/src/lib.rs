//! Representação de valores do **interpretador de bootstrap** (Fase 2).
//!
//! ADR-0003 fixa `enum Value` como representação. ADR-0002 permite `Rc` no
//! interpretador de bootstrap (não é o runtime de produção com GC). As coleções
//! aqui são simplificadas (lista realmente persistente via cons; vetor/mapa via
//! `Rc<Vec>` com copy-on-write) — as coleções persistentes reais (bitmapped trie /
//! HAMT) são das Fases 4/8. Ver specs/RUNTIME_SPEC.md.

use clojure_syntax::{Name, SForm};
use std::fmt;
use std::rc::Rc;

pub use clojure_syntax::Name as SymName;

/// Um valor Clojure em runtime (bootstrap).
#[derive(Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(char),
    Str(Rc<str>),
    Symbol(Rc<Name>),
    Keyword(Rc<Name>),
    List(Rc<List>),
    Vector(Rc<Vec<Value>>),
    /// array-map: pares ordenados por inserção (bootstrap).
    Map(Rc<Vec<(Value, Value)>>),
    Set(Rc<Vec<Value>>),
    Fn(Rc<Closure>),
    Native(Rc<NativeFn>),
}

/// Lista persistente (cons cells imutáveis).
pub enum List {
    Empty,
    Cons { head: Value, tail: Rc<List>, count: usize },
}

impl List {
    pub fn empty() -> Rc<List> {
        Rc::new(List::Empty)
    }

    pub fn count(&self) -> usize {
        match self {
            List::Empty => 0,
            List::Cons { count, .. } => *count,
        }
    }

    pub fn cons(head: Value, tail: Rc<List>) -> Rc<List> {
        let count = tail.count() + 1;
        Rc::new(List::Cons { head, tail, count })
    }

    pub fn from_vec(items: Vec<Value>) -> Rc<List> {
        let mut acc = List::empty();
        for v in items.into_iter().rev() {
            acc = List::cons(v, acc);
        }
        acc
    }

    pub fn iter(&self) -> ListIter<'_> {
        ListIter { cur: self }
    }
}

pub struct ListIter<'a> {
    cur: &'a List,
}

impl<'a> Iterator for ListIter<'a> {
    type Item = &'a Value;
    fn next(&mut self) -> Option<&'a Value> {
        match self.cur {
            List::Empty => None,
            List::Cons { head, tail, .. } => {
                self.cur = tail;
                Some(head)
            }
        }
    }
}

/// Método de uma função (uma aridade).
pub struct FnMethod {
    pub params: Vec<String>,
    pub rest: Option<String>,
    pub body: Vec<SForm>,
}

impl FnMethod {
    pub fn arity_matches(&self, n: usize) -> bool {
        if self.rest.is_some() {
            n >= self.params.len()
        } else {
            n == self.params.len()
        }
    }
}

/// Closure interpretada: métodos (aridades) + ambiente léxico capturado.
pub struct Closure {
    pub name: Option<String>,
    pub methods: Vec<FnMethod>,
    pub env: Option<Rc<Scope>>,
}

impl Closure {
    pub fn method_for(&self, n: usize) -> Option<&FnMethod> {
        // Preferir aridade fixa exata; senão, variádica compatível.
        self.methods
            .iter()
            .find(|m| m.rest.is_none() && m.params.len() == n)
            .or_else(|| self.methods.iter().find(|m| m.arity_matches(n)))
    }
}

/// Função nativa (implementada em Rust). Para o bootstrap, erros são `String`.
pub struct NativeFn {
    pub name: String,
    pub f: Box<dyn Fn(&[Value]) -> Result<Value, String>>,
}

impl NativeFn {
    pub fn new(name: impl Into<String>, f: impl Fn(&[Value]) -> Result<Value, String> + 'static) -> Rc<NativeFn> {
        Rc::new(NativeFn { name: name.into(), f: Box::new(f) })
    }
}

/// Ambiente léxico imutável (cadeia de frames). Locais do interpretador.
pub struct Scope {
    bindings: Vec<(String, Value)>,
    parent: Option<Rc<Scope>>,
}

impl Scope {
    pub fn child(parent: Option<Rc<Scope>>, bindings: Vec<(String, Value)>) -> Rc<Scope> {
        Rc::new(Scope { bindings, parent })
    }

    pub fn lookup(&self, name: &str) -> Option<Value> {
        if let Some((_, v)) = self.bindings.iter().rev().find(|(n, _)| n == name) {
            return Some(v.clone());
        }
        self.parent.as_ref().and_then(|p| p.lookup(name))
    }
}

impl Value {
    pub fn str(s: impl Into<Rc<str>>) -> Value {
        Value::Str(s.into())
    }

    pub fn symbol(name: Name) -> Value {
        Value::Symbol(Rc::new(name))
    }

    pub fn keyword(name: Name) -> Value {
        Value::Keyword(Rc::new(name))
    }

    /// `nil` e `false` são os únicos valores falsos (specs/LANGUAGE_SCOPE.md).
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Nil | Value::Bool(false))
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Bool(_) => "boolean",
            Value::Int(_) => "integer",
            Value::Float(_) => "float",
            Value::Char(_) => "char",
            Value::Str(_) => "string",
            Value::Symbol(_) => "symbol",
            Value::Keyword(_) => "keyword",
            Value::List(_) => "list",
            Value::Vector(_) => "vector",
            Value::Map(_) => "map",
            Value::Set(_) => "set",
            Value::Fn(_) => "function",
            Value::Native(_) => "function",
        }
    }

    pub fn is_callable(&self) -> bool {
        matches!(self, Value::Fn(_) | Value::Native(_) | Value::Keyword(_))
    }
}

/// Igualdade de valor estrutural (`=` de Clojure). `nil`/`false` distintos;
/// categorias numéricas distintas (`(= 1 1.0)` → false).
impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        use Value::*;
        match (self, other) {
            (Nil, Nil) => true,
            (Bool(a), Bool(b)) => a == b,
            (Int(a), Int(b)) => a == b,
            (Float(a), Float(b)) => a == b,
            (Char(a), Char(b)) => a == b,
            (Str(a), Str(b)) => a == b,
            (Symbol(a), Symbol(b)) => a == b,
            (Keyword(a), Keyword(b)) => a == b,
            (List(a), List(b)) => a.iter().eq(b.iter()),
            (Vector(a), Vector(b)) => a == b,
            (Set(a), Set(b)) => a.len() == b.len() && a.iter().all(|x| b.contains(x)),
            (Map(a), Map(b)) => {
                a.len() == b.len()
                    && a.iter().all(|(k, v)| b.iter().any(|(k2, v2)| k == k2 && v == v2))
            }
            // Sequências entre list e vector comparam como sequências.
            (List(a), Vector(b)) => a.iter().eq(b.iter()),
            (Vector(a), List(b)) => a.iter().eq(b.iter()),
            _ => false,
        }
    }
}

/// Impressão estilo `pr-str` (com aspas/escapes), determinística.
pub fn pr_str(v: &Value) -> String {
    let mut s = String::new();
    write_value(&mut s, v, true);
    s
}

/// Impressão estilo `print`/`str` (sem aspas em strings/chars).
pub fn print_str(v: &Value) -> String {
    let mut s = String::new();
    write_value(&mut s, v, false);
    s
}

fn write_value(out: &mut String, v: &Value, readable: bool) {
    use std::fmt::Write as _;
    match v {
        Value::Nil => out.push_str("nil"),
        Value::Bool(b) => {
            let _ = write!(out, "{b}");
        }
        Value::Int(n) => {
            let _ = write!(out, "{n}");
        }
        Value::Float(x) => {
            if x.fract() == 0.0 && x.is_finite() {
                let _ = write!(out, "{x:.1}");
            } else {
                let _ = write!(out, "{x}");
            }
        }
        Value::Char(c) => {
            if readable {
                let _ = write!(out, "\\{}", char_name(*c));
            } else {
                out.push(*c);
            }
        }
        Value::Str(s) => {
            if readable {
                out.push('"');
                for c in s.chars() {
                    match c {
                        '"' => out.push_str("\\\""),
                        '\\' => out.push_str("\\\\"),
                        '\n' => out.push_str("\\n"),
                        '\t' => out.push_str("\\t"),
                        '\r' => out.push_str("\\r"),
                        c => out.push(c),
                    }
                }
                out.push('"');
            } else {
                out.push_str(s);
            }
        }
        Value::Symbol(n) => {
            let _ = write!(out, "{n}");
        }
        Value::Keyword(n) => {
            let _ = write!(out, ":{n}");
        }
        Value::List(l) => {
            out.push('(');
            for (i, it) in l.iter().enumerate() {
                if i > 0 {
                    out.push(' ');
                }
                write_value(out, it, readable);
            }
            out.push(')');
        }
        Value::Vector(v) => {
            out.push('[');
            for (i, it) in v.iter().enumerate() {
                if i > 0 {
                    out.push(' ');
                }
                write_value(out, it, readable);
            }
            out.push(']');
        }
        Value::Set(v) => {
            out.push_str("#{");
            for (i, it) in v.iter().enumerate() {
                if i > 0 {
                    out.push(' ');
                }
                write_value(out, it, readable);
            }
            out.push('}');
        }
        Value::Map(pairs) => {
            out.push('{');
            for (i, (k, val)) in pairs.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                write_value(out, k, readable);
                out.push(' ');
                write_value(out, val, readable);
            }
            out.push('}');
        }
        Value::Fn(c) => {
            let _ = write!(out, "#<fn {}>", c.name.as_deref().unwrap_or("anonymous"));
        }
        Value::Native(n) => {
            let _ = write!(out, "#<native {}>", n.name);
        }
    }
}

fn char_name(c: char) -> String {
    match c {
        '\n' => "newline".into(),
        '\t' => "tab".into(),
        ' ' => "space".into(),
        '\r' => "return".into(),
        c => c.to_string(),
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&pr_str(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truthiness() {
        assert!(!Value::Nil.is_truthy());
        assert!(!Value::Bool(false).is_truthy());
        assert!(Value::Bool(true).is_truthy());
        assert!(Value::Int(0).is_truthy());
        assert!(Value::str("").is_truthy());
    }

    #[test]
    fn equality() {
        assert_eq!(Value::Int(1), Value::Int(1));
        assert_ne!(Value::Int(1), Value::Float(1.0));
        assert_ne!(Value::Nil, Value::Bool(false));
        let l = Value::List(List::from_vec(vec![Value::Int(1), Value::Int(2)]));
        let v = Value::Vector(Rc::new(vec![Value::Int(1), Value::Int(2)]));
        assert_eq!(l, v); // sequências iguais
    }

    #[test]
    fn printing() {
        assert_eq!(pr_str(&Value::str("hi\n")), "\"hi\\n\"");
        assert_eq!(print_str(&Value::str("hi")), "hi");
        assert_eq!(pr_str(&Value::Float(1.0)), "1.0");
        let l = Value::List(List::from_vec(vec![Value::Int(1), Value::Int(2)]));
        assert_eq!(pr_str(&l), "(1 2)");
    }
}
