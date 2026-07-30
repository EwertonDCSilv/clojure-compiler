//! Values owned by the bootstrap interpreter.
//!
//! [`Value`] is a safe Rust representation used only while interpreting source
//! during bootstrap. It is not the tagged 64-bit `Value` ABI used by generated
//! native code and the C runtime. Sharing uses [`Rc`]; lists are immutable cons
//! cells, while vectors, maps, and sets use copy-on-write vectors suitable for
//! the bootstrap path. Native persistent data structures and garbage collection
//! belong to `clojure-codegen` and its C runtime.

use clojure_syntax::{Name, SForm};
use std::fmt;
use std::rc::Rc;

/// Backwards-compatible exported name for syntax-level names.
pub use clojure_syntax::Name as SymName;

/// Runtime value understood by the bootstrap interpreter.
#[derive(Clone)]
pub enum Value {
    /// The singleton `nil` value.
    Nil,
    /// A boolean value.
    Bool(bool),
    /// A signed 64-bit integer.
    Int(i64),
    /// An IEEE-754 double-precision number.
    Float(f64),
    /// A Unicode scalar value.
    Char(char),
    /// An immutable UTF-8 string.
    Str(Rc<str>),
    /// A symbolic name treated as data.
    Symbol(Rc<Name>),
    /// An internable keyword name, stored directly during bootstrap.
    Keyword(Rc<Name>),
    /// An immutable singly linked list.
    List(Rc<List>),
    /// A copy-on-write vector.
    Vector(Rc<Vec<Value>>),
    /// Bootstrap array map whose pairs preserve insertion order.
    Map(Rc<Vec<(Value, Value)>>),
    /// Bootstrap set whose elements preserve insertion order.
    Set(Rc<Vec<Value>>),
    /// An interpreted closure with one or more arities.
    Fn(Rc<Closure>),
    /// A primitive implemented in Rust.
    Native(Rc<NativeFn>),
}

/// Persistent list built from immutable cons cells.
pub enum List {
    /// Empty list.
    Empty,
    /// One head value followed by a shared tail.
    Cons {
        /// First value in this cell.
        head: Value,
        /// Remaining immutable list.
        tail: Rc<List>,
        /// Cached number of cells from this node onward.
        count: usize,
    },
}

impl List {
    /// Creates an empty shared list.
    pub fn empty() -> Rc<List> {
        Rc::new(List::Empty)
    }

    /// Returns the cached number of elements in `O(1)`.
    pub fn count(&self) -> usize {
        match self {
            List::Empty => 0,
            List::Cons { count, .. } => *count,
        }
    }

    /// Prepends `head` to `tail` without modifying the tail.
    pub fn cons(head: Value, tail: Rc<List>) -> Rc<List> {
        let count = tail.count() + 1;
        Rc::new(List::Cons { head, tail, count })
    }

    /// Converts a vector to a list while preserving iteration order.
    ///
    /// # Examples
    ///
    /// ```
    /// use clojure_value::{List, Value};
    ///
    /// let list = List::from_vec(vec![Value::Int(1), Value::Int(2)]);
    /// assert_eq!(list.iter().cloned().collect::<Vec<_>>(), vec![
    ///     Value::Int(1),
    ///     Value::Int(2),
    /// ]);
    /// ```
    pub fn from_vec(items: Vec<Value>) -> Rc<List> {
        let mut acc = List::empty();
        for v in items.into_iter().rev() {
            acc = List::cons(v, acc);
        }
        acc
    }

    /// Iterates from the list head to its empty tail.
    pub fn iter(&self) -> ListIter<'_> {
        ListIter { cur: self }
    }
}

/// Borrowing iterator over an immutable [`List`].
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

/// One arity implementation of an interpreted function.
pub struct FnMethod {
    /// Fixed parameter names in binding order.
    pub params: Vec<String>,
    /// Optional rest parameter bound to arguments after the fixed parameters.
    pub rest: Option<String>,
    /// Function body evaluated in order.
    pub body: Vec<SForm>,
}

impl FnMethod {
    /// Returns whether this method accepts `n` arguments.
    pub fn arity_matches(&self, n: usize) -> bool {
        if self.rest.is_some() {
            n >= self.params.len()
        } else {
            n == self.params.len()
        }
    }
}

/// Interpreted function methods and their captured lexical environment.
pub struct Closure {
    /// Optional name used for self-reference and printed representations.
    pub name: Option<String>,
    /// Fixed and variadic arity implementations.
    pub methods: Vec<FnMethod>,
    /// Lexical environment captured when the function was created.
    pub env: Option<Rc<Scope>>,
}

impl Closure {
    /// Selects the implementation for `n` arguments.
    ///
    /// Exact fixed arity takes precedence over a matching variadic method,
    /// independent of declaration order.
    pub fn method_for(&self, n: usize) -> Option<&FnMethod> {
        // Prefer exact fixed arity before a compatible variadic method.
        self.methods
            .iter()
            .find(|m| m.rest.is_none() && m.params.len() == n)
            .or_else(|| self.methods.iter().find(|m| m.arity_matches(n)))
    }
}

/// Callable signature used by Rust bootstrap primitives.
///
/// Primitive failures use user-facing strings; `clojure-interp` attaches the
/// call-site span at the interpreter boundary.
pub type NativeCallable = dyn Fn(&[Value]) -> Result<Value, String>;

/// Named primitive implemented in Rust.
pub struct NativeFn {
    /// Name used in printed values and error context.
    pub name: String,
    /// Primitive implementation.
    pub f: Box<NativeCallable>,
}

impl NativeFn {
    /// Wraps a Rust closure as a shared bootstrap primitive.
    pub fn new(
        name: impl Into<String>,
        f: impl Fn(&[Value]) -> Result<Value, String> + 'static,
    ) -> Rc<NativeFn> {
        Rc::new(NativeFn {
            name: name.into(),
            f: Box::new(f),
        })
    }
}

/// Immutable lexical frame linked to an optional parent frame.
///
/// Bindings are searched from the end so a later binding shadows an earlier
/// binding in the same frame before lookup continues in the parent.
pub struct Scope {
    bindings: Vec<(String, Value)>,
    parent: Option<Rc<Scope>>,
}

impl Scope {
    /// Creates a lexical frame with an optional parent.
    pub fn child(parent: Option<Rc<Scope>>, bindings: Vec<(String, Value)>) -> Rc<Scope> {
        Rc::new(Scope { bindings, parent })
    }

    /// Finds and clones the nearest value bound to `name`.
    pub fn lookup(&self, name: &str) -> Option<Value> {
        if let Some((_, v)) = self.bindings.iter().rev().find(|(n, _)| n == name) {
            return Some(v.clone());
        }
        self.parent.as_ref().and_then(|p| p.lookup(name))
    }
}

impl Value {
    /// Creates an immutable string value.
    pub fn str(s: impl Into<Rc<str>>) -> Value {
        Value::Str(s.into())
    }

    /// Creates a symbol value from a syntax name.
    pub fn symbol(name: Name) -> Value {
        Value::Symbol(Rc::new(name))
    }

    /// Creates a keyword value from a syntax name.
    pub fn keyword(name: Name) -> Value {
        Value::Keyword(Rc::new(name))
    }

    /// Returns Clojure truthiness: only `nil` and `false` are false.
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Nil | Value::Bool(false))
    }

    /// Returns the bootstrap category name used in diagnostics.
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

    /// Returns whether the interpreter can invoke this value.
    ///
    /// Interpreted functions, native primitives, and keywords are callable.
    pub fn is_callable(&self) -> bool {
        matches!(self, Value::Fn(_) | Value::Native(_) | Value::Keyword(_))
    }
}

/// Implements bootstrap structural equality.
///
/// `nil` and `false` remain distinct, numeric categories do not widen
/// (`1 != 1.0`), lists and vectors compare sequentially, and map or set
/// insertion order is ignored.
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
                    && a.iter()
                        .all(|(k, v)| b.iter().any(|(k2, v2)| k == k2 && v == v2))
            }
            // Lists and vectors share sequential equality in the bootstrap.
            (List(a), Vector(b)) => a.iter().eq(b.iter()),
            (Vector(a), List(b)) => a.iter().eq(b.iter()),
            _ => false,
        }
    }
}

/// Returns deterministic, readable `pr-str`-style text.
///
/// Strings and characters include delimiters or names and escape control
/// characters.
pub fn pr_str(v: &Value) -> String {
    let mut s = String::new();
    write_value(&mut s, v, true);
    s
}

/// Returns deterministic `print`/`str`-style text.
///
/// Strings and characters are emitted without reader delimiters.
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
#[path = "../tests/unit/lib/mod.rs"]
mod tests;
