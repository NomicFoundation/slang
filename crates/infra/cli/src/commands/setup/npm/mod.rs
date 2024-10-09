use std::path::Path;

use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;

pub fn setup_npm() {
    if GitHub::is_running_in_ci() {
        Command::new("npm").arg("ci").run();
    } else {
        Command::new("npm").arg("install").run();
    }

    Command::new("npm")
        .current_dir(Path::repo_path("submodules/jco"))
        .args(["install"])
        .run();

    Command::new("npm")
        .current_dir(Path::repo_path("submodules/jco"))
        .args(["run", "build"])
        .run();

    Command::new("npm")
        .current_dir(Path::repo_path("submodules/jco"))
        .args(["run", "build:release"])
        .run();
}
