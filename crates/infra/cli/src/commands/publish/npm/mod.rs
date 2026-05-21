use std::path::Path;

use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl NpmController {
    pub fn execute(&self) -> Result<()> {
        let version = CargoWorkspace::local_version()?;
        let tarball = Path::repo_path(format!(
            "target/publish-artifacts/nomicfoundation-slang-{version}.tgz"
        ));
        if !tarball.exists() {
            println!("No tarball at {tarball:?}; nothing to publish.");
            return Ok(());
        }

        let mut command = Command::new("pnpm")
            .arg("publish")
            .arg(tarball.unwrap_str())
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
