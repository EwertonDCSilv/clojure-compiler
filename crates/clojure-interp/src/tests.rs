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
    let e = it.eval_source("t", "(* 9223372036854775807 2)").unwrap_err();
    assert!(e.msg.contains("overflow"), "{}", e.msg);
}

#[test]
fn if_let_fn_closures() {
    assert_eq!(eval_str("(if true 1 2)"), Value::Int(1));
    assert_eq!(eval_str("(if nil 1 2)"), Value::Int(2));
    assert_eq!(eval_str("(if false 1 2)"), Value::Int(2));
    assert_eq!(eval_str("(let [a 1 b 2] (+ a b))"), Value::Int(3));
    assert_eq!(eval_str("((fn [x] (* x x)) 5)"), Value::Int(25));
    // closure captura o ambiente
    assert_eq!(eval_str("(let [n 10] ((fn [x] (+ x n)) 5))"), Value::Int(15));
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
    // 100k iterações sem estourar a pilha (recur é loop).
    let src = "(defn cnt [n acc] (if (zero? n) acc (recur (- n 1) (+ acc 1)))) (cnt 100000 0)";
    assert_eq!(eval_str(src), Value::Int(100000));
}

#[test]
fn core_higher_order() {
    assert_eq!(
        eval_str("(reduce + 0 (list 1 2 3 4 5))"),
        Value::Int(15)
    );
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
    assert_eq!(eval_str("(->> (range 5) (map inc) (reduce + 0))"), Value::Int(15));
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
