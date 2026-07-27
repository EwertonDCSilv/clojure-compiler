//! Codegen nativo (Fase 4/5 slice): `Program` (AST) → objeto nativo via Cranelift.
//!
//! Representação de valor **tagged** em `i64` (ADR-0003, variante compilada):
//! fixnum `(n<<1)|1`; ponteiro (baixo3=000) para string/cons; imediatos
//! `NIL/TRUE/FALSE/EMPTY`. Toda operação semântica (aritmética, comparação, seq,
//! print, str) é uma **chamada ao runtime** (`runtime.c`, ABI C — ver
//! specs/ARCHITECTURE.md §Runtime ABI). Isso centraliza a semântica e mantém o
//! codegen simples.
//!
//! Suporta: `defn` (aridade fixa), `if`, `do`, `let`, recursão direta; literais
//! int/bool/nil/string; primitivas `+ - * quot mod inc dec = < <= > >= not nil?
//! empty? cons first rest count list str println print`.

use clojure_analyzer::{Ast, Callee, Dispatch, FnMethod, Prim, Program};
use clojure_diagnostics::{Diagnostic, Diagnostics};
use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{types, AbiParam, Block, InstBuilder, MemFlags, Value as CValue};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::{isa, Context};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{default_libcall_names, DataDescription, DataId, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use std::collections::HashMap;
use std::str::FromStr;
use target_lexicon::Triple;

macro_rules! embed_runtime_modules {
    ($(($name:literal, $path:literal)),+ $(,)?) => {
        /// Fonte C amalgamada do runtime, compilada junto ao objeto no passo de link.
        ///
        /// Os módulos permanecem em arquivos separados para revisão e testes, mas são
        /// concatenados na ordem original. O compilador C continua recebendo uma única
        /// unidade de tradução, preservando visibilidade interna, ABI e oportunidades
        /// de otimização entre subsistemas.
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
    ("print", "../runtime/90_print.c"),
    ("exceptions", "../runtime/100_exceptions.c"),
    ("multimethods", "../runtime/110_multimethods.c"),
    ("test-introspection", "../runtime/120_test_introspection.c"),
);

/// Nível de otimização aplicado pelo Cranelift ao código gerado.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptimizationLevel {
    None,
    Speed,
    SpeedAndSize,
}

impl OptimizationLevel {
    fn cranelift_value(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Speed => "speed",
            Self::SpeedAndSize => "speed_and_size",
        }
    }
}

impl FromStr for OptimizationLevel {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "none" => Ok(Self::None),
            "speed" => Ok(Self::Speed),
            "speed-and-size" | "speed_and_size" => Ok(Self::SpeedAndSize),
            _ => Err("esperado: none, speed ou speed-and-size"),
        }
    }
}

/// Opções do backend independentes do perfil usado para compilar o runtime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CodegenOptions {
    pub optimization_level: OptimizationLevel,
}

impl CodegenOptions {
    pub const fn unoptimized() -> Self {
        Self {
            optimization_level: OptimizationLevel::None,
        }
    }

    pub const fn optimized_for_speed() -> Self {
        Self {
            optimization_level: OptimizationLevel::Speed,
        }
    }
}

impl Default for CodegenOptions {
    fn default() -> Self {
        // `speed` permanece opt-in: o baseline Cormen de 2026-07-26 mostrou
        // regressão em 25/30 casos por aumento de spills e do tamanho dos frames.
        Self::unoptimized()
    }
}

// Constantes de valor tagged (devem casar com runtime.c).
const NIL: i64 = 2;
const FALSEV: i64 = 6;
const TRUEV: i64 = 10;
const T_VEC: i64 = 5; // tag de PersistentVector (deve casar com runtime.c)
                      // Offsets dos campos de PVec/VNode (Obj = 16 B). Devem casar com runtime.c.
const PV_COUNT: i32 = 16;
const PV_SHIFT: i32 = 24;
const PV_ROOT: i32 = 32;
const PV_TAIL: i32 = 40;
const PV_TAILLEN: i32 = 48;
const VNODE_SLOTS: i32 = 24; // Obj(16) + edit(8); slots começam em 24
// Intervalo de fixnum (deve casar com FIXNUM_MIN/MAX em runtime.c).
const FIX_MIN: i64 = -(1 << 62);
const FIX_MAX: i64 = (1 << 62) - 1;

/// FuncIds das funções de runtime importadas.
struct Runtime {
    // binárias (i64,i64)->i64
    add: FuncId,
    sub: FuncId,
    mul: FuncId,
    quot: FuncId,
    mod_: FuncId,
    lt: FuncId,
    le: FuncId,
    gt: FuncId,
    ge: FuncId,
    eq: FuncId,
    cons: FuncId,
    str_concat: FuncId,
    // unárias (i64)->i64
    inc: FuncId,
    dec: FuncId,
    not_: FuncId,
    nilp: FuncId,
    emptyp: FuncId,
    first: FuncId,
    rest: FuncId,
    count: FuncId,
    to_str: FuncId,
    // outras
    empty: FuncId,       // ()->i64
    str_from: FuncId,    // (ptr,i64)->i64
    truthy: FuncId,      // (i64)->i32
    print: FuncId,       // (i64)->void
    print_space: FuncId, // ()->void
    print_newline: FuncId,
    // GC / shadow-stack de roots
    gc_enter: FuncId, // (i64)->i64  reserva slots; devolve base
    gc_leave: FuncId, // (i64)->void restaura sp=base
    // ADR-0006 Fase 3: push/popn/set viram stores diretos nestes globais.
    gc_stack_data: DataId, // Value gc_stack[]
    gc_sp_data: DataId,    // int64_t gc_sp
    // Funções de primeira classe
    make_fn: FuncId,  // (code,arity,nfree)->fn
    set_free: FuncId, // (fn,i,v)->void
    fn_free: FuncId,  // (fn,i)->v
    fn_code: FuncId,  // (fn)->code
    check_fn: FuncId, // (fn)->void  (é função?)
    // convenção de chamada (self, argc, argv)
    argv: FuncId,         // (argc)->ptr (topo do shadow-stack)
    check_arity: FuncId,  // (argc,expected)->void
    collect_rest: FuncId, // (argc,argv,nfixed)->list
    spread_args: FuncId,  // (fixed_argc,coll)->argc_total
    // coleções
    kw: FuncId,               // (ptr,len)->kw
    vec_empty: FuncId,        // ()->vec
    vec_conj: FuncId,         // (vec,x)->vec
    set_alloc: FuncId,        // (n)->set
    set_add: FuncId,          // (set,x)->void
    map_alloc: FuncId,        // (n)->map
    map_set: FuncId,          // (map,i,k,v)->void
    p_get: FuncId,            // (coll,key)->v
    p_nth: FuncId,            // (coll,i)->v   (nth aridade 2)
    p_nth_or: FuncId,         // (coll,i,nf)->v (nth aridade 3)
    p_assoc: FuncId,          // (coll,k,v)->coll
    p_dissoc: FuncId,         // (map,k)->map
    p_contains: FuncId,       // (coll,key)->bool
    p_keys: FuncId,           // (map)->list
    p_vals: FuncId,           // (map)->list
    p_conj: FuncId,           // (coll,x)->coll
    sorted_map_empty: FuncId, // ()->sorted-map
    sorted_set_empty: FuncId, // ()->sorted-set
    sorted_assoc: FuncId,     // (smap,k,v)->smap
    compare: FuncId,          // (a,b)->fixnum(-1/0/1)
    try_: FuncId,             // (body_fn,catch_fn|nil,finally_fn|nil)->v
    throw_: FuncId,           // (v)->! (longjmp)
    multi_register: FuncId,   // (mid,dispatch_fn)->void
    multi_call: FuncId,       // (mid,argc,argv)->v
    transient: FuncId,        // (coll)->transient
    persistent_bang: FuncId,  // (t)->coll
    conj_bang: FuncId,        // (t,x)->t
    assoc_bang: FuncId,       // (t,k,v)->t
    dissoc_bang: FuncId,      // (t,k)->t
    make_record: FuncId,      // (type_name,map)->record
    // protocols
    type_key: FuncId,        // (v)->key
    register_method: FuncId, // (mid,key,impl)->void
    lookup_method: FuncId,   // (mid,key)->impl|NIL
    no_method: FuncId,       // (mid)->void
}

/// Compila o programa para bytes de um objeto nativo da plataforma host.
pub fn compile_object(program: &Program) -> Result<Vec<u8>, Diagnostics> {
    compile_object_with_options(program, CodegenOptions::default())
}

/// Compila o programa usando uma configuração explícita do backend.
pub fn compile_object_with_options(
    program: &Program,
    options: CodegenOptions,
) -> Result<Vec<u8>, Diagnostics> {
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

    // Dados: strings únicas do programa.
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
        // Bytes crus (o comprimento é passado à parte; sem NUL).
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

    // Declara funções do usuário (para recursão/forward-ref).
    // Convenção uniforme: entry(self, argc, argv) -> valor. `self` = a closure
    // (NIL em chamadas estáticas). `argv` aponta para os args no shadow-stack.
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
        // usize = aridade informativa (primeira aridade) para FnRef.
        let arity0 = f.methods.first().map(|m| m.params.len()).unwrap_or(0);
        fn_ids.insert(f.name.clone(), (id, arity0));
    }

    let mut diags = Diagnostics::new();

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
                &str_data,
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

    {
        let mut sig = module.make_signature();
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
                &str_data,
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
    product
        .emit()
        .map_err(|e| single(format!("emit do objeto: {e}")))
}

fn declare_runtime(m: &mut ObjectModule, ptr: types::Type) -> Runtime {
    let bin = |m: &mut ObjectModule, name: &str| {
        let mut s = m.make_signature();
        s.params.push(AbiParam::new(types::I64));
        s.params.push(AbiParam::new(types::I64));
        s.returns.push(AbiParam::new(types::I64));
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };
    let una = |m: &mut ObjectModule, name: &str| {
        let mut s = m.make_signature();
        s.params.push(AbiParam::new(types::I64));
        s.returns.push(AbiParam::new(types::I64));
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };
    let voidfn = |m: &mut ObjectModule, name: &str, has_arg: bool| {
        let mut s = m.make_signature();
        if has_arg {
            s.params.push(AbiParam::new(types::I64));
        }
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };
    let bin_void = |m: &mut ObjectModule, name: &str| {
        let mut s = m.make_signature();
        s.params.push(AbiParam::new(types::I64));
        s.params.push(AbiParam::new(types::I64));
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };
    let ternary = |m: &mut ObjectModule, name: &str| {
        let mut s = m.make_signature();
        for _ in 0..3 {
            s.params.push(AbiParam::new(types::I64));
        }
        s.returns.push(AbiParam::new(types::I64));
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };
    let ternary_void = |m: &mut ObjectModule, name: &str| {
        let mut s = m.make_signature();
        for _ in 0..3 {
            s.params.push(AbiParam::new(types::I64));
        }
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };
    let quaternary_void = |m: &mut ObjectModule, name: &str| {
        let mut s = m.make_signature();
        for _ in 0..4 {
            s.params.push(AbiParam::new(types::I64));
        }
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };

    let mut str_from_sig = m.make_signature();
    str_from_sig.params.push(AbiParam::new(ptr));
    str_from_sig.params.push(AbiParam::new(types::I64));
    str_from_sig.returns.push(AbiParam::new(types::I64));
    let str_from = m
        .declare_function("cljn_str_from", Linkage::Import, &str_from_sig)
        .unwrap();

    let mut empty_sig = m.make_signature();
    empty_sig.returns.push(AbiParam::new(types::I64));
    let empty = m
        .declare_function("cljn_empty", Linkage::Import, &empty_sig)
        .unwrap();

    let mut truthy_sig = m.make_signature();
    truthy_sig.params.push(AbiParam::new(types::I64));
    truthy_sig.returns.push(AbiParam::new(types::I32));
    let truthy = m
        .declare_function("cljn_truthy", Linkage::Import, &truthy_sig)
        .unwrap();

    Runtime {
        add: bin(m, "cljn_add"),
        sub: bin(m, "cljn_sub"),
        mul: bin(m, "cljn_mul"),
        quot: bin(m, "cljn_quot"),
        mod_: bin(m, "cljn_mod"),
        lt: bin(m, "cljn_lt"),
        le: bin(m, "cljn_le"),
        gt: bin(m, "cljn_gt"),
        ge: bin(m, "cljn_ge"),
        eq: bin(m, "cljn_eq"),
        cons: bin(m, "cljn_cons"),
        str_concat: bin(m, "cljn_str_concat"),
        inc: una(m, "cljn_inc"),
        dec: una(m, "cljn_dec"),
        not_: una(m, "cljn_not"),
        nilp: una(m, "cljn_nilp"),
        emptyp: una(m, "cljn_emptyp"),
        first: una(m, "cljn_first"),
        rest: una(m, "cljn_rest"),
        count: una(m, "cljn_count"),
        to_str: una(m, "cljn_to_str"),
        empty,
        str_from,
        truthy,
        print: voidfn(m, "cljn_print", true),
        print_space: voidfn(m, "cljn_print_space", false),
        print_newline: voidfn(m, "cljn_print_newline", false),
        gc_enter: una(m, "cljn_gc_enter"),
        gc_leave: voidfn(m, "cljn_gc_leave", true),
        gc_stack_data: m
            .declare_data("gc_stack", Linkage::Import, true, false)
            .unwrap(),
        gc_sp_data: m
            .declare_data("gc_sp", Linkage::Import, true, false)
            .unwrap(),
        make_fn: ternary(m, "cljn_make_fn"),
        set_free: ternary_void(m, "cljn_fn_set_free"),
        fn_free: bin(m, "cljn_fn_free"),
        fn_code: una(m, "cljn_fn_code"),
        check_fn: voidfn(m, "cljn_check_fn", true),
        argv: una(m, "cljn_argv"),
        check_arity: bin_void(m, "cljn_check_arity"),
        collect_rest: ternary(m, "cljn_collect_rest"),
        spread_args: bin(m, "cljn_spread_args"),
        kw: {
            let mut s = m.make_signature();
            s.params.push(AbiParam::new(ptr));
            s.params.push(AbiParam::new(types::I64));
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_kw", Linkage::Import, &s).unwrap()
        },
        vec_empty: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_vec_empty", Linkage::Import, &s)
                .unwrap()
        },
        vec_conj: bin(m, "cljn_vec_conj"),
        set_alloc: una(m, "cljn_set_alloc"),
        set_add: bin_void(m, "cljn_set_add"),
        map_alloc: una(m, "cljn_map_alloc"),
        map_set: quaternary_void(m, "cljn_map_set"),
        p_get: bin(m, "cljn_get"),
        p_nth: bin(m, "cljn_nth"),
        p_nth_or: ternary(m, "cljn_nth_or"),
        p_assoc: ternary(m, "cljn_assoc"),
        p_dissoc: bin(m, "cljn_map_dissoc"),
        p_contains: bin(m, "cljn_contains"),
        p_keys: una(m, "cljn_map_keys"),
        p_vals: una(m, "cljn_map_vals"),
        p_conj: bin(m, "cljn_conj"),
        sorted_map_empty: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_sorted_map_empty", Linkage::Import, &s)
                .unwrap()
        },
        sorted_set_empty: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_sorted_set_empty", Linkage::Import, &s)
                .unwrap()
        },
        sorted_assoc: ternary(m, "cljn_sorted_assoc"),
        compare: bin(m, "cljn_compare"),
        try_: ternary(m, "cljn_try"),
        throw_: una(m, "cljn_throw"),
        multi_register: bin_void(m, "cljn_multi_register"),
        multi_call: ternary(m, "cljn_multi_call"),
        transient: una(m, "cljn_transient"),
        persistent_bang: una(m, "cljn_persistent_bang"),
        conj_bang: bin(m, "cljn_conj_bang"),
        assoc_bang: ternary(m, "cljn_assoc_bang"),
        dissoc_bang: bin(m, "cljn_dissoc_bang"),
        make_record: bin(m, "cljn_make_record"),
        type_key: una(m, "cljn_type_key"),
        register_method: ternary_void(m, "cljn_register_method"),
        lookup_method: bin(m, "cljn_lookup_method"),
        no_method: voidfn(m, "cljn_no_method", true),
    }
}

/// Resultado de compilar uma expressão: produz um valor (fall-through) ou
/// diverge (o bloco já foi terminado por `recur`/`return`).
#[derive(Clone, Copy)]
enum Flow {
    Val(CValue),
    Diverged,
}

/// Classificação estática de valor para rooting em safepoints (ADR-0006 Fases 4-5).
/// `Imm` = comprovadamente fixnum/imediato (nunca é ponteiro heap → nunca precisa de
/// root). `Heap` = pode ser ponteiro heap; mantém o rooting eager atual.
///
/// A elisão é **sã por construção**: só deixamos de rootear valores que jamais são
/// ponteiros. Todo valor `Heap` continua sendo rooteado exatamente como antes, então a
/// precisão do coletor é preservada. Slots de frame não escritos permanecem `NIL`
/// (imediato) via `cljn_gc_enter`, então locais `Imm` sem store são seguros.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum VKind {
    Imm,
    Heap,
}
impl VKind {
    fn join(self, other: VKind) -> VKind {
        if self == VKind::Imm && other == VKind::Imm {
            VKind::Imm
        } else {
            VKind::Heap
        }
    }
}

/// Uma primitiva cujo resultado é sempre imediato (fixnum, boolean ou nil), qualquer
/// que seja o caminho (fast path ou slow path do runtime). Aritmética/comparação
/// devolvem fixnum/boolean; `println`/`print` devolvem nil.
fn prim_imm_result(p: Prim) -> bool {
    matches!(
        p,
        Prim::Add
            | Prim::Sub
            | Prim::Mul
            | Prim::Quot
            | Prim::Mod
            | Prim::Inc
            | Prim::Dec
            | Prim::Eq
            | Prim::Lt
            | Prim::Le
            | Prim::Gt
            | Prim::Ge
            | Prim::Not
            | Prim::NilP
            | Prim::EmptyP
            | Prim::Contains
            | Prim::Count
            | Prim::Compare
            | Prim::Println
            | Prim::Print
            | Prim::Throw // diverge; resultado nunca materializa
    )
}

/// Alvo de `recur`: bloco-cabeçalho de um loop/fn + variáveis e slots a religar.
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
    str_data: &'a HashMap<String, (DataId, usize)>,
    vars: HashMap<u32, Variable>,
    /// Kind estático de cada slot de local em escopo (ADR-0006 Fases 4-5).
    kinds: HashMap<u32, VKind>,
    recur_targets: Vec<RecurTarget>,
    /// Base do frame no shadow-stack (i64), definida na entrada da função.
    frame_base: Option<Variable>,
    /// `self` (a closure) da função atual — usado para ler capturas.
    self_var: Option<Variable>,
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
        str_data: &'a HashMap<String, (DataId, usize)>,
    ) -> Self {
        FnGen {
            module,
            builder: FunctionBuilder::new(func, fbctx),
            ptr,
            rt,
            fn_ids,
            str_data,
            vars: HashMap::new(),
            kinds: HashMap::new(),
            recur_targets: Vec::new(),
            frame_base: None,
            self_var: None,
        }
    }

    fn new_var(&mut self, slot: u32) -> Variable {
        let v = self.builder.declare_var(types::I64);
        self.vars.insert(slot, v);
        v
    }

    // -- análise de kind estático (ADR-0006 Fases 4-5) ---------------------
    fn slot_kind(&self, slot: u32, extra: &HashMap<u32, VKind>) -> VKind {
        extra
            .get(&slot)
            .or_else(|| self.kinds.get(&slot))
            .copied()
            .unwrap_or(VKind::Heap)
    }

    /// Kind estático de uma expressão. `extra` sobrepõe `self.kinds` com slots
    /// introduzidos por `let`/`loop` ainda não vinculados no codegen.
    fn kind_of(&self, ast: &Ast, extra: &HashMap<u32, VKind>) -> VKind {
        use VKind::{Heap, Imm};
        match ast {
            Ast::Int(_) | Ast::Bool(_) | Ast::Nil => Imm,
            Ast::Str(_)
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
            Ast::Recur(_) => Imm, // diverge; não produz valor
            Ast::Call { callee, .. } => match callee {
                Callee::Prim(p) if prim_imm_result(*p) => Imm,
                _ => Heap,
            },
            Ast::CallValue { .. } | Ast::Apply { .. } => Heap,
            Ast::RegisterMethod { .. } | Ast::RegisterMulti { .. } => Imm, // devolve nil
        }
    }

    /// Ponto fixo dos kinds dos slots de um `loop`: um slot é `Imm` só se o init e
    /// todo argumento de `recur` naquela posição forem `Imm`. Monótono (Imm→Heap).
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
            let mut rec = vec![VKind::Imm; slot_ids.len()];
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

    /// Junta os kinds dos argumentos de cada `recur` que mira o loop atual (não
    /// desce em loops aninhados). `recur` só ocorre em cauda: basta varrer ramos
    /// de `if`, corpo de `let` e statements de `do`.
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
            Ast::Loop { .. } => {} // recurs internos pertencem ao loop aninhado
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

    /// Se `expr(ast)` deixa um temporário no shadow-stack. É `kind==Heap` para a
    /// maioria dos nós, **exceto**:
    /// - `Local`: nunca empurra — imediato não é ponteiro; Local Heap já está
    ///   rooteado no seu slot de frame (estável enquanto o valor lido é usado);
    /// - `Do`/`Let`/`Loop`: delegam ao sub-expr que produz o valor.
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

    /// Avalia um operando, deixando-o no shadow-stack só se `expr_pushes`. Retorna
    /// o valor e se foi empurrado (para o consumidor desempilhar 1).
    fn operand(&mut self, ast: &Ast) -> Result<(CValue, bool), Diagnostic> {
        let pushed = self.expr_pushes(ast, &HashMap::new());
        let v = self.expr_val(ast)?;
        Ok((v, pushed))
    }

    /// Avalia um argumento de chamada, garantindo que fique no shadow-stack (os
    /// args de funções são passados por `argv`, um ponteiro para o topo da pilha;
    /// imediatos precisam ser derramados para manter a região contígua).
    fn spill_arg(&mut self, ast: &Ast) -> Result<CValue, Diagnostic> {
        let (v, pushed) = self.operand(ast)?;
        if !pushed {
            self.gc_push_val(v);
        }
        Ok(v)
    }

    // -- shadow-stack de roots (stores diretos, ADR-0006 Fase 3) ----------
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
    /// Endereço de `gc_stack[idx]`.
    fn slot_addr(&mut self, idx: CValue) -> CValue {
        let stack = self.addr_gc_stack();
        let off = self.builder.ins().imul_imm(idx, 8);
        self.builder.ins().iadd(stack, off)
    }
    /// Empurra um root: `gc_stack[gc_sp++] = v` (store direto).
    fn gc_push_val(&mut self, v: CValue) {
        let sp_addr = self.addr_gc_sp();
        let sp = self
            .builder
            .ins()
            .load(types::I64, MemFlags::trusted(), sp_addr, 0);
        let elem = self.slot_addr(sp);
        self.builder.ins().store(MemFlags::trusted(), v, elem, 0);
        let sp1 = self.builder.ins().iadd_imm(sp, 1);
        self.builder
            .ins()
            .store(MemFlags::trusted(), sp1, sp_addr, 0);
    }
    fn gc_popn(&mut self, n: usize) {
        if n > 0 {
            let sp_addr = self.addr_gc_sp();
            let sp = self
                .builder
                .ins()
                .load(types::I64, MemFlags::trusted(), sp_addr, 0);
            let sp2 = self.builder.ins().iadd_imm(sp, -(n as i64));
            self.builder
                .ins()
                .store(MemFlags::trusted(), sp2, sp_addr, 0);
        }
    }
    /// Retira uma quantidade calculada em runtime.
    fn gc_popn_val(&mut self, n: CValue) {
        let sp_addr = self.addr_gc_sp();
        let sp = self
            .builder
            .ins()
            .load(types::I64, MemFlags::trusted(), sp_addr, 0);
        let sp2 = self.builder.ins().isub(sp, n);
        self.builder
            .ins()
            .store(MemFlags::trusted(), sp2, sp_addr, 0);
    }
    /// Escreve o slot de root de um local: `gc_stack[base + slot] = v`.
    fn gc_set_local(&mut self, slot: u32, v: CValue) {
        let base_var = self.frame_base.expect("frame_base definido");
        let base = self.builder.use_var(base_var);
        let idx = self.builder.ins().iadd_imm(base, slot as i64);
        let elem = self.slot_addr(idx);
        self.builder.ins().store(MemFlags::trusted(), v, elem, 0);
    }
    /// Vincula um local Heap (params): define a variável e espelha no slot de root.
    fn bind_local(&mut self, slot: u32, v: CValue) {
        self.bind_local_kind(slot, v, VKind::Heap);
    }

    /// Vincula um local com kind conhecido. `Imm` só atualiza a variável Cranelift
    /// (o slot de frame permanece `NIL`, imediato seguro — sem store de root).
    fn bind_local_kind(&mut self, slot: u32, v: CValue, kind: VKind) {
        let var = match self.vars.get(&slot) {
            Some(v) => *v,
            None => self.new_var(slot),
        };
        self.builder.def_var(var, v);
        self.kinds.insert(slot, kind);
        if kind == VKind::Heap {
            self.gc_set_local(slot, v);
        }
    }

    /// Entra num frame de GC reservando `local_count` slots; guarda a base.
    fn enter_frame(&mut self, local_count: u32) {
        let k = self.builder.ins().iconst(types::I64, local_count as i64);
        let base = self.call1(self.rt.gc_enter, k);
        let base_var = self.builder.declare_var(types::I64);
        self.builder.def_var(base_var, base);
        self.frame_base = Some(base_var);
    }
    fn leave_frame(&mut self) {
        let base_var = self.frame_base.expect("frame_base");
        let base = self.builder.use_var(base_var);
        self.call_void(self.rt.gc_leave, &[base]);
    }

    fn build_entry(
        mut self,
        methods: &[FnMethod],
        local_count: u32,
        dispatch: Dispatch,
    ) -> Result<(), Diagnostic> {
        let entry = self.builder.create_block();
        self.builder.append_block_params_for_function_params(entry);
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);
        // Convenção: block params = [self, argc, argv].
        let block_vals: Vec<CValue> = self.builder.block_params(entry).to_vec();
        let self_v = self.builder.declare_var(types::I64);
        self.builder.def_var(self_v, block_vals[0]);
        self.self_var = Some(self_v);
        let argc_v = block_vals[1];
        let argv_v = block_vals[2];

        // Função-despacho (protocolo por tipo, ou multimethod por dispatch-fn).
        match dispatch {
            Dispatch::Protocol(mid) => {
                self.gen_dispatch(mid, argc_v, argv_v);
                self.builder.finalize();
                return Ok(());
            }
            Dispatch::Multi(mid) => {
                let mid_v = self.builder.ins().iconst(types::I64, mid);
                let r = self.call3(self.rt.multi_call, mid_v, argc_v, argv_v);
                self.builder.ins().return_(&[r]);
                self.builder.finalize();
                return Ok(());
            }
            Dispatch::None => {}
        }

        self.enter_frame(local_count);

        // Dispatch por aridade: cadeia de checagens argc.
        for m in methods {
            let k = m.params.len();
            let cond = if m.rest.is_some() {
                self.builder
                    .ins()
                    .icmp_imm(IntCC::SignedGreaterThanOrEqual, argc_v, k as i64)
            } else {
                self.builder.ins().icmp_imm(IntCC::Equal, argc_v, k as i64)
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
        // Nenhuma aridade correspondeu.
        let bad = self.builder.ins().iconst(types::I64, -1);
        self.call_void(self.rt.check_arity, &[argc_v, bad]); // sempre falha (exit)
        let z = self.konst(NIL);
        self.builder.ins().return_(&[z]);

        self.builder.finalize();
        Ok(())
    }

    /// Função-despacho: `impl = lookup(mid, type_key(argv[0]))`; encaminha argc/argv.
    fn gen_dispatch(&mut self, mid: i64, argc_v: CValue, argv_v: CValue) {
        let arg0 = self
            .builder
            .ins()
            .load(types::I64, MemFlags::trusted(), argv_v, 0);
        let key = self.call1(self.rt.type_key, arg0);
        let mid_v = self.builder.ins().iconst(types::I64, mid);
        let impl_v = self.call2(self.rt.lookup_method, mid_v, key);
        let is_nil = self.builder.ins().icmp_imm(IntCC::Equal, impl_v, NIL);
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

    /// Vincula os parâmetros (fixos + rest) e compila o corpo de uma aridade.
    /// O bloco atual já é o "matched"; termina com return (Val) ou recur (Diverged).
    fn gen_method_body(
        &mut self,
        m: &FnMethod,
        argc_v: CValue,
        argv_v: CValue,
    ) -> Result<(), Diagnostic> {
        let mut param_slots = Vec::new();
        for slot in 0..m.params.len() {
            let val =
                self.builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), argv_v, (slot * 8) as i32);
            self.bind_local(slot as u32, val);
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
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);
        self.enter_frame(local_count);
        for a in body {
            let (_, pushed) = self.operand(a)?;
            if pushed {
                self.gc_popn(1); // descarta resultado de topo (se Heap)
            }
        }
        self.leave_frame();
        let zero = self.builder.ins().iconst(types::I32, 0);
        self.builder.ins().return_(&[zero]);
        self.builder.finalize();
        Ok(())
    }

    fn konst(&mut self, tagged: i64) -> CValue {
        self.builder.ins().iconst(types::I64, tagged)
    }

    fn call1(&mut self, id: FuncId, a: CValue) -> CValue {
        let r = self.module.declare_func_in_func(id, self.builder.func);
        let c = self.builder.ins().call(r, &[a]);
        self.builder.inst_results(c)[0]
    }
    fn call2(&mut self, id: FuncId, a: CValue, b: CValue) -> CValue {
        let r = self.module.declare_func_in_func(id, self.builder.func);
        let c = self.builder.ins().call(r, &[a, b]);
        self.builder.inst_results(c)[0]
    }
    fn call0(&mut self, id: FuncId) -> CValue {
        let r = self.module.declare_func_in_func(id, self.builder.func);
        let c = self.builder.ins().call(r, &[]);
        self.builder.inst_results(c)[0]
    }
    fn call3(&mut self, id: FuncId, a: CValue, b: CValue, c: CValue) -> CValue {
        let r = self.module.declare_func_in_func(id, self.builder.func);
        let call = self.builder.ins().call(r, &[a, b, c]);
        self.builder.inst_results(call)[0]
    }
    fn call_void3(&mut self, id: FuncId, a: CValue, b: CValue, c: CValue) {
        let r = self.module.declare_func_in_func(id, self.builder.func);
        self.builder.ins().call(r, &[a, b, c]);
    }
    fn call_void4(&mut self, id: FuncId, a: CValue, b: CValue, c: CValue, d: CValue) {
        let r = self.module.declare_func_in_func(id, self.builder.func);
        self.builder.ins().call(r, &[a, b, c, d]);
    }
    /// Endereço (ponteiro de código) de uma função declarada.
    fn func_addr(&mut self, id: FuncId) -> CValue {
        let r = self.module.declare_func_in_func(id, self.builder.func);
        self.builder.ins().func_addr(self.ptr, r)
    }
    fn call_void(&mut self, id: FuncId, args: &[CValue]) {
        let r = self.module.declare_func_in_func(id, self.builder.func);
        self.builder.ins().call(r, args);
    }

    /// Avalia uma expressão em posição de operando (nunca diverge — o analyzer
    /// garante que `recur` só ocorre em posição de cauda).
    fn expr_val(&mut self, ast: &Ast) -> Result<CValue, Diagnostic> {
        match self.expr(ast)? {
            Flow::Val(v) => Ok(v),
            Flow::Diverged => Err(Diagnostic::error(
                "E0112",
                "recur em posição não-cauda (bug do compilador)",
            )),
        }
    }

    /// Invariante de rooting (ADR-0006 Fases 4-5): na saída `Flow::Val(v)`, `v`
    /// está no topo do shadow-stack **se e somente se** `expr_pushes(ast)` — isto
    /// é, valores `Heap` que não sejam um `Local` já rooteado. Imediatos e leituras
    /// de `Local` não vão à pilha (nunca precisam de novo root). Consumidores usam
    /// `operand`/`spill_arg`, que sabem se houve push. `Flow::Diverged` já terminou
    /// o bloco (recur/return) deixando o shadow-stack consistente para o alvo.
    fn expr(&mut self, ast: &Ast) -> Result<Flow, Diagnostic> {
        Ok(match ast {
            // Imediatos (Fase 4): nunca são ponteiros → não vão ao shadow-stack.
            Ast::Int(n) => {
                let tagged = (*n as i128) << 1 | 1;
                Flow::Val(self.konst(tagged as i64))
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
                // Local nunca precisa de novo root: imediato não é ponteiro, e Local
                // Heap já está rooteado no seu slot de frame (base+slot), estável
                // enquanto o valor lido é usado (rebind só em let-init/recur, nunca
                // no meio de um operando). Consumidores que precisam dele na pilha
                // (args por argv, coleções) usam `spill_arg`.
                Flow::Val(self.builder.use_var(var))
            }
            Ast::Do(stmts) => {
                if stmts.is_empty() {
                    return Ok(Flow::Val(self.konst(NIL)));
                }
                let last = stmts.len() - 1;
                for s in &stmts[..last] {
                    let (_, pushed) = self.operand(s)?; // descarta valor
                    if pushed {
                        self.gc_popn(1);
                    }
                }
                self.expr(&stmts[last])?
            }
            Ast::Let { slots, body } => {
                for (slot, init) in slots {
                    let (val, pushed) = self.operand(init)?;
                    let kind = self.kind_of(init, &HashMap::new());
                    self.bind_local_kind(*slot, val, kind); // escreve slot só se Heap
                    if pushed {
                        self.gc_popn(1); // remove o temp (já está no slot, se Heap)
                    }
                }
                self.expr(body)?
            }
            Ast::If(test, then, els) => self.gen_if(test, then, els)?,
            Ast::Loop { slots, body } => self.gen_loop(slots, body)?,
            Ast::Recur(args) => self.gen_recur(args)?,
            Ast::Call { callee, args } => {
                let v = self.gen_call(callee, args)?; // net-0; resultado não empurrado
                                                      // Resultado imediato (aritmética/comparação/nil) não precisa de root.
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

    /// Constrói um vetor persistente por `conj` sucessivos (net-0), rooteando o
    /// acumulador durante as alocações do trie.
    fn gen_vec(&mut self, items: &[Ast]) -> Result<CValue, Diagnostic> {
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

    /// sorted-set: parte do vazio e `conj` (genérico, dispatcha p/ árvore) cada
    /// item, rooteando o acumulador durante as alocações da LLRB.
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

    /// sorted-map: parte do vazio e `assoc` cada par (k v), rooteando o acumulador.
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

    /// Fast path inline de `(nth vec i)` (aridade 2): se `coll` for T_VEC e `i`
    /// um fixnum em [0,count), lê o tail/trie inline (sem chamada nem dispatch).
    /// Qualquer outra situação cai em `cljn_nth`, que preserva toda a semântica
    /// da ADR-0008 (nil, capability, sequência, erros). Não aloca → sem safepoint.
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

        // guard-1: coll é ponteiro alinhado não-nulo E idx é fixnum
        let low3 = self.builder.ins().band_imm(coll, 7);
        let aligned = self.builder.ins().icmp_imm(IntCC::Equal, low3, 0);
        let nonzero = self.builder.ins().icmp_imm(IntCC::NotEqual, coll, 0);
        let ptr_ok = self.builder.ins().band(aligned, nonzero);
        let idxlow = self.builder.ins().band_imm(idx, 1);
        let idx_fix = self.builder.ins().icmp_imm(IntCC::NotEqual, idxlow, 0);
        let pre = self.builder.ins().band(ptr_ok, idx_fix);
        self.builder.ins().brif(pre, type_b, &[], slow_b, &[]);

        // type_b: só aqui é seguro ler o byte de tipo (coll é ponteiro)
        self.builder.switch_to_block(type_b);
        self.builder.seal_block(type_b);
        let ty = self
            .builder
            .ins()
            .load(types::I8, MemFlags::trusted(), coll, 0);
        let is_vec = self.builder.ins().icmp_imm(IntCC::Equal, ty, T_VEC);
        self.builder.ins().brif(is_vec, vec_b, &[], slow_b, &[]);

        // vec_b: desempacota i e checa limites (fora → slow, que lança p/ aridade 2)
        self.builder.switch_to_block(vec_b);
        self.builder.seal_block(vec_b);
        let i = self.builder.ins().sshr_imm(idx, 1);
        let count = self
            .builder
            .ins()
            .load(types::I64, MemFlags::trusted(), coll, PV_COUNT);
        let neg = self.builder.ins().icmp_imm(IntCC::SignedLessThan, i, 0);
        let ge = self
            .builder
            .ins()
            .icmp(IntCC::SignedGreaterThanOrEqual, i, count);
        let oob = self.builder.ins().bor(neg, ge);
        self.builder.ins().brif(oob, slow_b, &[], inb_b, &[]);

        // inb_b: tail vs trie
        self.builder.switch_to_block(inb_b);
        self.builder.seal_block(inb_b);
        let tail_len = self
            .builder
            .ins()
            .load(types::I64, MemFlags::trusted(), coll, PV_TAILLEN);
        let tailoff = self.builder.ins().isub(count, tail_len);
        let in_tail = self
            .builder
            .ins()
            .icmp(IntCC::SignedGreaterThanOrEqual, i, tailoff);
        self.builder.ins().brif(in_tail, tail_b, &[], trie_b, &[]);

        // tail_b: tail->slots[i - tailoff]
        self.builder.switch_to_block(tail_b);
        self.builder.seal_block(tail_b);
        let tail = self
            .builder
            .ins()
            .load(types::I64, MemFlags::trusted(), coll, PV_TAIL);
        let ti = self.builder.ins().isub(i, tailoff);
        let toff = self.builder.ins().imul_imm(ti, 8);
        let taddr = self.builder.ins().iadd(tail, toff);
        let tres = self
            .builder
            .ins()
            .load(types::I64, MemFlags::trusted(), taddr, VNODE_SLOTS);
        self.builder.ins().jump(merge, &[tres.into()]);

        // trie_b: node=root, level=shift
        self.builder.switch_to_block(trie_b);
        self.builder.seal_block(trie_b);
        let root = self
            .builder
            .ins()
            .load(types::I64, MemFlags::trusted(), coll, PV_ROOT);
        let shift = self
            .builder
            .ins()
            .load(types::I64, MemFlags::trusted(), coll, PV_SHIFT);
        self.builder
            .ins()
            .jump(loop_b, &[root.into(), shift.into()]);

        // loop_b(node, level): level>0 ? descend : leaf
        self.builder.switch_to_block(loop_b);
        let node = self.builder.block_params(loop_b)[0];
        let level = self.builder.block_params(loop_b)[1];
        let more = self
            .builder
            .ins()
            .icmp_imm(IntCC::SignedGreaterThan, level, 0);
        self.builder.ins().brif(more, descend_b, &[], leaf_b, &[]);

        // descend_b: node = node->slots[(i>>level)&31]; level -= 5
        self.builder.switch_to_block(descend_b);
        self.builder.seal_block(descend_b);
        let sh = self.builder.ins().sshr(i, level);
        let sub = self.builder.ins().band_imm(sh, 31);
        let soff = self.builder.ins().imul_imm(sub, 8);
        let saddr = self.builder.ins().iadd(node, soff);
        let child = self
            .builder
            .ins()
            .load(types::I64, MemFlags::trusted(), saddr, VNODE_SLOTS);
        let level2 = self.builder.ins().iadd_imm(level, -5);
        self.builder
            .ins()
            .jump(loop_b, &[child.into(), level2.into()]);
        self.builder.seal_block(loop_b); // preds: trie_b, descend_b

        // leaf_b: node->slots[i&31]
        self.builder.switch_to_block(leaf_b);
        self.builder.seal_block(leaf_b);
        let slot = self.builder.ins().band_imm(i, 31);
        let loff = self.builder.ins().imul_imm(slot, 8);
        let laddr = self.builder.ins().iadd(node, loff);
        let lres = self
            .builder
            .ins()
            .load(types::I64, MemFlags::trusted(), laddr, VNODE_SLOTS);
        self.builder.ins().jump(merge, &[lres.into()]);

        // slow_b: delega ao runtime (semântica completa e erros)
        self.builder.switch_to_block(slow_b);
        self.builder.seal_block(slow_b);
        let sres = self.call2(self.rt.p_nth, coll, idx);
        self.builder.ins().jump(merge, &[sres.into()]);

        // merge
        self.builder.switch_to_block(merge);
        self.builder.seal_block(merge);
        let res = self.builder.block_params(merge)[0];
        self.gc_popn(coll_pushed as usize + idx_pushed as usize);
        Ok(res)
    }

    /// `assoc` variádico (ADR-0008): avalia TODOS os args antes da dobra e então
    /// dobra os pares da esquerda p/ direita sobre AssocOne (`cljn_assoc`),
    /// mantendo o acumulador rooteado a cada passo. args = [coll, k, v, k, v...].
    fn gen_assoc(&mut self, args: &[Ast]) -> Result<CValue, Diagnostic> {
        // Avalia todos os args antes da dobra; só operandos Heap vão à shadow
        // stack (imediatos — índice fixnum, `false` etc. — nunca precisam de root).
        // Pares Heap ainda não consumidos permanecem rooteados durante os passos
        // anteriores. O acumulador tem seu próprio root, atualizado a cada passo.
        let mut vals = Vec::with_capacity(args.len());
        let mut pushed = 0usize;
        for a in args {
            let (v, p) = self.operand(a)?;
            pushed += p as usize;
            vals.push(v);
        }
        let mut acc = vals[0];
        self.gc_push_val(acc); // root do acumulador (topo)
        let mut i = 1;
        while i + 1 < vals.len() {
            acc = self.call3(self.rt.p_assoc, acc, vals[i], vals[i + 1]);
            self.gc_popn(1); // remove acc antigo
            self.gc_push_val(acc); // novo acc
            i += 2;
        }
        self.gc_popn(1); // acc
        self.gc_popn(pushed); // operandos Heap
        Ok(acc)
    }

    /// Materializa uma keyword a partir do blob de string (sem empurrar).
    fn make_kw_value(&mut self, name: &str) -> CValue {
        let (data_id, len) = self.str_data[name];
        let gv = self.module.declare_data_in_func(data_id, self.builder.func);
        let p = self.builder.ins().symbol_value(self.ptr, gv);
        let len_v = self.builder.ins().iconst(types::I64, len as i64);
        self.call2(self.rt.kw, p, len_v)
    }

    /// Constrói um record (net-0): mapa dos campos + nome de tipo → cljn_make_record.
    fn gen_make_record(
        &mut self,
        type_name: &str,
        fields: &[(String, Ast)],
    ) -> Result<CValue, Diagnostic> {
        let pairs: Vec<(Ast, Ast)> = fields
            .iter()
            .map(|(fname, val)| (Ast::Keyword(fname.clone()), val.clone()))
            .collect();
        let map_val = self.gen_map(&pairs)?; // net-0
        self.gc_push_val(map_val); // rooteia o mapa durante o alloc do keyword/record
        let tn = self.make_kw_value(type_name);
        self.gc_push_val(tn);
        let rec = self.call2(self.rt.make_record, tn, map_val);
        self.gc_popn(2);
        Ok(rec)
    }

    /// Cria uma closure: avalia capturas, aloca o Fn e preenche `freev`.
    fn gen_make_fn(
        &mut self,
        lambda: &str,
        arity: usize,
        captures: &[Ast],
    ) -> Result<Flow, Diagnostic> {
        let mut cap_vals = Vec::with_capacity(captures.len());
        for c in captures {
            cap_vals.push(self.spill_arg(c)?); // +1 cada (rooteados durante o alloc)
        }
        let (id, _) = self.fn_ids[lambda];
        let code = self.func_addr(id);
        let ar = self.builder.ins().iconst(types::I64, arity as i64);
        let nfree = self.builder.ins().iconst(types::I64, captures.len() as i64);
        let fnv = self.call3(self.rt.make_fn, code, ar, nfree);
        // Capturas já estão dentro do Fn a partir de agora (set_free não aloca):
        self.gc_popn(captures.len()); // remove os temps de captura
        self.gc_push_val(fnv); // rooteia o Fn
        for (i, cv) in cap_vals.iter().enumerate() {
            let idx = self.builder.ins().iconst(types::I64, i as i64);
            self.call_void3(self.rt.set_free, fnv, idx, *cv);
        }
        Ok(Flow::Val(fnv))
    }

    /// Chamada indireta de um valor-função (net-0 no shadow-stack).
    fn gen_call_value(&mut self, f: &Ast, args: &[Ast]) -> Result<CValue, Diagnostic> {
        let f_val = self.spill_arg(f)?; // +1 (f fica abaixo dos args)
        for a in args {
            self.spill_arg(a)?; // +1 cada (args por argv → sempre na pilha)
        }
        self.call_void(self.rt.check_fn, &[f_val]);
        let argc_v = self.builder.ins().iconst(types::I64, args.len() as i64);
        let argv_ptr = self.call1(self.rt.argv, argc_v); // topo = os args (f está abaixo)
        let code = self.call1(self.rt.fn_code, f_val);
        // Assinatura uniforme da entrada: (self, argc, argv) -> i64.
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
        self.gc_popn(1 + args.len()); // f + args
        Ok(r)
    }

    /// `(apply f fixed... coll)`: net-0. Empilha f, os fixos, e espalha `coll`.
    fn gen_apply(&mut self, f: &Ast, fixed: &[Ast], coll: &Ast) -> Result<CValue, Diagnostic> {
        let f_val = self.spill_arg(f)?; // shadow: [f]
        for a in fixed {
            self.spill_arg(a)?; // [f, fixed...]
        }
        let coll_val = self.spill_arg(coll)?; // [f, fixed.., coll]
        self.gc_popn(1); // remove o temp de coll (spread não aloca; coll SSA segue válido)
        let n_fixed = self.builder.ins().iconst(types::I64, fixed.len() as i64);
        // Empurra os elementos de coll no topo; devolve argc total (fixos + extras).
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
        // Limpa: f (1) + total (fixos + elementos).
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
            self.builder.ins().icmp_imm(IntCC::NotEqual, truth, 0)
        } else {
            // Teste imediato: falsy só se FALSEV ou NIL → compara inline (sem call).
            let nf = self
                .builder
                .ins()
                .icmp_imm(IntCC::NotEqual, test_val, FALSEV);
            let nn = self.builder.ins().icmp_imm(IntCC::NotEqual, test_val, NIL);
            self.builder.ins().band(nf, nn)
        };

        let then_b = self.builder.create_block();
        let else_b = self.builder.create_block();
        self.builder.ins().brif(cond, then_b, &[], else_b, &[]);

        // Merge criado sob demanda (apenas se algum ramo alcança fall-through).
        let mut merge: Option<Block> = None;

        self.builder.switch_to_block(then_b);
        self.builder.seal_block(then_b);
        if let Flow::Val(tv) = self.expr(then)? {
            // Reconcilia a profundidade da pilha: se o merge é Heap mas este ramo
            // não empurrou (imediato ou Local), derrama para igualar os ramos.
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
        // Kinds dos slots por ponto fixo: um slot é Imm só se init e todo recur-arg
        // naquela posição forem Imm. Slots Imm não escrevem root (var apenas).
        let slot_kinds = self.loop_slot_kinds(slots, body, &HashMap::new());
        let mut slot_ids = Vec::with_capacity(slots.len());
        for (slot, init) in slots {
            let (v0, pushed) = self.operand(init)?;
            let k = *slot_kinds.get(slot).unwrap_or(&VKind::Heap);
            self.bind_local_kind(*slot, v0, k); // escreve slot só se Heap
            if pushed {
                self.gc_popn(1);
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
        // Avalia todos os argumentos antes de religar (evita clobber). Heap → +1.
        let mut vals = Vec::with_capacity(args.len());
        let mut pushed = 0usize;
        for a in args {
            let (v, p) = self.operand(a)?;
            pushed += p as usize;
            vals.push(v);
        }
        for (slot, val) in target.slots.iter().zip(vals) {
            // Kind do slot fixado pelo loop; um slot Imm nunca recebe arg Heap.
            let k = *self.kinds.get(slot).unwrap_or(&VKind::Heap);
            self.bind_local_kind(*slot, val, k);
        }
        self.gc_popn(pushed); // remove só os temps Heap
        self.builder.ins().jump(target.header, &[]);
        Ok(Flow::Diverged)
    }

    /// Chamada de função ou primitiva: avalia args (empurra temps), chama,
    /// retira os temps e devolve o resultado **sem** empurrá-lo (quem chama, em
    /// `expr`, empurra). Net-0 no shadow-stack.
    fn gen_call(&mut self, callee: &Callee, args: &[Ast]) -> Result<CValue, Diagnostic> {
        match callee {
            Callee::Prim(p) => self.gen_prim(*p, args),
            Callee::Fn(name) => {
                let (id, _) = self.fn_ids[name];
                // Empilha os args no shadow-stack; argv aponta para eles (imediatos derramados).
                for a in args {
                    self.spill_arg(a)?; // +1 cada
                }
                let argc_v = self.builder.ins().iconst(types::I64, args.len() as i64);
                let argv_ptr = self.call1(self.rt.argv, argc_v);
                let nil = self.konst(NIL); // self = NIL (fn de topo ignora capturas)
                let fref = self.module.declare_func_in_func(id, self.builder.func);
                let call = self.builder.ins().call(fref, &[nil, argc_v, argv_ptr]);
                let r = self.builder.inst_results(call)[0];
                self.gc_popn(args.len());
                Ok(r)
            }
        }
    }

    // -- fast paths de fixnum (ADR-0006) ---------------------------------
    /// Guarda "ambos são fixnum": `(a & b & 1) != 0`.
    fn fix_both_guard(&mut self, a: CValue, b: CValue) -> CValue {
        let ab = self.builder.ins().band(a, b);
        let m = self.builder.ins().band_imm(ab, 1);
        self.builder.ins().icmp_imm(IntCC::NotEqual, m, 0)
    }
    fn fix_guard(&mut self, a: CValue) -> CValue {
        let m = self.builder.ins().band_imm(a, 1);
        self.builder.ins().icmp_imm(IntCC::NotEqual, m, 0)
    }
    fn fix_retag(&mut self, raw: CValue) -> CValue {
        let sh = self.builder.ins().ishl_imm(raw, 1);
        self.builder.ins().bor_imm(sh, 1)
    }
    /// `raw` está em [FIX_MIN, FIX_MAX]?
    fn fix_in_range(&mut self, raw: CValue) -> CValue {
        let lo = self
            .builder
            .ins()
            .icmp_imm(IntCC::SignedGreaterThanOrEqual, raw, FIX_MIN);
        let hi = self
            .builder
            .ins()
            .icmp_imm(IntCC::SignedLessThanOrEqual, raw, FIX_MAX);
        self.builder.ins().band(lo, hi)
    }

    /// `+`/`-` inline com guard + range-check + retag; slow path = runtime.
    fn gen_fix_arith(&mut self, a: CValue, b: CValue, add: bool, slow: FuncId) -> CValue {
        let both = self.fix_both_guard(a, b);
        let fast_b = self.builder.create_block();
        let slow_b = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        self.builder.ins().brif(both, fast_b, &[], slow_b, &[]);

        self.builder.switch_to_block(fast_b);
        self.builder.seal_block(fast_b);
        let ar = self.builder.ins().sshr_imm(a, 1);
        let br = self.builder.ins().sshr_imm(b, 1);
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

    /// `*` inline: guard, unbox, imul + smulhi p/ detectar overflow de i64,
    /// range-check de fixnum, retag; slow path = runtime.
    fn gen_fix_mul(&mut self, a: CValue, b: CValue, slow: FuncId) -> CValue {
        let both = self.fix_both_guard(a, b);
        let fast_b = self.builder.create_block();
        let slow_b = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        self.builder.ins().brif(both, fast_b, &[], slow_b, &[]);

        self.builder.switch_to_block(fast_b);
        self.builder.seal_block(fast_b);
        let ar = self.builder.ins().sshr_imm(a, 1);
        let br = self.builder.ins().sshr_imm(b, 1);
        let lo = self.builder.ins().imul(ar, br);
        let hi = self.builder.ins().smulhi(ar, br);
        let exp = self.builder.ins().sshr_imm(lo, 63); // extensão de sinal de lo
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

    /// `inc`/`dec` inline (`delta` = +1/-1); slow path = runtime.
    fn gen_fix_unop(&mut self, a: CValue, delta: i64, slow: FuncId) -> CValue {
        let g = self.fix_guard(a);
        let fast_b = self.builder.create_block();
        let slow_b = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        self.builder.ins().brif(g, fast_b, &[], slow_b, &[]);

        self.builder.switch_to_block(fast_b);
        self.builder.seal_block(fast_b);
        let ar = self.builder.ins().sshr_imm(a, 1);
        let rr = self.builder.ins().iadd_imm(ar, delta);
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

    /// `quot`/`mod` inline. Guard de tag + divisor≠0 desviam ao slow path (que
    /// preserva "divisão por zero"). `quot`: range-check p/ o caso `-2^62 / -1`.
    /// `mod`: ajuste de sinal (mod floored de Clojure), resultado sempre em range.
    fn gen_fix_div(&mut self, a: CValue, b: CValue, is_quot: bool, slow: FuncId) -> CValue {
        let both = self.fix_both_guard(a, b);
        let fast_b = self.builder.create_block();
        let slow_b = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        self.builder.ins().brif(both, fast_b, &[], slow_b, &[]);

        self.builder.switch_to_block(fast_b);
        self.builder.seal_block(fast_b);
        let ar = self.builder.ins().sshr_imm(a, 1);
        let br = self.builder.ins().sshr_imm(b, 1);
        let bz = self.builder.ins().icmp_imm(IntCC::Equal, br, 0);
        let cont_b = self.builder.create_block();
        self.builder.ins().brif(bz, slow_b, &[], cont_b, &[]); // divisor 0 → slow
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
            // ajuste floored: se r!=0 && sinal(r)!=sinal(br) então r+br
            let rnz = self.builder.ins().icmp_imm(IntCC::NotEqual, r, 0);
            let xr = self.builder.ins().bxor(r, br);
            let diff = self.builder.ins().icmp_imm(IntCC::SignedLessThan, xr, 0);
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

    /// `< <= > >=` inline: guard, unbox, icmp, select(TRUE/FALSE); slow = runtime.
    fn gen_fix_cmp(&mut self, a: CValue, b: CValue, cc: IntCC, slow: FuncId) -> CValue {
        let both = self.fix_both_guard(a, b);
        let fast_b = self.builder.create_block();
        let slow_b = self.builder.create_block();
        let merge = self.builder.create_block();
        self.builder.append_block_param(merge, types::I64);
        self.builder.ins().brif(both, fast_b, &[], slow_b, &[]);

        self.builder.switch_to_block(fast_b);
        self.builder.seal_block(fast_b);
        let ar = self.builder.ins().sshr_imm(a, 1);
        let br = self.builder.ins().sshr_imm(b, 1);
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

    /// Fold de `+`/`-` com fast path por par.
    fn fold_fix(&mut self, args: &[Ast], add: bool, slow: FuncId) -> Result<CValue, Diagnostic> {
        let mut vals = Vec::with_capacity(args.len());
        let mut pushed = 0usize;
        for a in args {
            let (v, p) = self.operand(a)?;
            pushed += p as usize;
            vals.push(v);
        }
        let mut acc = vals[0];
        for v in &vals[1..] {
            acc = self.gen_fix_arith(acc, *v, add, slow);
        }
        self.gc_popn(pushed);
        Ok(acc)
    }
    fn fix_cmp2(&mut self, args: &[Ast], cc: IntCC, slow: FuncId) -> Result<CValue, Diagnostic> {
        let (a, pa) = self.operand(&args[0])?;
        let (b, pb) = self.operand(&args[1])?;
        let r = self.gen_fix_cmp(a, b, cc, slow);
        self.gc_popn(pa as usize + pb as usize);
        Ok(r)
    }
    fn fix_una(&mut self, args: &[Ast], delta: i64, slow: FuncId) -> Result<CValue, Diagnostic> {
        let (a, pa) = self.operand(&args[0])?;
        let r = self.gen_fix_unop(a, delta, slow);
        self.gc_popn(pa as usize);
        Ok(r)
    }

    fn gen_prim(&mut self, prim: Prim, args: &[Ast]) -> Result<CValue, Diagnostic> {
        match prim {
            Prim::Println | Prim::Print => self.gen_print(prim, args),
            Prim::Str => self.gen_str(args),
            Prim::List => self.gen_list(args),
            // aritmética com fast path de fixnum (ADR-0006)
            Prim::Add => self.fold_fix(args, true, self.rt.add),
            Prim::Sub => {
                if args.len() == 1 {
                    let (v, pv) = self.operand(&args[0])?;
                    let zero = self.konst(1); // MK_FIX(0) == 1 (imediato, sem root)
                    let r = self.call2(self.rt.sub, zero, v);
                    self.gc_popn(pv as usize);
                    Ok(r)
                } else {
                    self.fold_fix(args, false, self.rt.sub)
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
                    acc = self.gen_fix_mul(acc, *v, self.rt.mul);
                }
                self.gc_popn(pushed);
                Ok(acc)
            }
            // comparações com fast path de fixnum
            Prim::Lt => self.fix_cmp2(args, IntCC::SignedLessThan, self.rt.lt),
            Prim::Le => self.fix_cmp2(args, IntCC::SignedLessThanOrEqual, self.rt.le),
            Prim::Gt => self.fix_cmp2(args, IntCC::SignedGreaterThan, self.rt.gt),
            Prim::Ge => self.fix_cmp2(args, IntCC::SignedGreaterThanOrEqual, self.rt.ge),
            // divisão inteira com fast path de fixnum
            Prim::Quot => {
                let (a, pa) = self.operand(&args[0])?;
                let (b, pb) = self.operand(&args[1])?;
                let r = self.gen_fix_div(a, b, true, self.rt.quot);
                self.gc_popn(pa as usize + pb as usize);
                Ok(r)
            }
            Prim::Mod => {
                let (a, pa) = self.operand(&args[0])?;
                let (b, pb) = self.operand(&args[1])?;
                let r = self.gen_fix_div(a, b, false, self.rt.mod_);
                self.gc_popn(pa as usize + pb as usize);
                Ok(r)
            }
            Prim::Eq => self.bin(self.rt.eq, args),
            Prim::Cons => self.bin(self.rt.cons, args),
            // unárias
            Prim::Inc => self.fix_una(args, 1, self.rt.inc),
            Prim::Dec => self.fix_una(args, -1, self.rt.dec),
            Prim::Not => self.una(self.rt.not_, args),
            Prim::NilP => self.una(self.rt.nilp, args),
            Prim::EmptyP => self.una(self.rt.emptyp, args),
            Prim::First => self.una(self.rt.first, args),
            Prim::Rest => self.una(self.rt.rest, args),
            Prim::Count => self.una(self.rt.count, args),
            // coleções
            Prim::Get => self.bin(self.rt.p_get, args),
            Prim::Nth => {
                if args.len() == 2 {
                    self.gen_nth_fast(args) // fast path inline p/ vetor (ADR-0006/0008)
                } else {
                    self.tern(self.rt.p_nth_or, args) // aridade 3: not-found
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
                // (hash-map k v k v ...) → pares
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
            Prim::Transient => self.una(self.rt.transient, args),
            Prim::PersistentBang => self.una(self.rt.persistent_bang, args),
            Prim::ConjBang => self.bin(self.rt.conj_bang, args),
            Prim::AssocBang => self.tern(self.rt.assoc_bang, args),
            Prim::DissocBang => self.bin(self.rt.dissoc_bang, args),
        }
    }

    /// Primitiva ternária (net-0). Só operandos Heap são rooteados/retirados.
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
        for (i, a) in args.iter().enumerate() {
            if i > 0 {
                self.call_void(self.rt.print_space, &[]);
            }
            let v = self.spill_arg(a)?; // +1 (imediatos derramados; rooteado no print)
            self.call_void(self.rt.print, &[v]);
        }
        self.gc_popn(args.len());
        if matches!(prim, Prim::Println) {
            self.call_void(self.rt.print_newline, &[]);
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
            vals.push(self.spill_arg(a)?); // n temps (imediatos derramados)
        }
        let mut acc = self.call1(self.rt.to_str, vals[0]);
        self.gc_push_val(acc); // acc temp
        for v in &vals[1..] {
            let s = self.call1(self.rt.to_str, *v);
            self.gc_push_val(s); // s temp
            acc = self.call2(self.rt.str_concat, acc, s);
            self.gc_popn(2); // remove s e acc antigo
            self.gc_push_val(acc); // novo acc
        }
        self.gc_popn(args.len() + 1); // remove args + acc
        Ok(acc)
    }

    fn gen_list(&mut self, args: &[Ast]) -> Result<CValue, Diagnostic> {
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            vals.push(self.spill_arg(a)?); // n temps (imediatos derramados)
        }
        let mut acc = self.call0(self.rt.empty);
        self.gc_push_val(acc); // acc temp
        for v in vals.iter().rev() {
            acc = self.call2(self.rt.cons, *v, acc); // v e acc rooteados
            self.gc_popn(1); // remove acc antigo
            self.gc_push_val(acc); // novo acc
        }
        self.gc_popn(args.len() + 1); // remove args + acc
        Ok(acc)
    }
}

fn collect_strings(ast: &Ast, out: &mut Vec<String>) {
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
        _ => {}
    }
}

fn single(msg: impl Into<String>) -> Diagnostics {
    Diagnostic::error("E0120", msg).into()
}
fn single_d(msg: impl Into<String>) -> Diagnostic {
    Diagnostic::error("E0120", msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_runtime_is_the_ordered_concatenation_of_unique_modules() {
        let mut concatenated = String::new();
        let mut names = std::collections::BTreeSet::new();
        for (name, source) in RUNTIME_MODULES {
            assert!(names.insert(*name), "módulo duplicado: {name}");
            assert!(!source.is_empty(), "módulo vazio: {name}");
            concatenated.push_str(source);
        }
        assert_eq!(RUNTIME_C, concatenated);
    }

    #[test]
    fn collects_strings_from_every_nested_ast_container() {
        let ast = Ast::Do(vec![
            Ast::Str("plain".into()),
            Ast::Keyword("keyword".into()),
            Ast::VecLit(vec![Ast::Str("vector".into())]),
            Ast::SetLit(vec![Ast::Str("set".into())]),
            Ast::MapLit(vec![(Ast::Str("key".into()), Ast::Str("value".into()))]),
            Ast::If(
                Box::new(Ast::Str("test".into())),
                Box::new(Ast::Str("then".into())),
                Box::new(Ast::Str("else".into())),
            ),
            Ast::Recur(vec![Ast::Str("recur".into())]),
            Ast::MakeFn {
                lambda: "lambda".into(),
                arity: 1,
                captures: vec![Ast::Str("capture".into())],
            },
            Ast::CallValue {
                f: Box::new(Ast::Str("callable".into())),
                args: vec![Ast::Str("argument".into())],
            },
            Ast::Apply {
                f: Box::new(Ast::Str("apply".into())),
                fixed: vec![Ast::Str("fixed".into())],
                coll: Box::new(Ast::Str("collection".into())),
            },
            Ast::MakeRecord {
                type_name: "Point".into(),
                fields: vec![("x".into(), Ast::Str("field-value".into()))],
            },
            Ast::RegisterMethod {
                method_id: 1,
                key: Box::new(Ast::Str("dispatch-key".into())),
                impl_fn: Box::new(Ast::Str("implementation".into())),
            },
            Ast::Loop {
                slots: vec![(0, Ast::Str("loop-init".into()))],
                body: Box::new(Ast::Str("loop-body".into())),
            },
            Ast::Let {
                slots: vec![(1, Ast::Str("let-init".into()))],
                body: Box::new(Ast::Str("let-body".into())),
            },
            Ast::Call {
                callee: Callee::Prim(Prim::Print),
                args: vec![Ast::Str("call-arg".into())],
            },
        ]);

        let mut strings = Vec::new();
        collect_strings(&ast, &mut strings);
        assert!(strings.contains(&"plain".to_string()));
        assert!(strings.contains(&"keyword".to_string()));
        assert!(strings.contains(&"Point".to_string()));
        assert!(strings.contains(&"x".to_string()));
        assert!(strings.contains(&"implementation".to_string()));
        assert!(strings.contains(&"call-arg".to_string()));
        assert_eq!(strings.len(), 26);
    }

    #[test]
    fn compiles_minimal_program_to_nonempty_object() {
        let program = Program {
            functions: vec![],
            main_body: vec![Ast::Int(42), Ast::Str("done".into())],
            main_local_count: 0,
        };
        let object = compile_object(&program).expect("minimal program should compile");
        assert!(object.len() > 100);
        assert!(object.iter().any(|byte| *byte != 0));
    }

    #[test]
    fn compiles_with_every_supported_optimization_level() {
        let program = Program {
            functions: vec![],
            main_body: vec![Ast::Int(42)],
            main_local_count: 0,
        };

        for optimization_level in [
            OptimizationLevel::None,
            OptimizationLevel::Speed,
            OptimizationLevel::SpeedAndSize,
        ] {
            let object =
                compile_object_with_options(&program, CodegenOptions { optimization_level })
                    .expect("all supported optimization levels should compile");
            assert!(object.len() > 100);
        }
    }

    #[test]
    fn parses_public_optimization_level_names() {
        assert_eq!(
            "none".parse::<OptimizationLevel>(),
            Ok(OptimizationLevel::None)
        );
        assert_eq!(
            "speed".parse::<OptimizationLevel>(),
            Ok(OptimizationLevel::Speed)
        );
        assert_eq!(
            "speed-and-size".parse::<OptimizationLevel>(),
            Ok(OptimizationLevel::SpeedAndSize)
        );
        assert!("fast".parse::<OptimizationLevel>().is_err());
    }

    #[test]
    fn default_optimization_remains_unoptimized_until_the_speed_gate_passes() {
        assert_eq!(CodegenOptions::default(), CodegenOptions::unoptimized());
        assert_eq!(
            CodegenOptions::optimized_for_speed().optimization_level,
            OptimizationLevel::Speed
        );
    }

    #[test]
    fn codegen_diagnostics_use_stable_code() {
        let one = single("failure");
        assert_eq!(one.items[0].code, "E0120");
        assert_eq!(one.items[0].message, "failure");
        assert_eq!(single_d("other").code, "E0120");
    }
}
