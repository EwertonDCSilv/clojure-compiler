//! Unit tests for expand.rs.

use super::*;

fn expanded(source: &str) -> String {
    let forms = clojure_reader::read_all(0, source).expect("reader");
    expand_all(&forms)
        .iter()
        .map(|form| form.node.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn expands_conditional_macros() {
    assert_eq!(
        expanded("(when ready (print 1) (print 2))"),
        "(if ready (do (print 1) (print 2)))"
    );
    assert_eq!(expanded("(when-not ready 1)"), "(if ready nil (do 1))");
    assert_eq!(expanded("(if-not ready 1 2)"), "(if ready 2 1)");
    assert_eq!(expanded("(if-not ready 1)"), "(if ready nil 1)");
    assert_eq!(expanded("(cond a 1 b 2 :else 3)"), "(if a 1 (if b 2 3))");
}

#[test]
fn expands_doto_threading_the_object_and_returning_it() {
    let out = expanded("(doto (mk) (f 1) g)");
    assert!(out.contains("let*"), "doto uses let*: {out}");
    assert!(
        out.contains("(f __cljn_doto_1 1)"),
        "threads object first: {out}"
    );
    assert!(out.contains("(g __cljn_doto_1)"), "bare symbol step: {out}");
    assert!(
        out.trim_end().ends_with("__cljn_doto_1)"),
        "returns the object: {out}"
    );
}

#[test]
fn expands_boolean_macros_without_double_evaluation() {
    assert_eq!(expanded("(and)"), "true");
    assert_eq!(expanded("(and x)"), "x");
    assert!(expanded("(and (f) (g))").contains("__cljn_and_1"));
    assert_eq!(expanded("(or)"), "nil");
    assert_eq!(expanded("(or x)"), "x");
    assert!(expanded("(or (f) (g))").contains("__cljn_or_1"));
}

#[test]
fn expands_both_threading_directions_and_symbol_steps() {
    assert_eq!(expanded("(-> 5 inc (- 2))"), "(- (inc 5) 2)");
    assert_eq!(
        expanded("(->> xs (map f) (reduce + 0))"),
        "(reduce + 0 (map f xs))"
    );
}

#[test]
fn preserves_quote_and_expands_inside_collections_and_metadata() {
    assert_eq!(expanded("'(when true 1)"), "(quote (when true 1))");
    assert_eq!(expanded("[(when true 1)]"), "[(if true (do 1))]");
    assert_eq!(expanded("#{(if-not false 1 2)}"), "#{(if false 2 1)}");
    assert_eq!(expanded("{:x (when true 1)}"), "{:x (if true (do 1))}");
    assert_eq!(expanded("^:m (when true 1)"), "^:m (if true (do 1))");
}

#[test]
fn malformed_or_unknown_forms_remain_for_analyzer_diagnostics() {
    assert_eq!(expanded("(cond true)"), "(cond true)");
    assert_eq!(
        expanded("(unknown (when true 1))"),
        "(unknown (if true (do 1)))"
    );
}
