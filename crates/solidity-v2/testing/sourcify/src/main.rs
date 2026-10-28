mod command;
mod corpus;
mod corpus_run;
mod lock;

use std::path::Path;
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
        Commands::Lock(lock_command) => run_lock_command(&lock_command),
    }
}

fn run_lock_command(cmd: &command::LockCommand) -> Result<()> {
    let path = cmd
        .lock
        .clone()
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus.lock"));
    let lock = lock::CorpusLock::load(&path)?;
    let print = |name: &str, sha256: &str, bytes: u64, contracts: u64| {
        println!("{name}\t{sha256}\t{bytes}\t{contracts}");
    };
    match &cmd.query {
        command::LockQuery::Release => println!("{}\t{}", lock.repo, lock.tag),
        command::LockQuery::Pr => print(&lock.pr.asset, &lock.pr.sha256, 0, 0),
        command::LockQuery::Shard { count, index } => {
            anyhow::ensure!(*index < *count, "index must be less than count");
            for asset in lock.shard(*count, *index) {
                print(&asset.name, &asset.sha256, asset.bytes, asset.contracts);
            }
        }
    }
    Ok(())
}
