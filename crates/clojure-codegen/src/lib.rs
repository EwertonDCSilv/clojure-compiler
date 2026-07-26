//! Codegen nativo (corte vertical da Fase 5): `Program` (AST do analyzer) →
//! objeto nativo via Cranelift. Baseado no protótipo #1 (ADR-0001).
//!
//! Modelo de valores do slice: tudo é `i64` (inteiros; `nil`/`false`=0, `true`=1).
//! Strings existem apenas como literais argumento de `println` (emitidas como dados
//! e impressas via runtime C). A representação completa (`Value`/GC) é das Fases 4+.
//!
//! Chamadas ao runtime via ABI C: `cljn_print_str`, `cljn_print_i64`,
//! `cljn_print_space`, `cljn_print_newline` (fornecidas por `runtime.c`).

use clojure_analyzer::{Ast, Callee, Prim, Program};
use clojure_diagnostics::{Diagnostic, Diagnostics};
use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{types, AbiParam, InstBuilder, Value as CValue};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::{isa, Context};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{default_libcall_names, DataDescription, DataId, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use std::collections::HashMap;
use target_lexicon::Triple;

/// Fonte C do runtime mínimo, escrita em disco e compilada junto ao objeto.
pub const RUNTIME_C: &str = r#"/* runtime mínimo do slice compilável — gerado por clojure-codegen */
#include <stdio.h>
#include <stddef.h>
void cljn_print_str(const char* p, long len) { fwrite(p, 1, (size_t)len, stdout); }
void cljn_print_i64(long n) { printf("%ld", n); }
void cljn_print_space(void) { fputc(' ', stdout); }
void cljn_print_newline(void) { fputc('\n', stdout); }
"#;

/// IDs das funções de runtime importadas.
struct Runtime {
    print_str: FuncId,
    print_i64: FuncId,
    print_space: FuncId,
    print_newline: FuncId,
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

    // Runtime imports.
    let runtime = declare_runtime(&mut module, ptr);

    // Dados: strings únicas usadas em println.
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
        d.define(s.clone().into_bytes().into_boxed_slice());
        module.define_data(id, &d).map_err(|e| single(format!("define_data: {e}")))?;
        str_data.insert(s.clone(), (id, s.len()));
    }

    // Declara todas as funções do usuário (para recursão / forward-ref).
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

    // Define cada função do usuário.
    for f in &program.functions {
        let (id, _) = fn_ids[&f.name];
        let mut ctx = Context::new();
        ctx.func.signature = module.declarations().get_function_decl(id).signature.clone();
        let mut fbctx = FunctionBuilderContext::new();
        let res = {
            let mut fg = FnGen::new(&mut module, &mut ctx.func, &mut fbctx, ptr, &runtime, &fn_ids, &str_data);
            fg.build_function(&f.params, &f.body, f.local_count)
        };
        if let Err(d) = res {
            diags.push(d);
            continue;
        }
        if let Err(e) = module.define_function(id, &mut ctx) {
            diags.push(single_d(format!("define_function {}: {e}", f.name)));
        }
    }

    // Define `main` (int main()) a partir do corpo de topo (modelo de script).
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
        if let Err(d) = res {
            diags.push(d);
        } else if let Err(e) = module.define_function(main_id, &mut ctx) {
            diags.push(single_d(format!("define main: {e}")));
        }
    }

    if diags.has_errors() {
        return Err(diags);
    }

    let product = module.finish();
    product.emit().map_err(|e| single(format!("emit do objeto: {e}")))
}

fn declare_runtime(module: &mut ObjectModule, ptr: types::Type) -> Runtime {
    let mut sig_str = module.make_signature();
    sig_str.params.push(AbiParam::new(ptr));
    sig_str.params.push(AbiParam::new(types::I64));
    let print_str = module.declare_function("cljn_print_str", Linkage::Import, &sig_str).unwrap();

    let mut sig_i64 = module.make_signature();
    sig_i64.params.push(AbiParam::new(types::I64));
    let print_i64 = module.declare_function("cljn_print_i64", Linkage::Import, &sig_i64).unwrap();

    let sig_void = module.make_signature();
    let print_space = module.declare_function("cljn_print_space", Linkage::Import, &sig_void).unwrap();
    let print_newline = module.declare_function("cljn_print_newline", Linkage::Import, &sig_void).unwrap();

    Runtime { print_str, print_i64, print_space, print_newline }
}

struct FnGen<'a> {
    module: &'a mut ObjectModule,
    builder: FunctionBuilder<'a>,
    ptr: types::Type,
    runtime: &'a Runtime,
    fn_ids: &'a HashMap<String, (FuncId, usize)>,
    str_data: &'a HashMap<String, (DataId, usize)>,
    vars: HashMap<u32, Variable>,
}

impl<'a> FnGen<'a> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        module: &'a mut ObjectModule,
        func: &'a mut cranelift_codegen::ir::Function,
        fbctx: &'a mut FunctionBuilderContext,
        ptr: types::Type,
        runtime: &'a Runtime,
        fn_ids: &'a HashMap<String, (FuncId, usize)>,
        str_data: &'a HashMap<String, (DataId, usize)>,
    ) -> Self {
        FnGen {
            module,
            builder: FunctionBuilder::new(func, fbctx),
            ptr,
            runtime,
            fn_ids,
            str_data,
            vars: HashMap::new(),
        }
    }

    fn new_var(&mut self, slot: u32) -> Variable {
        let v = self.builder.declare_var(types::I64);
        self.vars.insert(slot, v);
        v
    }

    fn build_function(mut self, params: &[String], body: &Ast, _locals: u32) -> Result<(), Diagnostic> {
        let entry = self.builder.create_block();
        self.builder.append_block_params_for_function_params(entry);
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);

        // Vincula parâmetros aos slots 0..n.
        let param_vals: Vec<CValue> = self.builder.block_params(entry).to_vec();
        for (slot, val) in param_vals.iter().enumerate() {
            let v = self.new_var(slot as u32);
            self.builder.def_var(v, *val);
        }
        let _ = params;

        let ret = self.expr(body)?;
        self.builder.ins().return_(&[ret]);
        self.builder.finalize();
        Ok(())
    }

    fn build_main(mut self, body: &[Ast], _locals: u32) -> Result<(), Diagnostic> {
        let entry = self.builder.create_block();
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);
        for a in body {
            self.expr(a)?;
        }
        let zero = self.builder.ins().iconst(types::I32, 0);
        self.builder.ins().return_(&[zero]);
        self.builder.finalize();
        Ok(())
    }

    fn expr(&mut self, ast: &Ast) -> Result<CValue, Diagnostic> {
        match ast {
            Ast::Int(n) => Ok(self.builder.ins().iconst(types::I64, *n)),
            Ast::Bool(b) => Ok(self.builder.ins().iconst(types::I64, *b as i64)),
            Ast::Nil => Ok(self.builder.ins().iconst(types::I64, 0)),
            Ast::Str(_) => Err(Diagnostic::error(
                "E0110",
                "string só é compilável como argumento direto de println (slice)",
            )),
            Ast::Local(slot) => {
                let v = *self
                    .vars
                    .get(slot)
                    .ok_or_else(|| Diagnostic::error("E0111", format!("local {slot} não vinculado (bug do compilador)")))?;
                Ok(self.builder.use_var(v))
            }
            Ast::Do(stmts) => {
                let mut last = self.builder.ins().iconst(types::I64, 0);
                for s in stmts {
                    last = self.expr(s)?;
                }
                Ok(last)
            }
            Ast::Let { slots, body } => {
                for (slot, init) in slots {
                    let val = self.expr(init)?;
                    let v = self.new_var(*slot);
                    self.builder.def_var(v, val);
                }
                self.expr(body)
            }
            Ast::If(test, then, els) => self.gen_if(test, then, els),
            Ast::Call { callee, args } => self.gen_call(callee, args),
        }
    }

    fn gen_if(&mut self, test: &Ast, then: &Ast, els: &Ast) -> Result<CValue, Diagnostic> {
        let test_val = self.expr(test)?;
        let cond = self.builder.ins().icmp_imm(IntCC::NotEqual, test_val, 0);

        let then_b = self.builder.create_block();
        let else_b = self.builder.create_block();
        let merge_b = self.builder.create_block();
        self.builder.append_block_param(merge_b, types::I64);

        self.builder.ins().brif(cond, then_b, &[], else_b, &[]);

        self.builder.switch_to_block(then_b);
        self.builder.seal_block(then_b);
        let tv = self.expr(then)?;
        self.builder.ins().jump(merge_b, &[tv.into()]);

        self.builder.switch_to_block(else_b);
        self.builder.seal_block(else_b);
        let ev = self.expr(els)?;
        self.builder.ins().jump(merge_b, &[ev.into()]);

        self.builder.switch_to_block(merge_b);
        self.builder.seal_block(merge_b);
        Ok(self.builder.block_params(merge_b)[0])
    }

    fn gen_call(&mut self, callee: &Callee, args: &[Ast]) -> Result<CValue, Diagnostic> {
        match callee {
            Callee::Prim(Prim::Println) => self.gen_println(args),
            Callee::Prim(p) => self.gen_prim(*p, args),
            Callee::Fn(name) => {
                let (id, _) = self.fn_ids[name];
                let fref = self.module.declare_func_in_func(id, self.builder.func);
                let mut argv = Vec::with_capacity(args.len());
                for a in args {
                    argv.push(self.expr(a)?);
                }
                let call = self.builder.ins().call(fref, &argv);
                Ok(self.builder.inst_results(call)[0])
            }
        }
    }

    fn gen_prim(&mut self, prim: Prim, args: &[Ast]) -> Result<CValue, Diagnostic> {
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            vals.push(self.expr(a)?);
        }
        let b = &mut self.builder;
        Ok(match prim {
            Prim::Add => {
                let mut acc = vals[0];
                for v in &vals[1..] {
                    acc = b.ins().iadd(acc, *v);
                }
                acc
            }
            Prim::Mul => {
                let mut acc = vals[0];
                for v in &vals[1..] {
                    acc = b.ins().imul(acc, *v);
                }
                acc
            }
            Prim::Sub => {
                if vals.len() == 1 {
                    b.ins().ineg(vals[0])
                } else {
                    let mut acc = vals[0];
                    for v in &vals[1..] {
                        acc = b.ins().isub(acc, *v);
                    }
                    acc
                }
            }
            Prim::Eq => cmp(b, IntCC::Equal, vals[0], vals[1]),
            Prim::Lt => cmp(b, IntCC::SignedLessThan, vals[0], vals[1]),
            Prim::Le => cmp(b, IntCC::SignedLessThanOrEqual, vals[0], vals[1]),
            Prim::Gt => cmp(b, IntCC::SignedGreaterThan, vals[0], vals[1]),
            Prim::Ge => cmp(b, IntCC::SignedGreaterThanOrEqual, vals[0], vals[1]),
            Prim::Println => unreachable!(),
        })
    }

    fn gen_println(&mut self, args: &[Ast]) -> Result<CValue, Diagnostic> {
        for (i, a) in args.iter().enumerate() {
            if i > 0 {
                let sp = self.module.declare_func_in_func(self.runtime.print_space, self.builder.func);
                self.builder.ins().call(sp, &[]);
            }
            match a {
                Ast::Str(s) => {
                    let (data_id, len) = self.str_data[s];
                    let gv = self.module.declare_data_in_func(data_id, self.builder.func);
                    let p = self.builder.ins().symbol_value(self.ptr, gv);
                    let len_v = self.builder.ins().iconst(types::I64, len as i64);
                    let f = self.module.declare_func_in_func(self.runtime.print_str, self.builder.func);
                    self.builder.ins().call(f, &[p, len_v]);
                }
                other => {
                    let v = self.expr(other)?;
                    let f = self.module.declare_func_in_func(self.runtime.print_i64, self.builder.func);
                    self.builder.ins().call(f, &[v]);
                }
            }
        }
        let nl = self.module.declare_func_in_func(self.runtime.print_newline, self.builder.func);
        self.builder.ins().call(nl, &[]);
        Ok(self.builder.ins().iconst(types::I64, 0))
    }
}

fn cmp(b: &mut FunctionBuilder, cc: IntCC, x: CValue, y: CValue) -> CValue {
    let r = b.ins().icmp(cc, x, y);
    b.ins().uextend(types::I64, r)
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
        Ast::Let { slots, body } => {
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
