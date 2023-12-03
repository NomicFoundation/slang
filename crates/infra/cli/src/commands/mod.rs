mod check;
mod ci;
mod lint;
mod publish;
mod run;
mod setup;
mod test;
mod watch;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::{
    check::CheckController, ci::CiController, lint::LintController, publish::PublishController,
    run::RunController, setup::SetupController, test::TestController, watch::WatchController,
};

#[derive(Debug, Parser)]
pub struct CLI {
    #[command(subcommand)]
    command: AppCommand,
}

#[derive(Debug, Subcommand)]
enum AppCommand {
    /// Setup toolchains and dependencies.
    Setup(SetupController),
    /// Run codegen checks, and makes sure source files are up to date.
    Check(CheckController),
    /// Run unit tests.
    Test(TestController),
    /// Run linters for formatting, spelling, broken links, and other issues.
    Lint(LintController),
    /// Perform a full CI run locally, by running 'setup', 'check', 'test', and 'lint' (in that order).
    Ci(CiController),
    /// Run a local binary within this repository, forwarding any additional arguments along.
    Run(RunController),
    /// Build and serve documentation locally, watching for changes.
    Watch(WatchController),
    /// Publish different artifacts from this repository.
    Publish(PublishController),
}

impl CLI {
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
        }
    }
}
