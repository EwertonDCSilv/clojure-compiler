//! SHA-256 fixture-integrity manifest (checksums.sha256).

use crate::manifest::*;
use crate::CHECKSUM_FILE;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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

pub(crate) fn collect_case_files(directory: &Path, output: &mut Vec<PathBuf>) -> io::Result<()> {
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

pub(crate) fn parse_checksums(text: &str) -> Result<BTreeMap<String, String>, String> {
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

pub(crate) fn hex_digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
