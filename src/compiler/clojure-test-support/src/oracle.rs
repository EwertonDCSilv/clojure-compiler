//! Optional Clojure/JVM oracle: checking or refreshing eligible expectations.

use crate::checksum::*;
use crate::comparison::*;
use crate::execution::*;
use crate::manifest::*;
use crate::report::*;
use crate::workspace::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};
use tempfile::TempDir;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Operation performed by the optional Clojure/JVM oracle.
pub enum OracleMode {
    /// Compare JVM results with committed expectations without modifying them.
    Check,
    /// Replace eligible committed expectations with JVM results.
    Bless,
}

/// Paths, JVM configuration, and selectors for [`run_oracle`].
#[derive(Clone, Debug)]
pub struct OracleOptions {
    /// Whether expectations are checked or rewritten.
    pub mode: OracleMode,
    /// Conformance suite root.
    pub root: PathBuf,
    /// Directory that receives oracle JSON and text reports.
    pub report_directory: PathBuf,
    /// Classpath containing the pinned Clojure/JVM distribution.
    pub classpath: String,
    /// Java launcher executable.
    pub java: PathBuf,
    /// Clojure helper script invoked in the JVM process.
    pub helper: PathBuf,
    /// Case selectors applied before oracle eligibility rules.
    pub filters: Filters,
}

/// Runs the explicit Clojure/JVM comparison or blessing workflow.
///
/// Pending cases, cases with [`Oracle::NotApplicable`], and targets other than
/// reader or build-and-run are skipped. The function verifies the oracle's
/// Clojure version before processing fixtures.
///
/// # Errors
///
/// Returns an error for missing JVM configuration, a version mismatch, fixture
/// validation failures, process failures, unsafe blessing targets, or report
/// I/O errors. Per-case semantic differences are represented in the report.
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
        if !platform_matches(&case.manifest.run.platforms) {
            reports.push(CaseReport {
                id: case.manifest.id,
                level: case.manifest.level,
                area: case.manifest.area,
                status: case.manifest.status,
                result: ResultKind::Skipped,
                duration_ms: 0,
                message: "case does not target this platform".to_string(),
                error_category: None,
            });
            continue;
        }
        let oracle_mode = match case.manifest.target {
            Target::Reader => "reader",
            Target::BuildRun => "run",
            _ => unreachable!(),
        };
        let input = case
            .directory
            .join("input.clj")
            .canonicalize()
            .map_err(|error| {
                format!(
                    "cannot resolve oracle input for {}: {error}",
                    case.manifest.id
                )
            })?;
        let temporary = TempDir::new().map_err(|error| error.to_string())?;
        let work_directory = temporary.path().join("work");
        prepare_work_directory(&case, &work_directory)?;
        let mut command = Command::new(&options.java);
        command
            .arg("-cp")
            .arg(&options.classpath)
            .arg("clojure.main")
            .arg(&options.helper)
            .arg(oracle_mode)
            .arg(input)
            .args(&case.manifest.run.args)
            .envs(&case.manifest.run.env)
            .current_dir(&work_directory);
        let stdin = match case.manifest.run.stdin.as_deref() {
            Some(relative) => Some(fs::read(safe_join(&case.directory, relative)?).map_err(
                |error| format!("cannot read oracle stdin for {}: {error}", case.manifest.id),
            )?),
            None => None,
        };
        let output = run_process_with_stdin(
            &mut command,
            Duration::from_millis(case.manifest.timeout_ms),
            stdin.as_deref(),
        )?;
        let (result, message, category) = if output.timed_out {
            (
                ResultKind::Fail,
                "JVM oracle timed out".to_string(),
                Some("timeout".to_string()),
            )
        } else if output.status.code() != Some(case.manifest.run.expected_exit) {
            let stderr = output_text(&output.stderr);
            (
                ResultKind::Fail,
                format!("JVM oracle failed: {}", normalize_text(&stderr, &case)),
                error_category(&stderr),
            )
        } else if let Err(error) = compare_stream(
            &case,
            "stderr",
            "expected.stderr",
            "expected.stderr.bin",
            &output.stderr,
            false,
        ) {
            let stderr = output_text(&output.stderr);
            (
                ResultKind::Fail,
                format!("JVM oracle stderr mismatch: {error}"),
                error_category(&stderr),
            )
        } else if let Err(error) = compare_work_directory(&case, &work_directory) {
            (
                ResultKind::Fail,
                format!("JVM oracle filesystem mismatch: {error}"),
                Some("filesystem-mismatch".to_string()),
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
        schema_version: 2,
        checksum: verify_checksums(&options.root)?,
        success,
        summary,
        cases: reports,
    };
    write_report(&options.report_directory, "oracle-report", &report)?;
    Ok(report)
}

pub(crate) fn verify_oracle_version(options: &OracleOptions) -> Result<(), String> {
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
        let stderr = output_text(&output.stderr);
        return Err(format!(
            "cannot start the Clojure/JVM oracle: {}",
            normalize_newlines(&stderr)
        ));
    }
    let stdout = output_text(&output.stdout);
    if stdout.trim() != "1.12.5" {
        return Err(format!(
            "oracle version must be Clojure/JVM 1.12.5, found `{}`",
            stdout.trim()
        ));
    }
    Ok(())
}

pub(crate) fn oracle_result(
    mode: OracleMode,
    case: &Case,
    stdout: &[u8],
) -> Result<(ResultKind, String, Option<String>), String> {
    let expected_path = match case.manifest.target {
        Target::Reader => case.directory.join("expected.edn"),
        Target::BuildRun if case.directory.join("expected.stdout.bin").is_file() => {
            case.directory.join("expected.stdout.bin")
        }
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
        let output = if expected_path.extension().and_then(|value| value.to_str()) == Some("bin") {
            stdout.to_vec()
        } else {
            normalize_newlines(
                std::str::from_utf8(stdout)
                    .map_err(|error| format!("JVM oracle stdout is not UTF-8: {error}"))?,
            )
            .into_bytes()
        };
        fs::write(&expected_path, output)
            .map_err(|error| format!("cannot bless {}: {error}", expected_path.display()))?;
        return Ok((
            ResultKind::Pass,
            "expectation updated from Clojure/JVM 1.12.5".to_string(),
            None,
        ));
    }

    let expected = fs::read(&expected_path)
        .map_err(|error| format!("cannot read {}: {error}", expected_path.display()))?;
    let equal = if expected_path.extension().and_then(|value| value.to_str()) == Some("bin") {
        expected == stdout
    } else if case.manifest.target == Target::Reader {
        structurally_equal(
            std::str::from_utf8(&expected)
                .map_err(|error| format!("expected reader output is not UTF-8: {error}"))?,
            std::str::from_utf8(stdout)
                .map_err(|error| format!("JVM reader output is not UTF-8: {error}"))?,
        )?
    } else {
        normalize_newlines(
            std::str::from_utf8(&expected)
                .map_err(|error| format!("expected stdout is not UTF-8: {error}"))?,
        ) == normalize_newlines(
            std::str::from_utf8(stdout)
                .map_err(|error| format!("JVM stdout is not UTF-8: {error}"))?,
        )
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
