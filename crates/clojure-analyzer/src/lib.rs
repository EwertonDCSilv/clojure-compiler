//! Analisador do **subconjunto compilável** (Fase 3, corte vertical p/ Fase 5).
//!
//! Transforma `Form` em uma AST tipada, com resolução de locais/capturas/funções,
//! checagem de aridade, expansão prévia de macros de core (ADR-0004) e
//! **conversão de closures** (funções de 1ª classe: `fn`, captura léxica,
//! chamadas indiretas). Construções fora do subconjunto viram diagnósticos.

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
    Keyword(String),
    /// Literais de coleção.
    VecLit(Vec<Ast>),
    SetLit(Vec<Ast>),
    MapLit(Vec<(Ast, Ast)>),
    /// Local por slot (no frame da função/lambda atual).
    Local(u32),
    /// Variável capturada: `self->freev[idx]` da lambda atual.
    Capture(u32),
    /// Referência a uma função de topo como valor (cria closure com 0 capturas).
    FnRef(String),
    /// Cria uma closure: função-lambda + valores capturados (avaliados no contexto
    /// que contém o `fn`).
    MakeFn {
        lambda: String,
        arity: usize,
        captures: Vec<Ast>,
    },
    If(Box<Ast>, Box<Ast>, Box<Ast>),
    Do(Vec<Ast>),
    Let {
        slots: Vec<(u32, Ast)>,
        body: Box<Ast>,
    },
    Loop {
        slots: Vec<(u32, Ast)>,
        body: Box<Ast>,
    },
    Recur(Vec<Ast>),
    /// Chamada direta a primitiva ou função de topo conhecida.
    Call {
        callee: Callee,
        args: Vec<Ast>,
    },
    /// Chamada indireta de um valor-função (closure).
    CallValue {
        f: Box<Ast>,
        args: Vec<Ast>,
    },
    /// `(apply f a b ... coll)`: chama `f` com os args fixos + elementos de `coll`.
    Apply {
        f: Box<Ast>,
        fixed: Vec<Ast>,
        coll: Box<Ast>,
    },
    /// Constrói um record: nome do tipo + campos (nome → valor).
    MakeRecord {
        type_name: String,
        fields: Vec<(String, Ast)>,
    },
    /// Registra uma impl de protocolo/multimethod em runtime: `(method_id, key) → impl`.
    RegisterMethod {
        method_id: i64,
        key: Box<Ast>,
        impl_fn: Box<Ast>,
    },
    /// Registra a função de dispatch de um multimethod: `(method_id) → dispatch-fn`.
    RegisterMulti {
        method_id: i64,
        dispatch_fn: Box<Ast>,
    },
}

#[derive(Debug, Clone)]
pub enum Callee {
    Prim(Prim),
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
    // coleções
    Get,
    Nth,
    Assoc,
    Dissoc,
    Contains,
    Keys,
    Vals,
    Conj,
    Vector,
    HashMap,
    HashSet,
    SortedMap,
    SortedSet,
    Compare,
    Throw,
    Try,
    Transient,
    PersistentBang,
    ConjBang,
    AssocBang,
    DissocBang,
    Slurp,
    Spit,
    FileExists,
    Getenv,
}

/// Uma aridade de uma função: params fixos + `& rest` opcional + corpo.
#[derive(Debug, Clone)]
pub struct FnMethod {
    pub params: Vec<String>,
    pub rest: Option<String>,
    pub body: Ast,
}

impl FnMethod {
    /// Número de slots que a aridade ocupa (params + rest).
    pub fn nslots(&self) -> usize {
        self.params.len() + self.rest.is_some() as usize
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    /// Uma ou mais aridades (multi-arity). Só uma variádica, sempre a de maior aridade.
    pub methods: Vec<FnMethod>,
    /// Slots locais reservados no frame (máximo sobre as aridades). **Não** inclui `self`.
    pub local_count: u32,
    /// Verdadeiro para lambdas (usam `self` para ler capturas); falso p/ defns de topo.
    pub is_lambda: bool,
    /// Tipo de despacho: nenhum, protocolo (por `type_key(arg0)`) ou multimethod
    /// (por `(dispatch-fn args)`). Funções-despacho têm `methods` vazio.
    pub dispatch: Dispatch,
}

/// Estratégia de despacho de uma função de topo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dispatch {
    None,
    /// Protocolo: `impl = lookup(mid, type_key(arg0))`.
    Protocol(i64),
    /// Multimethod: `impl = lookup(mid, (dispatch-fn args))`, com `:default`.
    Multi(i64),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
    pub main_body: Vec<Ast>,
    pub main_local_count: u32,
}

/// Analisa forms de topo no `Program` compilável (expande macros antes).
pub fn analyze(forms: &[SForm]) -> Result<Program, Diagnostics> {
    let expanded = expand::expand_all(forms);
    analyze_expanded(&expanded)
}

fn analyze_expanded(forms: &[SForm]) -> Result<Program, Diagnostics> {
    let mut diags = Diagnostics::new();

    // Assinaturas de funções de topo: aridades (fixos, variádica?) por nome.
    let mut sigs: HashMap<String, Vec<(usize, bool)>> = HashMap::new();
    // Protocolos: nome do método → (method_id, aridade). Records: nomes de tipo.
    let mut protos: HashMap<String, (i64, usize)> = HashMap::new();
    let mut records: std::collections::HashSet<String> = std::collections::HashSet::new();
    // Multimethods: nome → method_id.
    let mut multis: HashMap<String, i64> = HashMap::new();
    let mut next_mid: i64 = 1;
    for f in forms {
        if let Some((name, decls)) = match_defn(f) {
            sigs.insert(
                name,
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
        frames: Vec::new(),
        functions: Vec::new(),
        lam: 0,
    };
    let mut main_body = Vec::new();

    // Frame do "main" (topo). Fica na base; frames de defn entram/saem acima.
    an.frames.push(Frame::new(false));

    for f in forms {
        if is_ns(f) {
            continue;
        }
        if let Some((name, decls)) = match_defn(f) {
            match an.analyze_methods(&decls, false, f.span) {
                Ok((methods, lc, _caps)) => an.functions.push(Function {
                    name,
                    methods,
                    local_count: lc,
                    is_lambda: false,
                    dispatch: Dispatch::None,
                }),
                Err(d) => diags.push(d),
            }
        } else if let Some((rname, fields)) = match_defrecord(f) {
            // Gera o construtor ->Name que monta o record.
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
                }],
                local_count: fields.len() as u32,
                is_lambda: false,
                dispatch: Dispatch::None,
            });
        } else if let Some((_pname, methods)) = match_defprotocol(f) {
            // Cada método vira uma função-despacho.
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
                // ADR-0008: capacidades de core (assoc/nth) usam IDs reservados
                // (negativos), fora do espaço de protocolos do programa.
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
            // A função-despacho de topo: encaminha por (dispatch-fn args).
            an.functions.push(Function {
                name,
                methods: vec![],
                local_count: 0,
                is_lambda: false,
                dispatch: Dispatch::Multi(mid),
            });
            // Registra a função de dispatch em runtime (avaliada no init).
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
    optimize_transients(&mut functions, &mut main_body); // ADR-0009/0010: auto-transient
    Ok(Program {
        functions,
        main_body,
        main_local_count: main_frame.max_slots,
    })
}

fn is_ns(f: &SForm) -> bool {
    matches!(f.node.strip_meta(), Form::List(items)
        if matches!(items.first().map(|h| h.node.strip_meta()), Some(Form::Symbol(n)) if n.ns.is_none() && n.name == "ns"))
}

type MethodDecl = (Vec<String>, Option<String>, Vec<SForm>);

/// Reconhece `(defn nome [params] body...)` ou multi-aridade
/// `(defn nome ([a] ...) ([a b] ...))`. Devolve o nome e as aridades.
fn match_defn(f: &SForm) -> Option<(String, Vec<MethodDecl>)> {
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
    // Descarta docstring opcional.
    let mut rest = &items[2..];
    if rest.len() > 1 && matches!(rest[0].node.strip_meta(), Form::Str(_)) {
        rest = &rest[1..];
    }
    let methods = parse_methods(rest)?;
    Some((name, methods))
}

/// Reconhece `(defrecord Nome [campos...])`. Impls inline de protocolo ainda não
/// são suportadas (viram erro no analyzer, não silêncio).
fn match_defrecord(f: &SForm) -> Option<(String, Vec<String>)> {
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

/// `(defprotocol Nome (m1 [args]) (m2 [args]))` → nome + (método, aridade).
fn match_defprotocol(f: &SForm) -> Option<(String, Vec<(String, usize)>)> {
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
        // strings (docstrings) e outros são ignorados
    }
    Some((name, methods))
}

/// `(extend-type Tipo Proto (m [args] body...) ...)` → tipo + impls (método/params/corpo).
/// Nomes de protocolo intercalados são ignorados (dispatch é por nome de método).
#[allow(clippy::type_complexity)]
fn match_extend_type(f: &SForm) -> Option<(String, Vec<(String, Vec<String>, Vec<SForm>)>)> {
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
        // Symbol (nome de protocolo) → ignorado.
    }
    Some((typename, impls))
}

/// `(defmulti nome dispatch-fn)` → nome + a forma da função de dispatch.
fn match_defmulti(f: &SForm) -> Option<(String, SForm)> {
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

/// `(defmethod nome dispatch-val [params] body...)` → nome, valor de dispatch,
/// params e corpo.
fn match_defmethod(f: &SForm) -> Option<(String, SForm, Vec<String>, Vec<SForm>)> {
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

/// ADR-0008: nomes reservados de capacidade de core → method_id reservado
/// (negativo, deve casar com CORE_ASSOC_ONE/CORE_NTH/CORE_NTH_OR no runtime).
/// A superfície pública não está congelada (aguarda deftype/impls inline).
fn core_capability_mid(mname: &str) -> Option<i64> {
    match mname {
        "-assoc" => Some(-1),
        "-nth" => Some(-2),
        "-nth-not-found" => Some(-3),
        _ => None,
    }
}

/// Chave de dispatch para um nome de tipo: records → keyword; builtins → fixnum.
/// Deve casar com `cljn_type_key` no runtime.
fn key_for(typename: &str, records: &std::collections::HashSet<String>) -> Option<Ast> {
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

/// Aridade única `[params] body...` ou multi `([params] body...) ...`.
fn parse_methods(forms: &[SForm]) -> Option<Vec<MethodDecl>> {
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

/// Vetor de parâmetros → (fixos, rest opcional após `&`).
fn parse_params(f: &SForm) -> Option<(Vec<String>, Option<String>)> {
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

/// Frame léxico de uma função ou lambda.
struct Frame {
    locals: Vec<(String, u32)>,
    next_slot: u32,
    max_slots: u32,
    recur_arity: Vec<usize>,
    is_lambda: bool,
    captures: Vec<Ast>,
    capture_names: Vec<String>,
}

impl Frame {
    fn new(is_lambda: bool) -> Self {
        Frame {
            locals: Vec::new(),
            next_slot: 0,
            max_slots: 0,
            recur_arity: Vec::new(),
            is_lambda,
            captures: Vec::new(),
            capture_names: Vec::new(),
        }
    }
}

struct Analyzer<'a> {
    sigs: &'a HashMap<String, Vec<(usize, bool)>>,
    protos: &'a HashMap<String, (i64, usize)>,
    records: &'a std::collections::HashSet<String>,
    frames: Vec<Frame>,
    functions: Vec<Function>,
    lam: u32,
}

/// Alguma aridade de `arities` aceita `n` argumentos?
fn arity_accepts(arities: &[(usize, bool)], n: usize) -> bool {
    arities
        .iter()
        .any(|&(fixed, variadic)| if variadic { n >= fixed } else { n == fixed })
}

impl<'a> Analyzer<'a> {
    fn top(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    fn push_local(&mut self, name: String) -> u32 {
        let fr = self.top();
        let slot = fr.next_slot;
        fr.next_slot += 1;
        fr.max_slots = fr.max_slots.max(fr.next_slot);
        fr.locals.push((name, slot));
        slot
    }

    /// Resolve `name` como valor válido no frame `i`: `Local`, `Capture` ou `None`.
    /// Ao cruzar fronteiras de lambda, registra capturas transitivas.
    fn resolve_from(&mut self, i: usize, name: &str) -> Option<Ast> {
        if let Some((_, slot)) = self.frames[i].locals.iter().rev().find(|(n, _)| n == name) {
            return Some(Ast::Local(*slot));
        }
        if self.frames[i].is_lambda {
            if let Some(idx) = self.frames[i].capture_names.iter().position(|n| n == name) {
                return Some(Ast::Capture(idx as u32));
            }
        }
        // Só uma lambda captura do frame que a contém.
        if i == 0 || !self.frames[i].is_lambda {
            return None;
        }
        let parent_ast = self.resolve_from(i - 1, name)?;
        let idx = self.frames[i].captures.len() as u32;
        self.frames[i].captures.push(parent_ast);
        self.frames[i].capture_names.push(name.to_string());
        Some(Ast::Capture(idx))
    }

    fn resolve(&mut self, name: &str) -> Option<Ast> {
        let top = self.frames.len() - 1;
        self.resolve_from(top, name)
    }

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
            Form::Keyword(n) => {
                if n.ns.is_some() {
                    return Err(unsupported(
                        "keyword qualificada ainda não é compilável",
                        f.span,
                    ));
                }
                Ok(Ast::Keyword(n.name.clone()))
            }
            Form::Symbol(n) => self.analyze_symbol_value(&n.ns, &n.name, f.span),
            Form::Vector(items) => Ok(Ast::VecLit(
                items
                    .iter()
                    .map(|it| self.analyze(it, false))
                    .collect::<Result<_, _>>()?,
            )),
            Form::Set(items) => Ok(Ast::SetLit(
                items
                    .iter()
                    .map(|it| self.analyze(it, false))
                    .collect::<Result<_, _>>()?,
            )),
            Form::Map(pairs) => {
                let mut out = Vec::with_capacity(pairs.len());
                for (k, v) in pairs {
                    out.push((self.analyze(k, false)?, self.analyze(v, false)?));
                }
                Ok(Ast::MapLit(out))
            }
            Form::List(items) => self.analyze_list(items, f.span, tail),
            Form::Meta { .. } => unreachable!(),
        }
    }

    /// Símbolo em posição de valor: local/captura, ou função de topo (FnRef).
    fn analyze_symbol_value(
        &mut self,
        ns: &Option<String>,
        name: &str,
        span: Span,
    ) -> Result<Ast, Diagnostic> {
        if ns.is_some() {
            return Err(unsupported(
                format!("símbolo qualificado {name} fora do slice"),
                span,
            ));
        }
        if let Some(ast) = self.resolve(name) {
            return Ok(ast);
        }
        if self.sigs.contains_key(name) {
            return Ok(Ast::FnRef(name.to_string()));
        }
        if let Some(prim) = prim_of(name) {
            // Primitiva como valor: sintetiza um wrapper `(fn [a b] (prim a b))`.
            if let Some(arity) = prim_value_arity(prim) {
                let params: Vec<String> = (0..arity).map(|i| format!("__p{i}")).collect();
                let call_args = (0..arity).map(|i| Ast::Local(i as u32)).collect();
                let body = Ast::Call {
                    callee: Callee::Prim(prim),
                    args: call_args,
                };
                let lname = format!("__prim_{}", self.lam);
                self.lam += 1;
                self.functions.push(Function {
                    name: lname.clone(),
                    methods: vec![FnMethod {
                        params,
                        rest: None,
                        body,
                    }],
                    local_count: arity as u32,
                    is_lambda: true,
                    dispatch: Dispatch::None,
                });
                return Ok(Ast::MakeFn {
                    lambda: lname,
                    arity,
                    captures: vec![],
                });
            }
            return Err(unsupported(
                format!("primitiva variádica `{name}` como valor ainda não é compilável; envolva em (fn [& xs] ...)"),
                span,
            ));
        }
        Err(
            Diagnostic::error("E0101", format!("símbolo não resolvido: {name}"))
                .with_span(span)
                .with_help(
                    "locais, capturas, funções de topo (como valor) e primitivas (em chamada)",
                ),
        )
    }

    fn analyze_list(&mut self, items: &[SForm], span: Span, tail: bool) -> Result<Ast, Diagnostic> {
        let Some((head, args)) = items.split_first() else {
            return Err(unsupported("lista vazia não é compilável", span));
        };

        // `(:kw coll)` → (get coll :kw).
        if let Form::Keyword(n) = head.node.strip_meta() {
            if n.ns.is_none() && args.len() == 1 {
                let coll = self.analyze(&args[0], false)?;
                return Ok(Ast::Call {
                    callee: Callee::Prim(Prim::Get),
                    args: vec![coll, Ast::Keyword(n.name.clone())],
                });
            }
        }

        // Operador não-simbólico: chamada indireta de um valor-função.
        let Form::Symbol(op) = head.node.strip_meta() else {
            let f = self.analyze(head, false)?;
            let a = self.analyze_seq(args)?;
            return Ok(Ast::CallValue {
                f: Box::new(f),
                args: a,
            });
        };
        if op.ns.is_some() {
            return Err(unsupported(
                format!("operador qualificado {op} fora do slice"),
                head.span,
            ));
        }
        match op.name.as_str() {
            "if" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(Diagnostic::error(
                        "E0102",
                        "if requer test, then e (opcional) else",
                    )
                    .with_span(span));
                }
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
            "fn" | "fn*" => self.analyze_fn(args, span),
            "try" => self.analyze_try(args, span),
            "apply" => {
                if args.len() < 2 {
                    return Err(unsupported("apply requer função e uma coleção", span));
                }
                let f = self.analyze(&args[0], false)?;
                let coll = self.analyze(&args[args.len() - 1], false)?;
                let fixed = self.analyze_seq(&args[1..args.len() - 1])?;
                Ok(Ast::Apply {
                    f: Box::new(f),
                    fixed,
                    coll: Box::new(coll),
                })
            }
            "quote" => Err(unsupported("quote ainda não é compilável", span)),
            "def" | "defn" | "defn-" => Err(unsupported(
                "def/defn só é permitido no nível de topo",
                span,
            )),
            name => {
                if let Some(prim) = prim_of(name) {
                    check_prim_arity(prim, args.len(), span)?;
                    let a = self.analyze_seq(args)?;
                    Ok(Ast::Call {
                        callee: Callee::Prim(prim),
                        args: a,
                    })
                } else if let Some(ast) = self.resolve(name) {
                    // Local/captura em posição de operador → chamada indireta.
                    let a = self.analyze_seq(args)?;
                    Ok(Ast::CallValue {
                        f: Box::new(ast),
                        args: a,
                    })
                } else if let Some(arities) = self.sigs.get(name) {
                    if !arity_accepts(arities, args.len()) {
                        return Err(Diagnostic::error(
                            "E0103",
                            format!("aridade errada ao chamar {name}: recebeu {}", args.len()),
                        )
                        .with_span(span));
                    }
                    let a = self.analyze_seq(args)?;
                    Ok(Ast::Call {
                        callee: Callee::Fn(name.to_string()),
                        args: a,
                    })
                } else {
                    Err(
                        Diagnostic::error("E0101", format!("função não resolvida: {name}"))
                            .with_span(span)
                            .with_help("defina-a com defn, use uma primitiva, um local ou um fn"),
                    )
                }
            }
        }
    }

    fn analyze_seq(&mut self, forms: &[SForm]) -> Result<Vec<Ast>, Diagnostic> {
        forms.iter().map(|f| self.analyze(f, false)).collect()
    }

    fn analyze_let(&mut self, args: &[SForm], span: Span, tail: bool) -> Result<Ast, Diagnostic> {
        let bindings = match args.first().map(|f| f.node.strip_meta()) {
            Some(Form::Vector(b)) => b,
            _ => {
                return Err(
                    Diagnostic::error("E0104", "let requer vetor de bindings").with_span(span)
                )
            }
        };
        if bindings.len() % 2 != 0 {
            return Err(Diagnostic::error("E0104", "let: bindings em pares").with_span(span));
        }
        let saved = self.top().locals.len();
        let saved_next = self.top().next_slot;
        let mut slots = Vec::new();
        for pair in bindings.chunks_exact(2) {
            let name = match pair[0].node.strip_meta() {
                Form::Symbol(n) if n.ns.is_none() => n.name.clone(),
                _ => {
                    return Err(unsupported(
                        "let: binding deve ser símbolo simples (sem destructuring)",
                        pair[0].span,
                    ))
                }
            };
            let val = self.analyze(&pair[1], false)?;
            let slot = self.push_local(name);
            slots.push((slot, val));
        }
        let body = self.analyze_body(&args[1..], span, tail)?;
        let fr = self.top();
        fr.locals.truncate(saved);
        fr.next_slot = saved_next;
        Ok(Ast::Let {
            slots,
            body: Box::new(body),
        })
    }

    fn analyze_loop(&mut self, args: &[SForm], span: Span, tail: bool) -> Result<Ast, Diagnostic> {
        let bindings = match args.first().map(|f| f.node.strip_meta()) {
            Some(Form::Vector(b)) => b,
            _ => {
                return Err(
                    Diagnostic::error("E0106", "loop requer vetor de bindings").with_span(span)
                )
            }
        };
        if bindings.len() % 2 != 0 {
            return Err(Diagnostic::error("E0106", "loop: bindings em pares").with_span(span));
        }
        let saved = self.top().locals.len();
        let saved_next = self.top().next_slot;
        let mut slots = Vec::new();
        for pair in bindings.chunks_exact(2) {
            let name = match pair[0].node.strip_meta() {
                Form::Symbol(n) if n.ns.is_none() => n.name.clone(),
                _ => {
                    return Err(unsupported(
                        "loop: binding deve ser símbolo simples",
                        pair[0].span,
                    ))
                }
            };
            let val = self.analyze(&pair[1], false)?;
            let slot = self.push_local(name);
            slots.push((slot, val));
        }
        // O corpo do loop é sempre contexto de cauda para o seu próprio recur,
        // independentemente de o loop estar ou não em posição de cauda externa.
        let _ = tail;
        self.top().recur_arity.push(slots.len());
        let body = self.analyze_body(&args[1..], span, true);
        self.top().recur_arity.pop();
        let body = body?;
        let fr = self.top();
        fr.locals.truncate(saved);
        fr.next_slot = saved_next;
        // A linearização (auto-transient, ADR-0009/0010) é um post-pass sobre o
        // Program inteiro (precisa dos sumários de todas as funções).
        Ok(Ast::Loop {
            slots,
            body: Box::new(body),
        })
    }

    fn analyze_recur(&mut self, args: &[SForm], span: Span, tail: bool) -> Result<Ast, Diagnostic> {
        let Some(&arity) = self.top().recur_arity.last() else {
            return Err(Diagnostic::error("E0107", "recur fora de loop/fn").with_span(span));
        };
        if !tail {
            return Err(
                Diagnostic::error("E0108", "recur não está em posição de cauda")
                    .with_span(span)
                    .with_help("recur só pode ser a última expressão de uma fn ou loop"),
            );
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

    /// Analisa as aridades de uma função num frame próprio; devolve
    /// (métodos, slots máximos, capturas). Para lambdas, as capturas são
    /// compartilhadas entre as aridades.
    fn analyze_methods(
        &mut self,
        decls: &[MethodDecl],
        is_lambda: bool,
        span: Span,
    ) -> Result<(Vec<FnMethod>, u32, Vec<Ast>), Diagnostic> {
        self.frames.push(Frame::new(is_lambda));
        let mut methods = Vec::new();
        let mut variadic_seen = false;
        for (params, rest, body_forms) in decls {
            if rest.is_some() {
                if variadic_seen {
                    self.frames.pop();
                    return Err(unsupported(
                        "fn: só uma aridade variádica é permitida",
                        span,
                    ));
                }
                variadic_seen = true;
            }
            // Cada aridade tem seu próprio conjunto de slots (reutilizam a região).
            self.top().locals.clear();
            self.top().next_slot = 0;
            for p in params {
                self.push_local(p.clone());
            }
            if let Some(r) = rest {
                self.push_local(r.clone());
            }
            let arity = params.len() + rest.is_some() as usize;
            self.top().recur_arity.push(arity);
            let body = self.analyze_body(body_forms, span, true);
            self.top().recur_arity.pop();
            match body {
                Ok(body) => methods.push(FnMethod {
                    params: params.clone(),
                    rest: rest.clone(),
                    body,
                }),
                Err(d) => {
                    self.frames.pop();
                    return Err(d);
                }
            }
        }
        let fr = self.frames.pop().unwrap();
        Ok((methods, fr.max_slots, fr.captures))
    }

    /// Cria uma lambda de aridade única (params fixos, sem rest) a partir de forms
    /// de corpo e devolve `MakeFn` com as capturas. Usada por `fn` e por `try`.
    fn make_lambda(
        &mut self,
        params: Vec<String>,
        body: Vec<SForm>,
        span: Span,
    ) -> Result<Ast, Diagnostic> {
        let arity = params.len();
        let decls = vec![(params, None, body)];
        let (methods, lc, captures) = self.analyze_methods(&decls, true, span)?;
        let name = format!("__lambda_{}", self.lam);
        self.lam += 1;
        self.functions.push(Function {
            name: name.clone(),
            methods,
            local_count: lc,
            is_lambda: true,
            dispatch: Dispatch::None,
        });
        Ok(Ast::MakeFn {
            lambda: name,
            arity,
            captures,
        })
    }

    /// `(try corpo... (catch Classe e handler...) (finally limpeza...))`.
    /// Desugar: corpo/catch/finally viram lambdas (com captura léxica) e a forma
    /// vira `(cljn_try body catch|nil finally|nil)`. `catch` é catch-all no
    /// subconjunto (sem hierarquia de classes); a classe é aceita e ignorada.
    fn analyze_try(&mut self, args: &[SForm], span: Span) -> Result<Ast, Diagnostic> {
        let mut body: Vec<SForm> = Vec::new();
        let mut catch: Option<(String, Vec<SForm>)> = None;
        let mut finally: Option<Vec<SForm>> = None;
        for f in args {
            if let Form::List(items) = f.node.strip_meta() {
                if let Some(Form::Symbol(s)) = items.first().map(|h| h.node.strip_meta()) {
                    match s.name.as_str() {
                        "catch" => {
                            if catch.is_some() {
                                return Err(unsupported(
                                    "try: múltiplos catch não suportados",
                                    span,
                                ));
                            }
                            if items.len() < 3 {
                                return Err(unsupported(
                                    "catch requer classe, binding e corpo",
                                    f.span,
                                ));
                            }
                            let bind = match items[2].node.strip_meta() {
                                Form::Symbol(b) => b.name.clone(),
                                _ => {
                                    return Err(unsupported(
                                        "catch: binding deve ser um símbolo",
                                        items[2].span,
                                    ))
                                }
                            };
                            catch = Some((bind, items[3..].to_vec()));
                            continue;
                        }
                        "finally" => {
                            if finally.is_some() {
                                return Err(unsupported("try: múltiplos finally", span));
                            }
                            finally = Some(items[1..].to_vec());
                            continue;
                        }
                        _ => {}
                    }
                }
            }
            if catch.is_some() || finally.is_some() {
                return Err(unsupported(
                    "try: expressão de corpo após catch/finally",
                    f.span,
                ));
            }
            body.push(f.clone());
        }
        let body_fn = self.make_lambda(vec![], body, span)?;
        let catch_fn = match catch {
            Some((bind, forms)) => self.make_lambda(vec![bind], forms, span)?,
            None => Ast::Nil,
        };
        let finally_fn = match finally {
            Some(forms) => self.make_lambda(vec![], forms, span)?,
            None => Ast::Nil,
        };
        Ok(Ast::Call {
            callee: Callee::Prim(Prim::Try),
            args: vec![body_fn, catch_fn, finally_fn],
        })
    }

    /// `(fn [params] body...)`, `(fn* nome? [params] body...)` ou multi-aridade
    /// `(fn ([a] ...) ([a b] ...))` → cria uma lambda + `MakeFn` com as capturas.
    fn analyze_fn(&mut self, args: &[SForm], span: Span) -> Result<Ast, Diagnostic> {
        let mut idx = 0;
        if matches!(
            args.first().map(|f| f.node.strip_meta()),
            Some(Form::Symbol(_))
        ) {
            idx = 1; // nome opcional (auto-referência não suportada; ignorado)
        }
        let decls = parse_methods(&args[idx..]).ok_or_else(|| {
            unsupported(
                "fn: forma inválida (esperava [params] corpo ou aridades)",
                span,
            )
        })?;
        let (methods, lc, captures) = self.analyze_methods(&decls, true, span)?;
        let name = format!("__lambda_{}", self.lam);
        self.lam += 1;
        let arity = methods[0].params.len();
        self.functions.push(Function {
            name: name.clone(),
            methods,
            local_count: lc,
            is_lambda: true,
            dispatch: Dispatch::None,
        });
        Ok(Ast::MakeFn {
            lambda: name,
            arity,
            captures,
        })
    }
}

// ===== Auto-transient de acumuladores de loop (ADR-0009) =====
// Reescreve slots de `loop` cujo acumulador é um vetor literal fresco usado de
// forma LINEAR (só conj/assoc no próprio slot, leituras nth/get/count/contains,
// e recur/escape do próprio slot — nunca passado a função, capturado, guardado,
// ou religado a outro slot). Nesses casos, o acumulador vira transiente (mutação
// in-place O(1)) e é congelado com `persistent!` nas saídas. É SÃO por construção:
// qualquer uso não reconhecido cancela a transformação daquele slot (fallback ao
// caminho persistente). A semântica é idêntica; só muda a representação interna.

fn ast_is_local(e: &Ast, s: u32) -> bool {
    matches!(e, Ast::Local(x) if *x == s)
}
/// `e` é um valor derivado do acumulador `s`? (o próprio `s`, ou uma cadeia de
/// conj/assoc enraizada em `s`).
fn s_derived(e: &Ast, s: u32) -> bool {
    match e {
        Ast::Local(x) => *x == s,
        Ast::Call { callee: Callee::Prim(Prim::Conj | Prim::Assoc), args } => {
            args.first().is_some_and(|r| s_derived(r, s))
        }
        _ => false,
    }
}
/// Verdadeiro se TODA ocorrência de `Local(s)` em `e` está numa posição linear
/// válida. `tail` indica que um valor derivado de `s` pode escapar/religar aqui.
/// `pos` é a posição do slot `s` na lista de slots do loop (para `recur`).
type LinMap = HashMap<String, usize>;
/// `e` é uma chamada a função linear que consome `s` no parâmetro linear (ADR-0010)?
fn is_lin_call(e: &Ast, s: u32, lin: &LinMap) -> bool {
    if let Ast::Call { callee: Callee::Fn(name), args } = e {
        if let Some(&j) = lin.get(name) {
            return args.get(j).is_some_and(|a| ast_is_local(a, s));
        }
    }
    false
}
/// Valor que produz o próximo transiente de `s`: cadeia conj/assoc, `s` puro, ou
/// chamada a função linear consumindo `s`.
fn transient_producing(e: &Ast, s: u32, lin: &LinMap) -> bool {
    s_derived(e, s) || is_lin_call(e, s, lin)
}
fn linear_ok(e: &Ast, s: u32, pos: usize, tail: bool, lin: &LinMap) -> bool {
    match e {
        Ast::Local(x) => *x != s || tail, // `s` puro só é válido em cauda (escape/recur-s)
        Ast::Int(_) | Ast::Bool(_) | Ast::Nil | Ast::Str(_) | Ast::Keyword(_)
        | Ast::Capture(_) | Ast::FnRef(_) => true,
        Ast::Call { callee: Callee::Prim(p), args }
            if matches!(p, Prim::Conj | Prim::Assoc) && args.first().is_some_and(|r| s_derived(r, s)) =>
        {
            // mutação enraizada em `s`: válida só em cauda; args seguintes podem LER `s`
            tail && args[1..].iter().all(|a| linear_ok(a, s, pos, false, lin))
        }
        Ast::Call { callee: Callee::Prim(p), args }
            if matches!(p, Prim::Nth | Prim::Get | Prim::Count | Prim::Contains)
                && args.first().is_some_and(|r| ast_is_local(r, s)) =>
        {
            // leitura de `s`: válida em qualquer posição
            args[1..].iter().all(|a| linear_ok(a, s, pos, false, lin))
        }
        // ADR-0010: chamada a função linear consumindo `s` no seu parâmetro linear;
        // válida só em cauda (o resultado vira o próximo `s`). Demais args podem LER
        // `s` mas não escapá-lo/mutá-lo. O transiente threada pela função (dispatch
        // em runtime de conj/assoc sobre T_TVEC).
        Ast::Call { callee: Callee::Fn(name), args }
            if lin.get(name).is_some_and(|&j| args.get(j).is_some_and(|a| ast_is_local(a, s))) =>
        {
            let j = lin[name];
            tail && args.iter().enumerate().all(|(i, a)| i == j || linear_ok(a, s, pos, false, lin))
        }
        Ast::Call { args, .. } => args.iter().all(|a| linear_ok(a, s, pos, false, lin)),
        Ast::CallValue { f, args } => {
            linear_ok(f, s, pos, false, lin) && args.iter().all(|a| linear_ok(a, s, pos, false, lin))
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
            slots.iter().all(|(_, i)| linear_ok(i, s, pos, false, lin)) && linear_ok(body, s, pos, tail, lin)
        }
        Ast::Loop { slots, body } => {
            slots.iter().all(|(_, i)| linear_ok(i, s, pos, false, lin))
                && linear_ok(body, s, pos, false, lin)
        }
        Ast::MakeFn { captures, .. } => captures.iter().all(|c| linear_ok(c, s, pos, false, lin)),
        Ast::MakeRecord { fields, .. } => fields.iter().all(|(_, v)| linear_ok(v, s, pos, false, lin)),
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
    }
}
/// Conta ocorrências de `Local(s)` em `e`.
fn count_local(e: &Ast, s: u32) -> usize {
    let mut n = 0;
    fn go(e: &Ast, s: u32, n: &mut usize) {
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
/// Toda saída (cauda) de um corpo de loop é derivada do acumulador `s` (a função
/// retorna o acumulador)? `recur` diverge (não é saída).
fn all_escapes_derived(e: &Ast, s: u32, lin: &LinMap) -> bool {
    match e {
        Ast::Recur(_) => true,
        Ast::If(_, then, els) => all_escapes_derived(then, s, lin) && all_escapes_derived(els, s, lin),
        Ast::Do(stmts) => stmts.last().is_none_or(|l| all_escapes_derived(l, s, lin)),
        Ast::Let { body, .. } => all_escapes_derived(body, s, lin),
        other => transient_producing(other, s, lin),
    }
}
/// Sumário de linearidade (ADR-0010): posição do parâmetro que a função consome
/// linearmente e devolve. Stage 1: corpo é um `loop` cujo slot é init a esse
/// parâmetro, o parâmetro aparece só ali, o slot é usado linearmente e retornado.
fn linear_param(m: &FnMethod, lin: &LinMap) -> Option<usize> {
    if m.rest.is_some() {
        return None;
    }
    let Ast::Loop { slots, body } = &m.body else {
        return None;
    };
    for pi in 0..m.params.len() as u32 {
        // slot cujo init é exatamente o parâmetro `pi`
        let Some(acc_pos) = slots.iter().position(|(_, init)| ast_is_local(init, pi)) else {
            continue;
        };
        let acc_slot = slots[acc_pos].0;
        // `pi` aparece só uma vez em todo o corpo (aquele init) e os outros inits não o usam
        if count_local(&m.body, pi) != 1 {
            continue;
        }
        if linear_ok(body, acc_slot, acc_pos, true, lin) && all_escapes_derived(body, acc_slot, lin) {
            return Some(pi as usize);
        }
    }
    None
}
/// Reescreve conj/assoc enraizados em `s` para conj!/assoc! (segue a cadeia).
fn rewrite_bang(e: &mut Ast, s: u32) {
    if let Ast::Call { callee: Callee::Prim(p), args } = e {
        if matches!(p, Prim::Conj | Prim::Assoc) && args.first().is_some_and(|r| s_derived(r, s)) {
            rewrite_bang(&mut args[0], s);
            *p = if *p == Prim::Conj { Prim::ConjBang } else { Prim::AssocBang };
        }
    }
}
/// Valor que continua transiente (arg de recur do próprio slot): desce em
/// `if`/`do`/`let` até os valores-folha e reescreve conj/assoc → conj!/assoc!.
fn rewrite_transient_value(e: &mut Ast, s: u32) {
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
/// Transforma o corpo: nas saídas (cauda) congela valores que produzem o
/// transiente com `persistent!`; nos recur do próprio slot reescreve conj/assoc →
/// conj!/assoc! (chamadas a funções lineares threadam o transiente sem reescrita —
/// conj/assoc dispatcham sobre T_TVEC em runtime, ADR-0010).
fn transform_body(e: &mut Ast, s: u32, pos: usize, tail: bool, lin: &LinMap) {
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
fn linearize_loop(slots: &mut [(u32, Ast)], body: &mut Ast, lin: &LinMap) {
    let n = slots.len();
    // Decide todos os slots elegíveis sobre o corpo ORIGINAL, depois transforma.
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
/// Percorre `e` (bottom-up) e lineariza cada `loop` usando os sumários `lin`.
fn walk_and_linearize(e: &mut Ast, lin: &LinMap) {
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
            slots.iter_mut().for_each(|(_, i)| walk_and_linearize(i, lin));
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
        Ast::MakeFn { captures, .. } => captures.iter_mut().for_each(|c| walk_and_linearize(c, lin)),
        Ast::MakeRecord { fields, .. } => {
            fields.iter_mut().for_each(|(_, v)| walk_and_linearize(v, lin))
        }
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
/// Post-pass de auto-transient (ADR-0009 rec.2 + ADR-0010): computa os sumários de
/// linearidade das funções e lineariza os acumuladores de loop (intra e inter).
fn optimize_transients(functions: &mut [Function], main_body: &mut [Ast]) {
    // Sumários (stage 1: não-recursivos — um passe, sem chaining de lineares).
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
        "get" => Prim::Get,
        "nth" => Prim::Nth,
        "assoc" => Prim::Assoc,
        "dissoc" => Prim::Dissoc,
        "contains?" => Prim::Contains,
        "keys" => Prim::Keys,
        "vals" => Prim::Vals,
        "conj" => Prim::Conj,
        "vector" => Prim::Vector,
        "hash-map" => Prim::HashMap,
        "hash-set" => Prim::HashSet,
        "set" => Prim::HashSet,
        "sorted-map" => Prim::SortedMap,
        "sorted-set" => Prim::SortedSet,
        "compare" => Prim::Compare,
        "throw" => Prim::Throw,
        "transient" => Prim::Transient,
        "persistent!" => Prim::PersistentBang,
        "conj!" => Prim::ConjBang,
        "assoc!" => Prim::AssocBang,
        "dissoc!" => Prim::DissocBang,
        "slurp" => Prim::Slurp,
        "spit" => Prim::Spit,
        "file-exists?" => Prim::FileExists,
        "getenv" => Prim::Getenv,
        _ => return None,
    })
}

/// Aridade canônica de uma primitiva usada como *valor* (para HOF). `None` p/
/// variádicas (str/list/vector/hash-map/hash-set/println/print).
fn prim_value_arity(prim: Prim) -> Option<usize> {
    Some(match prim {
        Prim::Inc
        | Prim::Dec
        | Prim::Not
        | Prim::NilP
        | Prim::EmptyP
        | Prim::First
        | Prim::Rest
        | Prim::Count
        | Prim::Keys
        | Prim::Throw
        | Prim::Transient
        | Prim::PersistentBang
        | Prim::Slurp
        | Prim::FileExists
        | Prim::Getenv
        | Prim::Vals => 1,
        Prim::Add
        | Prim::Sub
        | Prim::Mul
        | Prim::Quot
        | Prim::Mod
        | Prim::Eq
        | Prim::Lt
        | Prim::Le
        | Prim::Gt
        | Prim::Ge
        | Prim::Cons
        | Prim::Get
        | Prim::Nth
        | Prim::Dissoc
        | Prim::Contains
        | Prim::Compare
        | Prim::ConjBang
        | Prim::DissocBang
        | Prim::Spit
        | Prim::Conj => 2,
        Prim::Assoc | Prim::AssocBang => 3,
        Prim::Str
        | Prim::List
        | Prim::Vector
        | Prim::HashMap
        | Prim::HashSet
        | Prim::SortedMap
        | Prim::SortedSet
        | Prim::Println
        | Prim::Try // sintetizada; nunca usada como valor de 1ª classe
        | Prim::Print => return None,
    })
}

fn check_prim_arity(prim: Prim, n: usize, span: Span) -> Result<(), Diagnostic> {
    let ok = match prim {
        Prim::Sub | Prim::Add | Prim::Mul => n >= 1,
        Prim::Quot
        | Prim::Mod
        | Prim::Cons
        | Prim::Get
        | Prim::Dissoc
        | Prim::Contains
        | Prim::ConjBang
        | Prim::DissocBang
        | Prim::Spit
        | Prim::Conj => n == 2,
        Prim::Nth => n == 2 || n == 3, // ADR-0008: aridade 2 e 3 (not-found)
        Prim::AssocBang => n == 3,
        Prim::Assoc => n >= 3 && n % 2 == 1, // coll + um ou mais pares
        Prim::Inc
        | Prim::Dec
        | Prim::Not
        | Prim::NilP
        | Prim::EmptyP
        | Prim::First
        | Prim::Rest
        | Prim::Count
        | Prim::Keys
        | Prim::Throw
        | Prim::Transient
        | Prim::PersistentBang
        | Prim::Slurp
        | Prim::FileExists
        | Prim::Getenv
        | Prim::Vals => n == 1,
        Prim::Try => n == 3,
        Prim::Eq | Prim::Lt | Prim::Le | Prim::Gt | Prim::Ge | Prim::Compare => n == 2,
        Prim::HashMap | Prim::SortedMap => n & 1 == 0,
        Prim::List
        | Prim::Str
        | Prim::Println
        | Prim::Print
        | Prim::Vector
        | Prim::HashSet
        | Prim::SortedSet => true,
    };
    if ok {
        Ok(())
    } else {
        Err(Diagnostic::error(
            "E0105",
            format!("aridade inválida para primitiva ({n} args)"),
        )
        .with_span(span))
    }
}

fn unsupported(msg: impl Into<String>, span: Span) -> Diagnostic {
    Diagnostic::error("E0100", msg)
        .with_span(span)
        .with_help("fora do subconjunto compilável atual; ver specs/LANGUAGE_SCOPE.md e IMPLEMENTATION_PLAN.md")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prog(src: &str) -> Program {
        let forms = clojure_reader::read_all(0, src).expect("lê");
        analyze(&forms).expect("analisa")
    }
    fn err(src: &str) -> Diagnostics {
        let forms = clojure_reader::read_all(0, src).expect("lê");
        analyze(&forms).unwrap_err()
    }

    #[test]
    fn analyzes_fn_and_main() {
        let p = prog("(ns h.core)\n(defn fib [n] (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))\n(defn -main [] (println (fib 10)))\n(-main)");
        assert_eq!(p.functions.iter().filter(|f| !f.is_lambda).count(), 2);
        assert_eq!(p.main_body.len(), 1);
    }

    #[test]
    fn closure_captures() {
        // adder captura n; a lambda tem 1 captura.
        let p = prog("(defn adder [n] (fn [x] (+ x n)))");
        let lam = p.functions.iter().find(|f| f.is_lambda).unwrap();
        assert_eq!(lam.methods[0].params, vec!["x"]);
        // O MakeFn no corpo de adder deve capturar 1 valor.
        if let Ast::MakeFn {
            captures, arity, ..
        } = &p
            .functions
            .iter()
            .find(|f| f.name == "adder")
            .unwrap()
            .methods[0]
            .body
        {
            assert_eq!(*arity, 1);
            assert_eq!(captures.len(), 1);
            assert!(matches!(captures[0], Ast::Local(_)));
        } else {
            panic!("corpo de adder deveria ser MakeFn");
        }
    }

    #[test]
    fn higher_order_call_value() {
        let p = prog("(defn ap [f x] (f x))");
        let ap = p.functions.iter().find(|f| f.name == "ap").unwrap();
        assert!(matches!(ap.methods[0].body, Ast::CallValue { .. }));
    }

    #[test]
    fn fn_as_value_is_fnref() {
        let p = prog("(defn inc1 [x] (+ x 1))\n(defn use [] (ap inc1 5))\n(defn ap [f x] (f x))");
        let usef = p.functions.iter().find(|f| f.name == "use").unwrap();
        if let Ast::Call { args, .. } = &usef.methods[0].body {
            assert!(matches!(args[0], Ast::FnRef(_)));
        } else {
            panic!("esperava Call");
        }
    }

    #[test]
    fn macros_expand() {
        let p = prog("(defn f [n] (cond (< n 0) -1 :else 1))");
        // cond vira if aninhado
        assert!(matches!(p.functions[0].methods[0].body, Ast::If(..)));
    }

    #[test]
    fn unresolved_is_error() {
        assert_eq!(err("(defn f [] (nope 1))").items[0].code, "E0101");
    }
}
