//! Native object generation from an analyzed compiler program.
//!
//! [`compile_object`] lowers a [`Program`] through Cranelift into a relocatable
//! object for the host target. Generated code and the embedded C runtime share a
//! tagged 64-bit `Value` representation: fixnums use `(n << 1) | 1`, heap
//! pointers have low bits `000`, and several scalar values are immediate.
//!
//! All callable entries use the uniform ABI `(self, argc, argv) -> Value`.
//! Heap-capable values live in a generated shadow stack across allocation
//! safepoints. Semantic slow paths, allocation, collections, exceptions,
//! dispatch, and I/O are provided by [`RUNTIME_C`]; selected fixnum and vector
//! operations have guarded inline fast paths.

use clojure_analyzer::{Ast, Callee, Dispatch, FnMethod, Prim, Program};
use clojure_diagnostics::{Diagnostic, Diagnostics};
use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::MemFlagsData as M;
use cranelift_codegen::ir::{types, AbiParam, Block, InstBuilder, Value as CValue};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::{isa, Context};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{default_libcall_names, DataDescription, DataId, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use target_lexicon::Triple;

mod constants;
mod diagnostics;
mod gc_frame;
mod ir_adapter;
mod options;
mod runtime_abi;
mod stats;
mod value;

pub use options::{CodegenOptions, IrExperiment, IrOptimizationMode, OptimizationLevel};
pub use stats::OptimizationStats;

use constants::{
    collect_strings, CONST_CACHE_MAX, FALSEV, FIX_MAX, FIX_MIN, NIL, PV_COUNT, PV_ROOT, PV_SHIFT,
    PV_TAIL, PV_TAILLEN, TRUEV, T_VEC, VNODE_SLOTS,
};
use diagnostics::{single, single_d};
use runtime_abi::{declare_runtime, Runtime};
use value::{prim_fixnum_result, prim_imm_result, Flow, VKind};

macro_rules! embed_runtime_modules {
    ($(($name:literal, $path:literal)),+ $(,)?) => {
        /// Amalgamated C runtime source compiled during the native link step.
        ///
        /// Runtime fragments remain separate for review and subsystem tests but
        /// are concatenated in the declared order. The C compiler receives one
        /// translation unit, preserving internal visibility and cross-subsystem
        /// optimization.
        pub const RUNTIME_C: &str = concat!($(include_str!($path)),+);

        #[cfg(test)]
        const RUNTIME_MODULES: &[(&str, &str)] = &[
            $(($name, include_str!($path))),+
        ];
    };
}

embed_runtime_modules!(
    ("types", "../runtime/00_types.c"),
    ("gc", "../runtime/10_gc.c"),
    (
        "values-and-functions",
        "../runtime/20_values_and_functions.c"
    ),
    ("vector", "../runtime/30_vector.c"),
    ("hash-collections", "../runtime/40_hash_collections.c"),
    ("sorted-collections", "../runtime/50_sorted_collections.c"),
    (
        "records-and-dispatch",
        "../runtime/60_records_and_dispatch.c"
    ),
    ("transients", "../runtime/70_transients.c"),
    ("core-operations", "../runtime/80_core_operations.c"),
    ("writers", "../runtime/85_writers.c"),
    ("print", "../runtime/90_print.c"),
    ("exceptions", "../runtime/100_exceptions.c"),
    ("multimethods", "../runtime/110_multimethods.c"),
    ("test-introspection", "../runtime/120_test_introspection.c"),
    ("io", "../runtime/130_io.c"),
    ("reader", "../runtime/140_reader.c"),
    ("http", "../runtime/150_http.c"),
    ("server", "../runtime/160_server.c"),
);

/// Compiles `program` into a relocatable object for the host target.
///
/// Uses [`CodegenOptions::default`].
///
/// # Errors
///
/// Returns diagnostics when the host ISA, Cranelift configuration, symbol
/// declaration, lowering, verification, or object emission fails.
pub fn compile_object(program: &Program) -> Result<Vec<u8>, Diagnostics> {
    compile_object_with_options(program, CodegenOptions::default())
}

/// Compiles `program` with explicit backend options.
///
/// Output contains generated program functions and external references to the C
/// runtime ABI; the CLI is responsible for compiling [`RUNTIME_C`] and linking
/// both objects into an executable.
///
/// # Errors
///
/// Returns diagnostics when the target or Cranelift setup fails, an analyzed AST
/// violates a lowering invariant, generated IR fails verification, or the object
/// cannot be emitted.
pub fn compile_object_with_options(
    program: &Program,
    options: CodegenOptions,
) -> Result<Vec<u8>, Diagnostics> {
    compile_object_with_options_and_stats(program, options).map(|(object, _)| object)
}

/// Compiles a program and returns deterministic structural lowering metrics.
///
/// The object bytes are identical to [`compile_object_with_options`] for the
/// same inputs and options; collecting counters does not alter generated IR.
pub fn compile_object_with_options_and_stats(
    program: &Program,
    options: CodegenOptions,
) -> Result<(Vec<u8>, OptimizationStats), Diagnostics> {
    if options.ir_experiment != IrExperiment::None
        && options.ir_optimization != IrOptimizationMode::Safe
    {
        return Err(single(
            "experimentos de IR requerem `--ir-opt safe`".to_owned(),
        ));
    }
    let optimized_program;
    let program = match options.ir_optimization {
        IrOptimizationMode::None => program,
        IrOptimizationMode::Safe => {
            optimized_program = ir_adapter::optimize_program(program).map_err(|diagnostic| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.push(diagnostic);
                diagnostics
            })?;
            &optimized_program
        }
    };
    let mut flags = settings::builder();
    flags
        .set("is_pic", "true")
        .map_err(|e| single(format!("falha ao configurar is_pic: {e}")))?;
    flags
        .set("opt_level", options.optimization_level.cranelift_value())
        .map_err(|e| single(format!("falha ao configurar opt_level: {e}")))?;
    let isa = isa::lookup(Triple::host())
        .map_err(|e| single(format!("ISA host não suportada: {e}")))?
        .finish(settings::Flags::new(flags))
        .map_err(|e| single(format!("falha ao configurar ISA: {e}")))?;

    let builder = ObjectBuilder::new(isa, "clojure_native", default_libcall_names())
        .map_err(|e| single(format!("falha no ObjectBuilder: {e}")))?;
    let mut module = ObjectModule::new(builder);
    let ptr = module.target_config().pointer_type();
    let runtime = declare_runtime(&mut module, ptr);
    let enable_adr15 = options.ir_experiment == IrExperiment::Adr15;
    let stats = RefCell::new(OptimizationStats::default());

    // Materialize unique string bytes as local object data.
    let mut strings: Vec<String> = Vec::new();
    for f in &program.functions {
        for m in &f.methods {
            collect_strings(&m.body, &mut strings);
        }
    }
    for a in &program.main_body {
        collect_strings(a, &mut strings);
    }
    let mut str_data: HashMap<String, (DataId, usize)> = HashMap::new();
    for (i, s) in strings.iter().enumerate() {
        if str_data.contains_key(s) {
            continue;
        }
        let id = module
            .declare_data(&format!("str{i}"), Linkage::Local, false, false)
            .map_err(|e| single(format!("declare_data: {e}")))?;
        let mut d = DataDescription::new();
        // Raw bytes are length-delimited rather than NUL-terminated.
        let bytes = if s.is_empty() {
            vec![0u8]
        } else {
            s.clone().into_bytes()
        };
        d.define(bytes.into_boxed_slice());
        module
            .define_data(id, &d)
            .map_err(|e| single(format!("define_data: {e}")))?;
        str_data.insert(s.clone(), (id, s.len()));
    }

    // Declare all functions before definitions for recursion and forward refs.
    // ABI: entry(self, argc, argv) -> Value. Static calls pass NIL as `self`;
    // argv points to arguments rooted in the shadow stack.
    let entry_sig = |m: &mut ObjectModule| {
        let mut sig = m.make_signature();
        sig.params.push(AbiParam::new(types::I64)); // self
        sig.params.push(AbiParam::new(types::I64)); // argc
        sig.params.push(AbiParam::new(ptr)); // argv
        sig.returns.push(AbiParam::new(types::I64));
        sig
    };
    let mut fn_ids: HashMap<String, (FuncId, usize)> = HashMap::new();
    for f in &program.functions {
        let sig = entry_sig(&mut module);
        let id = module
            .declare_function(&f.name, Linkage::Local, &sig)
            .map_err(|e| single(format!("declare_function {}: {e}", f.name)))?;
        // Store the first arity for zero-capture FnRef construction.
        let arity0 = f.methods.first().map(|m| m.params.len()).unwrap_or(0);
        fn_ids.insert(f.name.clone(), (id, arity0));
    }

    // ADR-0015: eligible closed methods receive a raw-fixnum entry in addition
    // to the generic tagged callable entry. Symbols are compiler-private and
    // indexed rather than derived from source names, keeping them deterministic
    // and valid for every namespace spelling.
    let mut specialized_fn_ids: HashMap<(String, usize), FuncId> = HashMap::new();
    if enable_adr15 {
        for (function_index, function) in program.functions.iter().enumerate() {
            if function.dispatch != Dispatch::None || function.is_lambda {
                continue;
            }
            for (method_index, method) in function.methods.iter().enumerate() {
                if !method.optimization.specialized_fixnum_abi {
                    continue;
                }
                let mut signature = module.make_signature();
                for _ in &method.params {
                    signature.params.push(AbiParam::new(types::I64));
                }
                signature.returns.push(AbiParam::new(types::I64));
                let symbol = format!("__cljn_adr15_{function_index}_{method_index}");
                let id = module
                    .declare_function(&symbol, Linkage::Local, &signature)
                    .map_err(|error| single(format!("declare_function {symbol}: {error}")))?;
                specialized_fn_ids.insert((function.name.clone(), method.params.len()), id);
            }
        }
    }

    let mut diags = Diagnostics::new();
    let next_const = std::cell::Cell::new(0u32); // sites de vetor literal constante

    for f in &program.functions {
        let (id, _) = fn_ids[&f.name];
        let mut ctx = Context::new();
        ctx.func.signature = module
            .declarations()
            .get_function_decl(id)
            .signature
            .clone();
        let mut fbctx = FunctionBuilderContext::new();
        let res = {
            let fg = FnGen::new(
                &mut module,
                &mut ctx.func,
                &mut fbctx,
                ptr,
                &runtime,
                &fn_ids,
                &specialized_fn_ids,
                &str_data,
                &next_const,
                enable_adr15,
                &stats,
            );
            fg.build_entry(&f.methods, f.local_count, f.dispatch)
        };
        match res {
            Ok(()) => {
                if let Err(e) = module.define_function(id, &mut ctx) {
                    diags.push(single_d(format!("define_function {}: {e}", f.name)));
                }
            }
            Err(d) => diags.push(d),
        }
    }

    if enable_adr15 {
        for function in &program.functions {
            for method in &function.methods {
                let Some(id) = specialized_fn_ids
                    .get(&(function.name.clone(), method.params.len()))
                    .copied()
                else {
                    continue;
                };
                let mut ctx = Context::new();
                ctx.func.signature = module
                    .declarations()
                    .get_function_decl(id)
                    .signature
                    .clone();
                let mut fbctx = FunctionBuilderContext::new();
                let result = {
                    let generator = FnGen::new(
                        &mut module,
                        &mut ctx.func,
                        &mut fbctx,
                        ptr,
                        &runtime,
                        &fn_ids,
                        &specialized_fn_ids,
                        &str_data,
                        &next_const,
                        true,
                        &stats,
                    );
                    generator.build_specialized_fixnum_entry(method)
                };
                match result {
                    Ok(()) => {
                        if let Err(error) = module.define_function(id, &mut ctx) {
                            diags.push(single_d(format!(
                                "define_function {} aridade {}: {error}",
                                function.name,
                                method.params.len()
                            )));
                        }
                    }
                    Err(diagnostic) => diags.push(diagnostic),
                }
            }
        }
    }

    {
        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(types::I32)); // Raw C argc.
        sig.params.push(AbiParam::new(ptr)); // Raw C argv.
        sig.returns.push(AbiParam::new(types::I32));
        let main_id = module
            .declare_function("main", Linkage::Export, &sig)
            .map_err(|e| single(format!("declare main: {e}")))?;
        let mut ctx = Context::new();
        ctx.func.signature = sig;
        let mut fbctx = FunctionBuilderContext::new();
        let res = {
            let fg = FnGen::new(
                &mut module,
                &mut ctx.func,
                &mut fbctx,
                ptr,
                &runtime,
                &fn_ids,
                &specialized_fn_ids,
                &str_data,
                &next_const,
                enable_adr15,
                &stats,
            );
            fg.build_main(&program.main_body, program.main_local_count)
        };
        match res {
            Ok(()) => {
                if let Err(e) = module.define_function(main_id, &mut ctx) {
                    diags.push(single_d(format!("define main: {e}")));
                }
            }
            Err(d) => diags.push(d),
        }
    }

    if diags.has_errors() {
        return Err(diags);
    }
    let product = module.finish();
    let object = product
        .emit()
        .map_err(|e| single(format!("emit do objeto: {e}")))?;
    Ok((object, stats.into_inner()))
}

/// Recur target containing a loop header and slots to rebind.
#[derive(Clone)]
struct RecurTarget {
    header: Block,
    slots: Vec<u32>,
}

struct FnGen<'a> {
    module: &'a mut ObjectModule,
    builder: FunctionBuilder<'a>,
    ptr: types::Type,
    rt: &'a Runtime,
    fn_ids: &'a HashMap<String, (FuncId, usize)>,
    specialized_fn_ids: &'a HashMap<(String, usize), FuncId>,
    str_data: &'a HashMap<String, (DataId, usize)>,
    vars: HashMap<u32, Variable>,
    /// Static Value kind for each in-scope local slot.
    kinds: HashMap<u32, VKind>,
    /// Locals stored as untagged fixnum payloads in the ADR-0015 candidate.
    raw_fixnum_slots: HashSet<u32>,
    recur_targets: Vec<RecurTarget>,
    /// Raw shadow-stack frame base established at function entry.
    frame_base: Option<Variable>,
    /// Compact source-local to shadow-stack slot mapping.
    root_slots: HashMap<u32, u32>,
    /// Current closure `self`, used to read capture slots.
    self_var: Option<Variable>,
    /// Program-wide constant-vector site counter.
    next_const: &'a std::cell::Cell<u32>,
    /// Whether the isolated ADR-0015 candidate bundle is enabled.
    enable_adr15: bool,
    /// Shared aggregate metrics for every generated entry.
    stats: &'a RefCell<OptimizationStats>,
}

impl<'a> FnGen<'a> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        module: &'a mut ObjectModule,
        func: &'a mut cranelift_codegen::ir::Function,
        fbctx: &'a mut FunctionBuilderContext,
        ptr: types::Type,
        rt: &'a Runtime,
        fn_ids: &'a HashMap<String, (FuncId, usize)>,
        specialized_fn_ids: &'a HashMap<(String, usize), FuncId>,
        str_data: &'a HashMap<String, (DataId, usize)>,
        next_const: &'a std::cell::Cell<u32>,
        enable_adr15: bool,
        stats: &'a RefCell<OptimizationStats>,
    ) -> Self {
        FnGen {
            module,
            builder: FunctionBuilder::new(func, fbctx),
            ptr,
            rt,
            fn_ids,
            specialized_fn_ids,
            str_data,
            vars: HashMap::new(),
            kinds: HashMap::new(),
            raw_fixnum_slots: HashSet::new(),
            recur_targets: Vec::new(),
            frame_base: None,
            root_slots: HashMap::new(),
            self_var: None,
            next_const,
            enable_adr15,
            stats,
        }
    }

    fn new_var(&mut self, slot: u32) -> Variable {
        let v = self.builder.declare_var(types::I64);
        self.vars.insert(slot, v);
        v
    }

    // -- Static Value-kind analysis (ADR-0006) ----------------------------
    fn slot_kind(&self, slot: u32, extra: &HashMap<u32, VKind>) -> VKind {
        extra
            .get(&slot)
            .or_else(|| self.kinds.get(&slot))
            .copied()
            .unwrap_or(VKind::Heap)
    }

    /// Classifies an expression, with `extra` overriding not-yet-bound slots.
    fn kind_of(&self, ast: &Ast, extra: &HashMap<u32, VKind>) -> VKind {
        use VKind::{Fixnum, Heap, Imm};
        match ast {
            Ast::Int(_) => Fixnum,
            Ast::Bool(_) | Ast::Nil => Imm,
            Ast::DefGlobal { .. } => Imm, // inicializador; produz nil
            Ast::Float(_) // boxeado no heap
            | Ast::GlobalRef(_) // valor arbitrário lido de um global
            | Ast::Str(_)
            | Ast::Keyword(_)
            | Ast::VecLit(_)
            | Ast::SetLit(_)
            | Ast::MapLit(_)
            | Ast::MakeFn { .. }
            | Ast::FnRef(_)
            | Ast::MakeRecord { .. }
            | Ast::Capture(_) => Heap,
            Ast::Local(s) => self.slot_kind(*s, extra),
            Ast::Do(stmts) => stmts.last().map(|s| self.kind_of(s, extra)).unwrap_or(Imm),
            Ast::Let { slots, body } => {
                let mut e = extra.clone();
                for (slot, init) in slots {
                    let k = self.kind_of(init, &e);
                    e.insert(*slot, k);
                }
                self.kind_of(body, &e)
            }
            Ast::If(_, t, els) => self.kind_of(t, extra).join(self.kind_of(els, extra)),
            Ast::Loop { slots, body } => {
                let e = self.loop_slot_kinds(slots, body, extra);
                self.kind_of(body, &e)
            }
            Ast::Recur(_) => Imm, // Diverges and never materializes a Value.
            Ast::Call { callee, .. } => match callee {
                Callee::ProvenFixnumPrim(
                    Prim::Add
                    | Prim::Sub
                    | Prim::Mul
                    | Prim::Quot
                    | Prim::Mod
                    | Prim::Inc
                    | Prim::Dec,
                ) => Fixnum,
                Callee::ProvenFixnumPrim(_) => Imm,
                Callee::Prim(p) if prim_fixnum_result(*p) => Fixnum,
                Callee::Prim(p) if prim_imm_result(*p) => Imm,
                _ => Heap,
            },
            Ast::CallValue { .. } | Ast::Apply { .. } => Heap,
            Ast::RegisterMethod { .. } | Ast::RegisterMulti { .. } => Imm,
        }
    }

    /// Computes loop-slot kinds to a monotone `Imm -> Heap` fixed point.
    ///
    /// A slot remains immediate only when its initializer and every matching
    /// recur argument are immediate.
    fn loop_slot_kinds(
        &self,
        slots: &[(u32, Ast)],
        body: &Ast,
        extra: &HashMap<u32, VKind>,
    ) -> HashMap<u32, VKind> {
        let init_kinds: Vec<VKind> = slots.iter().map(|(_, i)| self.kind_of(i, extra)).collect();
        let slot_ids: Vec<u32> = slots.iter().map(|(s, _)| *s).collect();
        let mut e = extra.clone();
        for (i, slot) in slot_ids.iter().enumerate() {
            e.insert(*slot, init_kinds[i]);
        }
        loop {
            let mut rec = vec![VKind::Fixnum; slot_ids.len()];
            let mut seen = false;
            self.collect_recur_kinds(body, &e, &mut rec, &mut seen);
            let mut changed = false;
            for (i, slot) in slot_ids.iter().enumerate() {
                let cur = *e.get(slot).unwrap();
                let nk = if seen {
                    init_kinds[i].join(rec[i])
                } else {
                    init_kinds[i]
                };
                if nk != cur {
                    e.insert(*slot, nk);
                    changed = true;
                }
            }
            if !changed {
                break e;
            }
        }
    }

    /// Joins argument kinds from every `recur` targeting the current loop.
    ///
    /// Nested loops own their recurs. Because `recur` occurs only in tail
    /// position, traversal needs only conditional branches, let bodies, and do
    /// statements.
    fn collect_recur_kinds(
        &self,
        ast: &Ast,
        env: &HashMap<u32, VKind>,
        rec: &mut [VKind],
        seen: &mut bool,
    ) {
        match ast {
            Ast::Recur(args) => {
                *seen = true;
                for (i, a) in args.iter().enumerate() {
                    if i < rec.len() {
                        rec[i] = rec[i].join(self.kind_of(a, env));
                    }
                }
            }
            Ast::Loop { .. } => {} // Nested recurs belong to the nested loop.
            Ast::If(_, t, e) => {
                self.collect_recur_kinds(t, env, rec, seen);
                self.collect_recur_kinds(e, env, rec, seen);
            }
            Ast::Let { body, .. } => self.collect_recur_kinds(body, env, rec, seen),
            Ast::Do(stmts) => {
                for s in stmts {
                    self.collect_recur_kinds(s, env, rec, seen);
                }
            }
            _ => {}
        }
    }

    /// Plans compact root slots from conservative method representation facts.
    fn plan_method_roots(&mut self, methods: &[FnMethod]) {
        let mut rooted = BTreeSet::new();
        for method in methods {
            let mut environment = HashMap::new();
            for slot in 0..method.params.len() {
                // Parameters remain generic tagged roots until ADR-0015's
                // specialized entry owns the complete call boundary.
                let kind = VKind::Heap;
                environment.insert(slot as u32, kind);
                if kind == VKind::Heap {
                    rooted.insert(slot as u32);
                }
            }
            if method.rest.is_some() {
                let slot = method.params.len() as u32;
                environment.insert(slot, VKind::Heap);
                rooted.insert(slot);
            }
            self.collect_bound_roots(&method.body, &environment, &mut rooted);
        }
        self.root_slots = rooted
            .into_iter()
            .enumerate()
            .map(|(root_slot, source_slot)| (source_slot, root_slot as u32))
            .collect();
    }

    fn collect_bound_roots(
        &self,
        ast: &Ast,
        environment: &HashMap<u32, VKind>,
        rooted: &mut BTreeSet<u32>,
    ) {
        match ast {
            Ast::Let { slots, body } => {
                let mut local = environment.clone();
                for (slot, initializer) in slots {
                    self.collect_bound_roots(initializer, &local, rooted);
                    let kind = self.kind_of(initializer, &local);
                    local.insert(*slot, kind);
                    if kind == VKind::Heap {
                        rooted.insert(*slot);
                    }
                }
                self.collect_bound_roots(body, &local, rooted);
            }
            Ast::Loop { slots, body } => {
                let loop_environment = self.loop_slot_kinds(slots, body, environment);
                for (slot, initializer) in slots {
                    self.collect_bound_roots(initializer, environment, rooted);
                    if loop_environment.get(slot).copied() == Some(VKind::Heap) {
                        rooted.insert(*slot);
                    }
                }
                self.collect_bound_roots(body, &loop_environment, rooted);
            }
            Ast::If(test, then, otherwise) => {
                for expression in [test.as_ref(), then.as_ref(), otherwise.as_ref()] {
                    self.collect_bound_roots(expression, environment, rooted);
                }
            }
            Ast::VecLit(items) | Ast::SetLit(items) | Ast::Do(items) | Ast::Recur(items) => {
                for item in items {
                    self.collect_bound_roots(item, environment, rooted);
                }
            }
            Ast::MapLit(pairs) => {
                for (key, value) in pairs {
                    self.collect_bound_roots(key, environment, rooted);
                    self.collect_bound_roots(value, environment, rooted);
                }
            }
            Ast::DefGlobal { value, .. } => {
                self.collect_bound_roots(value, environment, rooted);
            }
            Ast::MakeFn { captures, .. } => {
                for capture in captures {
                    self.collect_bound_roots(capture, environment, rooted);
                }
            }
            Ast::Call { args, .. } => {
                for argument in args {
                    self.collect_bound_roots(argument, environment, rooted);
                }
            }
            Ast::CallValue { f, args } => {
                self.collect_bound_roots(f, environment, rooted);
                for argument in args {
                    self.collect_bound_roots(argument, environment, rooted);
                }
            }
            Ast::Apply { f, fixed, coll } => {
                self.collect_bound_roots(f, environment, rooted);
                for argument in fixed {
                    self.collect_bound_roots(argument, environment, rooted);
                }
                self.collect_bound_roots(coll, environment, rooted);
            }
            Ast::MakeRecord { fields, .. } => {
                for (_, value) in fields {
                    self.collect_bound_roots(value, environment, rooted);
                }
            }
            Ast::RegisterMethod { key, impl_fn, .. } => {
                self.collect_bound_roots(key, environment, rooted);
                self.collect_bound_roots(impl_fn, environment, rooted);
            }
            Ast::RegisterMulti { dispatch_fn, .. } => {
                self.collect_bound_roots(dispatch_fn, environment, rooted);
            }
            Ast::Nil
            | Ast::Bool(_)
            | Ast::Int(_)
            | Ast::Float(_)
            | Ast::Str(_)
            | Ast::Keyword(_)
            | Ast::GlobalRef(_)
            | Ast::Local(_)
            | Ast::Capture(_)
            | Ast::FnRef(_) => {}
        }
    }

    /// Returns whether evaluating `ast` leaves a temporary shadow-stack root.
    ///
    /// Heap expressions normally push. A local does not because either it is
    /// immediate or its frame slot already roots it. `Do`, `Let`, and `Loop`
    /// delegate the decision to the expression that produces their result.
    fn expr_pushes(&self, ast: &Ast, extra: &HashMap<u32, VKind>) -> bool {
        match ast {
            Ast::Local(_)
            | Ast::Int(_)
            | Ast::Bool(_)
            | Ast::Nil
            | Ast::Recur(_)
            | Ast::RegisterMethod { .. }
            | Ast::RegisterMulti { .. } => false,
            Ast::Do(stmts) => stmts.last().is_some_and(|s| self.expr_pushes(s, extra)),
            Ast::Let { slots, body } => {
                let mut e = extra.clone();
                for (slot, init) in slots {
                    e.insert(*slot, self.kind_of(init, &e));
                }
                self.expr_pushes(body, &e)
            }
            Ast::Loop { slots, body } => {
                let e = self.loop_slot_kinds(slots, body, extra);
                self.expr_pushes(body, &e)
            }
            _ => self.kind_of(ast, extra) == VKind::Heap,
        }
    }

    /// Evaluates an operand and reports whether it left a shadow-stack root.
    fn operand(&mut self, ast: &Ast) -> Result<(CValue, bool), Diagnostic> {
        let pushed = self.expr_pushes(ast, &HashMap::new());
        let v = self.expr_val(ast)?;
        Ok((v, pushed))
    }

    /// Evaluates a proven fixnum as an untagged payload.
    ///
    /// Raw locals and nested proven arithmetic remain unboxed. Other proven
    /// expressions cross the tagged boundary once and retain the ordinary
    /// temporary-root accounting.
    fn raw_fixnum_operand(&mut self, ast: &Ast) -> Result<(CValue, usize), Diagnostic> {
        match ast {
            Ast::Int(value) => Ok((self.builder.ins().iconst(types::I64, *value), 0)),
            Ast::Local(slot) if self.raw_fixnum_slots.contains(slot) => {
                let variable = *self.vars.get(slot).ok_or_else(|| {
                    Diagnostic::error("E0111", format!("local {slot} não vinculado (bug)"))
                })?;
                Ok((self.builder.use_var(variable), 0))
            }
            Ast::Call {
                callee: Callee::ProvenFixnumPrim(primitive),
                args,
            } if matches!(
                primitive,
                Prim::Add | Prim::Sub | Prim::Mul | Prim::Quot | Prim::Mod | Prim::Inc | Prim::Dec
            ) =>
            {
                self.gen_raw_fixnum_prim(*primitive, args)
            }
            _ => {
                let (tagged, pushed) = self.operand(ast)?;
                Ok((self.builder.ins().sshr_imm_s(tagged, 1), pushed as usize))
            }
        }
    }

    /// Evaluates a call argument and guarantees a contiguous shadow-stack slot.
    ///
    /// Even immediate arguments are spilled because `argv` points directly into
    /// the root stack.
    fn spill_arg(&mut self, ast: &Ast) -> Result<CValue, Diagnostic> {
        let (v, pushed) = self.operand(ast)?;
        if !pushed {
            self.gc_push_val(v);
        }
        Ok(v)
    }

    // -- Direct shadow-stack root stores (ADR-0006 phase 3) ---------------
    fn addr_gc_sp(&mut self) -> CValue {
        let gv = self
            .module
            .declare_data_in_func(self.rt.gc_sp_data, self.builder.func);
        self.builder.ins().symbol_value(self.ptr, gv)
    }
    fn addr_gc_stack(&mut self) -> CValue {
        let gv = self
            .module
            .declare_data_in_func(self.rt.gc_stack_data, self.builder.func);
        self.builder.ins().symbol_value(self.ptr, gv)
    }
    /// Computes the address of `gc_stack[idx]`.
    fn slot_addr(&mut self, idx: CValue) -> CValue {
        let stack = self.addr_gc_stack();
        let off = self.builder.ins().imul_imm_s(idx, 8);
        self.builder.ins().iadd(stack, off)
    }
    /// Pushes one root with the direct store `gc_stack[gc_sp++] = v`.
    fn gc_push_val(&mut self, v: CValue) {
        let sp_addr = self.addr_gc_sp();
        let sp = self
            .builder
            .ins()
            .load(types::I64, M::trusted(), sp_addr, 0);
        let elem = self.slot_addr(sp);
        self.builder.ins().store(M::trusted(), v, elem, 0);
        let sp1 = self.builder.ins().iadd_imm_s(sp, 1);
        self.builder.ins().store(M::trusted(), sp1, sp_addr, 0);
    }
    fn gc_popn(&mut self, n: usize) {
        if n > 0 {
            let sp_addr = self.addr_gc_sp();
            let sp = self
                .builder
                .ins()
                .load(types::I64, M::trusted(), sp_addr, 0);
            let sp2 = self.builder.ins().iadd_imm_s(sp, -(n as i64));
            self.builder.ins().store(M::trusted(), sp2, sp_addr, 0);
        }
    }
    /// Pops a root count computed at runtime.
    fn gc_popn_val(&mut self, n: CValue) {
        let sp_addr = self.addr_gc_sp();
        let sp = self
            .builder
            .ins()
            .load(types::I64, M::trusted(), sp_addr, 0);
        let sp2 = self.builder.ins().isub(sp, n);
        self.builder.ins().store(M::trusted(), sp2, sp_addr, 0);
    }
    /// Writes a local root slot as `gc_stack[base + slot] = v`.
    fn gc_set_local(&mut self, slot: u32, v: CValue) {
        let base_var = self.frame_base.expect("frame_base definido");
        let base = self.builder.use_var(base_var);
        let root_slot = *self
            .root_slots
            .get(&slot)
            .expect("slot heap-capable precisa de root planejado");
        let idx = self.builder.ins().iadd_imm_s(base, root_slot as i64);
        let elem = self.slot_addr(idx);
        self.builder.ins().store(M::trusted(), v, elem, 0);
        self.stats.borrow_mut().root_stores += 1;
    }
    /// Binds a heap-capable local and mirrors it into its root slot.
    fn bind_local(&mut self, slot: u32, v: CValue) {
        self.bind_local_kind(slot, v, VKind::Heap);
    }

    /// Binds a local whose static value kind is known.
    ///
    /// An immediate updates only the Cranelift variable; its frame slot safely
    /// remains NIL.
    fn bind_local_kind(&mut self, slot: u32, v: CValue, kind: VKind) {
        let var = match self.vars.get(&slot) {
            Some(v) => *v,
            None => self.new_var(slot),
        };
        self.raw_fixnum_slots.remove(&slot);
        self.builder.def_var(var, v);
        self.kinds.insert(slot, kind);
        if kind == VKind::Heap {
            self.gc_set_local(slot, v);
        }
    }

    /// Binds one local as an untagged fixnum payload.
    fn bind_raw_fixnum(&mut self, slot: u32, raw: CValue) {
        let var = match self.vars.get(&slot) {
            Some(variable) => *variable,
            None => self.new_var(slot),
        };
        self.builder.def_var(var, raw);
        self.kinds.insert(slot, VKind::Fixnum);
        self.raw_fixnum_slots.insert(slot);
        self.stats.borrow_mut().raw_fixnum_bindings += 1;
    }

    /// GC: enters a frame unless zero fixed slots and a balanced result prove it redundant.
    fn enter_planned_frame(&mut self, result_rooted: bool) {
        if !gc_frame::needs_gc_frame(self.root_slots.len(), result_rooted) {
            return;
        }
        self.stats.borrow_mut().root_slots += self.root_slots.len() as u64;
        self.stats.borrow_mut().root_frame_entries += 1;
        let slots = self.root_slots.len() as i64;
        let k = self.builder.ins().iconst(types::I64, slots);
        let base = self.call1(self.rt.gc_enter, k);
        let base_var = self.builder.declare_var(types::I64);
        self.builder.def_var(base_var, base);
        self.frame_base = Some(base_var);
    }
    fn leave_frame(&mut self) {
        let Some(base_var) = self.frame_base else {
            return;
        };
        let base = self.builder.use_var(base_var);
        self.call_void(self.rt.gc_leave, &[base]);
    }
    fn build_entry(
        mut self,
        methods: &[FnMethod],
        _local_count: u32,
        dispatch: Dispatch,
    ) -> Result<(), Diagnostic> {
        self.stats.borrow_mut().generic_entries += 1;
        let entry = self.builder.create_block();
        self.builder.append_block_params_for_function_params(entry);
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);
        // ABI: function block parameters are [self, argc, argv].
        let block_vals: Vec<CValue> = self.builder.block_params(entry).to_vec();
        let self_v = self.builder.declare_var(types::I64);
        self.builder.def_var(self_v, block_vals[0]);
        self.self_var = Some(self_v);
        let argc_v = block_vals[1];
        let argv_v = block_vals[2];

        // Dispatch stubs select either protocol type or multimethod dispatch.
        match dispatch {
            Dispatch::Protocol(mid) => {
                self.gen_dispatch(mid, argc_v, argv_v);
                self.builder.finalize(self.module.target_config());
                return Ok(());
            }
            Dispatch::Multi(mid) => {
                let mid_v = self.builder.ins().iconst(types::I64, mid);
                let r = self.call3(self.rt.multi_call, mid_v, argc_v, argv_v);
                self.builder.ins().return_(&[r]);
                self.builder.finalize(self.module.target_config());
                return Ok(());
            }
            Dispatch::None => {}
        }

        if self.enable_adr15 {
            self.plan_method_roots(methods);
        } else {
            self.root_slots = (0.._local_count).map(|slot| (slot, slot)).collect();
        }
        let no_kinds = HashMap::new();
        let rooted = methods.iter().any(|m| self.expr_pushes(&m.body, &no_kinds));
        self.enter_planned_frame(rooted);

        // Arity dispatch is a chain of argc checks.
        for m in methods {
            let k = m.params.len();
            let cond = if m.rest.is_some() {
                self.builder
                    .ins()
                    .icmp_imm_s(IntCC::SignedGreaterThanOrEqual, argc_v, k as i64)
            } else {
                self.builder
                    .ins()
                    .icmp_imm_s(IntCC::Equal, argc_v, k as i64)
            };
            let matched = self.builder.create_block();
            let next = self.builder.create_block();
            self.builder.ins().brif(cond, matched, &[], next, &[]);

            self.builder.switch_to_block(matched);
            self.builder.seal_block(matched);
            self.gen_method_body(m, argc_v, argv_v)?;

            self.builder.switch_to_block(next);
            self.builder.seal_block(next);
        }
        // No declared arity matched.
        let bad = self.builder.ins().iconst(types::I64, -1);
        self.call_void(self.rt.check_arity, &[argc_v, bad]); // sempre falha (exit)
        let z = self.konst(NIL);
        self.builder.ins().return_(&[z]);

        self.builder.finalize(self.module.target_config());
        Ok(())
    }

    /// Builds the compiler-private raw-fixnum entry for one closed method.
    ///
    /// Parameters and the return value are untagged signed payloads. The
    /// existing body lowering still owns checked arithmetic and semantic slow
    /// paths, so values are tagged once at entry and untagged once at return.
    /// Direct recursive calls use the same specialized ABI and never construct
    /// a generic `argv` array.
    fn build_specialized_fixnum_entry(mut self, method: &FnMethod) -> Result<(), Diagnostic> {
        debug_assert!(method.optimization.specialized_fixnum_abi);
        debug_assert!(method.rest.is_none());
        self.stats.borrow_mut().specialized_entries += 1;

        let entry = self.builder.create_block();
        self.builder.append_block_params_for_function_params(entry);
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);

        let mut environment = HashMap::new();
        for slot in 0..method.params.len() {
            environment.insert(slot as u32, VKind::Fixnum);
        }
        let mut rooted = BTreeSet::new();
        self.collect_bound_roots(&method.body, &environment, &mut rooted);
        self.root_slots = rooted
            .into_iter()
            .enumerate()
            .map(|(root_slot, source_slot)| (source_slot, root_slot as u32))
            .collect();
        let rooted = self.expr_pushes(&method.body, &environment);
        self.enter_planned_frame(rooted);

        let parameters = self.builder.block_params(entry).to_vec();
        let mut parameter_slots = Vec::with_capacity(parameters.len());
        for (slot, raw) in parameters.into_iter().enumerate() {
            self.bind_raw_fixnum(slot as u32, raw);
            parameter_slots.push(slot as u32);
        }

        let header = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        self.recur_targets.push(RecurTarget {
            header,
            slots: parameter_slots,
        });

        let flow = self.expr(&method.body);
        self.recur_targets.pop();
        self.builder.seal_block(header);
        match flow? {
            Flow::Val(tagged) => {
                self.leave_frame();
                let raw = self.builder.ins().sshr_imm_s(tagged, 1);
                self.builder.ins().return_(&[raw]);
            }
            Flow::Diverged => {}
        }
        self.builder.finalize(self.module.target_config());
        Ok(())
    }

    /// Resolves a protocol implementation and forwards the original arguments.
    ///
    /// The lookup is `lookup(mid, type_key(argv[0]))`.
    fn gen_dispatch(&mut self, mid: i64, argc_v: CValue, argv_v: CValue) {
        let arg0 = self.builder.ins().load(types::I64, M::trusted(), argv_v, 0);
        let key = self.call1(self.rt.type_key, arg0);
        let mid_v = self.builder.ins().iconst(types::I64, mid);
        let impl_v = self.call2(self.rt.lookup_method, mid_v, key);
        let is_nil = self.builder.ins().icmp_imm_s(IntCC::Equal, impl_v, NIL);
        let err_b = self.builder.create_block();
        let ok_b = self.builder.create_block();
        self.builder.ins().brif(is_nil, err_b, &[], ok_b, &[]);

        self.builder.switch_to_block(err_b);
        self.builder.seal_block(err_b);
        self.call_void(self.rt.no_method, &[mid_v]);
        let z = self.konst(NIL);
        self.builder.ins().return_(&[z]);

        self.builder.switch_to_block(ok_b);
        self.builder.seal_block(ok_b);
        let code = self.call1(self.rt.fn_code, impl_v);
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(types::I64));
        sig.params.push(AbiParam::new(types::I64));
        sig.params.push(AbiParam::new(self.ptr));
        sig.returns.push(AbiParam::new(types::I64));
        let sig_ref = self.builder.import_signature(sig);
        let call = self
            .builder
            .ins()
            .call_indirect(sig_ref, code, &[impl_v, argc_v, argv_v]);
        let r = self.builder.inst_results(call)[0];
        self.builder.ins().return_(&[r]);
    }

    /// Binds fixed/rest parameters and compiles one matched arity body.
    ///
    /// The body ends in either a returned value or diverged `recur` control flow.
    fn gen_method_body(
        &mut self,
        m: &FnMethod,
        argc_v: CValue,
        argv_v: CValue,
    ) -> Result<(), Diagnostic> {
        let mut param_slots = Vec::new();
        for slot in 0..m.params.len() {
            let val = self
                .builder
                .ins()
                .load(types::I64, M::trusted(), argv_v, (slot * 8) as i32);
            self.bind_local_kind(slot as u32, val, VKind::Heap);
            param_slots.push(slot as u32);
        }
        if m.rest.is_some() {
            let rest_slot = m.params.len() as u32;
            let nfixed = self.builder.ins().iconst(types::I64, m.params.len() as i64);
            let rest_list = self.call3(self.rt.collect_rest, argc_v, argv_v, nfixed);
            self.bind_local(rest_slot, rest_list);
            param_slots.push(rest_slot);
        }

        let header = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        self.recur_targets.push(RecurTarget {
            header,
            slots: param_slots,
        });

        let flow = self.expr(&m.body);
        self.recur_targets.pop();
        self.builder.seal_block(header);
        match flow? {
            Flow::Val(v) => {
                self.leave_frame();
                self.builder.ins().return_(&[v]);
            }
            Flow::Diverged => {}
        }
        Ok(())
    }

    fn build_main(mut self, body: &[Ast], local_count: u32) -> Result<(), Diagnostic> {
        let entry = self.builder.create_block();
        self.builder.append_block_params_for_function_params(entry);
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);
        // Capture argv in *command-line-args* before evaluating user code.
        let argc = self.builder.block_params(entry)[0];
        let argv = self.builder.block_params(entry)[1];
        let argc64 = self.builder.ins().uextend(types::I64, argc);
        self.call_void(self.rt.set_args, &[argc64, argv]);
        self.root_slots = (0..local_count).map(|slot| (slot, slot)).collect();
        self.enter_planned_frame(true);
        for a in body {
            let (_, pushed) = self.operand(a)?;
            if pushed {
                self.gc_popn(1); // Discard the top-level heap result.
            }
        }
        self.leave_frame();
        let zero = self.builder.ins().iconst(types::I32, 0);
        self.builder.ins().return_(&[zero]);
        self.builder.finalize(self.module.target_config());
        Ok(())
    }

    fn konst(&mut self, tagged: i64) -> CValue {
        self.builder.ins().iconst(types::I64, tagged)
    }

    fn call1(&mut self, id: FuncId, a: CValue) -> CValue {
        self.stats.borrow_mut().runtime_abi_calls += 1;
        let r = self.module.declare_func_in_func(id, self.builder.func);
        let c = self.builder.ins().call(r, &[a]);
        self.builder.inst_results(c)[0]
    }
    fn call2(&mut self, id: FuncId, a: CValue, b: CValue) -> CValue {
        self.stats.borrow_mut().runtime_abi_calls += 1;
        let r = self.module.declare_func_in_func(id, self.builder.func);
        let c = self.builder.ins().call(r, &[a, b]);
        self.builder.inst_results(c)[0]
    }
    fn call0(&mut self, id: FuncId) -> CValue {
        self.stats.borrow_mut().runtime_abi_calls += 1;
        let r = self.module.declare_func_in_func(id, self.builder.func);
        let c = self.builder.ins().call(r, &[]);
        self.builder.inst_results(c)[0]
    }
    fn call3(&mut self, id: FuncId, a: CValue, b: CValue, c: CValue) -> CValue {
        self.stats.borrow_mut().runtime_abi_calls += 1;
        let r = self.module.declare_func_in_func(id, self.builder.func);
        let call = self.builder.ins().call(r, &[a, b, c]);
        self.builder.inst_results(call)[0]
    }
    fn call_void3(&mut self, id: FuncId, a: CValue, b: CValue, c: CValue) {
        self.stats.borrow_mut().runtime_abi_calls += 1;
        let r = self.module.declare_func_in_func(id, self.builder.func);
        self.builder.ins().call(r, &[a, b, c]);
    }
    fn call_void4(&mut self, id: FuncId, a: CValue, b: CValue, c: CValue, d: CValue) {
        self.stats.borrow_mut().runtime_abi_calls += 1;
        let r = self.module.declare_func_in_func(id, self.builder.func);
        self.builder.ins().call(r, &[a, b, c, d]);
    }
    /// Materializes the code address of a declared function.
    fn func_addr(&mut self, id: FuncId) -> CValue {
        let r = self.module.declare_func_in_func(id, self.builder.func);
        self.builder.ins().func_addr(self.ptr, r)
    }
    fn call_void(&mut self, id: FuncId, args: &[CValue]) {
        self.stats.borrow_mut().runtime_abi_calls += 1;
        let r = self.module.declare_func_in_func(id, self.builder.func);
        self.builder.ins().call(r, args);
    }

    /// Evaluates an expression in a non-diverging operand position.
    ///
    /// The analyzer guarantees that `recur` occurs only in tail position.
    fn expr_val(&mut self, ast: &Ast) -> Result<CValue, Diagnostic> {
        match self.expr(ast)? {
            Flow::Val(v) => Ok(v),
            Flow::Diverged => Err(Diagnostic::error(
                "E0112",
                "recur em posição não-cauda (bug do compilador)",
            )),
        }
    }

    /// Lowers an expression while preserving the root-stack invariant.
    ///
    /// INVARIANT: on `Flow::Val(v)`, `v` is at the shadow-stack top exactly when
    /// `expr_pushes(ast)` is true. Heap values other than an already rooted local
    /// push; immediates and local reads do not. Consumers use `operand` or
    /// `spill_arg` to reconcile that state. `Flow::Diverged` has already
    /// terminated its block with the target's expected root depth.
    fn expr(&mut self, ast: &Ast) -> Result<Flow, Diagnostic> {
        Ok(match ast {
            // Immediates can never be heap pointers and therefore need no root.
            Ast::Int(n) => {
                let tagged = (*n as i128) << 1 | 1;
                Flow::Val(self.konst(tagged as i64))
            }
            // Boxed float: pass raw bits for the runtime to reinterpret.
            // It is heap-capable and must satisfy the `expr_pushes` root invariant.
            Ast::Float(x) => {
                let bits = self.builder.ins().iconst(types::I64, x.to_bits() as i64);
                let v = self.call1(self.rt.float_from_bits, bits);
                self.gc_push_val(v);
                Flow::Val(v)
            }
            // Read a top-level def global: heap-capable → satisfy the root invariant.
            Ast::GlobalRef(idx) => {
                let i = self.builder.ins().iconst(types::I64, *idx as i64);
                let v = self.call1(self.rt.global_get, i);
                self.gc_push_val(v);
                Flow::Val(v)
            }
            // Initialize a top-level def global once, in source order; yields nil.
            Ast::DefGlobal { index, value } => {
                let (v, pushed) = self.operand(value)?;
                let i = self.builder.ins().iconst(types::I64, *index as i64);
                self.call2(self.rt.global_set, i, v); // store; now a permanent root
                if pushed {
                    self.gc_popn(1);
                }
                Flow::Val(self.konst(NIL))
            }
            Ast::Bool(b) => Flow::Val(self.konst(if *b { TRUEV } else { FALSEV })),
            Ast::Nil => Flow::Val(self.konst(NIL)),
            Ast::Str(s) => {
                let (data_id, len) = self.str_data[s];
                let gv = self.module.declare_data_in_func(data_id, self.builder.func);
                let p = self.builder.ins().symbol_value(self.ptr, gv);
                let len_v = self.builder.ins().iconst(types::I64, len as i64);
                let v = self.call2(self.rt.str_from, p, len_v);
                self.gc_push_val(v);
                Flow::Val(v)
            }
            Ast::Local(slot) => {
                let var = *self.vars.get(slot).ok_or_else(|| {
                    Diagnostic::error("E0111", format!("local {slot} não vinculado (bug)"))
                })?;
                // A local needs no additional root: an immediate is not a pointer,
                // while a heap local remains in its stable frame slot. Consumers
                // that require contiguous stack storage call `spill_arg`.
                let value = self.builder.use_var(var);
                if self.raw_fixnum_slots.contains(slot) {
                    Flow::Val(self.fix_retag(value))
                } else {
                    Flow::Val(value)
                }
            }
            Ast::Do(stmts) => {
                if stmts.is_empty() {
                    return Ok(Flow::Val(self.konst(NIL)));
                }
                let last = stmts.len() - 1;
                for s in &stmts[..last] {
                    let (_, pushed) = self.operand(s)?; // Discard intermediate value.
                    if pushed {
                        self.gc_popn(1);
                    }
                }
                self.expr(&stmts[last])?
            }
            Ast::Let { slots, body } => {
                for (slot, init) in slots {
                    let kind = self.kind_of(init, &HashMap::new());
                    if self.enable_adr15 && kind == VKind::Fixnum {
                        let (raw, pushed) = self.raw_fixnum_operand(init)?;
                        self.bind_raw_fixnum(*slot, raw);
                        self.gc_popn(pushed);
                    } else {
                        let (val, pushed) = self.operand(init)?;
                        self.bind_local_kind(*slot, val, kind); // Root only heap values.
                        if pushed {
                            self.gc_popn(1); // The frame slot now owns the heap root.
                        }
                    }
                }
                self.expr(body)?
            }
            Ast::If(test, then, els) => self.gen_if(test, then, els)?,
            Ast::Loop { slots, body } => self.gen_loop(slots, body)?,
            Ast::Recur(args) => self.gen_recur(args)?,
            Ast::Call { callee, args } => {
                let v = self.gen_call(callee, args)?; // Net-zero; result is not pushed.
                                                      // Immediate results need no root.
                if self.kind_of(ast, &HashMap::new()) == VKind::Heap {
                    self.gc_push_val(v);
                }
                Flow::Val(v)
            }
            Ast::CallValue { f, args } => {
                let v = self.gen_call_value(f, args)?; // net-0
                self.gc_push_val(v);
                Flow::Val(v)
            }
            Ast::Apply { f, fixed, coll } => {
                let v = self.gen_apply(f, fixed, coll)?; // net-0
                self.gc_push_val(v);
                Flow::Val(v)
            }
            Ast::MakeRecord { type_name, fields } => {
                let v = self.gen_make_record(type_name, fields)?; // net-0
                self.gc_push_val(v);
                Flow::Val(v)
            }
            Ast::RegisterMethod {
                method_id,
                key,
                impl_fn,
            } => {
                let key_v = self.spill_arg(key)?; // +1 (rooteado durante register)
                let impl_v = self.spill_arg(impl_fn)?; // +1 (MakeFn rooteado)
                let mid_v = self.builder.ins().iconst(types::I64, *method_id);
                self.call_void3(self.rt.register_method, mid_v, key_v, impl_v);
                self.gc_popn(2);
                Flow::Val(self.konst(NIL)) // nil imediato: não empurra
            }
            Ast::RegisterMulti {
                method_id,
                dispatch_fn,
            } => {
                let df = self.spill_arg(dispatch_fn)?; // +1
                let mid_v = self.builder.ins().iconst(types::I64, *method_id);
                self.call_void(self.rt.multi_register, &[mid_v, df]);
                self.gc_popn(1);
                Flow::Val(self.konst(NIL))
            }
            Ast::Capture(i) => {
                let self_v = self.builder.use_var(self.self_var.expect("self"));
                let idx = self.builder.ins().iconst(types::I64, *i as i64);
                let v = self.call2(self.rt.fn_free, self_v, idx);
                self.gc_push_val(v);
                Flow::Val(v)
            }
            Ast::FnRef(name) => {
                let (id, arity) = self.fn_ids[name];
                let code = self.func_addr(id);
                let ar = self.builder.ins().iconst(types::I64, arity as i64);
                let nfree = self.builder.ins().iconst(types::I64, 0);
                let fnv = self.call3(self.rt.make_fn, code, ar, nfree);
                self.gc_push_val(fnv);
                Flow::Val(fnv)
            }
            Ast::MakeFn {
                lambda,
                arity,
                captures,
            } => self.gen_make_fn(lambda, *arity, captures)?,
            Ast::Keyword(name) => {
                let (data_id, len) = self.str_data[name];
                let gv = self.module.declare_data_in_func(data_id, self.builder.func);
                let p = self.builder.ins().symbol_value(self.ptr, gv);
                let len_v = self.builder.ins().iconst(types::I64, len as i64);
                let v = self.call2(self.rt.kw, p, len_v);
                self.gc_push_val(v);
                Flow::Val(v)
            }
            Ast::VecLit(items) => {
                let v = self.gen_vec(items)?;
                self.gc_push_val(v);
                Flow::Val(v)
            }
            Ast::SetLit(items) => {
                let v = self.gen_set(items)?;
                self.gc_push_val(v);
                Flow::Val(v)
            }
            Ast::MapLit(pairs) => {
                let v = self.gen_map(pairs)?;
                self.gc_push_val(v);
                Flow::Val(v)
            }
        })
    }

    /// Builds a persistent vector through successive net-zero `conj` calls.
    ///
    /// GC: the accumulator remains rooted during trie allocation.
    fn gen_vec(&mut self, items: &[Ast]) -> Result<CValue, Diagnostic> {
        // ADR-0009: an all-immediate immutable literal is constructed once and
        // cached by site.
        let all_const = !items.is_empty()
            && items
                .iter()
                .all(|it| matches!(it, Ast::Int(_) | Ast::Bool(_) | Ast::Nil));
        if all_const {
            let id = self.next_const.get();
            if (id as usize) < CONST_CACHE_MAX {
                self.next_const.set(id + 1);
                return self.gen_const_vec(id, items);
            }
        }
        self.gen_vec_build(items)
    }
    /// Loads or initializes an immediate-only vector literal cache site.
    ///
    /// A newly constructed vector becomes a permanent GC root. The function
    /// does not push its result, matching `gen_vec`.
    fn gen_const_vec(&mut self, id: u32, items: &[Ast]) -> Result<CValue, Diagnostic> {
        let gv = self
            .module
            .declare_data_in_func(self.rt.const_cache_data, self.builder.func);
        let base = self.builder.ins().symbol_value(self.ptr, gv);
        let slot = self.builder.ins().iadd_imm_s(base, (id as i64) * 8);
        let cached = self.builder.ins().load(types::I64, M::trusted(), slot, 0);
        let build_b = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        let nz = self.builder.ins().icmp_imm_s(IntCC::NotEqual, cached, 0);
        self.builder
            .ins()
            .brif(nz, merge, &[cached.into()], build_b, &[]);
        self.builder.switch_to_block(build_b);
        self.builder.seal_block(build_b);
        let built = self.gen_vec_build(items)?;
        let id_v = self.builder.ins().iconst(types::I64, id as i64);
        self.call_void(self.rt.const_register, &[id_v, built]);
        self.builder.ins().jump(merge, &[built.into()]);
        self.builder.switch_to_block(merge);
        self.builder.seal_block(merge);
        Ok(self.builder.block_params(merge)[0])
    }
    fn gen_vec_build(&mut self, items: &[Ast]) -> Result<CValue, Diagnostic> {
        let mut vals = Vec::with_capacity(items.len());
        for it in items {
            vals.push(self.spill_arg(it)?); // n temps (imediatos derramados)
        }
        let mut acc = self.call0(self.rt.vec_empty);
        self.gc_push_val(acc); // acc temp
        for iv in &vals {
            acc = self.call2(self.rt.vec_conj, acc, *iv); // iv e acc rooteados
            self.gc_popn(1);
            self.gc_push_val(acc);
        }
        self.gc_popn(items.len() + 1); // itens + acc
        Ok(acc)
    }

    /// Builds a sorted set by conjoining each item into an empty tree.
    ///
    /// GC: the accumulator remains rooted during LLRB allocation.
    fn gen_sorted_set(&mut self, items: &[Ast]) -> Result<CValue, Diagnostic> {
        let mut vals = Vec::with_capacity(items.len());
        for it in items {
            vals.push(self.spill_arg(it)?); // n temps (imediatos derramados)
        }
        let mut acc = self.call0(self.rt.sorted_set_empty);
        self.gc_push_val(acc); // acc temp
        for iv in &vals {
            acc = self.call2(self.rt.p_conj, acc, *iv);
            self.gc_popn(1);
            self.gc_push_val(acc);
        }
        self.gc_popn(items.len() + 1); // itens + acc
        Ok(acc)
    }

    /// Builds a sorted map by associating each pair into a rooted empty tree.
    fn gen_sorted_map(&mut self, pairs: &[(Ast, Ast)]) -> Result<CValue, Diagnostic> {
        let mut kvs = Vec::with_capacity(pairs.len());
        for (k, v) in pairs {
            let kv = self.spill_arg(k)?;
            let vv = self.spill_arg(v)?;
            kvs.push((kv, vv)); // 2 temps por par
        }
        let mut acc = self.call0(self.rt.sorted_map_empty);
        self.gc_push_val(acc); // acc temp
        for (kv, vv) in &kvs {
            acc = self.call3(self.rt.sorted_assoc, acc, *kv, *vv);
            self.gc_popn(1);
            self.gc_push_val(acc);
        }
        self.gc_popn(pairs.len() * 2 + 1); // pares + acc
        Ok(acc)
    }

    fn gen_set(&mut self, items: &[Ast]) -> Result<CValue, Diagnostic> {
        let mut vals = Vec::with_capacity(items.len());
        for it in items {
            vals.push(self.spill_arg(it)?);
        }
        let n = self.builder.ins().iconst(types::I64, items.len() as i64);
        let s = self.call1(self.rt.set_alloc, n);
        self.gc_popn(items.len());
        for iv in &vals {
            self.call_void(self.rt.set_add, &[s, *iv]); // dedup na construção
        }
        Ok(s)
    }

    fn gen_map(&mut self, pairs: &[(Ast, Ast)]) -> Result<CValue, Diagnostic> {
        let mut kvs = Vec::with_capacity(pairs.len() * 2);
        for (k, v) in pairs {
            let kv = self.spill_arg(k)?;
            let vv = self.spill_arg(v)?;
            kvs.push((kv, vv)); // 2 temps por par
        }
        let n = self.builder.ins().iconst(types::I64, pairs.len() as i64);
        let m = self.call1(self.rt.map_alloc, n);
        self.gc_popn(pairs.len() * 2);
        for (i, (kv, vv)) in kvs.iter().enumerate() {
            let idx = self.builder.ins().iconst(types::I64, i as i64);
            self.call_void4(self.rt.map_set, m, idx, *kv, *vv);
        }
        Ok(m)
    }

    /// Emits the inline fast path for two-argument `(nth vector index)`.
    ///
    /// A `T_VEC` with an in-range fixnum index reads its tail or trie without
    /// dispatch. Every other case calls `cljn_nth` for nil, capability,
    /// sequential, and error semantics from ADR-0008. Neither path allocates.
    fn gen_nth_fast(&mut self, args: &[Ast]) -> Result<CValue, Diagnostic> {
        let (coll, coll_pushed) = self.operand(&args[0])?;
        let (idx, idx_pushed) = self.operand(&args[1])?;
        let type_b = self.builder.create_block();
        let vec_b = self.builder.create_block();
        let inb_b = self.builder.create_block();
        let tail_b = self.builder.create_block();
        let trie_b = self.builder.create_block();
        let loop_b = self.builder.create_block();
        self.builder.append_block_param(loop_b, types::I64); // node
        self.builder.append_block_param(loop_b, types::I64); // level
        let descend_b = self.builder.create_block();
        let leaf_b = self.builder.create_block();
        let slow_b = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64); // resultado

        // Guard that coll is an aligned non-null pointer and index is a fixnum.
        let low3 = self.builder.ins().band_imm_s(coll, 7);
        let aligned = self.builder.ins().icmp_imm_s(IntCC::Equal, low3, 0);
        let nonzero = self.builder.ins().icmp_imm_s(IntCC::NotEqual, coll, 0);
        let ptr_ok = self.builder.ins().band(aligned, nonzero);
        let idxlow = self.builder.ins().band_imm_s(idx, 1);
        let idx_fix = self.builder.ins().icmp_imm_s(IntCC::NotEqual, idxlow, 0);
        let pre = self.builder.ins().band(ptr_ok, idx_fix);
        self.builder.ins().brif(pre, type_b, &[], slow_b, &[]);

        // Reading the type byte is safe only after the pointer guard.
        self.builder.switch_to_block(type_b);
        self.builder.seal_block(type_b);
        let ty = self.builder.ins().load(types::I8, M::trusted(), coll, 0);
        let is_vec = self.builder.ins().icmp_imm_s(IntCC::Equal, ty, T_VEC);
        self.builder.ins().brif(is_vec, vec_b, &[], slow_b, &[]);

        // Unbox and bounds-check; the slow arity-2 path throws out of bounds.
        self.builder.switch_to_block(vec_b);
        self.builder.seal_block(vec_b);
        let i = self.builder.ins().sshr_imm_s(idx, 1);
        let count = self
            .builder
            .ins()
            .load(types::I64, M::trusted(), coll, PV_COUNT);
        let neg = self.builder.ins().icmp_imm_s(IntCC::SignedLessThan, i, 0);
        let ge = self
            .builder
            .ins()
            .icmp(IntCC::SignedGreaterThanOrEqual, i, count);
        let oob = self.builder.ins().bor(neg, ge);
        self.builder.ins().brif(oob, slow_b, &[], inb_b, &[]);

        // In-bounds path selects tail or trie storage.
        self.builder.switch_to_block(inb_b);
        self.builder.seal_block(inb_b);
        let tail_len = self
            .builder
            .ins()
            .load(types::I64, M::trusted(), coll, PV_TAILLEN);
        let tailoff = self.builder.ins().isub(count, tail_len);
        let in_tail = self
            .builder
            .ins()
            .icmp(IntCC::SignedGreaterThanOrEqual, i, tailoff);
        self.builder.ins().brif(in_tail, tail_b, &[], trie_b, &[]);

        // Tail fast path: tail->slots[i - tailoff].
        self.builder.switch_to_block(tail_b);
        self.builder.seal_block(tail_b);
        let tail = self
            .builder
            .ins()
            .load(types::I64, M::trusted(), coll, PV_TAIL);
        let ti = self.builder.ins().isub(i, tailoff);
        let toff = self.builder.ins().imul_imm_s(ti, 8);
        let taddr = self.builder.ins().iadd(tail, toff);
        let tres = self
            .builder
            .ins()
            .load(types::I64, M::trusted(), taddr, VNODE_SLOTS);
        self.builder.ins().jump(merge, &[tres.into()]);

        // Trie path begins at root and shift.
        self.builder.switch_to_block(trie_b);
        self.builder.seal_block(trie_b);
        let root = self
            .builder
            .ins()
            .load(types::I64, M::trusted(), coll, PV_ROOT);
        let shift = self
            .builder
            .ins()
            .load(types::I64, M::trusted(), coll, PV_SHIFT);
        self.builder
            .ins()
            .jump(loop_b, &[root.into(), shift.into()]);

        // Walk internal nodes until the leaf level.
        self.builder.switch_to_block(loop_b);
        let node = self.builder.block_params(loop_b)[0];
        let level = self.builder.block_params(loop_b)[1];
        let more = self
            .builder
            .ins()
            .icmp_imm_s(IntCC::SignedGreaterThan, level, 0);
        self.builder.ins().brif(more, descend_b, &[], leaf_b, &[]);

        // Select five index bits per trie level.
        self.builder.switch_to_block(descend_b);
        self.builder.seal_block(descend_b);
        let sh = self.builder.ins().sshr(i, level);
        let sub = self.builder.ins().band_imm_s(sh, 31);
        let soff = self.builder.ins().imul_imm_s(sub, 8);
        let saddr = self.builder.ins().iadd(node, soff);
        let child = self
            .builder
            .ins()
            .load(types::I64, M::trusted(), saddr, VNODE_SLOTS);
        let level2 = self.builder.ins().iadd_imm_s(level, -5);
        self.builder
            .ins()
            .jump(loop_b, &[child.into(), level2.into()]);
        self.builder.seal_block(loop_b); // preds: trie_b, descend_b

        // Read the leaf slot.
        self.builder.switch_to_block(leaf_b);
        self.builder.seal_block(leaf_b);
        let slot = self.builder.ins().band_imm_s(i, 31);
        let loff = self.builder.ins().imul_imm_s(slot, 8);
        let laddr = self.builder.ins().iadd(node, loff);
        let lres = self
            .builder
            .ins()
            .load(types::I64, M::trusted(), laddr, VNODE_SLOTS);
        self.builder.ins().jump(merge, &[lres.into()]);

        // Slow path owns complete type/bounds semantics and diagnostics.
        self.builder.switch_to_block(slow_b);
        self.builder.seal_block(slow_b);
        let sres = self.call2(self.rt.p_nth, coll, idx);
        self.builder.ins().jump(merge, &[sres.into()]);

        // Merge the fast and slow Values.
        self.builder.switch_to_block(merge);
        self.builder.seal_block(merge);
        let res = self.builder.block_params(merge)[0];
        self.gc_popn(coll_pushed as usize + idx_pushed as usize);
        Ok(res)
    }

    /// Lowers variadic assoc by evaluating all operands before a left fold.
    ///
    /// GC: heap operands and the current accumulator stay rooted across every
    /// `cljn_assoc` allocation.
    fn gen_assoc(&mut self, args: &[Ast]) -> Result<CValue, Diagnostic> {
        // Unconsumed heap operands retain roots; immediates never need them.
        let mut vals = Vec::with_capacity(args.len());
        let mut pushed = 0usize;
        for a in args {
            let (v, p) = self.operand(a)?;
            pushed += p as usize;
            vals.push(v);
        }
        let mut acc = vals[0];
        self.gc_push_val(acc);
        let mut i = 1;
        while i + 1 < vals.len() {
            acc = self.call3(self.rt.p_assoc, acc, vals[i], vals[i + 1]);
            self.gc_popn(1);
            self.gc_push_val(acc);
            i += 2;
        }
        self.gc_popn(1);
        self.gc_popn(pushed);
        Ok(acc)
    }

    /// Materializes a keyword from object data without pushing the result.
    fn make_kw_value(&mut self, name: &str) -> CValue {
        let (data_id, len) = self.str_data[name];
        let gv = self.module.declare_data_in_func(data_id, self.builder.func);
        let p = self.builder.ins().symbol_value(self.ptr, gv);
        let len_v = self.builder.ins().iconst(types::I64, len as i64);
        self.call2(self.rt.kw, p, len_v)
    }

    /// Builds a record with net-zero shadow-stack depth.
    fn gen_make_record(
        &mut self,
        type_name: &str,
        fields: &[(String, Ast)],
    ) -> Result<CValue, Diagnostic> {
        let pairs: Vec<(Ast, Ast)> = fields
            .iter()
            .map(|(fname, val)| (Ast::Keyword(fname.clone()), val.clone()))
            .collect();
        let map_val = self.gen_map(&pairs)?;
        self.gc_push_val(map_val);
        let tn = self.make_kw_value(type_name);
        self.gc_push_val(tn);
        let rec = self.call2(self.rt.make_record, tn, map_val);
        self.gc_popn(2);
        Ok(rec)
    }

    /// Evaluates captures, allocates a function object, and fills `freev`.
    fn gen_make_fn(
        &mut self,
        lambda: &str,
        arity: usize,
        captures: &[Ast],
    ) -> Result<Flow, Diagnostic> {
        let mut cap_vals = Vec::with_capacity(captures.len());
        for c in captures {
            cap_vals.push(self.spill_arg(c)?);
        }
        let (id, _) = self.fn_ids[lambda];
        let code = self.func_addr(id);
        let ar = self.builder.ins().iconst(types::I64, arity as i64);
        let nfree = self.builder.ins().iconst(types::I64, captures.len() as i64);
        let fnv = self.call3(self.rt.make_fn, code, ar, nfree);
        // set_free does not allocate; the function owns captures after filling.
        self.gc_popn(captures.len());
        self.gc_push_val(fnv);
        for (i, cv) in cap_vals.iter().enumerate() {
            let idx = self.builder.ins().iconst(types::I64, i as i64);
            self.call_void3(self.rt.set_free, fnv, idx, *cv);
        }
        Ok(Flow::Val(fnv))
    }

    /// Emits an indirect callable invocation with net-zero shadow-stack depth.
    fn gen_call_value(&mut self, f: &Ast, args: &[Ast]) -> Result<CValue, Diagnostic> {
        self.stats.borrow_mut().generic_argv_spills += args.len() as u64;
        let f_val = self.spill_arg(f)?;
        for a in args {
            self.spill_arg(a)?;
        }
        self.call_void(self.rt.check_fn, &[f_val]);
        let argc_v = self.builder.ins().iconst(types::I64, args.len() as i64);
        let argv_ptr = self.call1(self.rt.argv, argc_v);
        let code = self.call1(self.rt.fn_code, f_val);
        // ABI: every entry uses (self, argc, argv) -> Value.
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(types::I64)); // self
        sig.params.push(AbiParam::new(types::I64)); // argc
        sig.params.push(AbiParam::new(self.ptr)); // argv
        sig.returns.push(AbiParam::new(types::I64));
        let sig_ref = self.builder.import_signature(sig);
        let call = self
            .builder
            .ins()
            .call_indirect(sig_ref, code, &[f_val, argc_v, argv_ptr]);
        let r = self.builder.inst_results(call)[0];
        self.gc_popn(1 + args.len());
        Ok(r)
    }

    /// Lowers apply by rooting callable/fixed args and spreading the collection.
    fn gen_apply(&mut self, f: &Ast, fixed: &[Ast], coll: &Ast) -> Result<CValue, Diagnostic> {
        self.stats.borrow_mut().generic_argv_spills += fixed.len() as u64;
        let f_val = self.spill_arg(f)?; // shadow: [f]
        for a in fixed {
            self.spill_arg(a)?; // [f, fixed...]
        }
        let coll_val = self.spill_arg(coll)?; // [f, fixed.., coll]
        self.gc_popn(1); // remove o temp de coll (spread não aloca; coll SSA segue válido)
        let n_fixed = self.builder.ins().iconst(types::I64, fixed.len() as i64);
        // Spread collection elements on top and compute fixed plus extra argc.
        let total = self.call2(self.rt.spread_args, n_fixed, coll_val);
        self.call_void(self.rt.check_fn, &[f_val]);
        let argv_ptr = self.call1(self.rt.argv, total); // topo `total` = fixos + elementos
        let code = self.call1(self.rt.fn_code, f_val);
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(types::I64));
        sig.params.push(AbiParam::new(types::I64));
        sig.params.push(AbiParam::new(self.ptr));
        sig.returns.push(AbiParam::new(types::I64));
        let sig_ref = self.builder.import_signature(sig);
        let call = self
            .builder
            .ins()
            .call_indirect(sig_ref, code, &[f_val, total, argv_ptr]);
        let r = self.builder.inst_results(call)[0];
        // Pop callable, fixed arguments, and spread elements.
        let one = self.builder.ins().iconst(types::I64, 1);
        let cleanup = self.builder.ins().iadd(total, one);
        self.gc_popn_val(cleanup);
        Ok(r)
    }

    fn gen_if(&mut self, test: &Ast, then: &Ast, els: &Ast) -> Result<Flow, Diagnostic> {
        let empty = HashMap::new();
        let merge_kind = self.kind_of(then, &empty).join(self.kind_of(els, &empty));
        let then_pushes = self.expr_pushes(then, &empty);
        let els_pushes = self.expr_pushes(els, &empty);

        let (test_val, tpushed) = self.operand(test)?;
        let cond = if tpushed {
            let truth = self.call1(self.rt.truthy, test_val); // i32
            self.gc_popn(1); // consome o temp do teste
            self.builder.ins().icmp_imm_s(IntCC::NotEqual, truth, 0)
        } else {
            // Only FALSEV and NIL are falsey, so the test is an inline comparison.
            let nf = self
                .builder
                .ins()
                .icmp_imm_s(IntCC::NotEqual, test_val, FALSEV);
            let nn = self
                .builder
                .ins()
                .icmp_imm_s(IntCC::NotEqual, test_val, NIL);
            self.builder.ins().band(nf, nn)
        };

        let then_b = self.builder.create_block();
        let else_b = self.builder.create_block();
        self.builder.ins().brif(cond, then_b, &[], else_b, &[]);

        // Create the merge lazily only when a branch reaches fall-through.
        let mut merge: Option<Block> = None;

        self.builder.switch_to_block(then_b);
        self.builder.seal_block(then_b);
        if let Flow::Val(tv) = self.expr(then)? {
            // If the merged value is heap-capable but this branch did not push,
            // spill it so every predecessor reaches the same root depth.
            if merge_kind == VKind::Heap && !then_pushes {
                self.gc_push_val(tv);
            }
            let m = *merge.get_or_insert_with(|| {
                let b = self.builder.create_block();
                self.builder.append_block_param(b, types::I64);
                b
            });
            self.builder.ins().jump(m, &[tv.into()]);
        }

        self.builder.switch_to_block(else_b);
        self.builder.seal_block(else_b);
        if let Flow::Val(ev) = self.expr(els)? {
            if merge_kind == VKind::Heap && !els_pushes {
                self.gc_push_val(ev);
            }
            let m = *merge.get_or_insert_with(|| {
                let b = self.builder.create_block();
                self.builder.append_block_param(b, types::I64);
                b
            });
            self.builder.ins().jump(m, &[ev.into()]);
        }

        match merge {
            Some(m) => {
                self.builder.switch_to_block(m);
                self.builder.seal_block(m);
                Ok(Flow::Val(self.builder.block_params(m)[0]))
            }
            None => Ok(Flow::Diverged), // ambos os ramos divergiram
        }
    }

    fn gen_loop(&mut self, slots: &[(u32, Ast)], body: &Ast) -> Result<Flow, Diagnostic> {
        // A slot is immediate only when its initializer and every recur argument
        // at that position are immediate; immediate slots omit root stores.
        let slot_kinds = self.loop_slot_kinds(slots, body, &HashMap::new());
        let mut slot_ids = Vec::with_capacity(slots.len());
        for (slot, init) in slots {
            let k = *slot_kinds.get(slot).unwrap_or(&VKind::Heap);
            if self.enable_adr15 && k == VKind::Fixnum {
                let (raw, pushed) = self.raw_fixnum_operand(init)?;
                self.bind_raw_fixnum(*slot, raw);
                self.gc_popn(pushed);
            } else {
                let (v0, pushed) = self.operand(init)?;
                self.bind_local_kind(*slot, v0, k); // Root only heap-capable slots.
                if pushed {
                    self.gc_popn(1);
                }
            }
            slot_ids.push(*slot);
        }
        let header = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        self.recur_targets.push(RecurTarget {
            header,
            slots: slot_ids,
        });

        let flow = self.expr(body);
        self.recur_targets.pop();
        self.builder.seal_block(header);
        flow
    }

    fn gen_recur(&mut self, args: &[Ast]) -> Result<Flow, Diagnostic> {
        let target = self
            .recur_targets
            .last()
            .cloned()
            .ok_or_else(|| Diagnostic::error("E0113", "recur sem alvo (bug)"))?;
        // Evaluate every argument before rebinding to avoid clobbering locals.
        let mut vals = Vec::with_capacity(args.len());
        let mut pushed = 0usize;
        for (slot, argument) in target.slots.iter().zip(args) {
            if self.raw_fixnum_slots.contains(slot) {
                let (raw, roots) = self.raw_fixnum_operand(argument)?;
                pushed += roots;
                vals.push((raw, true));
            } else {
                let (tagged, was_pushed) = self.operand(argument)?;
                pushed += was_pushed as usize;
                vals.push((tagged, false));
            }
        }
        for (slot, (value, is_raw)) in target.slots.iter().zip(vals) {
            // Loop fixed point guarantees an immediate slot never receives heap.
            if is_raw {
                self.bind_raw_fixnum(*slot, value);
            } else {
                let kind = *self.kinds.get(slot).unwrap_or(&VKind::Heap);
                self.bind_local_kind(*slot, value, kind);
            }
        }
        self.gc_popn(pushed); // Only heap temporaries were pushed.
        self.builder.ins().jump(target.header, &[]);
        Ok(Flow::Diverged)
    }

    /// Calls a function or primitive with net-zero shadow-stack effect.
    ///
    /// Arguments are evaluated and temporarily rooted before the call. This
    /// function removes those roots and returns an unpushed result; `expr`
    /// decides whether the result itself needs a root.
    fn gen_call(&mut self, callee: &Callee, args: &[Ast]) -> Result<CValue, Diagnostic> {
        match callee {
            Callee::Prim(p) => self.gen_prim(*p, args),
            Callee::ProvenFixnumPrim(p) => self.gen_proven_fixnum_prim(*p, args),
            Callee::Fn(name) => {
                if let Some(specialized) = self
                    .specialized_fn_ids
                    .get(&(name.clone(), args.len()))
                    .copied()
                {
                    self.stats.borrow_mut().specialized_direct_calls += 1;
                    // The adapter proved every argument is a fixnum at every
                    // call site. Keep raw locals and arithmetic islands
                    // unboxed while evaluating in source order.
                    let mut raw_arguments = Vec::with_capacity(args.len());
                    let mut pushed = 0usize;
                    for argument in args {
                        let (raw, roots) = self.raw_fixnum_operand(argument)?;
                        pushed += roots;
                        raw_arguments.push(raw);
                    }
                    let function = self
                        .module
                        .declare_func_in_func(specialized, self.builder.func);
                    let call = self.builder.ins().call(function, &raw_arguments);
                    let raw_result = self.builder.inst_results(call)[0];
                    self.gc_popn(pushed);
                    return Ok(self.fix_retag(raw_result));
                }
                let (id, _) = self.fn_ids[name];
                {
                    let mut stats = self.stats.borrow_mut();
                    stats.generic_direct_calls += 1;
                    stats.generic_argv_spills += args.len() as u64;
                }
                // `argv` points into roots for every argument, including spills.
                for a in args {
                    self.spill_arg(a)?; // One contiguous slot per argument.
                }
                let argc_v = self.builder.ins().iconst(types::I64, args.len() as i64);
                let argv_ptr = self.call1(self.rt.argv, argc_v);
                let nil = self.konst(NIL); // Top-level functions have no captures.
                let fref = self.module.declare_func_in_func(id, self.builder.func);
                let call = self.builder.ins().call(fref, &[nil, argc_v, argv_ptr]);
                let r = self.builder.inst_results(call)[0];
                self.gc_popn(args.len());
                Ok(r)
            }
        }
    }

    // -- Fixnum fast paths (ADR-0006) ------------------------------------
    /// Tests whether both operands are fixnums with `(a & b & 1) != 0`.
    fn fix_both_guard(&mut self, a: CValue, b: CValue) -> CValue {
        let ab = self.builder.ins().band(a, b);
        let m = self.builder.ins().band_imm_s(ab, 1);
        self.builder.ins().icmp_imm_s(IntCC::NotEqual, m, 0)
    }
    fn fix_guard(&mut self, a: CValue) -> CValue {
        let m = self.builder.ins().band_imm_s(a, 1);
        self.builder.ins().icmp_imm_s(IntCC::NotEqual, m, 0)
    }
    fn fix_retag(&mut self, raw: CValue) -> CValue {
        let sh = self.builder.ins().ishl_imm_s(raw, 1);
        self.builder.ins().bor_imm_s(sh, 1)
    }
    /// Tests whether `raw` is in the representable fixnum range.
    fn fix_in_range(&mut self, raw: CValue) -> CValue {
        let lo = self
            .builder
            .ins()
            .icmp_imm_s(IntCC::SignedGreaterThanOrEqual, raw, FIX_MIN);
        let hi = self
            .builder
            .ins()
            .icmp_imm_s(IntCC::SignedLessThanOrEqual, raw, FIX_MAX);
        self.builder.ins().band(lo, hi)
    }

    /// Emits guarded inline addition/subtraction with runtime fallback.
    fn gen_fix_arith(
        &mut self,
        a: CValue,
        b: CValue,
        add: bool,
        slow: FuncId,
        guard_types: bool,
    ) -> CValue {
        let slow_b = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        if guard_types {
            let both = self.fix_both_guard(a, b);
            let fast_b = self.builder.create_block();
            self.builder.ins().brif(both, fast_b, &[], slow_b, &[]);
            self.builder.switch_to_block(fast_b);
            self.builder.seal_block(fast_b);
        }
        let ar = self.builder.ins().sshr_imm_s(a, 1);
        let br = self.builder.ins().sshr_imm_s(b, 1);
        let rr = if add {
            self.builder.ins().iadd(ar, br)
        } else {
            self.builder.ins().isub(ar, br)
        };
        let inr = self.fix_in_range(rr);
        let ok_b = self.builder.create_block();
        self.builder.ins().brif(inr, ok_b, &[], slow_b, &[]);
        self.builder.switch_to_block(ok_b);
        self.builder.seal_block(ok_b);
        let tagged = self.fix_retag(rr);
        self.builder.ins().jump(merge, &[tagged.into()]);

        self.builder.switch_to_block(slow_b);
        self.builder.seal_block(slow_b);
        let sr = self.call2(slow, a, b);
        self.builder.ins().jump(merge, &[sr.into()]);

        self.builder.switch_to_block(merge);
        self.builder.seal_block(merge);
        self.builder.block_params(merge)[0]
    }

    /// Emits guarded inline multiplication with runtime fallback.
    ///
    /// The fast path uses low/high signed products to detect i64 overflow,
    /// checks the narrower fixnum range, and retags the result.
    fn gen_fix_mul(&mut self, a: CValue, b: CValue, slow: FuncId, guard_types: bool) -> CValue {
        let slow_b = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        if guard_types {
            let both = self.fix_both_guard(a, b);
            let fast_b = self.builder.create_block();
            self.builder.ins().brif(both, fast_b, &[], slow_b, &[]);
            self.builder.switch_to_block(fast_b);
            self.builder.seal_block(fast_b);
        }
        let ar = self.builder.ins().sshr_imm_s(a, 1);
        let br = self.builder.ins().sshr_imm_s(b, 1);
        let lo = self.builder.ins().imul(ar, br);
        let hi = self.builder.ins().smulhi(ar, br);
        let exp = self.builder.ins().sshr_imm_s(lo, 63); // extensão de sinal de lo
        let no_ovf = self.builder.ins().icmp(IntCC::Equal, hi, exp);
        let inr = self.fix_in_range(lo);
        let ok = self.builder.ins().band(no_ovf, inr);
        let ok_b = self.builder.create_block();
        self.builder.ins().brif(ok, ok_b, &[], slow_b, &[]);
        self.builder.switch_to_block(ok_b);
        self.builder.seal_block(ok_b);
        let tagged = self.fix_retag(lo);
        self.builder.ins().jump(merge, &[tagged.into()]);

        self.builder.switch_to_block(slow_b);
        self.builder.seal_block(slow_b);
        let sr = self.call2(slow, a, b);
        self.builder.ins().jump(merge, &[sr.into()]);

        self.builder.switch_to_block(merge);
        self.builder.seal_block(merge);
        self.builder.block_params(merge)[0]
    }

    /// Emits guarded inline increment/decrement with runtime fallback.
    fn gen_fix_unop(&mut self, a: CValue, delta: i64, slow: FuncId, guard_type: bool) -> CValue {
        let slow_b = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        if guard_type {
            let guard = self.fix_guard(a);
            let fast_b = self.builder.create_block();
            self.builder.ins().brif(guard, fast_b, &[], slow_b, &[]);
            self.builder.switch_to_block(fast_b);
            self.builder.seal_block(fast_b);
        }
        let ar = self.builder.ins().sshr_imm_s(a, 1);
        let rr = self.builder.ins().iadd_imm_s(ar, delta);
        let inr = self.fix_in_range(rr);
        let ok_b = self.builder.create_block();
        self.builder.ins().brif(inr, ok_b, &[], slow_b, &[]);
        self.builder.switch_to_block(ok_b);
        self.builder.seal_block(ok_b);
        let tagged = self.fix_retag(rr);
        self.builder.ins().jump(merge, &[tagged.into()]);

        self.builder.switch_to_block(slow_b);
        self.builder.seal_block(slow_b);
        let sr = self.call1(slow, a);
        self.builder.ins().jump(merge, &[sr.into()]);

        self.builder.switch_to_block(merge);
        self.builder.seal_block(merge);
        self.builder.block_params(merge)[0]
    }

    /// Emits guarded inline quotient or floored modulus with runtime fallback.
    ///
    /// Zero divisors use the slow path for the user-facing error. Quotient also
    /// guards the minimum-fixnum divided by -1 overflow case. Modulus adjusts a
    /// non-zero C remainder when operand signs differ.
    fn gen_fix_div(
        &mut self,
        a: CValue,
        b: CValue,
        is_quot: bool,
        slow: FuncId,
        guard_types: bool,
    ) -> CValue {
        let slow_b = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        if guard_types {
            let both = self.fix_both_guard(a, b);
            let fast_b = self.builder.create_block();
            self.builder.ins().brif(both, fast_b, &[], slow_b, &[]);
            self.builder.switch_to_block(fast_b);
            self.builder.seal_block(fast_b);
        }
        let ar = self.builder.ins().sshr_imm_s(a, 1);
        let br = self.builder.ins().sshr_imm_s(b, 1);
        let bz = self.builder.ins().icmp_imm_s(IntCC::Equal, br, 0);
        let cont_b = self.builder.create_block();
        self.builder.ins().brif(bz, slow_b, &[], cont_b, &[]); // Zero uses slow path.
        self.builder.switch_to_block(cont_b);
        self.builder.seal_block(cont_b);
        if is_quot {
            let q = self.builder.ins().sdiv(ar, br);
            let inr = self.fix_in_range(q);
            let ok_b = self.builder.create_block();
            self.builder.ins().brif(inr, ok_b, &[], slow_b, &[]);
            self.builder.switch_to_block(ok_b);
            self.builder.seal_block(ok_b);
            let tagged = self.fix_retag(q);
            self.builder.ins().jump(merge, &[tagged.into()]);
        } else {
            let r = self.builder.ins().srem(ar, br);
            // Floored adjustment: non-zero remainder with differing signs adds b.
            let rnz = self.builder.ins().icmp_imm_s(IntCC::NotEqual, r, 0);
            let xr = self.builder.ins().bxor(r, br);
            let diff = self.builder.ins().icmp_imm_s(IntCC::SignedLessThan, xr, 0);
            let adj = self.builder.ins().band(rnz, diff);
            let radj = self.builder.ins().iadd(r, br);
            let res = self.builder.ins().select(adj, radj, r);
            let tagged = self.fix_retag(res);
            self.builder.ins().jump(merge, &[tagged.into()]);
        }

        self.builder.switch_to_block(slow_b);
        self.builder.seal_block(slow_b);
        let sr = self.call2(slow, a, b);
        self.builder.ins().jump(merge, &[sr.into()]);

        self.builder.switch_to_block(merge);
        self.builder.seal_block(merge);
        self.builder.block_params(merge)[0]
    }

    /// Emits guarded inline numeric comparison with runtime fallback.
    fn gen_fix_cmp(
        &mut self,
        a: CValue,
        b: CValue,
        cc: IntCC,
        slow: FuncId,
        guard_types: bool,
    ) -> CValue {
        if !guard_types {
            let ar = self.builder.ins().sshr_imm_s(a, 1);
            let br = self.builder.ins().sshr_imm_s(b, 1);
            let comparison = self.builder.ins().icmp(cc, ar, br);
            let truth = self.builder.ins().iconst(types::I64, TRUEV);
            let falsehood = self.builder.ins().iconst(types::I64, FALSEV);
            return self.builder.ins().select(comparison, truth, falsehood);
        }
        let both = self.fix_both_guard(a, b);
        let fast_b = self.builder.create_block();
        let slow_b = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        self.builder.ins().brif(both, fast_b, &[], slow_b, &[]);

        self.builder.switch_to_block(fast_b);
        self.builder.seal_block(fast_b);
        let ar = self.builder.ins().sshr_imm_s(a, 1);
        let br = self.builder.ins().sshr_imm_s(b, 1);
        let c = self.builder.ins().icmp(cc, ar, br);
        let t = self.builder.ins().iconst(types::I64, TRUEV);
        let f = self.builder.ins().iconst(types::I64, FALSEV);
        let r = self.builder.ins().select(c, t, f);
        self.builder.ins().jump(merge, &[r.into()]);

        self.builder.switch_to_block(slow_b);
        self.builder.seal_block(slow_b);
        let sr = self.call2(slow, a, b);
        self.builder.ins().jump(merge, &[sr.into()]);

        self.builder.switch_to_block(merge);
        self.builder.seal_block(merge);
        self.builder.block_params(merge)[0]
    }

    /// Folds addition or subtraction through the pairwise fast path.
    fn fold_fix(
        &mut self,
        args: &[Ast],
        add: bool,
        slow: FuncId,
        guard_types: bool,
    ) -> Result<CValue, Diagnostic> {
        let mut vals = Vec::with_capacity(args.len());
        let mut pushed = 0usize;
        for a in args {
            let (v, p) = self.operand(a)?;
            pushed += p as usize;
            vals.push(v);
        }
        let mut acc = vals[0];
        for v in &vals[1..] {
            acc = self.gen_fix_arith(acc, *v, add, slow, guard_types);
        }
        self.gc_popn(pushed);
        Ok(acc)
    }
    fn fix_cmp2(
        &mut self,
        args: &[Ast],
        cc: IntCC,
        slow: FuncId,
        guard_types: bool,
    ) -> Result<CValue, Diagnostic> {
        let (a, pa) = self.operand(&args[0])?;
        let (b, pb) = self.operand(&args[1])?;
        let r = self.gen_fix_cmp(a, b, cc, slow, guard_types);
        self.gc_popn(pa as usize + pb as usize);
        Ok(r)
    }
    fn fix_una(
        &mut self,
        args: &[Ast],
        delta: i64,
        slow: FuncId,
        guard_type: bool,
    ) -> Result<CValue, Diagnostic> {
        let (a, pa) = self.operand(&args[0])?;
        let r = self.gen_fix_unop(a, delta, slow, guard_type);
        self.gc_popn(pa as usize);
        Ok(r)
    }

    /// Checked raw fixnum addition/subtraction with the tagged runtime as the
    /// semantic overflow slow path.
    fn gen_raw_fix_arith(
        &mut self,
        left: CValue,
        right: CValue,
        add: bool,
        slow: FuncId,
    ) -> CValue {
        let result = if add {
            self.builder.ins().iadd(left, right)
        } else {
            self.builder.ins().isub(left, right)
        };
        let in_range = self.fix_in_range(result);
        let fast = self.builder.create_block();
        let slow_block = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        self.builder
            .ins()
            .brif(in_range, fast, &[], slow_block, &[]);

        self.builder.switch_to_block(fast);
        self.builder.seal_block(fast);
        self.builder.ins().jump(merge, &[result.into()]);

        self.builder.switch_to_block(slow_block);
        self.builder.seal_block(slow_block);
        let tagged_left = self.fix_retag(left);
        let tagged_right = self.fix_retag(right);
        let tagged = self.call2(slow, tagged_left, tagged_right);
        let raw = self.builder.ins().sshr_imm_s(tagged, 1);
        self.builder.ins().jump(merge, &[raw.into()]);

        self.builder.switch_to_block(merge);
        self.builder.seal_block(merge);
        self.builder.block_params(merge)[0]
    }

    fn gen_raw_fix_mul(&mut self, left: CValue, right: CValue) -> CValue {
        let result = self.builder.ins().imul(left, right);
        let high = self.builder.ins().smulhi(left, right);
        let expected_high = self.builder.ins().sshr_imm_s(result, 63);
        let no_overflow = self.builder.ins().icmp(IntCC::Equal, high, expected_high);
        let in_range = self.fix_in_range(result);
        let valid = self.builder.ins().band(no_overflow, in_range);
        let fast = self.builder.create_block();
        let slow_block = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        self.builder.ins().brif(valid, fast, &[], slow_block, &[]);

        self.builder.switch_to_block(fast);
        self.builder.seal_block(fast);
        self.builder.ins().jump(merge, &[result.into()]);

        self.builder.switch_to_block(slow_block);
        self.builder.seal_block(slow_block);
        let tagged_left = self.fix_retag(left);
        let tagged_right = self.fix_retag(right);
        let tagged = self.call2(self.rt.mul, tagged_left, tagged_right);
        let raw = self.builder.ins().sshr_imm_s(tagged, 1);
        self.builder.ins().jump(merge, &[raw.into()]);

        self.builder.switch_to_block(merge);
        self.builder.seal_block(merge);
        self.builder.block_params(merge)[0]
    }

    fn gen_raw_fix_unop(&mut self, value: CValue, delta: i64, slow: FuncId) -> CValue {
        let result = self.builder.ins().iadd_imm_s(value, delta);
        let in_range = self.fix_in_range(result);
        let fast = self.builder.create_block();
        let slow_block = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        self.builder
            .ins()
            .brif(in_range, fast, &[], slow_block, &[]);

        self.builder.switch_to_block(fast);
        self.builder.seal_block(fast);
        self.builder.ins().jump(merge, &[result.into()]);

        self.builder.switch_to_block(slow_block);
        self.builder.seal_block(slow_block);
        let tagged_value = self.fix_retag(value);
        let tagged = self.call1(slow, tagged_value);
        let raw = self.builder.ins().sshr_imm_s(tagged, 1);
        self.builder.ins().jump(merge, &[raw.into()]);

        self.builder.switch_to_block(merge);
        self.builder.seal_block(merge);
        self.builder.block_params(merge)[0]
    }

    fn gen_raw_fix_div(
        &mut self,
        left: CValue,
        right: CValue,
        quotient: bool,
        slow: FuncId,
    ) -> CValue {
        let zero = self.builder.ins().icmp_imm_s(IntCC::Equal, right, 0);
        let slow_block = self.builder.create_block();
        let divide = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        self.builder.ins().brif(zero, slow_block, &[], divide, &[]);

        self.builder.switch_to_block(divide);
        self.builder.seal_block(divide);
        if quotient {
            let result = self.builder.ins().sdiv(left, right);
            let in_range = self.fix_in_range(result);
            let fast = self.builder.create_block();
            self.builder
                .ins()
                .brif(in_range, fast, &[], slow_block, &[]);
            self.builder.switch_to_block(fast);
            self.builder.seal_block(fast);
            self.builder.ins().jump(merge, &[result.into()]);
        } else {
            let remainder = self.builder.ins().srem(left, right);
            let nonzero = self.builder.ins().icmp_imm_s(IntCC::NotEqual, remainder, 0);
            let signs_differ = {
                let xor = self.builder.ins().bxor(remainder, right);
                self.builder.ins().icmp_imm_s(IntCC::SignedLessThan, xor, 0)
            };
            let adjust = self.builder.ins().band(nonzero, signs_differ);
            let adjusted = self.builder.ins().iadd(remainder, right);
            let result = self.builder.ins().select(adjust, adjusted, remainder);
            self.builder.ins().jump(merge, &[result.into()]);
        }

        self.builder.switch_to_block(slow_block);
        self.builder.seal_block(slow_block);
        let tagged_left = self.fix_retag(left);
        let tagged_right = self.fix_retag(right);
        let tagged = self.call2(slow, tagged_left, tagged_right);
        let raw = self.builder.ins().sshr_imm_s(tagged, 1);
        self.builder.ins().jump(merge, &[raw.into()]);

        self.builder.switch_to_block(merge);
        self.builder.seal_block(merge);
        self.builder.block_params(merge)[0]
    }

    /// Lowers one proven arithmetic island entirely in raw fixnum form.
    fn gen_raw_fixnum_prim(
        &mut self,
        primitive: Prim,
        arguments: &[Ast],
    ) -> Result<(CValue, usize), Diagnostic> {
        let mut values = Vec::with_capacity(arguments.len());
        let mut pushed = 0usize;
        for argument in arguments {
            let (value, roots) = self.raw_fixnum_operand(argument)?;
            values.push(value);
            pushed += roots;
        }
        let result = match primitive {
            Prim::Add => {
                let mut result = values[0];
                for value in &values[1..] {
                    result = self.gen_raw_fix_arith(result, *value, true, self.rt.add);
                }
                result
            }
            Prim::Sub if values.len() == 1 => {
                let zero = self.builder.ins().iconst(types::I64, 0);
                self.gen_raw_fix_arith(zero, values[0], false, self.rt.sub)
            }
            Prim::Sub => {
                let mut result = values[0];
                for value in &values[1..] {
                    result = self.gen_raw_fix_arith(result, *value, false, self.rt.sub);
                }
                result
            }
            Prim::Mul => {
                let mut result = values[0];
                for value in &values[1..] {
                    result = self.gen_raw_fix_mul(result, *value);
                }
                result
            }
            Prim::Quot => self.gen_raw_fix_div(values[0], values[1], true, self.rt.quot),
            Prim::Mod => self.gen_raw_fix_div(values[0], values[1], false, self.rt.mod_),
            Prim::Inc => self.gen_raw_fix_unop(values[0], 1, self.rt.inc),
            Prim::Dec => self.gen_raw_fix_unop(values[0], -1, self.rt.dec),
            _ => {
                return Err(Diagnostic::error(
                    "E0120",
                    "primitiva sem lowering fixnum raw",
                ))
            }
        };
        Ok((result, pushed))
    }

    /// Lowers an IR-proven fixnum primitive without redundant tag guards.
    ///
    /// Overflow, division by zero, and fixnum-range failures still branch to
    /// the original runtime slow path. Only the operand type checks are
    /// removed, because `clojure-ir` proved their tagged representation.
    fn gen_proven_fixnum_prim(&mut self, prim: Prim, args: &[Ast]) -> Result<CValue, Diagnostic> {
        if !self.enable_adr15 {
            return self.gen_tagged_proven_fixnum_prim(prim, args);
        }
        if matches!(
            prim,
            Prim::Add | Prim::Sub | Prim::Mul | Prim::Quot | Prim::Mod | Prim::Inc | Prim::Dec
        ) {
            let (raw, pushed) = self.gen_raw_fixnum_prim(prim, args)?;
            let tagged = self.fix_retag(raw);
            self.gc_popn(pushed);
            return Ok(tagged);
        }
        match prim {
            Prim::Lt | Prim::Le | Prim::Gt | Prim::Ge => {
                let (left, left_roots) = self.raw_fixnum_operand(&args[0])?;
                let (right, right_roots) = self.raw_fixnum_operand(&args[1])?;
                let condition = match prim {
                    Prim::Lt => IntCC::SignedLessThan,
                    Prim::Le => IntCC::SignedLessThanOrEqual,
                    Prim::Gt => IntCC::SignedGreaterThan,
                    Prim::Ge => IntCC::SignedGreaterThanOrEqual,
                    _ => unreachable!(),
                };
                let comparison = self.builder.ins().icmp(condition, left, right);
                let truth = self.builder.ins().iconst(types::I64, TRUEV);
                let falsehood = self.builder.ins().iconst(types::I64, FALSEV);
                let result = self.builder.ins().select(comparison, truth, falsehood);
                self.gc_popn(left_roots + right_roots);
                Ok(result)
            }
            Prim::Eq => {
                let (left, left_roots) = self.raw_fixnum_operand(&args[0])?;
                let (right, right_roots) = self.raw_fixnum_operand(&args[1])?;
                let equal = self.builder.ins().icmp(IntCC::Equal, left, right);
                let truth = self.builder.ins().iconst(types::I64, TRUEV);
                let falsehood = self.builder.ins().iconst(types::I64, FALSEV);
                let result = self.builder.ins().select(equal, truth, falsehood);
                self.gc_popn(left_roots + right_roots);
                Ok(result)
            }
            _ => Err(Diagnostic::error(
                "E0120",
                "primitiva marcada como fixnum sem lowering especializado",
            )),
        }
    }

    /// Preserves the admitted ADR-0014 tagged lowering when ADR-0015 is off.
    fn gen_tagged_proven_fixnum_prim(
        &mut self,
        prim: Prim,
        args: &[Ast],
    ) -> Result<CValue, Diagnostic> {
        match prim {
            Prim::Add => self.fold_fix(args, true, self.rt.add, false),
            Prim::Sub => {
                if args.len() == 1 {
                    let (value, pushed) = self.operand(&args[0])?;
                    let zero = self.konst(1);
                    let result = self.gen_fix_arith(zero, value, false, self.rt.sub, false);
                    self.gc_popn(pushed as usize);
                    Ok(result)
                } else {
                    self.fold_fix(args, false, self.rt.sub, false)
                }
            }
            Prim::Mul => {
                let mut values = Vec::with_capacity(args.len());
                let mut pushed = 0usize;
                for argument in args {
                    let (value, was_pushed) = self.operand(argument)?;
                    pushed += was_pushed as usize;
                    values.push(value);
                }
                let mut result = values[0];
                for value in &values[1..] {
                    result = self.gen_fix_mul(result, *value, self.rt.mul, false);
                }
                self.gc_popn(pushed);
                Ok(result)
            }
            Prim::Quot | Prim::Mod => {
                let (left, left_pushed) = self.operand(&args[0])?;
                let (right, right_pushed) = self.operand(&args[1])?;
                let (quotient, slow) = if prim == Prim::Quot {
                    (true, self.rt.quot)
                } else {
                    (false, self.rt.mod_)
                };
                let result = self.gen_fix_div(left, right, quotient, slow, false);
                self.gc_popn(left_pushed as usize + right_pushed as usize);
                Ok(result)
            }
            Prim::Inc | Prim::Dec => {
                let delta = if prim == Prim::Inc { 1 } else { -1 };
                let slow = if prim == Prim::Inc {
                    self.rt.inc
                } else {
                    self.rt.dec
                };
                self.fix_una(args, delta, slow, false)
            }
            Prim::Lt | Prim::Le | Prim::Gt | Prim::Ge => {
                let condition = match prim {
                    Prim::Lt => IntCC::SignedLessThan,
                    Prim::Le => IntCC::SignedLessThanOrEqual,
                    Prim::Gt => IntCC::SignedGreaterThan,
                    Prim::Ge => IntCC::SignedGreaterThanOrEqual,
                    _ => unreachable!(),
                };
                let slow = match prim {
                    Prim::Lt => self.rt.lt,
                    Prim::Le => self.rt.le,
                    Prim::Gt => self.rt.gt,
                    Prim::Ge => self.rt.ge,
                    _ => unreachable!(),
                };
                self.fix_cmp2(args, condition, slow, false)
            }
            Prim::Eq => {
                let (left, left_pushed) = self.operand(&args[0])?;
                let (right, right_pushed) = self.operand(&args[1])?;
                let equal = self.builder.ins().icmp(IntCC::Equal, left, right);
                let truth = self.builder.ins().iconst(types::I64, TRUEV);
                let falsehood = self.builder.ins().iconst(types::I64, FALSEV);
                let result = self.builder.ins().select(equal, truth, falsehood);
                self.gc_popn(left_pushed as usize + right_pushed as usize);
                Ok(result)
            }
            _ => Err(Diagnostic::error(
                "E0120",
                "primitiva marcada como fixnum sem lowering especializado",
            )),
        }
    }

    fn gen_prim(&mut self, prim: Prim, args: &[Ast]) -> Result<CValue, Diagnostic> {
        match prim {
            Prim::Println | Prim::Print | Prim::Pr | Prim::Prn => self.gen_print(prim, args),
            Prim::Newline => {
                self.call_void(self.rt.out_newline, &[]);
                Ok(self.konst(NIL))
            }
            Prim::Str => self.gen_str(args),
            Prim::List => self.gen_list(args),
            // Arithmetic with fixnum fast paths (ADR-0006).
            Prim::Add => self.fold_fix(args, true, self.rt.add, true),
            Prim::Sub => {
                if args.len() == 1 {
                    let (v, pv) = self.operand(&args[0])?;
                    let zero = self.konst(1); // MK_FIX(0) is immediate and unrooted.
                    let r = self.call2(self.rt.sub, zero, v);
                    self.gc_popn(pv as usize);
                    Ok(r)
                } else {
                    self.fold_fix(args, false, self.rt.sub, true)
                }
            }
            Prim::Mul => {
                let mut vals = Vec::with_capacity(args.len());
                let mut pushed = 0usize;
                for a in args {
                    let (v, p) = self.operand(a)?;
                    pushed += p as usize;
                    vals.push(v);
                }
                let mut acc = vals[0];
                for v in &vals[1..] {
                    acc = self.gen_fix_mul(acc, *v, self.rt.mul, true);
                }
                self.gc_popn(pushed);
                Ok(acc)
            }
            // Comparisons with fixnum fast paths.
            Prim::Lt => self.fix_cmp2(args, IntCC::SignedLessThan, self.rt.lt, true),
            Prim::Le => self.fix_cmp2(args, IntCC::SignedLessThanOrEqual, self.rt.le, true),
            Prim::Gt => self.fix_cmp2(args, IntCC::SignedGreaterThan, self.rt.gt, true),
            Prim::Ge => self.fix_cmp2(args, IntCC::SignedGreaterThanOrEqual, self.rt.ge, true),
            // Integer division with fixnum fast paths.
            Prim::Quot => {
                let (a, pa) = self.operand(&args[0])?;
                let (b, pb) = self.operand(&args[1])?;
                let r = self.gen_fix_div(a, b, true, self.rt.quot, true);
                self.gc_popn(pa as usize + pb as usize);
                Ok(r)
            }
            Prim::Mod => {
                let (a, pa) = self.operand(&args[0])?;
                let (b, pb) = self.operand(&args[1])?;
                let r = self.gen_fix_div(a, b, false, self.rt.mod_, true);
                self.gc_popn(pa as usize + pb as usize);
                Ok(r)
            }
            Prim::Eq => self.bin(self.rt.eq, args),
            Prim::Cons => self.bin(self.rt.cons, args),
            // Unary operations.
            Prim::Inc => self.fix_una(args, 1, self.rt.inc, true),
            Prim::Dec => self.fix_una(args, -1, self.rt.dec, true),
            Prim::Not => self.una(self.rt.not_, args),
            Prim::NilP => self.una(self.rt.nilp, args),
            Prim::EmptyP => self.una(self.rt.emptyp, args),
            Prim::First => self.una(self.rt.first, args),
            Prim::Rest => self.una(self.rt.rest, args),
            Prim::Count => self.una(self.rt.count, args),
            // Collection operations.
            Prim::Get => self.bin(self.rt.p_get, args),
            Prim::Nth => {
                if args.len() == 2 {
                    self.gen_nth_fast(args) // Inline vector fast path (ADR-0006/0008).
                } else {
                    self.tern(self.rt.p_nth_or, args) // Arity 3 supplies not-found.
                }
            }
            Prim::Assoc => self.gen_assoc(args),
            Prim::Dissoc => self.bin(self.rt.p_dissoc, args),
            Prim::Contains => self.bin(self.rt.p_contains, args),
            Prim::Keys => self.una(self.rt.p_keys, args),
            Prim::Vals => self.una(self.rt.p_vals, args),
            Prim::Conj => self.bin(self.rt.p_conj, args),
            Prim::Vector => self.gen_vec(args),
            Prim::HashSet => self.gen_set(args),
            Prim::HashMap => {
                // Convert alternating hash-map operands to key/value pairs.
                let pairs: Vec<(Ast, Ast)> = args
                    .chunks_exact(2)
                    .map(|c| (c[0].clone(), c[1].clone()))
                    .collect();
                self.gen_map(&pairs)
            }
            Prim::SortedSet => self.gen_sorted_set(args),
            Prim::SortedMap => {
                let pairs: Vec<(Ast, Ast)> = args
                    .chunks_exact(2)
                    .map(|c| (c[0].clone(), c[1].clone()))
                    .collect();
                self.gen_sorted_map(&pairs)
            }
            Prim::Compare => self.bin(self.rt.compare, args),
            Prim::Throw => self.una(self.rt.throw_, args), // noreturn (longjmp)
            Prim::Try => self.tern(self.rt.try_, args),
            Prim::Slurp => self.una(self.rt.slurp, args),
            Prim::Spit => self.bin(self.rt.spit, args),
            Prim::FileExists => self.una(self.rt.file_exists, args),
            Prim::Getenv => self.una(self.rt.getenv, args),
            Prim::WithOutStr => self.una(self.rt.with_out_str, args),
            Prim::VarGet => self.una(self.rt.var_get, args),
            Prim::WithBinding => self.tern(self.rt.with_binding, args),
            Prim::ReadLine => Ok(self.call0(self.rt.read_line)),
            Prim::StringReader => self.una(self.rt.string_reader, args),
            Prim::StringWriter => Ok(self.call0(self.rt.string_writer)),
            Prim::WriterToString => self.una(self.rt.writer_to_string, args),
            Prim::StreamClosed => self.una(self.rt.stream_closed, args),
            Prim::StreamReaderP => self.una(self.rt.reader_p, args),
            Prim::StreamWriterP => self.una(self.rt.writer_p, args),
            Prim::ReadCharFrom => self.una(self.rt.read_char_from, args),
            Prim::ReadLineFrom => self.una(self.rt.read_line_from, args),
            Prim::UnreadCharTo => self.bin(self.rt.unread_char_to, args),
            Prim::WriteTo => self.bin(self.rt.write_to, args),
            Prim::FlushWriter => self.una(self.rt.flush_writer, args),
            Prim::CloseableP => self.una(self.rt.closeable_p, args),
            Prim::CharOf => self.una(self.rt.char_of, args),
            Prim::IntOf => self.una(self.rt.int_of, args),
            Prim::CharP => self.una(self.rt.charp, args),
            Prim::ReadChar => Ok(self.call0(self.rt.read_char)),
            Prim::PathJoin => self.bin(self.rt.path_join, args),
            Prim::FileName => self.una(self.rt.file_name, args),
            Prim::Parent => self.una(self.rt.parent, args),
            Prim::Bytes => self.una(self.rt.bytes, args),
            Prim::BytesToString => self.una(self.rt.bytes_to_string, args),
            Prim::Bget => self.bin(self.rt.bget, args),
            Prim::BytesOfVec => self.una(self.rt.bytes_of_vec, args),
            Prim::BytesToVec => self.una(self.rt.bytes_to_vec, args),
            Prim::ValidUtf8 => self.una(self.rt.valid_utf8, args),
            Prim::ByteInputStream => self.una(self.rt.byte_input_stream, args),
            Prim::ByteOutputStream => Ok(self.call0(self.rt.byte_output_stream)),
            Prim::ReadBytes => self.bin(self.rt.read_bytes, args),
            Prim::WriteBytes => self.bin(self.rt.write_bytes, args),
            Prim::OutputBytes => self.una(self.rt.output_bytes, args),
            Prim::ReadBlock => self.bin(self.rt.read_block, args),
            Prim::ByteInputP => self.una(self.rt.byte_input_p, args),
            Prim::ByteOutputP => self.una(self.rt.byte_output_p, args),
            Prim::SeekFile => self.bin(self.rt.seek_file, args),
            Prim::TruncateFile => self.bin(self.rt.truncate_file, args),
            Prim::PositionFile => self.una(self.rt.position_file, args),
            Prim::FileReaderP => self.una(self.rt.file_reader_p, args),
            Prim::FileWriterP => self.una(self.rt.file_writer_p, args),
            Prim::CreateSymlink => self.bin(self.rt.create_symlink, args),
            Prim::ReadLink => self.una(self.rt.read_link, args),
            Prim::NativeSymlinkP => self.una(self.rt.native_symlink_p, args),
            Prim::PathAbsolute => self.una(self.rt.path_absolute, args),
            Prim::PathNormalize => self.una(self.rt.path_normalize, args),
            Prim::RealPath => self.una(self.rt.real_path, args),
            Prim::ProcessCwd => Ok(self.call0(self.rt.process_cwd)),
            Prim::ProcessEnvironment => Ok(self.call0(self.rt.process_environment)),
            Prim::SlurpBytes => self.una(self.rt.slurp_bytes, args),
            Prim::SpitBytes => self.bin(self.rt.spit_bytes, args),
            Prim::ReadString => self.una(self.rt.read_string, args),
            Prim::ReadFrom => self.una(self.rt.read_from, args),
            Prim::ReaderEof => self.una(self.rt.reader_eof, args),
            Prim::WriterOpen => self.una(self.rt.writer_open, args),
            Prim::ReaderOpen => self.una(self.rt.reader_open, args),
            Prim::Close => self.una(self.rt.close, args),
            Prim::Flush => Ok(self.call0(self.rt.flush)),
            Prim::Mkdir => self.una(self.rt.mkdir, args),
            Prim::Mkdirs => self.una(self.rt.mkdirs, args),
            Prim::ListDir => self.una(self.rt.list_dir, args),
            Prim::DeleteFile => self.una(self.rt.delete_file, args),
            Prim::Rename => self.bin(self.rt.rename, args),
            Prim::DirectoryP => self.una(self.rt.directoryp, args),
            Prim::FileP => self.una(self.rt.filep, args),
            Prim::FileSize => self.una(self.rt.file_size, args),
            Prim::FileModified => self.una(self.rt.file_modified, args),
            Prim::Div => self.bin(self.rt.div, args),
            Prim::FloatP => self.una(self.rt.floatp, args),
            Prim::DoubleOf => self.una(self.rt.double_of, args),
            Prim::StringP => self.una(self.rt.stringp, args),
            Prim::IntP => self.una(self.rt.intp, args),
            Prim::KeywordP => self.una(self.rt.keywordp, args),
            Prim::VectorP => self.una(self.rt.vectorp, args),
            Prim::MapP => self.una(self.rt.mapp, args),
            Prim::BytesP => self.una(self.rt.bytesp, args),
            Prim::StrSplit => self.bin(self.rt.str_split, args),
            Prim::ParseHttpRequest => self.una(self.rt.parse_http_request, args),
            Prim::SerializeHttpResponse => self.una(self.rt.serialize_http_response, args),
            Prim::HttpServerOpen => self.una(self.rt.http_server_open, args),
            Prim::HttpServerPort => self.una(self.rt.http_server_port, args),
            Prim::HttpServerAccept => self.una(self.rt.http_server_accept, args),
            Prim::HttpServerRespond => self.bin(self.rt.http_server_respond, args),
            Prim::HttpServerClose => self.una(self.rt.http_server_close, args),
            Prim::HttpServerStop => self.una(self.rt.http_server_stop, args),
            Prim::Transient => self.una(self.rt.transient, args),
            Prim::PersistentBang => self.una(self.rt.persistent_bang, args),
            Prim::ConjBang => self.bin(self.rt.conj_bang, args),
            Prim::AssocBang => self.tern(self.rt.assoc_bang, args),
            Prim::DissocBang => self.bin(self.rt.dissoc_bang, args),
        }
    }

    /// Calls a ternary primitive with net-zero root-stack effect.
    ///
    /// Only heap-capable operands are rooted and subsequently removed.
    fn tern(&mut self, id: FuncId, args: &[Ast]) -> Result<CValue, Diagnostic> {
        let (a, pa) = self.operand(&args[0])?;
        let (b, pb) = self.operand(&args[1])?;
        let (c, pc) = self.operand(&args[2])?;
        let r = self.call3(id, a, b, c);
        self.gc_popn(pa as usize + pb as usize + pc as usize);
        Ok(r)
    }

    fn bin(&mut self, id: FuncId, args: &[Ast]) -> Result<CValue, Diagnostic> {
        let (a, pa) = self.operand(&args[0])?;
        let (b, pb) = self.operand(&args[1])?;
        let r = self.call2(id, a, b);
        self.gc_popn(pa as usize + pb as usize);
        Ok(r)
    }
    fn una(&mut self, id: FuncId, args: &[Ast]) -> Result<CValue, Diagnostic> {
        let (a, pa) = self.operand(&args[0])?;
        let r = self.call1(id, a);
        self.gc_popn(pa as usize);
        Ok(r)
    }

    fn gen_print(&mut self, prim: Prim, args: &[Ast]) -> Result<CValue, Diagnostic> {
        // pr/prn honor the cljn.io stream contract: writing to a closed *out*
        // raises :invalid-input before any bytes are emitted.
        let readable = matches!(prim, Prim::Pr | Prim::Prn);
        if readable {
            self.call_void(self.rt.out_check, &[]);
        }
        for (i, a) in args.iter().enumerate() {
            if i > 0 {
                self.call_void(self.rt.print_space, &[]);
            }
            let v = self.spill_arg(a)?; // Spill immediates and root during print.
            self.call_void(self.rt.print, &[v]);
        }
        self.gc_popn(args.len());
        if matches!(prim, Prim::Println) {
            self.call_void(self.rt.print_newline, &[]);
        } else if matches!(prim, Prim::Prn) {
            self.call_void(self.rt.out_newline, &[]);
        }
        Ok(self.konst(NIL))
    }

    fn gen_str(&mut self, args: &[Ast]) -> Result<CValue, Diagnostic> {
        if args.is_empty() {
            let nil = self.konst(NIL);
            return Ok(self.call1(self.rt.to_str, nil));
        }
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            vals.push(self.spill_arg(a)?); // One rooted temporary per argument.
        }
        let mut acc = self.call1(self.rt.to_str, vals[0]);
        self.gc_push_val(acc); // Root the accumulator.
        for v in &vals[1..] {
            let s = self.call1(self.rt.to_str, *v);
            self.gc_push_val(s); // Root the converted string.
            acc = self.call2(self.rt.str_concat, acc, s);
            self.gc_popn(2); // Remove string and previous accumulator.
            self.gc_push_val(acc); // Root the new accumulator.
        }
        self.gc_popn(args.len() + 1); // Remove arguments and accumulator.
        Ok(acc)
    }

    fn gen_list(&mut self, args: &[Ast]) -> Result<CValue, Diagnostic> {
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            vals.push(self.spill_arg(a)?); // One rooted temporary per argument.
        }
        let mut acc = self.call0(self.rt.empty);
        self.gc_push_val(acc); // Root the accumulator.
        for v in vals.iter().rev() {
            acc = self.call2(self.rt.cons, *v, acc); // Both inputs are rooted.
            self.gc_popn(1); // Remove the previous accumulator.
            self.gc_push_val(acc); // Root the new accumulator.
        }
        self.gc_popn(args.len() + 1); // Remove arguments and accumulator.
        Ok(acc)
    }
}

#[cfg(test)]
#[path = "../tests/unit/lib/mod.rs"]
mod tests;
