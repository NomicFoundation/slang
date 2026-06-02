use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    /// Path to the prebuilt tarball produced by `publish prepare`.
    #[arg(long)]
    tarball: PathBuf,

    #[command(flatten)]
    dry_run: DryRun,
}

impl NpmController {
    pub fn execute(&self) -> Result<()> {
        let mut command = Command::new("pnpm")
            .arg("publish")
            .arg(self.tarball.unwrap_str())
            .property("--access", "public")
            // CI checkout is detached HEAD; pnpm's branch check would reject it.
            // The workflow's `branches:` filter and the slang-release environment
            // gate are what actually authorize a release.
            .flag("--no-git-checks");

        if self.dry_run.get() {
            command = command.flag("--dry-run");
        }

        command.run();
        Ok(())
    }
}
