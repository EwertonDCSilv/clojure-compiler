//! `clojure-native` command-line frontend.
//!
//! `read` emits a deterministic form dump; `eval` and `run` execute through the
//! bootstrap interpreter; and `build` reads the compiled core and user source
//! as one program, analyzes it, emits a Cranelift object, and links it with the
//! embedded C runtime. Temporary object and runtime-source files are removed
//! after the host C compiler exits. User-facing diagnostics remain Portuguese.

use clojure_interp::Interp;
use clojure_span::SourceMap;
use std::process::ExitCode;

/// Compilable `clojure.core` subset prepended to every native build.
const CORE_COMPILED: &str = include_str!("core_compiled.clj");

/// Compiler-owned built-in namespaces, embedded and resolved ahead of any local
/// `--source-path` root (ADR-0013 §8). Offline and deterministic.
const BUILTIN_MODULES: &[(&str, &str)] = &[(
    "cljn.http.response",
    include_str!("../../../stdlib/cljn/http/response.clj"),
)];

/// Returns the embedded source of a built-in namespace, if any.
fn builtin_source(ns: &str) -> Option<&'static str> {
    BUILTIN_MODULES
        .iter()
        .find(|(name, _)| *name == ns)
        .map(|(_, src)| *src)
}

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

/// Extracts required namespace names from the `ns` form's `:require` clauses.
///
/// Only literal `(ns name (:require spec...))` is inspected. Each spec is a bare
/// namespace symbol or a vector whose head is the namespace symbol (ADR-0013).
fn parse_requires(forms: &[clojure_syntax::SForm]) -> Vec<String> {
    use clojure_syntax::Form;
    let mut out = Vec::new();
    for f in forms {
        let Form::List(items) = f.node.strip_meta() else {
            continue;
        };
        let Some(Form::Symbol(h)) = items.first().map(|x| x.node.strip_meta()) else {
            continue;
        };
        if h.ns.is_some() || h.name != "ns" {
            continue;
        }
        for clause in items.iter().skip(2) {
            let Form::List(citems) = clause.node.strip_meta() else {
                continue;
            };
            let is_require = matches!(
                citems.first().map(|x| x.node.strip_meta()),
                Some(Form::Keyword(k)) if k.ns.is_none() && k.name == "require"
            );
            if !is_require {
                continue;
            }
            for spec in citems.iter().skip(1) {
                let ns_sym = match spec.node.strip_meta() {
                    Form::Vector(v) => v.first().map(|x| x.node.strip_meta()),
                    other @ Form::Symbol(_) => Some(other),
                    _ => None,
                };
                if let Some(Form::Symbol(nsym)) = ns_sym {
                    out.push(nsym.to_string());
                }
            }
        }
    }
    out
}

/// Resolves a namespace name to a source file within the given roots.
///
/// Dots become directory separators and hyphens become underscores in each path
/// segment, matching the Clojure file-naming convention.
fn resolve_ns_path(ns: &str, roots: &[std::path::PathBuf]) -> Option<std::path::PathBuf> {
    let rel = ns.replace('.', "/").replace('-', "_") + ".clj";
    roots.iter().map(|r| r.join(&rel)).find(|p| p.is_file())
}

/// Loads `requires` and their transitive dependencies into `out`, in dependency
/// (post-order) order, before any dependent. Detects cycles and missing files.
///
/// Resolution is offline and deterministic: no Maven, JAR, Git, or network access.
fn load_deps(
    requires: &[String],
    roots: &[std::path::PathBuf],
    sm: &mut SourceMap,
    loaded: &mut std::collections::HashSet<String>,
    stack: &mut Vec<String>,
    out: &mut Vec<clojure_syntax::SForm>,
) -> Result<(), String> {
    for ns in requires {
        if loaded.contains(ns) {
            continue;
        }
        if stack.contains(ns) {
            return Err(format!(
                "ciclo de dependência de namespace: {} -> {ns}",
                stack.join(" -> ")
            ));
        }
        // Built-in compiler-owned sources resolve before the local source path.
        let (source_name, text) = if let Some(src) = builtin_source(ns) {
            (
                ns.replace('.', "/").replace('-', "_") + ".clj",
                src.to_string(),
            )
        } else {
            let Some(path) = resolve_ns_path(ns, roots) else {
                return Err(format!("namespace não encontrado no source-path: {ns}"));
            };
            let text = std::fs::read_to_string(&path)
                .map_err(|e| format!("erro ao ler {}: {e}", path.display()))?;
            (path.display().to_string(), text)
        };
        let sid = sm.add(source_name, text.clone());
        let forms = clojure_reader::read_all(sid, &text).map_err(|d| d.render(sm))?;
        let sub = parse_requires(&forms);
        stack.push(ns.clone());
        load_deps(&sub, roots, sm, loaded, stack, out)?;
        stack.pop();
        out.extend(forms);
        loaded.insert(ns.clone());
    }
    Ok(())
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
    // Script forms execute from top to bottom. `--main` adds an explicit
    // zero-argument call to `-main` after all top-level forms have run.
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
    // The first positional argument is the source file; flags configure the
    // output path and Cranelift optimization level.
    let mut path = None;
    let mut output = None;
    let mut optimization_level = None;
    let mut source_paths: Vec<std::path::PathBuf> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                output = args.get(i + 1).cloned();
                i += 2;
            }
            "--source-path" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("--source-path requer um diretório");
                    return ExitCode::FAILURE;
                };
                source_paths.push(std::path::PathBuf::from(value));
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

    // The entry file's own directory is always a resolution root (ADR-0013).
    if let Some(parent) = std::path::Path::new(&path).parent() {
        source_paths.push(parent.to_path_buf());
    }

    let mut sm = SourceMap::new();
    // Stage 0: prepend the compiled bootstrap core so its collection functions
    // are available without user declarations (ADR-0005).
    let core_id = sm.add("clojure/core.clj", CORE_COMPILED);
    let id = sm.add(path.clone(), text.clone());

    // Stage 1: read core and entry forms with distinct source identities.
    let mut forms = match clojure_reader::read_all(core_id, CORE_COMPILED) {
        Ok(f) => f,
        Err(d) => {
            eprintln!("erro interno no core.clj compilável:\n{}", d.render(&sm));
            return ExitCode::FAILURE;
        }
    };
    let entry_forms = match clojure_reader::read_all(id, &text) {
        Ok(f) => f,
        Err(d) => {
            eprintln!("{}", d.render(&sm));
            return ExitCode::FAILURE;
        }
    };
    // Stage 1b: statically resolve `:require`d namespaces from the source path,
    // in dependency (topological) order, before the entry (ADR-0013 Gate 1).
    let entry_requires = parse_requires(&entry_forms);
    let mut loaded = std::collections::HashSet::new();
    let mut stack = Vec::new();
    if let Err(msg) = load_deps(
        &entry_requires,
        &source_paths,
        &mut sm,
        &mut loaded,
        &mut stack,
        &mut forms,
    ) {
        eprintln!("{msg}");
        return ExitCode::FAILURE;
    }
    forms.extend(entry_forms);
    // Stage 2: resolve and validate the combined program.
    let program = match clojure_analyzer::analyze(&forms) {
        Ok(p) => p,
        Err(d) => {
            eprintln!("{}", d.render(&sm));
            return ExitCode::FAILURE;
        }
    };
    // Stage 3: lower the analyzed program to a host object.
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

    // Stage 4: materialize the object and amalgamated runtime, then ask the host
    // C compiler driver to link the final executable.
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
