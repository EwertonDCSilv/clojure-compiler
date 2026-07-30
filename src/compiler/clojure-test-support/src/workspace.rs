//! Isolated per-case work directories, symlink fixtures, and filesystem snapshots.

use crate::manifest::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SnapshotEntry {
    Directory,
    File(Vec<u8>),
    Symlink(PathBuf),
}

pub(crate) fn prepare_work_directory(case: &Case, destination: &Path) -> Result<(), String> {
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

pub(crate) fn copy_directory_contents(source: &Path, destination: &Path) -> Result<(), String> {
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

pub(crate) fn create_fixture_symlink(root: &Path, fixture: &SymlinkFixture) -> Result<(), String> {
    let link = safe_join(root, &fixture.path)?;
    let target = PathBuf::from(&fixture.target);
    fs::create_dir_all(link.parent().expect("validated symlink parent"))
        .map_err(|error| format!("cannot create symlink parent: {error}"))?;
    create_symlink(&target, &link)
}

#[cfg(unix)]
pub(crate) fn create_symlink(target: &Path, link: &Path) -> Result<(), String> {
    std::os::unix::fs::symlink(target, link)
        .map_err(|error| format!("cannot create symlink {}: {error}", link.display()))
}

#[cfg(not(unix))]
pub(crate) fn create_symlink(_target: &Path, link: &Path) -> Result<(), String> {
    Err(format!(
        "symlink fixtures are not supported on this host: {}",
        link.display()
    ))
}

pub(crate) fn compare_work_directory(case: &Case, actual_root: &Path) -> Result<(), String> {
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

pub(crate) fn snapshot_directory(root: &Path) -> Result<BTreeMap<PathBuf, SnapshotEntry>, String> {
    let mut snapshot = BTreeMap::new();
    collect_snapshot(root, root, &mut snapshot)?;
    Ok(snapshot)
}

pub(crate) fn collect_snapshot(
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
