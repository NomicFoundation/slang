mod chains;
mod metadata;
mod command;
mod sourcify;
mod events;
mod reporting;
mod results;

use events::{Events, TestOutcome};
use sourcify::{ContractArchive, Repository};

use anyhow::Result;
use clap::Parser;
use rayon::{iter::ParallelBridge, prelude::ParallelIterator};

fn main() -> Result<()> {
    let command::Cli { command } = command::Cli::parse();

    match command {
        command::Commands::Test(test_command) => run_test_command(&test_command),
    }
}

fn run_test_command(cmd: &command::TestCommand) -> Result<()> {
    let repo = Repository::new(cmd.chain)?;
    let shards = repo.fetch_entries()?;

    let mut events = Events::new(shards.len(), 0);

    for shard in shards {
        let archive = repo.fetch_archive(&shard)?;

        events.start_directory(archive.contract_count());

        run_in_parallel(&archive, &events);

        events.finish_directory();
    }

    Ok(())
}

fn run_in_parallel(archive: &ContractArchive, events: &Events) {
    archive.into_iter().flatten().par_bridge().panic_fuse().for_each(|contract| {
        let mut contract_buffer = String::new();

        let sources_count = contract.sources_count();
        events.inc_files_count(sources_count);

        let mut did_fail = false;
        match contract.create_compilation_unit(&mut contract_buffer) {
            Ok(unit) => {
                for file in unit.files() {
                    if !file.errors().is_empty() {
                        for error in file.errors() {
                            events.parse_error(error.message());
                        }
                        did_fail = true;
                    }
                }
            }
            Err(e) => {
                events.trace(format!("Failed to compile {}: {e}", contract.name));
                did_fail = true;
            }
        }

        events.inc_files_processed(sources_count);

        if did_fail {
            events.test(TestOutcome::Failed);
        } else {
            events.test(TestOutcome::Passed);
        }
    })
}
