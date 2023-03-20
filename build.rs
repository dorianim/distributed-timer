#[cfg(not(debug_assertions))]
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=web");
    build_web();
}

#[cfg(not(debug_assertions))]
fn build_web() {
    let status = Command::new("npm")
        .arg("install")
        .current_dir("web")
        .status()
        .expect("failed to install web dependencies");
    assert!(status.success());

    Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir("web")
        .status()
        .expect("failed to build web");
    assert!(status.success());
}

#[cfg(debug_assertions)]
fn build_web() {
    // skip in debug builds
}
