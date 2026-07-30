//! Bootstrap interpreter for the source-level subset.
//!
//! [`Interp`] reads and evaluates Clojure forms with [`clojure_value::Value`],
//! installs fundamental primitives from Rust, and can load the self-hosted
//! bootstrap library in [`CORE_CLJ`]. It is useful for frontend execution and
//! macro bootstrap work. It is not the execution engine for generated programs:
//! native objects use the tagged C runtime owned by `clojure-codegen`.

use clojure_span::{Span, Spanned};
use clojure_syntax::{Form, Name, SForm};
use clojure_value::{Closure, FnMethod, List, Scope, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod primitives;

/// Minimal bootstrap `clojure.core` written in the interpreted subset.
pub const CORE_CLJ: &str = include_str!("core.clj");

/// Interpreter failure with an optional source range.
#[derive(Debug, Clone)]
pub struct EvalError {
    /// User-facing Portuguese error text.
    pub msg: String,
    /// Source form responsible for the failure, when known.
    pub span: Option<Span>,
}

impl EvalError {
    fn new(msg: impl Into<String>) -> EvalError {
        EvalError {
            msg: msg.into(),
            span: None,
        }
    }
    fn at(msg: impl Into<String>, span: Span) -> EvalError {
        EvalError {
            msg: msg.into(),
            span: Some(span),
        }
    }
}

/// Internal non-local control flow for an error or `recur`.
///
/// INVARIANT: only loop and function invocation boundaries consume `Recur`.
/// Reaching a top-level boundary is converted into an [`EvalError`].
enum Control {
    Err(EvalError),
    Recur(Vec<Value>),
}

impl From<EvalError> for Control {
    fn from(e: EvalError) -> Self {
        Control::Err(e)
    }
}

type E = Result<Value, Control>;

/// Mutable state of one bootstrap interpreter.
///
/// Globals occupy a flat table in this bootstrap implementation. Lexical
/// bindings live in immutable [`Scope`] chains, and output is captured instead
/// of being written directly to a process stream.
pub struct Interp {
    globals: HashMap<String, Value>,
    macros: std::collections::HashSet<&'static str>,
    current_ns: String,
    /// Output buffer shared with the installed `print` and `println` primitives.
    ///
    /// Prefer [`Self::take_output`] or [`Self::peek_output`] unless a primitive
    /// needs direct shared access.
    pub output: Rc<RefCell<String>>,
    gensym: u64,
}

impl Default for Interp {
    fn default() -> Self {
        Self::new()
    }
}

impl Interp {
    /// Creates an interpreter with Rust primitives and an empty global scope.
    ///
    /// The self-hosted [`CORE_CLJ`] library is not loaded; use
    /// [`Self::with_core`] for a ready-to-evaluate instance.
    pub fn new() -> Interp {
        let mut it = Interp {
            globals: HashMap::new(),
            macros: [
                "defn", "defn-", "let", "when", "when-not", "if-not", "cond", "and", "or", "->",
                "->>",
            ]
            .into_iter()
            .collect(),
            current_ns: "user".to_string(),
            output: Rc::new(RefCell::new(String::new())),
            gensym: 0,
        };
        primitives::install(&mut it);
        it
    }

    /// Removes and returns all captured program output.
    pub fn take_output(&self) -> String {
        std::mem::take(&mut self.output.borrow_mut())
    }

    /// Clones the captured output without clearing it.
    pub fn peek_output(&self) -> String {
        self.output.borrow().clone()
    }

    /// Creates an interpreter and evaluates the embedded bootstrap core.
    ///
    /// # Errors
    ///
    /// Returns the user-facing message from a reader or evaluation failure in
    /// the embedded core.
    pub fn with_core() -> Result<Interp, String> {
        let mut it = Interp::new();
        it.eval_source("core.clj", CORE_CLJ).map_err(|e| e.msg)?;
        Ok(it)
    }

    /// Defines or replaces one global value.
    pub fn define(&mut self, name: impl Into<String>, v: Value) {
        self.globals.insert(name.into(), v);
    }

    /// Borrows a global value by its unqualified storage name.
    pub fn get_global(&self, name: &str) -> Option<&Value> {
        self.globals.get(name)
    }

    fn next_gensym(&mut self, prefix: &str) -> String {
        self.gensym += 1;
        format!("{prefix}__{}__auto", self.gensym)
    }

    /// Reads and evaluates every top-level form in source order.
    ///
    /// The last value is returned, or `nil` when `text` contains no forms.
    /// Definitions mutate this interpreter and remain available to later calls.
    ///
    /// # Errors
    ///
    /// Returns a reader error prefixed with `name`, or the first evaluation
    /// error. Spans use source ID zero and byte offsets into `text`.
    ///
    /// # Examples
    ///
    /// ```
    /// use clojure_interp::Interp;
    /// use clojure_value::Value;
    ///
    /// let mut interpreter = Interp::new();
    /// assert_eq!(
    ///     interpreter.eval_source("example.clj", "(+ 20 22)").unwrap(),
    ///     Value::Int(42),
    /// );
    /// ```
    pub fn eval_source(&mut self, name: &str, text: &str) -> Result<Value, EvalError> {
        let forms = clojure_reader::read_all(0, text).map_err(|d| {
            let first = d.items.into_iter().next().unwrap();
            EvalError {
                msg: format!("{}: erro de leitura: {}", name, first.message),
                span: first.span,
            }
        })?;
        let mut last = Value::Nil;
        for f in &forms {
            last = self.eval_top(f)?;
        }
        Ok(last)
    }

    /// Evaluates one form without an enclosing lexical environment.
    ///
    /// # Errors
    ///
    /// Returns evaluation failures and rejects a `recur` that escapes its legal
    /// loop or function boundary.
    pub fn eval_top(&mut self, form: &SForm) -> Result<Value, EvalError> {
        match self.eval(form, &None) {
            Ok(v) => Ok(v),
            Err(Control::Err(e)) => Err(e),
            Err(Control::Recur(_)) => Err(EvalError::at("`recur` fora de loop/fn", form.span)),
        }
    }

    /// Invokes global `-main` with no arguments when it is defined.
    ///
    /// A missing entry point returns `nil`.
    ///
    /// # Errors
    ///
    /// Returns invocation, arity, or evaluation errors from `-main`.
    pub fn call_main(&mut self) -> Result<Value, EvalError> {
        if let Some(f) = self.globals.get("-main").cloned() {
            self.invoke(&f, vec![], None).map_err(|c| match c {
                Control::Err(e) => e,
                Control::Recur(_) => EvalError::new("`recur` inválido em -main"),
            })
        } else {
            Ok(Value::Nil)
        }
    }

    // -- Evaluation -------------------------------------------------------

    fn eval(&mut self, form: &SForm, env: &Option<Rc<Scope>>) -> E {
        match &form.node {
            Form::Nil => Ok(Value::Nil),
            Form::Bool(b) => Ok(Value::Bool(*b)),
            Form::Int(n) => Ok(Value::Int(*n)),
            Form::Float(x) => Ok(Value::Float(*x)),
            Form::Char(c) => Ok(Value::Char(*c)),
            Form::Str(s) => Ok(Value::str(s.as_str())),
            Form::Keyword(n) => Ok(Value::keyword(n.clone())),
            Form::Symbol(n) => self.resolve(n, env, form.span),
            Form::Vector(items) => {
                let mut v = Vec::with_capacity(items.len());
                for it in items {
                    v.push(self.eval(it, env)?);
                }
                Ok(Value::Vector(Rc::new(v)))
            }
            Form::Set(items) => {
                let mut v = Vec::with_capacity(items.len());
                for it in items {
                    let val = self.eval(it, env)?;
                    if !v.contains(&val) {
                        v.push(val);
                    }
                }
                Ok(Value::Set(Rc::new(v)))
            }
            Form::Map(pairs) => {
                let mut v = Vec::with_capacity(pairs.len());
                for (k, val) in pairs {
                    let kk = self.eval(k, env)?;
                    let vv = self.eval(val, env)?;
                    v.push((kk, vv));
                }
                Ok(Value::Map(Rc::new(v)))
            }
            Form::Meta { form, .. } => self.eval(form, env),
            Form::List(items) => self.eval_list(items, env, form.span),
        }
    }

    fn resolve(&self, n: &Name, env: &Option<Rc<Scope>>, span: Span) -> E {
        if n.ns.is_none() {
            if let Some(scope) = env {
                if let Some(v) = scope.lookup(&n.name) {
                    return Ok(v);
                }
            }
        }
        if let Some(v) = self.globals.get(&n.name) {
            return Ok(v.clone());
        }
        Err(Control::Err(EvalError::at(
            format!("símbolo não resolvido: {n}"),
            span,
        )))
    }

    fn eval_list(&mut self, items: &[SForm], env: &Option<Rc<Scope>>, span: Span) -> E {
        let Some((head, args)) = items.split_first() else {
            // An empty source list evaluates to the empty list value.
            return Ok(Value::List(List::empty()));
        };

        // Symbols in operator position select special forms and bootstrap
        // macros before ordinary value evaluation.
        if let Form::Symbol(name) = head.node.strip_meta() {
            if name.ns.is_none() {
                match name.name.as_str() {
                    "quote" => return self.sf_quote(args, span),
                    "if" => return self.sf_if(args, env, span),
                    "do" => return self.sf_do(args, env),
                    "let*" | "let" => return self.sf_let(args, env, span),
                    "fn*" | "fn" => return self.sf_fn(args, env, span),
                    "def" => return self.sf_def(args, env, span),
                    "loop*" | "loop" => return self.sf_loop(args, env, span),
                    "recur" => return self.sf_recur(args, env),
                    "ns" => return self.sf_ns(args),
                    "var" => return self.sf_var(args, span),
                    "apply" => return self.sf_apply(args, env, span),
                    "quote*" => return self.sf_quote(args, span),
                    n if self.macros.contains(n) => {
                        let expanded = self.expand_macro(n, args, span)?;
                        return self.eval(&expanded, env);
                    }
                    _ => {}
                }
            }
        }

        // Ordinary calls evaluate the callee and every argument eagerly.
        let f = self.eval(head, env)?;
        let mut argv = Vec::with_capacity(args.len());
        for a in args {
            argv.push(self.eval(a, env)?);
        }
        self.invoke(&f, argv, Some(span))
    }

    // -- Special forms ----------------------------------------------------

    fn sf_quote(&self, args: &[SForm], span: Span) -> E {
        let f = args
            .first()
            .ok_or_else(|| EvalError::at("quote requer 1 argumento", span))?;
        Ok(form_to_value(f))
    }

    fn sf_if(&mut self, args: &[SForm], env: &Option<Rc<Scope>>, span: Span) -> E {
        if args.len() < 2 || args.len() > 3 {
            return Err(EvalError::at("if requer test, then e (opcional) else", span).into());
        }
        let test = self.eval(&args[0], env)?;
        if test.is_truthy() {
            self.eval(&args[1], env)
        } else if let Some(else_) = args.get(2) {
            self.eval(else_, env)
        } else {
            Ok(Value::Nil)
        }
    }

    fn sf_do(&mut self, args: &[SForm], env: &Option<Rc<Scope>>) -> E {
        let mut last = Value::Nil;
        for a in args {
            last = self.eval(a, env)?;
        }
        Ok(last)
    }

    fn sf_let(&mut self, args: &[SForm], env: &Option<Rc<Scope>>, span: Span) -> E {
        let bindings = match args.first().map(|f| &f.node) {
            Some(Form::Vector(b)) => b,
            _ => return Err(EvalError::at("let requer vetor de bindings", span).into()),
        };
        if bindings.len() % 2 != 0 {
            return Err(EvalError::at("let: bindings em pares", span).into());
        }
        let mut scope_env = env.clone();
        for pair in bindings.chunks_exact(2) {
            let name = match pair[0].node.strip_meta() {
                Form::Symbol(n) if n.ns.is_none() => n.name.clone(),
                _ => {
                    return Err(EvalError::at(
                        "let: nome de binding deve ser símbolo simples",
                        pair[0].span,
                    )
                    .into())
                }
            };
            let val = self.eval(&pair[1], &scope_env)?;
            scope_env = Some(Scope::child(scope_env, vec![(name, val)]));
        }
        let mut last = Value::Nil;
        for b in &args[1..] {
            last = self.eval(b, &scope_env)?;
        }
        Ok(last)
    }

    fn sf_fn(&mut self, args: &[SForm], env: &Option<Rc<Scope>>, span: Span) -> E {
        let mut idx = 0;
        let mut name = None;
        if let Some(Form::Symbol(n)) = args.first().map(|f| f.node.strip_meta().clone()) {
            name = Some(n.name);
            idx = 1;
        }
        let rest = &args[idx..];
        let methods = if rest
            .first()
            .map(|f| matches!(f.node.strip_meta(), Form::Vector(_)))
            .unwrap_or(false)
        {
            // Single arity: (fn [params] body...).
            vec![parse_method(rest, span)?]
        } else {
            // Multiple arities: (fn ([p] b) ([p q] b)).
            let mut ms = Vec::new();
            for m in rest {
                match m.node.strip_meta() {
                    Form::List(items) => ms.push(parse_method(items, m.span)?),
                    _ => {
                        return Err(
                            EvalError::at("fn: método deve ser (params body...)", m.span).into(),
                        )
                    }
                }
            }
            ms
        };
        Ok(Value::Fn(Rc::new(Closure {
            name,
            methods,
            env: env.clone(),
        })))
    }

    fn sf_def(&mut self, args: &[SForm], env: &Option<Rc<Scope>>, span: Span) -> E {
        let name = match args.first().map(|f| f.node.strip_meta()) {
            Some(Form::Symbol(n)) => n.name.clone(),
            _ => return Err(EvalError::at("def requer um símbolo", span).into()),
        };
        let val = match args.get(1) {
            Some(init) => self.eval(init, env)?,
            None => Value::Nil,
        };
        self.globals.insert(name.clone(), val);
        Ok(Value::symbol(Name::simple(name)))
    }

    fn sf_loop(&mut self, args: &[SForm], env: &Option<Rc<Scope>>, span: Span) -> E {
        let bindings = match args.first().map(|f| &f.node) {
            Some(Form::Vector(b)) => b,
            _ => return Err(EvalError::at("loop requer vetor de bindings", span).into()),
        };
        if bindings.len() % 2 != 0 {
            return Err(EvalError::at("loop: bindings em pares", span).into());
        }
        let mut names = Vec::new();
        let mut vals = Vec::new();
        let mut scope_env = env.clone();
        for pair in bindings.chunks_exact(2) {
            let name = match pair[0].node.strip_meta() {
                Form::Symbol(n) if n.ns.is_none() => n.name.clone(),
                _ => {
                    return Err(
                        EvalError::at("loop: binding deve ser símbolo", pair[0].span).into(),
                    )
                }
            };
            let val = self.eval(&pair[1], &scope_env)?;
            scope_env = Some(Scope::child(
                scope_env.clone(),
                vec![(name.clone(), val.clone())],
            ));
            names.push(name);
            vals.push(val);
        }
        let body = &args[1..];
        // INVARIANT: each iteration binds exactly one value per loop name.
        // Recur carries the next value vector instead of using the Rust stack.
        loop {
            let iter_env = Some(Scope::child(
                env.clone(),
                names.iter().cloned().zip(vals.iter().cloned()).collect(),
            ));
            let mut last = Value::Nil;
            let mut recurred = false;
            for b in body {
                match self.eval(b, &iter_env) {
                    Ok(v) => last = v,
                    Err(Control::Recur(new)) => {
                        if new.len() != names.len() {
                            return Err(EvalError::at(
                                format!(
                                    "recur: esperava {} args, recebeu {}",
                                    names.len(),
                                    new.len()
                                ),
                                span,
                            )
                            .into());
                        }
                        vals = new;
                        recurred = true;
                        break;
                    }
                    Err(e) => return Err(e),
                }
            }
            if !recurred {
                return Ok(last);
            }
        }
    }

    fn sf_recur(&mut self, args: &[SForm], env: &Option<Rc<Scope>>) -> E {
        let mut v = Vec::with_capacity(args.len());
        for a in args {
            v.push(self.eval(a, env)?);
        }
        Err(Control::Recur(v))
    }

    fn sf_ns(&mut self, args: &[SForm]) -> E {
        if let Some(Form::Symbol(n)) = args.first().map(|f| f.node.strip_meta()) {
            self.current_ns = n.name.clone();
        }
        Ok(Value::Nil)
    }

    fn sf_var(&self, args: &[SForm], span: Span) -> E {
        // Bootstrap difference: (var x) returns the value directly because the
        // interpreter does not model a first-class Var object.
        match args.first().map(|f| f.node.strip_meta()) {
            Some(Form::Symbol(n)) => self
                .globals
                .get(&n.name)
                .cloned()
                .ok_or_else(|| EvalError::at(format!("var não encontrada: {n}"), span).into()),
            _ => Err(EvalError::at("var requer símbolo", span).into()),
        }
    }

    fn sf_apply(&mut self, args: &[SForm], env: &Option<Rc<Scope>>, span: Span) -> E {
        if args.len() < 2 {
            return Err(EvalError::at("apply requer fn e ao menos um argumento", span).into());
        }
        let f = self.eval(&args[0], env)?;
        let mut argv = Vec::new();
        for a in &args[1..args.len() - 1] {
            argv.push(self.eval(a, env)?);
        }
        let last = self.eval(&args[args.len() - 1], env)?;
        match seq_items(&last) {
            Some(items) => argv.extend(items),
            None => {
                return Err(
                    EvalError::at("apply: último argumento deve ser uma sequência", span).into(),
                )
            }
        }
        self.invoke(&f, argv, Some(span))
    }

    // -- Invocation -------------------------------------------------------

    /// Invokes a callable bootstrap value without a source call-site.
    ///
    /// # Errors
    ///
    /// Returns an arity, callability, primitive, or body-evaluation failure.
    /// An escaping `recur` is rejected as invalid.
    pub fn invoke_pub(&mut self, f: &Value, args: Vec<Value>) -> Result<Value, EvalError> {
        self.invoke(f, args, None).map_err(|c| match c {
            Control::Err(e) => e,
            Control::Recur(_) => EvalError::new("recur inválido"),
        })
    }

    fn invoke(&mut self, f: &Value, mut args: Vec<Value>, span: Option<Span>) -> E {
        match f {
            Value::Native(n) => (n.f)(&args).map_err(|msg| Control::Err(EvalError { msg, span })),
            Value::Keyword(_) => {
                // Keywords are unary map/set lookup functions.
                if args.len() != 1 {
                    return Err(
                        EvalError::new("keyword como fn requer 1 argumento (o mapa)").into(),
                    );
                }
                Ok(primitives::get(&args[0], f))
            }
            Value::Fn(closure) => {
                let method = closure.method_for(args.len()).ok_or_else(|| {
                    Control::Err(EvalError {
                        msg: format!(
                            "aridade errada: {} recebeu {} args",
                            closure.name.as_deref().unwrap_or("fn"),
                            args.len()
                        ),
                        span,
                    })
                })?;
                // INVARIANT: recur remains in this method. A changed argument
                // count is checked below and cannot select another overload.
                let method_idx = closure
                    .methods
                    .iter()
                    .position(|m| std::ptr::eq(m, method))
                    .unwrap();
                loop {
                    let bindings =
                        bind_params(&closure.methods[method_idx], &args).map_err(Control::Err)?;
                    let call_env = Some(Scope::child(closure.env.clone(), bindings));
                    let mut last = Value::Nil;
                    let mut recurred = false;
                    for b in &closure.methods[method_idx].body {
                        match self.eval(b, &call_env) {
                            Ok(v) => last = v,
                            Err(Control::Recur(new)) => {
                                args = new;
                                recurred = true;
                                break;
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    if !recurred {
                        return Ok(last);
                    }
                    if !closure.methods[method_idx].arity_matches(args.len()) {
                        return Err(EvalError {
                            msg: format!("recur com aridade incompatível: {} args", args.len()),
                            span,
                        }
                        .into());
                    }
                }
            }
            other => Err(EvalError {
                msg: format!("{} não é chamável", other.type_name()),
                span,
            }
            .into()),
        }
    }

    // -- Rust bootstrap macro expansion (ADR-0004) ------------------------

    fn expand_macro(&mut self, name: &str, args: &[SForm], span: Span) -> Result<SForm, Control> {
        let mk = |node| Spanned::new(node, span);
        let sym = |s: &str| Spanned::new(Form::sym(s), span);
        let list = |v: Vec<SForm>| Spanned::new(Form::List(v), span);

        match name {
            "defn" | "defn-" => {
                // (defn name doc? attrs? [params] body...) or multi-arity defn.
                let nm = args
                    .first()
                    .cloned()
                    .ok_or_else(|| Control::Err(EvalError::at("defn requer nome", span)))?;
                let mut rest: Vec<SForm> = args[1..].to_vec();
                // Docstrings and attribute maps are accepted syntactically but
                // are not retained as first-class Var metadata in bootstrap.
                if rest.len() > 1 && matches!(rest[0].node.strip_meta(), Form::Str(_)) {
                    rest.remove(0);
                }
                if rest.len() > 1 && matches!(rest[0].node.strip_meta(), Form::Map(_)) {
                    rest.remove(0);
                }
                let mut fn_form = vec![sym("fn*"), nm.clone()];
                fn_form.extend(rest);
                Ok(list(vec![sym("def"), nm, list(fn_form)]))
            }
            "let" => {
                let mut v = vec![sym("let*")];
                v.extend(args.iter().cloned());
                Ok(list(v))
            }
            "when" => {
                let test = arg(args, 0, "when", span)?;
                let mut do_form = vec![sym("do")];
                do_form.extend(args[1..].iter().cloned());
                Ok(list(vec![sym("if"), test, list(do_form)]))
            }
            "when-not" => {
                let test = arg(args, 0, "when-not", span)?;
                let mut do_form = vec![sym("do")];
                do_form.extend(args[1..].iter().cloned());
                Ok(list(vec![sym("if"), test, mk(Form::Nil), list(do_form)]))
            }
            "if-not" => {
                let test = arg(args, 0, "if-not", span)?;
                let then = arg(args, 1, "if-not", span)?;
                let els = args.get(2).cloned().unwrap_or_else(|| mk(Form::Nil));
                Ok(list(vec![sym("if"), test, els, then]))
            }
            "cond" => {
                if args.len() & 1 != 0 {
                    return Err(Control::Err(EvalError::at("cond requer pares", span)));
                }
                let mut result = mk(Form::Nil);
                for pair in args.chunks_exact(2).rev() {
                    let test = pair[0].clone();
                    let expr = pair[1].clone();
                    let is_else = matches!(&test.node, Form::Keyword(n) if n.ns.is_none() && n.name == "else");
                    result = if is_else {
                        expr
                    } else {
                        list(vec![sym("if"), test, expr, result])
                    };
                }
                Ok(result)
            }
            "and" => match args.len() {
                0 => Ok(mk(Form::Bool(true))),
                1 => Ok(args[0].clone()),
                _ => {
                    let g = self.next_gensym("and");
                    let gsym = Spanned::new(Form::sym(&g), span);
                    let mut rest_and = vec![sym("and")];
                    rest_and.extend(args[1..].iter().cloned());
                    let binding =
                        Spanned::new(Form::Vector(vec![gsym.clone(), args[0].clone()]), span);
                    Ok(list(vec![
                        sym("let*"),
                        binding,
                        list(vec![sym("if"), gsym.clone(), list(rest_and), gsym]),
                    ]))
                }
            },
            "or" => match args.len() {
                0 => Ok(mk(Form::Nil)),
                1 => Ok(args[0].clone()),
                _ => {
                    let g = self.next_gensym("or");
                    let gsym = Spanned::new(Form::sym(&g), span);
                    let mut rest_or = vec![sym("or")];
                    rest_or.extend(args[1..].iter().cloned());
                    let binding =
                        Spanned::new(Form::Vector(vec![gsym.clone(), args[0].clone()]), span);
                    Ok(list(vec![
                        sym("let*"),
                        binding,
                        list(vec![sym("if"), gsym.clone(), gsym, list(rest_or)]),
                    ]))
                }
            },
            "->" => {
                let mut expr = arg(args, 0, "->", span)?;
                for step in &args[1..] {
                    expr = thread_into(step, expr, true, span);
                }
                Ok(expr)
            }
            "->>" => {
                let mut expr = arg(args, 0, "->>", span)?;
                for step in &args[1..] {
                    expr = thread_into(step, expr, false, span);
                }
                Ok(expr)
            }
            _ => Err(Control::Err(EvalError::at(
                format!("macro desconhecida: {name}"),
                span,
            ))),
        }
    }
}

fn arg(args: &[SForm], i: usize, who: &str, span: Span) -> Result<SForm, Control> {
    args.get(i)
        .cloned()
        .ok_or_else(|| Control::Err(EvalError::at(format!("{who}: argumento {i} ausente"), span)))
}

/// Inserts `expr` as the first (`->`) or last (`->>`) argument of `step`.
fn thread_into(step: &SForm, expr: SForm, first: bool, span: Span) -> SForm {
    match step.node.strip_meta() {
        Form::List(items) => {
            let head = items[0].clone();
            let mut v = vec![head];
            if first {
                v.push(expr);
                v.extend(items[1..].iter().cloned());
            } else {
                v.extend(items[1..].iter().cloned());
                v.push(expr);
            }
            Spanned::new(Form::List(v), span)
        }
        // A symbol step such as (-> x f) becomes (f x).
        _ => Spanned::new(Form::List(vec![step.clone(), expr]), span),
    }
}

fn parse_method(items: &[SForm], span: Span) -> Result<FnMethod, EvalError> {
    let params_form = items
        .first()
        .ok_or_else(|| EvalError::at("fn: faltam parâmetros", span))?;
    let params_vec = match params_form.node.strip_meta() {
        Form::Vector(p) => p,
        _ => {
            return Err(EvalError::at(
                "fn: parâmetros devem ser um vetor",
                params_form.span,
            ))
        }
    };
    let mut params = Vec::new();
    let mut rest = None;
    let mut it = params_vec.iter();
    while let Some(p) = it.next() {
        match p.node.strip_meta() {
            Form::Symbol(n) if n.ns.is_none() && n.name == "&" => {
                let r = it
                    .next()
                    .ok_or_else(|| EvalError::at("fn: `&` sem símbolo de rest", p.span))?;
                match r.node.strip_meta() {
                    Form::Symbol(rn) if rn.ns.is_none() => rest = Some(rn.name.clone()),
                    _ => return Err(EvalError::at("fn: rest deve ser símbolo simples", r.span)),
                }
            }
            Form::Symbol(n) if n.ns.is_none() => params.push(n.name.clone()),
            _ => {
                return Err(EvalError::at(
                    "fn: destructuring ainda não suportado no interpretador de bootstrap",
                    p.span,
                ))
            }
        }
    }
    Ok(FnMethod {
        params,
        rest,
        body: items[1..].to_vec(),
    })
}

fn bind_params(method: &FnMethod, args: &[Value]) -> Result<Vec<(String, Value)>, EvalError> {
    if method.rest.is_some() {
        if args.len() < method.params.len() {
            return Err(EvalError::new(format!(
                "aridade errada: esperava ao menos {}, recebeu {}",
                method.params.len(),
                args.len()
            )));
        }
    } else if args.len() != method.params.len() {
        return Err(EvalError::new(format!(
            "aridade errada: esperava {}, recebeu {}",
            method.params.len(),
            args.len()
        )));
    }
    let mut bindings = Vec::with_capacity(method.params.len() + 1);
    for (p, a) in method.params.iter().zip(args.iter()) {
        bindings.push((p.clone(), a.clone()));
    }
    if let Some(rest) = &method.rest {
        let rest_vals: Vec<Value> = args[method.params.len()..].to_vec();
        let rest_list = if rest_vals.is_empty() {
            Value::Nil
        } else {
            Value::List(List::from_vec(rest_vals))
        };
        bindings.push((rest.clone(), rest_list));
    }
    Ok(bindings)
}

/// Converts source syntax to bootstrap data for `quote`.
///
/// Metadata wrappers are intentionally stripped because bootstrap values do not
/// carry metadata.
pub fn form_to_value(f: &SForm) -> Value {
    match f.node.strip_meta() {
        Form::Nil => Value::Nil,
        Form::Bool(b) => Value::Bool(*b),
        Form::Int(n) => Value::Int(*n),
        Form::Float(x) => Value::Float(*x),
        Form::Char(c) => Value::Char(*c),
        Form::Str(s) => Value::str(s.as_str()),
        Form::Symbol(n) => Value::symbol(n.clone()),
        Form::Keyword(n) => Value::keyword(n.clone()),
        Form::List(items) => Value::List(List::from_vec(items.iter().map(form_to_value).collect())),
        Form::Vector(items) => Value::Vector(Rc::new(items.iter().map(form_to_value).collect())),
        Form::Set(items) => Value::Set(Rc::new(items.iter().map(form_to_value).collect())),
        Form::Map(pairs) => Value::Map(Rc::new(
            pairs
                .iter()
                .map(|(k, v)| (form_to_value(k), form_to_value(v)))
                .collect(),
        )),
        Form::Meta { .. } => unreachable!("strip_meta remove Meta"),
    }
}

/// Materializes a sequential bootstrap value for `apply` and rest binding.
///
/// Returns `None` for maps, scalars, and functions. The result is eager and
/// cloned; this interpreter does not model lazy sequences.
pub fn seq_items(v: &Value) -> Option<Vec<Value>> {
    match v {
        Value::Nil => Some(vec![]),
        Value::List(l) => Some(l.iter().cloned().collect()),
        Value::Vector(v) => Some(v.as_ref().clone()),
        Value::Set(s) => Some(s.as_ref().clone()),
        Value::Str(s) => Some(s.chars().map(Value::Char).collect()),
        _ => None,
    }
}

#[cfg(test)]
#[path = "../tests/unit/tests/mod.rs"]
mod tests;
