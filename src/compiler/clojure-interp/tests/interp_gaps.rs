//! Closes characterization gaps in the bootstrap interpreter (issue #116): a
//! normal/boundary/error case for special forms and paths not yet covered by
//! `tests/unit/tests/mod.rs` (`var`, `apply`, arity mismatches, unresolved
//! symbols, and malformed binding forms). No new special form or semantics.

use clojure_interp::Interp;
use clojure_value::{List, Value};

fn eval_ok(source: &str) -> Value {
    Interp::with_core()
        .expect("core bootstrap")
        .eval_source("test", source)
        .expect("should evaluate")
}

fn eval_err(source: &str) -> String {
    Interp::with_core()
        .expect("core bootstrap")
        .eval_source("test", source)
        .expect_err("should fail")
        .msg
}

#[test]
fn var_resolves_a_defined_global() {
    assert_eq!(eval_ok("(def x 42) (var x)"), Value::Int(42));
}

#[test]
fn var_on_an_undefined_global_is_an_error() {
    assert_eq!(
        eval_err("(var never-defined)"),
        "var não encontrada: never-defined"
    );
}

#[test]
fn var_requires_a_symbol_argument() {
    assert_eq!(eval_err("(var 1)"), "var requer símbolo");
}

#[test]
fn apply_spreads_the_trailing_sequence() {
    assert_eq!(eval_ok("(apply + 1 2 (list 3 4))"), Value::Int(10));
}

#[test]
fn apply_accepts_an_empty_trailing_sequence() {
    assert_eq!(eval_ok("(apply + (list))"), Value::Int(0));
}

#[test]
fn apply_requires_at_least_a_function_and_one_argument() {
    assert_eq!(
        eval_err("(apply +)"),
        "apply requer fn e ao menos um argumento"
    );
}

#[test]
fn apply_rejects_a_non_sequence_trailing_argument() {
    assert_eq!(
        eval_err("(apply + 1 2)"),
        "apply: último argumento deve ser uma sequência"
    );
}

#[test]
fn user_function_call_with_too_few_arguments_is_an_arity_error() {
    assert_eq!(
        eval_err("(defn f [a b] (+ a b)) (f 1)"),
        "aridade errada: f recebeu 1 args"
    );
}

#[test]
fn user_function_call_with_too_many_arguments_is_an_arity_error() {
    assert_eq!(
        eval_err("(defn f [a b] (+ a b)) (f 1 2 3)"),
        "aridade errada: f recebeu 3 args"
    );
}

#[test]
fn variadic_function_accepts_at_least_its_fixed_parameter_count() {
    assert_eq!(
        eval_ok("(defn f [a & rest] (cons a rest)) (f 1 2 3)"),
        Value::List(List::from_vec(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3)
        ]))
    );
}

#[test]
fn variadic_function_rejects_fewer_than_its_fixed_parameter_count() {
    assert_eq!(
        eval_err("(defn f [a b & rest] a) (f 1)"),
        "aridade errada: f recebeu 1 args"
    );
}

#[test]
fn resolving_an_unbound_symbol_is_an_error() {
    // Any stable, non-empty diagnostic is acceptable; the exact wording is an
    // implementation detail of symbol resolution, not this test's contract.
    assert!(!eval_err("never-bound-symbol").is_empty());
}

#[test]
fn let_requires_a_vector_of_bindings() {
    assert_eq!(eval_err("(let (a 1) a)"), "let requer vetor de bindings");
}

#[test]
fn let_rejects_an_odd_number_of_binding_forms() {
    assert_eq!(eval_err("(let [a 1 b] a)"), "let: bindings em pares");
}

#[test]
fn loop_requires_a_vector_of_bindings() {
    assert_eq!(eval_err("(loop (a 1) a)"), "loop requer vetor de bindings");
}

#[test]
fn def_requires_a_leading_symbol() {
    assert_eq!(eval_err("(def 1 2)"), "def requer um símbolo");
}

#[test]
fn def_without_an_initializer_defaults_to_nil() {
    assert_eq!(eval_ok("(def x) x"), Value::Nil);
}

#[test]
fn redefining_a_global_replaces_its_previous_value() {
    assert_eq!(eval_ok("(def x 1) (def x 2) x"), Value::Int(2));
}

#[test]
fn recur_outside_a_loop_or_fn_is_an_error() {
    assert_eq!(eval_err("(recur 1)"), "`recur` fora de loop/fn");
}
