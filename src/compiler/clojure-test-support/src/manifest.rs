//! Manifest schema, path safety, and case discovery.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::{self};
use std::path::{Component, Path, PathBuf};

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
    pub(crate) fn directory(self) -> &'static str {
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

pub(crate) fn collect_manifests(directory: &Path, output: &mut Vec<PathBuf>) -> io::Result<()> {
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

pub(crate) fn load_case(root: &Path, manifest_path: &Path) -> Result<Case, String> {
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

pub(crate) fn validate_manifest(
    root: &Path,
    directory: &Path,
    manifest: &CaseManifest,
) -> Result<(), String> {
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

pub(crate) fn validate_relative_path(value: &str) -> Result<(), String> {
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

pub(crate) fn safe_join(base: &Path, relative: &str) -> Result<PathBuf, String> {
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
