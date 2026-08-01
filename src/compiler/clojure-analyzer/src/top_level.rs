use crate::analysis::*;
use crate::ast::*;
use crate::optimizations::*;
use crate::primitives::*;
use clojure_diagnostics::{Diagnostic, Diagnostics};
use clojure_syntax::{Form, SForm};
use std::collections::HashMap;

/// Expands and analyzes top-level forms into a compilable program.
///
/// Analysis is deterministic for a fixed form sequence. It collects top-level
/// signatures before analyzing bodies, allowing forward references between
/// functions.
///
/// # Errors
///
/// Returns all diagnostics collected while analyzing independent top-level
/// forms. No partial [`Program`] is returned when any error is present.
pub fn analyze(forms: &[SForm]) -> Result<Program, Diagnostics> {
    let expanded = crate::expand_all(forms);
    analyze_expanded(&expanded)
}

pub(crate) fn analyze_expanded(forms: &[SForm]) -> Result<Program, Diagnostics> {
    let mut diags = Diagnostics::new();

    // Pass 1 collects top-level signatures so forward calls have arity data.
    let mut sigs: HashMap<String, Vec<(usize, bool)>> = HashMap::new();
    // Protocol methods map to (runtime method ID, arity); record names form the
    // user-defined dispatch-key set.
    let mut protos: HashMap<String, (i64, usize)> = HashMap::new();
    let mut records: std::collections::HashSet<String> = std::collections::HashSet::new();
    // Multimethod names share the positive runtime method-ID namespace.
    let mut multis: HashMap<String, i64> = HashMap::new();
    // Top-level `def` globals: mangled name -> permanent-root index, source order.
    let mut globals: HashMap<String, u32> = HashMap::new();
    // Per-namespace alias -> target-namespace, for qualified reference resolution.
    let mut aliases: HashMap<String, HashMap<String, String>> = HashMap::new();
    // Current namespace while walking forms; `def`/`defn` names are mangled by it.
    let mut cur_ns = String::from("user");
    let mut next_mid: i64 = 1;
    for f in forms {
        if let Some(ns) = form_ns(f) {
            let entry = aliases.entry(ns.clone()).or_default();
            for (alias, target) in form_aliases(f) {
                entry.insert(alias, target);
            }
            cur_ns = ns;
            continue;
        }
        if let Some((name, _)) = match_def(f) {
            let sym = mangle(&cur_ns, &name);
            if globals.contains_key(&sym) {
                diags.push(
                    Diagnostic::error("E0113", format!("redefinição de def não suportada: {name}"))
                        .with_span(f.span),
                );
            } else {
                let idx = globals.len() as u32;
                globals.insert(sym, idx);
            }
        } else if let Some((name, decls)) = match_defn(f) {
            sigs.insert(
                mangle(&cur_ns, &name),
                decls
                    .iter()
                    .map(|(p, r, _)| (p.len(), r.is_some()))
                    .collect(),
            );
        } else if let Some((name, fields)) = match_defrecord(f) {
            records.insert(name.clone());
            sigs.insert(format!("->{name}"), vec![(fields.len(), false)]);
        } else if let Some((_pname, methods)) = match_defprotocol(f) {
            for (mname, arity) in methods {
                protos.insert(mname.clone(), (next_mid, arity));
                sigs.insert(mname, vec![(arity, false)]);
                next_mid += 1;
            }
        } else if let Some((name, _df)) = match_defmulti(f) {
            multis.insert(name.clone(), next_mid);
            sigs.insert(name, vec![(0, true)]); // variádico: encaminha qualquer aridade
            next_mid += 1;
        }
    }

    let mut an = Analyzer {
        sigs: &sigs,
        protos: &protos,
        records: &records,
        globals: &globals,
        aliases: &aliases,
        cur_ns: String::from("user"),
        frames: Vec::new(),
        functions: Vec::new(),
        lam: 0,
    };
    let mut main_body = Vec::new();

    // The synthetic main frame remains at the bottom while function frames are
    // pushed and popped above it.
    an.frames.push(Frame::new(false));

    for f in forms {
        if let Some(ns) = form_ns(f) {
            an.cur_ns = ns; // subsequent def/defn names are mangled by this namespace
            continue;
        }
        if let Some((name, value_form)) = match_def(f) {
            let index = an.globals[&mangle(&an.cur_ns, &name)];
            match an.analyze(&value_form, false) {
                Ok(value) => main_body.push(Ast::DefGlobal {
                    index,
                    value: Box::new(value),
                }),
                Err(d) => diags.push(d),
            }
        } else if let Some((name, decls)) = match_defn(f) {
            let sym = mangle(&an.cur_ns, &name);
            match an.analyze_methods(&decls, false, f.span) {
                Ok((methods, lc, _caps)) => an.functions.push(Function {
                    name: sym,
                    methods,
                    local_count: lc,
                    is_lambda: false,
                    dispatch: Dispatch::None,
                }),
                Err(d) => diags.push(d),
            }
        } else if let Some((rname, fields)) = match_defrecord(f) {
            // Synthesize ->Name as an ordinary function constructing the record.
            let make = Ast::MakeRecord {
                type_name: rname.clone(),
                fields: fields
                    .iter()
                    .enumerate()
                    .map(|(i, fld)| (fld.clone(), Ast::Local(i as u32)))
                    .collect(),
            };
            an.functions.push(Function {
                name: format!("->{rname}"),
                methods: vec![FnMethod {
                    params: fields.clone(),
                    rest: None,
                    body: make,
                    optimization: MethodOptimization::default(),
                }],
                local_count: fields.len() as u32,
                is_lambda: false,
                dispatch: Dispatch::None,
            });
        } else if let Some((_pname, methods)) = match_defprotocol(f) {
            // Each protocol method becomes a body-less dispatch stub.
            for (mname, _arity) in methods {
                let mid = an.protos[&mname].0;
                an.functions.push(Function {
                    name: mname,
                    methods: vec![],
                    local_count: 0,
                    is_lambda: false,
                    dispatch: Dispatch::Protocol(mid),
                });
            }
        } else if let Some((typename, impls)) = match_extend_type(f) {
            for (mname, params, body_forms) in impls {
                // ABI: core capabilities use reserved negative IDs outside the
                // positive program protocol-ID namespace (ADR-0008).
                let mid = match core_capability_mid(&mname) {
                    Some(cid) => cid,
                    None => {
                        let Some(&(mid, _)) = an.protos.get(&mname) else {
                            diags.push(unsupported(
                                format!("método de protocolo desconhecido: {mname}"),
                                f.span,
                            ));
                            continue;
                        };
                        mid
                    }
                };
                let Some(key) = key_for(&typename, an.records) else {
                    diags.push(unsupported(
                        format!("tipo desconhecido em extend-type: {typename}"),
                        f.span,
                    ));
                    continue;
                };
                let decls = vec![(params, None, body_forms)];
                match an.analyze_methods(&decls, true, f.span) {
                    Ok((lmethods, lc, caps)) => {
                        let lname = format!("__impl_{}", an.lam);
                        an.lam += 1;
                        let arity = lmethods[0].params.len();
                        an.functions.push(Function {
                            name: lname.clone(),
                            methods: lmethods,
                            local_count: lc,
                            is_lambda: true,
                            dispatch: Dispatch::None,
                        });
                        let make = Ast::MakeFn {
                            lambda: lname,
                            arity,
                            captures: caps,
                        };
                        main_body.push(Ast::RegisterMethod {
                            method_id: mid,
                            key: Box::new(key),
                            impl_fn: Box::new(make),
                        });
                    }
                    Err(d) => diags.push(d),
                }
            }
        } else if let Some((name, df_form)) = match_defmulti(f) {
            let mid = multis[&name];
            // The top-level symbol is a multimethod dispatch stub.
            an.functions.push(Function {
                name,
                methods: vec![],
                local_count: 0,
                is_lambda: false,
                dispatch: Dispatch::Multi(mid),
            });
            // Register the dispatch function during program initialization.
            match an.analyze(&df_form, false) {
                Ok(df) => main_body.push(Ast::RegisterMulti {
                    method_id: mid,
                    dispatch_fn: Box::new(df),
                }),
                Err(d) => diags.push(d),
            }
        } else if let Some((name, dv_form, params, body_forms)) = match_defmethod(f) {
            let Some(&mid) = multis.get(&name) else {
                diags.push(unsupported(
                    format!("defmethod para multimethod desconhecido: {name}"),
                    f.span,
                ));
                continue;
            };
            let key = match an.analyze(&dv_form, false) {
                Ok(k) => k,
                Err(d) => {
                    diags.push(d);
                    continue;
                }
            };
            let decls = vec![(params, None, body_forms)];
            match an.analyze_methods(&decls, true, f.span) {
                Ok((lmethods, lc, caps)) => {
                    let lname = format!("__method_{}", an.lam);
                    an.lam += 1;
                    let arity = lmethods[0].params.len();
                    an.functions.push(Function {
                        name: lname.clone(),
                        methods: lmethods,
                        local_count: lc,
                        is_lambda: true,
                        dispatch: Dispatch::None,
                    });
                    let make = Ast::MakeFn {
                        lambda: lname,
                        arity,
                        captures: caps,
                    };
                    main_body.push(Ast::RegisterMethod {
                        method_id: mid,
                        key: Box::new(key),
                        impl_fn: Box::new(make),
                    });
                }
                Err(d) => diags.push(d),
            }
        } else {
            match an.analyze(f, false) {
                Ok(a) => main_body.push(a),
                Err(d) => diags.push(d),
            }
        }
    }

    let main_frame = an.frames.pop().unwrap();
    if diags.has_errors() {
        return Err(diags);
    }
    let mut functions = an.functions;
    optimize_transients(&mut functions, &mut main_body); // ADR-0009/0010.
    Ok(Program {
        functions,
        main_body,
        main_local_count: main_frame.max_slots,
        global_count: globals.len() as u32,
    })
}

pub(crate) type MethodDecl = (Vec<String>, Option<String>, Vec<SForm>);

/// Recognizes single- and multi-arity `defn` and returns its declarations.
pub(crate) fn match_defn(f: &SForm) -> Option<(String, Vec<MethodDecl>)> {
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
    // Docstrings affect documentation, not the compiled method body.
    let mut rest = &items[2..];
    if rest.len() > 1 && matches!(rest[0].node.strip_meta(), Form::Str(_)) {
        rest = &rest[1..];
    }
    let methods = parse_methods(rest)?;
    Some((name, methods))
}

/// Mangles a namespace-qualified top-level name into a unique linker symbol, e.g.
/// `cljn.http.request` + `valid?` -> `cljn_http_request__valid?` (ADR-0013). Dots
/// become underscores; the `__` separator keeps names from distinct namespaces
/// disjoint. `def`/`defn` names are namespaced; protocol/multimethod/record names
/// stay global (project-wide unique) in the current P1 subset.
pub(crate) fn mangle(ns: &str, name: &str) -> String {
    format!("{}__{}", ns.replace('.', "_"), name)
}

/// The namespace name declared by a `(ns name ...)` form, if this form is one.
pub(crate) fn form_ns(f: &SForm) -> Option<String> {
    let Form::List(items) = f.node.strip_meta() else {
        return None;
    };
    let Some(Form::Symbol(h)) = items.first().map(|x| x.node.strip_meta()) else {
        return None;
    };
    if h.ns.is_some() || h.name != "ns" {
        return None;
    }
    match items.get(1).map(|x| x.node.strip_meta()) {
        Some(Form::Symbol(n)) => Some(n.to_string()),
        _ => None,
    }
}

/// The `alias -> target-namespace` pairs from a `(ns ... (:require [t :as a]))`
/// form. Bare `(:require t)` specs contribute `t -> t` so `t/name` also resolves.
pub(crate) fn form_aliases(f: &SForm) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let Form::List(items) = f.node.strip_meta() else {
        return out;
    };
    for clause in items.iter().skip(2) {
        let Form::List(citems) = clause.node.strip_meta() else {
            continue;
        };
        let is_require = matches!(
            citems.first().map(|x| x.node.strip_meta()),
            Some(Form::Keyword(k)) if k.ns.is_none() && k.name == "require"
        );
        if !is_require {
            continue;
        }
        for spec in citems.iter().skip(1) {
            match spec.node.strip_meta() {
                Form::Vector(v) => {
                    let Some(Form::Symbol(target)) = v.first().map(|x| x.node.strip_meta()) else {
                        continue;
                    };
                    let target = target.to_string();
                    // Look for `:as alias`.
                    let mut alias = target.clone();
                    let mut i = 1;
                    while i + 1 < v.len() {
                        if matches!(v[i].node.strip_meta(), Form::Keyword(k) if k.ns.is_none() && k.name == "as")
                        {
                            if let Form::Symbol(a) = v[i + 1].node.strip_meta() {
                                alias = a.to_string();
                            }
                        }
                        i += 1;
                    }
                    out.push((alias, target));
                }
                Form::Symbol(t) => {
                    let t = t.to_string();
                    out.push((t.clone(), t));
                }
                _ => {}
            }
        }
    }
    out
}

/// Full keyword text including any namespace, e.g. `cljn.error/domain`. The
/// runtime stores keywords as interned strings, so a namespaced keyword is simply
/// one whose name contains a slash.
pub(crate) fn keyword_text(n: &clojure_syntax::Name) -> String {
    match &n.ns {
        Some(ns) => format!("{ns}/{}", n.name),
        None => n.name.clone(),
    }
}

/// Recognizes `(def name value)` (top-level data). Returns the name and the
/// initializer form. A docstring between name and value is ignored.
pub(crate) fn match_def(f: &SForm) -> Option<(String, SForm)> {
    let Form::List(items) = f.node.strip_meta() else {
        return None;
    };
    let head = items.first()?;
    let Form::Symbol(n) = head.node.strip_meta() else {
        return None;
    };
    if n.ns.is_some() || n.name != "def" {
        return None;
    }
    let name = match items.get(1).map(|f| f.node.strip_meta()) {
        Some(Form::Symbol(nm)) if nm.ns.is_none() => nm.name.clone(),
        _ => return None,
    };
    let mut rest = &items[2..];
    if rest.len() > 1 && matches!(rest[0].node.strip_meta(), Form::Str(_)) {
        rest = &rest[1..]; // skip docstring
    }
    let value = rest.first()?.clone();
    Some((name, value))
}

/// Recognizes `(defrecord Name [fields...])`.
///
/// Inline protocol implementations are not accepted approximately; remaining
/// forms are left for the analyzer to diagnose.
pub(crate) fn match_defrecord(f: &SForm) -> Option<(String, Vec<String>)> {
    let Form::List(items) = f.node.strip_meta() else {
        return None;
    };
    let head = items.first()?;
    let Form::Symbol(n) = head.node.strip_meta() else {
        return None;
    };
    if n.ns.is_some() || n.name != "defrecord" {
        return None;
    }
    let name = match items.get(1).map(|f| f.node.strip_meta()) {
        Some(Form::Symbol(nm)) if nm.ns.is_none() => nm.name.clone(),
        _ => return None,
    };
    let (fields, rest) = parse_params(items.get(2)?)?;
    if rest.is_some() {
        return None;
    }
    Some((name, fields))
}

/// Extracts a protocol name and `(method, arity)` declarations.
pub(crate) fn match_defprotocol(f: &SForm) -> Option<(String, Vec<(String, usize)>)> {
    let Form::List(items) = f.node.strip_meta() else {
        return None;
    };
    let Form::Symbol(n) = items.first()?.node.strip_meta() else {
        return None;
    };
    if n.ns.is_some() || n.name != "defprotocol" {
        return None;
    }
    let name = match items.get(1).map(|f| f.node.strip_meta()) {
        Some(Form::Symbol(nm)) if nm.ns.is_none() => nm.name.clone(),
        _ => return None,
    };
    let mut methods = Vec::new();
    for m in &items[2..] {
        if let Form::List(mi) = m.node.strip_meta() {
            if let (Some(Form::Symbol(mn)), Some(Form::Vector(ps))) = (
                mi.first().map(|x| x.node.strip_meta()),
                mi.get(1).map(|x| x.node.strip_meta()),
            ) {
                methods.push((mn.name.clone(), ps.len()));
            }
        }
        // Protocol docstrings and other non-method forms do not affect dispatch.
    }
    Some((name, methods))
}

/// Extracts an `extend-type` target and its method implementations.
///
/// Interleaved protocol names are ignored because current dispatch identity is
/// the method name.
#[allow(clippy::type_complexity)]
pub(crate) fn match_extend_type(
    f: &SForm,
) -> Option<(String, Vec<(String, Vec<String>, Vec<SForm>)>)> {
    let Form::List(items) = f.node.strip_meta() else {
        return None;
    };
    let Form::Symbol(n) = items.first()?.node.strip_meta() else {
        return None;
    };
    if n.ns.is_some() || n.name != "extend-type" {
        return None;
    }
    let typename = match items.get(1).map(|f| f.node.strip_meta()) {
        Some(Form::Symbol(t)) if t.ns.is_none() => t.name.clone(),
        _ => return None,
    };
    let mut impls = Vec::new();
    for it in &items[2..] {
        if let Form::List(mi) = it.node.strip_meta() {
            let (params, rest) = parse_params(mi.get(1)?)?;
            if rest.is_some() {
                return None;
            }
            if let Some(Form::Symbol(mn)) = mi.first().map(|x| x.node.strip_meta()) {
                impls.push((mn.name.clone(), params, mi[2..].to_vec()));
            }
        }
        // A protocol-name symbol does not define a method and is ignored.
    }
    Some((typename, impls))
}

/// Extracts a multimethod name and dispatch-function form.
pub(crate) fn match_defmulti(f: &SForm) -> Option<(String, SForm)> {
    let Form::List(items) = f.node.strip_meta() else {
        return None;
    };
    let Form::Symbol(n) = items.first()?.node.strip_meta() else {
        return None;
    };
    if n.ns.is_some() || n.name != "defmulti" {
        return None;
    }
    let name = match items.get(1).map(|f| f.node.strip_meta()) {
        Some(Form::Symbol(nm)) if nm.ns.is_none() => nm.name.clone(),
        _ => return None,
    };
    Some((name, items.get(2)?.clone()))
}

/// Extracts a method name, dispatch value, parameters, and body from `defmethod`.
pub(crate) fn match_defmethod(f: &SForm) -> Option<(String, SForm, Vec<String>, Vec<SForm>)> {
    let Form::List(items) = f.node.strip_meta() else {
        return None;
    };
    let Form::Symbol(n) = items.first()?.node.strip_meta() else {
        return None;
    };
    if n.ns.is_some() || n.name != "defmethod" {
        return None;
    }
    let name = match items.get(1).map(|f| f.node.strip_meta()) {
        Some(Form::Symbol(nm)) if nm.ns.is_none() => nm.name.clone(),
        _ => return None,
    };
    let dispatch_val = items.get(2)?.clone();
    let (params, rest) = parse_params(items.get(3)?)?;
    if rest.is_some() {
        return None;
    }
    Some((name, dispatch_val, params, items[4..].to_vec()))
}

/// Maps core capability names to reserved negative method IDs.
///
/// ABI: values must match `CORE_ASSOC_ONE`, `CORE_NTH`, and `CORE_NTH_OR` in the
/// C runtime. These IDs are deliberately separate from program protocol IDs.
pub(crate) fn core_capability_mid(mname: &str) -> Option<i64> {
    match mname {
        "-assoc" => Some(-1),
        "-nth" => Some(-2),
        "-nth-not-found" => Some(-3),
        _ => None,
    }
}

/// Builds the runtime dispatch key for a declared type name.
///
/// ABI: record types use keywords and built-in types use fixnums matching
/// `cljn_type_key` in `runtime/60_records_and_dispatch.c`.
pub(crate) fn key_for(typename: &str, records: &std::collections::HashSet<String>) -> Option<Ast> {
    if records.contains(typename) {
        return Some(Ast::Keyword(typename.to_string()));
    }
    let code = match typename {
        "Int" | "Long" | "Integer" | "Number" => 1000,
        "String" => 1001,
        "List" | "PersistentList" | "Cons" | "Seq" => 1002,
        "Fn" | "IFn" | "Function" => 1003,
        "Keyword" => 1004,
        "Vector" | "PersistentVector" => 1005,
        "Map" | "PersistentArrayMap" | "PersistentHashMap" => 1006,
        "Set" | "PersistentHashSet" => 1007,
        "Nil" => 1010,
        "Boolean" | "Bool" => 1011,
        _ => return None,
    };
    Some(Ast::Int(code))
}

/// Parses either `[params] body...` or multi-arity method lists.
pub(crate) fn parse_methods(forms: &[SForm]) -> Option<Vec<MethodDecl>> {
    match forms.first().map(|f| f.node.strip_meta()) {
        Some(Form::Vector(_)) => {
            let (params, rest) = parse_params(&forms[0])?;
            Some(vec![(params, rest, forms[1..].to_vec())])
        }
        Some(Form::List(_)) => {
            let mut out = Vec::new();
            for m in forms {
                let Form::List(items) = m.node.strip_meta() else {
                    return None;
                };
                let (params, rest) = parse_params(items.first()?)?;
                out.push((params, rest, items[1..].to_vec()));
            }
            Some(out)
        }
        _ => None,
    }
}

/// Splits a parameter vector into fixed names and an optional rest name.
pub(crate) fn parse_params(f: &SForm) -> Option<(Vec<String>, Option<String>)> {
    let Form::Vector(ps) = f.node.strip_meta() else {
        return None;
    };
    let mut out = Vec::new();
    let mut it = ps.iter();
    while let Some(p) = it.next() {
        match p.node.strip_meta() {
            Form::Symbol(pn) if pn.ns.is_none() && pn.name == "&" => {
                let r = it.next()?;
                match r.node.strip_meta() {
                    Form::Symbol(rn) if rn.ns.is_none() => {
                        return Some((out, Some(rn.name.clone())))
                    }
                    _ => return None,
                }
            }
            Form::Symbol(pn) if pn.ns.is_none() => out.push(pn.name.clone()),
            _ => return None, // destructuring fora do slice
        }
    }
    Some((out, None))
}
