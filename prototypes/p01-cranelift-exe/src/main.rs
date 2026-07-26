//! PROTÓTIPO DESCARTÁVEL — NÃO É CÓDIGO DE PRODUÇÃO.
//!
//! Valida o backend do ADR-0001: emitir um objeto com Cranelift, gerando um
//! `main` que chama `puts` (libc) e retorna 0, depois linkar (via `cc`) em um
//! executável nativo autônomo. Responde: "compilamos para binário sem JVM?".

use cranelift_codegen::ir::{types, AbiParam, InstBuilder};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::{isa, Context};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{default_libcall_names, DataDescription, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use std::process::Command;
use target_lexicon::Triple;

fn main() {
    let obj_path = "proto.o";
    let exe_path = if cfg!(windows) { "proto.exe" } else { "./proto" };

    // --- configura a ISA host ---
    let mut flags = settings::builder();
    flags.set("is_pic", "true").unwrap();
    flags.set("opt_level", "none").unwrap();
    let isa = isa::lookup(Triple::host())
        .expect("ISA host suportada")
        .finish(settings::Flags::new(flags))
        .unwrap();

    let builder = ObjectBuilder::new(isa, "p01", default_libcall_names()).unwrap();
    let mut module = ObjectModule::new(builder);
    let ptr = module.target_config().pointer_type();

    // --- dado: string C terminada em NUL ---
    let msg = b"Hello from Cranelift (native, no JVM)\0".to_vec();
    let mut data = DataDescription::new();
    data.define(msg.into_boxed_slice());
    let data_id = module.declare_data("msg", Linkage::Local, false, false).unwrap();
    module.define_data(data_id, &data).unwrap();

    // --- declara puts: int puts(const char*) ---
    let mut puts_sig = module.make_signature();
    puts_sig.params.push(AbiParam::new(ptr));
    puts_sig.returns.push(AbiParam::new(types::I32));
    let puts_id = module.declare_function("puts", Linkage::Import, &puts_sig).unwrap();

    // --- declara main: int main() ---
    let mut main_sig = module.make_signature();
    main_sig.returns.push(AbiParam::new(types::I32));
    let main_id = module.declare_function("main", Linkage::Export, &main_sig).unwrap();

    // --- corpo de main ---
    let mut ctx = Context::new();
    ctx.func.signature = main_sig;
    let mut fctx = FunctionBuilderContext::new();
    {
        let mut b = FunctionBuilder::new(&mut ctx.func, &mut fctx);
        let block = b.create_block();
        b.append_block_params_for_function_params(block);
        b.switch_to_block(block);
        b.seal_block(block);

        let msg_gv = module.declare_data_in_func(data_id, b.func);
        let msg_ptr = b.ins().symbol_value(ptr, msg_gv);

        let puts_ref = module.declare_func_in_func(puts_id, b.func);
        b.ins().call(puts_ref, &[msg_ptr]);

        let zero = b.ins().iconst(types::I32, 0);
        b.ins().return_(&[zero]);
        b.finalize();
    }
    module.define_function(main_id, &mut ctx).unwrap();

    // --- emite o objeto ---
    let product = module.finish();
    let bytes = product.emit().unwrap();
    std::fs::write(obj_path, &bytes).unwrap();
    println!("[proto] objeto emitido: {obj_path} ({} bytes)", bytes.len());

    // --- linka com cc (clang/gcc) ---
    let cc = std::env::var("CC").unwrap_or_else(|_| "cc".to_string());
    let out_flag = if cfg!(windows) { "proto.exe" } else { "proto" };
    let status = Command::new(&cc)
        .args([obj_path, "-o", out_flag])
        .status()
        .expect("falha ao invocar o linker (cc)");
    assert!(status.success(), "link falhou");
    println!("[proto] executável linkado: {exe_path}");
    println!("[proto] agora rode: {exe_path}");
}
