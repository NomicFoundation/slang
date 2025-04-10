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

// Need: sharding options

#[derive(Debug, Parser)]
pub struct ShardingOptions {
    #[arg(long, requires = "shard_index")]
    shard_count: Option<usize>,

    #[arg(long, requires = "shard_count")]
    shard_index: Option<usize>,
}

#[derive(Debug, Parser)]
pub struct TestCommand {
    #[command(subcommand)]
    pub chain: Chain,

    #[command(flatten)]
    pub sharding_options: ShardingOptions,

    #[arg(long, conflicts_with = "sharding_options")]
    pub contract: Option<String>,

    #[arg(long, default_value_t = false)]
    pub check_bindings: bool,

    #[arg(long, default_value_t = false)]
    pub trace: bool,

    
}
