use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use clap::{Parser as ClapParser, Subcommand};
use semver::Version;
use slang_solidity::{language::Language, syntax::parser::ProductionKind};

mod _supress_library_dependencies {
    // Below are dependencies used by the API `lib.rs`, but not the CLI "main.rs".
    // However, we need to add a fake usage to suppress Cargo warnings about unused dependencies.
    // This is a known issue, and we should remove this hack once there is a better solution from Cargo.
    // https://github.com/rust-lang/cargo/issues/1982
    mod api {
        use ariadne as _;
        use serde as _;
        use strum as _;
        use strum_macros as _;
        use thiserror as _;
    }

    // Make sure codegen runs before building for tests.
    #[cfg(test)]
    mod tests {
        use solidity_cargo_build as _;
    }
}

#[derive(ClapParser, Debug)]
#[command(next_line_help = true)]
#[command(author, about)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parses a Solidity (*.sol) source file, and outputs any syntax errors, or a JSON concrete syntax tree
    Parse {
        /// File path to the Solidity (*.sol) source file to parse
        file_path: String,

        /// The Solidity language version to use for parsing
        #[arg(short, long)]
        version: Version,

        /// Print the concrete syntax tree as JSON
        #[clap(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    return match CLI::parse().command {
        Commands::Parse {
            file_path,
            version,
            json,
        } => execute_parse_command(file_path, version, json),
    };
}

fn execute_parse_command(file_path_string: String, version: Version, json: bool) -> Result<()> {
    let file_path = PathBuf::from(&file_path_string)
        .canonicalize()
        .with_context(|| format!("Failed to find file path: {file_path_string:?}"))?;

    let input = fs::read_to_string(file_path)?;
    let language = Language::new(version)?;
    let output = language.parse(ProductionKind::SourceUnit, &input)?;

    let errors = output.errors();
    for error in errors {
        let report = error.to_error_report(&file_path_string, &input, /* with_colour */ true);
        eprintln!("{report}");
    }

    if json {
        let root_node = output.parse_tree();
        let json = serde_json::to_string_pretty(&root_node)?;
        println!("{json}");
    }

    if !errors.is_empty() {
        std::process::exit(1);
    }

    return Ok(());
}
