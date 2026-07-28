use anyhow::{Result, bail};
use clap::Parser;
use infra_utils::commands::Command;

use crate::commands::perf::binaries;
use crate::toolchains::bencher::{BencherProject, archive_branch, unarchive_branch};

#[derive(Clone, Debug, Parser)]
pub struct ArchiveController {
    /// Bencher project to archive.
    #[arg(long)]
    project: BencherProject,

    /// Branch name to archive. If omitted, uses the current git branch.
    #[arg(long)]
    branch: Option<String>,
}

#[derive(Clone, Debug, Parser)]
pub struct UnarchiveController {
    /// Bencher project to unarchive.
    #[arg(long)]
    project: BencherProject,

    /// Branch name to unarchive. If omitted, uses the current git branch.
    #[arg(long)]
    branch: Option<String>,
}

fn resolve_branch(branch: Option<&str>) -> Result<String> {
    let branch = match branch {
        Some(branch) => branch.trim_matches('"').trim().to_owned(),
        None => Command::new("git")
            .args(["branch", "--show-current"])
            .evaluate()?
            .trim()
            .to_owned(),
    };

    if branch.is_empty() || branch == "main" || branch == "master" {
        bail!(
            "Cannot archive/unarchive the main branch. \
             Switch to a feature branch or use --branch <name>."
        );
    }

    Ok(branch)
}

impl ArchiveController {
    pub fn execute(&self) -> Result<()> {
        let branch = resolve_branch(self.branch.as_deref())?;
        binaries::install_bencher_cli()?;
        archive_branch(self.project, &branch);
        Ok(())
    }
}

impl UnarchiveController {
    pub fn execute(&self) -> Result<()> {
        let branch = resolve_branch(self.branch.as_deref())?;
        binaries::install_bencher_cli()?;
        unarchive_branch(self.project, &branch);
        Ok(())
    }
}
