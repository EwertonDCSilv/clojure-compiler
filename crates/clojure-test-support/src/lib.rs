//! Executable conformance-suite support for the native Clojure compiler.
//!
//! The crate deliberately has no JVM or network dependency. JVM comparison is
//! exposed as an explicit, manual oracle operation.

use clojure_span::SourceMap;
use clojure_syntax::Form;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use wait_timeout::ChildExt;

pub const MAX_JOBS: usize = 4;
pub const CHECKSUM_FILE: &str = "checksums.sha256";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Level {
    A,
    B,
    C,
    D,
    E,
}

impl Level {
    fn directory(self) -> &'static str {
        match self {
            Self::A => "level-a-syntax",
            Self::B => "level-b-semantics",
            Self::C => "level-c-stdlib",
            Self::D => "level-d-pure-libraries",
            Self::E => "level-e-ecosystem",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaseStatus {
    Active,
    Xfail,
    Pending,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaseClass {
    Spec,
    Official,
    ExpectedDiff,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Target {
    Reader,
    BuildRun,
    BuildError,
    Project,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Oracle {
    Equal,
    ExpectedDiff,
    NotApplicable,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CaseManifest {
    pub id: String,
    pub level: Level,
    pub area: String,
    pub status: CaseStatus,
    pub class: CaseClass,
    pub target: Target,
    pub oracle: Oracle,
    pub timeout_ms: u64,
    pub gc_stress: bool,
    pub reason: String,
    pub tracking: String,
    #[serde(default)]
    pub namespace: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Case {
    pub manifest: CaseManifest,
    pub directory: PathBuf,
}

#[derive(Clone, Debug, Default)]
pub struct Filters {
    pub level: Option<Level>,
    pub area: Option<String>,
    pub status: Option<CaseStatus>,
    pub namespace: Option<String>,
}

impl Filters {
    pub fn matches(&self, case: &Case) -> bool {
        let level_matches = match self.level {
            Some(value) => value == case.manifest.level,
            None => true,
        };
        let area_matches = match &self.area {
            Some(value) => case.manifest.area.contains(value),
            None => true,
        };
        let status_matches = match self.status {
            Some(value) => value == case.manifest.status,
            None => true,
        };
        let namespace_matches = match (&self.namespace, &case.manifest.namespace) {
            (Some(filter), Some(namespace)) => namespace.contains(filter),
            (Some(_), None) => false,
            (None, _) => true,
        };
        level_matches && area_matches && status_matches && namespace_matches
    }
}

#[derive(Clone, Debug)]
pub struct VerifyOptions {
    pub root: PathBuf,
    pub compiler: PathBuf,
    pub report_directory: PathBuf,
    pub jobs: usize,
    pub filters: Filters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResultKind {
    Pass,
    ExpectedFailure,
    Pending,
    Fail,
    UnexpectedPass,
}

#[derive(Clone, Debug, Serialize)]
pub struct CaseReport {
    pub id: String,
    pub level: Level,
    pub area: String,
    pub status: CaseStatus,
    pub result: ResultKind,
    pub duration_ms: u128,
    pub message: String,
    pub error_category: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Summary {
    pub total: usize,
    pub active: usize,
    pub xfail: usize,
    pub pending: usize,
    pub passed: usize,
    pub expected_failures: usize,
    pub failed: usize,
    pub unexpected_passes: usize,
    pub duration_ms: u128,
}

#[derive(Clone, Debug, Serialize)]
pub struct VerifyReport {
    pub schema_version: u32,
    pub checksum: String,
    pub success: bool,
    pub summary: Summary,
    pub cases: Vec<CaseReport>,
}

#[derive(Clone, Debug)]
struct ProcessOutput {
    status: ExitStatus,
    stdout: String,
    stderr: String,
    timed_out: bool,
}

#[derive(Clone, Debug)]
enum MatchResult {
    Match(String),
    Mismatch(String, Option<String>),
}

pub fn parse_level(value: &str) -> Result<Level, String> {
    match value.to_ascii_uppercase().as_str() {
        "A" => Ok(Level::A),
        "B" => Ok(Level::B),
        "C" => Ok(Level::C),
        "D" => Ok(Level::D),
        "E" => Ok(Level::E),
        _ => Err(format!(
            "invalid level `{value}`; expected A, B, C, D, or E"
        )),
    }
}

pub fn parse_status(value: &str) -> Result<CaseStatus, String> {
    match value {
        "active" => Ok(CaseStatus::Active),
        "xfail" => Ok(CaseStatus::Xfail),
        "pending" => Ok(CaseStatus::Pending),
        _ => Err(format!(
            "invalid status `{value}`; expected active, xfail, or pending"
        )),
    }
}

pub fn discover_cases(root: &Path) -> Result<Vec<Case>, String> {
    if !root.is_dir() {
        return Err(format!(
            "conformance root does not exist: {}",
            root.display()
        ));
    }
    let mut manifests = Vec::new();
    collect_manifests(root, &mut manifests).map_err(|error| error.to_string())?;
    manifests.sort();
    if manifests.is_empty() {
        return Err(format!("no case.toml files found under {}", root.display()));
    }

    let mut ids = HashSet::new();
    let mut cases = Vec::with_capacity(manifests.len());
    let mut errors = Vec::new();
    for path in manifests {
        match load_case(root, &path) {
            Ok(case) => {
                if !ids.insert(case.manifest.id.clone()) {
                    errors.push(format!(
                        "{}: duplicate id `{}`",
                        path.display(),
                        case.manifest.id
                    ));
                } else {
                    cases.push(case);
                }
            }
            Err(error) => errors.push(error),
        }
    }
    if errors.is_empty() {
        Ok(cases)
    } else {
        Err(errors.join("\n"))
    }
}

fn collect_manifests(directory: &Path, output: &mut Vec<PathBuf>) -> io::Result<()> {
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_manifests(&path, output)?;
        } else if path.file_name().and_then(|name| name.to_str()) == Some("case.toml") {
            output.push(path);
        }
    }
    Ok(())
}

fn load_case(root: &Path, manifest_path: &Path) -> Result<Case, String> {
    let text = fs::read_to_string(manifest_path)
        .map_err(|error| format!("{}: {error}", manifest_path.display()))?;
    let manifest: CaseManifest = toml::from_str(&text)
        .map_err(|error| format!("{}: invalid schema: {error}", manifest_path.display()))?;
    let directory = manifest_path
        .parent()
        .ok_or_else(|| format!("{}: missing parent directory", manifest_path.display()))?
        .to_path_buf();
    validate_manifest(root, &directory, &manifest)?;
    Ok(Case {
        manifest,
        directory,
    })
}

fn validate_manifest(root: &Path, directory: &Path, manifest: &CaseManifest) -> Result<(), String> {
    let fail = |message: &str| {
        Err(format!(
            "{}: {message}",
            directory.join("case.toml").display()
        ))
    };
    if manifest.id.trim().is_empty()
        || !manifest
            .id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || ".-_".contains(c))
    {
        return fail("id must contain only lowercase ASCII letters, digits, '.', '-', or '_'");
    }
    if manifest.area.trim().is_empty() {
        return fail("area must not be empty");
    }
    if manifest.reason.trim().is_empty() || manifest.tracking.trim().is_empty() {
        return fail("reason and tracking must not be empty");
    }
    if manifest.timeout_ms == 0 {
        return fail("timeout_ms must be greater than zero");
    }
    let relative = directory.strip_prefix(root).map_err(|_| {
        format!(
            "{}: case is outside conformance root {}",
            directory.display(),
            root.display()
        )
    })?;
    if relative
        .components()
        .next()
        .and_then(|part| part.as_os_str().to_str())
        != Some(manifest.level.directory())
    {
        return fail("level does not match the case directory");
    }
    if !directory.join("input.clj").is_file() {
        return fail("input.clj is required");
    }
    if manifest.status == CaseStatus::Active && manifest.class == CaseClass::Unsupported {
        return fail("an active case cannot be classified as unsupported");
    }
    if manifest.class == CaseClass::ExpectedDiff && manifest.oracle != Oracle::ExpectedDiff {
        return fail("class expected-diff requires oracle expected-diff");
    }
    if manifest.status != CaseStatus::Pending && manifest.target == Target::Project {
        return fail("project cases are pending until a project execution path exists");
    }
    if manifest.status != CaseStatus::Pending {
        let expected = match manifest.target {
            Target::Reader => "expected.edn",
            Target::BuildRun => "expected.stdout",
            Target::BuildError => "expected.stderr",
            Target::Project => unreachable!(),
        };
        if !directory.join(expected).is_file() {
            return fail(&format!("{expected} is required for this executable case"));
        }
    }
    Ok(())
}

pub fn list_cases(root: &Path, filters: &Filters) -> Result<Vec<Case>, String> {
    discover_cases(root).map(|cases| {
        cases
            .into_iter()
            .filter(|case| filters.matches(case))
            .collect()
    })
}

pub fn verify(options: &VerifyOptions) -> Result<VerifyReport, String> {
    if !(1..=MAX_JOBS).contains(&options.jobs) {
        return Err(format!(
            "jobs must be between 1 and {MAX_JOBS}, got {}",
            options.jobs
        ));
    }
    if !options.compiler.is_file() {
        return Err(format!(
            "compiler executable does not exist: {}",
            options.compiler.display()
        ));
    }
    let checksum = verify_checksums(&options.root)?;
    let all_cases = discover_cases(&options.root)?;
    let selected = all_cases
        .into_iter()
        .filter(|case| options.filters.matches(case))
        .collect::<Vec<_>>();
    let started = Instant::now();
    let queue = Arc::new(Mutex::new(VecDeque::from(selected)));
    let (sender, receiver) = mpsc::channel();

    thread::scope(|scope| {
        for _ in 0..options.jobs {
            let queue = Arc::clone(&queue);
            let sender = sender.clone();
            let compiler = &options.compiler;
            scope.spawn(move || loop {
                let case = queue.lock().expect("case queue poisoned").pop_front();
                let Some(case) = case else {
                    break;
                };
                let report = execute_case(&case, compiler);
                if sender.send(report).is_err() {
                    break;
                }
            });
        }
    });
    drop(sender);

    let mut cases = receiver.into_iter().collect::<Vec<_>>();
    cases.sort_by(|left, right| left.id.cmp(&right.id));
    let mut summary = summarize(&cases);
    summary.duration_ms = started.elapsed().as_millis();
    let success = summary.failed == 0 && summary.unexpected_passes == 0;
    let report = VerifyReport {
        schema_version: 1,
        checksum,
        success,
        summary,
        cases,
    };
    write_report(&options.report_directory, "report", &report)?;
    Ok(report)
}

fn execute_case(case: &Case, compiler: &Path) -> CaseReport {
    let started = Instant::now();
    if case.manifest.status == CaseStatus::Pending {
        return CaseReport {
            id: case.manifest.id.clone(),
            level: case.manifest.level,
            area: case.manifest.area.clone(),
            status: case.manifest.status,
            result: ResultKind::Pending,
            duration_ms: 0,
            message: case.manifest.reason.clone(),
            error_category: None,
        };
    }

    let outcome = execute_target(case, compiler);
    let (result, message, category) = match (case.manifest.status, outcome) {
        (CaseStatus::Active, MatchResult::Match(message)) => (ResultKind::Pass, message, None),
        (CaseStatus::Active, MatchResult::Mismatch(message, category)) => {
            (ResultKind::Fail, message, category)
        }
        (CaseStatus::Xfail, MatchResult::Mismatch(message, category)) => (
            ResultKind::ExpectedFailure,
            format!("{}: {message}", case.manifest.reason),
            category,
        ),
        (CaseStatus::Xfail, MatchResult::Match(message)) => (
            ResultKind::UnexpectedPass,
            format!("xfail passed and must be promoted: {message}"),
            None,
        ),
        (CaseStatus::Pending, _) => unreachable!(),
    };
    CaseReport {
        id: case.manifest.id.clone(),
        level: case.manifest.level,
        area: case.manifest.area.clone(),
        status: case.manifest.status,
        result,
        duration_ms: started.elapsed().as_millis(),
        message,
        error_category: category,
    }
}

fn execute_target(case: &Case, compiler: &Path) -> MatchResult {
    let timeout = Duration::from_millis(case.manifest.timeout_ms);
    match case.manifest.target {
        Target::Reader => execute_reader(case, compiler, timeout),
        Target::BuildRun => execute_build_run(case, compiler, timeout),
        Target::BuildError => execute_build_error(case, compiler, timeout),
        Target::Project => MatchResult::Mismatch(
            "project execution is not implemented".to_string(),
            Some("unsupported".to_string()),
        ),
    }
}

fn execute_reader(case: &Case, compiler: &Path, timeout: Duration) -> MatchResult {
    let input = case.directory.join("input.clj");
    let mut command = Command::new(compiler);
    command.arg("read").arg(&input);
    let output = match run_process(&mut command, timeout) {
        Ok(output) => output,
        Err(error) => {
            return MatchResult::Mismatch(error, Some("runner-error".to_string()));
        }
    };
    if output.timed_out {
        return MatchResult::Mismatch("reader timed out".to_string(), Some("timeout".to_string()));
    }
    if !output.status.success() {
        return MatchResult::Mismatch(
            format!("reader failed: {}", normalize_text(&output.stderr, case)),
            error_category(&output.stderr),
        );
    }
    let expected = match fs::read_to_string(case.directory.join("expected.edn")) {
        Ok(value) => value,
        Err(error) => {
            return MatchResult::Mismatch(
                format!("cannot read expected.edn: {error}"),
                Some("fixture-error".to_string()),
            );
        }
    };
    match structurally_equal(&expected, &output.stdout) {
        Ok(true) => MatchResult::Match("reader forms match structurally".to_string()),
        Ok(false) => MatchResult::Mismatch(
            format!(
                "reader mismatch\nexpected:\n{}\nactual:\n{}",
                normalize_newlines(&expected),
                normalize_newlines(&output.stdout)
            ),
            Some("output-mismatch".to_string()),
        ),
        Err(error) => MatchResult::Mismatch(error, Some("fixture-error".to_string())),
    }
}

fn execute_build_run(case: &Case, compiler: &Path, timeout: Duration) -> MatchResult {
    let temporary = match TempDir::new() {
        Ok(value) => value,
        Err(error) => {
            return MatchResult::Mismatch(
                format!("cannot create temporary directory: {error}"),
                Some("runner-error".to_string()),
            );
        }
    };
    let executable = temporary.path().join("program");
    let input = case.directory.join("input.clj");
    let mut build = Command::new(compiler);
    build.arg("build").arg(&input).arg("-o").arg(&executable);
    let build_output = match run_process(&mut build, timeout) {
        Ok(output) => output,
        Err(error) => {
            return MatchResult::Mismatch(error, Some("runner-error".to_string()));
        }
    };
    if build_output.timed_out {
        return MatchResult::Mismatch("build timed out".to_string(), Some("timeout".to_string()));
    }
    if !build_output.status.success() {
        return MatchResult::Mismatch(
            format!(
                "build failed: {}",
                normalize_text(&build_output.stderr, case)
            ),
            error_category(&build_output.stderr),
        );
    }

    let mut run = Command::new(&executable);
    if case.manifest.gc_stress {
        run.env("CLJN_GC_STRESS", "1");
    }
    let run_output = match run_process(&mut run, timeout) {
        Ok(output) => output,
        Err(error) => {
            return MatchResult::Mismatch(error, Some("runner-error".to_string()));
        }
    };
    if run_output.timed_out {
        return MatchResult::Mismatch("program timed out".to_string(), Some("timeout".to_string()));
    }
    if !run_output.status.success() {
        return MatchResult::Mismatch(
            format!(
                "program failed: {}",
                normalize_text(&run_output.stderr, case)
            ),
            error_category(&run_output.stderr),
        );
    }

    let expected_stdout = match fs::read_to_string(case.directory.join("expected.stdout")) {
        Ok(value) => normalize_newlines(&value),
        Err(error) => {
            return MatchResult::Mismatch(
                format!("cannot read expected.stdout: {error}"),
                Some("fixture-error".to_string()),
            );
        }
    };
    let actual_stdout = normalize_newlines(&run_output.stdout);
    if expected_stdout != actual_stdout {
        return MatchResult::Mismatch(
            format!("stdout mismatch\nexpected:\n{expected_stdout}\nactual:\n{actual_stdout}"),
            Some("output-mismatch".to_string()),
        );
    }
    let expected_stderr_path = case.directory.join("expected.stderr");
    let expected_stderr = fs::read_to_string(&expected_stderr_path).unwrap_or_default();
    if normalize_newlines(&expected_stderr) != normalize_newlines(&run_output.stderr) {
        return MatchResult::Mismatch(
            format!(
                "stderr mismatch\nexpected:\n{}\nactual:\n{}",
                normalize_newlines(&expected_stderr),
                normalize_newlines(&run_output.stderr)
            ),
            error_category(&run_output.stderr),
        );
    }
    MatchResult::Match("native output matches".to_string())
}

fn execute_build_error(case: &Case, compiler: &Path, timeout: Duration) -> MatchResult {
    let temporary = match TempDir::new() {
        Ok(value) => value,
        Err(error) => {
            return MatchResult::Mismatch(
                format!("cannot create temporary directory: {error}"),
                Some("runner-error".to_string()),
            );
        }
    };
    let executable = temporary.path().join("program");
    let input = case.directory.join("input.clj");
    let mut command = Command::new(compiler);
    command.arg("build").arg(&input).arg("-o").arg(&executable);
    let output = match run_process(&mut command, timeout) {
        Ok(output) => output,
        Err(error) => {
            return MatchResult::Mismatch(error, Some("runner-error".to_string()));
        }
    };
    if output.timed_out {
        return MatchResult::Mismatch("build timed out".to_string(), Some("timeout".to_string()));
    }
    if output.status.success() {
        return MatchResult::Mismatch(
            "build unexpectedly succeeded".to_string(),
            Some("unexpected-success".to_string()),
        );
    }
    let expected = match fs::read_to_string(case.directory.join("expected.stderr")) {
        Ok(value) => value,
        Err(error) => {
            return MatchResult::Mismatch(
                format!("cannot read expected.stderr: {error}"),
                Some("fixture-error".to_string()),
            );
        }
    };
    let actual = normalize_text(&output.stderr, case);
    let missing = expected
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !actual.contains(line))
        .collect::<Vec<_>>();
    if missing.is_empty() {
        MatchResult::Match(format!(
            "build failed in category {}",
            error_category(&actual).unwrap_or_else(|| "compiler-error".to_string())
        ))
    } else {
        MatchResult::Mismatch(
            format!(
                "build failed, but expected diagnostic fragments were missing: {}",
                missing.join(", ")
            ),
            error_category(&actual),
        )
    }
}

fn run_process(command: &mut Command, timeout: Duration) -> Result<ProcessOutput, String> {
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|error| format!("cannot start {:?}: {error}", command.get_program()))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "cannot capture process stdout".to_string())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "cannot capture process stderr".to_string())?;
    let stdout_reader = thread::spawn(move || read_pipe(stdout));
    let stderr_reader = thread::spawn(move || read_pipe(stderr));

    let status = match child
        .wait_timeout(timeout)
        .map_err(|error| format!("cannot wait for process: {error}"))?
    {
        Some(status) => (status, false),
        None => {
            child
                .kill()
                .map_err(|error| format!("cannot kill timed-out process: {error}"))?;
            let status = child
                .wait()
                .map_err(|error| format!("cannot reap timed-out process: {error}"))?;
            (status, true)
        }
    };
    let stdout = stdout_reader
        .join()
        .map_err(|_| "stdout reader thread panicked".to_string())?
        .map_err(|error| format!("cannot read process stdout: {error}"))?;
    let stderr = stderr_reader
        .join()
        .map_err(|_| "stderr reader thread panicked".to_string())?
        .map_err(|error| format!("cannot read process stderr: {error}"))?;
    Ok(ProcessOutput {
        status: status.0,
        stdout,
        stderr,
        timed_out: status.1,
    })
}

fn read_pipe(mut pipe: impl Read) -> io::Result<String> {
    let mut bytes = Vec::new();
    pipe.read_to_end(&mut bytes)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

pub fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

fn normalize_text(text: &str, case: &Case) -> String {
    normalize_newlines(text)
        .replace(&case.directory.to_string_lossy().to_string(), "<case>")
        .replace('\\', "/")
}

pub fn structurally_equal(expected: &str, actual: &str) -> Result<bool, String> {
    let expected = parse_forms("<expected>", expected)?;
    let actual = parse_forms("<actual>", actual)?;
    if expected.len() != actual.len() {
        return Ok(false);
    }
    Ok(expected
        .iter()
        .zip(actual.iter())
        .all(|(left, right)| canonical_form(&left.node) == canonical_form(&right.node)))
}

fn parse_forms(name: &str, text: &str) -> Result<Vec<clojure_syntax::SForm>, String> {
    let mut sources = SourceMap::new();
    let id = sources.add(name, text);
    clojure_reader::read_all(id, text).map_err(|diagnostics| {
        format!(
            "{name} is not valid EDN/forms:\n{}",
            diagnostics.render(&sources)
        )
    })
}

fn canonical_form(form: &Form) -> String {
    match form {
        Form::List(items) => canonical_sequence("list", items),
        Form::Vector(items) => canonical_sequence("vector", items),
        Form::Set(items) => {
            let mut values = items
                .iter()
                .map(|item| canonical_form(&item.node))
                .collect::<Vec<_>>();
            values.sort();
            format!("set[{}]", values.join("|"))
        }
        Form::Map(pairs) => {
            let mut values = pairs
                .iter()
                .map(|(key, value)| {
                    format!(
                        "{}=>{}",
                        canonical_form(&key.node),
                        canonical_form(&value.node)
                    )
                })
                .collect::<Vec<_>>();
            values.sort();
            format!("map[{}]", values.join("|"))
        }
        Form::Meta { meta, form } => format!(
            "meta[{}]{}",
            canonical_form(&meta.node),
            canonical_form(&form.node)
        ),
        other => format!("{}:{}", other.kind(), other),
    }
}

fn canonical_sequence(kind: &str, items: &[clojure_syntax::SForm]) -> String {
    format!(
        "{kind}[{}]",
        items
            .iter()
            .map(|item| canonical_form(&item.node))
            .collect::<Vec<_>>()
            .join("|")
    )
}

pub fn error_category(stderr: &str) -> Option<String> {
    if stderr.to_ascii_lowercase().contains("timed out") {
        return Some("timeout".to_string());
    }
    let bytes = stderr.as_bytes();
    for index in 0..bytes.len().saturating_sub(6) {
        if bytes[index] == b'['
            && bytes[index + 1] == b'E'
            && bytes[index + 2..index + 6].iter().all(u8::is_ascii_digit)
            && bytes[index + 6] == b']'
        {
            return Some(stderr[index + 1..index + 6].to_string());
        }
    }
    if !stderr.trim().is_empty() {
        Some("runtime-error".to_string())
    } else {
        None
    }
}

fn summarize(cases: &[CaseReport]) -> Summary {
    let mut summary = Summary {
        total: cases.len(),
        ..Summary::default()
    };
    for case in cases {
        match case.status {
            CaseStatus::Active => summary.active += 1,
            CaseStatus::Xfail => summary.xfail += 1,
            CaseStatus::Pending => summary.pending += 1,
        }
        match case.result {
            ResultKind::Pass => summary.passed += 1,
            ResultKind::ExpectedFailure => summary.expected_failures += 1,
            ResultKind::Fail => summary.failed += 1,
            ResultKind::UnexpectedPass => summary.unexpected_passes += 1,
            ResultKind::Pending => {}
        }
    }
    summary
}

fn write_report(report_directory: &Path, stem: &str, report: &VerifyReport) -> Result<(), String> {
    fs::create_dir_all(report_directory).map_err(|error| {
        format!(
            "cannot create report directory {}: {error}",
            report_directory.display()
        )
    })?;
    let json = serde_json::to_string_pretty(report)
        .map_err(|error| format!("cannot serialize JSON report: {error}"))?;
    fs::write(
        report_directory.join(format!("{stem}.json")),
        format!("{json}\n"),
    )
    .map_err(|error| format!("cannot write JSON report: {error}"))?;
    let summary = &report.summary;
    let text = format!(
        "Clojure conformance\n\
         success: {}\n\
         checksum: {}\n\
         total: {}\n\
         active: {}\n\
         xfail: {}\n\
         pending: {}\n\
         passed: {}\n\
         expected failures: {}\n\
         failed: {}\n\
         unexpected passes: {}\n\
         duration_ms: {}\n",
        report.success,
        report.checksum,
        summary.total,
        summary.active,
        summary.xfail,
        summary.pending,
        summary.passed,
        summary.expected_failures,
        summary.failed,
        summary.unexpected_passes,
        summary.duration_ms
    );
    fs::write(report_directory.join(format!("{stem}-summary.txt")), text)
        .map_err(|error| format!("cannot write text report: {error}"))
}

pub fn checksum_entries(root: &Path) -> Result<BTreeMap<String, String>, String> {
    let cases = discover_cases(root)?;
    let mut files = Vec::new();
    for case in cases {
        let mut entries = fs::read_dir(&case.directory)
            .map_err(|error| format!("{}: {error}", case.directory.display()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?;
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            if entry.path().is_file() {
                files.push(entry.path());
            }
        }
    }
    files.sort();
    files.dedup();
    let mut result = BTreeMap::new();
    for path in files {
        let bytes = fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        let relative = path
            .strip_prefix(root)
            .map_err(|_| format!("{} is outside {}", path.display(), root.display()))?
            .to_string_lossy()
            .replace('\\', "/");
        result.insert(relative, hex_digest(&bytes));
    }
    Ok(result)
}

pub fn update_checksums(root: &Path) -> Result<String, String> {
    let entries = checksum_entries(root)?;
    let contents = entries
        .iter()
        .map(|(path, digest)| format!("{digest}  {path}\n"))
        .collect::<String>();
    fs::write(root.join(CHECKSUM_FILE), &contents)
        .map_err(|error| format!("cannot write {CHECKSUM_FILE}: {error}"))?;
    Ok(hex_digest(contents.as_bytes()))
}

pub fn verify_checksums(root: &Path) -> Result<String, String> {
    let checksum_path = root.join(CHECKSUM_FILE);
    let text = fs::read_to_string(&checksum_path)
        .map_err(|error| format!("cannot read {}: {error}", checksum_path.display()))?;
    let expected = parse_checksums(&text)?;
    let actual = checksum_entries(root)?;
    if expected != actual {
        let missing = actual
            .keys()
            .filter(|path| !expected.contains_key(*path))
            .cloned()
            .collect::<Vec<_>>();
        let stale = expected
            .keys()
            .filter(|path| !actual.contains_key(*path))
            .cloned()
            .collect::<Vec<_>>();
        let changed = actual
            .iter()
            .filter(|(path, digest)| match expected.get(*path) {
                Some(old) => old != *digest,
                None => false,
            })
            .map(|(path, _)| path.clone())
            .collect::<Vec<_>>();
        return Err(format!(
            "conformance checksum mismatch; missing={missing:?}, stale={stale:?}, changed={changed:?}"
        ));
    }
    Ok(hex_digest(text.as_bytes()))
}

fn parse_checksums(text: &str) -> Result<BTreeMap<String, String>, String> {
    let mut entries = BTreeMap::new();
    for (line_number, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let Some((digest, path)) = line.split_once("  ") else {
            return Err(format!(
                "{CHECKSUM_FILE}:{}: expected '<sha256>  <path>'",
                line_number + 1
            ));
        };
        if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(format!(
                "{CHECKSUM_FILE}:{}: invalid SHA-256",
                line_number + 1
            ));
        }
        if entries
            .insert(path.to_string(), digest.to_string())
            .is_some()
        {
            return Err(format!(
                "{CHECKSUM_FILE}:{}: duplicate path `{path}`",
                line_number + 1
            ));
        }
    }
    Ok(entries)
}

fn hex_digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OracleMode {
    Check,
    Bless,
}

#[derive(Clone, Debug)]
pub struct OracleOptions {
    pub mode: OracleMode,
    pub root: PathBuf,
    pub report_directory: PathBuf,
    pub classpath: String,
    pub java: PathBuf,
    pub helper: PathBuf,
    pub filters: Filters,
}

pub fn run_oracle(options: &OracleOptions) -> Result<VerifyReport, String> {
    if options.classpath.trim().is_empty() {
        return Err(
            "Clojure/JVM classpath is required; set CLOJURE_CLASSPATH or pass --classpath"
                .to_string(),
        );
    }
    if !options.helper.is_file() {
        return Err(format!(
            "oracle helper does not exist: {}",
            options.helper.display()
        ));
    }
    verify_oracle_version(options)?;
    let cases = discover_cases(&options.root)?;
    let started = Instant::now();
    let mut reports = Vec::new();
    for case in cases.into_iter().filter(|case| {
        options.filters.matches(case)
            && case.manifest.status != CaseStatus::Pending
            && case.manifest.oracle != Oracle::NotApplicable
            && matches!(case.manifest.target, Target::Reader | Target::BuildRun)
    }) {
        let case_started = Instant::now();
        let oracle_mode = match case.manifest.target {
            Target::Reader => "reader",
            Target::BuildRun => "run",
            _ => unreachable!(),
        };
        let mut command = Command::new(&options.java);
        command
            .arg("-cp")
            .arg(&options.classpath)
            .arg("clojure.main")
            .arg(&options.helper)
            .arg(oracle_mode)
            .arg(case.directory.join("input.clj"));
        let output = run_process(
            &mut command,
            Duration::from_millis(case.manifest.timeout_ms),
        )?;
        let (result, message, category) = if output.timed_out {
            (
                ResultKind::Fail,
                "JVM oracle timed out".to_string(),
                Some("timeout".to_string()),
            )
        } else if !output.status.success() {
            (
                ResultKind::Fail,
                format!(
                    "JVM oracle failed: {}",
                    normalize_text(&output.stderr, &case)
                ),
                error_category(&output.stderr),
            )
        } else {
            oracle_result(options.mode, &case, &output.stdout)?
        };
        reports.push(CaseReport {
            id: case.manifest.id,
            level: case.manifest.level,
            area: case.manifest.area,
            status: case.manifest.status,
            result,
            duration_ms: case_started.elapsed().as_millis(),
            message,
            error_category: category,
        });
    }
    if options.mode == OracleMode::Bless {
        update_checksums(&options.root)?;
    }
    reports.sort_by(|left, right| left.id.cmp(&right.id));
    let mut summary = summarize(&reports);
    summary.duration_ms = started.elapsed().as_millis();
    let success = summary.failed == 0 && summary.unexpected_passes == 0;
    let report = VerifyReport {
        schema_version: 1,
        checksum: verify_checksums(&options.root)?,
        success,
        summary,
        cases: reports,
    };
    write_report(&options.report_directory, "oracle-report", &report)?;
    Ok(report)
}

fn verify_oracle_version(options: &OracleOptions) -> Result<(), String> {
    let mut command = Command::new(&options.java);
    command
        .arg("-cp")
        .arg(&options.classpath)
        .arg("clojure.main")
        .arg("-e")
        .arg("(print (clojure-version))");
    let output = run_process(&mut command, Duration::from_secs(10))?;
    if output.timed_out {
        return Err("timed out while checking the Clojure/JVM oracle version".to_string());
    }
    if !output.status.success() {
        return Err(format!(
            "cannot start the Clojure/JVM oracle: {}",
            normalize_newlines(&output.stderr)
        ));
    }
    if output.stdout.trim() != "1.12.5" {
        return Err(format!(
            "oracle version must be Clojure/JVM 1.12.5, found `{}`",
            output.stdout.trim()
        ));
    }
    Ok(())
}

fn oracle_result(
    mode: OracleMode,
    case: &Case,
    stdout: &str,
) -> Result<(ResultKind, String, Option<String>), String> {
    let expected_path = match case.manifest.target {
        Target::Reader => case.directory.join("expected.edn"),
        Target::BuildRun => case.directory.join("expected.stdout"),
        _ => unreachable!(),
    };
    if mode == OracleMode::Bless {
        if case.manifest.oracle != Oracle::Equal {
            return Ok((
                ResultKind::Pending,
                "expected-diff case is never blessed from the JVM".to_string(),
                None,
            ));
        }
        fs::write(&expected_path, normalize_newlines(stdout))
            .map_err(|error| format!("cannot bless {}: {error}", expected_path.display()))?;
        return Ok((
            ResultKind::Pass,
            "expectation updated from Clojure/JVM 1.12.5".to_string(),
            None,
        ));
    }

    let expected = fs::read_to_string(&expected_path)
        .map_err(|error| format!("cannot read {}: {error}", expected_path.display()))?;
    let equal = if case.manifest.target == Target::Reader {
        structurally_equal(&expected, stdout)?
    } else {
        normalize_newlines(&expected) == normalize_newlines(stdout)
    };
    match (case.manifest.oracle, equal) {
        (Oracle::Equal, true) => Ok((
            ResultKind::Pass,
            "Clojure/JVM oracle matches".to_string(),
            None,
        )),
        (Oracle::Equal, false) => Ok((
            ResultKind::Fail,
            "Clojure/JVM oracle differs from the committed expectation".to_string(),
            Some("oracle-mismatch".to_string()),
        )),
        (Oracle::ExpectedDiff, false) => Ok((
            ResultKind::ExpectedFailure,
            "declared JVM/native difference observed".to_string(),
            None,
        )),
        (Oracle::ExpectedDiff, true) => Ok((
            ResultKind::UnexpectedPass,
            "declared JVM/native difference disappeared".to_string(),
            None,
        )),
        (Oracle::NotApplicable, _) => unreachable!(),
    }
}

pub fn human_summary(report: &VerifyReport) -> String {
    let summary = &report.summary;
    format!(
        "{}: {} active, {} xfail, {} pending; {} passed, {} expected failures, {} failed, {} unexpected passes ({} ms)",
        if report.success { "PASS" } else { "FAIL" },
        summary.active,
        summary.xfail,
        summary.pending,
        summary.passed,
        summary.expected_failures,
        summary.failed,
        summary.unexpected_passes,
        summary.duration_ms
    )
}

#[cfg(test)]
mod tests {
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
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/conformance");
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
                case.manifest.status == CaseStatus::Pending
                    || case.manifest.target != Target::Project
            }));
        }
    }

    #[test]
    fn normalizes_newline_styles() {
        assert_eq!(normalize_newlines("a\r\nb\rc\n"), "a\nb\nc\n");
    }

    #[test]
    fn compares_maps_and_sets_structurally() {
        assert!(
            structurally_equal("{:a 1 :b #{2 3}}\n", "{:b #{3 2}, :a 1}\n").expect("valid forms")
        );
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
        update_checksums(temp.path()).expect("write checksums");
        assert!(verify_checksums(temp.path()).is_ok());
        write(&directory.join("input.clj"), "2\n");
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

    #[test]
    fn status_transitions_are_counted() {
        let cases = vec![
            report(CaseStatus::Active, ResultKind::Pass),
            report(CaseStatus::Xfail, ResultKind::ExpectedFailure),
            report(CaseStatus::Xfail, ResultKind::UnexpectedPass),
            report(CaseStatus::Pending, ResultKind::Pending),
        ];
        let summary = summarize(&cases);
        assert_eq!(summary.active, 1);
        assert_eq!(summary.xfail, 2);
        assert_eq!(summary.pending, 1);
        assert_eq!(summary.passed, 1);
        assert_eq!(summary.expected_failures, 1);
        assert_eq!(summary.unexpected_passes, 1);
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
}
