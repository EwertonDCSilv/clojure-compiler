//! Versioned JSON/text reports and the human-readable summary line.

use crate::execution::*;
use crate::manifest::*;
use serde::Serialize;
use std::fs;
use std::path::Path;

/// Versioned report emitted by verification and oracle operations.
#[derive(Clone, Debug, Serialize)]
pub struct VerifyReport {
    /// Report-schema revision for downstream consumers.
    pub schema_version: u32,
    /// SHA-256 digest of the committed checksum manifest.
    pub checksum: String,
    /// Whether there are zero failures and zero unexpected passes.
    pub success: bool,
    /// Aggregate counters and duration.
    pub summary: Summary,
    /// Per-case results sorted by case identifier.
    pub cases: Vec<CaseReport>,
}

pub(crate) fn summarize(cases: &[CaseReport]) -> Summary {
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
            ResultKind::Skipped => summary.skipped += 1,
            ResultKind::Fail => summary.failed += 1,
            ResultKind::UnexpectedPass => summary.unexpected_passes += 1,
            ResultKind::Pending => {}
        }
    }
    summary
}

pub(crate) fn write_report(
    report_directory: &Path,
    stem: &str,
    report: &VerifyReport,
) -> Result<(), String> {
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
         skipped: {}\n\
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
        summary.skipped,
        summary.failed,
        summary.unexpected_passes,
        summary.duration_ms
    );
    fs::write(report_directory.join(format!("{stem}-summary.txt")), text)
        .map_err(|error| format!("cannot write text report: {error}"))
}

/// Formats a one-line, stable human summary of a verification report.
pub fn human_summary(report: &VerifyReport) -> String {
    let summary = &report.summary;
    format!(
        "{}: {} active, {} xfail, {} pending; {} passed, {} expected failures, {} skipped, {} failed, {} unexpected passes ({} ms)",
        if report.success { "PASS" } else { "FAIL" },
        summary.active,
        summary.xfail,
        summary.pending,
        summary.passed,
        summary.expected_failures,
        summary.skipped,
        summary.failed,
        summary.unexpected_passes,
        summary.duration_ms
    )
}
