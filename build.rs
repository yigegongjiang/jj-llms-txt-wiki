use std::process::Command;

/// Bake the source checkout's `origin` URL into the binary as the snapshot
/// mirror target. A fork is then correct with zero configuration: whoever builds
/// it — locally or in their own CI — mirrors to the repository they cloned from,
/// never to the upstream one. Falls back to `[package].repository` when there is
/// no checkout to ask (source tarball, vendored build).
fn main() {
    // `origin` lives in the config, so that is what invalidates the value.
    println!("cargo:rerun-if-changed=.git/config");
    let Some(origin) = git_origin() else { return };
    println!("cargo:rustc-env=JJ_LLMS_TXT_WIKI_ORIGIN={origin}");
}

fn git_origin() -> Option<String> {
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let origin = String::from_utf8(output.stdout).ok()?.trim().to_string();
    // A newline would truncate the `cargo:` directive and silently swallow the
    // rest of it; a URL can never contain one, so reject rather than sanitize.
    (!origin.is_empty() && !origin.contains('\n')).then_some(origin)
}
