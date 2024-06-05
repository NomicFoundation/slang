use std::process::ExitCode;

use clap::Parser as ClapParser;
use slang_solidity::cli::Commands;

// Below are dependencies used by the API `lib.rs`, but not the CLI "main.rs".
// However, we need to add a fake usage to suppress Cargo warnings about unused dependencies.
// This is a known issue, and we should remove this hack once there is a better solution from Cargo.
// https://github.com/rust-lang/cargo/issues/1982
mod supress_api_dependencies {
    use {
        ariadne as _, metaslang_cst as _, semver as _, serde as _, serde_json as _, strum as _,
        strum_macros as _, thiserror as _,
    };
    #[cfg(feature = "__experimental_bindings_api")]
    use {metaslang_graph_builder as _, once_cell as _, stack_graphs as _};
}

#[derive(ClapParser, Debug)]
#[command(next_line_help = true)]
#[command(author, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> ExitCode {
    Cli::parse().command.execute()
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <Cli as clap::CommandFactory>::command().debug_assert();
}
