mod command;
mod events;
mod reporting;
mod results;
mod run;
mod sourcify;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use command::{Commands, ShowCombinedResultsCommand};
use events::Events;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use infra_utils::terminal::Terminal;
use results::{display_all_results, AllResults};
use run::{run_in_parallel, run_with_trace, test_single_contract};
use sourcify::{ContractArchive, Manifest};

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
    Terminal::step(format!("Initialize chain {chain}", chain = cmd.chain_id,));

    let manifest = Manifest::new(cmd.chain_id, &cmd.sharding_options)
        .inspect_err(|e| eprintln!("Error fetching chain manifest: {e}"))?;

    if let Some(contract) = &cmd.contract {
        return test_single_contract(&manifest, contract, &cmd.test_options);
    }

    let archive_count = manifest.archive_count();

    let (tx, rx) = std::sync::mpsc::channel::<ContractArchive>();

    // Test archives which have been fetched and unpacked
    let testing_thread = std::thread::spawn(move || -> Events {
        let mut events = Events::new(archive_count, 0);
        for archive in rx {
            println!(
                "Display path: {} | Len: {}",
                archive.display_path(),
                archive.display_path().len()
            );
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
        for archive_desc in manifest.archives() {
            match ContractArchive::fetch(archive_desc) {
                Ok(archive) => t.send(archive).unwrap(),
                Err(e) => eprintln!("Failed to fetch archive {}:\n{e}", archive_desc.url),
            }
        }
    };

    fetcher(tx);

    let events = testing_thread.join().unwrap();

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
