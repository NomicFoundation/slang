use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use serde::Deserialize;

pub struct PipEnv;

impl PipEnv {
    pub fn install_packages() -> Result<()> {
        let pip_file_toml = Path::repo_path("Pipfile").read_to_string()?;
        let pip_file: Pipfile = toml::from_str(&pip_file_toml)?;

        // This should be a value like "==YYYY.MM.DD"
        let version = pip_file
            .packages
            .get("pipenv")
            .context("Failed to find 'pipenv' in 'Pipfile' packages.")?;

        // pip3 install "pipenv==YYYY.MM.DD"
        Command::new("pip3")
            .arg("install")
            .arg(format!("pipenv{version}"))
            .run();

        let mut command = Command::new("python3")
            .property("-m", "pipenv")
            .arg("install");

        if GitHub::is_running_in_ci() {
            command = command.flag("--deploy");
        }

        command.run();

        Ok(())
    }

    #[must_use]
    pub fn run(name: impl Into<String>) -> Command {
        Command::new("python3")
            .property("-m", "pipenv")
            .args(["run", &name.into()])
    }
}

#[derive(Deserialize)]
struct Pipfile {
    packages: HashMap<String, String>,
}
