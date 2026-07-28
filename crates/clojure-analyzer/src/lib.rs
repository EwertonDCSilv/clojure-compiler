//! Semantic analysis and closure conversion for the compilable subset.
//!
//! [`analyze`] expands supported core macros, resolves lexical locals, captures,
//! top-level functions, protocols, records, and multimethods, validates call and
//! `recur` arities, and produces a backend-oriented [`Program`]. Closure
//! conversion assigns numeric frame slots and explicit capture vectors.
//! Unsupported source constructs become stable diagnostics; code generation
//! never needs to recover source-level binding rules.

use clojure_diagnostics::{Diagnostic, Diagnostics};
use clojure_span::Span;
use clojure_syntax::{Form, SForm};
use std::collections::HashMap;

mod expand;
pub use expand::expand_all;

/// Backend-oriented expression in the compilable subset.
///
/// Local and capture names have already been resolved to numeric slots. The AST
/// intentionally contains no source spans; all user-facing errors are emitted
/// while analysis still owns the spanned source forms.
#[derive(Debug, Clone)]
pub enum Ast {
    /// Signed 64-bit integer literal.
    Int(i64),
    /// IEEE-754 double literal (boxed at runtime).
    Float(f64),
    /// Boolean literal.
    Bool(bool),
    /// The `nil` literal.
    Nil,
    /// Owned UTF-8 string literal.
    Str(String),
    /// Keyword encoded without the leading colon.
    Keyword(String),
    /// Vector literal whose elements evaluate left to right.
    VecLit(Vec<Ast>),
    /// Set literal whose elements evaluate left to right.
    SetLit(Vec<Ast>),
    /// Map literal whose key/value pairs preserve source order.
    MapLit(Vec<(Ast, Ast)>),
    /// Read a top-level `def` global by its permanent-root index (ADR-0013).
    GlobalRef(u32),
    /// Initialize a top-level `def` global once, in source order (ADR-0013).
    DefGlobal { index: u32, value: Box<Ast> },
    /// Local slot in the current function or lambda frame.
    Local(u32),
    /// Captured value read from `self->freev[index]`.
    Capture(u32),
    /// Top-level function used as a value; lowering creates a zero-capture closure.
    FnRef(String),
    /// Closure construction with captures evaluated in the enclosing context.
    MakeFn {
        /// Generated top-level symbol containing the lambda body.
        lambda: String,
        /// Canonical callable arity stored in the closure header.
        arity: usize,
        /// Capture expressions in the lambda's assigned capture-slot order.
        captures: Vec<Ast>,
    },
    /// Conditional expression: test, then branch, else branch.
    If(Box<Ast>, Box<Ast>, Box<Ast>),
    /// Ordered expression sequence whose last value is returned.
    Do(Vec<Ast>),
    /// Lexical bindings followed by their body.
    Let {
        /// Local slot and initializer pairs in evaluation order.
        slots: Vec<(u32, Ast)>,
        /// Expression evaluated after all bindings are installed.
        body: Box<Ast>,
    },
    /// Recur target with mutable iteration slots.
    Loop {
        /// Local slot and initial-value pairs.
        slots: Vec<(u32, Ast)>,
        /// Loop body that may produce [`Ast::Recur`].
        body: Box<Ast>,
    },
    /// Tail transfer to the nearest loop or function method.
    Recur(Vec<Ast>),
    /// Direct call to a primitive or known top-level function.
    Call {
        /// Statically selected target.
        callee: Callee,
        /// Eager argument expressions in call order.
        args: Vec<Ast>,
    },
    /// Indirect invocation of a first-class callable value.
    CallValue {
        /// Expression producing the callable.
        f: Box<Ast>,
        /// Eager argument expressions.
        args: Vec<Ast>,
    },
    /// `(apply f a b ... coll)` with explicit fixed and spread arguments.
    Apply {
        /// Expression producing the callable.
        f: Box<Ast>,
        /// Arguments preceding the final collection.
        fixed: Vec<Ast>,
        /// Collection whose elements complete the argument vector.
        coll: Box<Ast>,
    },
    /// Record construction from a type name and field expressions.
    MakeRecord {
        /// Declared record type.
        type_name: String,
        /// Field names and values in declaration order.
        fields: Vec<(String, Ast)>,
    },
    /// Runtime registration of a protocol or multimethod implementation.
    RegisterMethod {
        /// Analyzer-assigned dispatch table identifier.
        method_id: i64,
        /// Dispatch key expression.
        key: Box<Ast>,
        /// Closure implementing the method.
        impl_fn: Box<Ast>,
    },
    /// Runtime registration of a multimethod dispatch function.
    RegisterMulti {
        /// Analyzer-assigned multimethod identifier.
        method_id: i64,
        /// Function applied to invocation arguments to obtain the dispatch key.
        dispatch_fn: Box<Ast>,
    },
}

/// Statically resolved target of a direct call.
#[derive(Debug, Clone)]
pub enum Callee {
    /// Built-in operation lowered through a known runtime or fast path.
    Prim(Prim),
    /// Top-level function symbol.
    Fn(String),
}

/// Built-in operation recognized by semantic analysis.
///
/// Variants are an internal Rust/backend contract. `clojure-codegen` maps each
/// operation to inline lowering, a C ABI symbol, or a synthesized control-flow
/// sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prim {
    /// Numeric addition.
    Add,
    /// Numeric subtraction.
    Sub,
    /// Numeric multiplication.
    Mul,
    /// Truncating integer quotient.
    Quot,
    /// Euclidean integer remainder.
    Mod,
    /// Increment an integer.
    Inc,
    /// Decrement an integer.
    Dec,
    /// Structural equality.
    Eq,
    /// Numeric less-than comparison.
    Lt,
    /// Numeric less-than-or-equal comparison.
    Le,
    /// Numeric greater-than comparison.
    Gt,
    /// Numeric greater-than-or-equal comparison.
    Ge,
    /// Clojure logical negation.
    Not,
    /// Test for `nil`.
    NilP,
    /// Test whether a supported collection is empty.
    EmptyP,
    /// Prepend an item to a sequence.
    Cons,
    /// Return the first sequence item.
    First,
    /// Return the sequence after its first item.
    Rest,
    /// Count a supported collection.
    Count,
    /// Construct a list.
    List,
    /// Concatenate printable values as a string.
    Str,
    /// Print values followed by a newline.
    Println,
    /// Print values without a newline.
    Print,
    /// Perform associative lookup.
    Get,
    /// Perform indexed lookup.
    Nth,
    /// Associate one or more key/value pairs.
    Assoc,
    /// Remove an associative key.
    Dissoc,
    /// Test for an associative key or set member.
    Contains,
    /// Return map keys.
    Keys,
    /// Return map values.
    Vals,
    /// Add an item to a persistent collection.
    Conj,
    /// Construct a vector.
    Vector,
    /// Construct a hash map.
    HashMap,
    /// Construct a hash set.
    HashSet,
    /// Construct a sorted map.
    SortedMap,
    /// Construct a sorted set.
    SortedSet,
    /// Compare two supported values.
    Compare,
    /// Throw a native exception value.
    Throw,
    /// Synthesized try/catch/finally operation.
    Try,
    /// Convert a persistent collection to a transient.
    Transient,
    /// Freeze a transient collection.
    PersistentBang,
    /// Add an item to a transient collection.
    ConjBang,
    /// Associate a key/value pair in a transient collection.
    AssocBang,
    /// Remove a key from a transient collection.
    DissocBang,
    /// Read a UTF-8 file as a string.
    Slurp,
    /// Write a string to a file.
    Spit,
    /// Test whether a filesystem path exists.
    FileExists,
    /// Read a process environment variable.
    Getenv,
    /// Synthesized output capture operation.
    WithOutStr,
    /// Read a built-in dynamic Var.
    VarGet,
    /// Invoke a thunk with one built-in dynamic Var rebound.
    WithBinding,
    /// Read one line from the current input stream.
    ReadLine,
    /// Create an in-memory string reader.
    StringReader,
    /// Convert an integer to a character.
    CharOf,
    /// Convert a character to its integer code point.
    IntOf,
    /// Test whether a value is a character.
    CharP,
    /// Read one character from the current input stream.
    ReadChar,
    /// Join two path components.
    PathJoin,
    /// Return the final component of a path.
    FileName,
    /// Return the parent of a path.
    Parent,
    /// Encode a string as bytes.
    Bytes,
    /// Decode bytes as a string.
    BytesToString,
    /// Read one byte by index.
    Bget,
    /// Read a file as bytes.
    SlurpBytes,
    /// Write bytes to a file.
    SpitBytes,
    /// Parse one string at native runtime.
    ReadString,
    /// Open a file-backed writer.
    WriterOpen,
    /// Open a file-backed reader.
    ReaderOpen,
    /// Close a closeable stream.
    Close,
    /// Flush the current output stream.
    Flush,
    /// Create one directory.
    Mkdir,
    /// Create a directory hierarchy.
    Mkdirs,
    /// List directory entries.
    ListDir,
    /// Delete a file.
    DeleteFile,
    /// Rename a filesystem entry.
    Rename,
    /// Test whether a path names a directory.
    DirectoryP,
    /// Test whether a path names a regular file.
    FileP,
    /// File size in bytes.
    FileSize,
    /// File last-modified time in seconds.
    FileModified,
    /// Division (`/`): exact fixnum quotient when divisible, otherwise a double.
    Div,
    /// Test whether a value is a boxed float.
    FloatP,
    /// Coerce a number to a boxed double.
    DoubleOf,
}

/// Maps built-in dynamic Vars to the C runtime's stable IDs.
///
/// ABI: values must match `enum DynVarId` in `runtime/85_writers.c`.
fn dyn_var_id(name: &str) -> Option<i64> {
    match name {
        "*out*" => Some(0),
        "*err*" => Some(1),
        "*flush-on-newline*" => Some(2),
        "*in*" => Some(3),
        "*command-line-args*" => Some(4),
        _ => None,
    }
}

/// One fixed or variadic arity of a function.
#[derive(Debug, Clone)]
pub struct FnMethod {
    /// Fixed parameter names in source order.
    pub params: Vec<String>,
    /// Optional variadic rest parameter.
    pub rest: Option<String>,
    /// Analyzed method body.
    pub body: Ast,
}

impl FnMethod {
    /// Returns the parameter slots occupied by this arity.
    pub fn nslots(&self) -> usize {
        self.params.len() + self.rest.is_some() as usize
    }
}

/// Top-level code-generation unit.
#[derive(Debug, Clone)]
pub struct Function {
    /// Linkage symbol, including generated lambda names.
    pub name: String,
    /// One or more arities; at most one is variadic and it has greatest arity.
    pub methods: Vec<FnMethod>,
    /// Maximum local slots over all arities, excluding the implicit `self`.
    pub local_count: u32,
    /// Whether the function reads captures through the implicit `self`.
    pub is_lambda: bool,
    /// Dispatch strategy; dispatch stubs have no method bodies.
    pub dispatch: Dispatch,
}

/// Runtime dispatch strategy for a top-level symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dispatch {
    /// Ordinary function with directly compiled methods.
    None,
    /// Protocol dispatch via `lookup(method_id, type_key(arg0))`.
    Protocol(i64),
    /// Multimethod dispatch via a registered dispatch function and `:default`.
    Multi(i64),
}

/// Complete analyzer output consumed by native code generation.
#[derive(Debug, Clone)]
pub struct Program {
    /// Top-level definitions and generated lambda functions.
    pub functions: Vec<Function>,
    /// Initialization and top-level expressions, in source order.
    pub main_body: Vec<Ast>,
    /// Local slots required by the synthesized native entry point.
    pub main_local_count: u32,
    /// Number of top-level `def` globals; each owns a permanent GC root slot.
    pub global_count: u32,
}

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
    let expanded = expand::expand_all(forms);
    analyze_expanded(&expanded)
}

fn analyze_expanded(forms: &[SForm]) -> Result<Program, Diagnostics> {
    let mut diags = Diagnostics::new();

    // Pass 1 collects top-level signatures so forward calls have arity data.
    let mut sigs: HashMap<String, Vec<(usize, bool)>> = HashMap::new();
    // Protocol methods map to (runtime method ID, arity); record names form the
    // user-defined dispatch-key set.
    let mut protos: HashMap<String, (i64, usize)> = HashMap::new();
    let mut records: std::collections::HashSet<String> = std::collections::HashSet::new();
    // Multimethod names share the positive runtime method-ID namespace.
    let mut multis: HashMap<String, i64> = HashMap::new();
    // Top-level `def` globals: name -> permanent-root index, in source order.
    let mut globals: HashMap<String, u32> = HashMap::new();
    let mut next_mid: i64 = 1;
    for f in forms {
        if let Some((name, _)) = match_def(f) {
            if globals.contains_key(&name) {
                diags.push(
                    Diagnostic::error("E0113", format!("redefinição de def não suportada: {name}"))
                        .with_span(f.span),
                );
            } else {
                let idx = globals.len() as u32;
                globals.insert(name, idx);
            }
        } else if let Some((name, decls)) = match_defn(f) {
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
        globals: &globals,
        frames: Vec::new(),
        functions: Vec::new(),
        lam: 0,
    };
    let mut main_body = Vec::new();

    // The synthetic main frame remains at the bottom while function frames are
    // pushed and popped above it.
    an.frames.push(Frame::new(false));

    for f in forms {
        if is_ns(f) {
            continue;
        }
        if let Some((name, value_form)) = match_def(f) {
            let index = an.globals[&name];
            match an.analyze(&value_form, false) {
                Ok(value) => main_body.push(Ast::DefGlobal {
                    index,
                    value: Box::new(value),
                }),
                Err(d) => diags.push(d),
            }
        } else if let Some((name, decls)) = match_defn(f) {
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

fn is_ns(f: &SForm) -> bool {
    matches!(f.node.strip_meta(), Form::List(items)
        if matches!(items.first().map(|h| h.node.strip_meta()), Some(Form::Symbol(n)) if n.ns.is_none() && n.name == "ns"))
}

type MethodDecl = (Vec<String>, Option<String>, Vec<SForm>);

/// Recognizes single- and multi-arity `defn` and returns its declarations.
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
    // Docstrings affect documentation, not the compiled method body.
    let mut rest = &items[2..];
    if rest.len() > 1 && matches!(rest[0].node.strip_meta(), Form::Str(_)) {
        rest = &rest[1..];
    }
    let methods = parse_methods(rest)?;
    Some((name, methods))
}

/// Recognizes `(def name value)` (top-level data). Returns the name and the
/// initializer form. A docstring between name and value is ignored.
fn match_def(f: &SForm) -> Option<(String, SForm)> {
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

/// Extracts a protocol name and `(method, arity)` declarations.
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
        // Protocol docstrings and other non-method forms do not affect dispatch.
    }
    Some((name, methods))
}

/// Extracts an `extend-type` target and its method implementations.
///
/// Interleaved protocol names are ignored because current dispatch identity is
/// the method name.
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
        // A protocol-name symbol does not define a method and is ignored.
    }
    Some((typename, impls))
}

/// Extracts a multimethod name and dispatch-function form.
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

/// Extracts a method name, dispatch value, parameters, and body from `defmethod`.
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

/// Maps core capability names to reserved negative method IDs.
///
/// ABI: values must match `CORE_ASSOC_ONE`, `CORE_NTH`, and `CORE_NTH_OR` in the
/// C runtime. These IDs are deliberately separate from program protocol IDs.
fn core_capability_mid(mname: &str) -> Option<i64> {
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

/// Parses either `[params] body...` or multi-arity method lists.
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

/// Splits a parameter vector into fixed names and an optional rest name.
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

/// Lexical frame for one function or lambda analysis.
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
    /// Top-level `def` name -> permanent-root index (ADR-0013 Gate 1).
    globals: &'a HashMap<String, u32>,
    frames: Vec<Frame>,
    functions: Vec<Function>,
    lam: u32,
}

/// Returns whether any declared arity accepts `n` arguments.
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

    /// Resolves `name` in frame `i` as a local, capture, or missing value.
    ///
    /// Crossing lambda boundaries records transitive captures in deterministic
    /// first-use order.
    fn resolve_from(&mut self, i: usize, name: &str) -> Option<Ast> {
        if let Some((_, slot)) = self.frames[i].locals.iter().rev().find(|(n, _)| n == name) {
            return Some(Ast::Local(*slot));
        }
        if self.frames[i].is_lambda {
            if let Some(idx) = self.frames[i].capture_names.iter().position(|n| n == name) {
                return Some(Ast::Capture(idx as u32));
            }
        }
        // Only a lambda captures values from its enclosing frame.
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
            Form::Float(x) => Ok(Ast::Float(*x)),
            Form::Str(s) => Ok(Ast::Str(s.clone())),
            Form::Char(c) => Ok(Ast::Call {
                // Character literals lower through `char` to an immediate code point.
                callee: Callee::Prim(Prim::CharOf),
                args: vec![Ast::Int(*c as u32 as i64)],
            }),
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

    /// Resolves a symbol in value position to a local, capture, or function ref.
    fn analyze_symbol_value(
        &mut self,
        ns: &Option<String>,
        name: &str,
        span: Span,
    ) -> Result<Ast, Diagnostic> {
        // A namespace-qualified reference (`alias/name` or `ns.name/name`) resolves
        // to the simple `name` in the flat project symbol space (ADR-0013 Gate 1).
        // Local shadowing does not apply to qualified references.
        if ns.is_some() {
            if let Some(&idx) = self.globals.get(name) {
                return Ok(Ast::GlobalRef(idx));
            }
            if self.sigs.contains_key(name) {
                return Ok(Ast::FnRef(name.to_string()));
            }
            return Err(unsupported(
                format!("referência qualificada não resolvida: {name}"),
                span,
            ));
        }
        if let Some(ast) = self.resolve(name) {
            return Ok(ast);
        }
        if let Some(id) = dyn_var_id(name) {
            // A dynamic Var in value position lowers to `(var-get id)`.
            return Ok(Ast::Call {
                callee: Callee::Prim(Prim::VarGet),
                args: vec![Ast::Int(id)],
            });
        }
        if let Some(&idx) = self.globals.get(name) {
            return Ok(Ast::GlobalRef(idx)); // top-level def global (ADR-0013)
        }
        if self.sigs.contains_key(name) {
            return Ok(Ast::FnRef(name.to_string()));
        }
        if let Some(prim) = prim_of(name) {
            // A primitive used as a value gets a synthesized callable wrapper.
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

        // A non-symbol operator is an indirect call through a callable value.
        let Form::Symbol(op) = head.node.strip_meta() else {
            let f = self.analyze(head, false)?;
            let a = self.analyze_seq(args)?;
            return Ok(Ast::CallValue {
                f: Box::new(f),
                args: a,
            });
        };
        // A qualified operator (`alias/fn`) is never a special form; resolve the
        // simple name in the flat project symbol space (ADR-0013 Gate 1).
        if op.ns.is_some() {
            return self.resolve_call(&op.name, args, span);
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
            "with-out-str" => {
                // `with-out-str` turns its body into a thunk rebound through *out*.
                let body_fn = self.make_lambda(vec![], args.to_vec(), span)?;
                Ok(Ast::Call {
                    callee: Callee::Prim(Prim::WithOutStr),
                    args: vec![body_fn],
                })
            }
            "binding" => self.analyze_binding(args, span),
            "with-open" => self.analyze_with_open(args, span),
            "with-in-str" => {
                // `with-in-str` binds *in* to a Reader over the supplied string.
                if args.is_empty() {
                    return Err(unsupported("with-in-str requer a string de entrada", span));
                }
                let s = self.analyze(&args[0], false)?;
                let reader = Ast::Call {
                    callee: Callee::Prim(Prim::StringReader),
                    args: vec![s],
                };
                let body_fn = self.make_lambda(vec![], args[1..].to_vec(), span)?;
                Ok(Ast::Call {
                    callee: Callee::Prim(Prim::WithBinding),
                    args: vec![Ast::Int(3), reader, body_fn], // 3 = *in*
                })
            }
            "__cljn-with-binding" => {
                // Internal binding form produced by desugaring: `(id value thunk)`.
                let id = self.analyze(&args[0], false)?;
                let val = self.analyze(&args[1], false)?;
                let thunk = self.analyze(&args[2], false)?;
                Ok(Ast::Call {
                    callee: Callee::Prim(Prim::WithBinding),
                    args: vec![id, val, thunk],
                })
            }
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
            name => self.resolve_call(name, args, span),
        }
    }

    /// Resolves a call whose operator is a simple `name`: primitive, local/capture
    /// (indirect), top-level function, or a `def` global holding a callable.
    fn resolve_call(&mut self, name: &str, args: &[SForm], span: Span) -> Result<Ast, Diagnostic> {
        if let Some(prim) = prim_of(name) {
            check_prim_arity(prim, args.len(), span)?;
            let a = self.analyze_seq(args)?;
            Ok(Ast::Call {
                callee: Callee::Prim(prim),
                args: a,
            })
        } else if let Some(ast) = self.resolve(name) {
            // A local or capture in operator position requires indirect call.
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
        } else if let Some(&idx) = self.globals.get(name) {
            // A global holding a callable value is invoked indirectly.
            let a = self.analyze_seq(args)?;
            Ok(Ast::CallValue {
                f: Box::new(Ast::GlobalRef(idx)),
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
        // A loop body is a tail context for its own recur regardless of whether
        // the loop expression itself occupies an outer tail position.
        let _ = tail;
        self.top().recur_arity.push(slots.len());
        let body = self.analyze_body(&args[1..], span, true);
        self.top().recur_arity.pop();
        let body = body?;
        let fr = self.top();
        fr.locals.truncate(saved);
        fr.next_slot = saved_next;
        // Auto-transient linearization is a whole-Program post-pass because it
        // requires summaries for every top-level function (ADR-0009/ADR-0010).
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

    /// Analyzes function arities in a dedicated lexical frame.
    ///
    /// Returns methods, maximum slot count, and captures. All arities of a
    /// lambda share one capture layout.
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
            // Each arity has its own slots but reuses the function's frame region.
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

    /// Creates a single fixed-arity lambda and its explicit capture expression.
    ///
    /// This common path is used by source `fn` and synthesized `try` thunks.
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

    /// Lowers `try`, optional `catch`, and optional `finally` into thunks.
    ///
    /// Body, handler, and cleanup become capture-aware lambdas passed to
    /// [`Prim::Try`]. Catch is catch-all in the current subset; the source class
    /// form is accepted but has no runtime hierarchy semantics.
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

    /// Lowers `binding` to nested runtime rebinding calls and zero-arity thunks.
    ///
    /// The runtime restores each dynamic Var even on exception. Binding values
    /// evaluate in order after the preceding rebind; this differs from the JVM
    /// only when a later value reads a Var already rebound by the same form.
    fn analyze_binding(&mut self, args: &[SForm], span: Span) -> Result<Ast, Diagnostic> {
        let first = args
            .first()
            .ok_or_else(|| unsupported("binding requer vetor de bindings", span))?;
        let Form::Vector(binds) = first.node.strip_meta() else {
            return Err(unsupported("binding requer vetor de bindings", span));
        };
        if binds.len() % 2 != 0 {
            return Err(unsupported("binding: bindings em pares", span));
        }
        let mut pairs: Vec<(i64, SForm)> = Vec::new();
        let mut i = 0;
        while i < binds.len() {
            let vf = &binds[i];
            let Form::Symbol(nm) = vf.node.strip_meta() else {
                return Err(unsupported(
                    "binding: alvo deve ser uma Var dinâmica (símbolo)",
                    vf.span,
                ));
            };
            let id = dyn_var_id(&nm.name).ok_or_else(|| {
                unsupported(
                    format!(
                        "binding: `{}` não é Var dinâmica suportada (*out*/*err*/*flush-on-newline*)",
                        nm.name
                    ),
                    vf.span,
                )
            })?;
            pairs.push((id, binds[i + 1].clone()));
            i += 2;
        }
        let sf = |f: Form| SForm::new(f, span);
        let mut acc: Vec<SForm> = args[1..].to_vec();
        for (id, val) in pairs.into_iter().rev() {
            let mut thunk_items = vec![sf(Form::sym("fn*")), sf(Form::Vector(vec![]))];
            thunk_items.extend(acc);
            let thunk = sf(Form::List(thunk_items));
            acc = vec![sf(Form::List(vec![
                sf(Form::sym("__cljn-with-binding")),
                sf(Form::Int(id)),
                val,
                thunk,
            ]))];
        }
        match acc.len() {
            0 => Ok(Ast::Nil),
            1 => self.analyze(&acc[0], false),
            _ => self.analyze_body(&acc, span, false),
        }
    }

    /// Lowers `with-open` to nested bindings and `try/finally` cleanup.
    ///
    /// Resources close in reverse acquisition order, including exceptional exit.
    fn analyze_with_open(&mut self, args: &[SForm], span: Span) -> Result<Ast, Diagnostic> {
        let first = args
            .first()
            .ok_or_else(|| unsupported("with-open requer vetor de bindings", span))?;
        let Form::Vector(binds) = first.node.strip_meta() else {
            return Err(unsupported("with-open requer vetor de bindings", span));
        };
        if binds.len() % 2 != 0 {
            return Err(unsupported("with-open: bindings em pares", span));
        }
        let mut pairs: Vec<(SForm, SForm)> = Vec::new();
        let mut i = 0;
        while i < binds.len() {
            let nf = &binds[i];
            if !matches!(nf.node.strip_meta(), Form::Symbol(_)) {
                return Err(unsupported("with-open: alvo deve ser um símbolo", nf.span));
            }
            pairs.push((nf.clone(), binds[i + 1].clone()));
            i += 2;
        }
        let sf = |f: Form| SForm::new(f, span);
        let mut acc: Vec<SForm> = args[1..].to_vec();
        for (name, res) in pairs.into_iter().rev() {
            let close_call = sf(Form::List(vec![sf(Form::sym("close")), name.clone()]));
            let finally = sf(Form::List(vec![sf(Form::sym("finally")), close_call]));
            let mut try_items = vec![sf(Form::sym("try"))];
            try_items.extend(acc);
            try_items.push(finally);
            let let_form = sf(Form::List(vec![
                sf(Form::sym("let*")),
                sf(Form::Vector(vec![name, res])),
                sf(Form::List(try_items)),
            ]));
            acc = vec![let_form];
        }
        match acc.len() {
            0 => Ok(Ast::Nil),
            1 => self.analyze(&acc[0], false),
            _ => self.analyze_body(&acc, span, false),
        }
    }

    /// Analyzes named, anonymous, single-, or multi-arity function syntax.
    ///
    /// The result is a generated lambda function plus [`Ast::MakeFn`] containing
    /// capture expressions evaluated in the enclosing frame.
    fn analyze_fn(&mut self, args: &[SForm], span: Span) -> Result<Ast, Diagnostic> {
        let mut idx = 0;
        if matches!(
            args.first().map(|f| f.node.strip_meta()),
            Some(Form::Symbol(_))
        ) {
            idx = 1; // Optional name; self-reference is not currently modeled.
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

// ===== Automatic loop transients (ADR-0009 and ADR-0010) =================
//
// A loop slot initialized by a fresh vector may become transient when every use
// is linear: rooted conj/assoc updates, recognized reads, transfer through the
// same recur slot, or escape as the final result. Capturing, storing, aliasing,
// rebinding to another slot, or passing to an unknown function rejects the
// candidate. Accepted exits are frozen with persistent!.
//
// INVARIANT: the analysis is conservative. Any unrecognized use keeps the
// original persistent representation, so the transformation changes only
// internal cost, never source semantics.

fn ast_is_local(e: &Ast, s: u32) -> bool {
    matches!(e, Ast::Local(x) if *x == s)
}
/// Tests whether `e` is slot `s` or a persistent update chain rooted at `s`.
fn s_derived(e: &Ast, s: u32) -> bool {
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
type LinMap = HashMap<String, usize>;
/// Tests whether `e` calls a summarized function with `s` in its linear position.
fn is_lin_call(e: &Ast, s: u32, lin: &LinMap) -> bool {
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
fn transient_producing(e: &Ast, s: u32, lin: &LinMap) -> bool {
    s_derived(e, s) || is_lin_call(e, s, lin)
}
/// Verifies that every occurrence of slot `s` occupies an accepted linear use.
///
/// `tail` permits a derived value to escape or rebind here. `pos` is the slot's
/// position in the loop binding vector and therefore in each [`Ast::Recur`].
fn linear_ok(e: &Ast, s: u32, pos: usize, tail: bool, lin: &LinMap) -> bool {
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
/// Tests whether every terminating loop path returns a value derived from `s`.
///
/// `recur` is a back edge, not a terminating path.
fn all_escapes_derived(e: &Ast, s: u32, lin: &LinMap) -> bool {
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
fn linear_param(m: &FnMethod, lin: &LinMap) -> Option<usize> {
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
fn rewrite_bang(e: &mut Ast, s: u32) {
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
/// Applies transient operations and freezes every terminating tail result.
///
/// Recur transfers use mutating updates. Calls summarized as linear thread the
/// transient unchanged because runtime conj/assoc dispatch recognizes `T_TVEC`.
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
fn optimize_transients(functions: &mut [Function], main_body: &mut [Ast]) {
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
        "read-line" => Prim::ReadLine,
        "read-char" => Prim::ReadChar,
        "char" => Prim::CharOf,
        "int" => Prim::IntOf,
        "char?" => Prim::CharP,
        "path-join" => Prim::PathJoin,
        "file-name" => Prim::FileName,
        "parent" => Prim::Parent,
        "bytes" => Prim::Bytes,
        "bytes->string" => Prim::BytesToString,
        "bget" => Prim::Bget,
        "slurp-bytes" => Prim::SlurpBytes,
        "spit-bytes" => Prim::SpitBytes,
        "read-string" => Prim::ReadString,
        "writer" => Prim::WriterOpen,
        "reader" => Prim::ReaderOpen,
        "close" => Prim::Close,
        "flush" => Prim::Flush,
        "mkdir" => Prim::Mkdir,
        "mkdirs" => Prim::Mkdirs,
        "list-dir" => Prim::ListDir,
        "delete-file" => Prim::DeleteFile,
        "rename" => Prim::Rename,
        "directory?" => Prim::DirectoryP,
        "file?" => Prim::FileP,
        "file-size" => Prim::FileSize,
        "file-modified" => Prim::FileModified,
        "/" => Prim::Div,
        "float?" => Prim::FloatP,
        "double" => Prim::DoubleOf,
        _ => return None,
    })
}

/// Returns the canonical arity when a primitive is used as a first-class value.
///
/// Variadic and synthesized primitives return `None`.
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
        | Prim::CharOf
        | Prim::IntOf
        | Prim::CharP
        | Prim::FileName
        | Prim::Parent
        | Prim::Bytes
        | Prim::BytesToString
        | Prim::SlurpBytes
        | Prim::ReadString
        | Prim::WriterOpen
        | Prim::ReaderOpen
        | Prim::Close
        | Prim::Mkdir
        | Prim::Mkdirs
        | Prim::ListDir
        | Prim::DeleteFile
        | Prim::DirectoryP
        | Prim::FileP
        | Prim::FileSize
        | Prim::FileModified
        | Prim::FloatP
        | Prim::DoubleOf
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
        | Prim::PathJoin
        | Prim::Bget
        | Prim::SpitBytes
        | Prim::Rename
        | Prim::Div
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
        | Prim::WithOutStr // idem: só via forma especial
        | Prim::VarGet // sintetizada (leitura de Var dinâmica)
        | Prim::WithBinding // sintetizada (desugar de binding)
        | Prim::ReadLine // 0-ária; use (fn [] (read-line)) como valor
        | Prim::ReadChar // idem
        | Prim::Flush // 0-ária
        | Prim::StringReader // sintetizada (with-in-str)
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
        Prim::WithOutStr => n == 1,
        Prim::VarGet => n == 1,
        Prim::WithBinding => n == 3,
        Prim::ReadLine => n == 0,
        Prim::ReadChar => n == 0,
        Prim::StringReader => n == 1,
        Prim::CharOf | Prim::IntOf | Prim::CharP => n == 1,
        Prim::PathJoin => n == 2,
        Prim::FileName | Prim::Parent => n == 1,
        Prim::Bytes | Prim::BytesToString | Prim::SlurpBytes | Prim::ReadString => n == 1,
        Prim::Bget | Prim::SpitBytes => n == 2,
        Prim::WriterOpen | Prim::ReaderOpen | Prim::Close => n == 1,
        Prim::Flush => n == 0,
        Prim::Mkdir
        | Prim::Mkdirs
        | Prim::ListDir
        | Prim::DeleteFile
        | Prim::DirectoryP
        | Prim::FileP
        | Prim::FileSize
        | Prim::FileModified
        | Prim::FloatP
        | Prim::DoubleOf => n == 1,
        Prim::Rename | Prim::Div => n == 2,
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
        // `adder` captures `n`, so its generated lambda has one capture.
        let p = prog("(defn adder [n] (fn [x] (+ x n)))");
        let lam = p.functions.iter().find(|f| f.is_lambda).unwrap();
        assert_eq!(lam.methods[0].params, vec!["x"]);
        // The MakeFn inside `adder` must materialize exactly one capture.
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
        // `cond` expands to nested conditional expressions.
        assert!(matches!(p.functions[0].methods[0].body, Ast::If(..)));
    }

    #[test]
    fn unresolved_is_error() {
        assert_eq!(err("(defn f [] (nope 1))").items[0].code, "E0101");
    }
}
