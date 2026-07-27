//! Compila e executa os harnesses C do runtime.
//!
//! Os testes ficam sob `cargo test` para que o mesmo gate usado pelo workspace
//! também valide o runtime que é linkado aos executáveis nativos.

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

struct TestBinary {
    directory: PathBuf,
    executable: PathBuf,
}

impl Drop for TestBinary {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn c_compiler() -> OsString {
    std::env::var_os("CC").unwrap_or_else(|| OsString::from("cc"))
}

fn temporary_directory(suite: &str) -> PathBuf {
    let serial = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "cljn-runtime-c-{suite}-{}-{serial}",
        std::process::id()
    ))
}

fn compile(suite: &str, sources: &[&Path]) -> TestBinary {
    let directory = temporary_directory(suite);
    fs::create_dir_all(&directory).expect("cria diretório temporário do teste C");
    let executable = directory.join(if cfg!(windows) {
        format!("{suite}.exe")
    } else {
        suite.to_string()
    });

    let mut command = Command::new(c_compiler());
    command
        .args([
            "-std=c11",
            "-O0",
            "-g",
            "-Wall",
            "-Wextra",
            "-Werror",
            // O runtime existente tem uma linha compacta intencional que dispara
            // esse aviso. Os demais avisos continuam promovidos a erro.
            "-Wno-misleading-indentation",
        ])
        .args(sources)
        .arg("-o")
        .arg(&executable);

    if std::env::var_os("CLJN_RUNTIME_C_SANITIZE").is_some() {
        command.args(["-fsanitize=address,undefined", "-fno-omit-frame-pointer"]);
    }

    let output = command.output().expect("invoca o compilador C");
    assert!(
        output.status.success(),
        "falha ao compilar a suíte C `{suite}`\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    TestBinary {
        directory,
        executable,
    }
}

fn run(executable: &Path, args: &[&str], environment: &[(&str, &str)]) -> Output {
    let mut command = Command::new(executable);
    command.args(args);
    command.envs(environment.iter().copied());
    if std::env::var_os("CLJN_RUNTIME_C_SANITIZE").is_some() {
        // Slabs e tabelas globais vivem até o fim do processo por desenho. O
        // teste sanitizado procura acessos inválidos e UB, não teardown global.
        command.env("ASAN_OPTIONS", "detect_leaks=0");
    }
    command.output().expect("executa harness C")
}

fn assert_success(output: &Output, expected_stdout: &str) {
    assert!(
        output.status.success(),
        "harness C falhou com {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_stdout);
    assert!(
        output.stderr.is_empty(),
        "harness C escreveu em stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn runtime_internal_unit_suite() {
    let source = manifest_dir().join("tests/c/runtime_unit.c");
    let binary = compile("unit", &[&source]);

    assert_success(
        &run(&binary.executable, &[], &[]),
        "runtime C unit tests: ok\n",
    );
    assert_success(
        &run(&binary.executable, &[], &[("CLJN_GC_STRESS", "1")]),
        "runtime C unit tests: ok\n",
    );
}

#[test]
fn runtime_public_abi_integration_suite() {
    let root = manifest_dir();
    let runtime = root.join("runtime.c");
    let source = root.join("tests/c/runtime_abi.c");
    let binary = compile("abi", &[&runtime, &source]);

    assert_success(
        &run(&binary.executable, &[], &[]),
        "runtime C ABI integration: ok\n",
    );
    assert_success(
        &run(&binary.executable, &[], &[("CLJN_GC_STRESS", "1")]),
        "runtime C ABI integration: ok\n",
    );
}

#[test]
fn runtime_error_contracts_are_observable_at_process_boundary() {
    let root = manifest_dir();
    let runtime = root.join("runtime.c");
    let source = root.join("tests/c/runtime_errors.c");
    let binary = compile("errors", &[&runtime, &source]);

    for (scenario, expected) in [
        ("division-by-zero", "divisão por zero"),
        ("fixnum-overflow", "overflow em +"),
        ("non-numeric", "argumento não-numérico em +"),
        ("nth-out-of-bounds", "nth: índice fora dos limites"),
        ("wrong-arity", "aridade errada"),
        ("uncaught-throw", "exceção não capturada: 9"),
    ] {
        let output = run(&binary.executable, &[scenario], &[]);
        assert!(
            !output.status.success(),
            "cenário `{scenario}` deveria falhar"
        );
        assert!(
            output.stdout.is_empty(),
            "cenário `{scenario}` escreveu em stdout: {}",
            String::from_utf8_lossy(&output.stdout)
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(expected),
            "stderr inesperado em `{scenario}`; esperava `{expected}` em:\n{stderr}"
        );
    }
}
