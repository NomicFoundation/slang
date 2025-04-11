use std::ops::Range;

use crate::chains::Chain;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Test(TestCommand),
}

#[derive(Debug, Parser)]
pub struct ShardingOptions {
    #[arg(long, requires = "shard_index")]
    /// Divide the dataset into a smaller number of shards. Must be a factor of 256. 'shard_index'
    /// must be included along with this option.
    pub shard_count: Option<usize>,

    #[arg(long, requires = "shard_count")]
    /// Select a single shard to test. Must be within the range [0..shard_count). Required if
    /// 'shard_count' is specified.
    pub shard_index: Option<usize>,

    #[arg(long, default_value_t = false)]
    /// If set, will only test contracts under the 'full_match' category.
    pub exclude_partials: bool,
}

impl ShardingOptions {
    pub fn get_id_range(&self) -> Range<u16> {
        if let Some(shard_count) = self.shard_count {
            let shard_index = self.shard_index.unwrap() as u16;

            let shard_size: u16 = 256 / (shard_count as u16);
            let shard_start: u16 = (shard_size as u16) * shard_index;

            shard_start..(shard_start + shard_size)
        } else {
            0..256
        }
    }
}

#[derive(Debug, Parser)]
pub struct TestCommand {
    #[command(subcommand)]
    pub chain: Chain,

    #[command(flatten)]
    pub sharding_options: ShardingOptions,

    #[arg(long, conflicts_with = "sharding_options")]
    /// Specify a single contract to test using the contract address.
    pub contract: Option<String>,

    #[arg(long, default_value_t = false)]
    /// Run bindings tests.
    pub check_bindings: bool,

    #[arg(long, default_value_t = false)]
    /// Run tests sequentially, and output extra logging. Tests will run significantly slower
    /// with this option enabled.
    pub trace: bool,
}

impl TestCommand {
    pub fn sharding_options(&self) -> Option<(usize, usize)> {
        if self.sharding_options.shard_count.is_none() {
            None
        } else {
            Some((self.sharding_options.shard_count.unwrap(), self.sharding_options.shard_index.unwrap()))
        }
    }
}
