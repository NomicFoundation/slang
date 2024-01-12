mod check;
mod ci;
mod completions;
mod lint;
mod publish;
mod run;
mod setup;
mod test;
mod watch;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::check::CheckController;
use crate::commands::ci::CiController;
use crate::commands::completions::CompletionController;
use crate::commands::lint::LintController;
use crate::commands::publish::PublishController;
use crate::commands::run::RunController;
use crate::commands::setup::SetupController;
use crate::commands::test::TestController;
use crate::commands::watch::WatchController;

#[derive(Debug, Parser)]
#[command(bin_name = "infra")]
pub struct Cli {
    #[command(subcommand)]
    command: AppCommand,
}

#[derive(Debug, Subcommand)]
enum AppCommand {
    /// Setup toolchains and dependencies.
    ///
    /// Running this command without any args will setup everything.
    Setup(SetupController),
    /// Run codegen checks, and makes sure source files are up to date.
    ///
    /// Running this command without any args will check everything.
    Check(CheckController),
    /// Run unit tests.
    ///
    /// Running this command without any args will test everything.
    Test(TestController),
    /// Run linters for formatting, spelling, broken links, and other issues.
    ///
    /// Running this command without any args will lint everything.
    Lint(LintController),
    /// Perform a full CI run locally, by running 'setup', 'check', 'test', and 'lint' (in that order).
    Ci(CiController),
    /// Run specific local binaries within this repository, forwarding any additional arguments along.
    Run(RunController),
    /// Watch for repository file changes, and run the specified workflows as needed.
    Watch(WatchController),
    /// Publish different artifacts from this repository.
    Publish(PublishController),
    /// Generate shell completions for this CLI.
    Completions(CompletionController),
}

impl Cli {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            AppCommand::Setup(command) => command.execute(),
            AppCommand::Check(command) => command.execute(),
            AppCommand::Test(command) => command.execute(),
            AppCommand::Lint(command) => command.execute(),
            AppCommand::Ci(command) => command.execute(),
            AppCommand::Run(command) => command.execute(),
            AppCommand::Watch(command) => command.execute(),
            AppCommand::Publish(command) => command.execute(),
            AppCommand::Completions(command) => command.execute(),
        }
    }
}
