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

use clojure_analyzer::{Ast, Callee, Prim, Program};
use clojure_diagnostics::{Diagnostic, Diagnostics};
use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{types, AbiParam, Block, InstBuilder, Value as CValue};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::{isa, Context};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{default_libcall_names, DataDescription, DataId, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use std::collections::HashMap;
use target_lexicon::Triple;

/// Fonte C do runtime, compilada junto ao objeto no passo de link.
pub const RUNTIME_C: &str = include_str!("../runtime.c");

// Constantes de valor tagged (devem casar com runtime.c).
const NIL: i64 = 2;
const FALSEV: i64 = 6;
const TRUEV: i64 = 10;

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
    gc_enter: FuncId,    // (i64)->i64  reserva slots; devolve base
    gc_leave: FuncId,    // (i64)->void restaura sp=base
    gc_push: FuncId,     // (i64)->void empurra temporário
    gc_popn: FuncId,     // (i64)->void retira n temporários
    gc_set: FuncId,      // (i64,i64)->void escreve slot de local
}

/// Compila o programa para bytes de um objeto nativo da plataforma host.
pub fn compile_object(program: &Program) -> Result<Vec<u8>, Diagnostics> {
    let mut flags = settings::builder();
    flags.set("is_pic", "true").unwrap();
    flags.set("opt_level", "none").unwrap();
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
        collect_strings(&f.body, &mut strings);
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
        let bytes = if s.is_empty() { vec![0u8] } else { s.clone().into_bytes() };
        d.define(bytes.into_boxed_slice());
        module.define_data(id, &d).map_err(|e| single(format!("define_data: {e}")))?;
        str_data.insert(s.clone(), (id, s.len()));
    }

    // Declara funções do usuário (para recursão/forward-ref).
    let mut fn_ids: HashMap<String, (FuncId, usize)> = HashMap::new();
    for f in &program.functions {
        let mut sig = module.make_signature();
        for _ in 0..f.params.len() {
            sig.params.push(AbiParam::new(types::I64));
        }
        sig.returns.push(AbiParam::new(types::I64));
        let id = module
            .declare_function(&f.name, Linkage::Local, &sig)
            .map_err(|e| single(format!("declare_function {}: {e}", f.name)))?;
        fn_ids.insert(f.name.clone(), (id, f.params.len()));
    }

    let mut diags = Diagnostics::new();

    for f in &program.functions {
        let (id, _) = fn_ids[&f.name];
        let mut ctx = Context::new();
        ctx.func.signature = module.declarations().get_function_decl(id).signature.clone();
        let mut fbctx = FunctionBuilderContext::new();
        let res = {
            let mut fg = FnGen::new(&mut module, &mut ctx.func, &mut fbctx, ptr, &runtime, &fn_ids, &str_data);
            fg.build_function(&f.params, &f.body, f.local_count)
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
            let mut fg = FnGen::new(&mut module, &mut ctx.func, &mut fbctx, ptr, &runtime, &fn_ids, &str_data);
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
    product.emit().map_err(|e| single(format!("emit do objeto: {e}")))
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

    let mut str_from_sig = m.make_signature();
    str_from_sig.params.push(AbiParam::new(ptr));
    str_from_sig.params.push(AbiParam::new(types::I64));
    str_from_sig.returns.push(AbiParam::new(types::I64));
    let str_from = m.declare_function("cljn_str_from", Linkage::Import, &str_from_sig).unwrap();

    let mut empty_sig = m.make_signature();
    empty_sig.returns.push(AbiParam::new(types::I64));
    let empty = m.declare_function("cljn_empty", Linkage::Import, &empty_sig).unwrap();

    let mut truthy_sig = m.make_signature();
    truthy_sig.params.push(AbiParam::new(types::I64));
    truthy_sig.returns.push(AbiParam::new(types::I32));
    let truthy = m.declare_function("cljn_truthy", Linkage::Import, &truthy_sig).unwrap();

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
        gc_push: voidfn(m, "cljn_gc_push", true),
        gc_popn: voidfn(m, "cljn_gc_popn", true),
        gc_set: bin_void(m, "cljn_gc_set"),
    }
}

/// Resultado de compilar uma expressão: produz um valor (fall-through) ou
/// diverge (o bloco já foi terminado por `recur`/`return`).
#[derive(Clone, Copy)]
enum Flow {
    Val(CValue),
    Diverged,
}

/// Alvo de `recur`: bloco-cabeçalho de um loop/fn + variáveis e slots a religar.
#[derive(Clone)]
struct RecurTarget {
    header: Block,
    vars: Vec<Variable>,
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
    recur_targets: Vec<RecurTarget>,
    /// Base do frame no shadow-stack (i64), definida na entrada da função.
    frame_base: Option<Variable>,
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
            recur_targets: Vec::new(),
            frame_base: None,
        }
    }

    fn new_var(&mut self, slot: u32) -> Variable {
        let v = self.builder.declare_var(types::I64);
        self.vars.insert(slot, v);
        v
    }

    // -- shadow-stack de roots (ABI de GC) --------------------------------
    fn gc_push_val(&mut self, v: CValue) {
        self.call_void(self.rt.gc_push, &[v]);
    }
    fn gc_popn(&mut self, n: usize) {
        if n > 0 {
            let k = self.builder.ins().iconst(types::I64, n as i64);
            self.call_void(self.rt.gc_popn, &[k]);
        }
    }
    /// Escreve o slot de root de um local: shadow[base + slot] = v.
    fn gc_set_local(&mut self, slot: u32, v: CValue) {
        let base_var = self.frame_base.expect("frame_base definido");
        let base = self.builder.use_var(base_var);
        let idx = self.builder.ins().iadd_imm(base, slot as i64);
        self.call_void(self.rt.gc_set, &[idx, v]);
    }
    /// Vincula um local: define a variável Cranelift e espelha no shadow-stack.
    fn bind_local(&mut self, slot: u32, v: CValue) {
        let var = match self.vars.get(&slot) {
            Some(v) => *v,
            None => self.new_var(slot),
        };
        self.builder.def_var(var, v);
        self.gc_set_local(slot, v);
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

    fn build_function(mut self, params: &[String], body: &Ast, local_count: u32) -> Result<(), Diagnostic> {
        let entry = self.builder.create_block();
        self.builder.append_block_params_for_function_params(entry);
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);
        let param_vals: Vec<CValue> = self.builder.block_params(entry).to_vec();

        self.enter_frame(local_count);

        let mut param_vars = Vec::with_capacity(param_vals.len());
        let mut param_slots = Vec::with_capacity(param_vals.len());
        for (slot, val) in param_vals.iter().enumerate() {
            self.bind_local(slot as u32, *val); // define var + espelha shadow slot
            param_vars.push(self.vars[&(slot as u32)]);
            param_slots.push(slot as u32);
        }
        let _ = params;

        let header = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        self.recur_targets.push(RecurTarget { header, vars: param_vars, slots: param_slots });

        let flow = self.expr(body);
        self.recur_targets.pop();
        self.builder.seal_block(header);
        match flow? {
            Flow::Val(v) => {
                self.leave_frame();
                self.builder.ins().return_(&[v]);
            }
            Flow::Diverged => {} // blocos já terminados (recur/return); fn não retorna
        }
        self.builder.finalize();
        Ok(())
    }

    fn build_main(mut self, body: &[Ast], local_count: u32) -> Result<(), Diagnostic> {
        let entry = self.builder.create_block();
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);
        self.enter_frame(local_count);
        for a in body {
            self.expr_val(a)?; // empurra 1 temporário
            self.gc_popn(1); // descarta resultado de topo
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
    fn call_void(&mut self, id: FuncId, args: &[CValue]) {
        let r = self.module.declare_func_in_func(id, self.builder.func);
        self.builder.ins().call(r, args);
    }

    /// Avalia uma expressão em posição de operando (nunca diverge — o analyzer
    /// garante que `recur` só ocorre em posição de cauda).
    fn expr_val(&mut self, ast: &Ast) -> Result<CValue, Diagnostic> {
        match self.expr(ast)? {
            Flow::Val(v) => Ok(v),
            Flow::Diverged => {
                Err(Diagnostic::error("E0112", "recur em posição não-cauda (bug do compilador)"))
            }
        }
    }

    /// Invariante de rooting: na saída `Flow::Val(v)`, `v` foi empurrado no
    /// shadow-stack exatamente uma vez (é o topo). `Flow::Diverged` já terminou
    /// o bloco (recur/return) deixando o shadow-stack consistente para o alvo.
    fn expr(&mut self, ast: &Ast) -> Result<Flow, Diagnostic> {
        Ok(match ast {
            Ast::Int(n) => {
                let tagged = (*n as i128) << 1 | 1;
                let v = self.konst(tagged as i64);
                self.gc_push_val(v);
                Flow::Val(v)
            }
            Ast::Bool(b) => {
                let v = self.konst(if *b { TRUEV } else { FALSEV });
                self.gc_push_val(v);
                Flow::Val(v)
            }
            Ast::Nil => {
                let v = self.konst(NIL);
                self.gc_push_val(v);
                Flow::Val(v)
            }
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
                let var = *self
                    .vars
                    .get(slot)
                    .ok_or_else(|| Diagnostic::error("E0111", format!("local {slot} não vinculado (bug)")))?;
                let v = self.builder.use_var(var);
                self.gc_push_val(v);
                Flow::Val(v)
            }
            Ast::Do(stmts) => {
                if stmts.is_empty() {
                    let v = self.konst(NIL);
                    self.gc_push_val(v);
                    return Ok(Flow::Val(v));
                }
                let last = stmts.len() - 1;
                for s in &stmts[..last] {
                    self.expr_val(s)?; // +1 temporário
                    self.gc_popn(1); // descarta
                }
                self.expr(&stmts[last])?
            }
            Ast::Let { slots, body } => {
                for (slot, init) in slots {
                    let val = self.expr_val(init)?; // +1 temp
                    self.bind_local(*slot, val); // escreve slot de local
                    self.gc_popn(1); // remove o temp (já está no slot)
                }
                self.expr(body)?
            }
            Ast::If(test, then, els) => self.gen_if(test, then, els)?,
            Ast::Loop { slots, body } => self.gen_loop(slots, body)?,
            Ast::Recur(args) => self.gen_recur(args)?,
            Ast::Call { callee, args } => {
                let v = self.gen_call(callee, args)?; // net-0; resultado não empurrado
                self.gc_push_val(v);
                Flow::Val(v)
            }
        })
    }

    fn gen_if(&mut self, test: &Ast, then: &Ast, els: &Ast) -> Result<Flow, Diagnostic> {
        let test_val = self.expr_val(test)?;
        let truth = self.call1(self.rt.truthy, test_val); // i32
        self.gc_popn(1); // consome o temp do teste
        let cond = self.builder.ins().icmp_imm(IntCC::NotEqual, truth, 0);

        let then_b = self.builder.create_block();
        let else_b = self.builder.create_block();
        self.builder.ins().brif(cond, then_b, &[], else_b, &[]);

        // Merge criado sob demanda (apenas se algum ramo alcança fall-through).
        let mut merge: Option<Block> = None;

        self.builder.switch_to_block(then_b);
        self.builder.seal_block(then_b);
        if let Flow::Val(tv) = self.expr(then)? {
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
        let mut vars = Vec::with_capacity(slots.len());
        let mut slot_ids = Vec::with_capacity(slots.len());
        for (slot, init) in slots {
            let v0 = self.expr_val(init)?; // +1 temp
            self.bind_local(*slot, v0); // escreve slot de local + var
            self.gc_popn(1); // remove temp
            vars.push(self.vars[slot]);
            slot_ids.push(*slot);
        }
        let header = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        self.recur_targets.push(RecurTarget { header, vars, slots: slot_ids });

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
        // Avalia todos os argumentos antes de religar (evita clobber). +k temps.
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            vals.push(self.expr_val(a)?);
        }
        for (slot, val) in target.slots.iter().zip(vals) {
            self.bind_local(*slot, val); // atualiza var + shadow slot
        }
        self.gc_popn(args.len()); // remove os k temps
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
                let mut argv = Vec::with_capacity(args.len());
                for a in args {
                    argv.push(self.expr_val(a)?); // +1 cada
                }
                let fref = self.module.declare_func_in_func(id, self.builder.func);
                let call = self.builder.ins().call(fref, &argv);
                let r = self.builder.inst_results(call)[0];
                self.gc_popn(args.len());
                Ok(r)
            }
        }
    }

    fn gen_prim(&mut self, prim: Prim, args: &[Ast]) -> Result<CValue, Diagnostic> {
        match prim {
            Prim::Println | Prim::Print => self.gen_print(prim, args),
            Prim::Str => self.gen_str(args),
            Prim::List => self.gen_list(args),
            // binárias associativas (fold)
            Prim::Add => self.fold_bin(self.rt.add, args),
            Prim::Mul => self.fold_bin(self.rt.mul, args),
            Prim::Sub => {
                if args.len() == 1 {
                    let v = self.expr_val(&args[0])?; // +1
                    let zero = self.konst(1); // MK_FIX(0) == 1 (imediato, sem root)
                    let r = self.call2(self.rt.sub, zero, v);
                    self.gc_popn(1);
                    Ok(r)
                } else {
                    self.fold_bin(self.rt.sub, args)
                }
            }
            // binárias estritas
            Prim::Quot => self.bin(self.rt.quot, args),
            Prim::Mod => self.bin(self.rt.mod_, args),
            Prim::Eq => self.bin(self.rt.eq, args),
            Prim::Lt => self.bin(self.rt.lt, args),
            Prim::Le => self.bin(self.rt.le, args),
            Prim::Gt => self.bin(self.rt.gt, args),
            Prim::Ge => self.bin(self.rt.ge, args),
            Prim::Cons => self.bin(self.rt.cons, args),
            // unárias
            Prim::Inc => self.una(self.rt.inc, args),
            Prim::Dec => self.una(self.rt.dec, args),
            Prim::Not => self.una(self.rt.not_, args),
            Prim::NilP => self.una(self.rt.nilp, args),
            Prim::EmptyP => self.una(self.rt.emptyp, args),
            Prim::First => self.una(self.rt.first, args),
            Prim::Rest => self.una(self.rt.rest, args),
            Prim::Count => self.una(self.rt.count, args),
        }
    }

    /// Fold de binária **sem alocação** (aritmética): args viram temps, calcula,
    /// retira todos os temps. Net-0.
    fn fold_bin(&mut self, id: FuncId, args: &[Ast]) -> Result<CValue, Diagnostic> {
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            vals.push(self.expr_val(a)?); // +1 cada
        }
        let mut acc = vals[0];
        for v in &vals[1..] {
            acc = self.call2(id, acc, *v);
        }
        self.gc_popn(args.len());
        Ok(acc)
    }
    fn bin(&mut self, id: FuncId, args: &[Ast]) -> Result<CValue, Diagnostic> {
        let a = self.expr_val(&args[0])?; // +1
        let b = self.expr_val(&args[1])?; // +1
        let r = self.call2(id, a, b);
        self.gc_popn(2);
        Ok(r)
    }
    fn una(&mut self, id: FuncId, args: &[Ast]) -> Result<CValue, Diagnostic> {
        let a = self.expr_val(&args[0])?; // +1
        let r = self.call1(id, a);
        self.gc_popn(1);
        Ok(r)
    }

    fn gen_print(&mut self, prim: Prim, args: &[Ast]) -> Result<CValue, Diagnostic> {
        for (i, a) in args.iter().enumerate() {
            if i > 0 {
                self.call_void(self.rt.print_space, &[]);
            }
            let v = self.expr_val(a)?; // +1 (mantido rooteado durante os próximos)
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
            vals.push(self.expr_val(a)?); // n temps
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
            vals.push(self.expr_val(a)?); // n temps
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
        Ast::If(a, c, d) => {
            collect_strings(a, out);
            collect_strings(c, out);
            collect_strings(d, out);
        }
        Ast::Do(v) => v.iter().for_each(|a| collect_strings(a, out)),
        Ast::Recur(v) => v.iter().for_each(|a| collect_strings(a, out)),
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
