use anyhow::{bail, Result};
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;

use crate::toolchains::bencher::archive_branch;

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

impl ArchiveController {
    pub fn execute(&self) -> Result<()> {
        let branch = match &self.branch {
            Some(branch) => branch.clone(),
            None => {
                let branch = Command::new("git")
                    .args(["branch", "--show-current"])
                    .evaluate()?
                    .trim()
                    .to_owned();

                if branch.is_empty() || branch == "main" || branch == "master" {
                    bail!(
                        "Cannot archive the main branch. \
                         Switch to a feature branch or use --branch <name>."
                    );
                }

                branch
            }
        };

        CargoWorkspace::install_binary("bencher_cli")?;

        for project in BENCHER_PROJECTS {
            archive_branch(project, &branch);
        }

        Ok(())
    }
}
