use std::fs;
use std::path::{Path, PathBuf};
use tempfile::{Builder, TempDir};

pub struct Snapshot {
    temporary: Option<TempDir>,
    target: PathBuf,
    root: PathBuf,
    site: String,
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
        let temporary = Builder::new()
            .prefix(&format!(".{site}.sync."))
            .tempdir_in(root)
            .map_err(|error| format!("create temporary snapshot in {}: {error}", root.display()))?;
        Ok(Self {
            temporary: Some(temporary),
            target,
            root: root.to_path_buf(),
            site: site.to_owned(),
        })
    }

    pub fn path(&self) -> &Path {
        self.temporary
            .as_ref()
            .expect("snapshot temporary directory must exist")
            .path()
    }

    pub fn commit(mut self) -> Result<(), String> {
        let backup = if self.target.exists() {
            let placeholder = Builder::new()
                .prefix(&format!(".{}.backup.", self.site))
                .tempdir_in(&self.root)
                .map_err(|error| format!("create snapshot backup path: {error}"))?;
            let backup = placeholder.path().to_path_buf();
            placeholder
                .close()
                .map_err(|error| format!("prepare snapshot backup path: {error}"))?;
            fs::rename(&self.target, &backup).map_err(|error| {
                format!(
                    "move old snapshot {} to backup: {error}",
                    self.target.display()
                )
            })?;
            Some(backup)
        } else {
            None
        };

        let temporary = self
            .temporary
            .take()
            .expect("snapshot temporary directory must exist")
            .keep();
        if let Err(error) = fs::rename(&temporary, &self.target) {
            let restore = backup
                .as_ref()
                .map(|backup| fs::rename(backup, &self.target));
            let _ = fs::remove_dir_all(&temporary);
            return match restore {
                Some(Err(restore_error)) => Err(format!(
                    "commit snapshot {}: {error}; restore old snapshot: {restore_error}",
                    self.target.display()
                )),
                _ => Err(format!(
                    "commit snapshot {}: {error}",
                    self.target.display()
                )),
            };
        }

        if let Some(backup) = backup
            && let Err(error) = fs::remove_dir_all(&backup)
        {
            let failed_snapshot = temporary;
            if let Err(rollback_error) = fs::rename(&self.target, &failed_snapshot) {
                return Err(format!(
                    "remove old snapshot {}: {error}; begin rollback: {rollback_error}",
                    backup.display()
                ));
            }
            if let Err(rollback_error) = fs::rename(&backup, &self.target) {
                let restore_new = fs::rename(&failed_snapshot, &self.target);
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
}
