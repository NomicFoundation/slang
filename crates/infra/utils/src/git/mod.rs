use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use itertools::Itertools;
use regex::Regex;

use crate::commands::Command;
use crate::paths::PathExtensions;

pub struct TemporaryChangeset {
    base_branch: String,
    head_branch: String,
    message: String,
    expected_changes: Vec<PathBuf>,
}

impl TemporaryChangeset {
    pub fn new(branch_name: impl Into<String>, message: impl Into<String>) -> Result<Self> {
        let local_changes = get_local_changes()?;
        if !local_changes.is_empty() {
            bail!("Found existing local changes:\n{local_changes:?}");
        }

        let base_branch = get_current_branch()?;
        let head_branch = branch_name.into();
        assert_ne!(base_branch, head_branch);

        Ok(Self {
            base_branch,
            head_branch,
            message: message.into(),
            expected_changes: vec![],
        })
    }

    pub fn expect_change(&mut self, path: impl Into<PathBuf>) {
        self.expected_changes.push(path.into());
    }

    pub fn commit_changes(&self) -> Result<()> {
        let local_changes = get_local_changes()?;
        assert_eq!(
            local_changes.iter().sorted().collect_vec(),
            self.expected_changes.iter().sorted().collect_vec(),
        );

        if self.expected_changes.is_empty() {
            println!("No changes to commit, no need to create a branch.");
            return Ok(());
        }

        Command::new("git")
            .arg("checkout")
            .property("-b", &self.head_branch)
            .run();

        Command::new("git")
            .arg("add")
            .args(local_changes.iter().map(|path| path.unwrap_str()))
            .run();

        Command::new("git").arg("diff").flag("--cached").run();

        Command::new("git")
            .arg("commit")
            .property("--message", &self.message)
            .run();

        Ok(())
    }

    pub fn revert_changes(&self) -> Result<()> {
        if self.expected_changes.is_empty() {
            return Ok(());
        }

        let current_branch = get_current_branch()?;
        assert_eq!(current_branch, self.head_branch);

        Command::new("git")
            .arg("checkout")
            .arg(&self.base_branch)
            .run();

        Command::new("git")
            .arg("branch")
            .property("-D", &self.head_branch)
            .run();

        Ok(())
    }
}

fn get_local_changes() -> Result<Vec<PathBuf>> {
    // Command output example:
    //
    // D foo.txt
    // AM bar.txt
    //  M baz.txt
    let pattern = Regex::new(r"[ ]*[ADM]+[ ]+(.+)")?;

    Ok(Command::new("git")
        .arg("status")
        .flag("--short")
        .evaluate()?
        .trim()
        .lines()
        .map(|line| Path::repo_path(&pattern.captures(line).unwrap()[1]))
        .collect_vec())
}

fn get_current_branch() -> Result<String> {
    Ok(Command::new("git")
        .arg("branch")
        .flag("--show-current")
        .evaluate()?
        .trim()
        .to_owned())
}
