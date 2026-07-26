//! `clojure-native` — CLI. MVP: `read`, `eval`, `run`.
//!
//! O caminho `build` (compilação AOT nativa via Cranelift) é das Fases 5+ e ainda
//! não está ligado aqui; ver specs/IMPLEMENTATION_PLAN.md.

use clojure_interp::Interp;
use clojure_span::SourceMap;
use std::process::ExitCode;

/// `clojure.core` no subconjunto compilável, pré-carregado em todo `build`.
const CORE_COMPILED: &str = include_str!("core_compiled.clj");

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("read") => cmd_read(&args[1..]),
        Some("eval") => cmd_eval(&args[1..]),
        Some("run") => cmd_run(&args[1..]),
        Some("--help") | Some("-h") | Some("help") | None => {
            print_usage();
            ExitCode::SUCCESS
        }
        Some("build") => cmd_build(&args[1..]),
        Some(other) => {
            eprintln!("comando desconhecido: {other}\n");
            print_usage();
            ExitCode::FAILURE
        }
    }
}

fn print_usage() {
    println!(
        "clojure-native {} — implementação nativa de Clojure (em desenvolvimento)\n\n\
         USO:\n\
         \x20 clojure-native read  <arquivo.clj>           Lê e imprime as forms (dump determinístico)\n\
         \x20 clojure-native eval  <expr>                  Avalia uma expressão (interpretador)\n\
         \x20 clojure-native run   <arquivo.clj> [--main]  Executa via interpretador (script)\n\
         \x20 clojure-native build <arquivo.clj> [-o out] [--opt-level nível]\n\
         \x20                                              Compila para binário nativo\n\
         \n\
         NÍVEIS DE OTIMIZAÇÃO DO BUILD:\n\
         \x20 none | speed | speed-and-size\n\
         \x20 Padrão atual: none; speed permanece experimental devido ao gate Cormen\n",
        env!("CARGO_PKG_VERSION")
    );
}

fn read_file(path: &str) -> Result<String, ExitCode> {
    std::fs::read_to_string(path).map_err(|e| {
        eprintln!("erro ao ler {path}: {e}");
        ExitCode::FAILURE
    })
}

fn cmd_read(args: &[String]) -> ExitCode {
    let Some(path) = args.first() else {
        eprintln!("uso: clojure-native read <arquivo.clj>");
        return ExitCode::FAILURE;
    };
    let text = match read_file(path) {
        Ok(t) => t,
        Err(c) => return c,
    };
    let mut sm = SourceMap::new();
    let id = sm.add(path.clone(), text.clone());
    match clojure_reader::read_all(id, &text) {
        Ok(forms) => {
            for f in &forms {
                println!("{}", f.node);
            }
            ExitCode::SUCCESS
        }
        Err(diags) => {
            eprintln!("{}", diags.render(&sm));
            ExitCode::FAILURE
        }
    }
}

fn cmd_eval(args: &[String]) -> ExitCode {
    if args.is_empty() {
        eprintln!("uso: clojure-native eval <expr>");
        return ExitCode::FAILURE;
    }
    let src = args.join(" ");
    let mut it = match Interp::with_core() {
        Ok(it) => it,
        Err(e) => {
            eprintln!("erro ao carregar core: {e}");
            return ExitCode::FAILURE;
        }
    };
    match it.eval_source("<eval>", &src) {
        Ok(v) => {
            let out = it.take_output();
            if !out.is_empty() {
                print!("{out}");
            }
            println!("{}", clojure_value::pr_str(&v));
            ExitCode::SUCCESS
        }
        Err(e) => {
            report(&e);
            ExitCode::FAILURE
        }
    }
}

fn cmd_run(args: &[String]) -> ExitCode {
    let Some(path) = args.first() else {
        eprintln!("uso: clojure-native run <arquivo.clj>");
        return ExitCode::FAILURE;
    };
    let text = match read_file(path) {
        Ok(t) => t,
        Err(c) => return c,
    };
    let mut it = match Interp::with_core() {
        Ok(it) => it,
        Err(e) => {
            eprintln!("erro ao carregar core: {e}");
            return ExitCode::FAILURE;
        }
    };
    // Semântica de script: as forms de topo executam na ordem (como o exemplo da
    // Fase 5, que chama `(-main)` no topo). Programas que preferem o modelo "main"
    // podem passar `--main` para invocar `-main` explicitamente.
    if let Err(e) = it.eval_source(path, &text) {
        print!("{}", it.take_output());
        report(&e);
        return ExitCode::FAILURE;
    }
    if args.iter().any(|a| a == "--main") {
        if let Err(e) = it.call_main() {
            print!("{}", it.take_output());
            report(&e);
            return ExitCode::FAILURE;
        }
    }
    print!("{}", it.take_output());
    ExitCode::SUCCESS
}

fn cmd_build(args: &[String]) -> ExitCode {
    // Parse simples: primeiro não-flag é o arquivo; as demais opções configuram
    // a saída e o backend.
    let mut path = None;
    let mut output = None;
    let mut optimization_level = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                output = args.get(i + 1).cloned();
                i += 2;
            }
            "--opt-level" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("--opt-level requer um valor: none, speed ou speed-and-size");
                    return ExitCode::FAILURE;
                };
                match value.parse::<clojure_codegen::OptimizationLevel>() {
                    Ok(level) => optimization_level = Some(level),
                    Err(message) => {
                        eprintln!("nível de otimização inválido `{value}`; {message}");
                        return ExitCode::FAILURE;
                    }
                }
                i += 2;
            }
            option if option.starts_with('-') => {
                eprintln!("opção de build desconhecida: {option}");
                return ExitCode::FAILURE;
            }
            other => {
                if path.is_none() {
                    path = Some(other.to_string());
                } else {
                    eprintln!("argumento inesperado no build: {other}");
                    return ExitCode::FAILURE;
                }
                i += 1;
            }
        }
    }
    let Some(path) = path else {
        eprintln!("uso: clojure-native build <arquivo.clj> [-o saída]");
        return ExitCode::FAILURE;
    };
    let text = match read_file(&path) {
        Ok(t) => t,
        Err(c) => return c,
    };
    let out_name = output.unwrap_or_else(|| {
        std::path::Path::new(&path)
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "a".to_string())
    });

    let mut sm = SourceMap::new();
    // 0) core.clj compilável (bootstrap): map/filter/reduce/range/... disponíveis
    //    sem o usuário defini-los (ADR-0005).
    let core_id = sm.add("clojure/core.clj", CORE_COMPILED);
    let id = sm.add(path.clone(), text.clone());

    // 1) Reader (core + usuário).
    let mut forms = match clojure_reader::read_all(core_id, CORE_COMPILED) {
        Ok(f) => f,
        Err(d) => {
            eprintln!("erro interno no core.clj compilável:\n{}", d.render(&sm));
            return ExitCode::FAILURE;
        }
    };
    match clojure_reader::read_all(id, &text) {
        Ok(f) => forms.extend(f),
        Err(d) => {
            eprintln!("{}", d.render(&sm));
            return ExitCode::FAILURE;
        }
    }
    // 2) Analyzer.
    let program = match clojure_analyzer::analyze(&forms) {
        Ok(p) => p,
        Err(d) => {
            eprintln!("{}", d.render(&sm));
            return ExitCode::FAILURE;
        }
    };
    // 3) Codegen → objeto.
    let codegen_options = optimization_level.map_or_else(
        clojure_codegen::CodegenOptions::default,
        |optimization_level| clojure_codegen::CodegenOptions { optimization_level },
    );
    let obj = match clojure_codegen::compile_object_with_options(&program, codegen_options) {
        Ok(o) => o,
        Err(d) => {
            eprintln!("{}", d.render(&sm));
            return ExitCode::FAILURE;
        }
    };

    // 4) Escreve objeto + runtime C e linka com `cc`.
    let tmp = std::env::temp_dir();
    let obj_path = tmp.join(format!("{out_name}.o"));
    let rt_path = tmp.join(format!("{out_name}.cljn_runtime.c"));
    if let Err(e) = std::fs::write(&obj_path, &obj) {
        eprintln!("erro ao escrever objeto: {e}");
        return ExitCode::FAILURE;
    }
    if let Err(e) = std::fs::write(&rt_path, clojure_codegen::RUNTIME_C) {
        eprintln!("erro ao escrever runtime: {e}");
        return ExitCode::FAILURE;
    }
    let cc = std::env::var("CC").unwrap_or_else(|_| "cc".to_string());
    let status = std::process::Command::new(&cc)
        .arg(&obj_path)
        .arg(&rt_path)
        .arg("-o")
        .arg(&out_name)
        .status();
    let _ = std::fs::remove_file(&obj_path);
    let _ = std::fs::remove_file(&rt_path);
    match status {
        Ok(s) if s.success() => {
            println!("binário nativo gerado: {out_name}");
            ExitCode::SUCCESS
        }
        Ok(s) => {
            eprintln!("linker ({cc}) falhou com status {s}");
            ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("falha ao invocar o linker `{cc}`: {e}");
            ExitCode::FAILURE
        }
    }
}

fn report(e: &clojure_interp::EvalError) {
    match e.span {
        Some(s) => eprintln!("erro: {} [{}:{}..{}]", e.msg, s.source, s.start, s.end),
        None => eprintln!("erro: {}", e.msg),
    }
}
