use std::path::Path;

use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

use crate::toolchains::npm::Npm;
use crate::toolchains::wasm::NPM_CRATE;
use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl NpmController {
    pub fn execute(&self) -> Result<()> {
        // Read the version from the same source `pnpm pack` did (package.json),
        // so the computed tarball name can't drift from what's on disk.
        let package_dir = CargoWorkspace::locate_source_crate(NPM_CRATE)?;
        let version = Npm::local_version(&package_dir)?;
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
