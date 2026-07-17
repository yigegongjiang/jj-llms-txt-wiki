use std::env;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{self, Command, Stdio};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

fn assert_installed(executable: &Path, action: &str) -> Result<(), String> {
    if executable.file_name() != Some(OsStr::new(NAME)) {
        return Err(format!("refusing to {action}: not the installed binary"));
    }
    Ok(())
}

fn asset_name() -> Result<String, String> {
    if env::consts::OS != "macos" {
        return Err(format!("unsupported OS: {}", env::consts::OS));
    }

    let architecture = match env::consts::ARCH {
        "aarch64" => "arm64",
        "x86_64" => "x64",
        architecture => return Err(format!("unsupported arch: {architecture}")),
    };
    Ok(format!("{NAME}-darwin-{architecture}"))
}

fn repository_slug() -> Result<&'static str, String> {
    REPOSITORY
        .strip_prefix("https://github.com/")
        .map(|slug| slug.trim_end_matches(".git"))
        .filter(|slug| slug.split('/').count() == 2)
        .ok_or_else(|| format!("unsupported repository URL: {REPOSITORY}"))
}

fn download(url: &str, destination: &Path, optional: bool) -> Result<(), String> {
    let mut command = Command::new("curl");
    command.args(["-fL", "--retry", "3", "--silent"]);

    if optional {
        command.stdout(Stdio::null()).stderr(Stdio::null());
    } else {
        command.arg("--show-error");
    }
    command.arg("--output").arg(destination).arg(url);

    let status = command
        .status()
        .map_err(|error| format!("failed to run curl: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("GET {url} failed with {status}"))
    }
}

fn expected_checksum(contents: &str, asset: &str) -> Option<String> {
    contents.lines().find_map(|line| {
        let mut fields = line.split_whitespace();
        let checksum = fields.next()?;
        let filename = fields.next()?.trim_start_matches('*');
        (filename == asset).then(|| checksum.to_ascii_lowercase())
    })
}

fn sha256(path: &Path) -> Result<String, String> {
    let output = Command::new("shasum")
        .args(["-a", "256"])
        .arg(path)
        .output()
        .map_err(|error| format!("failed to run shasum: {error}"))?;
    if !output.status.success() {
        return Err(format!("shasum failed with {}", output.status));
    }

    String::from_utf8(output.stdout)
        .map_err(|error| format!("invalid shasum output: {error}"))?
        .split_whitespace()
        .next()
        .map(str::to_ascii_lowercase)
        .ok_or_else(|| "empty shasum output".to_owned())
}

fn read_version(executable: &Path) -> Option<String> {
    let output = Command::new(executable).arg("--version").output().ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()?
        .split_whitespace()
        .next_back()
        .map(str::to_owned)
}

pub fn update() -> Result<(), String> {
    let executable = env::current_exe().map_err(|error| format!("current executable: {error}"))?;
    assert_installed(&executable, "self-update")?;

    let asset = asset_name()?;
    let base = format!(
        "https://github.com/{}/releases/latest/download",
        repository_slug()?
    );
    let parent = executable
        .parent()
        .ok_or_else(|| "current executable has no parent directory".to_owned())?;
    let temporary = parent.join(format!(".{NAME}.update.{}", process::id()));
    let checksum_file = parent.join(format!(".{NAME}.checksums.{}", process::id()));

    println!("==> Updating {NAME} {VERSION} -> latest");
    let result = (|| {
        download(&format!("{base}/{asset}"), &temporary, false)?;

        if download(&format!("{base}/checksums.txt"), &checksum_file, true).is_ok() {
            let contents = fs::read_to_string(&checksum_file)
                .map_err(|error| format!("read checksums.txt: {error}"))?;
            if let Some(expected) = expected_checksum(&contents, &asset) {
                let actual = sha256(&temporary)?;
                if expected != actual {
                    return Err("checksum mismatch".to_owned());
                }
            }
        }

        fs::set_permissions(&temporary, fs::Permissions::from_mode(0o755))
            .map_err(|error| format!("set executable permission: {error}"))?;
        fs::rename(&temporary, &executable)
            .map_err(|error| format!("replace executable: {error}"))?;
        Ok(())
    })();

    let _ = fs::remove_file(&temporary);
    let _ = fs::remove_file(&checksum_file);
    result?;

    let new_version = read_version(&executable).unwrap_or_else(|| "unknown".to_owned());
    println!("==> Updated {NAME} {VERSION} -> {new_version}");
    println!("    {}", executable.display());
    Ok(())
}

pub fn uninstall() -> Result<(), String> {
    let executable = env::current_exe().map_err(|error| format!("current executable: {error}"))?;
    assert_installed(&executable, "uninstall")?;
    fs::remove_file(&executable).map_err(|error| format!("remove executable: {error}"))?;
    println!("==> Removed: {}", executable.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{assert_installed, expected_checksum};
    use std::path::Path;

    #[test]
    fn finds_checksum_for_exact_asset() {
        let checksums = "aaa  jj-llms-txt-wiki-darwin-arm64\nbbb *jj-llms-txt-wiki-darwin-x64\n";
        assert_eq!(
            expected_checksum(checksums, "jj-llms-txt-wiki-darwin-x64"),
            Some("bbb".to_owned())
        );
    }

    #[test]
    fn ignores_other_assets() {
        assert_eq!(
            expected_checksum("aaa  jj-llms-txt-wiki-darwin-arm64\n", "other"),
            None
        );
    }

    #[test]
    fn refuses_non_installed_binary_names() {
        assert!(assert_installed(Path::new("/tmp/not-jj-llms-txt-wiki"), "uninstall").is_err());
    }
}
