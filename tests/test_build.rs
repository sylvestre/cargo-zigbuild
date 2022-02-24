use anyhow::Result;
use cargo_zigbuild::Build;
use once_cell::sync::Lazy;
use std::process::{Command, Stdio};
use std::sync::Mutex;

static RUSTUP_LOCK: Lazy<Mutex<()>> = Lazy::new(Mutex::default);

fn run_build(manifest_path: &str, target: &str) -> Result<()> {
    let rust_target = target.split_once('.').map(|(t, _)| t).unwrap_or(target);
    {
        let _guard = RUSTUP_LOCK.lock().unwrap();
        // Install Rust target
        let status = Command::new("rustup")
            .arg("target")
            .arg("add")
            .arg(rust_target)
            .stderr(Stdio::null())
            .status()
            .expect("Failed to execute rustup process");
        assert!(status.success());

        // Set env var so that the linker scripts refer to the cargo-zigbuild binary
        std::env::set_var(
            "CARGO_BIN_EXE_cargo-zigbuild",
            env!("CARGO_BIN_EXE_cargo-zigbuild"),
        );
    }

    let mut build = Build::default();
    build.target = Some(target.to_string());
    build.manifest_path = Some(manifest_path.into());
    build.quiet = true;
    build.execute()?;
    Ok(())
}

#[test]
fn test_linux_glibc_x86_64() {
    run_build("Cargo.toml", "x86_64-unknown-linux-gnu").unwrap();
}

#[test]
fn test_linux_glibc_2_17_x86_64() {
    // only run on nightly
    let meta = rustc_version::version_meta().unwrap();
    if !matches!(meta.channel, rustc_version::Channel::Nightly) {
        return;
    }
    run_build("Cargo.toml", "x86_64-unknown-linux-gnu.2.17").unwrap();
}

#[test]
fn test_linux_glibc_aarch64() {
    run_build("Cargo.toml", "aarch64-unknown-linux-gnu").unwrap();
}

#[test]
fn test_linux_glibc_aarch64_cdylib() {
    run_build("tests/libhello/Cargo.toml", "aarch64-unknown-linux-gnu").unwrap();
}

#[test]
fn test_linux_glibc_2_17_aarch64() {
    run_build("Cargo.toml", "aarch64-unknown-linux-gnu.2.17").unwrap();
}

#[test]
fn test_linux_glibc_arm() {
    run_build("Cargo.toml", "arm-unknown-linux-gnueabihf").unwrap();
}

#[test]
fn test_linux_musl_aarch64() {
    run_build("Cargo.toml", "aarch64-unknown-linux-musl").unwrap();
}

#[test]
fn test_macos_aarch64() {
    run_build("Cargo.toml", "aarch64-apple-darwin").unwrap();
}

#[test]
fn test_macos_aarch64_cdylib() {
    run_build("tests/libhello/Cargo.toml", "aarch64-apple-darwin").unwrap();
}

#[test]
fn test_windows_x84_64_gnu() {
    run_build("Cargo.toml", "x86_64-pc-windows-gnu").unwrap();
}

#[test]
fn test_windows_x84_64_gnu_winapi_windows_rs() {
    run_build("tests/hello-windows/Cargo.toml", "x86_64-pc-windows-gnu").unwrap();
}
