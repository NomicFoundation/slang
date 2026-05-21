mod cargo;
mod changesets;
mod github_release;
mod mkdocs;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::publish::cargo::CargoController;
use crate::commands::publish::changesets::ChangesetsController;
use crate::commands::publish::github_release::GithubReleaseController;
use crate::commands::publish::mkdocs::MkdocsController;

#[derive(Clone, Debug, Parser)]
pub struct PublishController {
    #[command(subcommand)]
    command: PublishCommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum PublishCommand {
    /// Consume pending changesets, update changelogs and package versions, then send a PR.
    Changesets(ChangesetsController),
    /// Publish the documentation to GitHub pages.
    Mkdocs(MkdocsController),
    /// Publish the user-facing v1 cargo crates to [crates.io].
    Cargo(CargoController),
    /// Publish a new release in the GitHub repository.
    GithubRelease(GithubReleaseController),
}

impl PublishController {
    pub fn execute(&self) -> Result<()> {
        self.command.execute()
    }
}

impl PublishCommand {
    pub fn execute(&self) -> Result<()> {
        match self {
            PublishCommand::Changesets(controller) => controller.execute(),
            PublishCommand::Mkdocs(controller) => controller.execute(),
            PublishCommand::Cargo(controller) => controller.execute(),
            PublishCommand::GithubRelease(controller) => controller.execute(),
        }
    }
}
