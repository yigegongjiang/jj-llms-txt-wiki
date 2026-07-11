use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tempfile::Builder;

/// A staged, in-progress copy of a site's content. Content is written into a
/// hidden `.{site}.sync.*` working directory next to the final `{site}` target,
/// then swapped in atomically by [`Snapshot::commit`]. The working directory is
/// deliberately *not* auto-deleted: if the process is interrupted before commit,
/// the partial download survives on disk and the next run adopts it (resume).
pub struct Snapshot {
    working: PathBuf,
    target: PathBuf,
    root: PathBuf,
    site: String,
    resumed: bool,
}

impl Snapshot {
    pub fn new(root: &Path, site: &str) -> Result<Self, String> {
        fs::create_dir_all(root)
            .map_err(|error| format!("create output directory {}: {error}", root.display()))?;
        if !root.is_dir() {
            return Err(format!(
                "output path is not a directory: {}",
                root.display()
            ));
        }
        let target = root.join(site);
        if target.exists() && !target.is_dir() {
            return Err(format!(
                "site path is not a directory: {}",
                target.display()
            ));
        }

        // Adopt an interrupted earlier run: reuse the most recent leftover working
        // directory as this run's staging area (zero-copy), and garbage-collect any
        // others so partials never pile up. A single partial only ever grows, so
        // "most recent" is also the most complete — no progress is lost.
        let prefix = format!(".{site}.sync.");
        let mut partials = stale_partials(root, &prefix);
        partials.sort_by_key(|partial| std::cmp::Reverse(partial.1));
        let mut partials = partials.into_iter();

        let (working, resumed) = match partials.next() {
            Some((newest, _)) => (newest, true),
            None => {
                let directory = Builder::new()
                    .prefix(&prefix)
                    .tempdir_in(root)
                    .map_err(|error| {
                        format!("create temporary snapshot in {}: {error}", root.display())
                    })?
                    .keep();
                (directory, false)
            }
        };
        for (stale, _) in partials {
            let _ = fs::remove_dir_all(&stale);
        }

        // An adopted partial may hold `.part` temporaries from a mid-write
        // interrupt. They are not real content and must never reach the committed
        // snapshot, so drop them before this run writes into the directory.
        if resumed {
            sweep_part_files(&working);
        }

        Ok(Self {
            working,
            target,
            root: root.to_path_buf(),
            site: site.to_owned(),
            resumed,
        })
    }

    /// Create an empty staging directory and discard interrupted partials.
    /// Aggregated inputs are complete snapshots, so resuming their old files
    /// would retain pages removed from the latest bundle.
    pub fn fresh(root: &Path, site: &str) -> Result<Self, String> {
        fs::create_dir_all(root)
            .map_err(|error| format!("create output directory {}: {error}", root.display()))?;
        if !root.is_dir() {
            return Err(format!(
                "output path is not a directory: {}",
                root.display()
            ));
        }
        let target = root.join(site);
        if target.exists() && !target.is_dir() {
            return Err(format!(
                "site path is not a directory: {}",
                target.display()
            ));
        }

        let prefix = format!(".{site}.sync.");
        for (partial, _) in stale_partials(root, &prefix) {
            fs::remove_dir_all(&partial)
                .map_err(|error| format!("remove stale snapshot {}: {error}", partial.display()))?;
        }
        let working = Builder::new()
            .prefix(&prefix)
            .tempdir_in(root)
            .map_err(|error| format!("create temporary snapshot in {}: {error}", root.display()))?
            .keep();

        Ok(Self {
            working,
            target,
            root: root.to_path_buf(),
            site: site.to_owned(),
            resumed: false,
        })
    }

    pub fn path(&self) -> &Path {
        &self.working
    }

    /// Whether this snapshot adopted an interrupted earlier run's partial content.
    pub fn resumed(&self) -> bool {
        self.resumed
    }

    pub fn discard(self) {
        let _ = fs::remove_dir_all(self.working);
    }

    pub fn commit(self) -> Result<(), String> {
        let Snapshot {
            working,
            target,
            root,
            site,
            ..
        } = self;

        let backup = if target.exists() {
            let placeholder = Builder::new()
                .prefix(&format!(".{site}.backup."))
                .tempdir_in(&root)
                .map_err(|error| format!("create snapshot backup path: {error}"))?;
            let backup = placeholder.path().to_path_buf();
            placeholder
                .close()
                .map_err(|error| format!("prepare snapshot backup path: {error}"))?;
            fs::rename(&target, &backup).map_err(|error| {
                format!("move old snapshot {} to backup: {error}", target.display())
            })?;
            Some(backup)
        } else {
            None
        };

        if let Err(error) = fs::rename(&working, &target) {
            let restore = backup.as_ref().map(|backup| fs::rename(backup, &target));
            let _ = fs::remove_dir_all(&working);
            return match restore {
                Some(Err(restore_error)) => Err(format!(
                    "commit snapshot {}: {error}; restore old snapshot: {restore_error}",
                    target.display()
                )),
                _ => Err(format!("commit snapshot {}: {error}", target.display())),
            };
        }

        if let Some(backup) = backup
            && let Err(error) = fs::remove_dir_all(&backup)
        {
            let failed_snapshot = working;
            if let Err(rollback_error) = fs::rename(&target, &failed_snapshot) {
                return Err(format!(
                    "remove old snapshot {}: {error}; begin rollback: {rollback_error}",
                    backup.display()
                ));
            }
            if let Err(rollback_error) = fs::rename(&backup, &target) {
                let restore_new = fs::rename(&failed_snapshot, &target);
                return match restore_new {
                    Ok(()) => Err(format!(
                        "remove old snapshot {}: {error}; restore old snapshot: {rollback_error}",
                        backup.display()
                    )),
                    Err(restore_error) => Err(format!(
                        "remove old snapshot {}: {error}; restore old snapshot: {rollback_error}; restore new snapshot: {restore_error}",
                        backup.display()
                    )),
                };
            }
            if let Err(cleanup_error) = fs::remove_dir_all(&failed_snapshot) {
                return Err(format!(
                    "remove old snapshot {}: {error}; remove rolled-back snapshot {}: {cleanup_error}",
                    backup.display(),
                    failed_snapshot.display()
                ));
            }
            return Err(format!("remove old snapshot {}: {error}", backup.display()));
        }

        Ok(())
    }
}

/// Collect this site's leftover `.{site}.sync.*` working directories under `root`,
/// each paired with its modification time so the caller can pick the most recent.
fn stale_partials(root: &Path, prefix: &str) -> Vec<(PathBuf, SystemTime)> {
    let mut partials = Vec::new();
    let Ok(entries) = fs::read_dir(root) else {
        return partials;
    };
    for entry in entries.flatten() {
        let matches = entry
            .file_name()
            .to_str()
            .is_some_and(|name| name.starts_with(prefix));
        if !matches {
            continue;
        }
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let mtime = entry
            .metadata()
            .and_then(|meta| meta.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);
        partials.push((path, mtime));
    }
    partials
}

/// Recursively delete `*.part` staging files left behind by an interrupted atomic
/// write, so an adopted partial contains only complete files.
fn sweep_part_files(dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            sweep_part_files(&path);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("part") {
            let _ = fs::remove_file(&path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Snapshot;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn creates_and_replaces_complete_snapshots() {
        let output = tempdir().unwrap();
        let first = Snapshot::new(output.path(), "docs").unwrap();
        fs::write(first.path().join("old.md"), "old").unwrap();
        first.commit().unwrap();
        assert_eq!(
            fs::read_to_string(output.path().join("docs/old.md")).unwrap(),
            "old"
        );

        let second = Snapshot::new(output.path(), "docs").unwrap();
        fs::write(second.path().join("new.md"), "new").unwrap();
        second.commit().unwrap();
        assert!(!output.path().join("docs/old.md").exists());
        assert_eq!(
            fs::read_to_string(output.path().join("docs/new.md")).unwrap(),
            "new"
        );
        assert_eq!(fs::read_dir(output.path()).unwrap().count(), 1);
    }

    #[test]
    fn restores_old_snapshot_when_new_rename_fails() {
        let output = tempdir().unwrap();
        let target = output.path().join("docs");
        fs::create_dir(&target).unwrap();
        fs::write(target.join("old.md"), "old").unwrap();
        let snapshot = Snapshot::new(output.path(), "docs").unwrap();
        fs::remove_dir_all(snapshot.path()).unwrap();
        assert!(snapshot.commit().is_err());
        assert_eq!(fs::read_to_string(target.join("old.md")).unwrap(), "old");
        assert_eq!(fs::read_dir(output.path()).unwrap().count(), 1);
    }

    #[test]
    fn refuses_non_directory_target() {
        let output = tempdir().unwrap();
        fs::write(output.path().join("docs"), "file").unwrap();
        assert!(Snapshot::new(output.path(), "docs").is_err());
    }

    #[test]
    fn adopts_leftover_partial_and_sweeps_part_files() {
        let output = tempdir().unwrap();
        let root = output.path();

        let first = Snapshot::new(root, "docs").unwrap();
        assert!(!first.resumed(), "no leftover to adopt on a fresh run");
        let partial = first.path().to_path_buf();
        fs::write(partial.join("done.md"), "done").unwrap();
        fs::write(partial.join("done.md.part"), "half-written").unwrap();
        drop(first); // interrupted: working dir is kept, not deleted

        let resumed = Snapshot::new(root, "docs").unwrap();
        assert!(resumed.resumed(), "second run adopts the leftover");
        assert_eq!(
            resumed.path(),
            partial,
            "adopts the same directory in place"
        );
        assert_eq!(
            fs::read_to_string(resumed.path().join("done.md")).unwrap(),
            "done"
        );
        assert!(
            !resumed.path().join("done.md.part").exists(),
            "stray .part is swept before reuse"
        );
    }

    #[test]
    fn fresh_discards_every_leftover_partial() {
        let output = tempdir().unwrap();
        let root = output.path();
        let interrupted = Snapshot::new(root, "docs").unwrap();
        let old = interrupted.path().to_path_buf();
        fs::write(old.join("stale.md"), "stale").unwrap();
        drop(interrupted);

        let fresh = Snapshot::fresh(root, "docs").unwrap();
        assert!(!fresh.resumed());
        assert_ne!(fresh.path(), old);
        assert!(!old.exists());
        assert!(!fresh.path().join("stale.md").exists());
    }

    #[test]
    fn adopts_newest_partial_and_garbage_collects_others() {
        let output = tempdir().unwrap();
        let root = output.path();

        let first = Snapshot::new(root, "docs").unwrap();
        let older = first.path().to_path_buf();
        fs::write(older.join("first.md"), "1").unwrap();
        drop(first);

        std::thread::sleep(std::time::Duration::from_millis(20));
        let newer = root.join(".docs.sync.newer");
        fs::create_dir(&newer).unwrap();
        fs::write(newer.join("second.md"), "2").unwrap();

        let adopted = Snapshot::new(root, "docs").unwrap();
        assert!(adopted.resumed());
        assert_eq!(adopted.path(), newer, "newest partial is adopted");
        assert!(!older.exists(), "older partial is garbage-collected");
        let remaining = fs::read_dir(root)
            .unwrap()
            .filter(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .starts_with(".docs.sync.")
            })
            .count();
        assert_eq!(remaining, 1, "exactly one partial survives");
    }
}
