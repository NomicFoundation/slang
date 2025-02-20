use std::path::Path;

use anyhow::Result;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;

pub fn setup_npm() -> Result<()> {
    if GitHub::is_running_in_ci() {
        Command::new("npm").arg("ci").run();
    } else {
        Command::new("npm").arg("install").run();
    }

    Command::new("npm")
        .current_dir(Path::repo_path("submodules/jco"))
        .args(["install"])
        .run();

    // jco needs to be built in debug mode once before it can be built in release mode,
    // since it requires some build artifacts in order to perform an optimization step.
    Command::new("npm")
        .current_dir(Path::repo_path("submodules/jco"))
        .args(["run", "build"])
        .run();

    Command::new("npm")
        .current_dir(Path::repo_path("submodules/jco"))
        .args(["run", "build:release"])
        .run();

    if GitHub::is_running_in_ci() {
        // Remove debug artifacts in CI to reduce total build size
        Command::new("cargo")
            .arg("clean")
            .args(["--target-dir", "submodules/jco/target"])
            .args(["--profile", "dev"])
            .run();

        std::fs::remove_dir_all("submodules/jco/target/wasm32-wasip1/debug")?;
    }

    Ok(())
}
