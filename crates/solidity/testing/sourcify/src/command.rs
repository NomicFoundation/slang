use std::ops::RangeInclusive;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::chains::Chain;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Test(TestCommand),
    ShowCombinedResults(ShowCombinedResultsCommand),
}

#[derive(Debug, Parser)]
pub struct TestCommand {
    #[command(subcommand)]
    pub chain: Chain,

    #[command(flatten)]
    pub test_options: TestOptions,

    #[command(flatten)]
    pub sharding_options: ShardingOptions,

    /// Specify a single contract to test using the contract address.
    #[arg(long, conflicts_with = "shard_count")]
    pub contract: Option<String>,

    /// Save the fetch archive under `target/` and don't delete it after the test
    /// is complete. Only used for debugging purposes. Requires you to select a
    /// specific contract to test using the `--contract` option.
    #[arg(long, requires = "contract", default_value_t = false)]
    pub save: bool,

    /// Run tests sequentially, and output extra logging. Tests will run significantly slower
    /// with this option enabled.
    #[arg(long, default_value_t = false)]
    pub trace: bool,
}

#[derive(Debug, Parser)]
pub struct ShowCombinedResultsCommand {
    pub results_file: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TestOptions {
    #[arg(long, default_value_t = true)]
    pub check_parser: bool,

    /// Run bindings tests.
    #[arg(long, default_value_t = false)]
    pub check_bindings: bool,

    #[arg(long, default_value_t = false)]
    pub check_infer_version: bool,
}

#[derive(Debug, Parser)]
pub struct ShardingOptions {
    /// Divide the dataset into a smaller number of shards. Must be a factor of 256. '`shard_index`'
    /// must be included along with this option.
    #[arg(long, requires = "shard_index")]
    pub shard_count: Option<u8>,

    /// Select a single shard to test. Must be within the range [`0..shard_count`). Required if
    /// '`shard_count`' is specified.
    #[arg(long, requires = "shard_count")]
    pub shard_index: Option<u8>,

    /// If set, will only test contracts under the '`full_match`' category.
    #[arg(long, default_value_t = false)]
    pub exclude_partial_matches: bool,
}

impl ShardingOptions {
    // clippy complains about the exclusive range below, but its suggestion (change to exclusive range)
    // doesn't work because we're returning a `RangeInclusive`
    #[allow(clippy::range_minus_one)]
    pub fn get_id_range(&self) -> RangeInclusive<u8> {
        if let Some(shard_count) = self.shard_count {
            let shard_size = u8::try_from(256 / u16::from(shard_count)).unwrap();

            let shard_index = self.shard_index.unwrap();
            let shard_start = shard_size * shard_index;

            shard_start..=(shard_start + shard_size - 1)
        } else {
            0..=255
        }
    }
}
