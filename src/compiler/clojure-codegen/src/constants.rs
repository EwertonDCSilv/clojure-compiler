//! ABI tag constants shared with the C runtime, and collection of the
//! literal string/keyword constants a program references (materialized as
//! local object data by `compile_object_with_options_and_stats`).

use clojure_analyzer::Ast;

// ABI: tagged constants must match runtime/00_types.c.
pub(crate) const NIL: i64 = 2;
pub(crate) const FALSEV: i64 = 6;
pub(crate) const TRUEV: i64 = 10;
pub(crate) const T_VEC: i64 = 5;
pub(crate) const CONST_CACHE_MAX: usize = 8192; // ABI: matches CONST_MAX in runtime/10_gc.c.
                                                // ABI: PVec/VNode offsets assume a 16-byte Obj and match runtime/00_types.c.
pub(crate) const PV_COUNT: i32 = 16;
pub(crate) const PV_SHIFT: i32 = 24;
pub(crate) const PV_ROOT: i32 = 32;
pub(crate) const PV_TAIL: i32 = 40;
pub(crate) const PV_TAILLEN: i32 = 48;
pub(crate) const VNODE_SLOTS: i32 = 24; // Obj(16) followed by the 8-byte edit token.
                                        // ABI: fixnum range must match FIXNUM_MIN/MAX in runtime/00_types.c.
pub(crate) const FIX_MIN: i64 = -(1 << 62);
pub(crate) const FIX_MAX: i64 = (1 << 62) - 1;

pub(crate) fn collect_strings(ast: &Ast, out: &mut Vec<String>) {
    match ast {
        Ast::Str(s) => out.push(s.clone()),
        Ast::Keyword(s) => out.push(s.clone()),
        Ast::VecLit(items) | Ast::SetLit(items) => {
            items.iter().for_each(|a| collect_strings(a, out))
        }
        Ast::MapLit(pairs) => pairs.iter().for_each(|(k, v)| {
            collect_strings(k, out);
            collect_strings(v, out);
        }),
        Ast::If(a, c, d) => {
            collect_strings(a, out);
            collect_strings(c, out);
            collect_strings(d, out);
        }
        Ast::Do(v) => v.iter().for_each(|a| collect_strings(a, out)),
        Ast::Recur(v) => v.iter().for_each(|a| collect_strings(a, out)),
        Ast::MakeFn { captures, .. } => captures.iter().for_each(|a| collect_strings(a, out)),
        Ast::CallValue { f, args } => {
            collect_strings(f, out);
            args.iter().for_each(|a| collect_strings(a, out));
        }
        Ast::Apply { f, fixed, coll } => {
            collect_strings(f, out);
            fixed.iter().for_each(|a| collect_strings(a, out));
            collect_strings(coll, out);
        }
        Ast::MakeRecord { type_name, fields } => {
            out.push(type_name.clone());
            fields.iter().for_each(|(fname, v)| {
                out.push(fname.clone());
                collect_strings(v, out);
            });
        }
        Ast::RegisterMethod { key, impl_fn, .. } => {
            collect_strings(key, out);
            collect_strings(impl_fn, out);
        }
        Ast::Loop { slots, body } | Ast::Let { slots, body } => {
            slots.iter().for_each(|(_, a)| collect_strings(a, out));
            collect_strings(body, out);
        }
        Ast::Call { args, .. } => args.iter().for_each(|a| collect_strings(a, out)),
        Ast::DefGlobal { value, .. } => collect_strings(value, out),
        Ast::RegisterMulti { dispatch_fn, .. } => collect_strings(dispatch_fn, out),
        _ => {}
    }
}
