use std::fmt::Display;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{Parser, Subcommand, ValueEnum};

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
    /// Chain to pull contracts from. See <https://docs.sourcify.dev/docs/chains/> to get a list of valid chain IDs.
    /// Defaults to Ethereum Mainnet.
    #[arg(long, default_value_t = ChainId(1))]
    pub chain_id: ChainId,

    #[command(flatten)]
    pub test_options: TestOptions,

    #[command(flatten)]
    pub sharding_options: ShardingOptions,

    #[command(flatten)]
    pub archive_options: ArchiveOptions,

    /// Specify a single contract to test using the contract address.
    #[arg(long, conflicts_with = "shard_count")]
    pub contract: Option<String>,

    /// Run tests sequentially, and output extra logging. Tests will run significantly slower
    /// with this option enabled.
    #[arg(long, default_value_t = false)]
    pub trace: bool,
}

#[derive(Debug, Parser)]
pub struct ShowCombinedResultsCommand {
    pub results_file: PathBuf,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
pub struct TestOptions {
    /// Run version inference tests.
    #[arg(long, default_value_t = false)]
    pub check_infer_version: bool,

    /// Run bindings tests
    #[arg(long, value_enum)]
    pub check_binder: Option<CheckBinderMode>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum CheckBinderMode {
    V1,
    V2,
    Compare,
}

#[derive(Debug, Parser)]
pub struct ShardingOptions {
    /// Divide the dataset into a smaller number of shards. Must be a factor of 256. '`--shard-index`'
    /// must be included along with this option.
    #[arg(long, requires = "shard_index", value_parser = validate_shard_count)]
    pub shard_count: Option<u16>,

    /// Select a single shard to test. Must be within the range [`0..shard-count`). Required if
    /// '`--shard-count`' is specified.
    #[arg(long, requires = "shard_count")]
    pub shard_index: Option<u8>,

    /// If set, will only test contracts under the '`full_match`' category.
    #[arg(long, default_value_t = false)]
    pub exclude_partial_matches: bool,
}

#[derive(Debug, Parser)]
pub struct ArchiveOptions {
    /// Don't attempt to download files and fail if some are not available
    #[arg(long, default_value_t = false)]
    pub offline: bool,
}

fn validate_shard_count(count: &str) -> Result<u16, String> {
    let count = count.parse::<u16>().map_err(|_| "Invalid shard count")?;
    if count > 0 && count <= 256 && (256 % count) == 0 {
        Ok(count)
    } else {
        Err("Shard count must be in the range 1..=256 and must divide 256".to_owned())
    }
}

impl ShardingOptions {
    // clippy complains about the exclusive range below, but its suggestion (change to exclusive range)
    // doesn't work because we're returning a `RangeInclusive`
    #[allow(clippy::range_minus_one)]
    pub fn get_id_range(&self) -> RangeInclusive<u8> {
        if let Some(shard_count) = self.shard_count {
            let shard_size = 256 / shard_count;

            let shard_index = self.shard_index.unwrap();
            let shard_start = shard_size * u16::from(shard_index);
            let shard_end = shard_start + shard_size - 1;

            // at this point both ends of the range should fit in a u8
            let shard_start = u8::try_from(shard_start).unwrap();
            let shard_end = u8::try_from(shard_end).unwrap();

            shard_start..=shard_end
        } else {
            0..=255
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
#[repr(transparent)]
pub struct ChainId(pub u64);

impl Display for ChainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ChainId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val: u64 = s.parse()?;
        Ok(ChainId(val))
    }
}
