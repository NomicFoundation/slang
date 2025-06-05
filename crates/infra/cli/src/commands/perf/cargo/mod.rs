use std::fs::{self, File};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;

use crate::commands::perf::bencher::run_bench;
use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct CargoController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl CargoController {
    pub fn execute(&self) -> Result<()> {
        Self::install_llvm_for_solang()?;
        Self::install_valgrind();
        CargoWorkspace::install_binary("iai-callgrind-runner")?;
        CargoWorkspace::install_binary("bencher_cli")?;

        // Bencher supports multiple languages/frameworks: https://bencher.dev/docs/explanation/adapters/
        // We currently only have one benchmark suite (Rust/iai), but we can add more here in the future.
        self.run_iai_bench("solidity_testing_perf_cargo", "iai");

        Ok(())
    }

    fn install_valgrind() {
        if GitHub::is_running_in_ci() {
            Command::new("sudo").args(["apt", "update"]).run();

            Command::new("sudo")
                .args(["apt", "install", "valgrind"])
                .flag("--yes")
                .run();

            return;
        }

        if GitHub::is_running_in_devcontainer() {
            Command::new("sudo").args(["apt-get", "update"]).run();

            Command::new("sudo")
                .args(["apt-get", "install", "valgrind"])
                .flag("--yes")
                .run();

            return;
        }

        match Command::new("valgrind").flag("--version").evaluate() {
            Ok(output) if output.starts_with("valgrind-") => {
                // Valgrind is available
            }
            other => {
                panic!(
                    "valgrind needs to be installed on this machine to run perf tests.
                    Supported Platforms: https://valgrind.org/downloads/current.html
                    {other:?}"
                );
            }
        }
    }

    fn run_iai_bench(&self, package_name: &str, bench_name: &str) {
        let test_runner = format!("cargo bench --package {package_name} --bench {bench_name}");

        run_bench(self.dry_run.get(), "rust_iai_callgrind", &test_runner);

        let reports_dir = Path::repo_path("target/iai")
            .join(package_name)
            .join(bench_name);

        println!("

Bencher Run is complete...
Test Results: [https://bencher.dev/console/projects/slang/reports]

Reports/Logs: {reports_dir}
- Callgrind flamegraphs (callgrind.*.svg) can be viewed directly in the browser.
- DHAT traces (dhat.*.out) can be viewed using the [dhat/dh_view.html] tool from the Valgrind release [https://valgrind.org/downloads/].

", reports_dir = reports_dir.display());
    }

    fn install_llvm_for_solang() -> Result<()> {
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

            println!("Extracting {archive_path} into {bin_dir:?}");
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
}
