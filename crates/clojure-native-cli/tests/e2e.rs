//! Testes end-to-end do compilador nativo: compila um `.clj` para binário nativo
//! via a CLI `build` e executa o resultado, comparando a saída. Requer um C
//! compiler (`cc`) disponível — parte da matriz de build (specs/TESTING_STRATEGY.md).

use std::process::Command;

fn cli() -> &'static str {
    env!("CARGO_BIN_EXE_clojure-native")
}

fn have_cc() -> bool {
    Command::new(std::env::var("CC").unwrap_or_else(|_| "cc".into()))
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Compila `src` e devolve o stdout do binário nativo resultante.
fn build_and_run(name: &str, src: &str) -> String {
    build_and_run_env(name, src, &[])
}

/// Igual, mas com variáveis de ambiente na execução (ex.: CLJN_GC_STRESS).
fn build_and_run_env(name: &str, src: &str, env: &[(&str, &str)]) -> String {
    let dir = std::env::temp_dir();
    let clj = dir.join(format!("{name}.clj"));
    let exe = dir.join(format!("{name}.bin"));
    std::fs::write(&clj, src).unwrap();

    let out = Command::new(cli())
        .arg("build")
        .arg(&clj)
        .arg("-o")
        .arg(&exe)
        .output()
        .expect("executa build");
    assert!(
        out.status.success(),
        "build falhou: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let mut cmd = Command::new(&exe);
    for (k, v) in env {
        cmd.env(k, v);
    }
    let run = cmd.output().expect("executa binário nativo");
    assert!(run.status.success(), "binário retornou erro");
    let _ = std::fs::remove_file(&clj);
    let _ = std::fs::remove_file(&exe);
    String::from_utf8(run.stdout).unwrap()
}

#[test]
fn compiles_and_runs_hello() {
    if !have_cc() {
        eprintln!("pulando: `cc` indisponível");
        return;
    }
    let src = r#"(ns hello.core)
(defn -main [] (println "Hello from native Clojure"))
(-main)"#;
    assert_eq!(
        build_and_run("cljn_e2e_hello", src),
        "Hello from native Clojure\n"
    );
}

#[test]
fn compiles_recursion_and_arithmetic() {
    if !have_cc() {
        return;
    }
    let src = r#"(ns m.core)
(defn fib [n] (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))
(defn -main []
  (println "fib 10 =" (fib 10))
  (println "sum =" (let [a 20 b 22] (+ a b))))
(-main)"#;
    assert_eq!(
        build_and_run("cljn_e2e_fib", src),
        "fib 10 = 55\nsum = 42\n"
    );
}

#[test]
fn compiles_strings_and_lists() {
    if !have_cc() {
        return;
    }
    let src = r#"(ns s.core)
(defn upto [n acc] (if (< n 0) acc (upto (dec n) (cons n acc))))
(defn -main []
  (println (str "soma=" (+ 1 2 3)))
  (println (upto 4 (list)))
  (println (count (upto 4 (list))))
  (println (first (upto 4 (list))))
  (println (= (list 1 2) (cons 1 (cons 2 (list))))))
(-main)"#;
    assert_eq!(
        build_and_run("cljn_e2e_strlist", src),
        "soma=6\n(0 1 2 3 4)\n5\n0\ntrue\n"
    );
}

#[test]
fn compiles_loop_recur() {
    if !have_cc() {
        return;
    }
    // `conta` faz 1_000_000 de iterações via recur-para-fn: se fosse recursão
    // nativa estouraria a pilha; passar prova que recur é um backedge de loop.
    let src = r#"(ns l.core)
(defn soma [n] (loop [i 0 acc 0] (if (> i n) acc (recur (inc i) (+ acc i)))))
(defn conta [n acc] (if (= n 0) acc (recur (dec n) (inc acc))))
(defn -main []
  (println (soma 100))
  (println (conta 1000000 0)))
(-main)"#;
    assert_eq!(build_and_run("cljn_e2e_loop", src), "5050\n1000000\n");
}

#[test]
fn fixnum_fast_paths_correct() {
    if !have_cc() {
        return;
    }
    // Correção dos fast paths: negativos, zero, comparações, inc/dec, mistura.
    let src = r#"(ns n.core)
(defn -main []
  (println (+ 2 3) (+ -5 2) (- 10 3 4) (inc -1) (dec 0))
  (println (< 1 2) (< 2 1) (<= 3 3) (> 5 4) (>= 4 5))
  (loop [i 0 acc 0] (if (< i 1000) (recur (inc i) (+ acc i)) (println "soma:" acc))))
(-main)"#;
    let expected = "5 -3 3 0 -1\ntrue false true true false\nsoma: 499500\n";
    assert_eq!(build_and_run("cljn_e2e_fix", src), expected);
    // sob GC-stress (fast path não deve interferir no rooting dos temporários)
    assert_eq!(
        build_and_run_env("cljn_e2e_fix_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiles_protocols() {
    if !have_cc() {
        return;
    }
    let src = r#"(ns p.core)
(defprotocol Forma (area [this]) (nome [this]))
(defrecord Circulo [r])
(defrecord Retangulo [w h])
(extend-type Circulo Forma (area [this] (* 3 (* (:r this) (:r this)))) (nome [this] "circ"))
(extend-type Retangulo Forma (area [this] (* (:w this) (:h this))) (nome [this] "ret"))
(extend-type List Forma (area [this] (count this)) (nome [this] "lst"))
(defn -main []
  (println (nome (->Circulo 10)) (area (->Circulo 10)))
  (println (nome (->Retangulo 4 5)) (area (->Retangulo 4 5)))
  (println (nome (list 1 2 3)) (area (list 1 2 3)))
  (println (map area (list (->Circulo 1) (->Retangulo 2 3) (list 9 9)))))
(-main)"#;
    let expected = "circ 300\nret 20\nlst 3\n(3 6 2)\n";
    assert_eq!(build_and_run("cljn_e2e_proto", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_proto_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiles_records() {
    if !have_cc() {
        return;
    }
    let src = r#"(ns r.core)
(defrecord Point [x y])
(defn -main []
  (let [p (->Point 3 4)]
    (println p)
    (println (:x p) (:y p) (count p))
    (println (assoc p :x 10))
    (println (= p (->Point 3 4)) (= p (->Point 9 9)))))
(-main)"#;
    let expected = "#Point{:x 3, :y 4}\n3 4 2\n#Point{:x 10, :y 4}\ntrue false\n";
    assert_eq!(build_and_run("cljn_e2e_rec", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_rec_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiles_variadic_multiarity_apply() {
    if !have_cc() {
        return;
    }
    let src = r#"(ns v.core)
(defn lista [a & mais] (cons a mais))
(defn saudar
  ([] "oi")
  ([n] (str "oi " n))
  ([s n] (str s " " n)))
(defn soma3 [a b c] (+ a b c))
(defn sum-all [& xs] (reduce + 0 xs))
(defn -main []
  (println (lista 1 2 3 4) (lista 9))
  (println (saudar) (saudar "a") (saudar "b" "c"))
  (println (apply soma3 (list 1 2 3)) (apply soma3 10 (list 20 30)))
  (println (apply sum-all (range 5)) (apply soma3 [7 8 9])))
(-main)"#;
    let expected = "(1 2 3 4) (9)\noi oi a b c\n6 60\n10 24\n";
    assert_eq!(build_and_run("cljn_e2e_var", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_var_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiled_stdlib_available() {
    if !have_cc() {
        return;
    }
    // map/filter/reduce/range/into/mapv vêm do core.clj compilável, sem o usuário
    // defini-los; `inc`/`+`/`even?` passados como valores (wrappers de primitiva).
    let src = r#"(ns s.core)
(defn -main []
  (println (map inc (range 5)))
  (println (filter even? (range 10)))
  (println (reduce + 0 (map (fn [x] (* x x)) (range 6))))
  (println (into [] (map inc (range 4))))
  (println ((comp inc inc) 10)))
(-main)"#;
    let expected = "(1 2 3 4 5)\n(0 2 4 6 8)\n55\n[1 2 3 4]\n12\n";
    assert_eq!(build_and_run("cljn_e2e_std", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_std_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiles_collections() {
    if !have_cc() {
        return;
    }
    // Vetores, mapas (array-map), sets, keywords. Rodado também sob GC-stress:
    // valida o rooting de acumuladores intermediários (keys/vals/rest).
    let src = r#"(ns c.core)
(defn -main []
  (println [1 2 3 (+ 2 2)])
  (println (assoc [1 2 3] 1 99) (nth [10 20 30] 2) (conj [1 2] 3))
  (let [m {:a 1 :b 2}]
    (println (get m :a) (:b m) (assoc m :c 3) (dissoc m :a) (keys m) (vals m) (contains? m :b)))
  (let [s #{1 2 3 2 1}]
    (println s (count s) (conj s 4) (contains? s 2)))
  (println (= [1 2 3] [1 2 3]) (= {:a 1} {:a 1}) (= #{1 2} #{2 1})))
(-main)"#;
    let expected = "[1 2 3 4]\n[1 99 3] 30 [1 2 3]\n1 2 {:a 1, :b 2, :c 3} {:b 2} (:a :b) (1 2) true\n#{1 2 3} 3 #{1 2 3 4} true\ntrue true true\n";
    assert_eq!(build_and_run("cljn_e2e_coll", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_coll_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiles_closures_and_hof() {
    if !have_cc() {
        return;
    }
    let src = r#"(ns c.core)
(defn my-map [f coll] (if (empty? coll) (list) (cons (f (first coll)) (my-map f (rest coll)))))
(defn adder [n] (fn [x] (+ x n)))
(defn dobro [x] (* x 2))
(defn -main []
  (println (my-map (fn [x] (* x x)) (list 1 2 3 4)))
  (println (my-map dobro (list 1 2 3)))
  (let [add5 (adder 5)] (println (add5 10)))
  (println (((fn [a] (fn [b] (+ a b))) 3) 4)))
(-main)"#;
    let expected = "(1 4 9 16)\n(2 4 6)\n15\n7\n";
    // roda em modo normal e sob GC-stress (valida tracing das capturas)
    assert_eq!(build_and_run("cljn_e2e_clos", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_clos_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiles_core_macros() {
    if !have_cc() {
        return;
    }
    // when/cond/and/or/-> expandidos no caminho compilado (ADR-0004).
    let src = r#"(ns m.core)
(defn sinal [n] (cond (< n 0) "neg" (= n 0) "zero" :else "pos"))
(defn -main []
  (println (sinal -3) (sinal 0) (sinal 9))
  (when (and (> 5 3) (< 2 4)) (println "ok"))
  (println (or false nil 42))
  (println (-> 10 inc inc)))
(-main)"#;
    assert_eq!(
        build_and_run("cljn_e2e_macros", src),
        "neg zero pos\nok\n42\n12\n"
    );
}

#[test]
fn gc_correctness_under_stress() {
    if !have_cc() {
        return;
    }
    // CLJN_GC_STRESS=1 coleta a CADA alocação: se algum valor vivo não estiver
    // rooteado no shadow-stack, seria liberado → saída errada/crash. Passar prova
    // que o rooting preciso está correto.
    let src = r#"(ns g.core)
(defn upto [n acc] (if (< n 0) acc (upto (dec n) (cons n acc))))
(defn -main []
  (println (upto 20 (list)))
  (println (count (upto 20 (list))))
  (println (str "x=" (first (upto 20 (list))) " len=" (count (upto 20 (list))))))
(-main)"#;
    let expected = "(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20)\n21\nx=0 len=21\n";
    assert_eq!(
        build_and_run_env("cljn_e2e_gc_stress", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn gc_reclaims_loop_garbage() {
    if !have_cc() {
        return;
    }
    // Aloca ~2M cons descartáveis em loop. Sem coletor cresceria sem limite;
    // com o mark-sweep completa com memória limitada (aqui só exigimos correção).
    let src = r#"(ns b.core)
(defn burn [n] (loop [i 0] (if (< i n) (do (count (cons i (list))) (recur (inc i))) i)))
(defn -main [] (println (burn 2000000)))
(-main)"#;
    assert_eq!(build_and_run("cljn_e2e_gc_burn", src), "2000000\n");
}

#[test]
fn native_binary_has_no_jvm_dep() {
    if !have_cc() || !cfg!(target_os = "linux") {
        return;
    }
    let dir = std::env::temp_dir();
    let clj = dir.join("cljn_e2e_dep.clj");
    let exe = dir.join("cljn_e2e_dep.bin");
    std::fs::write(
        &clj,
        "(ns d.core)\n(defn -main [] (println \"x\"))\n(-main)",
    )
    .unwrap();
    let out = Command::new(cli())
        .arg("build")
        .arg(&clj)
        .arg("-o")
        .arg(&exe)
        .output()
        .unwrap();
    assert!(out.status.success());
    let ldd = Command::new("ldd").arg(&exe).output();
    if let Ok(ldd) = ldd {
        let deps = String::from_utf8_lossy(&ldd.stdout).to_lowercase();
        assert!(
            !deps.contains("jvm"),
            "binário não deve depender de JVM: {deps}"
        );
        assert!(
            !deps.contains("java"),
            "binário não deve depender de Java: {deps}"
        );
    }
    let _ = std::fs::remove_file(&clj);
    let _ = std::fs::remove_file(&exe);
}
