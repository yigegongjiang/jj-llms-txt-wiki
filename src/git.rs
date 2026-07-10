use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use jiff::Timestamp;

const FALLBACK_NAME: &str = "llms-wiki";
const FALLBACK_EMAIL: &str = "llms-wiki@localhost";

pub struct Repository {
    root: PathBuf,
}

impl Repository {
    pub fn prepare(root: &Path) -> Result<Self, String> {
        fs::create_dir_all(root)
            .map_err(|error| format!("create output directory {}: {error}", root.display()))?;
        if !root.is_dir() {
            return Err(format!(
                "output path is not a directory: {}",
                root.display()
            ));
        }

        let repository = Self {
            root: root.to_path_buf(),
        };
        if !root.join(".git").exists() {
            repository.run(&["init", "--quiet", "--initial-branch=main", "."])?;
        }
        repository.verify_root()?;
        ensure_gitignore(root)?;
        Ok(repository)
    }

    pub fn record_sync(&self, sites: &[String]) -> Result<(), String> {
        if sites.is_empty() {
            return Ok(());
        }

        self.run(&["add", "--all", "--", "."])?;
        let timestamp = Timestamp::now().strftime("%Y-%m-%dT%H:%M:%SZ");
        let message = format!("chore(sync): {} @ {timestamp}", sites.join(","));
        let mut command = self.command();
        if !self.has_config("user.name")? {
            command.args(["-c", &format!("user.name={FALLBACK_NAME}")]);
        }
        if !self.has_config("user.email")? {
            command.args(["-c", &format!("user.email={FALLBACK_EMAIL}")]);
        }
        command.args([
            "commit",
            "--quiet",
            "--allow-empty",
            "--no-gpg-sign",
            "-m",
            &message,
        ]);
        self.finish("commit sync snapshot", command.output())
            .map(drop)
    }

    fn verify_root(&self) -> Result<(), String> {
        let output = self.output(&["rev-parse", "--show-toplevel"])?;
        let actual = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
        let actual = fs::canonicalize(&actual)
            .map_err(|error| format!("resolve Git root {}: {error}", actual.display()))?;
        let expected = fs::canonicalize(&self.root).map_err(|error| {
            format!("resolve output directory {}: {error}", self.root.display())
        })?;
        if actual != expected {
            return Err(format!(
                "Git root {} does not match output directory {}",
                actual.display(),
                expected.display()
            ));
        }
        Ok(())
    }

    fn has_config(&self, key: &str) -> Result<bool, String> {
        let output = self
            .command()
            .args(["config", "--get", key])
            .output()
            .map_err(|error| format!("run git config in {}: {error}", self.root.display()))?;
        match output.status.code() {
            Some(0) => Ok(!String::from_utf8_lossy(&output.stdout).trim().is_empty()),
            Some(1) => Ok(false),
            _ => self
                .finish(&format!("read Git config {key}"), Ok(output))
                .map(|_| false),
        }
    }

    fn run(&self, args: &[&str]) -> Result<(), String> {
        let output = self.command().args(args).output();
        self.finish(&format!("git {}", args.join(" ")), output)
            .map(drop)
    }

    fn output(&self, args: &[&str]) -> Result<Output, String> {
        let output = self.command().args(args).output();
        self.finish(&format!("git {}", args.join(" ")), output)
    }

    fn command(&self) -> Command {
        let mut command = Command::new("git");
        command.current_dir(&self.root);
        command
    }

    fn finish(&self, action: &str, output: std::io::Result<Output>) -> Result<Output, String> {
        let output =
            output.map_err(|error| format!("{action} in {}: {error}", self.root.display()))?;
        if output.status.success() {
            return Ok(output);
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let detail = if stderr.trim().is_empty() {
            stdout.trim()
        } else {
            stderr.trim()
        };
        Err(format!("{action} in {}: {detail}", self.root.display()))
    }
}

/// Ensure the data repository ignores snapshot staging directories, so adopted or
/// leftover `.{site}.sync.*` / `.{site}.backup.*` partials are never committed into
/// the versioned content. Idempotent: appends only the patterns not already present.
fn ensure_gitignore(root: &Path) -> Result<(), String> {
    let path = root.join(".gitignore");
    let existing = fs::read_to_string(&path).unwrap_or_default();
    let mut content = existing.clone();
    let mut changed = false;
    for pattern in [".*.sync.*", ".*.backup.*"] {
        if existing.lines().any(|line| line.trim() == pattern) {
            continue;
        }
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(pattern);
        content.push('\n');
        changed = true;
    }
    if changed {
        fs::write(&path, &content).map_err(|error| format!("write {}: {error}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Repository;
    use std::fs;
    use std::process::Command;
    use tempfile::tempdir;

    fn git(root: &std::path::Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .current_dir(root)
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }

    #[test]
    fn creates_nested_repository_and_records_unchanged_syncs() {
        let parent = tempdir().unwrap();
        git(parent.path(), &["init", "--quiet"]);
        let root = parent.path().join("wiki");
        let repository = Repository::prepare(&root).unwrap();
        fs::write(root.join("docs.md"), "v1").unwrap();

        repository.record_sync(&["docs".to_owned()]).unwrap();
        repository.record_sync(&["docs".to_owned()]).unwrap();

        assert_eq!(git(&root, &["rev-list", "--count", "HEAD"]), "2");
        assert_eq!(
            fs::canonicalize(git(&root, &["rev-parse", "--show-toplevel"])).unwrap(),
            fs::canonicalize(&root).unwrap()
        );
        let subject = git(&root, &["log", "-1", "--format=%s"]);
        assert!(subject.starts_with("chore(sync): docs @ "));
        assert!(subject.ends_with('Z'));
        assert!(git(&root, &["status", "--porcelain"]).is_empty());
    }

    #[test]
    fn writes_gitignore_for_snapshot_dirs_idempotently() {
        let parent = tempdir().unwrap();
        git(parent.path(), &["init", "--quiet"]);
        let root = parent.path().join("wiki");

        Repository::prepare(&root).unwrap();
        let ignore = fs::read_to_string(root.join(".gitignore")).unwrap();
        assert!(ignore.lines().any(|line| line == ".*.sync.*"));
        assert!(ignore.lines().any(|line| line == ".*.backup.*"));

        Repository::prepare(&root).unwrap();
        assert_eq!(
            fs::read_to_string(root.join(".gitignore")).unwrap(),
            ignore,
            "re-preparing must not duplicate ignore patterns"
        );

        // A leftover partial is ignored, so the work tree stays clean.
        fs::create_dir_all(root.join(".docs.sync.abc")).unwrap();
        fs::write(root.join(".docs.sync.abc/x.md"), "x").unwrap();
        let repository = Repository::prepare(&root).unwrap();
        repository.record_sync(&["docs".to_owned()]).unwrap();
        assert!(git(&root, &["status", "--porcelain"]).is_empty());
    }
}
