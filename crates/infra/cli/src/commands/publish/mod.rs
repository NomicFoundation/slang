mod cargo;
mod changesets;
mod check;
mod github_release;
mod mkdocs;
mod npm;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::publish::cargo::CargoController;
use crate::commands::publish::changesets::ChangesetsController;
use crate::commands::publish::check::CheckController;
use crate::commands::publish::github_release::GithubReleaseController;
use crate::commands::publish::mkdocs::MkdocsController;
use crate::commands::publish::npm::NpmController;

#[derive(Clone, Debug, Parser)]
pub struct PublishController {
    #[command(subcommand)]
    command: PublishCommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum PublishCommand {
    /// Check if the local version differs from the latest published release.
    Check(CheckController),
    /// Consume pending changesets, update changelogs and package versions, then send a PR.
    Changesets(ChangesetsController),
    /// Publish the documentation to GitHub pages.
    Mkdocs(MkdocsController),
    /// Publish source packages to [npmjs.com].
    Npm(NpmController),
    /// Publish source crates to [crates.io].
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
            PublishCommand::Check(controller) => controller.execute(),
            PublishCommand::Changesets(controller) => controller.execute(),
            PublishCommand::Mkdocs(controller) => controller.execute(),
            PublishCommand::Npm(controller) => controller.execute(),
            PublishCommand::Cargo(controller) => controller.execute(),
            PublishCommand::GithubRelease(controller) => controller.execute(),
        }
    }
}
