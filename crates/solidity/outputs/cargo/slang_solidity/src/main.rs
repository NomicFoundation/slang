use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::{Parser as ClapParser, Subcommand};
use semver::Version;
use slang_solidity::graph_builder::{ExecutionConfig, File, Functions, NoCancellation, Variables};
use slang_solidity::kinds::NonTerminalKind;
use slang_solidity::language::Language;

// Below are dependencies used by the API `lib.rs`, but not the CLI "main.rs".
// However, we need to add a fake usage to suppress Cargo warnings about unused dependencies.
// This is a known issue, and we should remove this hack once there is a better solution from Cargo.
// https://github.com/rust-lang/cargo/issues/1982
mod supress_api_dependencies {
    use {
        ariadne as _, metaslang_cst as _, metaslang_graph_builder as _, serde as _, strum as _,
        strum_macros as _, thiserror as _,
    };
}

#[derive(ClapParser, Debug)]
#[command(next_line_help = true)]
#[command(author, about)]
struct Cli {
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

    /// Parses a Solidity (*.sol) source file and builds a graph executing the instructions from the builder file (*.msgb)
    BuildGraph {
        /// File path to the Solidity (*.sol) source file to parse
        file_path: String,

        /// The graph buider (.msgb) file to use
        msgb_path: String,

        /// The Solidity language version to use for parsing
        #[arg(short, long)]
        version: Version,

        /// Print the graph as JSON
        #[clap(long)]
        json: bool,
    },
}

fn main() -> Result<ExitCode> {
    match Cli::parse().command {
        Commands::Parse {
            file_path,
            version,
            json,
        } => execute_parse_command(&file_path, version, json),
        Commands::BuildGraph {
            file_path,
            version,
            msgb_path,
            json,
        } => execute_build_graph_command(&file_path, version, &msgb_path, json),
    }
}

fn execute_parse_command(file_path_string: &str, version: Version, json: bool) -> Result<ExitCode> {
    let file_path = PathBuf::from(&file_path_string)
        .canonicalize()
        .with_context(|| format!("Failed to find file path: {file_path_string:?}"))?;

    let input = fs::read_to_string(file_path)?;
    let language = Language::new(version)?;
    let output = language.parse(NonterminalKind::SourceUnit, &input);

    let errors = output.errors();
    for error in errors {
        const COLOR: bool = true;
        let report = slang_solidity::diagnostic::render(error, file_path_string, &input, COLOR);
        eprintln!("{report}");
    }

    if json {
        let root_node = output.tree();
        let json = serde_json::to_string_pretty(&root_node)?;
        println!("{json}");
    }

    if errors.is_empty() {
        Ok(ExitCode::SUCCESS)
    } else {
        eprintln!("Couldn't parse the Solidity source file.");
        Ok(ExitCode::FAILURE)
    }
}

fn execute_build_graph_command(
    file_path_string: &str,
    version: Version,
    msgb_path_string: &str,
    output_json: bool,
) -> Result<ExitCode> {
    let file_path = PathBuf::from(&file_path_string)
        .canonicalize()
        .with_context(|| format!("Failed to find source file: {file_path_string:?}"))?;

    let input = fs::read_to_string(file_path)?;
    let language = Language::new(version)?;
    let output = language.parse(NonTerminalKind::SourceUnit, &input);

    let errors = output.errors();
    for error in errors {
        let report = error.to_error_report(file_path_string, &input, /* with_color */ true);
        eprintln!("{report}");
    }
    if !errors.is_empty() {
        eprintln!("Couldn't parse the Solidity source file.");
        return Ok(ExitCode::FAILURE);
    }

    let msgb_path = PathBuf::from(&msgb_path_string)
        .canonicalize()
        .with_context(|| format!("Failed to find graph builder source: {msgb_path_string:?}"))?;

    let msgb_source = fs::read_to_string(&msgb_path)?;
    let msgb = match File::from_str(&msgb_source) {
        Ok(msgb) => msgb,
        Err(parser_error) => {
            eprintln!(
                "Failed to parse graph builder file {msgb_path_string}: {}",
                parser_error.display_pretty(&msgb_path, &msgb_source),
            );
            return Ok(ExitCode::FAILURE);
        }
    };

    let functions = Functions::stdlib();
    let variables = Variables::new();
    let execution_config = ExecutionConfig::new(&functions, &variables);

    let tree = output.create_tree_cursor();
    let graph = msgb.execute(&tree, &execution_config, &NoCancellation)?;

    if output_json {
        graph.display_json(None)?;
    } else {
        print!("{}", graph.pretty_print());
    }

    Ok(ExitCode::SUCCESS)
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <Cli as clap::CommandFactory>::command().debug_assert();
}
