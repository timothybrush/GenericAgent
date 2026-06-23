use std::process::Command;

fn main() {
    // Build identity used to decide whether a bridge already holding :14168 belongs to THIS
    // build. commit hash + build timestamp → distinct on every build, even when the human
    // version in tauri.conf.json is unchanged (so same-version re-publishes still take over
    // a stale bridge). The bridge reports this back via GET /services/identity.
    let commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "nogit".to_string());
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    println!("cargo:rustc-env=GA_BUILD_ID={}-{}", commit, stamp);
    // Re-run when the checked-out commit changes so the id stays fresh.
    println!("cargo:rerun-if-changed=../../../.git/HEAD");

    tauri_build::build()
}
