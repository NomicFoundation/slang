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
pub struct TestCommand {
    #[command(subcommand)]
    pub chain: Chain,

    #[arg(long, default_value_t = false)]
    pub check_bindings: bool,

    #[arg(long, default_value_t = false)]
    pub trace: bool,
}
