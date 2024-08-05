use anyhow::Result;
use clap::{Parser, ValueEnum};

use crate::toolchains::mkdocs::Mkdocs;
use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct MkdocsController {
    /// The target version to publish.
    #[arg(long)]
    target: PublishTarget,

    #[command(flatten)]
    dry_run: DryRun,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum PublishTarget {
    MainBranch,
    LatestRelease,
}

impl MkdocsController {
    pub fn execute(&self) -> Result<()> {
        match self.target {
            PublishTarget::MainBranch => Mkdocs::publish_main_branch(self.dry_run),
            PublishTarget::LatestRelease => Mkdocs::publish_latest_release(self.dry_run),
        }
    }
}
