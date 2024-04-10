mod cargo;
mod changesets;
mod github_release;
mod npm;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::terminal::Terminal;

use crate::commands::publish::cargo::publish_cargo;
use crate::commands::publish::changesets::publish_changesets;
use crate::commands::publish::github_release::publish_github_release;
use crate::commands::publish::npm::publish_npm;
use crate::utils::ClapExtensions;

#[derive(Clone, Debug, Parser)]
pub struct PublishController {
    command: PublishCommand,

    #[arg(long)]
    dry_run: bool,
}

#[derive(Clone, Copy)]
enum DryRun {
    Yes,
    No,
}

impl DryRun {
    fn is_yes(self) -> bool {
        matches!(self, DryRun::Yes)
    }
}

impl From<bool> for DryRun {
    fn from(value: bool) -> Self {
        if value {
            DryRun::Yes
        } else {
            DryRun::No
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum PublishCommand {
    /// Consume pending changesets, update changelogs and package versions, then send a PR.
    Changesets,
    /// Publish source packages to [npmjs.com].
    Npm,
    /// Publish source crates to [crates.io].
    Cargo,
    /// Publish a new release in the GitHub repository.
    GithubRelease,
}

impl PublishController {
    pub fn execute(&self) -> Result<()> {
        Terminal::step(format!("publish {name}", name = self.command.clap_name()));

        let dry_run = DryRun::from(self.dry_run);

        match self.command {
            PublishCommand::Changesets => publish_changesets(),
            PublishCommand::Npm => publish_npm(dry_run),
            PublishCommand::Cargo => publish_cargo(dry_run),
            PublishCommand::GithubRelease => publish_github_release(dry_run),
        }
    }
}
