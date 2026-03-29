use anyhow::{bail, Result};
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;

use crate::toolchains::bencher;

const BENCHER_PROJECTS: &[&str] = &[
    "slang-dashboard-cargo-slang",
    "slang-dashboard-cargo-cmp",
    "slang-dashboard-npm",
];

#[derive(Clone, Debug, Parser)]
pub struct ArchiveController {
    /// Branch name to archive. If omitted, uses the current git branch.
    #[arg(long)]
    branch: Option<String>,
}

#[derive(Clone, Debug, Parser)]
pub struct UnarchiveController {
    /// Branch name to unarchive. If omitted, uses the current git branch.
    #[arg(long)]
    branch: Option<String>,
}

fn resolve_branch(branch: &Option<String>) -> Result<String> {
    match branch {
        Some(branch) => Ok(branch.clone()),
        None => {
            let branch = Command::new("git")
                .args(["branch", "--show-current"])
                .evaluate()?
                .trim()
                .to_owned();

            if branch.is_empty() || branch == "main" || branch == "master" {
                bail!(
                    "Cannot archive/unarchive the main branch. \
                     Switch to a feature branch or use --branch <name>."
                );
            }

            Ok(branch)
        }
    }
}

impl ArchiveController {
    pub fn execute(&self) -> Result<()> {
        let branch = resolve_branch(&self.branch)?;
        CargoWorkspace::install_binary("bencher_cli")?;

        for project in BENCHER_PROJECTS {
            bencher::archive_branch(project, &branch);
        }

        Ok(())
    }
}

impl UnarchiveController {
    pub fn execute(&self) -> Result<()> {
        let branch = resolve_branch(&self.branch)?;
        CargoWorkspace::install_binary("bencher_cli")?;

        for project in BENCHER_PROJECTS {
            bencher::unarchive_branch(project, &branch);
        }

        Ok(())
    }
}
