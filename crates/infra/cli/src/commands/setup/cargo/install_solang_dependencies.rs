use std::fs::{self};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

pub fn install_dependencies() -> Result<()> {
    // Solang requires a custom build of LLVM. We fetch it and untar it.

    let url = if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        "https://github.com/hyperledger/solang-llvm/releases/download/llvm15-2/llvm15.0-linux-x86-64.tar.xz"
    } else if cfg!(target_os = "linux") && cfg!(target_arch = "aarch64") {
        "https://github.com/hyperledger/solang-llvm/releases/download/llvm15-2/llvm15.0-linux-arm64.tar.xz"
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
        "https://github.com/hyperledger/solang-llvm/releases/download/llvm15-2/llvm15.0-mac-intel.tar.xz"
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        "https://github.com/hyperledger/solang-llvm/releases/download/llvm15-2/llvm15.0-mac-arm.tar.xz"
    } else {
        return Err(anyhow!("Unsupported OS or architecture"));
    };

    let bin_dir = Path::repo_path("bin/solang-llvm"); // _SLANG_INFRA_SOLANG_LLVM_PATH_ (keep in sync)
    let archive_path = std::env::temp_dir().join(format!(
        "llvm-{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
    ));
    let archive_path = archive_path.as_os_str().to_str().unwrap();

    if !std::path::Path::new(&bin_dir).exists() {
        println!("Downloading {url}");
        Command::new("curl")
            .args(["-L", "-o", archive_path, url])
            .run();

        fs::create_dir_all(&bin_dir)?;

        println!(
            "Extracting {archive_path} into {bin_dir}",
            bin_dir = bin_dir.display()
        );
        Command::new("tar")
            .args([
                "-xJf",
                archive_path,
                "-C",
                bin_dir.as_os_str().to_str().unwrap(),
                "--strip-components", // we want to splat the `llvm-XX.X` into the `llvm` dir but...
                "2", // HACK: the tar produced starts with `./`, meaning we must strip `./` too.
            ])
            .run();
    }

    Ok(())
}
