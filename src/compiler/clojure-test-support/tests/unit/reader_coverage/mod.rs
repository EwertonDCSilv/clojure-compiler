//! Unit tests for reader_coverage.rs.

use super::*;
use crate::{CaseClass, CaseManifest, RunConfig, Target};
use std::path::PathBuf;
use tempfile::TempDir;

fn case(id: &str, status: CaseStatus, oracle: Oracle) -> Case {
    Case {
        manifest: CaseManifest {
            id: id.into(),
            level: Level::A,
            area: "syntax/test".into(),
            status,
            class: if oracle == Oracle::ExpectedDiff {
                CaseClass::ExpectedDiff
            } else {
                CaseClass::Spec
            },
            target: Target::Reader,
            oracle,
            timeout_ms: 100,
            gc_stress: false,
            reason: "test".into(),
            tracking: "test".into(),
            namespace: None,
            run: RunConfig::default(),
        },
        directory: PathBuf::new(),
    }
}

#[test]
fn counts_catalog_scenarios_instead_of_fixture_count() {
    let catalog = ReaderCatalog {
        schema_version: 1,
        clojure_version: "1.12.5".into(),
        reference: "official".into(),
        capabilities: vec![Capability {
            id: "literal.integer".into(),
            category: "literals".into(),
            syntax: "42".into(),
            source: "LispReader intPat".into(),
            behavior: "Reads decimal integers.".into(),
            issue: 9,
            dependencies: vec![],
            decision: "Clojure/JVM equality.".into(),
            normal: vec!["a.normal".into()],
            boundary: vec!["a.boundary".into(), "a.second-boundary".into()],
            error: vec![],
            not_applicable: vec![],
        }],
    };
    let cases = vec![
        case("a.normal", CaseStatus::Active, Oracle::Equal),
        case("a.boundary", CaseStatus::Active, Oracle::ExpectedDiff),
        case("a.second-boundary", CaseStatus::Active, Oracle::Equal),
    ];

    let report = measure_reader_coverage(catalog, &cases).expect("valid catalog");

    assert_eq!(report.scenarios.applicable, 3);
    assert_eq!(report.scenarios.active, 2);
    assert_eq!(report.scenarios.missing, 1);
    assert_eq!(report.scenarios.oracle_equal, 1);
    assert_eq!(report.scenarios.expected_diff, 1);
}

#[test]
fn rejects_unknown_and_orphan_level_a_fixtures() {
    let catalog = ReaderCatalog {
        schema_version: 1,
        clojure_version: "1.12.5".into(),
        reference: "official".into(),
        capabilities: vec![Capability {
            id: "literal.integer".into(),
            category: "literals".into(),
            syntax: "42".into(),
            source: "LispReader intPat".into(),
            behavior: "Reads decimal integers.".into(),
            issue: 9,
            dependencies: vec![],
            decision: "Clojure/JVM equality.".into(),
            normal: vec!["a.unknown".into()],
            boundary: vec![],
            error: vec![],
            not_applicable: vec![],
        }],
    };
    let cases = vec![case("a.orphan", CaseStatus::Active, Oracle::Equal)];

    let error = measure_reader_coverage(catalog, &cases).expect_err("invalid catalog");

    assert!(error.contains("unknown fixture `a.unknown`"));
    assert!(error.contains("level-A fixture `a.orphan` is not cataloged"));
}

#[test]
fn excluded_scenario_is_not_part_of_the_denominator() {
    let catalog = ReaderCatalog {
        schema_version: 1,
        clojure_version: "1.12.5".into(),
        reference: "official".into(),
        capabilities: vec![Capability {
            id: "literal.nil".into(),
            category: "literals".into(),
            syntax: "nil".into(),
            source: "LispReader".into(),
            behavior: "Reads nil.".into(),
            issue: 9,
            dependencies: vec![],
            decision: "No malformed nil token exists; other spellings are symbols.".into(),
            normal: vec!["a.nil".into()],
            boundary: vec!["a.nil".into()],
            error: vec![],
            not_applicable: vec![ScenarioName::Error],
        }],
    };
    let cases = vec![case("a.nil", CaseStatus::Active, Oracle::Equal)];

    let report = measure_reader_coverage(catalog, &cases).expect("valid catalog");

    assert_eq!(report.scenarios.applicable, 2);
    assert_eq!(report.scenarios.excluded, 1);
    assert_eq!(report.scenarios.active, 2);
    assert_eq!(report.capabilities.complete, 1);
}

#[allow(dead_code)]
fn compile_time_schema_contract() {
    let _: fn(ReaderCatalog, &[Case]) -> Result<ReaderCoverageReport, String> =
        measure_reader_coverage;
    let _ = TempDir::new();
}
