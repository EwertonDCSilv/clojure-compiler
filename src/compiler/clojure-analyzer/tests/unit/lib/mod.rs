//! Unit tests for lib.rs.

use super::*;

fn prog(src: &str) -> Program {
    let forms = clojure_reader::read_all(0, src).expect("lê");
    analyze(&forms).expect("analisa")
}
fn err(src: &str) -> Diagnostics {
    let forms = clojure_reader::read_all(0, src).expect("lê");
    analyze(&forms).unwrap_err()
}

#[test]
fn analyzes_fn_and_main() {
    let p = prog("(ns h.core)\n(defn fib [n] (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))\n(defn -main [] (println (fib 10)))\n(-main)");
    assert_eq!(p.functions.iter().filter(|f| !f.is_lambda).count(), 2);
    assert_eq!(p.main_body.len(), 1);
}

#[test]
fn closure_captures() {
    // `adder` captures `n`, so its generated lambda has one capture.
    let p = prog("(defn adder [n] (fn [x] (+ x n)))");
    let lam = p.functions.iter().find(|f| f.is_lambda).unwrap();
    assert_eq!(lam.methods[0].params, vec!["x"]);
    // The MakeFn inside `adder` must materialize exactly one capture.
    if let Ast::MakeFn {
        captures, arity, ..
    } = &p
        .functions
        .iter()
        .find(|f| f.name == "user__adder")
        .unwrap()
        .methods[0]
        .body
    {
        assert_eq!(*arity, 1);
        assert_eq!(captures.len(), 1);
        assert!(matches!(captures[0], Ast::Local(_)));
    } else {
        panic!("corpo de adder deveria ser MakeFn");
    }
}

#[test]
fn higher_order_call_value() {
    let p = prog("(defn ap [f x] (f x))");
    let ap = p.functions.iter().find(|f| f.name == "user__ap").unwrap();
    assert!(matches!(ap.methods[0].body, Ast::CallValue { .. }));
}

#[test]
fn fn_as_value_is_fnref() {
    let p = prog("(defn inc1 [x] (+ x 1))\n(defn use [] (ap inc1 5))\n(defn ap [f x] (f x))");
    let usef = p.functions.iter().find(|f| f.name == "user__use").unwrap();
    if let Ast::Call { args, .. } = &usef.methods[0].body {
        assert!(matches!(args[0], Ast::FnRef(_)));
    } else {
        panic!("esperava Call");
    }
}

#[test]
fn macros_expand() {
    let p = prog("(defn f [n] (cond (< n 0) -1 :else 1))");
    // `cond` expands to nested conditional expressions.
    assert!(matches!(p.functions[0].methods[0].body, Ast::If(..)));
}

#[test]
fn unresolved_is_error() {
    assert_eq!(err("(defn f [] (nope 1))").items[0].code, "E0101");
}
