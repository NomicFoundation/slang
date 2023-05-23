use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::{Parser as ClapParser, Subcommand};
use semver::Version;
use slang_solidity::{syntax::parser::ProductionKind, Language};

mod _supress_library_dependencies_ {
    // Below are dependencies used by the library `lib.rs`, but not here.
    // However, we need to add a fake usage below to supress Cargo warnings about unused dependencies.
    // This is a known issue, and we should remove this hack once there is a better solution from Cargo.
    // https://github.com/rust-lang/cargo/issues/1982
    use ariadne as _;
    use serde as _;
    #[cfg(test)]
    use solidity_cargo_build as _;
    use strum as _;
    use strum_macros as _;
    use thiserror as _;
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

fn execute_parse_command(file_path: String, version: Version, json: bool) -> Result<()> {
    let input_file = &PathBuf::from(file_path).canonicalize()?;
    let input = fs::read_to_string(input_file)?;

    let language = Language::new(version)?;
    let output = language.parse(ProductionKind::SourceUnit, &input);

    let errors = output.errors();
    for error in errors {
        eprintln!(
            "{report}",
            report = error.to_error_report(
                input_file.to_str().unwrap(),
                &input,
                /* with_colour */ true,
            )
        );
    }

    if json {
        if let Some(root_node) = output.parse_tree() {
            let json = serde_json::to_string_pretty(&root_node)?;
            println!("{json}");
        }
    }

    if !errors.is_empty() {
        std::process::exit(1);
    }

    return Ok(());
}
