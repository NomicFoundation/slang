use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use anyhow::Result;
use infra_utils::{
    cargo::CargoWorkspace,
    commands::Command,
    paths::{FileWalker, PathExtensions},
};
use url::Url;

use crate::datasets::Dataset;

pub struct GitDataset {
    name: &'static str,
    git_url: Url,
}

impl Dataset for GitDataset {
    fn get_title(&self) -> &'static str {
        return self.name;
    }

    fn prepare(&self) -> Result<Vec<PathBuf>> {
        let dataset_dir = CargoWorkspace::locate_source_crate("solidity_testing_smoke")?
            .join("target/git-datasets")
            .join(self.name);

        if dataset_dir.exists() {
            Self::update(&dataset_dir)?;
        } else {
            Self::clone(&self.git_url, &dataset_dir)?;
        }

        let source_files = FileWalker::from_directory(dataset_dir)
            .find(["**/*.sol"])?
            .into_iter()
            .collect();

        return Ok(source_files);
    }
}

impl GitDataset {
    const MAX_REPO_STALENESS: Duration = Duration::from_secs(60 * 60 * 24); // one day

    pub fn new(name: &'static str, url: Url) -> Self {
        return Self { name, git_url: url };
    }

    fn clone(git_url: &Url, dataset_dir: &Path) -> Result<()> {
        std::fs::create_dir_all(dataset_dir.unwrap_parent())?;

        return Command::new("git")
            .args(["clone", git_url.as_str(), dataset_dir.unwrap_str()])
            .property("--depth", "1")
            .run();
    }

    fn update(dataset_dir: &Path) -> Result<()> {
        let since_epoch = Command::new("git")
            .args(["log", "HEAD"])
            .flag("--no-walk")
            .flag("--format=%ct")
            .current_dir(dataset_dir)
            .evaluate()?
            .trim()
            .parse::<u64>()?;

        let last_change_timestamp = SystemTime::UNIX_EPOCH + Duration::from_secs(since_epoch);
        let current_commit_age = SystemTime::now().duration_since(last_change_timestamp)?;

        if current_commit_age < Self::MAX_REPO_STALENESS {
            return Ok(());
        }

        Command::new("git")
            .args(["fetch", "origin", "HEAD"])
            .property("--depth", "1")
            .current_dir(dataset_dir)
            .run()?;

        Command::new("git")
            .args(["checkout", "origin/HEAD"])
            .current_dir(dataset_dir)
            .run()?;

        return Ok(());
    }
}
