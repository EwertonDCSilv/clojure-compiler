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
const BUILTIN_MODULES: &[(&str, &str)] = &[
    ("cljn.io", include_str!("../../../stdlib/cljn/io.clj")),
    (
        "cljn.process",
        include_str!("../../../stdlib/cljn/process.clj"),
    ),
    (
        "clojure.edn",
        include_str!("../../../stdlib/clojure/edn.clj"),
    ),
    (
        "cljn.http.request",
        include_str!("../../../stdlib/cljn/http/request.clj"),
    ),
    (
        "cljn.http.response",
        include_str!("../../../stdlib/cljn/http/response.clj"),
    ),
    (
        "cljn.pedestal.chain",
        include_str!("../../../stdlib/cljn/pedestal/chain.clj"),
    ),
    (
        "cljn.pedestal.route",
        include_str!("../../../stdlib/cljn/pedestal/route.clj"),
    ),
    (
        "cljn.pedestal.connector",
        include_str!("../../../stdlib/cljn/pedestal/connector.clj"),
    ),
    (
        "cljn.pedestal.service",
        include_str!("../../../stdlib/cljn/pedestal/service.clj"),
    ),
];

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
         \x20 clojure-native build <arquivo.clj> [-o out] [--opt-level nível] [--ir-opt modo] [--ir-experiment nome] [--ir-stats arquivo]\n\
         \x20                                              Compila para binário nativo\n\
         \n\
         NÍVEIS DE OTIMIZAÇÃO DO BUILD:\n\
         \x20 none | speed | speed-and-size\n\
         \x20 Padrão atual: none; speed permanece experimental devido ao gate Cormen\n\
         \n\
         IR DE OTIMIZAÇÃO OPCIONAL:\n\
         \x20 none | safe\n\
         \x20 Padrão: none; safe executa passes verificados e especialização de fixnum\n\
         \x20 --ir-experiment adr15 ativa o bundle isolado de valores/roots/ABI\n\
         \x20 --ir-stats ARQUIVO grava métricas estruturais determinísticas\n",
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

/// Collects compiler-owned built-in namespaces referenced through a qualified
/// symbol (for example `cljn.io/exists?`) without an explicit `:require`.
///
/// The `cljn.io` and `cljn.process` namespaces are always-available native I/O
/// sugar (issue #103): using a qualified name is enough to pull in the built-in
/// module. Only namespaces backed by [`builtin_source`] are auto-loaded, so a
/// typo in a user namespace still fails as an unresolved reference.
fn implicit_builtin_requires(forms: &[clojure_syntax::SForm]) -> Vec<String> {
    use clojure_syntax::Form;
    fn walk(form: &Form, out: &mut Vec<String>) {
        match form {
            Form::Symbol(s) => {
                if let Some(ns) = &s.ns {
                    if builtin_source(ns).is_some() && !out.iter().any(|n| n == ns) {
                        out.push(ns.clone());
                    }
                }
            }
            Form::List(items) | Form::Vector(items) | Form::Set(items) => {
                for i in items {
                    walk(i.node.strip_meta(), out);
                }
            }
            Form::Map(pairs) => {
                for (k, v) in pairs {
                    walk(k.node.strip_meta(), out);
                    walk(v.node.strip_meta(), out);
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    for f in forms {
        walk(f.node.strip_meta(), &mut out);
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
        let mut sub = parse_requires(&forms);
        sub.extend(implicit_builtin_requires(&forms));
        // A built-in module may reference its own qualified names (for example a
        // `cljn.io` helper naming `cljn.io/IOException`); dropping the self-edge
        // keeps the auto-load scan from reporting a spurious dependency cycle.
        sub.retain(|dep| dep != ns);
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
    let mut ir_optimization = None;
    let mut ir_experiment = None;
    let mut ir_stats_path = None;
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
            "--ir-opt" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("--ir-opt requer um valor: none ou safe");
                    return ExitCode::FAILURE;
                };
                match value.parse::<clojure_codegen::IrOptimizationMode>() {
                    Ok(mode) => ir_optimization = Some(mode),
                    Err(message) => {
                        eprintln!("modo de IR inválido `{value}`; {message}");
                        return ExitCode::FAILURE;
                    }
                }
                i += 2;
            }
            "--ir-experiment" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("--ir-experiment requer um valor: none ou adr15");
                    return ExitCode::FAILURE;
                };
                match value.parse::<clojure_codegen::IrExperiment>() {
                    Ok(experiment) => ir_experiment = Some(experiment),
                    Err(message) => {
                        eprintln!("experimento de IR inválido `{value}`; {message}");
                        return ExitCode::FAILURE;
                    }
                }
                i += 2;
            }
            "--ir-stats" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("--ir-stats requer um caminho de saída");
                    return ExitCode::FAILURE;
                };
                ir_stats_path = Some(std::path::PathBuf::from(value));
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
    let mut entry_requires = parse_requires(&entry_forms);
    entry_requires.extend(implicit_builtin_requires(&entry_forms));
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
    let codegen_options = clojure_codegen::CodegenOptions {
        optimization_level: optimization_level.unwrap_or(clojure_codegen::OptimizationLevel::None),
        ir_optimization: ir_optimization.unwrap_or(clojure_codegen::IrOptimizationMode::None),
        ir_experiment: ir_experiment.unwrap_or(clojure_codegen::IrExperiment::None),
    };
    let (obj, optimization_stats) =
        match clojure_codegen::compile_object_with_options_and_stats(&program, codegen_options) {
            Ok(result) => result,
            Err(d) => {
                eprintln!("{}", d.render(&sm));
                return ExitCode::FAILURE;
            }
        };
    if let Some(path) = ir_stats_path {
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            if let Err(error) = std::fs::create_dir_all(parent) {
                eprintln!(
                    "erro ao criar diretório do relatório de otimização {}: {error}",
                    parent.display()
                );
                return ExitCode::FAILURE;
            }
        }
        if let Err(error) = std::fs::write(&path, optimization_stats.to_json()) {
            eprintln!(
                "erro ao escrever relatório de otimização {}: {error}",
                path.display()
            );
            return ExitCode::FAILURE;
        }
    }

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

#[cfg(test)]
#[path = "../tests/unit/main/mod.rs"]
mod tests;
