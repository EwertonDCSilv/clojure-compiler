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

    let run = Command::new(&exe).output().expect("executa binário nativo");
    assert!(run.status.success(), "binário retornou erro");
    // limpeza
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
