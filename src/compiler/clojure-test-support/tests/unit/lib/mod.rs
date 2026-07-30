//! Unit tests for lib.rs.

use super::*;

fn write(path: &Path, text: &str) {
    fs::create_dir_all(path.parent().expect("parent")).expect("create fixture directory");
    fs::write(path, text).expect("write fixture");
}

fn manifest(status: &str, target: &str) -> String {
    manifest_with_id("a.test.case", status, target)
}

fn manifest_with_id(id: &str, status: &str, target: &str) -> String {
    format!(
        "id = \"{id}\"\n\
             level = \"A\"\n\
             area = \"syntax/test\"\n\
             status = \"{status}\"\n\
             class = \"spec\"\n\
             target = \"{target}\"\n\
             oracle = \"equal\"\n\
             timeout_ms = 100\n\
             gc_stress = false\n\
             reason = \"test\"\n\
             tracking = \"test\"\n"
    )
}

fn create_case(
    root: &Path,
    name: &str,
    status: &str,
    target: &str,
    input: &str,
    expected_name: Option<&str>,
    expected: &str,
) {
    let directory = root.join("level-a-syntax/literals").join(name);
    write(
        &directory.join("case.toml"),
        &manifest_with_id(&format!("a.test.{name}"), status, target),
    );
    write(&directory.join("input.clj"), input);
    if let Some(expected_name) = expected_name {
        write(&directory.join(expected_name), expected);
    }
}

#[cfg(unix)]
fn executable(path: &Path, source: &str) {
    use std::os::unix::fs::PermissionsExt;
    write(path, source);
    let mut permissions = fs::metadata(path).expect("metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("executable permissions");
}

#[test]
fn parses_filters_and_rejects_invalid_values() {
    assert_eq!(parse_level("a"), Ok(Level::A));
    assert_eq!(parse_level("E"), Ok(Level::E));
    assert!(parse_level("F").is_err());
    assert_eq!(parse_status("xfail"), Ok(CaseStatus::Xfail));
    assert!(parse_status("skip").is_err());
}

#[test]
fn discovers_and_sorts_valid_cases() {
    let temp = TempDir::new().expect("temp");
    for name in ["z", "a"] {
        let directory = temp.path().join("level-a-syntax/literals").join(name);
        write(
            &directory.join("case.toml"),
            &manifest("pending", "reader").replace("a.test.case", &format!("a.test.{name}")),
        );
        write(&directory.join("input.clj"), "1\n");
    }
    let cases = discover_cases(temp.path()).expect("discover");
    assert_eq!(cases.len(), 2);
    assert!(cases[0].directory.ends_with("a"));
}

#[test]
fn rejects_duplicate_ids_and_missing_required_files() {
    let temp = TempDir::new().expect("temp");
    let first = temp.path().join("level-a-syntax/literals/one");
    let second = temp.path().join("level-a-syntax/literals/two");
    write(&first.join("case.toml"), &manifest("pending", "reader"));
    write(&first.join("input.clj"), "1");
    write(&second.join("case.toml"), &manifest("active", "reader"));
    write(&second.join("input.clj"), "2");
    let error = discover_cases(temp.path()).expect_err("invalid suite");
    assert!(error.contains("expected.edn is required") || error.contains("duplicate id"));
}

#[test]
fn rejects_unknown_toml_fields_and_wrong_level_directories() {
    let temp = TempDir::new().expect("temp");
    let bad_schema = temp.path().join("level-a-syntax/literals/schema");
    write(
        &bad_schema.join("case.toml"),
        &(manifest("pending", "reader") + "surprise = true\n"),
    );
    write(&bad_schema.join("input.clj"), "1");
    let error = discover_cases(temp.path()).expect_err("strict TOML");
    assert!(error.contains("unknown field"));

    let other = TempDir::new().expect("temp");
    let bad_level = other.path().join("level-b-semantics/arithmetic/wrong");
    write(&bad_level.join("case.toml"), &manifest("pending", "reader"));
    write(&bad_level.join("input.clj"), "1");
    assert!(discover_cases(other.path())
        .expect_err("wrong level")
        .contains("level does not match"));
}

#[test]
fn validates_process_contract_schema_and_rejects_unsafe_paths() {
    let temp = TempDir::new().expect("temp");
    let directory = temp.path().join("level-a-syntax/literals/process");
    write(
        &directory.join("case.toml"),
        &(manifest_with_id("a.test.process", "active", "build-run")
            + "[run]\n\
                   args = [\"one\", \"two\"]\n\
                   stdin = \"stdin.bin\"\n\
                   expected_exit = 7\n\
                   platforms = [\"linux\"]\n\
                   setup_symlinks = [{ path = \"link\", target = \"target\" }]\n\
                   expected_symlinks = [{ path = \"link\", target = \"target\" }]\n\
                   [run.env]\n\
                   IO_TEST = \"yes\"\n"),
    );
    write(&directory.join("input.clj"), "1\n");
    write(&directory.join("expected.stdout.bin"), "");
    write(&directory.join("stdin.bin"), "payload");
    let cases = discover_cases(temp.path()).expect("valid process contract");
    assert_eq!(cases[0].manifest.run.args, ["one", "two"]);
    assert_eq!(cases[0].manifest.run.expected_exit, 7);
    assert_eq!(
        cases[0].manifest.run.env.get("IO_TEST"),
        Some(&"yes".into())
    );

    let invalid = TempDir::new().expect("temp");
    let directory = invalid.path().join("level-a-syntax/literals/process");
    write(
        &directory.join("case.toml"),
        &(manifest_with_id("a.test.process", "pending", "build-run")
            + "[run]\nstdin = \"../outside\"\n"),
    );
    write(&directory.join("input.clj"), "1\n");
    assert!(discover_cases(invalid.path())
        .expect_err("unsafe stdin path")
        .contains("relative path components"));
}

#[test]
fn compares_binary_stream_expectations() {
    let temp = TempDir::new().expect("temp");
    let directory = temp.path().join("level-a-syntax/literals/binary");
    write(
        &directory.join("case.toml"),
        &manifest_with_id("a.test.binary", "active", "build-run"),
    );
    write(&directory.join("input.clj"), "1\n");
    fs::write(directory.join("expected.stdout.bin"), [0_u8, 255, 10]).expect("binary fixture");
    let case = discover_cases(temp.path()).expect("discover").remove(0);
    assert!(compare_stream(
        &case,
        "stdout",
        "expected.stdout",
        "expected.stdout.bin",
        &[0, 255, 10],
        true,
    )
    .is_ok());
    assert!(compare_stream(
        &case,
        "stdout",
        "expected.stdout",
        "expected.stdout.bin",
        &[0, 10],
        true,
    )
    .is_err());
}

#[test]
fn filters_by_level_area_status_and_namespace() {
    let case = Case {
        manifest: CaseManifest {
            id: "c.test".into(),
            level: Level::C,
            area: "stdlib/clojure-core".into(),
            status: CaseStatus::Active,
            class: CaseClass::Official,
            target: Target::BuildRun,
            oracle: Oracle::Equal,
            timeout_ms: 10,
            gc_stress: false,
            reason: "test".into(),
            tracking: "test".into(),
            namespace: Some("clojure.core".into()),
            run: RunConfig::default(),
        },
        directory: PathBuf::new(),
    };
    assert!(Filters {
        level: Some(Level::C),
        area: Some("core".into()),
        status: Some(CaseStatus::Active),
        namespace: Some("clojure".into()),
    }
    .matches(&case));
    assert!(!Filters {
        level: Some(Level::A),
        ..Filters::default()
    }
    .matches(&case));
}

#[test]
fn tracked_levels_d_and_e_are_not_pending_only() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../tests/conformance");
    let cases = discover_cases(&root).expect("discover tracked conformance suite");
    for (level, minimum_active, minimum_xfail) in
        [(Level::D, 8_usize, 5_usize), (Level::E, 5_usize, 6_usize)]
    {
        let level_cases = cases
            .iter()
            .filter(|case| case.manifest.level == level)
            .collect::<Vec<_>>();
        let active = level_cases
            .iter()
            .filter(|case| case.manifest.status == CaseStatus::Active)
            .count();
        let xfail = level_cases
            .iter()
            .filter(|case| case.manifest.status == CaseStatus::Xfail)
            .count();
        assert!(
            active >= minimum_active,
            "level {level:?} needs executable passing coverage"
        );
        assert!(
            xfail >= minimum_xfail,
            "level {level:?} needs executable gap coverage"
        );
        assert!(level_cases.iter().all(|case| {
            case.manifest.status == CaseStatus::Pending || case.manifest.target != Target::Project
        }));
    }
}

#[test]
fn tracked_pedestal_target_has_an_http_project_contract() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../tests/conformance");
    let cases = discover_cases(&root).expect("discover tracked conformance suite");
    let case = cases
        .iter()
        .find(|case| case.manifest.id == "e.pedestal.hello_world_api")
        .expect("Pedestal Hello World target");

    assert_eq!(case.manifest.level, Level::E);
    assert_eq!(case.manifest.status, CaseStatus::Pending);
    assert_eq!(case.manifest.target, Target::Project);
    assert_eq!(case.manifest.area, "ecosystem/web-frameworks/pedestal");
    assert_eq!(case.manifest.namespace.as_deref(), Some("hello"));

    for relative_path in [
        "deps.edn",
        "src/hello.clj",
        "request.http",
        "expected-response.edn",
    ] {
        assert!(
            case.directory.join(relative_path).is_file(),
            "missing Pedestal project contract file: {relative_path}"
        );
    }
}

#[test]
fn tracked_io_gate_has_three_scenarios_per_native_api() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../tests/conformance");
    let cases = discover_cases(&root).expect("discover tracked conformance suite");
    let mut scenarios = BTreeMap::<String, HashSet<String>>::new();
    for case in cases.iter().filter(|case| {
        case.manifest.namespace.as_deref() == Some("cljn.io")
            && case.manifest.id != "c.cljn_io.filesystem.isolated_tree_and_symlink"
    }) {
        // The native cljn.io surface is tracked with three scenarios per API on
        // Linux and is never evaluated by the Clojure/JVM oracle. Scenarios are
        // promoted from xfail to active as each API's backing primitive lands
        // (issue #99), so status may be either while the surface is completed.
        assert!(matches!(
            case.manifest.status,
            CaseStatus::Xfail | CaseStatus::Active
        ));
        assert_eq!(case.manifest.oracle, Oracle::NotApplicable);
        assert_eq!(case.manifest.run.platforms, ["linux"]);
        let (api, scenario) = case
            .manifest
            .id
            .rsplit_once('.')
            .expect("I/O case id has scenario suffix");
        scenarios
            .entry(api.to_string())
            .or_default()
            .insert(scenario.to_string());
    }
    assert_eq!(scenarios.len(), 51);
    let expected = HashSet::from([
        "normal".to_string(),
        "boundary".to_string(),
        "error".to_string(),
    ]);
    assert!(scenarios.values().all(|actual| actual == &expected));

    let active_output = cases
        .iter()
        .filter(|case| {
            case.manifest.level == Level::B
                && case.manifest.area == "semantics/io/output"
                && case.manifest.status == CaseStatus::Active
        })
        .count();
    assert_eq!(active_output, 6);

    let process_cases = cases
        .iter()
        .filter(|case| case.manifest.namespace.as_deref() == Some("cljn.process"))
        .count();
    assert_eq!(process_cases, 9);
}

#[test]
fn normalizes_newline_styles() {
    assert_eq!(normalize_newlines("a\r\nb\rc\n"), "a\nb\nc\n");
}

#[test]
fn compares_maps_and_sets_structurally() {
    assert!(structurally_equal("{:a 1 :b #{2 3}}\n", "{:b #{3 2}, :a 1}\n").expect("valid forms"));
    assert!(!structurally_equal("[1 2]", "[2 1]").expect("valid forms"));
}

#[test]
fn identifies_diagnostic_and_timeout_categories() {
    assert_eq!(error_category("error[E0010]: nope"), Some("E0010".into()));
    assert_eq!(error_category("process timed out"), Some("timeout".into()));
    assert_eq!(
        error_category("plain failure"),
        Some("runtime-error".into())
    );
    assert_eq!(error_category(""), None);
}

#[test]
fn detects_checksum_changes() {
    let temp = TempDir::new().expect("temp");
    let directory = temp.path().join("level-a-syntax/literals/one");
    write(&directory.join("case.toml"), &manifest("pending", "reader"));
    write(&directory.join("input.clj"), "1\n");
    write(&directory.join("src/nested.clj"), "(ns nested)\n");
    update_checksums(temp.path()).expect("write checksums");
    assert!(verify_checksums(temp.path()).is_ok());
    write(&directory.join("src/nested.clj"), "(ns nested.changed)\n");
    assert!(verify_checksums(temp.path())
        .expect_err("changed checksum")
        .contains("changed"));
}

#[test]
fn rejects_invalid_checksum_syntax() {
    assert!(parse_checksums("bad line\n").is_err());
    assert!(parse_checksums("abcd  input.clj\n").is_err());
    let digest = "0".repeat(64);
    assert!(parse_checksums(&format!("{digest}  input.clj\n{digest}  input.clj\n")).is_err());
}

#[test]
fn process_timeout_is_enforced() {
    let mut command = if cfg!(windows) {
        let mut command = Command::new("cmd");
        command.args(["/C", "ping", "-n", "3", "127.0.0.1"]);
        command
    } else {
        let mut command = Command::new("sh");
        command.args(["-c", "while :; do :; done"]);
        command
    };
    let output = run_process(&mut command, Duration::from_millis(25)).expect("run");
    assert!(output.timed_out);
    assert!(!output.status.success());
}

#[cfg(unix)]
#[test]
fn build_run_applies_stdin_args_env_exit_and_filesystem_contract() {
    let temp = TempDir::new().expect("temp");
    let root = temp.path().join("suite");
    let directory = root.join("level-a-syntax/literals/process");
    write(
        &directory.join("case.toml"),
        &(manifest_with_id("a.test.process", "active", "build-run")
            + "[run]\n\
                   args = [\"argument\"]\n\
                   stdin = \"stdin.txt\"\n\
                   expected_exit = 7\n\
                   platforms = [\"linux\"]\n\
                   [run.env]\n\
                   IO_TEST = \"environment\"\n"),
    );
    write(&directory.join("input.clj"), "ignored\n");
    write(
        &directory.join("expected.stdout"),
        "argument|environment|payload\n",
    );
    write(&directory.join("stdin.txt"), "payload\n");
    write(&directory.join("work.before/seed.txt"), "seed\n");
    write(&directory.join("work.after/seed.txt"), "seed\n");
    write(&directory.join("work.after/result.txt"), "seed\n");
    update_checksums(&root).expect("checksums");

    let compiler = temp.path().join("fake-compiler");
    executable(
        &compiler,
        r#"#!/bin/sh
shift 2
while [ "$#" -gt 0 ]; do
  if [ "$1" = -o ]; then output="$2"; break; fi
  shift
done
printf '#!/bin/sh\nprintf "%%s|%%s|" "$1" "$IO_TEST"\ncat\ncp seed.txt result.txt\nexit 7\n' > "$output"
chmod +x "$output"
"#,
    );
    let report = verify(&VerifyOptions {
        root,
        compiler,
        report_directory: temp.path().join("reports"),
        jobs: 1,
        filters: Filters::default(),
        ir_optimization: None,
        ir_experiment: None,
    })
    .expect("verify process fixture");
    assert!(report.success, "{:?}", report.cases);
    assert_eq!(report.summary.passed, 1);
}

#[cfg(unix)]
#[test]
fn symlink_setup_and_expected_snapshot_are_declarative() {
    let temp = TempDir::new().expect("temp");
    let directory = temp.path().join("level-a-syntax/literals/symlink");
    write(
        &directory.join("case.toml"),
        &(manifest_with_id("a.test.symlink", "active", "build-run")
            + "[run]\n\
                   platforms = [\"linux\"]\n\
                   setup_symlinks = [{ path = \"link\", target = \"target.txt\" }]\n\
                   expected_symlinks = [{ path = \"link\", target = \"target.txt\" }]\n"),
    );
    write(&directory.join("input.clj"), "1\n");
    write(&directory.join("expected.stdout"), "");
    write(&directory.join("work.before/target.txt"), "target\n");
    write(&directory.join("work.after/target.txt"), "target\n");
    let case = discover_cases(temp.path()).expect("discover").remove(0);
    let work = temp.path().join("work");
    prepare_work_directory(&case, &work).expect("prepare");
    compare_work_directory(&case, &work).expect("snapshot");
}

#[test]
fn status_transitions_are_counted() {
    let cases = vec![
        report(CaseStatus::Active, ResultKind::Pass),
        report(CaseStatus::Xfail, ResultKind::ExpectedFailure),
        report(CaseStatus::Xfail, ResultKind::UnexpectedPass),
        report(CaseStatus::Pending, ResultKind::Pending),
        report(CaseStatus::Active, ResultKind::Skipped),
    ];
    let summary = summarize(&cases);
    assert_eq!(summary.active, 2);
    assert_eq!(summary.xfail, 2);
    assert_eq!(summary.pending, 1);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.expected_failures, 1);
    assert_eq!(summary.unexpected_passes, 1);
    assert_eq!(summary.skipped, 1);
}

#[cfg(unix)]
#[test]
fn verify_executes_all_targets_and_writes_reports() {
    let temp = TempDir::new().expect("temp");
    let root = temp.path().join("suite");
    create_case(
        &root,
        "reader",
        "active",
        "reader",
        "ignored\n",
        Some("expected.edn"),
        "{:a 1 :b #{2 3}}\n",
    );
    create_case(
        &root,
        "run",
        "active",
        "build-run",
        "ignored\n",
        Some("expected.stdout"),
        "1\n",
    );
    let run_manifest = root.join("level-a-syntax/literals/run/case.toml");
    let run_schema = fs::read_to_string(&run_manifest).expect("read run manifest");
    write(
        &run_manifest,
        &run_schema.replace("gc_stress = false", "gc_stress = true"),
    );
    create_case(
        &root,
        "build-error",
        "active",
        "build-error",
        "ignored\n",
        Some("expected.stderr"),
        "E0101\n",
    );
    create_case(
        &root,
        "xfail",
        "xfail",
        "build-run",
        "ignored\n",
        Some("expected.stdout"),
        "desired\n",
    );
    create_case(&root, "pending", "pending", "reader", "ignored\n", None, "");
    update_checksums(&root).expect("checksums");
    let compiler = temp.path().join("fake-compiler");
    executable(
        &compiler,
        r#"#!/bin/sh
command="$1"
input="$2"
if [ "$command" = read ]; then
  printf '{:b #{3 2}, :a 1}\n'
  exit 0
fi
case "$input" in
  *build-error*) printf 'error[E0101]: expected\n' >&2; exit 1 ;;
  *xfail*) printf 'unsupported\n' >&2; exit 1 ;;
esac
shift 2
while [ "$#" -gt 0 ]; do
  if [ "$1" = -o ]; then output="$2"; break; fi
  shift
done
printf '#!/bin/sh\nprintf "%%s\\n" "${CLJN_GC_STRESS:-off}"\n' > "$output"
chmod +x "$output"
"#,
    );
    let report_directory = temp.path().join("reports");
    let report = verify(&VerifyOptions {
        root: root.clone(),
        compiler,
        report_directory: report_directory.clone(),
        jobs: 2,
        filters: Filters::default(),
        ir_optimization: None,
        ir_experiment: None,
    })
    .expect("verify");
    assert!(report.success, "{:?}", report.cases);
    assert_eq!(report.summary.passed, 3);
    assert_eq!(report.summary.expected_failures, 1);
    assert_eq!(report.summary.pending, 1);
    assert!(report_directory.join("report.json").is_file());
    assert!(report_directory.join("report-summary.txt").is_file());
    assert_eq!(
        list_cases(
            &root,
            &Filters {
                status: Some(CaseStatus::Pending),
                ..Filters::default()
            }
        )
        .expect("list")
        .len(),
        1
    );
    assert!(human_summary(&report).starts_with("PASS:"));
}

#[cfg(unix)]
#[test]
fn verify_forwards_the_optional_ir_profile_and_experiment_to_build_targets() {
    let temp = TempDir::new().expect("temp");
    let root = temp.path().join("suite");
    create_case(
        &root,
        "run",
        "active",
        "build-run",
        "ignored\n",
        Some("expected.stdout"),
        "safe\n",
    );
    update_checksums(&root).expect("checksums");

    let arguments = temp.path().join("compiler-arguments");
    let compiler = temp.path().join("fake-compiler");
    executable(
        &compiler,
        &format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" > '{}'\n\
                 shift 2\n\
                 while [ \"$#\" -gt 0 ]; do\n\
                 \u{20} if [ \"$1\" = -o ]; then output=\"$2\"; break; fi\n\
                 \u{20} shift\n\
                 done\n\
                 printf '#!/bin/sh\\nprintf \"safe\\\\n\"\\n' > \"$output\"\n\
                 chmod +x \"$output\"\n",
            arguments.display()
        ),
    );

    let report = verify(&VerifyOptions {
        root,
        compiler,
        report_directory: temp.path().join("reports"),
        jobs: 1,
        filters: Filters::default(),
        ir_optimization: Some("safe".to_string()),
        ir_experiment: Some("adr15".to_string()),
    })
    .expect("verify safe profile");

    assert!(report.success, "{:?}", report.cases);
    let arguments = fs::read_to_string(arguments).expect("compiler arguments");
    assert!(arguments.contains("--ir-opt safe"), "{arguments}");
    assert!(arguments.contains("--ir-experiment adr15"), "{arguments}");
}

#[cfg(unix)]
#[test]
fn xfail_that_passes_fails_the_suite() {
    let temp = TempDir::new().expect("temp");
    let root = temp.path().join("suite");
    create_case(
        &root,
        "xpass",
        "xfail",
        "reader",
        "ignored\n",
        Some("expected.edn"),
        "1\n",
    );
    update_checksums(&root).expect("checksums");
    let compiler = temp.path().join("fake-compiler");
    executable(&compiler, "#!/bin/sh\nprintf '1\\n'\n");
    let report = verify(&VerifyOptions {
        root,
        compiler,
        report_directory: temp.path().join("reports"),
        jobs: 1,
        filters: Filters::default(),
        ir_optimization: None,
        ir_experiment: None,
    })
    .expect("verify");
    assert!(!report.success);
    assert_eq!(report.summary.unexpected_passes, 1);
}

#[cfg(unix)]
#[test]
fn manual_oracle_checks_version_and_can_bless() {
    let temp = TempDir::new().expect("temp");
    let root = temp.path().join("suite");
    create_case(
        &root,
        "reader",
        "active",
        "reader",
        "1\n",
        Some("expected.edn"),
        "1\n",
    );
    update_checksums(&root).expect("checksums");
    let helper = root.join("oracle/runner.clj");
    write(&helper, ";; helper\n");
    let java = temp.path().join("fake-java");
    executable(
            &java,
            "#!/bin/sh\ncase \"$*\" in *clojure-version*) printf '1.12.5' ;; *) printf '1\\n' ;; esac\n",
        );
    let options = OracleOptions {
        mode: OracleMode::Check,
        root: root.clone(),
        report_directory: temp.path().join("reports"),
        classpath: "fake-classpath".into(),
        java: java.clone(),
        helper: helper.clone(),
        filters: Filters::default(),
    };
    let checked = run_oracle(&options).expect("oracle check");
    assert!(checked.success);
    let blessed = run_oracle(&OracleOptions {
        mode: OracleMode::Bless,
        ..options
    })
    .expect("oracle bless");
    assert!(blessed.success);

    let wrong_java = temp.path().join("wrong-java");
    executable(&wrong_java, "#!/bin/sh\nprintf '1.11.0'\n");
    let error = run_oracle(&OracleOptions {
        mode: OracleMode::Check,
        root,
        report_directory: temp.path().join("wrong-report"),
        classpath: "fake-classpath".into(),
        java: wrong_java,
        helper,
        filters: Filters::default(),
    })
    .expect_err("wrong version");
    assert!(error.contains("must be Clojure/JVM 1.12.5"));
}

fn report(status: CaseStatus, result: ResultKind) -> CaseReport {
    CaseReport {
        id: "test".into(),
        level: Level::A,
        area: "test".into(),
        status,
        result,
        duration_ms: 0,
        message: String::new(),
        error_category: None,
    }
}
