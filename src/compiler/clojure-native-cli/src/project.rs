//! Offline, deterministic namespace-dependency loading for `build`.
//!
//! Resolves a source file's `:require` clauses and any qualified reference to a
//! compiler-owned built-in namespace (ADR-0013 §8) into a topologically ordered
//! form list, with no Maven, JAR, Git, or network access.

use clojure_span::SourceMap;

/// Compiler-owned built-in namespaces, embedded and resolved ahead of any local
/// `--source-path` root (ADR-0013 §8). Offline and deterministic.
pub(crate) const BUILTIN_MODULES: &[(&str, &str)] = &[
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
pub(crate) fn builtin_source(ns: &str) -> Option<&'static str> {
    BUILTIN_MODULES
        .iter()
        .find(|(name, _)| *name == ns)
        .map(|(_, src)| *src)
}

/// Extracts required namespace names from the `ns` form's `:require` clauses.
///
/// Only literal `(ns name (:require spec...))` is inspected. Each spec is a bare
/// namespace symbol or a vector whose head is the namespace symbol (ADR-0013).
pub(crate) fn parse_requires(forms: &[clojure_syntax::SForm]) -> Vec<String> {
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
pub(crate) fn implicit_builtin_requires(forms: &[clojure_syntax::SForm]) -> Vec<String> {
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
pub(crate) fn load_deps(
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
