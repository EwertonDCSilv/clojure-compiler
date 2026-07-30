//! Unit tests for main.rs.

use super::implicit_builtin_requires;

fn read(source: &str) -> Vec<clojure_syntax::SForm> {
    clojure_reader::read_all(0, source).expect("test source should parse")
}

#[test]
fn implicit_builtin_requires_walk_nested_forms_once_in_source_order() {
    let forms = read(
        "(ns app.core)\n\
             (def calls [(cljn.io/exists? \"x\") \
                         {:env (cljn.process/getenv \"HOME\")}])\n\
             (cljn.io/file? \"x\")",
    );

    assert_eq!(
        implicit_builtin_requires(&forms),
        vec!["cljn.io".to_string(), "cljn.process".to_string()]
    );
}

#[test]
fn implicit_builtin_requires_ignores_unbundled_namespaces() {
    let forms = read("(ns app.core)\n(user.library/call)");

    assert!(implicit_builtin_requires(&forms).is_empty());
}
