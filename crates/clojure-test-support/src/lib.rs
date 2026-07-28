//! Execution engine and manifest model for the compiler conformance suite.
//!
//! A conformance case is a directory containing a strict `case.toml` manifest,
//! source files, and committed expectations. [`discover_cases`] validates that
//! schema, while [`verify`] copies each selected case to an isolated temporary
//! directory, invokes the native compiler, and compares process output and
//! filesystem effects with the fixture.
//!
//! Normal verification is deterministic and has no JVM or network dependency.
//! [`run_oracle`] is a separate maintainer operation for checking or refreshing
//! eligible expectations against the pinned Clojure/JVM implementation.

use clojure_span::SourceMap;
use clojure_syntax::Form;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use wait_timeout::ChildExt;

/// Maximum number of case workers accepted by [`verify`].
pub const MAX_JOBS: usize = 4;
/// Name of the fixture-integrity manifest stored at the suite root.
pub const CHECKSUM_FILE: &str = "checksums.sha256";

/// Progressive conformance tier assigned to a case.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Level {
    /// Reader and surface-syntax behavior.
    A,
    /// Core language semantics.
    B,
    /// Compiled standard-library behavior.
    C,
    /// Pure third-party library behavior.
    D,
    /// Ecosystem and application-level behavior.
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

/// Lifecycle state of a conformance case.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaseStatus {
    /// Required behavior that must pass.
    Active,
    /// Known failure that must continue to fail until explicitly promoted.
    Xfail,
    /// Declared coverage that is not executed yet.
    Pending,
}

/// Source or intent of a conformance expectation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaseClass {
    /// Behavior required by the language specification or project contract.
    Spec,
    /// Behavior copied from an official Clojure test or example.
    Official,
    /// Intentional difference from Clojure/JVM.
    ExpectedDiff,
    /// Capability outside the native compiler's supported surface.
    Unsupported,
}

/// Compiler operation exercised by a case.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Target {
    /// Parse forms and compare their structural representation.
    Reader,
    /// Build a native executable, run it, and compare its effects.
    BuildRun,
    /// Build a source file and compare the expected diagnostic failure.
    BuildError,
    /// Build and run a multi-file project fixture.
    Project,
}

/// Relationship between a committed expectation and Clojure/JVM.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Oracle {
    /// The JVM result must equal the committed expectation.
    Equal,
    /// A documented JVM/native difference is expected.
    ExpectedDiff,
    /// The case cannot be meaningfully evaluated by the JVM oracle.
    NotApplicable,
}

/// A symbolic link to create or require relative to a case sandbox.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SymlinkFixture {
    /// Relative link path.
    pub path: String,
    /// Link target as stored in the symbolic link.
    pub target: String,
}

/// Process and filesystem settings for an executable case.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RunConfig {
    /// Command-line arguments passed after the generated executable path.
    #[serde(default)]
    pub args: Vec<String>,
    /// Additional environment variables for the generated executable.
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    /// Optional UTF-8 data written to standard input.
    #[serde(default)]
    pub stdin: Option<String>,
    /// Expected process exit code.
    #[serde(default)]
    pub expected_exit: i32,
    /// Optional platform allowlist using the suite's platform identifiers.
    #[serde(default)]
    pub platforms: Vec<String>,
    /// Symbolic links created before the executable runs.
    #[serde(default)]
    pub setup_symlinks: Vec<SymlinkFixture>,
    /// Symbolic links that must exist after the executable exits.
    #[serde(default)]
    pub expected_symlinks: Vec<SymlinkFixture>,
}

/// Validated contents of a fixture's `case.toml`.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CaseManifest {
    /// Globally unique, stable case identifier.
    pub id: String,
    /// Progressive conformance tier.
    pub level: Level,
    /// Slash-separated feature area used for filtering and reporting.
    pub area: String,
    /// Lifecycle state controlling pass/fail interpretation.
    pub status: CaseStatus,
    /// Provenance or compatibility class.
    pub class: CaseClass,
    /// Compiler operation exercised by the fixture.
    pub target: Target,
    /// Required relationship with the optional JVM oracle.
    pub oracle: Oracle,
    /// Per-process timeout in milliseconds.
    pub timeout_ms: u64,
    /// Whether generated programs run with the runtime's GC stress mode enabled.
    pub gc_stress: bool,
    /// Human-readable rationale, required for non-active cases.
    pub reason: String,
    /// Issue, specification, or source reference that tracks the case.
    pub tracking: String,
    /// Optional project namespace used by namespace filters.
    #[serde(default)]
    pub namespace: Option<String>,
    /// Execution settings; defaults are used by non-executable cases.
    #[serde(default)]
    pub run: RunConfig,
}

/// A discovered manifest together with its fixture directory.
#[derive(Clone, Debug)]
pub struct Case {
    /// Parsed and validated manifest.
    pub manifest: CaseManifest,
    /// Absolute or caller-relative directory containing the fixture.
    pub directory: PathBuf,
}

/// Optional selectors applied after fixture discovery and validation.
#[derive(Clone, Debug, Default)]
pub struct Filters {
    /// Exact level to retain.
    pub level: Option<Level>,
    /// Substring that must occur in the case area.
    pub area: Option<String>,
    /// Exact lifecycle status to retain.
    pub status: Option<CaseStatus>,
    /// Substring that must occur in the optional namespace.
    pub namespace: Option<String>,
}

impl Filters {
    /// Returns whether `case` satisfies every configured selector.
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

/// Paths, concurrency, and selection settings for [`verify`].
#[derive(Clone, Debug)]
pub struct VerifyOptions {
    /// Root directory containing level directories and [`CHECKSUM_FILE`].
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

#[derive(Clone, Debug)]
struct ProcessOutput {
    status: ExitStatus,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    timed_out: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SnapshotEntry {
    Directory,
    File(Vec<u8>),
    Symlink(PathBuf),
}

#[derive(Clone, Debug)]
enum MatchResult {
    Match(String),
    Mismatch(String, Option<String>),
}

/// Parses a conformance level accepted by the command-line interface.
///
/// # Errors
///
/// Returns a message when `value` is not one of `A` through `E`,
/// case-insensitively.
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

/// Parses a manifest lifecycle status.
///
/// # Errors
///
/// Returns a message unless `value` is `active`, `xfail`, or `pending`.
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

/// Recursively discovers and validates every `case.toml` below `root`.
///
/// Discovery order is deterministic. The function rejects an empty suite,
/// duplicate identifiers, unknown TOML fields, invalid paths, inconsistent
/// target files, and manifests stored under the wrong level directory.
///
/// # Errors
///
/// Returns all manifest-validation errors joined by newlines, or an I/O error
/// if the suite cannot be traversed.
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
    if !(0..=255).contains(&manifest.run.expected_exit) {
        return fail("run.expected_exit must be between 0 and 255");
    }
    for platform in &manifest.run.platforms {
        if !matches!(platform.as_str(), "linux" | "macos" | "windows") {
            return fail("run.platforms accepts only linux, macos, or windows");
        }
    }
    for argument in &manifest.run.args {
        if argument.contains('\0') {
            return fail("run.args cannot contain NUL");
        }
    }
    for (name, value) in &manifest.run.env {
        if name.is_empty() || name.contains(['=', '\0']) || value.contains('\0') {
            return fail("run.env names must be non-empty and values cannot contain NUL");
        }
    }
    if manifest.target != Target::BuildRun && manifest.run != RunConfig::default() {
        return fail("the [run] table is valid only for build-run cases");
    }
    if let Some(stdin) = &manifest.run.stdin {
        let stdin_path = safe_join(directory, stdin)
            .map_err(|message| format!("{}: {message}", directory.join("case.toml").display()))?;
        if !stdin_path.is_file() {
            return fail("run.stdin must name a file inside the case directory");
        }
    }
    for symlink in manifest
        .run
        .setup_symlinks
        .iter()
        .chain(&manifest.run.expected_symlinks)
    {
        validate_relative_path(&symlink.path).map_err(|message| {
            format!(
                "{}: invalid symlink path: {message}",
                directory.join("case.toml").display()
            )
        })?;
        validate_relative_path(&symlink.target).map_err(|message| {
            format!(
                "{}: invalid symlink target: {message}",
                directory.join("case.toml").display()
            )
        })?;
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
        match manifest.target {
            Target::Reader if !directory.join("expected.edn").is_file() => {
                return fail("expected.edn is required for this executable case");
            }
            Target::BuildError if !directory.join("expected.stderr").is_file() => {
                return fail("expected.stderr is required for this executable case");
            }
            Target::BuildRun => {
                let text = directory.join("expected.stdout").is_file();
                let binary = directory.join("expected.stdout.bin").is_file();
                if text == binary {
                    return fail(
                        "build-run requires exactly one of expected.stdout or expected.stdout.bin",
                    );
                }
                if directory.join("expected.stderr").is_file()
                    && directory.join("expected.stderr.bin").is_file()
                {
                    return fail(
                        "build-run accepts at most one of expected.stderr or expected.stderr.bin",
                    );
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_relative_path(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("path must not be empty".to_string());
    }
    let path = Path::new(value);
    if path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(format!(
            "`{value}` must contain only relative path components"
        ));
    }
    Ok(())
}

fn safe_join(base: &Path, relative: &str) -> Result<PathBuf, String> {
    validate_relative_path(relative)?;
    Ok(base.join(relative))
}

/// Discovers the complete suite and returns cases selected by `filters`.
///
/// Validation always covers the complete suite, including filtered-out cases.
///
/// # Errors
///
/// Propagates discovery and manifest-validation failures from
/// [`discover_cases`].
pub fn list_cases(root: &Path, filters: &Filters) -> Result<Vec<Case>, String> {
    discover_cases(root).map(|cases| {
        cases
            .into_iter()
            .filter(|case| filters.matches(case))
            .collect()
    })
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
            scope.spawn(move || loop {
                let case = queue.lock().expect("case queue poisoned").pop_front();
                let Some(case) = case else {
                    break;
                };
                let report = execute_case(&case, compiler, ir_optimization);
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

fn execute_case(case: &Case, compiler: &Path, ir_optimization: Option<&str>) -> CaseReport {
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

    let outcome = execute_target(case, compiler, ir_optimization);
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

fn platform_matches(platforms: &[String]) -> bool {
    platforms.is_empty()
        || platforms
            .iter()
            .any(|platform| platform == std::env::consts::OS)
}

fn execute_target(case: &Case, compiler: &Path, ir_optimization: Option<&str>) -> MatchResult {
    let timeout = Duration::from_millis(case.manifest.timeout_ms);
    match case.manifest.target {
        Target::Reader => execute_reader(case, compiler, timeout),
        Target::BuildRun => execute_build_run(case, compiler, timeout, ir_optimization),
        Target::BuildError => execute_build_error(case, compiler, timeout, ir_optimization),
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

fn execute_build_run(
    case: &Case,
    compiler: &Path,
    timeout: Duration,
    ir_optimization: Option<&str>,
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

fn compare_stream(
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

fn prepare_work_directory(case: &Case, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination)
        .map_err(|error| format!("cannot create {}: {error}", destination.display()))?;
    let before = case.directory.join("work.before");
    if before.exists() {
        if !before.is_dir() {
            return Err("work.before must be a directory".to_string());
        }
        copy_directory_contents(&before, destination)?;
    }
    for symlink in &case.manifest.run.setup_symlinks {
        create_fixture_symlink(destination, symlink)?;
    }
    Ok(())
}

fn copy_directory_contents(source: &Path, destination: &Path) -> Result<(), String> {
    let mut entries = fs::read_dir(source)
        .map_err(|error| format!("cannot read {}: {error}", source.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let metadata = fs::symlink_metadata(&source_path)
            .map_err(|error| format!("cannot inspect {}: {error}", source_path.display()))?;
        if metadata.file_type().is_symlink() {
            let target = fs::read_link(&source_path)
                .map_err(|error| format!("cannot read link {}: {error}", source_path.display()))?;
            create_symlink(&target, &destination_path)?;
        } else if metadata.is_dir() {
            fs::create_dir_all(&destination_path).map_err(|error| {
                format!("cannot create {}: {error}", destination_path.display())
            })?;
            copy_directory_contents(&source_path, &destination_path)?;
        } else if metadata.is_file() {
            fs::copy(&source_path, &destination_path).map_err(|error| {
                format!(
                    "cannot copy {} to {}: {error}",
                    source_path.display(),
                    destination_path.display()
                )
            })?;
        } else {
            return Err(format!(
                "unsupported fixture file type: {}",
                source_path.display()
            ));
        }
    }
    Ok(())
}

fn create_fixture_symlink(root: &Path, fixture: &SymlinkFixture) -> Result<(), String> {
    let link = safe_join(root, &fixture.path)?;
    let target = PathBuf::from(&fixture.target);
    fs::create_dir_all(link.parent().expect("validated symlink parent"))
        .map_err(|error| format!("cannot create symlink parent: {error}"))?;
    create_symlink(&target, &link)
}

#[cfg(unix)]
fn create_symlink(target: &Path, link: &Path) -> Result<(), String> {
    std::os::unix::fs::symlink(target, link)
        .map_err(|error| format!("cannot create symlink {}: {error}", link.display()))
}

#[cfg(not(unix))]
fn create_symlink(_target: &Path, link: &Path) -> Result<(), String> {
    Err(format!(
        "symlink fixtures are not supported on this host: {}",
        link.display()
    ))
}

fn compare_work_directory(case: &Case, actual_root: &Path) -> Result<(), String> {
    let expected_root = case.directory.join("work.after");
    if !expected_root.exists() && case.manifest.run.expected_symlinks.is_empty() {
        return Ok(());
    }
    if expected_root.exists() && !expected_root.is_dir() {
        return Err("work.after must be a directory".to_string());
    }
    let actual = snapshot_directory(actual_root)?;
    let mut expected = if expected_root.exists() {
        snapshot_directory(&expected_root)?
    } else {
        BTreeMap::new()
    };
    for symlink in &case.manifest.run.expected_symlinks {
        expected.insert(
            PathBuf::from(&symlink.path),
            SnapshotEntry::Symlink(PathBuf::from(&symlink.target)),
        );
    }
    if actual != expected {
        let actual_paths = actual
            .keys()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        let expected_paths = expected
            .keys()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        return Err(format!(
            "work directory mismatch; expected paths={expected_paths:?}, actual paths={actual_paths:?}"
        ));
    }
    Ok(())
}

fn snapshot_directory(root: &Path) -> Result<BTreeMap<PathBuf, SnapshotEntry>, String> {
    let mut snapshot = BTreeMap::new();
    collect_snapshot(root, root, &mut snapshot)?;
    Ok(snapshot)
}

fn collect_snapshot(
    root: &Path,
    directory: &Path,
    snapshot: &mut BTreeMap<PathBuf, SnapshotEntry>,
) -> Result<(), String> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| format!("cannot read {}: {error}", directory.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .map_err(|_| format!("{} escaped {}", path.display(), root.display()))?
            .to_path_buf();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("cannot inspect {}: {error}", path.display()))?;
        if metadata.file_type().is_symlink() {
            let target = fs::read_link(&path)
                .map_err(|error| format!("cannot read link {}: {error}", path.display()))?;
            snapshot.insert(relative, SnapshotEntry::Symlink(target));
        } else if metadata.is_dir() {
            snapshot.insert(relative, SnapshotEntry::Directory);
            collect_snapshot(root, &path, snapshot)?;
        } else if metadata.is_file() {
            let contents = fs::read(&path)
                .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
            snapshot.insert(relative, SnapshotEntry::File(contents));
        } else {
            return Err(format!("unsupported file type: {}", path.display()));
        }
    }
    Ok(())
}

fn execute_build_error(
    case: &Case,
    compiler: &Path,
    timeout: Duration,
    ir_optimization: Option<&str>,
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

fn run_process(command: &mut Command, timeout: Duration) -> Result<ProcessOutput, String> {
    run_process_with_stdin(command, timeout, None)
}

fn run_process_with_stdin(
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

fn read_pipe(mut pipe: impl Read) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    pipe.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn output_text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// Converts CRLF and lone carriage returns to line feeds.
///
/// This is the only platform-dependent normalization applied before textual
/// fixture comparison.
pub fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

fn normalize_text(text: &str, case: &Case) -> String {
    normalize_newlines(text)
        .replace(&case.directory.to_string_lossy().to_string(), "<case>")
        .replace('\\', "/")
}

/// Compares two sequences of Clojure forms after structural canonicalization.
///
/// Map entries and set elements are sorted during canonicalization, while
/// sequence order and metadata remain significant.
///
/// # Errors
///
/// Returns a rendered reader diagnostic if either input is not valid forms.
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
            ResultKind::Skipped => summary.skipped += 1,
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

/// Computes SHA-256 entries for every regular file inside discovered cases.
///
/// Paths use `/` separators and are relative to `root`. The top-level checksum
/// manifest itself is excluded because it is outside every case directory.
///
/// # Errors
///
/// Returns an error when cases are invalid or a fixture file cannot be read.
pub fn checksum_entries(root: &Path) -> Result<BTreeMap<String, String>, String> {
    let cases = discover_cases(root)?;
    let mut files = Vec::new();
    for case in cases {
        collect_case_files(&case.directory, &mut files)
            .map_err(|error| format!("{}: {error}", case.directory.display()))?;
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

fn collect_case_files(directory: &Path, output: &mut Vec<PathBuf>) -> io::Result<()> {
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_case_files(&path, output)?;
        } else if path.is_file() {
            output.push(path);
        }
    }
    Ok(())
}

/// Rewrites [`CHECKSUM_FILE`] from the current fixture contents.
///
/// The return value is the SHA-256 digest of the serialized checksum manifest.
/// This mutating operation is intended for explicit fixture maintenance.
///
/// # Errors
///
/// Returns an error if cases cannot be discovered, files cannot be hashed, or
/// the checksum manifest cannot be written.
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

/// Verifies fixture contents against the committed checksum manifest.
///
/// The return value is the SHA-256 digest of the manifest text itself.
///
/// # Errors
///
/// Returns an error for malformed entries or any missing, stale, or changed
/// fixture path.
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

fn oracle_result(
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
    fn tracked_pedestal_target_has_an_http_project_contract() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/conformance");
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
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/conformance");
        let cases = discover_cases(&root).expect("discover tracked conformance suite");
        let mut scenarios = BTreeMap::<String, HashSet<String>>::new();
        for case in cases.iter().filter(|case| {
            case.manifest.namespace.as_deref() == Some("cljn.io")
                && case.manifest.id != "c.cljn_io.filesystem.isolated_tree_and_symlink"
        }) {
            assert_eq!(case.manifest.status, CaseStatus::Xfail);
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
    fn verify_forwards_the_optional_ir_profile_only_to_build_targets() {
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
        })
        .expect("verify safe profile");

        assert!(report.success, "{:?}", report.cases);
        let arguments = fs::read_to_string(arguments).expect("compiler arguments");
        assert!(arguments.contains("--ir-opt safe"), "{arguments}");
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
