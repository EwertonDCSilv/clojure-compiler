//! End-to-end contracts for native compilation.
//!
//! The tests invoke the public `build` command on temporary `.clj` sources,
//! execute the resulting host binaries, and compare observable output. They
//! require the C compiler driver selected by `CC`, as specified by
//! `specs/TESTING_STRATEGY.md`.

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

/// Builds `src`, runs the resulting native program, and returns its stdout.
fn build_and_run(name: &str, src: &str) -> String {
    build_and_run_with_options(name, src, &[], &[])
}

/// Builds and runs `src` with additional runtime environment variables.
fn build_and_run_env(name: &str, src: &str, env: &[(&str, &str)]) -> String {
    build_and_run_with_options(name, src, &[], env)
}

/// Builds and runs `src` with native command-line arguments.
fn build_and_run_argv(name: &str, src: &str, argv: &[&str]) -> String {
    let dir = std::env::temp_dir();
    let clj = dir.join(format!("{name}.clj"));
    let exe = dir.join(format!("{name}.bin"));
    std::fs::write(&clj, src).unwrap();
    let mut build = Command::new(cli());
    build.arg("build").arg(&clj).arg("-o").arg(&exe);
    let out = build.output().expect("executa build");
    assert!(
        out.status.success(),
        "build falhou: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let mut cmd = Command::new(&exe);
    cmd.args(argv);
    let run = cmd.output().expect("executa binário nativo");
    assert!(run.status.success(), "binário retornou erro");
    let _ = std::fs::remove_file(&clj);
    let _ = std::fs::remove_file(&exe);
    String::from_utf8(run.stdout).unwrap()
}

/// Builds a multi-file project: writes `(rel_path, content)` files under a fresh
/// temp root, then builds `entry` (relative to the root) with `--source-path root`
/// and runs the resulting binary (ADR-0013 Gate 1 static module loader).
fn build_and_run_project(name: &str, files: &[(&str, &str)], entry: &str) -> String {
    let root = std::env::temp_dir().join(format!("cljn_proj_{name}"));
    let _ = std::fs::remove_dir_all(&root);
    for (rel, content) in files {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(&p, content).unwrap();
    }
    let exe = std::env::temp_dir().join(format!("{name}.bin"));
    let out = Command::new(cli())
        .arg("build")
        .arg(root.join(entry))
        .arg("--source-path")
        .arg(&root)
        .arg("-o")
        .arg(&exe)
        .output()
        .expect("executa build");
    assert!(
        out.status.success(),
        "build falhou: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let run = Command::new(&exe).output().expect("executa binário nativo");
    assert!(run.status.success(), "binário retornou erro");
    let _ = std::fs::remove_dir_all(&root);
    let _ = std::fs::remove_file(&exe);
    String::from_utf8(run.stdout).unwrap()
}

fn build_and_run_with_options(
    name: &str,
    src: &str,
    build_options: &[&str],
    env: &[(&str, &str)],
) -> String {
    let dir = std::env::temp_dir();
    let clj = dir.join(format!("{name}.clj"));
    let exe = dir.join(format!("{name}.bin"));
    std::fs::write(&clj, src).unwrap();

    let mut build = Command::new(cli());
    build.arg("build").arg(&clj).arg("-o").arg(&exe);
    build.args(build_options);
    let out = build.output().expect("executa build");
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
fn supports_every_cranelift_optimization_level() {
    if !have_cc() {
        return;
    }
    let src = "(ns opt.core)\n(defn -main [] (println (+ 20 22)))\n(-main)";
    for (name, level) in [
        ("none", "none"),
        ("speed", "speed"),
        ("size", "speed-and-size"),
    ] {
        assert_eq!(
            build_and_run_with_options(
                &format!("cljn_e2e_opt_{name}"),
                src,
                &["--opt-level", level],
                &[],
            ),
            "42\n"
        );
    }
}

#[test]
fn rejects_invalid_cranelift_optimization_level() {
    let output = Command::new(cli())
        .args(["build", "unused.clj", "--opt-level", "fast"])
        .output()
        .expect("executa build");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("nível de otimização inválido"));
}

#[test]
fn static_module_loader_two_namespace_project() {
    if !have_cc() {
        return;
    }
    // ADR-0013 Gate 1: a two-namespace local project. The entry :require's another
    // namespace resolved from --source-path; aliased and fully-qualified refs work;
    // required namespaces initialize (their top-level defs) before the entry.
    let util = r#"(ns math.util)
(def pi 3)
(defn square [x] (* x x))
(defn area [r] (* pi (square r)))"#;
    let app = r#"(ns app.core (:require [math.util :as m]))
(def label "r=")
(defn -main []
  (println (m/square 5) math.util/pi)
  (println label (m/area 2)))
(-main)"#;
    let expected = "25 3\nr= 12\n";
    assert_eq!(
        build_and_run_project(
            "two_ns",
            &[("math/util.clj", util), ("app.clj", app)],
            "app.clj",
        ),
        expected
    );
}

#[test]
fn builtin_stdlib_bundle_resolves_cljn_namespaces() {
    if !have_cc() {
        return;
    }
    // ADR-0013 Gate 1 §8 / Gate 2: compiler-owned cljn.* sources resolve from the
    // embedded built-in bundle, ahead of --source-path, with no local file.
    let app = r#"(ns app.core (:require [cljn.http.response :as resp]))
(defn -main []
  (let [r (resp/ok "hi")]
    (println (get r :status) (get r :body)))
  (println (get (resp/not-found) :status))
  (println (get (resp/respond 204) :status)))
(-main)"#;
    let expected = "200 hi\n404\n204\n";
    assert_eq!(
        build_and_run_project("builtin_bundle", &[("app.clj", app)], "app.clj"),
        expected
    );
}

#[test]
fn static_module_loader_rejects_dependency_cycle() {
    if !have_cc() {
        return;
    }
    // ADR-0013 Gate 1: a require cycle is a build error, not a hang or crash.
    let root = std::env::temp_dir().join("cljn_proj_cycle");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("p")).unwrap();
    std::fs::write(
        root.join("p/a.clj"),
        "(ns p.a (:require [p.b :as b]))\n(defn fa [] (b/fb))",
    )
    .unwrap();
    std::fs::write(
        root.join("p/b.clj"),
        "(ns p.b (:require [p.a :as a]))\n(defn fb [] (a/fa))",
    )
    .unwrap();
    std::fs::write(
        root.join("main.clj"),
        "(ns main (:require [p.a :as a]))\n(defn -main [] (println 1))\n(-main)",
    )
    .unwrap();
    let out = Command::new(cli())
        .arg("build")
        .arg(root.join("main.clj"))
        .arg("--source-path")
        .arg(&root)
        .arg("-o")
        .arg(std::env::temp_dir().join("cljn_cycle.bin"))
        .output()
        .expect("executa build");
    let _ = std::fs::remove_dir_all(&root);
    assert!(!out.status.success(), "ciclo deveria falhar o build");
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("ciclo de dependência"),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn top_level_def_globals_are_rooted() {
    if !have_cc() {
        return;
    }
    // ADR-0013 Gate 1: initialized top-level `def` data, resolved as a global,
    // initialized once before -main, backed by a permanent GC root. Globals are
    // reachable from functions and survive GC stress.
    let src = r#"(ns d.core)
(def base 40)
(def answer (+ base 2))
(def routes [:a :b :c])
(def config {:port 8080 :name "svc"})
(defn describe [] (str "answer=" answer " port=" (get config :port)))
(defn -main []
  (println base answer)
  (println routes (count routes))
  (println (describe))
  (let [_ (reduce (fn [a i] (cons i a)) (list) (range 500))]
    (println answer (get config :name))))
(-main)"#;
    let expected = "40 42\n[:a :b :c] 3\nanswer=42 port=8080\n42 svc\n";
    assert_eq!(build_and_run("top_level_def", src), expected);
    assert_eq!(
        build_and_run_env("top_level_def_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
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
    // `conta` performs 1,000,000 iterations through function-targeting `recur`.
    // Completion demonstrates that lowering uses a loop backedge, not recursion.
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
fn rooting_elision_preserves_gc_safety() {
    if !have_cc() {
        return;
    }
    // ADR-0006 phases 4-5: root elision for immediates and locals must not drop
    // live heap values. These specification sentinels collect on every allocation.
    let src = r#"(ns re.core)
;; heap `x` vivo enquanto o outro operando aloca
(defn s1 [] (cons (list 1 2 3) (list 4 5 6)))
;; acc heap loop-carried + i imediato: 3000 conses
(defn s2 [] (loop [i 0 acc (list)] (if (< i 3000) (recur (inc i) (cons i acc)) (count acc))))
;; loop numérico puro (sem tráfego de root no corpo)
(defn s3 [n] (loop [i 0 acc 0] (if (< i n) (recur (inc i) (+ acc i)) acc)))
;; closure captura heap, usada após muitas alocações
(defn wrap [lst] (fn [x] (cons x lst)))
;; if de kind misto (heap vs imediato)
(defn s5 [b] (if b (list 1 2) 42))
(defn -main []
  (println (s1))
  (println (s2))
  (println (s3 100000))
  (let [w (wrap (list :a))
        _ (reduce (fn [a i] (cons i a)) (list) (range 1000))]
    (println (w 9)))
  (println (s5 true) (s5 false)))
(-main)"#;
    let expected = "((1 2 3) 4 5 6)\n3000\n4999950000\n(9 :a)\n(1 2) 42\n";
    assert_eq!(build_and_run("cljn_e2e_root", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_root_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn fixnum_fast_paths_correct() {
    if !have_cc() {
        return;
    }
    // Fast-path coverage: negatives, zero, comparisons, inc/dec, and mixtures.
    let src = r#"(ns n.core)
(defn -main []
  (println (+ 2 3) (+ -5 2) (- 10 3 4) (inc -1) (dec 0))
  (println (< 1 2) (< 2 1) (<= 3 3) (> 5 4) (>= 4 5))
  (loop [i 0 acc 0] (if (< i 1000) (recur (inc i) (+ acc i)) (println "soma:" acc))))
(-main)"#;
    let expected = "5 -3 3 0 -1\ntrue false true true false\nsoma: 499500\n";
    assert_eq!(build_and_run("cljn_e2e_fix", src), expected);
    // GC stress ensures fast paths do not interfere with temporary rooting.
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
fn compiles_large_hash_map() {
    if !have_cc() {
        return;
    }
    // Small maps preserve array-map order; larger maps promote to HAMT lookup.
    let src = r#"(ns h.core)
(defn build [n] (loop [i 0 m {}] (if (< i n) (recur (inc i) (assoc m i (* i i))) m)))
(defn -main []
  (println {:a 1 :b 2 :c 3})
  (let [m (build 1000)]
    (println (count m) (get m 0) (get m 999) (get m 5000) (contains? m 500))
    (println (count (dissoc m 500)) (get (dissoc m 500) 500) (reduce + 0 (vals m))))
  (println (= (build 20) (build 20))))
(-main)"#;
    let expected = "{:a 1, :b 2, :c 3}\n1000 0 998001 nil true\n999 nil 332833500\ntrue\n";
    assert_eq!(build_and_run("cljn_e2e_hmap", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_hmap_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiles_large_hash_set() {
    if !have_cc() {
        return;
    }
    // Small sets are arrays (O(n)); `conj` beyond eight entries promotes to HAMT.
    let src = r#"(ns hs.core)
(defn build [n] (loop [i 1 s #{}] (if (<= i n) (recur (inc i) (conj s i)) s)))
(defn -main []
  (println #{1 2 3})
  (let [s (build 1000)]
    (println (count s) (contains? s 500) (contains? s 1000) (contains? s 0) (get s 42) (get s 9999))
    (println (count (conj s 500)) (count (conj s 5000)))
    (println (reduce + 0 (build 100)))
    (println (apply max (build 50)))
    (println (empty? s) (empty? #{})))
  (println (= (build 20) (build 20)) (= (build 20) (build 21))))
(-main)"#;
    let expected =
        "#{1 2 3}\n1000 true true false 42 nil\n1000 1001\n5050\n50\nfalse true\ntrue false\n";
    assert_eq!(build_and_run("cljn_e2e_hset", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_hset_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiles_associative_indexed_dispatch() {
    if !have_cc() {
        return;
    }
    // ADR-0008: variadic `assoc` folds over AssocOne; `nth` supports arity 2/3,
    // not-found and sequential fallbacks, and reserved-name capability dispatch
    // for a type without an inline tag (`Set`).
    let src = r#"(ns ai.core)
(defrecord Pt [x y])
(extend-type Set
  Assoc (-assoc [s k v] (conj s k))
  Indexed (-nth [s i] (+ i 1000))
          (-nth-not-found [s i nf] (if (< i 3) (+ i 2000) nf)))
(defn -main []
  (println (assoc {} :a 1 :b 2 :c 3))
  (println (assoc {:a 1} :a 9 :d 4))
  (let [m {:a 1}] (println (count (assoc m :a 9)) (count (assoc m :z 9))))
  (println (assoc [10 20 30] 1 99 3 40))
  (println (assoc nil :k :v))
  (println (assoc (->Pt 1 2) :x 10))
  (println (nth [10 20 30] 1) (nth (list :a :b :c) 2))
  (println (nth [1 2 3] 9 :nao) (nth (list) 0 :vazio) (nth nil 5 :nnf) (nth nil 3))
  (println (nth [] 0 false) (nth [] 0 (list :h)))
  (println (assoc #{1 2} 3 :x))
  (println (nth #{1 2 3} 5) (nth #{} 1 :cnf) (nth #{} 9 :cnf)))
(-main)"#;
    let expected = "{:a 1, :b 2, :c 3}\n{:a 9, :d 4}\n1 2\n[10 99 30 40]\n{:k :v}\n\
#Pt{:x 10, :y 2}\n20 :c\n:nao :vazio :nnf nil\nfalse (:h)\n#{1 2 3}\n1005 2001 :cnf\n";
    assert_eq!(build_and_run("cljn_e2e_ai", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_ai_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiles_transients() {
    if !have_cc() {
        return;
    }
    // `transient`/`persistent!` and mutating collection operations support batch
    // construction. Vectors share trie structure; maps and sets use a value box.
    let src = r#"(ns tr.core)
(defn bvec [n] (loop [i 0 t (transient [])] (if (< i n) (recur (+ i 1) (conj! t i)) (persistent! t))))
(defn bmap [n] (loop [i 0 t (transient {})] (if (< i n) (recur (+ i 1) (assoc! t i (* i i))) (persistent! t))))
(defn bset [n] (loop [i 0 t (transient #{})] (if (< i n) (recur (+ i 1) (conj! t (mod i 100))) (persistent! t))))
(defn -main []
  (let [v (bvec 1000)] (println (count v) (nth v 0) (nth v 999) (get v 500)))
  (let [t (assoc! (transient [10 20 30]) 1 99)] (println (persistent! (conj! t 40))))
  (let [m (bmap 500)] (println (count m) (get m 20) (get m 999)))
  (println (persistent! (dissoc! (transient {:a 1 :b 2 :c 3}) :b)))
  (let [s (bset 500)] (println (count s) (contains? s 42) (contains? s 200))))
(-main)"#;
    let expected = "1000 0 999 500\n[10 99 30 40]\n500 400 nil\n{:a 1, :c 3}\n100 true false\n";
    assert_eq!(build_and_run("cljn_e2e_trans", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_trans_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn native_io_slurp_spit_getenv() {
    if !have_cc() {
        return;
    }
    // ADR-0007 subset: slurp/spit/file-exists?/getenv use ABI primitives.
    // Operating-system failures become ex-data maps catchable by `try`/`catch`.
    let src = r#"(ns io.core)
(defn -main []
  (spit "/tmp/cljn_e2e_io.txt" "abc\ndef\n")
  (println (file-exists? "/tmp/cljn_e2e_io.txt") (file-exists? "/tmp/cljn_e2e_nope.txt"))
  (println (count (slurp "/tmp/cljn_e2e_io.txt")))
  (println (try (slurp "/tmp/cljn_e2e_nope.txt")
                (catch E e (str (get e :kind) " " (get e :operation) " " (get e :os-code)))))
  (println (getenv "CLJN_E2E_VAR") (getenv "CLJN_E2E_MISSING")))
(-main)"#;
    let expected = "true false\n8\n:not-found :slurp 2\nxyz nil\n";
    assert_eq!(
        build_and_run_env("cljn_e2e_io", src, &[("CLJN_E2E_VAR", "xyz")]),
        expected
    );
    assert_eq!(
        build_and_run_env(
            "cljn_e2e_io_gc",
            src,
            &[("CLJN_E2E_VAR", "xyz"), ("CLJN_GC_STRESS", "1")]
        ),
        expected
    );
}

#[test]
fn with_out_str_captures_and_restores_out() {
    if !have_cc() {
        return;
    }
    // ADR-0007 / IO-0: `*out*` is dynamic. `with-out-str` binds a string writer
    // and restores the prior value both on return and while propagating throws.
    let src = r#"(ns wos.core)
(defn -main []
  (let [s (with-out-str (print "ab") (print 42) (println))]
    (println (count s) (= s "ab42\n")))
  (let [o (with-out-str (print "A") (print (with-out-str (print "BC"))) (print "-D"))]
    (println (= o "ABC-D")))
  (println "normal")
  (println (try (with-out-str (print "x") (throw {:e 1})) (catch E e "caught")))
  (println "still-ok"))
(-main)"#;
    let expected = "5 true\ntrue\nnormal\ncaught\nstill-ok\n";
    assert_eq!(build_and_run("with_out_str", src), expected);
    assert_eq!(
        build_and_run_env("with_out_str_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn binding_rebinds_and_restores_dynamic_vars() {
    if !have_cc() {
        return;
    }
    // ADR-0007 / IO-0: `binding` rebinds dynamic Vars; a Var is readable as a value.
    // Binding restoration preserves normal results and exceptional control flow.
    let src = r#"(ns bind.core)
(defn -main []
  (println *flush-on-newline*)
  (println (binding [*flush-on-newline* false] *flush-on-newline*))
  (println *flush-on-newline*)
  (println (binding [*flush-on-newline* false] (+ 40 2)))
  (println (try (binding [*flush-on-newline* false] (throw {:e 1})) (catch E e :caught)))
  (println *flush-on-newline*))
(-main)"#;
    let expected = "true\nfalse\ntrue\n42\n:caught\ntrue\n";
    assert_eq!(build_and_run("binding_dynvars", src), expected);
    assert_eq!(
        build_and_run_env("binding_dynvars_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn read_line_from_with_in_str() {
    if !have_cc() {
        return;
    }
    // ADR-0007 / IO-0: `*in*` is dynamic; `with-in-str` binds a string reader.
    // `read-line` removes the newline and returns nil at end of input.
    let src = r#"(ns rl.core)
(defn read-all [acc]
  (let [l (read-line)]
    (if (nil? l) acc (recur (conj acc l)))))
(defn -main []
  (println (with-in-str "um\ndois\ntres" (read-all [])))
  (println (with-in-str "a\nb" (read-all [])))
  (println (with-in-str "" (nil? (read-line))))
  (println (with-in-str "\nx" (read-all []))))
(-main)"#;
    let expected = "[um dois tres]\n[a b]\ntrue\n[ x]\n";
    assert_eq!(build_and_run("read_line_wis", src), expected);
    assert_eq!(
        build_and_run_env("read_line_wis_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn char_type_literals_conversion_and_read_char() {
    if !have_cc() {
        return;
    }
    // ADR-0007 / IO-1: Char is a tagged immediate. Cover named and Unicode
    // literals, conversions, predicates, equality, and multibyte UTF-8 reading.
    let src = r#"(ns ch.core)
(defn read-chars [acc]
  (let [c (read-char)]
    (if (nil? c) acc (recur (conj acc c)))))
(defn -main []
  (println (int \A) (int \a) (char 66))
  (println (= \Z (char (int \Z))) (= \a \a) (= \a \b) (= \A 65))
  (println (char? \a) (char? 97) (char? "a"))
  (println (str \h \i \! \space \é))
  (println (with-in-str "aé" (read-chars []))))
(-main)"#;
    let expected = "65 97 B\ntrue true false false\ntrue false false\nhi! é\n[a é]\n";
    assert_eq!(build_and_run("char_type", src), expected);
    assert_eq!(
        build_and_run_env("char_type_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn path_helpers_join_name_parent() {
    if !have_cc() {
        return;
    }
    // ADR-0007 / IO-1: POSIX-style path helpers are purely textual.
    let src = r#"(ns p.core)
(defn -main []
  (println (path-join "/a/b" "c.txt") (path-join "/a/b/" "c") (path-join "x" "y") (path-join "/a" "/abs"))
  (println (file-name "/a/b/c.txt") (file-name "solo"))
  (println (parent "/a/b/c.txt") (parent "solo") (parent "/x"))
  (println (file-name (path-join (parent "/a/b/c.txt") "d.txt"))))
(-main)"#;
    let expected = "/a/b/c.txt /a/b/c x/y /abs\nc.txt solo\n/a/b nil /\nd.txt\n";
    assert_eq!(build_and_run("path_helpers", src), expected);
    assert_eq!(
        build_and_run_env("path_helpers_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn bytes_type_and_binary_io() {
    if !have_cc() {
        return;
    }
    // ADR-0007 / IO-1: byte arrays, binary slurp/spit, `count`, and `bget`;
    // bytes-to-string round trips and OS errors become ex-data.
    let src = r#"(ns by.core)
(defn -main []
  (let [b (bytes "Aé!")]
    (println (count b) (bget b 0) (bget b 1) (= "Aé!" (bytes->string b)) b))
  (spit-bytes "/tmp/cljn_e2e_bytes.bin" (bytes "hello world"))
  (let [r (slurp-bytes "/tmp/cljn_e2e_bytes.bin")]
    (println (count r) (= "hello world" (bytes->string r))))
  (println (try (slurp-bytes "/nao/existe.bin") (catch E e (get e :kind)))))
(-main)"#;
    let expected = "4 65 195 true #bytes[4]\n11 true\n:not-found\n";
    assert_eq!(build_and_run("bytes_io", src), expected);
    assert_eq!(
        build_and_run_env("bytes_io_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn read_string_edn_reader() {
    if !have_cc() {
        return;
    }
    // ADR-0007 / IO-5: runtime `read-string` parses the supported EDN subset.
    let src = r##"(ns rs.core)
(defn -main []
  (println (read-string "42") (read-string "-7") (read-string "nil") (read-string "true"))
  (println (read-string ":kw") (= :abc (read-string ":abc")))
  (println (read-string "[1 [2 {:x [3 4]}] :k \"s\"]"))
  (println (read-string "#{1 2 3}") (read-string "(1 2 3)"))
  (println (= \newline (read-string "\\newline")) (read-string "\\u0041"))
  (let [cfg (read-string "{:port 8080 :hosts [\"a\" \"b\"]}")]
    (println (get cfg :port) (get cfg :hosts)))
  (println (read-string "[1, 2, 3]")))
(-main)"##;
    let expected = "42 -7 nil true\n:kw true\n[1 [2 {:x [3 4]}] :k s]\n#{1 2 3} (1 2 3)\ntrue A\n8080 [a b]\n[1 2 3]\n";
    assert_eq!(build_and_run("read_string_edn", src), expected);
    assert_eq!(
        build_and_run_env("read_string_edn_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn file_streams_and_with_open() {
    if !have_cc() {
        return;
    }
    // ADR-0007 / IO-2/3: file readers/writers and `with-open` close through
    // `try`/`finally`, including exceptions, and integrate with `*out*`/`*in*`.
    let src = r##"(ns wo.core)
(defn read-lines [acc]
  (let [l (read-line)]
    (if (nil? l) acc (recur (conj acc l)))))
(defn -main []
  (with-open [w (writer "/tmp/cljn_e2e_wo.txt")]
    (binding [*out* w] (println "l1") (println "l2") (print "end")))
  (println (with-open [r (reader "/tmp/cljn_e2e_wo.txt")]
             (binding [*in* r] (read-lines []))))
  (println (= "l1\nl2\nend" (slurp "/tmp/cljn_e2e_wo.txt")))
  (println (try (with-open [w (writer "/tmp/cljn_e2e_wo2.txt")]
                  (binding [*out* w] (print "partial") (throw {:e 1})))
                (catch E e "caught")))
  (println (slurp "/tmp/cljn_e2e_wo2.txt"))
  (println (try (reader "/nao/existe.txt") (catch E e (get e :kind)))))
(-main)"##;
    let expected = "[l1 l2 end]\ntrue\ncaught\npartial\n:not-found\n";
    assert_eq!(build_and_run("file_streams", src), expected);
    assert_eq!(
        build_and_run_env("file_streams_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn float_type_arithmetic_and_predicates() {
    if !have_cc() {
        return;
    }
    // IO-1: boxed floats cover mixed arithmetic, division, comparisons,
    // predicates/conversions, printing, and survival across GC allocations.
    let src = r#"(ns f.core)
(defn media [xs] (/ (reduce + 0.0 xs) (count xs)))
(defn -main []
  (println 3.14 1.0 0.5 (- 2.5))
  (println (+ 1 2.0) (* 3 1.5) (- 10 0.5) (+ 1.5 2.5))
  (println (/ 6 2) (/ 7 2) (/ 1.0 4) (/ 10.0 4))
  (println (< 1 1.5) (> 2.0 3) (= 1.0 1.0) (= 1 1.0) (<= 2.5 2.5))
  (println (float? 1.5) (float? 1) (double 5) (int 3.9))
  (println (inc 1.5) (dec 2.5) (media [1.0 2.0 3.0 4.0]))
  (let [x (+ 0.5 0.5) _ (reduce (fn [a i] (cons i a)) (list) (range 400))]
    (println x)))
(-main)"#;
    let expected = "3.14 1.0 0.5 -2.5\n3.0 4.5 9.5 4.0\n3 3.5 0.25 2.5\ntrue false true false true\ntrue false 5.0 3\n2.5 1.5 2.5\n1.0\n";
    assert_eq!(build_and_run("float_type", src), expected);
    assert_eq!(
        build_and_run_env("float_type_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn command_line_args_and_file_metadata() {
    if !have_cc() {
        return;
    }
    // `*command-line-args*` excludes argv[0]; file metadata exposes size and mtime.
    let src = r#"(ns a.core)
(defn -main []
  (println (count *command-line-args*) *command-line-args* (first *command-line-args*))
  (spit "/tmp/cljn_e2e_size.txt" "12345")
  (println (file-size "/tmp/cljn_e2e_size.txt") (> (file-modified "/tmp/cljn_e2e_size.txt") 0))
  (delete-file "/tmp/cljn_e2e_size.txt")
  (println (try (file-size "/nao/existe") (catch E e (get e :kind)))))
(-main)"#;
    assert_eq!(
        build_and_run_argv("cla_args", src, &["alpha", "beta"]),
        "2 [alpha beta] alpha\n5 true\n:not-found\n"
    );
    assert_eq!(
        build_and_run_argv("cla_none", src, &[]),
        "0 [] nil\n5 true\n:not-found\n"
    );
}

#[test]
fn filesystem_operations() {
    if !have_cc() {
        return;
    }
    // ADR-0007 / IO-4: directory creation/listing, deletion, rename, and predicates.
    let src = r#"(ns fs.core)
(defn -main []
  (let [root "/tmp/cljn_e2e_fs"]
    (mkdirs (path-join root "a/b"))
    (println (directory? root) (directory? (path-join root "a/b")) (file? root))
    (spit (path-join root "x.txt") "c")
    (spit (path-join root "y.txt") "d")
    (println (file? (path-join root "x.txt")) (count (list-dir root)))
    (rename (path-join root "x.txt") (path-join root "z.txt"))
    (println (file? (path-join root "x.txt")) (file? (path-join root "z.txt")))
    (delete-file (path-join root "z.txt"))
    (delete-file (path-join root "y.txt"))
    (println (count (list-dir root)))
    (println (try (list-dir "/nao/existe/dir") (catch E e (get e :kind))))
    (delete-file (path-join root "a/b"))
    (delete-file (path-join root "a"))
    (delete-file root)
    (println (directory? root))))
(-main)"#;
    let expected = "true true false\ntrue 3\nfalse true\n1\n:not-found\nfalse\n";
    assert_eq!(build_and_run("fs_ops", src), expected);
    assert_eq!(
        build_and_run_env("fs_ops_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn constant_vector_literals_are_hoisted() {
    if !have_cc() {
        return;
    }
    // ADR-0009: vector literals containing only immediates are immutable, so one
    // cached object is reused. Transient copy-on-write, reads, and equality must
    // make reuse observationally transparent.
    let src = r#"(ns ch.core)
(defn fresh [] [10 20 30])
(defn fill [] (loop [i 0 v [0 0 0]] (if (< i 6) (recur (inc i) (assoc v (mod i 3) (+ (nth v (mod i 3)) 1))) v)))
(defn -main []
  (println (fresh) (fresh) (= (fresh) [10 20 30]))
  (println (fill))                         ;; transiente sobre o literal cacheado (COW)
  (println (nth [7 8 9] 1))
  (loop [k 0 s 0] (if (< k 3) (recur (inc k) (+ s (nth [100 200 300] k))) (println s))))
(-main)"#;
    let expected = "[10 20 30] [10 20 30] true\n[2 2 2]\n8\n600\n";
    assert_eq!(build_and_run("cljn_e2e_hoist", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_hoist_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn interprocedural_uniqueness_preserves_semantics() {
    if !have_cc() {
        return;
    }
    // ADR-0010: a loop accumulator passed to a linear helper and returned from it
    // is threaded as a transient. Uniqueness analysis must cancel after aliasing
    // or a non-linear helper call.
    let src = r#"(ns iu.core)
;; helper linear (consome e devolve o acumulador) — deve threadar o transiente
(defn step [acc lim] (loop [i 0 r acc] (if (>= i lim) r (recur (inc i) (assoc r (mod i 4) (+ (nth r (mod i 4)) i))))))
(defn work [] (loop [k 0 acc [0 0 0 0]] (if (>= k 3) acc (recur (inc k) (step acc 8)))))
;; ADVERSARIAL: acumulador guardado (aliased) → NÃO pode mutar in-place
(defn snaps [] (loop [i 0 v [] s (list)] (if (>= i 4) (map count s) (recur (inc i) (conj v i) (cons v s)))))
;; make devolve o acumulador inteiro; dois usos independentes
(defn make [n] (loop [i 0 v []] (if (< i n) (recur (inc i) (conj v i)) v)))
(defn -main []
  (println (work))
  (println (snaps))
  (let [a (make 3) b (make 5)] (println a b)))
(-main)"#;
    let expected = "[12 18 24 30]\n(3 2 1 0)\n[0 1 2] [0 1 2 3 4]\n";
    assert_eq!(build_and_run("cljn_e2e_uniq", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_uniq_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn core_vector_builders_use_transients() {
    if !have_cc() {
        return;
    }
    // `mapv` and `into` use structural transients for linear core accumulators.
    // Semantics remain identical; `into` stays generic for vectors and boxed lists.
    let src = r#"(ns cv.core)
(defn -main []
  (println (mapv (fn [x] (* x x)) (range 8)))
  (println (into [] (list 1 2 3)))
  (println (into [10 20] (range 3)))
  (println (into (list) (range 3)))
  (println (count (mapv (fn [x] x) (range 1000)))))
(-main)"#;
    let expected = "[0 1 4 9 16 25 36 49]\n[1 2 3]\n[10 20 0 1 2]\n(2 1 0)\n1000\n";
    assert_eq!(build_and_run("cljn_e2e_corevec", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_corevec_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn auto_transient_loop_accumulators() {
    if !have_cc() {
        return;
    }
    // ADR-0009: linear loop accumulators initialized from vector literals become
    // transients without semantic change. Cover `conj` in `recur` and `if`, an
    // unchanged accumulator, `assoc`, chained escape, fallback, and read access.
    let src = r#"(ns at.core)
(defn buildv [n] (loop [i 0 v []] (if (< i n) (recur (+ i 1) (conj v (* i i))) v)))
(defn evens [n] (loop [i 0 v []] (if (< i n) (recur (+ i 1) (if (= 0 (mod i 2)) (conj v i) v)) v)))
(defn zero-init [n] (loop [i 0 v [0 0 0 0 0]] (if (< i n) (recur (+ i 1) (assoc v (mod i 5) (+ (nth v (mod i 5)) 1))) v)))
(defn -main []
  (let [v (buildv 100)] (println (count v) (nth v 0) (nth v 99)))
  (let [e (evens 10)] (println e (count e)))
  (let [h (zero-init 23)] (println h))
  ;; escape encadeado (padrão do sieve initial-flags)
  (println (loop [i 0 v []] (if (< i 5) (recur (+ i 1) (conj v i)) (assoc (assoc v 0 :a) 4 :z)))))
(-main)"#;
    let expected = "100 0 9801\n[0 2 4 6 8] 5\n[5 5 5 4 4]\n[:a 1 2 3 :z]\n";
    assert_eq!(build_and_run("cljn_e2e_autotrans", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_autotrans_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn structural_transients_preserve_persistence() {
    if !have_cc() {
        return;
    }
    // A structural transient initially shares the persistent vector trie.
    // In-place mutation is limited to owned nodes; shared nodes copy on write.
    // The persistent source and independent transients must never interfere.
    let src = r#"(ns st.core)
(defn build [n] (loop [i 0 v []] (if (< i n) (recur (+ i 1) (conj v (* i 10))) v)))
(defn -main []
  (let [v (build 100)
        t (assoc! (assoc! (transient v) 0 :mudou) 50 :meio)
        v2 (persistent! (conj! t :novo))]
    (println (nth v 0) (nth v 50) (count v))
    (println (nth v2 0) (nth v2 50) (nth v2 100) (count v2)))
  (let [big (build 2000)
        t (persistent! (assoc! (assoc! (transient big) 5 :x) 1500 :y))]
    (println (nth t 5) (nth t 1500) (nth big 5) (nth big 1500)))
  (let [v (build 50)
        a (persistent! (conj! (transient v) :a))
        b (persistent! (conj! (transient v) :b))]
    (println (count v) (count a) (count b) (nth a 50) (nth b 50))))
(-main)"#;
    let expected = "0 500 100\n:mudou :meio :novo 101\n:x :y 50 15000\n50 51 51 :a :b\n";
    assert_eq!(build_and_run("cljn_e2e_strans", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_strans_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiles_multimethods() {
    if !have_cc() {
        return;
    }
    // `defmulti` dispatches on `(dispatch-fn args)` and matches with equality.
    // Cover keyword, numeric, multi-argument, and `:default` dispatch values.
    let src = r#"(ns mm.core)
(defmulti area (fn [s] (get s :shape)))
(defmethod area :circle [s] (* 3 (* (get s :r) (get s :r))))
(defmethod area :square [s] (* (get s :side) (get s :side)))
(defmethod area :default [s] (str "?" (get s :shape)))
(defmulti classify (fn [n] (mod n 2)))
(defmethod classify 0 [n] "par")
(defmethod classify 1 [n] "impar")
(defmulti combine (fn [a b] (get a :op)))
(defmethod combine :add [a b] (+ (get a :v) b))
(defmethod combine :mul [a b] (* (get a :v) b))
(defn -main []
  (println (area {:shape :circle :r 10}) (area {:shape :square :side 5}))
  (println (classify 4) (classify 7))
  (println (combine {:op :add :v 10} 5) (combine {:op :mul :v 10} 5))
  (println (area {:shape :triangle})))
(-main)"#;
    let expected = "300 25\npar impar\n15 50\n?:triangle\n";
    assert_eq!(build_and_run("cljn_e2e_multi", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_multi_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiles_try_catch_finally() {
    if !have_cc() {
        return;
    }
    // `try`/`catch`/`finally`: catch-all binds the thrown value, finally always
    // runs, lexical captures survive, and nested throws reach the outer handler.
    let src = r#"(ns tc.core)
(defn safe-div [a b]
  (try (if (= b 0) (throw "div0") (quot a b))
       (catch Exception e (str "erro: " e))))
(defn -main []
  (println (safe-div 10 2) (safe-div 10 0))
  (println (try 42 (finally (println "fin"))))
  (println (try (throw "boom") (catch T e (str "peguei: " e)) (finally (println "fin2"))))
  (println (try (throw 99) (catch E e (+ e 1))))
  (println (try (try (throw "in") (finally (println "inf"))) (catch E e (str "out: " e))))
  (let [base 100] (println (try (throw base) (catch E e (+ e base))))))
(-main)"#;
    let expected = "5 erro: div0\nfin\n42\nfin2\npeguei: boom\n100\ninf\nout: in\n200\n";
    assert_eq!(build_and_run("cljn_e2e_try", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_try_gc", src, &[("CLJN_GC_STRESS", "1")]),
        expected
    );
}

#[test]
fn compiles_sorted_collections() {
    if !have_cc() {
        return;
    }
    // Ordered LLRB collections iterate deterministically regardless of insertion
    // order. Exercise lookup, membership, updates, sequence, and equality.
    let src = r#"(ns so.core)
(defn build [s i n] (if (> i n) s (recur (conj s (mod (* i 7) 101)) (+ i 1) n)))
(defn -main []
  (let [s (sorted-set 5 3 8 1 9 2)]
    (println s (count s) (first s) (rest s))
    (println (contains? s 8) (contains? s 7) (get s 3) (get s 99)))
  (let [m (sorted-map 3 :c 1 :a 2 :b)]
    (println m (keys m) (vals m))
    (println (assoc m 0 :z))
    (println (dissoc m 2) (count (dissoc m 2))))
  (let [big (build (sorted-set) 1 100)]
    (println (count big) (first big) (reduce + 0 big) (apply max big)))
  (println (compare 3 5) (compare 5 3) (compare 4 4))
  (println (= (sorted-set 1 2 3) #{3 2 1}) (= (sorted-map 1 10 2 20) {2 20 1 10})))
(-main)"#;
    let expected = "#{1 2 3 5 8 9} 6 1 (2 3 5 8 9)\ntrue false 3 nil\n\
{1 :a, 2 :b, 3 :c} (1 2 3) (:a :b :c)\n\
{0 :z, 1 :a, 2 :b, 3 :c}\n\
{1 :a, 3 :c} 2\n\
100 1 5050 100\n\
-1 1 0\n\
true true\n";
    assert_eq!(build_and_run("cljn_e2e_sorted", src), expected);
    assert_eq!(
        build_and_run_env("cljn_e2e_sorted_gc", src, &[("CLJN_GC_STRESS", "1")]),
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
    // Collection functions come from compiled core without user definitions;
    // primitive wrappers allow `inc`, `+`, and `even?` to flow as values.
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
    // Vectors, array maps, sets, and keywords under GC stress validate rooting
    // of intermediate accumulators used by `keys`, `vals`, and `rest`.
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
    // Run normally and under GC stress to validate capture tracing.
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
    // The compiled path expands `when`, `cond`, `and`, `or`, and `->` (ADR-0004).
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
    // CLJN_GC_STRESS=1 collects at every allocation. Any unrooted live value
    // would be reclaimed, producing wrong output or a crash.
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
    // Allocate about two million disposable cons cells. Mark-sweep must reclaim
    // them; this contract checks completion and correctness rather than RSS.
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
