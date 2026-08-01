//! Transient-accumulator auto-optimization: rewrites eligible persistent
//! accumulation loops into transient/`conj!`-based ones (ADR-0009/0010).

use crate::ast::*;
use std::collections::HashMap;

pub(crate) fn ast_is_local(e: &Ast, s: u32) -> bool {
    matches!(e, Ast::Local(x) if *x == s)
}
/// Tests whether `e` is slot `s` or a persistent update chain rooted at `s`.
pub(crate) fn s_derived(e: &Ast, s: u32) -> bool {
    match e {
        Ast::Local(x) => *x == s,
        Ast::Call {
            callee: Callee::Prim(Prim::Conj | Prim::Assoc),
            args,
        } => args.first().is_some_and(|r| s_derived(r, s)),
        _ => false,
    }
}
/// Function-name to linearly consumed parameter-index summary.
pub(crate) type LinMap = HashMap<String, usize>;
/// Tests whether `e` calls a summarized function with `s` in its linear position.
pub(crate) fn is_lin_call(e: &Ast, s: u32, lin: &LinMap) -> bool {
    if let Ast::Call {
        callee: Callee::Fn(name),
        args,
    } = e
    {
        if let Some(&j) = lin.get(name) {
            return args.get(j).is_some_and(|a| ast_is_local(a, s));
        }
    }
    false
}
/// Tests whether `e` produces the next representation of transient slot `s`.
pub(crate) fn transient_producing(e: &Ast, s: u32, lin: &LinMap) -> bool {
    s_derived(e, s) || is_lin_call(e, s, lin)
}
/// Verifies that every occurrence of slot `s` occupies an accepted linear use.
///
/// `tail` permits a derived value to escape or rebind here. `pos` is the slot's
/// position in the loop binding vector and therefore in each [`Ast::Recur`].
pub(crate) fn linear_ok(e: &Ast, s: u32, pos: usize, tail: bool, lin: &LinMap) -> bool {
    match e {
        Ast::Local(x) => *x != s || tail, // A bare `s` is valid only as a tail transfer.
        Ast::Int(_)
        | Ast::Float(_)
        | Ast::Bool(_)
        | Ast::Nil
        | Ast::Str(_)
        | Ast::Keyword(_)
        | Ast::Capture(_)
        | Ast::GlobalRef(_) // reads a global; independent of slot s
        | Ast::FnRef(_) => true,
        Ast::Call {
            callee: Callee::Prim(p),
            args,
        } if matches!(p, Prim::Conj | Prim::Assoc)
            && args.first().is_some_and(|r| s_derived(r, s)) =>
        {
            // A rooted update is valid only in tail position. Remaining
            // arguments may read `s` through recognized operations.
            tail && args[1..].iter().all(|a| linear_ok(a, s, pos, false, lin))
        }
        Ast::Call {
            callee: Callee::Prim(p),
            args,
        } if matches!(p, Prim::Nth | Prim::Get | Prim::Count | Prim::Contains)
            && args.first().is_some_and(|r| ast_is_local(r, s)) =>
        {
            // Recognized reads may occur in any expression position.
            args[1..].iter().all(|a| linear_ok(a, s, pos, false, lin))
        }
        // ADR-0010: a summarized call consumes `s` in one parameter and is
        // valid only in tail position, where its result becomes the next `s`.
        // Other arguments may read but not escape or mutate `s`. Runtime
        // conj/assoc dispatch threads the transient representation.
        Ast::Call {
            callee: Callee::Fn(name),
            args,
        } if lin
            .get(name)
            .is_some_and(|&j| args.get(j).is_some_and(|a| ast_is_local(a, s))) =>
        {
            let j = lin[name];
            tail && args
                .iter()
                .enumerate()
                .all(|(i, a)| i == j || linear_ok(a, s, pos, false, lin))
        }
        Ast::Call { args, .. } => args.iter().all(|a| linear_ok(a, s, pos, false, lin)),
        Ast::CallValue { f, args } => {
            linear_ok(f, s, pos, false, lin)
                && args.iter().all(|a| linear_ok(a, s, pos, false, lin))
        }
        Ast::Apply { f, fixed, coll } => {
            linear_ok(f, s, pos, false, lin)
                && fixed.iter().all(|a| linear_ok(a, s, pos, false, lin))
                && linear_ok(coll, s, pos, false, lin)
        }
        Ast::Recur(args) => args
            .iter()
            .enumerate()
            .all(|(i, a)| linear_ok(a, s, pos, i == pos, lin)),
        Ast::If(t, then, els) => {
            linear_ok(t, s, pos, false, lin)
                && linear_ok(then, s, pos, tail, lin)
                && linear_ok(els, s, pos, tail, lin)
        }
        Ast::Do(stmts) => stmts
            .iter()
            .enumerate()
            .all(|(i, st)| linear_ok(st, s, pos, tail && i + 1 == stmts.len(), lin)),
        Ast::Let { slots, body } => {
            slots.iter().all(|(_, i)| linear_ok(i, s, pos, false, lin))
                && linear_ok(body, s, pos, tail, lin)
        }
        Ast::Loop { slots, body } => {
            slots.iter().all(|(_, i)| linear_ok(i, s, pos, false, lin))
                && linear_ok(body, s, pos, false, lin)
        }
        Ast::MakeFn { captures, .. } => captures.iter().all(|c| linear_ok(c, s, pos, false, lin)),
        Ast::MakeRecord { fields, .. } => {
            fields.iter().all(|(_, v)| linear_ok(v, s, pos, false, lin))
        }
        Ast::VecLit(items) | Ast::SetLit(items) => {
            items.iter().all(|i| linear_ok(i, s, pos, false, lin))
        }
        Ast::MapLit(pairs) => pairs
            .iter()
            .all(|(k, v)| linear_ok(k, s, pos, false, lin) && linear_ok(v, s, pos, false, lin)),
        Ast::RegisterMethod { key, impl_fn, .. } => {
            linear_ok(key, s, pos, false, lin) && linear_ok(impl_fn, s, pos, false, lin)
        }
        Ast::RegisterMulti { dispatch_fn, .. } => linear_ok(dispatch_fn, s, pos, false, lin),
        // Top-level only; never appears inside an analyzed loop body.
        Ast::DefGlobal { value, .. } => linear_ok(value, s, pos, false, lin),
    }
}
/// Counts occurrences of local slot `s` throughout an AST.
pub(crate) fn count_local(e: &Ast, s: u32) -> usize {
    let mut n = 0;
    pub(crate) fn go(e: &Ast, s: u32, n: &mut usize) {
        match e {
            Ast::Local(x) => {
                if *x == s {
                    *n += 1;
                }
            }
            Ast::If(a, b, c) => {
                go(a, s, n);
                go(b, s, n);
                go(c, s, n);
            }
            Ast::Do(v) | Ast::VecLit(v) | Ast::SetLit(v) | Ast::Recur(v) => {
                v.iter().for_each(|x| go(x, s, n))
            }
            Ast::Let { slots, body } | Ast::Loop { slots, body } => {
                slots.iter().for_each(|(_, i)| go(i, s, n));
                go(body, s, n);
            }
            Ast::Call { args, .. } => args.iter().for_each(|a| go(a, s, n)),
            Ast::CallValue { f, args } => {
                go(f, s, n);
                args.iter().for_each(|a| go(a, s, n));
            }
            Ast::Apply { f, fixed, coll } => {
                go(f, s, n);
                fixed.iter().for_each(|a| go(a, s, n));
                go(coll, s, n);
            }
            Ast::MakeFn { captures, .. } => captures.iter().for_each(|c| go(c, s, n)),
            Ast::MakeRecord { fields, .. } => fields.iter().for_each(|(_, v)| go(v, s, n)),
            Ast::MapLit(p) => p.iter().for_each(|(k, v)| {
                go(k, s, n);
                go(v, s, n);
            }),
            Ast::RegisterMethod { key, impl_fn, .. } => {
                go(key, s, n);
                go(impl_fn, s, n);
            }
            Ast::RegisterMulti { dispatch_fn, .. } => go(dispatch_fn, s, n),
            _ => {}
        }
    }
    go(e, s, &mut n);
    n
}
/// Tests whether every terminating loop path returns a value derived from `s`.
///
/// `recur` is a back edge, not a terminating path.
pub(crate) fn all_escapes_derived(e: &Ast, s: u32, lin: &LinMap) -> bool {
    match e {
        Ast::Recur(_) => true,
        Ast::If(_, then, els) => {
            all_escapes_derived(then, s, lin) && all_escapes_derived(els, s, lin)
        }
        Ast::Do(stmts) => stmts.last().is_none_or(|l| all_escapes_derived(l, s, lin)),
        Ast::Let { body, .. } => all_escapes_derived(body, s, lin),
        other => transient_producing(other, s, lin),
    }
}
/// Infers the parameter a function consumes linearly and returns.
///
/// Stage one accepts a single-method non-variadic function whose body is a loop:
/// one accumulator initializes directly from the parameter, that parameter
/// occurs nowhere else, the slot is linear, and every exit returns it.
pub(crate) fn linear_param(m: &FnMethod, lin: &LinMap) -> Option<usize> {
    if m.rest.is_some() {
        return None;
    }
    let Ast::Loop { slots, body } = &m.body else {
        return None;
    };
    for pi in 0..m.params.len() as u32 {
        // Find the loop slot initialized directly from parameter `pi`.
        let Some(acc_pos) = slots.iter().position(|(_, init)| ast_is_local(init, pi)) else {
            continue;
        };
        let acc_slot = slots[acc_pos].0;
        // The parameter may occur only in that initializer.
        if count_local(&m.body, pi) != 1 {
            continue;
        }
        if linear_ok(body, acc_slot, acc_pos, true, lin) && all_escapes_derived(body, acc_slot, lin)
        {
            return Some(pi as usize);
        }
    }
    None
}
/// Rewrites a rooted conj/assoc chain to conj!/assoc!.
pub(crate) fn rewrite_bang(e: &mut Ast, s: u32) {
    if let Ast::Call {
        callee: Callee::Prim(p),
        args,
    } = e
    {
        if matches!(p, Prim::Conj | Prim::Assoc) && args.first().is_some_and(|r| s_derived(r, s)) {
            rewrite_bang(&mut args[0], s);
            *p = if *p == Prim::Conj {
                Prim::ConjBang
            } else {
                Prim::AssocBang
            };
        }
    }
}
/// Rewrites leaf updates in the value transferred to the same recur slot.
pub(crate) fn rewrite_transient_value(e: &mut Ast, s: u32) {
    match e {
        Ast::If(_, then, els) => {
            rewrite_transient_value(then, s);
            rewrite_transient_value(els, s);
        }
        Ast::Do(stmts) => {
            if let Some(last) = stmts.last_mut() {
                rewrite_transient_value(last, s);
            }
        }
        Ast::Let { body, .. } => rewrite_transient_value(body, s),
        _ => rewrite_bang(e, s),
    }
}
/// Applies transient operations and freezes every terminating tail result.
///
/// Recur transfers use mutating updates. Calls summarized as linear thread the
/// transient unchanged because runtime conj/assoc dispatch recognizes `T_TVEC`.
pub(crate) fn transform_body(e: &mut Ast, s: u32, pos: usize, tail: bool, lin: &LinMap) {
    match e {
        Ast::If(_, then, els) => {
            transform_body(then, s, pos, tail, lin);
            transform_body(els, s, pos, tail, lin);
        }
        Ast::Do(stmts) => {
            if let Some(last) = stmts.last_mut() {
                transform_body(last, s, pos, tail, lin);
            }
        }
        Ast::Let { body, .. } => transform_body(body, s, pos, tail, lin),
        Ast::Recur(args) => {
            if let Some(a) = args.get_mut(pos) {
                rewrite_transient_value(a, s);
            }
        }
        other if tail && transient_producing(other, s, lin) => {
            let mut inner = std::mem::replace(other, Ast::Nil);
            rewrite_bang(&mut inner, s);
            *other = Ast::Call {
                callee: Callee::Prim(Prim::PersistentBang),
                args: vec![inner],
            };
        }
        _ => {}
    }
}
pub(crate) fn linearize_loop(slots: &mut [(u32, Ast)], body: &mut Ast, lin: &LinMap) {
    let n = slots.len();
    // INVARIANT: decide all candidates against the original body before any
    // rewrite, preventing one slot's transformation from affecting another.
    let eligible: Vec<usize> = (0..n)
        .filter(|&pos| {
            matches!(slots[pos].1, Ast::VecLit(_)) && linear_ok(body, slots[pos].0, pos, true, lin)
        })
        .collect();
    for pos in eligible {
        let s = slots[pos].0;
        let init = std::mem::replace(&mut slots[pos].1, Ast::Nil);
        slots[pos].1 = Ast::Call {
            callee: Callee::Prim(Prim::Transient),
            args: vec![init],
        };
        transform_body(body, s, pos, true, lin);
    }
}
/// Walks bottom-up and linearizes each loop using `lin` summaries.
pub(crate) fn walk_and_linearize(e: &mut Ast, lin: &LinMap) {
    match e {
        Ast::Loop { slots, body } => {
            for (_, init) in slots.iter_mut() {
                walk_and_linearize(init, lin);
            }
            walk_and_linearize(body, lin);
            linearize_loop(slots, body, lin);
        }
        Ast::If(a, b, c) => {
            walk_and_linearize(a, lin);
            walk_and_linearize(b, lin);
            walk_and_linearize(c, lin);
        }
        Ast::Do(v) | Ast::VecLit(v) | Ast::SetLit(v) | Ast::Recur(v) => {
            v.iter_mut().for_each(|x| walk_and_linearize(x, lin))
        }
        Ast::Let { slots, body } => {
            slots
                .iter_mut()
                .for_each(|(_, i)| walk_and_linearize(i, lin));
            walk_and_linearize(body, lin);
        }
        Ast::Call { args, .. } => args.iter_mut().for_each(|a| walk_and_linearize(a, lin)),
        Ast::CallValue { f, args } => {
            walk_and_linearize(f, lin);
            args.iter_mut().for_each(|a| walk_and_linearize(a, lin));
        }
        Ast::Apply { f, fixed, coll } => {
            walk_and_linearize(f, lin);
            fixed.iter_mut().for_each(|a| walk_and_linearize(a, lin));
            walk_and_linearize(coll, lin);
        }
        Ast::MakeFn { captures, .. } => {
            captures.iter_mut().for_each(|c| walk_and_linearize(c, lin))
        }
        Ast::MakeRecord { fields, .. } => fields
            .iter_mut()
            .for_each(|(_, v)| walk_and_linearize(v, lin)),
        Ast::MapLit(p) => p.iter_mut().for_each(|(k, v)| {
            walk_and_linearize(k, lin);
            walk_and_linearize(v, lin);
        }),
        Ast::RegisterMethod { key, impl_fn, .. } => {
            walk_and_linearize(key, lin);
            walk_and_linearize(impl_fn, lin);
        }
        Ast::RegisterMulti { dispatch_fn, .. } => walk_and_linearize(dispatch_fn, lin),
        _ => {}
    }
}
/// Computes first-stage linearity summaries and rewrites eligible accumulators.
pub(crate) fn optimize_transients(functions: &mut [Function], main_body: &mut [Ast]) {
    // Stage-one summaries are non-recursive and do not chain linear callees.
    let mut lin: LinMap = HashMap::new();
    for f in functions.iter() {
        if f.methods.len() == 1 {
            if let Some(pi) = linear_param(&f.methods[0], &HashMap::new()) {
                lin.insert(f.name.clone(), pi);
            }
        }
    }
    for f in functions.iter_mut() {
        for m in f.methods.iter_mut() {
            walk_and_linearize(&mut m.body, &lin);
        }
    }
    for stmt in main_body.iter_mut() {
        walk_and_linearize(stmt, &lin);
    }
}
