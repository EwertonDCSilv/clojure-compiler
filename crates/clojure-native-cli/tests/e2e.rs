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
    assert_eq!(build_and_run("cljn_e2e_hello", src), "Hello from native Clojure\n");
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
    assert_eq!(build_and_run_env("cljn_e2e_gc_stress", src, &[("CLJN_GC_STRESS", "1")]), expected);
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
    std::fs::write(&clj, "(ns d.core)\n(defn -main [] (println \"x\"))\n(-main)").unwrap();
    let out = Command::new(cli()).arg("build").arg(&clj).arg("-o").arg(&exe).output().unwrap();
    assert!(out.status.success());
    let ldd = Command::new("ldd").arg(&exe).output();
    if let Ok(ldd) = ldd {
        let deps = String::from_utf8_lossy(&ldd.stdout).to_lowercase();
        assert!(!deps.contains("jvm"), "binário não deve depender de JVM: {deps}");
        assert!(!deps.contains("java"), "binário não deve depender de Java: {deps}");
    }
    let _ = std::fs::remove_file(&clj);
    let _ = std::fs::remove_file(&exe);
}
