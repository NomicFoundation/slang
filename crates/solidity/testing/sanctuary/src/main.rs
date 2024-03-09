mod chains;
mod datasets;
mod events;
mod reporting;
mod tests;

use anyhow::Result;
use clap::Parser;
use infra_utils::paths::PathExtensions;
use infra_utils::terminal::{NumbersDefaultDisplay, Terminal};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::chains::Chain;
use crate::datasets::{DataSet, SourceFile};
use crate::events::Events;
use crate::tests::{run_test, select_tests, TestSelection};

#[derive(Debug, Parser)]
struct Cli {
    /// Chain and sub-network to run against.
    #[command(subcommand)]
    chain: Chain,

    #[command(flatten)]
    sharding_options: ShardingOptions,

    /// Disables parallelism, and logs traces to help with debugging errors or panics.
    #[arg(long, default_value_t = false)]
    trace: bool,
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

fn main() -> Result<()> {
    let Cli {
        chain,
        sharding_options,
        trace,
    } = Cli::parse();

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

        let files = dataset.checkout_directory(directory)?;

        events.start_directory(files.len());

        if trace {
            run_with_traces(files, &events)?;
        } else {
            run_in_parallel(files, &events)?;
        }

        events.finish_directory();
    }

    let failure_count = events.failure_count();
    if failure_count > 0 {
        println!();
        println!(
            "Found {failure_count} failure{suffix}. Please check the logs above for more information.",
            suffix = if failure_count == 1 { "" } else { "s" },
        );
        println!();

        // Exit cleanly without useless stack traces:
        #[allow(clippy::exit)]
        std::process::exit(1);
    }

    Ok(())
}

fn run_with_traces(files: &Vec<SourceFile>, events: &Events) -> Result<()> {
    for file in files {
        let compiler = &file.compiler;
        let path = file.path.strip_repo_root()?;

        events.trace(format!("[{compiler}] Starting: {path:?}"));

        run_test(file, events)?;

        events.trace(format!("[{compiler}] Finished: {path:?}"));
    }

    Ok(())
}

fn run_in_parallel(files: &Vec<SourceFile>, events: &Events) -> Result<()> {
    files
    .par_iter()
    .panic_fuse(/* Halt as soon as possible if a child panics */)
    .try_for_each(|file| run_test(file, events))
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <Cli as clap::CommandFactory>::command().debug_assert();
}
