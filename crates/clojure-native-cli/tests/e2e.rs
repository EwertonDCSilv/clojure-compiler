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
    build_and_run_with_options(name, src, &[], &[])
}

/// Igual, mas com variáveis de ambiente na execução (ex.: CLJN_GC_STRESS).
fn build_and_run_env(name: &str, src: &str, env: &[(&str, &str)]) -> String {
    build_and_run_with_options(name, src, &[], env)
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
fn rooting_elision_preserves_gc_safety() {
    if !have_cc() {
        return;
    }
    // ADR-0006 Fases 4-5: a elisão de root para imediatos/Locals não pode soltar
    // valores heap vivos. Casos sentinela do spec, sob coleta a cada alocação.
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
fn compiles_large_hash_map() {
    if !have_cc() {
        return;
    }
    // Mapa pequeno preserva ordem (array-map); grande promove a HAMT (O(log n)).
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
    // Set pequeno é array (O(n)); grande promove a HAMT (O(log n)) via conj > 8 elems.
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
    // ADR-0008: assoc variádico (dobra sobre AssocOne), nth aridade 2/3 com
    // not-found e fallback sequencial, e capability dispatch para tipo sem tag
    // embutida (Set) via extend-type com nomes reservados.
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
    // transient/persistent!/conj!/assoc!/dissoc!: construção em lote mutável.
    // Vetor transiente é estrutural (compartilha a trie); mapa/set via caixa.
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
fn interprocedural_uniqueness_preserves_semantics() {
    if !have_cc() {
        return;
    }
    // ADR-0010: um acumulador de loop passado a um helper LINEAR (que o consome e
    // devolve) é threaded como transiente; conj/assoc dispatcham sobre T_TVEC em
    // runtime. A análise de unicidade DEVE cancelar quando o acumulador é aliased
    // (guardado noutra estrutura) ou passado a um helper não-linear.
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
    // mapv/into constroem via transiente estrutural (acumulador linear no core).
    // Semântica idêntica; `into` continua genérico (vetor / lista via caixa).
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
    // ADR-0009: acumuladores de loop com init de vetor literal, usados de forma
    // linear, viram transientes automaticamente (semântica idêntica). Cobre os
    // padrões que exercitam o transform: conj no recur, conj num ramo de `if`
    // (regressão do bug de descida), bare-unchanged, assoc, escape encadeado e
    // leituras via nth/count.
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
    // Transient estrutural: `transient` compartilha a trie do vetor persistente e
    // muta in-place só nós próprios (copy-on-write dos compartilhados). O vetor
    // original NUNCA pode ser alterado; transientes independentes não interferem.
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
    // defmulti/defmethod: dispatch por (dispatch-fn args), casado por = sobre o
    // valor. Cobre keyword, número, multi-argumento e :default.
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
    // try/catch/finally + throw: catch-all liga o valor lançado; finally sempre
    // roda; captura léxica no corpo/catch; try aninhado propaga ao handler externo.
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
    // Coleções ordenadas (árvore LLRB): iteração crescente é determinística,
    // independente da ordem de inserção. Cobre get/contains/assoc/dissoc/seq/=.
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
