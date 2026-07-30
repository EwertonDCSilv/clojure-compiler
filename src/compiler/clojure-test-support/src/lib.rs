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
//!
//! This crate is split by responsibility: `manifest` (schema, discovery,
//! filtering), `execution` (process spawning and dispatch), `workspace`
//! (isolated directories, symlinks, snapshots), `comparison` (structural
//! source-form equality), `checksum` (fixture-integrity manifest),
//! `report` (versioned reports), and `oracle` (the optional JVM check).
//! Submodules share crate-internal helpers freely; only the historical public
//! surface below is re-exported outside the crate.

mod reader_coverage;

pub use reader_coverage::{
    load_reader_coverage, reader_coverage_summary, CapabilityCoverage, ReaderCoverageReport,
    ScenarioCoverage,
};

mod checksum;
mod comparison;
mod execution;
mod manifest;
mod oracle;
mod report;
mod workspace;

pub use checksum::{checksum_entries, update_checksums, verify_checksums};
pub use comparison::structurally_equal;
pub use execution::{
    error_category, normalize_newlines, verify, CaseReport, ResultKind, Summary, VerifyOptions,
};
pub use manifest::{
    discover_cases, list_cases, parse_level, parse_status, Case, CaseClass, CaseManifest,
    CaseStatus, Filters, Level, Oracle, RunConfig, SymlinkFixture, Target,
};
pub use oracle::{run_oracle, OracleMode, OracleOptions};
pub use report::{human_summary, VerifyReport};

/// Maximum number of case workers accepted by [`verify`].
pub const MAX_JOBS: usize = 4;
/// Name of the fixture-integrity manifest stored at the suite root.
pub const CHECKSUM_FILE: &str = "checksums.sha256";

#[cfg(test)]
pub(crate) use checksum::*;
#[cfg(test)]
pub(crate) use execution::*;
#[cfg(test)]
pub(crate) use report::*;
#[cfg(test)]
pub(crate) use std::collections::{BTreeMap, HashSet};
#[cfg(test)]
pub(crate) use std::fs;
#[cfg(test)]
pub(crate) use std::path::{Path, PathBuf};
#[cfg(test)]
pub(crate) use std::process::Command;
#[cfg(test)]
pub(crate) use std::time::Duration;
#[cfg(test)]
pub(crate) use tempfile::TempDir;
#[cfg(test)]
pub(crate) use workspace::*;

#[cfg(test)]
#[path = "../tests/unit/lib/mod.rs"]
mod tests;
