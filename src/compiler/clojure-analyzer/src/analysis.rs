use crate::ast::*;
use crate::primitives::*;
use crate::top_level::*;
use clojure_diagnostics::Diagnostic;
use clojure_span::Span;
use clojure_syntax::{Form, SForm};
use std::collections::HashMap;

/// Lexical frame for one function or lambda analysis.
pub(crate) struct Frame {
    pub(crate) locals: Vec<(String, u32)>,
    pub(crate) next_slot: u32,
    pub(crate) max_slots: u32,
    pub(crate) recur_arity: Vec<usize>,
    pub(crate) is_lambda: bool,
    pub(crate) captures: Vec<Ast>,
    pub(crate) capture_names: Vec<String>,
}

impl Frame {
    pub(crate) fn new(is_lambda: bool) -> Self {
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

pub(crate) struct Analyzer<'a> {
    pub(crate) sigs: &'a HashMap<String, Vec<(usize, bool)>>,
    pub(crate) protos: &'a HashMap<String, (i64, usize)>,
    pub(crate) records: &'a std::collections::HashSet<String>,
    /// Mangled top-level `def` name -> permanent-root index (ADR-0013 Gate 1).
    pub(crate) globals: &'a HashMap<String, u32>,
    /// Per-namespace alias -> target-namespace, for qualified references.
    pub(crate) aliases: &'a HashMap<String, HashMap<String, String>>,
    /// Namespace of the current top-level form; drives `def`/`defn` mangling.
    pub(crate) cur_ns: String,
    pub(crate) frames: Vec<Frame>,
    pub(crate) functions: Vec<Function>,
    pub(crate) lam: u32,
}

/// Returns whether any declared arity accepts `n` arguments.
pub(crate) fn arity_accepts(arities: &[(usize, bool)], n: usize) -> bool {
    arities
        .iter()
        .any(|&(fixed, variadic)| if variadic { n >= fixed } else { n == fixed })
}

impl<'a> Analyzer<'a> {
    pub(crate) fn top(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    pub(crate) fn push_local(&mut self, name: String) -> u32 {
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
    pub(crate) fn resolve_from(&mut self, i: usize, name: &str) -> Option<Ast> {
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

    pub(crate) fn resolve(&mut self, name: &str) -> Option<Ast> {
        let top = self.frames.len() - 1;
        self.resolve_from(top, name)
    }

    pub(crate) fn analyze_body(
        &mut self,
        body: &[SForm],
        _span: Span,
        tail: bool,
    ) -> Result<Ast, Diagnostic> {
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

    pub(crate) fn analyze(&mut self, f: &SForm, tail: bool) -> Result<Ast, Diagnostic> {
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
            // A namespaced keyword is a keyword whose name contains a slash, e.g.
            // `:cljn.error/domain` -> keyword text `cljn.error/domain` (ADR-0013 §7).
            Form::Keyword(n) => Ok(Ast::Keyword(keyword_text(n))),
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
    pub(crate) fn analyze_symbol_value(
        &mut self,
        ns: &Option<String>,
        name: &str,
        span: Span,
    ) -> Result<Ast, Diagnostic> {
        // Unqualified names may be locals or dynamic Vars; qualified names never are.
        if ns.is_none() {
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
        }
        // Top-level reference: resolve per-namespace to a mangled symbol.
        if let Some(sym) = self.resolve_top(ns.as_deref(), name) {
            if let Some(&idx) = self.globals.get(&sym) {
                return Ok(Ast::GlobalRef(idx));
            }
            return Ok(Ast::FnRef(sym));
        }
        if ns.is_some() {
            return Err(unsupported(
                format!("referência qualificada não resolvida: {name}"),
                span,
            ));
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
                        optimization: MethodOptimization::default(),
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

    pub(crate) fn analyze_list(
        &mut self,
        items: &[SForm],
        span: Span,
        tail: bool,
    ) -> Result<Ast, Diagnostic> {
        let Some((head, args)) = items.split_first() else {
            return Err(unsupported("lista vazia não é compilável", span));
        };

        // `(:kw coll)` → (get coll :kw); supports namespaced keywords too.
        if let Form::Keyword(n) = head.node.strip_meta() {
            if args.len() == 1 {
                let coll = self.analyze(&args[0], false)?;
                return Ok(Ast::Call {
                    callee: Callee::Prim(Prim::Get),
                    args: vec![coll, Ast::Keyword(keyword_text(n))],
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
        // A qualified operator (`alias/fn`) is never a special form; resolve it
        // per-namespace (ADR-0013).
        if op.ns.is_some() {
            return self.resolve_call(op.ns.as_deref(), &op.name, args, span);
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
            name => self.resolve_call(None, name, args, span),
        }
    }

    /// Resolves a top-level reference to its mangled linker symbol (ADR-0013).
    ///
    /// Unqualified names try the current namespace's own `def`/`defn`, then a
    /// project-global protocol/multimethod/record name, then the auto-referred
    /// `clojure.core`. Qualified names resolve the alias (or literal namespace) to
    /// its target and mangle there.
    pub(crate) fn resolve_top(&self, ns: Option<&str>, name: &str) -> Option<String> {
        if let Some(q) = ns {
            let target = self
                .aliases
                .get(&self.cur_ns)
                .and_then(|m| m.get(q))
                .cloned()
                .unwrap_or_else(|| q.to_string());
            let sym = mangle(&target, name);
            if self.globals.contains_key(&sym) || self.sigs.contains_key(&sym) {
                return Some(sym);
            }
            // Fall back to a project-global name referenced through a namespace.
            if self.sigs.contains_key(name) {
                return Some(name.to_string());
            }
            return None;
        }
        let own = mangle(&self.cur_ns, name);
        if self.globals.contains_key(&own) || self.sigs.contains_key(&own) {
            return Some(own);
        }
        // Project-global protocol/multimethod/record-constructor name.
        if self.sigs.contains_key(name) {
            return Some(name.to_string());
        }
        let core = mangle("clojure.core", name);
        if self.globals.contains_key(&core) || self.sigs.contains_key(&core) {
            return Some(core);
        }
        None
    }

    /// Resolves a call operator: primitive, local/capture (indirect), top-level
    /// function, or a `def` global holding a callable. `ns` is the operator's
    /// namespace qualifier, if any.
    pub(crate) fn resolve_call(
        &mut self,
        ns: Option<&str>,
        name: &str,
        args: &[SForm],
        span: Span,
    ) -> Result<Ast, Diagnostic> {
        if ns.is_none() {
            if let Some(prim) = prim_of(name) {
                check_prim_arity(prim, args.len(), span)?;
                let a = self.analyze_seq(args)?;
                return Ok(Ast::Call {
                    callee: Callee::Prim(prim),
                    args: a,
                });
            }
            if let Some(ast) = self.resolve(name) {
                // A local or capture in operator position requires indirect call.
                let a = self.analyze_seq(args)?;
                return Ok(Ast::CallValue {
                    f: Box::new(ast),
                    args: a,
                });
            }
        }
        if let Some(sym) = self.resolve_top(ns, name) {
            if let Some(&idx) = self.globals.get(&sym) {
                // A global holding a callable value is invoked indirectly.
                let a = self.analyze_seq(args)?;
                return Ok(Ast::CallValue {
                    f: Box::new(Ast::GlobalRef(idx)),
                    args: a,
                });
            }
            if let Some(arities) = self.sigs.get(&sym) {
                if !arity_accepts(arities, args.len()) {
                    return Err(Diagnostic::error(
                        "E0103",
                        format!("aridade errada ao chamar {name}: recebeu {}", args.len()),
                    )
                    .with_span(span));
                }
                let a = self.analyze_seq(args)?;
                return Ok(Ast::Call {
                    callee: Callee::Fn(sym),
                    args: a,
                });
            }
        }
        Err(
            Diagnostic::error("E0101", format!("função não resolvida: {name}"))
                .with_span(span)
                .with_help("defina-a com defn, use uma primitiva, um local ou um fn"),
        )
    }

    pub(crate) fn analyze_seq(&mut self, forms: &[SForm]) -> Result<Vec<Ast>, Diagnostic> {
        forms.iter().map(|f| self.analyze(f, false)).collect()
    }

    pub(crate) fn analyze_let(
        &mut self,
        args: &[SForm],
        span: Span,
        tail: bool,
    ) -> Result<Ast, Diagnostic> {
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

    pub(crate) fn analyze_loop(
        &mut self,
        args: &[SForm],
        span: Span,
        tail: bool,
    ) -> Result<Ast, Diagnostic> {
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

    pub(crate) fn analyze_recur(
        &mut self,
        args: &[SForm],
        span: Span,
        tail: bool,
    ) -> Result<Ast, Diagnostic> {
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
    pub(crate) fn analyze_methods(
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
                    optimization: MethodOptimization::default(),
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
    pub(crate) fn make_lambda(
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
    pub(crate) fn analyze_try(&mut self, args: &[SForm], span: Span) -> Result<Ast, Diagnostic> {
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
    pub(crate) fn analyze_binding(
        &mut self,
        args: &[SForm],
        span: Span,
    ) -> Result<Ast, Diagnostic> {
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
    pub(crate) fn analyze_with_open(
        &mut self,
        args: &[SForm],
        span: Span,
    ) -> Result<Ast, Diagnostic> {
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
    pub(crate) fn analyze_fn(&mut self, args: &[SForm], span: Span) -> Result<Ast, Diagnostic> {
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
