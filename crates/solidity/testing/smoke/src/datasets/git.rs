use std::{
    path::PathBuf,
    time::{Duration, SystemTime},
};

use anyhow::{anyhow, Result};
use codegen_utils::commands::{run_command, run_command_inline};
use url::Url;
use walkdir::WalkDir;

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
        let dataset_dir = PathBuf::from(std::env::var("REPO_ROOT")?)
            .join("target/solidity/testing/smoke/git-datasets")
            .join(self.name);

        if dataset_dir.exists() {
            Self::update(&dataset_dir)?;
        } else {
            Self::clone(&self.git_url, &dataset_dir)?;
        }

        let source_files = Self::collect_files(&dataset_dir)?;

        return Ok(source_files);
    }
}

impl GitDataset {
    const MAX_REPO_STALENESS: Duration = Duration::from_secs(60 * 60 * 24); // one day

    pub fn new(name: &'static str, url: Url) -> Self {
        return Self { name, git_url: url };
    }

    fn clone(git_url: &Url, dataset_dir: &PathBuf) -> Result<()> {
        let parent_dir = dataset_dir.parent().unwrap().to_owned();

        std::fs::create_dir_all(&parent_dir)?;

        run_command_inline(
            &parent_dir,
            &[
                "git",
                "clone",
                "--depth",
                "1",
                git_url.as_str(),
                dataset_dir.to_str().unwrap(),
            ],
        )?;

        return Ok(());
    }

    fn update(dataset_dir: &PathBuf) -> Result<()> {
        let current_commit_age = {
            let since_epoch = run_command(
                &dataset_dir,
                &["git", "log", "--no-walk", "--format=%ct", "HEAD"],
                None,
            )?;

            let since_epoch = Duration::from_secs(since_epoch.trim().parse::<u64>()?);
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH + since_epoch)?
        };

        if current_commit_age < Self::MAX_REPO_STALENESS {
            return Ok(());
        }

        run_command_inline(
            &dataset_dir,
            &["git", "fetch", "origin", "HEAD", "--depth", "1"],
        )?;

        run_command_inline(&dataset_dir, &["git", "checkout", "origin/HEAD"])?;

        return Ok(());
    }

    fn collect_files(dataset_dir: &PathBuf) -> Result<Vec<PathBuf>> {
        return WalkDir::new(dataset_dir)
            .into_iter()
            .filter_map(|entry| match entry {
                Err(error) => Some(Err(anyhow!(error))),
                Ok(entry) => {
                    if entry.file_type().is_dir() {
                        return None;
                    }

                    let file_path = entry.path().to_path_buf();
                    if file_path.extension()? != "sol" {
                        return None;
                    }

                    Some(Ok(file_path))
                }
            })
            .collect();
    }
}
