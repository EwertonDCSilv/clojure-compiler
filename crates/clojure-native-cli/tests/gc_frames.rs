//! End-to-end contracts for generated shadow-stack frame restoration.

use std::path::PathBuf;
use std::process::Command;

fn cli() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_clojure-native"))
}

fn have_cc() -> bool {
    Command::new(std::env::var("CC").unwrap_or_else(|_| "cc".into()))
        .arg("--version")
        .output()
        .is_ok()
}

fn build_and_run(name: &str, source: &str) -> String {
    let directory = std::env::temp_dir().join(format!("cljn-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("cria diretório temporário");
    let source_path = directory.join(format!("{name}.clj"));
    let executable = directory.join(name);
    std::fs::write(&source_path, source).expect("grava fonte");

    let build = Command::new(cli())
        .arg("build")
        .arg(&source_path)
        .arg("-o")
        .arg(&executable)
        .output()
        .expect("executa compilador");
    assert!(
        build.status.success(),
        "compilação falhou:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );

    let run = Command::new(&executable)
        .output()
        .expect("executa binário nativo");
    assert!(run.status.success(), "binário retornou erro");
    std::fs::remove_dir_all(directory).expect("remove diretório temporário");
    String::from_utf8(run.stdout).expect("stdout UTF-8")
}

#[test]
fn rootless_allocating_function_releases_its_gc_frame() {
    if !have_cc() {
        return;
    }
    // A zero-argument function has no local roots, but its returned string is
    // temporarily rooted. Every return must restore gc_sp.
    let source = r#"(ns g.frame)
(defn value [] "Hello, World!")
(defn exercise [rounds]
  (loop [i 0 checksum 0]
    (if (= i rounds)
      checksum
      (recur (inc i)
             (if (= (value) "Hello, World!") (inc checksum) checksum)))))
(defn -main [] (println (exercise 12500000)))
(-main)"#;
    assert_eq!(build_and_run("rootless_gc_frame", source), "12500000\n");
}
