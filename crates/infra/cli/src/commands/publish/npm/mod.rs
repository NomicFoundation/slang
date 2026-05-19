use anyhow::{bail, Result};
use clap::Parser;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

use crate::commands::publish::artifacts::{ArtifactPaths, Manifest};
use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl NpmController {
    pub fn execute(&self) -> Result<()> {
        let manifest = Manifest::load()?;
        manifest.verify_integrity()?;

        let Some(npm) = manifest.npm.as_ref() else {
            println!("No npm artifact in manifest; nothing to publish.");
            return Ok(());
        };

        let tarball = Manifest::absolute_path(&npm.path);
        if !tarball.exists() {
            bail!("Prebuilt npm tarball missing: {tarball:?}");
        }

        println!("Publishing prebuilt tarball: {tarball:?}");
        println!("Workspace version: {}", manifest.workspace_version);
        println!("SHA-256: {}", npm.sha256);

        let mut command = Command::new("pnpm")
            .arg("publish")
            .arg(tarball.unwrap_str())
            .property("--access", "public")
            // CI checkout is detached HEAD, which fails pnpm's branch check.
            // The workflow's `branches:` filter and the slang-release environment
            // gate are what actually authorize a release.
            .flag("--no-git-checks")
            // Don't run the npm package's lifecycle scripts during publish; the
            // tarball was already built in the prepare step.
            .flag("--ignore-scripts");

        if self.dry_run.get() {
            command = command.flag("--dry-run");
        }

        command.current_dir(ArtifactPaths::root()).run();

        Ok(())
    }
}
