//! Measures Clojure 1.12.5 reader coverage against a frozen capability catalog.
//!
//! The catalog supplies the denominator independently of the fixtures that
//! happen to exist. Each capability owns normal, boundary, and error scenarios;
//! a scenario may be excluded only through an explicit not-applicable decision.

use crate::{discover_cases, Case, CaseStatus, Level, Oracle};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

const SCHEMA_VERSION: u32 = 1;
const GAP_ISSUE_BASE_URL: &str = "https://github.com/EwertonDCSilv/clojure-compiler/issues/";

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReaderCatalog {
    schema_version: u32,
    clojure_version: String,
    reference: String,
    #[serde(rename = "capability")]
    capabilities: Vec<Capability>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Capability {
    id: String,
    category: String,
    syntax: String,
    source: String,
    behavior: String,
    issue: u64,
    #[serde(default)]
    dependencies: Vec<u64>,
    decision: String,
    normal: Vec<String>,
    boundary: Vec<String>,
    error: Vec<String>,
    #[serde(default)]
    not_applicable: Vec<ScenarioName>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum ScenarioName {
    Normal,
    Boundary,
    Error,
}

impl ScenarioName {
    fn label(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Boundary => "boundary",
            Self::Error => "error",
        }
    }
}

/// Machine-readable reader coverage result.
#[derive(Clone, Debug, Serialize)]
pub struct ReaderCoverageReport {
    /// Catalog schema understood by this runner.
    pub schema_version: u32,
    /// Clojure version whose reader surface defines the denominator.
    pub clojure_version: String,
    /// Official source used to freeze the catalog.
    pub reference: String,
    /// Capability-level completion.
    pub capabilities: CapabilityCoverage,
    /// Scenario-level lifecycle and oracle evidence.
    pub scenarios: ScenarioCoverage,
    /// Percentage of applicable scenarios connected to at least one fixture.
    pub traceability_percent: f64,
    /// Percentage of applicable scenarios backed by active native fixtures.
    pub native_support_percent: f64,
    /// Percentage of applicable scenarios backed by active, equal JVM evidence.
    pub strict_oracle_parity_percent: f64,
    /// Applicable scenarios without strict equal JVM evidence.
    pub strict_remaining: usize,
    /// Stable issue URL prefix used by catalog entries.
    pub issue_base_url: String,
}

/// Completion state for catalog capabilities.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CapabilityCoverage {
    /// Number of cataloged capabilities.
    pub total: usize,
    /// Capabilities whose applicable scenarios are all active.
    pub complete: usize,
    /// Capabilities with at least one, but not all, active scenarios.
    pub partial: usize,
    /// Capabilities with no active scenario.
    pub missing: usize,
}

/// Counts for the frozen normal/boundary/error scenario denominator.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ScenarioCoverage {
    /// Scenarios included in the denominator.
    pub applicable: usize,
    /// Explicitly justified scenarios excluded from the denominator.
    pub excluded: usize,
    /// Applicable scenarios whose fixtures are all active.
    pub active: usize,
    /// Applicable scenarios containing a known expected-failure fixture.
    pub xfail: usize,
    /// Applicable scenarios containing a pending fixture.
    pub pending: usize,
    /// Applicable scenarios without any fixture.
    pub missing: usize,
    /// Active scenarios whose fixtures all require equality with the JVM.
    pub oracle_equal: usize,
    /// Active scenarios containing a declared JVM/native difference.
    pub expected_diff: usize,
    /// Active scenarios without an applicable JVM oracle.
    pub oracle_not_applicable: usize,
}

/// Loads, validates, and measures a reader coverage catalog.
///
/// # Errors
///
/// Returns a stable validation error when the catalog is malformed, references
/// an unknown fixture, or omits an existing level-A fixture.
pub fn load_reader_coverage(
    catalog_path: &Path,
    conformance_root: &Path,
) -> Result<ReaderCoverageReport, String> {
    let text = fs::read_to_string(catalog_path)
        .map_err(|error| format!("cannot read {}: {error}", catalog_path.display()))?;
    let catalog: ReaderCatalog = toml::from_str(&text)
        .map_err(|error| format!("{}: invalid schema: {error}", catalog_path.display()))?;
    let cases = discover_cases(conformance_root)?;
    measure_reader_coverage(catalog, &cases)
}

/// Formats the stable human-readable reader coverage summary.
pub fn reader_coverage_summary(report: &ReaderCoverageReport) -> String {
    format!(
        "Clojure {} reader syntax coverage\n\
         Capabilities: {} total; {} complete, {} partial, {} missing\n\
         Scenarios: {} applicable, {} explicitly excluded\n\
         Traceability: {:.2}% ({} missing fixture scenarios)\n\
         Native executable support: {:.2}% ({} active, {} xfail, {} pending)\n\
         Strict JVM parity evidence: {:.2}% ({} equal, {} remaining)\n\
         Classified active differences: {} expected-diff, {} oracle not-applicable",
        report.clojure_version,
        report.capabilities.total,
        report.capabilities.complete,
        report.capabilities.partial,
        report.capabilities.missing,
        report.scenarios.applicable,
        report.scenarios.excluded,
        report.traceability_percent,
        report.scenarios.missing,
        report.native_support_percent,
        report.scenarios.active,
        report.scenarios.xfail,
        report.scenarios.pending,
        report.strict_oracle_parity_percent,
        report.scenarios.oracle_equal,
        report.strict_remaining,
        report.scenarios.expected_diff,
        report.scenarios.oracle_not_applicable,
    )
}

fn measure_reader_coverage(
    catalog: ReaderCatalog,
    cases: &[Case],
) -> Result<ReaderCoverageReport, String> {
    let mut errors = Vec::new();
    validate_catalog_header(&catalog, &mut errors);

    let case_by_id: BTreeMap<_, _> = cases
        .iter()
        .map(|case| (case.manifest.id.as_str(), case))
        .collect();
    let mut capability_ids = HashSet::new();
    let mut referenced_fixtures = HashSet::new();
    let mut capabilities = CapabilityCoverage {
        total: catalog.capabilities.len(),
        ..CapabilityCoverage::default()
    };
    let mut scenarios = ScenarioCoverage::default();

    for capability in &catalog.capabilities {
        validate_capability(capability, &mut capability_ids, &mut errors);
        let excluded: HashSet<_> = capability.not_applicable.iter().copied().collect();
        let slots = [
            (ScenarioName::Normal, &capability.normal),
            (ScenarioName::Boundary, &capability.boundary),
            (ScenarioName::Error, &capability.error),
        ];
        let mut active_slots = 0;
        let mut applicable_slots = 0;

        for (name, fixture_ids) in slots {
            if excluded.contains(&name) {
                scenarios.excluded += 1;
                if !fixture_ids.is_empty() {
                    errors.push(format!(
                        "{}.{} is not-applicable but names fixtures",
                        capability.id,
                        name.label()
                    ));
                }
                continue;
            }
            applicable_slots += 1;
            scenarios.applicable += 1;
            if fixture_ids.is_empty() {
                scenarios.missing += 1;
                continue;
            }

            let mut slot_cases = Vec::new();
            let mut slot_ids = HashSet::new();
            for fixture_id in fixture_ids {
                if !slot_ids.insert(fixture_id) {
                    errors.push(format!(
                        "{}.{} repeats fixture `{fixture_id}`",
                        capability.id,
                        name.label()
                    ));
                    continue;
                }
                referenced_fixtures.insert(fixture_id.as_str());
                match case_by_id.get(fixture_id.as_str()) {
                    Some(case) if case.manifest.level == Level::A => slot_cases.push(*case),
                    Some(_) => errors.push(format!(
                        "{}.{} references non-level-A fixture `{fixture_id}`",
                        capability.id,
                        name.label()
                    )),
                    None => errors.push(format!(
                        "{}.{} references unknown fixture `{fixture_id}`",
                        capability.id,
                        name.label()
                    )),
                }
            }
            if slot_cases.len() != fixture_ids.len() {
                continue;
            }

            if slot_cases
                .iter()
                .all(|case| case.manifest.status == CaseStatus::Active)
            {
                scenarios.active += 1;
                active_slots += 1;
                if slot_cases
                    .iter()
                    .all(|case| case.manifest.oracle == Oracle::Equal)
                {
                    scenarios.oracle_equal += 1;
                } else if slot_cases
                    .iter()
                    .any(|case| case.manifest.oracle == Oracle::ExpectedDiff)
                {
                    scenarios.expected_diff += 1;
                } else {
                    scenarios.oracle_not_applicable += 1;
                }
            } else if slot_cases
                .iter()
                .any(|case| case.manifest.status == CaseStatus::Pending)
            {
                scenarios.pending += 1;
            } else {
                scenarios.xfail += 1;
            }
        }

        if active_slots == applicable_slots {
            capabilities.complete += 1;
        } else if active_slots > 0 {
            capabilities.partial += 1;
        } else {
            capabilities.missing += 1;
        }
    }

    for case in cases.iter().filter(|case| case.manifest.level == Level::A) {
        if !referenced_fixtures.contains(case.manifest.id.as_str()) {
            errors.push(format!(
                "level-A fixture `{}` is not cataloged",
                case.manifest.id
            ));
        }
    }

    if !errors.is_empty() {
        errors.sort();
        errors.dedup();
        return Err(errors.join("\n"));
    }

    let traceable = scenarios.applicable.saturating_sub(scenarios.missing);
    let strict_remaining = scenarios.applicable.saturating_sub(scenarios.oracle_equal);
    Ok(ReaderCoverageReport {
        schema_version: catalog.schema_version,
        clojure_version: catalog.clojure_version,
        reference: catalog.reference,
        traceability_percent: percentage(traceable, scenarios.applicable),
        native_support_percent: percentage(scenarios.active, scenarios.applicable),
        strict_oracle_parity_percent: percentage(scenarios.oracle_equal, scenarios.applicable),
        strict_remaining,
        issue_base_url: GAP_ISSUE_BASE_URL.to_string(),
        capabilities,
        scenarios,
    })
}

fn validate_catalog_header(catalog: &ReaderCatalog, errors: &mut Vec<String>) {
    if catalog.schema_version != SCHEMA_VERSION {
        errors.push(format!(
            "unsupported schema_version {}; expected {SCHEMA_VERSION}",
            catalog.schema_version
        ));
    }
    if catalog.clojure_version != "1.12.5" {
        errors.push(format!(
            "clojure_version must be 1.12.5, got `{}`",
            catalog.clojure_version
        ));
    }
    if catalog.reference.trim().is_empty() {
        errors.push("catalog reference must not be empty".to_string());
    }
    if catalog.capabilities.is_empty() {
        errors.push("catalog must contain at least one capability".to_string());
    }
}

fn validate_capability(
    capability: &Capability,
    ids: &mut HashSet<String>,
    errors: &mut Vec<String>,
) {
    if capability.id.is_empty()
        || !capability.id.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || matches!(character, '.' | '-')
        })
    {
        errors.push(format!(
            "invalid capability id `{}`; use lowercase ASCII, digits, '.' or '-'",
            capability.id
        ));
    } else if !ids.insert(capability.id.clone()) {
        errors.push(format!("duplicate capability id `{}`", capability.id));
    }
    for (field, value) in [
        ("category", &capability.category),
        ("syntax", &capability.syntax),
        ("source", &capability.source),
        ("behavior", &capability.behavior),
        ("decision", &capability.decision),
    ] {
        if value.trim().is_empty() {
            errors.push(format!("{}.{} must not be empty", capability.id, field));
        }
    }
    if capability.issue == 0 {
        errors.push(format!("{}.issue must be positive", capability.id));
    }
    if capability.dependencies.contains(&0) {
        errors.push(format!(
            "{}.dependencies must contain positive issue numbers",
            capability.id
        ));
    }
    let mut excluded = HashSet::new();
    for scenario in &capability.not_applicable {
        if !excluded.insert(*scenario) {
            errors.push(format!(
                "{} repeats not-applicable scenario `{}`",
                capability.id,
                scenario.label()
            ));
        }
    }
    if excluded.len() == 3 {
        errors.push(format!("{} cannot exclude every scenario", capability.id));
    }
}

fn percentage(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 * 100.0 / denominator as f64
    }
}

#[cfg(test)]
#[path = "../tests/unit/reader_coverage/mod.rs"]
mod tests;
