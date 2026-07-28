//! Behavioral tests for bootstrap evaluation, macro expansion, and primitives.

use super::*;

fn eval_str(src: &str) -> Value {
    let mut it = Interp::with_core().expect("core deve carregar");
    it.eval_source("test", src).expect("deve avaliar")
}

fn eval_out(src: &str) -> String {
    let mut it = Interp::with_core().expect("core deve carregar");
    it.eval_source("test", src).expect("deve avaliar");
    it.take_output()
}

fn eval_err(src: &str) -> EvalError {
    let mut it = Interp::with_core().expect("core deve carregar");
    it.eval_source("test", src).expect_err("deveria falhar")
}

#[test]
fn literals_and_arith() {
    assert_eq!(eval_str("42"), Value::Int(42));
    assert_eq!(eval_str("(+ 1 2 3)"), Value::Int(6));
    assert_eq!(eval_str("(* 2 3 4)"), Value::Int(24));
    assert_eq!(eval_str("(- 10 3 2)"), Value::Int(5));
    assert_eq!(eval_str("(+ 1 2.0)"), Value::Float(3.0));
}

#[test]
fn overflow_is_error() {
    let mut it = Interp::with_core().unwrap();
    let e = it
        .eval_source("t", "(* 9223372036854775807 2)")
        .unwrap_err();
    assert!(e.msg.contains("overflow"), "{}", e.msg);
}

#[test]
fn if_let_fn_closures() {
    assert_eq!(eval_str("(if true 1 2)"), Value::Int(1));
    assert_eq!(eval_str("(if nil 1 2)"), Value::Int(2));
    assert_eq!(eval_str("(if false 1 2)"), Value::Int(2));
    assert_eq!(eval_str("(let [a 1 b 2] (+ a b))"), Value::Int(3));
    assert_eq!(eval_str("((fn [x] (* x x)) 5)"), Value::Int(25));
    // A closure retains its defining lexical environment.
    assert_eq!(
        eval_str("(let [n 10] ((fn [x] (+ x n)) 5))"),
        Value::Int(15)
    );
}

#[test]
fn defn_and_recursion() {
    let src = "(defn fact [n] (if (<= n 1) 1 (* n (fact (- n 1))))) (fact 5)";
    assert_eq!(eval_str(src), Value::Int(120));
}

#[test]
fn loop_recur() {
    let src = "(loop [i 0 acc 0] (if (< i 5) (recur (+ i 1) (+ acc i)) acc))";
    assert_eq!(eval_str(src), Value::Int(10));
}

#[test]
fn recur_tail_no_overflow() {
    // Recur is an interpreter loop and does not consume the Rust call stack.
    let src = "(defn cnt [n acc] (if (zero? n) acc (recur (- n 1) (+ acc 1)))) (cnt 100000 0)";
    assert_eq!(eval_str(src), Value::Int(100000));
}

#[test]
fn core_higher_order() {
    assert_eq!(eval_str("(reduce + 0 (list 1 2 3 4 5))"), Value::Int(15));
    assert_eq!(
        eval_str("(reduce + 0 (map (fn [x] (* x x)) (list 1 2 3)))"),
        Value::Int(14)
    );
    assert_eq!(eval_str("(sum (range 5))"), Value::Int(10));
    assert_eq!(eval_str("(count-if even? (range 10))"), Value::Int(5));
}

#[test]
fn macros_cond_and_or_thread() {
    assert_eq!(eval_str("(cond false 1 true 2 :else 3)"), Value::Int(2));
    assert_eq!(eval_str("(cond false 1 :else 3)"), Value::Int(3));
    assert_eq!(eval_str("(and 1 2 3)"), Value::Int(3));
    assert_eq!(eval_str("(and 1 nil 3)"), Value::Nil);
    assert_eq!(eval_str("(or nil false 7)"), Value::Int(7));
    assert_eq!(eval_str("(-> 5 (- 2) (* 10))"), Value::Int(30));
    assert_eq!(
        eval_str("(->> (range 5) (map inc) (reduce + 0))"),
        Value::Int(15)
    );
}

#[test]
fn collections() {
    assert_eq!(eval_str("(count [1 2 3])"), Value::Int(3));
    assert_eq!(eval_str("(get {:a 1 :b 2} :b)"), Value::Int(2));
    assert_eq!(eval_str("(:a {:a 1})"), Value::Int(1));
    assert_eq!(eval_str("(conj [1 2] 3)"), eval_str("[1 2 3]"));
    assert_eq!(eval_str("(assoc {:a 1} :b 2)"), eval_str("{:a 1 :b 2}"));
    assert_eq!(eval_str("(first [1 2 3])"), Value::Int(1));
}

#[test]
fn hello_world_prints() {
    let src = r#"(ns hello.core)
(defn -main [] (println "Hello from native Clojure"))
(-main)"#;
    assert_eq!(eval_out(src), "Hello from native Clojure\n");
}

#[test]
fn errors_have_span() {
    let mut it = Interp::with_core().unwrap();
    let e = it.eval_source("t", "(+ 1 undefined-thing)").unwrap_err();
    assert!(e.msg.contains("não resolvido"), "{}", e.msg);
    assert!(e.span.is_some());
}

#[test]
fn quote_yields_data() {
    assert_eq!(eval_str("(count '(1 2 3))"), Value::Int(3));
    assert_eq!(eval_str("(first '(a b c))"), eval_str("'a"));
}

#[test]
fn arithmetic_comparisons_and_numeric_errors() {
    assert_eq!(eval_str("(- 7)"), Value::Int(-7));
    assert_eq!(eval_str("(/ 8 2 2)"), Value::Int(2));
    assert_eq!(eval_str("(quot 17 5)"), Value::Int(3));
    assert_eq!(eval_str("(rem 17 5)"), Value::Int(2));
    assert_eq!(eval_str("(mod -3 5)"), Value::Int(2));
    assert_eq!(eval_str("(max -2 7 3)"), Value::Int(7));
    assert_eq!(eval_str("(min 2.5 1 3)"), Value::Int(1));
    assert_eq!(eval_str("(abs -9)"), Value::Int(9));
    assert_eq!(eval_str("(== 1 1.0)"), Value::Bool(true));
    assert_eq!(eval_str("(< 1 2 3)"), Value::Bool(true));
    assert_eq!(eval_str("(>= 3 3 2)"), Value::Bool(true));

    assert!(eval_err("(/ 3 2)").msg.contains("ratios"));
    assert!(eval_err("(/ 1 0)").msg.contains("zero"));
    assert!(eval_err("(+ 1 :x)").msg.contains("número"));
    assert!(eval_err("(min)").msg.contains("ao menos"));
}

#[test]
fn predicates_cover_scalars_collections_and_functions() {
    let src = r#"[
      (nil? nil) (some? 0) (true? true) (false? false)
      (zero? 0.0) (pos? 2) (neg? -2) (even? 4) (odd? 5)
      (int? 1) (string? "s") (keyword? :k) (symbol? 's)
      (map? {}) (vector? []) (list? '()) (set? #{})
      (fn? +) (coll? [1]) (empty? "")]"#;
    assert_eq!(
        eval_str(src),
        Value::Vector(std::rc::Rc::new(vec![Value::Bool(true); 20]))
    );
}

#[test]
fn collection_constructors_and_accessors() {
    assert_eq!(eval_str("(vec '(1 2 3))"), eval_str("[1 2 3]"));
    assert_eq!(eval_str("(hash-set 1 2 1 3)"), eval_str("#{1 2 3}"));
    assert_eq!(
        eval_str("(hash-map :a 1 :b 2 :a 3)"),
        eval_str("{:a 3 :b 2}")
    );
    assert_eq!(eval_str("(rest [1 2 3])"), eval_str("'(2 3)"));
    assert_eq!(eval_str("(next [1])"), Value::Nil);
    assert_eq!(eval_str("(cons 1 [2 3])"), eval_str("'(1 2 3)"));
    assert_eq!(eval_str("(conj nil 1 2)"), eval_str("'(2 1)"));
    assert_eq!(eval_str("(nth [10 20] 9 99)"), Value::Int(99));
    assert_eq!(eval_str("(get [10 20] 1)"), Value::Int(20));
    assert_eq!(eval_str("(get #{1 2} 2)"), Value::Int(2));
    assert_eq!(eval_str("(contains? [10 20] 1)"), Value::Bool(true));
    assert_eq!(eval_str("(keys nil)"), Value::Nil);
    assert_eq!(eval_str("(vals {:a 1 :b 2})"), eval_str("'(1 2)"));
}

#[test]
fn assoc_supports_maps_vectors_and_reports_bounds() {
    assert_eq!(eval_str("(assoc nil :a 1)"), eval_str("{:a 1}"));
    assert_eq!(eval_str("(assoc [1 2] 1 9 2 10)"), eval_str("[1 9 10]"));
    assert!(eval_err("(assoc [1] 3 9)").msg.contains("limites"));
    assert!(eval_err("(assoc [1] :bad 9)").msg.contains("índice"));
    assert!(eval_err("(assoc 1 :a 2)").msg.contains("não suportado"));
    assert!(eval_err("(hash-map :a)").msg.contains("par"));
}

#[test]
fn names_ranges_and_sequence_errors() {
    assert_eq!(eval_str("(name :algo/item)"), Value::str("item"));
    assert_eq!(eval_str("(name 'algo/item)"), Value::str("item"));
    assert_eq!(eval_str("(keyword \"ready\")"), eval_str(":ready"));
    assert_eq!(eval_str("(symbol \"ready\")"), eval_str("'ready"));
    assert_eq!(eval_str("(range 2 8 2)"), eval_str("'(2 4 6)"));
    assert_eq!(eval_str("(range 5 0 -2)"), eval_str("'(5 3 1)"));
    assert!(eval_err("(range)").msg.contains("infinito"));
    assert!(eval_err("(range 0 3 0)").msg.contains("step"));
    assert!(eval_err("(vec 42)").msg.contains("sequenciável"));
    assert!(eval_err("(nth [1] -1)").msg.contains("limites"));
}

#[test]
fn print_variants_and_core_sequence_functions() {
    assert_eq!(
        eval_out(r#"(print "a" 1) (pr "b") (prn "c")"#),
        "a 1\"b\"\"c\"\n"
    );
    assert_eq!(eval_str("(reverse [1 2 3])"), eval_str("'(3 2 1)"));
    assert_eq!(eval_str("(filter odd? (range 6))"), eval_str("'(1 3 5)"));
    assert_eq!(eval_str("(remove even? (range 6))"), eval_str("'(1 3 5)"));
    assert_eq!(eval_str("(take 3 (range 10))"), eval_str("'(0 1 2)"));
    assert_eq!(eval_str("(drop 3 (range 6))"), eval_str("'(3 4 5)"));
    assert_eq!(eval_str("(every? pos? [1 2 3])"), Value::Bool(true));
    assert_eq!(eval_str("(some even? [1 3 4 5])"), Value::Int(4));
}
