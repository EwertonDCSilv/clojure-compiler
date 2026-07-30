//! Rust primitives installed into the bootstrap `clojure.core`.
//!
//! This module owns operations that require host arithmetic, collection access,
//! printing, or constructors. Higher-level functions are evaluated from
//! `core.clj`. Primitive errors are Portuguese strings; the interpreter adds the
//! source call-site when it converts them to `EvalError`.

use crate::Interp;
use clojure_value::{pr_str, print_str, List, NativeFn, Value};
use std::rc::Rc;

/// Internal numeric classification for mixed integer/float arithmetic.
#[derive(Clone, Copy)]
enum Num {
    I(i64),
    F(f64),
}

fn as_num(v: &Value) -> Result<Num, String> {
    match v {
        Value::Int(n) => Ok(Num::I(*n)),
        Value::Float(x) => Ok(Num::F(*x)),
        other => Err(format!("esperava número, recebeu {}", other.type_name())),
    }
}

fn num_val(n: Num) -> Value {
    match n {
        Num::I(i) => Value::Int(i),
        Num::F(f) => Value::Float(f),
    }
}

fn to_f(n: Num) -> f64 {
    match n {
        Num::I(i) => i as f64,
        Num::F(f) => f,
    }
}

fn install_fn(
    it: &mut Interp,
    name: &'static str,
    f: impl Fn(&[Value]) -> Result<Value, String> + 'static,
) {
    it.define(name, Value::Native(NativeFn::new(name, f)));
}

pub(crate) fn install(it: &mut Interp) {
    // -- Arithmetic -------------------------------------------------------
    install_fn(it, "+", |a| fold_arith(a, Num::I(0), add));
    install_fn(it, "*", |a| fold_arith(a, Num::I(1), mul));
    install_fn(it, "-", |a| match a.len() {
        0 => Err("- requer ao menos 1 argumento".into()),
        1 => Ok(num_val(neg(as_num(&a[0])?))),
        _ => {
            let mut acc = as_num(&a[0])?;
            for v in &a[1..] {
                acc = sub(acc, as_num(v)?)?;
            }
            Ok(num_val(acc))
        }
    });
    install_fn(it, "/", |a| {
        if a.len() < 2 {
            return Err("/ requer ao menos 2 argumentos".into());
        }
        let mut acc = as_num(&a[0])?;
        for v in &a[1..] {
            acc = div(acc, as_num(v)?)?;
        }
        Ok(num_val(acc))
    });
    install_fn(it, "inc", |a| {
        Ok(num_val(add(as_num(one(a)?)?, Num::I(1))?))
    });
    install_fn(it, "dec", |a| {
        Ok(num_val(sub(as_num(one(a)?)?, Num::I(1))?))
    });
    install_fn(it, "quot", |a| {
        int_binop(a, |x, y| {
            if y == 0 {
                Err("divisão por zero".into())
            } else {
                Ok(x / y)
            }
        })
    });
    install_fn(it, "rem", |a| {
        int_binop(a, |x, y| {
            if y == 0 {
                Err("divisão por zero".into())
            } else {
                Ok(x % y)
            }
        })
    });
    install_fn(it, "mod", |a| {
        int_binop(a, |x, y| {
            if y == 0 {
                Err("divisão por zero".into())
            } else {
                Ok(x.rem_euclid(y))
            }
        })
    });
    install_fn(it, "max", |a| fold_cmp(a, true));
    install_fn(it, "min", |a| fold_cmp(a, false));
    install_fn(it, "abs", |a| match as_num(one(a)?)? {
        Num::I(i) => Ok(Value::Int(i.abs())),
        Num::F(f) => Ok(Value::Float(f.abs())),
    });

    // -- Comparison -------------------------------------------------------
    install_fn(it, "=", |a| Ok(Value::Bool(all_eq(a))));
    install_fn(it, "not=", |a| Ok(Value::Bool(!all_eq(a))));
    install_fn(it, "==", |a| {
        num_chain(a, |o| o == std::cmp::Ordering::Equal)
    });
    install_fn(it, "<", |a| num_chain(a, |o| o == std::cmp::Ordering::Less));
    install_fn(it, ">", |a| {
        num_chain(a, |o| o == std::cmp::Ordering::Greater)
    });
    install_fn(it, "<=", |a| {
        num_chain(a, |o| o != std::cmp::Ordering::Greater)
    });
    install_fn(it, ">=", |a| {
        num_chain(a, |o| o != std::cmp::Ordering::Less)
    });

    // -- Predicates -------------------------------------------------------
    install_fn(it, "not", |a| Ok(Value::Bool(!one(a)?.is_truthy())));
    install_fn(it, "nil?", |a| {
        Ok(Value::Bool(matches!(one(a)?, Value::Nil)))
    });
    install_fn(it, "some?", |a| {
        Ok(Value::Bool(!matches!(one(a)?, Value::Nil)))
    });
    install_fn(it, "true?", |a| {
        Ok(Value::Bool(matches!(one(a)?, Value::Bool(true))))
    });
    install_fn(it, "false?", |a| {
        Ok(Value::Bool(matches!(one(a)?, Value::Bool(false))))
    });
    install_fn(it, "zero?", |a| {
        Ok(Value::Bool(
            matches!(as_num(one(a)?)?, Num::I(0))
                || matches!(as_num(one(a)?)?, Num::F(f) if f == 0.0),
        ))
    });
    install_fn(it, "pos?", |a| {
        Ok(Value::Bool(to_f(as_num(one(a)?)?) > 0.0))
    });
    install_fn(it, "neg?", |a| {
        Ok(Value::Bool(to_f(as_num(one(a)?)?) < 0.0))
    });
    install_fn(it, "even?", |a| Ok(Value::Bool(int1(a)? % 2 == 0)));
    install_fn(it, "odd?", |a| Ok(Value::Bool(int1(a)? % 2 != 0)));
    install_fn(it, "int?", |a| {
        Ok(Value::Bool(matches!(one(a)?, Value::Int(_))))
    });
    install_fn(it, "string?", |a| {
        Ok(Value::Bool(matches!(one(a)?, Value::Str(_))))
    });
    install_fn(it, "keyword?", |a| {
        Ok(Value::Bool(matches!(one(a)?, Value::Keyword(_))))
    });
    install_fn(it, "symbol?", |a| {
        Ok(Value::Bool(matches!(one(a)?, Value::Symbol(_))))
    });
    install_fn(it, "map?", |a| {
        Ok(Value::Bool(matches!(one(a)?, Value::Map(_))))
    });
    install_fn(it, "vector?", |a| {
        Ok(Value::Bool(matches!(one(a)?, Value::Vector(_))))
    });
    install_fn(it, "list?", |a| {
        Ok(Value::Bool(matches!(one(a)?, Value::List(_))))
    });
    install_fn(it, "set?", |a| {
        Ok(Value::Bool(matches!(one(a)?, Value::Set(_))))
    });
    install_fn(it, "fn?", |a| {
        Ok(Value::Bool(matches!(
            one(a)?,
            Value::Fn(_) | Value::Native(_)
        )))
    });
    install_fn(it, "coll?", |a| {
        Ok(Value::Bool(matches!(
            one(a)?,
            Value::List(_) | Value::Vector(_) | Value::Map(_) | Value::Set(_)
        )))
    });
    install_fn(it, "empty?", |a| Ok(Value::Bool(is_empty(one(a)?))));

    // -- Printing ---------------------------------------------------------
    install_fn(it, "str", |a| {
        let mut s = String::new();
        for v in a {
            if !matches!(v, Value::Nil) {
                s.push_str(&print_str(v));
            }
        }
        Ok(Value::str(s))
    });
    install_fn(it, "pr-str", |a| {
        Ok(Value::str(
            a.iter().map(pr_str).collect::<Vec<_>>().join(" "),
        ))
    });
    install_fn(it, "print-str", |a| {
        Ok(Value::str(
            a.iter().map(print_str).collect::<Vec<_>>().join(" "),
        ))
    });
    let out = it.output.clone();
    install_fn(it, "println", move |a| {
        let line = a.iter().map(print_str).collect::<Vec<_>>().join(" ");
        out.borrow_mut().push_str(&line);
        out.borrow_mut().push('\n');
        Ok(Value::Nil)
    });
    let out = it.output.clone();
    install_fn(it, "print", move |a| {
        let s = a.iter().map(print_str).collect::<Vec<_>>().join(" ");
        out.borrow_mut().push_str(&s);
        Ok(Value::Nil)
    });
    let out = it.output.clone();
    install_fn(it, "prn", move |a| {
        let line = a.iter().map(pr_str).collect::<Vec<_>>().join(" ");
        out.borrow_mut().push_str(&line);
        out.borrow_mut().push('\n');
        Ok(Value::Nil)
    });
    let out = it.output.clone();
    install_fn(it, "pr", move |a| {
        let s = a.iter().map(pr_str).collect::<Vec<_>>().join(" ");
        out.borrow_mut().push_str(&s);
        Ok(Value::Nil)
    });

    // -- Collection constructors -----------------------------------------
    install_fn(it, "list", |a| Ok(Value::List(List::from_vec(a.to_vec()))));
    install_fn(it, "vector", |a| Ok(Value::Vector(Rc::new(a.to_vec()))));
    install_fn(it, "vec", |a| {
        Ok(Value::Vector(Rc::new(seq_or_err(one(a)?)?)))
    });
    install_fn(it, "hash-set", |a| {
        let mut v: Vec<Value> = Vec::new();
        for x in a {
            if !v.contains(x) {
                v.push(x.clone());
            }
        }
        Ok(Value::Set(Rc::new(v)))
    });
    install_fn(it, "hash-map", |a| {
        if a.len() % 2 != 0 {
            return Err("hash-map requer número par de argumentos".into());
        }
        let mut pairs: Vec<(Value, Value)> = Vec::new();
        for c in a.chunks_exact(2) {
            assoc_into(&mut pairs, c[0].clone(), c[1].clone());
        }
        Ok(Value::Map(Rc::new(pairs)))
    });

    // -- Collection access ------------------------------------------------
    install_fn(it, "first", |a| Ok(first(one(a)?)));
    install_fn(it, "rest", |a| Ok(rest(one(a)?)));
    install_fn(it, "next", |a| {
        let r = rest(one(a)?);
        Ok(if is_empty(&r) { Value::Nil } else { r })
    });
    install_fn(it, "cons", |a| {
        two(a)?;
        Ok(cons(a[0].clone(), &a[1]))
    });
    install_fn(it, "conj", |a| {
        if a.is_empty() {
            return Err("conj requer ao menos a coleção".into());
        }
        let mut coll = a[0].clone();
        for x in &a[1..] {
            coll = conj(coll, x.clone());
        }
        Ok(coll)
    });
    install_fn(it, "count", |a| Ok(Value::Int(count(one(a)?) as i64)));
    install_fn(it, "nth", |a| {
        two(a)?;
        let idx = match &a[1] {
            Value::Int(i) => *i,
            _ => return Err("nth: índice deve ser inteiro".into()),
        };
        nth(&a[0], idx, a.get(2).cloned())
    });
    install_fn(it, "get", |a| {
        two(a)?;
        let d = a.get(2).cloned().unwrap_or(Value::Nil);
        let r = get(&a[0], &a[1]);
        Ok(if matches!(r, Value::Nil) { d } else { r })
    });
    install_fn(it, "assoc", |a| {
        if a.len() < 3 || a.len() % 2 == 0 {
            return Err("assoc requer coleção e pares chave/valor".into());
        }
        assoc(a)
    });
    install_fn(it, "contains?", |a| {
        two(a)?;
        Ok(Value::Bool(contains(&a[0], &a[1])))
    });
    install_fn(it, "keys", |a| match one(a)? {
        Value::Map(m) => Ok(Value::List(List::from_vec(
            m.iter().map(|(k, _)| k.clone()).collect(),
        ))),
        Value::Nil => Ok(Value::Nil),
        _ => Err("keys requer um mapa".into()),
    });
    install_fn(it, "vals", |a| match one(a)? {
        Value::Map(m) => Ok(Value::List(List::from_vec(
            m.iter().map(|(_, v)| v.clone()).collect(),
        ))),
        Value::Nil => Ok(Value::Nil),
        _ => Err("vals requer um mapa".into()),
    });

    // -- Symbols and keywords --------------------------------------------
    install_fn(it, "name", |a| match one(a)? {
        Value::Keyword(n) | Value::Symbol(n) => Ok(Value::str(n.name.as_str())),
        Value::Str(s) => Ok(Value::Str(s.clone())),
        _ => Err("name requer keyword, símbolo ou string".into()),
    });
    install_fn(it, "keyword", |a| match one(a)? {
        Value::Str(s) => Ok(Value::keyword(clojure_syntax::Name::simple(s.as_ref()))),
        Value::Keyword(n) => Ok(Value::Keyword(n.clone())),
        _ => Err("keyword requer string".into()),
    });
    install_fn(it, "symbol", |a| match one(a)? {
        Value::Str(s) => Ok(Value::symbol(clojure_syntax::Name::simple(s.as_ref()))),
        Value::Symbol(n) => Ok(Value::Symbol(n.clone())),
        _ => Err("symbol requer string".into()),
    });

    // -- Range ------------------------------------------------------------
    install_fn(it, "range", range);
}

// -- Arity helpers --------------------------------------------------------

fn one(a: &[Value]) -> Result<&Value, String> {
    a.first().ok_or_else(|| "esperava 1 argumento".to_string())
}
fn two(a: &[Value]) -> Result<(), String> {
    if a.len() < 2 {
        Err("esperava ao menos 2 argumentos".into())
    } else {
        Ok(())
    }
}
fn int1(a: &[Value]) -> Result<i64, String> {
    match one(a)? {
        Value::Int(n) => Ok(*n),
        _ => Err("esperava inteiro".into()),
    }
}

// -- Arithmetic -----------------------------------------------------------

fn add(a: Num, b: Num) -> Result<Num, String> {
    Ok(match (a, b) {
        (Num::I(x), Num::I(y)) => Num::I(x.checked_add(y).ok_or("overflow em +")?),
        _ => Num::F(to_f(a) + to_f(b)),
    })
}
fn sub(a: Num, b: Num) -> Result<Num, String> {
    Ok(match (a, b) {
        (Num::I(x), Num::I(y)) => Num::I(x.checked_sub(y).ok_or("overflow em -")?),
        _ => Num::F(to_f(a) - to_f(b)),
    })
}
fn mul(a: Num, b: Num) -> Result<Num, String> {
    Ok(match (a, b) {
        (Num::I(x), Num::I(y)) => Num::I(x.checked_mul(y).ok_or("overflow em *")?),
        _ => Num::F(to_f(a) * to_f(b)),
    })
}
fn div(a: Num, b: Num) -> Result<Num, String> {
    match (a, b) {
        (Num::I(_), Num::I(0)) => Err("divisão por zero".into()),
        (Num::I(x), Num::I(y)) => {
            if x % y == 0 {
                Ok(Num::I(x / y))
            } else {
                Err(format!(
                    "ratios não são suportados no MVP: {x}/{y} (ver specs/LANGUAGE_SCOPE.md)"
                ))
            }
        }
        _ => Ok(Num::F(to_f(a) / to_f(b))),
    }
}
fn neg(a: Num) -> Num {
    match a {
        Num::I(i) => Num::I(-i),
        Num::F(f) => Num::F(-f),
    }
}

fn fold_arith(
    a: &[Value],
    init: Num,
    op: fn(Num, Num) -> Result<Num, String>,
) -> Result<Value, String> {
    let mut acc = init;
    for v in a {
        acc = op(acc, as_num(v)?)?;
    }
    Ok(num_val(acc))
}

fn int_binop(a: &[Value], op: fn(i64, i64) -> Result<i64, String>) -> Result<Value, String> {
    two(a)?;
    let (x, y) = match (&a[0], &a[1]) {
        (Value::Int(x), Value::Int(y)) => (*x, *y),
        _ => return Err("operação requer inteiros".into()),
    };
    Ok(Value::Int(op(x, y)?))
}

fn fold_cmp(a: &[Value], want_max: bool) -> Result<Value, String> {
    if a.is_empty() {
        return Err("min/max requer ao menos 1 argumento".into());
    }
    let mut best = as_num(&a[0])?;
    for v in &a[1..] {
        let n = as_num(v)?;
        let take = if want_max {
            to_f(n) > to_f(best)
        } else {
            to_f(n) < to_f(best)
        };
        if take {
            best = n;
        }
    }
    Ok(num_val(best))
}

fn num_chain(a: &[Value], pred: fn(std::cmp::Ordering) -> bool) -> Result<Value, String> {
    for w in a.windows(2) {
        let x = to_f(as_num(&w[0])?);
        let y = to_f(as_num(&w[1])?);
        let ord = x.partial_cmp(&y).ok_or("comparação com NaN")?;
        if !pred(ord) {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

fn all_eq(a: &[Value]) -> bool {
    a.windows(2).all(|w| w[0] == w[1])
}

// -- Collections ----------------------------------------------------------

fn is_empty(v: &Value) -> bool {
    match v {
        Value::Nil => true,
        Value::List(l) => l.count() == 0,
        Value::Vector(v) => v.is_empty(),
        Value::Set(s) => s.is_empty(),
        Value::Map(m) => m.is_empty(),
        Value::Str(s) => s.is_empty(),
        _ => false,
    }
}

fn count(v: &Value) -> usize {
    match v {
        Value::Nil => 0,
        Value::List(l) => l.count(),
        Value::Vector(v) => v.len(),
        Value::Set(s) => s.len(),
        Value::Map(m) => m.len(),
        Value::Str(s) => s.chars().count(),
        _ => 0,
    }
}

fn first(v: &Value) -> Value {
    match v {
        Value::List(l) => l.iter().next().cloned().unwrap_or(Value::Nil),
        Value::Vector(v) => v.first().cloned().unwrap_or(Value::Nil),
        Value::Set(s) => s.first().cloned().unwrap_or(Value::Nil),
        Value::Str(s) => s.chars().next().map(Value::Char).unwrap_or(Value::Nil),
        _ => Value::Nil,
    }
}

fn rest(v: &Value) -> Value {
    let items: Vec<Value> = match v {
        Value::List(l) => l.iter().skip(1).cloned().collect(),
        Value::Vector(v) => v.iter().skip(1).cloned().collect(),
        Value::Set(s) => s.iter().skip(1).cloned().collect(),
        Value::Str(s) => s.chars().skip(1).map(Value::Char).collect(),
        _ => vec![],
    };
    Value::List(List::from_vec(items))
}

fn cons(x: Value, coll: &Value) -> Value {
    match coll {
        Value::List(l) => Value::List(List::cons(x, l.clone())),
        Value::Nil => Value::List(List::cons(x, List::empty())),
        other => {
            let mut items = vec![x];
            if let Some(rest) = seq_items_opt(other) {
                items.extend(rest);
            }
            Value::List(List::from_vec(items))
        }
    }
}

fn conj(coll: Value, x: Value) -> Value {
    match coll {
        Value::Nil => Value::List(List::cons(x, List::empty())),
        Value::List(l) => Value::List(List::cons(x, l)),
        Value::Vector(v) => {
            let mut nv = v.as_ref().clone();
            nv.push(x);
            Value::Vector(Rc::new(nv))
        }
        Value::Set(s) => {
            let mut ns = s.as_ref().clone();
            if !ns.contains(&x) {
                ns.push(x);
            }
            Value::Set(Rc::new(ns))
        }
        Value::Map(m) => {
            // Bootstrap map conj accepts a [key value] vector or another map.
            let mut nm = m.as_ref().clone();
            match &x {
                Value::Vector(kv) if kv.len() == 2 => {
                    assoc_into(&mut nm, kv[0].clone(), kv[1].clone())
                }
                Value::Map(other) => {
                    for (k, v) in other.iter() {
                        assoc_into(&mut nm, k.clone(), v.clone());
                    }
                }
                _ => {}
            }
            Value::Map(Rc::new(nm))
        }
        other => other,
    }
}

fn nth(v: &Value, idx: i64, default: Option<Value>) -> Result<Value, String> {
    if idx < 0 {
        return default.ok_or_else(|| "nth: índice fora dos limites".into());
    }
    let i = idx as usize;
    let got = match v {
        Value::Vector(v) => v.get(i).cloned(),
        Value::List(l) => l.iter().nth(i).cloned(),
        Value::Str(s) => s.chars().nth(i).map(Value::Char),
        _ => None,
    };
    got.or(default)
        .ok_or_else(|| "nth: índice fora dos limites".into())
}

/// Performs bootstrap lookup by map key, vector index, or set member.
pub(crate) fn get(coll: &Value, key: &Value) -> Value {
    match coll {
        Value::Map(m) => m
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
            .unwrap_or(Value::Nil),
        Value::Vector(v) => match key {
            Value::Int(i) if *i >= 0 => v.get(*i as usize).cloned().unwrap_or(Value::Nil),
            _ => Value::Nil,
        },
        Value::Set(s) => {
            if s.contains(key) {
                key.clone()
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn contains(coll: &Value, key: &Value) -> bool {
    match coll {
        Value::Map(m) => m.iter().any(|(k, _)| k == key),
        Value::Set(s) => s.contains(key),
        Value::Vector(v) => matches!(key, Value::Int(i) if *i >= 0 && (*i as usize) < v.len()),
        _ => false,
    }
}

fn assoc_into(pairs: &mut Vec<(Value, Value)>, k: Value, v: Value) {
    if let Some(slot) = pairs.iter_mut().find(|(pk, _)| *pk == k) {
        slot.1 = v;
    } else {
        pairs.push((k, v));
    }
}

fn assoc(a: &[Value]) -> Result<Value, String> {
    match &a[0] {
        Value::Map(_) | Value::Nil => {
            let mut pairs = match &a[0] {
                Value::Map(m) => m.as_ref().clone(),
                _ => Vec::new(),
            };
            for c in a[1..].chunks_exact(2) {
                assoc_into(&mut pairs, c[0].clone(), c[1].clone());
            }
            Ok(Value::Map(Rc::new(pairs)))
        }
        Value::Vector(v) => {
            let mut nv = v.as_ref().clone();
            for c in a[1..].chunks_exact(2) {
                let idx = match &c[0] {
                    Value::Int(i) if *i >= 0 => *i as usize,
                    _ => return Err("assoc em vetor requer índice inteiro válido".into()),
                };
                if idx == nv.len() {
                    nv.push(c[1].clone());
                } else if idx < nv.len() {
                    nv[idx] = c[1].clone();
                } else {
                    return Err("assoc: índice fora dos limites do vetor".into());
                }
            }
            Ok(Value::Vector(Rc::new(nv)))
        }
        other => Err(format!("assoc não suportado em {}", other.type_name())),
    }
}

fn range(a: &[Value]) -> Result<Value, String> {
    let (start, end, step) = match a.len() {
        0 => return Err("range infinito não suportado no bootstrap".into()),
        1 => (0, int_of(&a[0])?, 1),
        2 => (int_of(&a[0])?, int_of(&a[1])?, 1),
        _ => (int_of(&a[0])?, int_of(&a[1])?, int_of(&a[2])?),
    };
    if step == 0 {
        return Err("range: step não pode ser 0".into());
    }
    let mut items = Vec::new();
    let mut i = start;
    while (step > 0 && i < end) || (step < 0 && i > end) {
        items.push(Value::Int(i));
        i += step;
    }
    Ok(Value::List(List::from_vec(items)))
}

fn int_of(v: &Value) -> Result<i64, String> {
    match v {
        Value::Int(n) => Ok(*n),
        _ => Err("esperava inteiro".into()),
    }
}

fn seq_or_err(v: &Value) -> Result<Vec<Value>, String> {
    seq_items_opt(v).ok_or_else(|| format!("não é sequenciável: {}", v.type_name()))
}

fn seq_items_opt(v: &Value) -> Option<Vec<Value>> {
    match v {
        Value::Nil => Some(vec![]),
        Value::List(l) => Some(l.iter().cloned().collect()),
        Value::Vector(v) => Some(v.as_ref().clone()),
        Value::Set(s) => Some(s.as_ref().clone()),
        Value::Str(s) => Some(s.chars().map(Value::Char).collect()),
        _ => None,
    }
}
