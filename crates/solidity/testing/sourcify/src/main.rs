mod chains;
mod command;
mod compilation_builder;
mod events;
mod import_resolver;
mod reporting;
mod results;
mod sourcify;
mod tests;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use command::{Commands, ShowCombinedResultsCommand};
use events::Events;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use infra_utils::terminal::Terminal;
use results::{display_all_results, AllResults};
use slang_solidity::compilation::CompilationUnit;
use sourcify::{ContractArchive, Manifest};
use tests::{run_in_parallel, run_with_trace, test_single_contract};

fn main() -> Result<()> {
    let command::Cli { command } = command::Cli::parse();
    match command {
        Commands::Test(test_command) => run_test_command(test_command),
        Commands::ShowCombinedResults(results_command) => {
            run_show_combined_results_command(results_command)
        }
    }
}

// `cmd` is passed by value because it's consumed in the spawned thread,
// but for whatever reason clippy can't figure that out
#[allow(clippy::needless_pass_by_value)]
fn run_test_command(cmd: command::TestCommand) -> Result<()> {
    Terminal::step(format!(
        "Initialize {chain}/{network}",
        chain = cmd.chain.name(),
        network = cmd.chain.network_name()
    ));

    let manifest = Manifest::new(cmd.chain, &cmd.sharding_options)?;

    if let Some(contract) = &cmd.contract {
        return test_single_contract(&manifest, contract, &cmd.test_options);
    }

    let archive_count = manifest.archive_count();

    let (tx, rx) = std::sync::mpsc::channel::<ContractArchive>();

    // process_thread vs. fetching_thread - we don't want the thread to take ownership of repo
    // because then repo will be dropped as soon as the last archive is fetched (before processing
    // has finished), which will delete all the archive files
    let process_thread = std::thread::spawn(move || -> Events {
        let mut events = Events::new(archive_count, 0);
        for archive in rx {
            Terminal::step(archive.display_path());

            events.start_archive(archive.contract_count());
            if cmd.trace {
                run_with_trace(&archive, &events, &cmd.test_options);
            } else {
                run_in_parallel(&archive, &events, &cmd.test_options);
            }
            events.finish_archive();

            if !cmd.save {
                archive.clean();
            }
        }

        events
    });

    // Fetching the shards in this closure so that it takes ownership of the sender
    // The sender needs to be dropped so that process_thread can finish
    let fetcher = |t: std::sync::mpsc::Sender<ContractArchive>| {
        for archive in manifest.archives() {
            t.send(archive).unwrap();
        }
    };

    fetcher(tx);

    let events = process_thread.join().unwrap();

    if GitHub::is_running_in_ci() {
        let output_path = PathBuf::from("target").join("__SLANG_SOURCIFY_SHARD_RESULTS__.json");
        let results = events.to_results();
        let value = serde_json::to_string(&results)?;

        std::fs::create_dir_all(output_path.parent().unwrap())?;
        output_path.write_string(value)?;
        println!("Wrote results to {output_path:?}");
    }

    let failure_count = events.failure_count();
    if failure_count > 0 {
        println!(
            "\nFound {failure_count} failure(s). Please check the logs above for more information.\n",
        );
    }

    Ok(())
}

fn run_show_combined_results_command(command: ShowCombinedResultsCommand) -> Result<()> {
    let ShowCombinedResultsCommand { results_file } = command;

    let contents = results_file.read_to_string()?;
    let all_results: AllResults = serde_json::from_str(&contents)?;
    display_all_results(&all_results);
    Ok(())
}
