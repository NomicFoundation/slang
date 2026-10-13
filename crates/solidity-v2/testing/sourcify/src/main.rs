mod command;
mod corpus;
mod corpus_run;

use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use command::Commands;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            // The message is the verdict; anyhow's backtrace would bury it in CI logs.
            eprintln!("Error: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let command::Cli { command } = command::Cli::parse();
    match command {
        Commands::RunCorpus(corpus_command) => corpus_run::run(&corpus_command),
        Commands::Report(report_command) => corpus_run::report(&report_command),
    }
}
