mod chains;
mod datasets;
mod events;
mod reporting;
mod results;
mod tests;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use infra_utils::terminal::{NumbersDefaultDisplay, Terminal};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use results::{display_all_results, AllResults};

use crate::chains::Chain;
use crate::datasets::{DataSet, SourceFile};
use crate::events::Events;
use crate::tests::{run_test, select_tests, TestSelection};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Test(TestCommand),
    ShowCombinedResults(ShowCombinedResultsCommand),
}

#[derive(Debug, Parser)]
struct TestCommand {
    /// Chain and sub-network to run against.
    #[command(subcommand)]
    chain: Chain,

    #[command(flatten)]
    sharding_options: ShardingOptions,

    /// Disables parallelism, and logs traces to help with debugging errors or panics.
    #[arg(long, default_value_t = false)]
    trace: bool,

    /// Enables checking bindings for each contract, failing if any symbol cannot be resolved.
    #[arg(long, default_value_t = false)]
    check_bindings: bool,

    #[arg(long, default_value_t = false)]
    check_infer_version: bool,
}

#[derive(Debug, Parser)]
struct ShardingOptions {
    /// If splitting files across multiple shards, this is the total number of shards.
    #[arg(long, requires = "shard_index")]
    shards_count: Option<usize>,

    /// If splitting files across multiple shards, this is the index of this shard (0-based).
    #[arg(long, requires = "shards_count")]
    shard_index: Option<usize>,
}

#[derive(Debug, Parser)]
struct ShowCombinedResultsCommand {
    results_file: PathBuf,
}

fn main() -> Result<()> {
    let Cli { command } = Cli::parse();

    match command {
        Commands::Test(test_command) => run_test_command(test_command),
        Commands::ShowCombinedResults(show_command) => {
            run_show_combined_results_command(show_command)
        }
    }
}

fn run_test_command(command: TestCommand) -> Result<()> {
    let TestCommand {
        chain,
        sharding_options,
        trace,
        check_bindings,
        check_infer_version,
    } = command;

    Terminal::step(format!(
        "initialize {chain}/{network}",
        chain = chain.name(),
        network = chain.network_name(),
    ));

    let dataset = DataSet::initialize(&chain)?;

    let TestSelection {
        directories,
        files_count,
    } = select_tests(&dataset, &sharding_options);

    println!();
    println!(
            "Testing the following {directories_count} directories, containing a total of {files_count} source files:",
            directories_count = directories.len().num_display(),
            files_count = files_count.num_display(),
    );
    println!();
    println!("{directories:?}");
    println!();

    let mut events = Events::new(directories.len(), files_count);

    for directory in directories {
        Terminal::step(format!("testing directory '{directory}'"));

        let files = dataset.checkout_directory(directory);

        events.start_directory(files.len());

        if trace {
            run_with_traces(files, &events, check_bindings, check_infer_version)?;
        } else {
            run_in_parallel(files, &events, check_bindings, check_infer_version)?;
        }

        events.finish_directory();
    }

    if GitHub::is_running_in_ci() {
        let output_path = PathBuf::from("target").join("__SLANG_SANCTUARY_SHARD_RESULTS__.json");
        let results = events.to_results();
        let value = serde_json::to_string(&results)?;

        std::fs::create_dir_all(output_path.parent().unwrap())?;
        output_path.write_string(value)?;
        println!("Wrote results to {output_path:?}");
    }

    let failure_count = events.failure_count();
    if failure_count > 0 {
        println!();
        println!(
            "Found {failure_count} failure(s). Please check the logs above for more information.",
        );
        println!();

        // Exit cleanly without useless stack traces:
        #[allow(clippy::exit)]
        std::process::exit(1);
    }

    Ok(())
}

fn run_with_traces(
    files: &Vec<SourceFile>,
    events: &Events,
    check_bindings: bool,
    check_infer_version: bool,
) -> Result<()> {
    for file in files {
        let compiler = &file.compiler;
        let path = file.path.strip_repo_root()?;

        events.trace(format!("[{compiler}] Starting: {path:?}"));

        run_test(file, events, check_bindings, check_infer_version)?;

        events.trace(format!("[{compiler}] Finished: {path:?}"));
    }

    Ok(())
}

fn run_in_parallel(
    files: &Vec<SourceFile>,
    events: &Events,
    check_bindings: bool,
    check_infer_version: bool,
) -> Result<()> {
    files
    .par_iter()
    .panic_fuse(/* Halt as soon as possible if a child panics */)
    .try_for_each(|file| run_test(file, events, check_bindings, check_infer_version))
}

fn run_show_combined_results_command(command: ShowCombinedResultsCommand) -> Result<()> {
    let ShowCombinedResultsCommand { results_file } = command;

    let contents = results_file.read_to_string()?;
    let all_results: AllResults = serde_json::from_str(&contents)?;
    display_all_results(&all_results);
    Ok(())
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <Cli as clap::CommandFactory>::command().debug_assert();
}
