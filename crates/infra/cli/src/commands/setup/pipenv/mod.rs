use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result};
use infra_utils::{
    commands::Command,
    github::GitHub,
    paths::{FileWalker, PathExtensions},
};
use serde::Deserialize;

pub fn setup_pipenv() -> Result<()> {
    // Install the 'pipenv' binary using the version in the top-level `Pipfile`.
    install_pipenv_binary()?;

    // Each Python project has its own Pipfile at the root of the project.
    // Find all of them, and run 'pipenv install' in each directory.
    for pip_file in FileWalker::from_repo_root().find(["**/Pipfile"])? {
        install_project_packages(&pip_file)?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct Pipfile {
    packages: HashMap<String, String>,
}

fn install_pipenv_binary() -> Result<()> {
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
        .run()
}

fn install_project_packages(pip_file: &Path) -> Result<()> {
    let project_directory = pip_file.unwrap_parent();

    let mut command = Command::new("python3")
        .property("-m", "pipenv")
        .arg("install")
        .current_dir(project_directory);

    if GitHub::is_running_in_ci() {
        command = command.flag("--deploy");
    }

    command.run()
}
