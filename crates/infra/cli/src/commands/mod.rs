mod check;
mod ci;
mod completions;
mod lint;
mod perf;
mod publish;
mod run;
mod setup;
mod test;
mod watch;

use anyhow::Result;
use clap::{command, Parser, Subcommand};

use crate::commands::check::CheckController;
use crate::commands::ci::CiController;
use crate::commands::completions::CompletionController;
use crate::commands::lint::LintController;
use crate::commands::perf::PerfController;
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
pub enum AppCommand {
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
    /// Executes performance benchmarks, and reports the results.
    Perf(PerfController),
    /// Publish different artifacts from this repository.
    Publish(PublishController),
    /// Generate shell completions for this CLI.
    Completions(CompletionController),
}

impl Cli {
    pub fn execute(&self) -> Result<()> {
        self.command.execute()
    }
}

impl AppCommand {
    pub fn execute(&self) -> Result<()> {
        match self {
            AppCommand::Setup(controller) => controller.execute()?,
            AppCommand::Check(controller) => controller.execute()?,
            AppCommand::Test(controller) => controller.execute()?,
            AppCommand::Lint(controller) => controller.execute()?,
            AppCommand::Ci(controller) => controller.execute()?,
            AppCommand::Run(controller) => controller.execute(),
            AppCommand::Watch(controller) => controller.execute()?,
            AppCommand::Perf(controller) => controller.execute()?,
            AppCommand::Publish(controller) => controller.execute()?,
            AppCommand::Completions(controller) => controller.execute(),
        };

        Ok(())
    }
}
