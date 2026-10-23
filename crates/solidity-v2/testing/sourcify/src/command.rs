use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    RunCorpus(RunCorpusCommand),
    Report(ReportCommand),
}
/// Compile every contract of a corpus snapshot with Slang v2 and classify what it
/// reports. Exits non-zero on a panic, a failure bucket missing from the expected
/// failures file, or (with `--stale-check`) an expected bucket that no longer fires.
#[derive(Debug, Parser)]
pub struct RunCorpusCommand {
    /// Corpus snapshot directory: `corpus.json` plus `contracts/*.json`.
    pub corpus: PathBuf,

    /// Divide the corpus into this many equally-sized slices and only run one of them.
    #[arg(long, default_value_t = 1)]
    pub shard_count: usize,

    /// Which slice to run. Must be within the range [`0..shard-count`).
    #[arg(long, default_value_t = 0)]
    pub shard_index: usize,

    /// Write one JSON outcome per contract to this file (JSON Lines), for `report`.
    #[arg(long)]
    pub out: Option<PathBuf>,

    #[command(flatten)]
    pub report_options: ReportOptions,

    /// Worker threads. Defaults to the number of CPUs.
    #[arg(long)]
    pub jobs: Option<usize>,

    /// Print every error diagnostic with its file and line, for debugging a failure.
    #[arg(long, default_value_t = false)]
    pub print_diagnostics: bool,
}

/// Render the census and apply the gate over saved `run-corpus --out` results, e.g.
/// the outputs of all shards.
#[derive(Debug, Parser)]
pub struct ReportCommand {
    /// `run-corpus --out` files.
    #[arg(required = true)]
    pub results: Vec<PathBuf>,

    #[command(flatten)]
    pub report_options: ReportOptions,
}

#[derive(Debug, Parser)]
pub struct ReportOptions {
    /// Write the markdown census to this file as well as stdout.
    #[arg(long)]
    pub report: Option<PathBuf>,

    /// Failure buckets to tolerate. Defaults to the crate's `expected-failures.toml`.
    #[arg(long)]
    pub expected_failures: Option<PathBuf>,

    /// Print the census and the gate's verdict, but exit zero regardless.
    #[arg(long, default_value_t = false)]
    pub report_only: bool,

    /// Also fail on expected buckets that did not fire. For runs over the full
    /// corpus, where a silent bucket means the entry is stale.
    #[arg(long, default_value_t = false)]
    pub stale_check: bool,
}
