use std::fs;

use anyhow::{bail, Result};
use clap::Parser;
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
        let root = std::path::Path::repo_path("target/publish-artifacts");
        let tarballs: Vec<_> = fs::read_dir(&root)?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("tgz"))
            .collect();
        let tarball = match tarballs.as_slice() {
            [] => {
                println!("No tarball in {root:?}; nothing to publish.");
                return Ok(());
            }
            [one] => one,
            many => bail!(
                "Expected one .tgz in {root:?}, found {}: {many:?}",
                many.len()
            ),
        };

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
