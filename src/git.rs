use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;

use fs2::FileExt;
use jiff::Timestamp;

const FALLBACK_NAME: &str = "llms-wiki";
const FALLBACK_EMAIL: &str = "llms-wiki@localhost";
const LOCK_FILE: &str = ".git/llms-wiki.commit.lock";
const RETRY_BACKOFF_MS: &[u64] = &[200, 500, 1000, 2000, 4000, 8000];
/// Remote branch that mirrors the data repo's local HEAD. Kept distinct from
/// the code repo's `main` so both can share a single remote without collision.
const PUSH_BRANCH: &str = "wiki-data";

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

    /// Commit a single site's subtree. Serialized across processes by an OS file
    /// lock so concurrent `llms-wiki sync <site>` invocations never contend on
    /// git's `.git/index.lock`. Retries with exponential backoff if any external
    /// git tool briefly holds the index lock. `.gitignore` is staged alongside
    /// the site so the repo's initial `.gitignore` lands with the first site
    /// commit rather than requiring a separate bootstrap step.
    pub fn record_site(&self, site: &str) -> Result<(), String> {
        let _guard = self.acquire_commit_lock()?;
        let mut paths: Vec<&str> = vec![site];
        if self.root.join(".gitignore").exists() {
            paths.push(".gitignore");
        }
        let mut args: Vec<&str> = vec!["add", "--"];
        args.extend(paths);
        self.run_with_retry(&args)?;
        let timestamp = Timestamp::now().strftime("%Y-%m-%dT%H:%M:%SZ");
        let message = format!("chore(sync): {site} @ {timestamp}");
        self.commit(&message)
    }

    /// Best-effort mirror of the local snapshot HEAD to a distinct branch on
    /// `url`. Non-interactive: HTTPS without credentials fails immediately
    /// (`GIT_TERMINAL_PROMPT=0`), SSH declines password / host-key prompts
    /// (`BatchMode=yes`), and slow-or-stalled HTTPS transfers bail out via
    /// git's low-speed thresholds. Callers surface any Err as informational —
    /// contributors and end users without push access should never see the
    /// sync fail because of this. No `--force`: divergence from the remote
    /// (recreated data repo, force-reset) intentionally fails so nothing is
    /// silently overwritten upstream.
    pub fn push_snapshot(&self, url: &str) -> Result<(), String> {
        let refspec = format!("HEAD:refs/heads/{PUSH_BRANCH}");
        let mut command = self.command();
        command
            .env("GIT_TERMINAL_PROMPT", "0")
            .env(
                "GIT_SSH_COMMAND",
                "ssh -o BatchMode=yes -o ConnectTimeout=10",
            )
            .args([
                "-c",
                "http.lowSpeedLimit=1000",
                "-c",
                "http.lowSpeedTime=10",
                "push",
                "--quiet",
                url,
                &refspec,
            ]);
        let action = format!("git push {url} {refspec}");
        self.finish(&action, command.output()).map(drop)
    }

    fn acquire_commit_lock(&self) -> Result<CommitLock, String> {
        let path = self.root.join(LOCK_FILE);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("create lock dir {}: {error}", parent.display()))?;
        }
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .map_err(|error| format!("open lock {}: {error}", path.display()))?;
        file.lock_exclusive()
            .map_err(|error| format!("lock {}: {error}", path.display()))?;
        Ok(CommitLock { file })
    }

    fn commit(&self, message: &str) -> Result<(), String> {
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
            message,
        ]);
        self.finish_with_retry("commit sync snapshot", || command_output(&mut command))
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

    fn run_with_retry(&self, args: &[&str]) -> Result<(), String> {
        let label = format!("git {}", args.join(" "));
        self.finish_with_retry(&label, || self.command().args(args).output())
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

    /// Runs a git command; if it fails with an index/ref lock-contention error
    /// (external GUI tool briefly holding `.git/index.lock`), retries with
    /// exponential backoff up to ~15 s. Non-lock errors return immediately.
    fn finish_with_retry<F>(&self, action: &str, mut run: F) -> Result<Output, String>
    where
        F: FnMut() -> std::io::Result<Output>,
    {
        let mut last = self.finish(action, run());
        for &delay_ms in RETRY_BACKOFF_MS {
            match &last {
                Ok(_) => return last,
                Err(error) if is_lock_contention(error) => {
                    thread::sleep(Duration::from_millis(delay_ms));
                    last = self.finish(action, run());
                }
                Err(_) => return last,
            }
        }
        last
    }
}

fn command_output(command: &mut Command) -> std::io::Result<Output> {
    command.output()
}

fn is_lock_contention(error: &str) -> bool {
    let lower = error.to_ascii_lowercase();
    lower.contains("unable to create") && lower.contains(".lock") || lower.contains("index.lock")
}

struct CommitLock {
    file: File,
}

impl Drop for CommitLock {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.file);
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
    fn creates_nested_repository_and_commits_per_site() {
        let parent = tempdir().unwrap();
        git(parent.path(), &["init", "--quiet"]);
        let root = parent.path().join("wiki");
        let repository = Repository::prepare(&root).unwrap();
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(root.join("docs/a.md"), "v1").unwrap();

        repository.record_site("docs").unwrap();
        repository.record_site("docs").unwrap();

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
    fn per_site_add_ignores_other_sites_working_state() {
        let parent = tempdir().unwrap();
        git(parent.path(), &["init", "--quiet"]);
        let root = parent.path().join("wiki");
        let repository = Repository::prepare(&root).unwrap();
        fs::create_dir_all(root.join("a")).unwrap();
        fs::create_dir_all(root.join("b")).unwrap();
        fs::write(root.join("a/a.md"), "a").unwrap();
        fs::write(root.join("b/b.md"), "b").unwrap();

        // Committing site `a` must not include site `b`, even if `b` has changes on disk.
        repository.record_site("a").unwrap();
        let files = git(&root, &["show", "--name-only", "--pretty=", "HEAD"]);
        assert!(files.contains("a/a.md"), "HEAD missing a/a.md: {files}");
        assert!(
            !files.contains("b/"),
            "HEAD unexpectedly touched b/: {files}"
        );
        assert_eq!(git(&root, &["status", "--porcelain"]), "?? b/");
    }

    #[test]
    fn recovers_stale_index_lock_via_retry() {
        let parent = tempdir().unwrap();
        git(parent.path(), &["init", "--quiet"]);
        let root = parent.path().join("wiki");
        let repository = Repository::prepare(&root).unwrap();
        fs::create_dir_all(root.join("s")).unwrap();
        fs::write(root.join("s/x.md"), "x").unwrap();

        let lock_path = root.join(".git/index.lock");
        fs::write(&lock_path, "").unwrap();
        let clear = std::thread::spawn({
            let lock_path = lock_path.clone();
            move || {
                std::thread::sleep(std::time::Duration::from_millis(400));
                let _ = fs::remove_file(&lock_path);
            }
        });

        repository.record_site("s").unwrap();
        clear.join().unwrap();

        assert_eq!(git(&root, &["rev-list", "--count", "HEAD"]), "1");
        assert!(git(&root, &["status", "--porcelain"]).is_empty());
    }

    #[test]
    fn push_snapshot_pushes_local_head_to_wiki_data_branch() {
        let parent = tempdir().unwrap();
        let bare = parent.path().join("remote.git");
        Command::new("git")
            .args(["init", "--quiet", "--bare"])
            .arg(&bare)
            .status()
            .unwrap();
        let root = parent.path().join("wiki");
        let repository = Repository::prepare(&root).unwrap();
        fs::create_dir_all(root.join("s")).unwrap();
        fs::write(root.join("s/a.md"), "a").unwrap();
        repository.record_site("s").unwrap();

        let url = format!("file://{}", bare.display());
        repository.push_snapshot(&url).unwrap();
        assert_eq!(
            git(&bare, &["rev-parse", "refs/heads/wiki-data"]),
            git(&root, &["rev-parse", "HEAD"]),
            "remote wiki-data branch must match local HEAD"
        );

        // A second push after a further commit fast-forwards the mirror.
        fs::write(root.join("s/b.md"), "b").unwrap();
        repository.record_site("s").unwrap();
        repository.push_snapshot(&url).unwrap();
        assert_eq!(
            git(&bare, &["rev-parse", "refs/heads/wiki-data"]),
            git(&root, &["rev-parse", "HEAD"])
        );

        // Repeated push with no new commits is a no-op and stays clean.
        repository.push_snapshot(&url).unwrap();
    }

    #[test]
    fn push_snapshot_reports_error_for_unknown_remote() {
        let parent = tempdir().unwrap();
        let root = parent.path().join("wiki");
        let repository = Repository::prepare(&root).unwrap();
        fs::create_dir_all(root.join("s")).unwrap();
        fs::write(root.join("s/a.md"), "a").unwrap();
        repository.record_site("s").unwrap();

        // Local path that isn't a git repository: git push fails immediately.
        let missing = parent.path().join("missing.git");
        let url = format!("file://{}", missing.display());
        let error = repository.push_snapshot(&url).unwrap_err();
        assert!(
            error.contains("push"),
            "error must identify the failing action: {error}"
        );
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

        // A leftover partial is ignored, so the site commit stays clean.
        fs::create_dir_all(root.join(".docs.sync.abc")).unwrap();
        fs::write(root.join(".docs.sync.abc/x.md"), "x").unwrap();
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(root.join("docs/d.md"), "d").unwrap();
        let repository = Repository::prepare(&root).unwrap();
        repository.record_site("docs").unwrap();
        assert!(git(&root, &["status", "--porcelain"]).is_empty());
    }
}
