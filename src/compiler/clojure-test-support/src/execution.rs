//! Case execution: process spawning, comparison dispatch, and summarization.

use crate::checksum::*;
use crate::comparison::*;
use crate::manifest::*;
use crate::report::*;
use crate::workspace::*;
use crate::MAX_JOBS;
use serde::Serialize;
use std::collections::VecDeque;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use wait_timeout::ChildExt;

/// Paths, concurrency, and selection settings for [`verify`].
#[derive(Clone, Debug)]
pub struct VerifyOptions {
    /// Root directory containing level directories and [`crate::CHECKSUM_FILE`].
    pub root: PathBuf,
    /// Native compiler executable used for build targets.
    pub compiler: PathBuf,
    /// Directory that receives JSON and text reports.
    pub report_directory: PathBuf,
    /// Worker count in the inclusive range `1..=MAX_JOBS`.
    pub jobs: usize,
    /// Case selectors applied before execution.
    pub filters: Filters,
    /// Optional compiler-owned IR profile passed to build targets.
    ///
    /// Reader targets do not use the code-generation pipeline and therefore
    /// do not receive this option.
    pub ir_optimization: Option<String>,
    /// Optional candidate bundle forwarded only to native build targets.
    pub ir_experiment: Option<String>,
}

/// Interpreted outcome of one conformance case.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResultKind {
    /// Required behavior matched its expectation.
    Pass,
    /// An `xfail` or declared difference failed as expected.
    ExpectedFailure,
    /// The case is declared but not executable.
    Pending,
    /// The case does not target the current platform.
    Skipped,
    /// Required behavior did not match its expectation.
    Fail,
    /// An expected failure passed and should be reviewed for promotion.
    UnexpectedPass,
}

/// Machine-readable result for one case.
#[derive(Clone, Debug, Serialize)]
pub struct CaseReport {
    /// Stable case identifier.
    pub id: String,
    /// Conformance tier copied from the manifest.
    pub level: Level,
    /// Feature area copied from the manifest.
    pub area: String,
    /// Declared lifecycle status.
    pub status: CaseStatus,
    /// Outcome after applying status semantics.
    pub result: ResultKind,
    /// Wall-clock case duration in milliseconds.
    pub duration_ms: u128,
    /// Concise comparison or execution explanation.
    pub message: String,
    /// Stable diagnostic category when one can be extracted.
    pub error_category: Option<String>,
}

/// Aggregate counters for a verification or oracle run.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Summary {
    /// Number of selected cases.
    pub total: usize,
    /// Number of selected active cases.
    pub active: usize,
    /// Number of selected expected-failure cases.
    pub xfail: usize,
    /// Number of selected pending cases.
    pub pending: usize,
    /// Number of passing cases.
    pub passed: usize,
    /// Number of failures matching an `xfail` or expected difference.
    pub expected_failures: usize,
    /// Number of cases excluded by their platform allowlist.
    pub skipped: usize,
    /// Number of required cases that failed.
    pub failed: usize,
    /// Number of expected failures that now pass.
    pub unexpected_passes: usize,
    /// Wall-clock duration of the complete run in milliseconds.
    pub duration_ms: u128,
}

#[derive(Clone, Debug)]
pub(crate) struct ProcessOutput {
    pub(crate) status: ExitStatus,
    pub(crate) stdout: Vec<u8>,
    pub(crate) stderr: Vec<u8>,
    pub(crate) timed_out: bool,
}

#[derive(Clone, Debug)]
pub(crate) enum MatchResult {
    Match(String),
    Mismatch(String, Option<String>),
}

/// Executes selected fixtures and writes versioned JSON and text reports.
///
/// The checksum and every manifest are validated before worker threads start.
/// Each executable case runs in an isolated temporary directory. Reports are
/// sorted by identifier, so worker scheduling cannot affect serialized output.
///
/// An `xfail` that passes is reported as [`ResultKind::UnexpectedPass`] and
/// makes [`VerifyReport::success`] false; callers must promote it explicitly.
///
/// # Errors
///
/// Returns an error for invalid options, checksum or fixture failures, process
/// setup failures, and report I/O errors. A compiler or expectation mismatch is
/// represented in the returned report instead.
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
            let ir_optimization = options.ir_optimization.as_deref();
            let ir_experiment = options.ir_experiment.as_deref();
            scope.spawn(move || loop {
                let case = queue.lock().expect("case queue poisoned").pop_front();
                let Some(case) = case else {
                    break;
                };
                let report = execute_case(&case, compiler, ir_optimization, ir_experiment);
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
        schema_version: 2,
        checksum,
        success,
        summary,
        cases,
    };
    write_report(&options.report_directory, "report", &report)?;
    Ok(report)
}

pub(crate) fn execute_case(
    case: &Case,
    compiler: &Path,
    ir_optimization: Option<&str>,
    ir_experiment: Option<&str>,
) -> CaseReport {
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
    if !platform_matches(&case.manifest.run.platforms) {
        return CaseReport {
            id: case.manifest.id.clone(),
            level: case.manifest.level,
            area: case.manifest.area.clone(),
            status: case.manifest.status,
            result: ResultKind::Skipped,
            duration_ms: 0,
            message: format!(
                "case is restricted to platforms: {}",
                case.manifest.run.platforms.join(", ")
            ),
            error_category: None,
        };
    }

    let outcome = execute_target(case, compiler, ir_optimization, ir_experiment);
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

pub(crate) fn platform_matches(platforms: &[String]) -> bool {
    platforms.is_empty()
        || platforms
            .iter()
            .any(|platform| platform == std::env::consts::OS)
}

pub(crate) fn execute_target(
    case: &Case,
    compiler: &Path,
    ir_optimization: Option<&str>,
    ir_experiment: Option<&str>,
) -> MatchResult {
    let timeout = Duration::from_millis(case.manifest.timeout_ms);
    match case.manifest.target {
        Target::Reader => execute_reader(case, compiler, timeout),
        Target::BuildRun => {
            execute_build_run(case, compiler, timeout, ir_optimization, ir_experiment)
        }
        Target::BuildError => {
            execute_build_error(case, compiler, timeout, ir_optimization, ir_experiment)
        }
        Target::Project => MatchResult::Mismatch(
            "project execution is not implemented".to_string(),
            Some("unsupported".to_string()),
        ),
    }
}

pub(crate) fn execute_reader(case: &Case, compiler: &Path, timeout: Duration) -> MatchResult {
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
        let stderr = output_text(&output.stderr);
        return MatchResult::Mismatch(
            format!("reader failed: {}", normalize_text(&stderr, case)),
            error_category(&stderr),
        );
    }
    let stdout = match std::str::from_utf8(&output.stdout) {
        Ok(value) => value,
        Err(error) => {
            return MatchResult::Mismatch(
                format!("reader stdout is not valid UTF-8: {error}"),
                Some("runner-error".to_string()),
            );
        }
    };
    let expected = match fs::read_to_string(case.directory.join("expected.edn")) {
        Ok(value) => value,
        Err(error) => {
            return MatchResult::Mismatch(
                format!("cannot read expected.edn: {error}"),
                Some("fixture-error".to_string()),
            );
        }
    };
    match structurally_equal(&expected, stdout) {
        Ok(true) => MatchResult::Match("reader forms match structurally".to_string()),
        Ok(false) => MatchResult::Mismatch(
            format!(
                "reader mismatch\nexpected:\n{}\nactual:\n{}",
                normalize_newlines(&expected),
                normalize_newlines(stdout)
            ),
            Some("output-mismatch".to_string()),
        ),
        Err(error) => MatchResult::Mismatch(error, Some("fixture-error".to_string())),
    }
}

pub(crate) fn execute_build_run(
    case: &Case,
    compiler: &Path,
    timeout: Duration,
    ir_optimization: Option<&str>,
    ir_experiment: Option<&str>,
) -> MatchResult {
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
    if let Some(profile) = ir_optimization {
        build.arg("--ir-opt").arg(profile);
    }
    if let Some(experiment) = ir_experiment {
        build.arg("--ir-experiment").arg(experiment);
    }
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
        let stderr = output_text(&build_output.stderr);
        return MatchResult::Mismatch(
            format!("build failed: {}", normalize_text(&stderr, case)),
            error_category(&stderr),
        );
    }

    let work_directory = temporary.path().join("work");
    if let Err(error) = prepare_work_directory(case, &work_directory) {
        return MatchResult::Mismatch(error, Some("fixture-error".to_string()));
    }
    let mut run = Command::new(&executable);
    run.args(&case.manifest.run.args)
        .envs(&case.manifest.run.env)
        .current_dir(&work_directory);
    if case.manifest.gc_stress {
        run.env("CLJN_GC_STRESS", "1");
    }
    let stdin = match case.manifest.run.stdin.as_deref() {
        Some(relative) => match safe_join(&case.directory, relative).and_then(|path| {
            fs::read(&path).map_err(|error| format!("cannot read {}: {error}", path.display()))
        }) {
            Ok(bytes) => Some(bytes),
            Err(error) => {
                return MatchResult::Mismatch(error, Some("fixture-error".to_string()));
            }
        },
        None => None,
    };
    let run_output = match run_process_with_stdin(&mut run, timeout, stdin.as_deref()) {
        Ok(output) => output,
        Err(error) => {
            return MatchResult::Mismatch(error, Some("runner-error".to_string()));
        }
    };
    if run_output.timed_out {
        return MatchResult::Mismatch("program timed out".to_string(), Some("timeout".to_string()));
    }
    let actual_exit = run_output.status.code();
    if actual_exit != Some(case.manifest.run.expected_exit) {
        let stderr = output_text(&run_output.stderr);
        return MatchResult::Mismatch(
            format!(
                "exit status mismatch: expected {}, got {:?}; stderr: {}",
                case.manifest.run.expected_exit,
                actual_exit,
                normalize_text(&stderr, case)
            ),
            error_category(&stderr),
        );
    }

    if let Err(error) = compare_stream(
        case,
        "stdout",
        "expected.stdout",
        "expected.stdout.bin",
        &run_output.stdout,
        true,
    ) {
        return MatchResult::Mismatch(error, Some("output-mismatch".to_string()));
    }
    if let Err(error) = compare_stream(
        case,
        "stderr",
        "expected.stderr",
        "expected.stderr.bin",
        &run_output.stderr,
        false,
    ) {
        let stderr = output_text(&run_output.stderr);
        return MatchResult::Mismatch(error, error_category(&stderr));
    }
    if let Err(error) = compare_work_directory(case, &work_directory) {
        return MatchResult::Mismatch(error, Some("filesystem-mismatch".to_string()));
    }
    MatchResult::Match("native process contract matches".to_string())
}

pub(crate) fn compare_stream(
    case: &Case,
    label: &str,
    text_name: &str,
    binary_name: &str,
    actual: &[u8],
    required: bool,
) -> Result<(), String> {
    let text_path = case.directory.join(text_name);
    let binary_path = case.directory.join(binary_name);
    if binary_path.is_file() {
        let expected = fs::read(&binary_path)
            .map_err(|error| format!("cannot read {}: {error}", binary_path.display()))?;
        if expected != actual {
            return Err(format!(
                "{label} binary mismatch: expected {} bytes, got {} bytes",
                expected.len(),
                actual.len()
            ));
        }
        return Ok(());
    }
    if !text_path.is_file() {
        if !required && actual.is_empty() {
            return Ok(());
        }
        return Err(format!("missing {}", text_path.display()));
    }
    let expected = fs::read_to_string(&text_path)
        .map_err(|error| format!("cannot read {}: {error}", text_path.display()))?;
    let actual = std::str::from_utf8(actual)
        .map_err(|error| format!("{label} is not valid UTF-8: {error}"))?;
    let expected = normalize_newlines(&expected);
    let actual = normalize_newlines(actual);
    if expected != actual {
        return Err(format!(
            "{label} mismatch\nexpected:\n{expected}\nactual:\n{actual}"
        ));
    }
    Ok(())
}

pub(crate) fn execute_build_error(
    case: &Case,
    compiler: &Path,
    timeout: Duration,
    ir_optimization: Option<&str>,
    ir_experiment: Option<&str>,
) -> MatchResult {
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
    if let Some(profile) = ir_optimization {
        command.arg("--ir-opt").arg(profile);
    }
    if let Some(experiment) = ir_experiment {
        command.arg("--ir-experiment").arg(experiment);
    }
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
    let stderr = output_text(&output.stderr);
    let actual = normalize_text(&stderr, case);
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

pub(crate) fn run_process(
    command: &mut Command,
    timeout: Duration,
) -> Result<ProcessOutput, String> {
    run_process_with_stdin(command, timeout, None)
}

pub(crate) fn run_process_with_stdin(
    command: &mut Command,
    timeout: Duration,
    stdin: Option<&[u8]>,
) -> Result<ProcessOutput, String> {
    command
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|error| format!("cannot start {:?}: {error}", command.get_program()))?;
    let stdin_writer = match stdin {
        Some(bytes) => {
            let mut pipe = child
                .stdin
                .take()
                .ok_or_else(|| "cannot open process stdin".to_string())?;
            let bytes = bytes.to_vec();
            Some(thread::spawn(move || {
                pipe.write_all(&bytes)?;
                pipe.flush()
            }))
        }
        None => None,
    };
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
    if let Some(writer) = stdin_writer {
        writer
            .join()
            .map_err(|_| "stdin writer thread panicked".to_string())?
            .map_err(|error| format!("cannot write process stdin: {error}"))?;
    }
    Ok(ProcessOutput {
        status: status.0,
        stdout,
        stderr,
        timed_out: status.1,
    })
}

pub(crate) fn read_pipe(mut pipe: impl Read) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    pipe.read_to_end(&mut bytes)?;
    Ok(bytes)
}

pub(crate) fn output_text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// Converts CRLF and lone carriage returns to line feeds.
///
/// This is the only platform-dependent normalization applied before textual
/// fixture comparison.
pub fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

pub(crate) fn normalize_text(text: &str, case: &Case) -> String {
    normalize_newlines(text)
        .replace(&case.directory.to_string_lossy().to_string(), "<case>")
        .replace('\\', "/")
}

/// Extracts the stable category used in conformance reports.
///
/// Timeout text has precedence, followed by the first `[Edddd]` compiler
/// diagnostic code. Other non-empty output is classified as `runtime-error`.
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
