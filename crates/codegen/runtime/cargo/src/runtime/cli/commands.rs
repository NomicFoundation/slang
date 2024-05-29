use std::fs;
use std::path::PathBuf;

use semver::Version;
use thiserror::Error;

use crate::diagnostic;
use crate::graph_builder::{
    ExecutionConfig, ExecutionError, File as GraphBuilderFile, Functions, NoCancellation, Variables,
};
use crate::language::{Error as LanguageError, Language};
use crate::parse_output::ParseOutput;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("File not found: {0:?}")]
    FileNotFound(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    LanguageError(#[from] LanguageError),

    #[error("Parsing failed: {0}")]
    ParseFailed(String),

    #[error(transparent)]
    ExecutionFailed(#[from] ExecutionError),
}

pub fn execute_parse_command(
    file_path_string: &str,
    version: Version,
    json: bool,
) -> Result<(), CommandError> {
    parse_source_file(file_path_string, version, |output| {
        if json {
            let root_node = output.tree();
            let json = serde_json::to_string_pretty(&root_node).expect("JSON serialization failed");
            println!("{json}");
        }
    })
    .map(|_| ())
}

pub fn execute_build_graph_command(
    file_path_string: &str,
    version: Version,
    msgb_path_string: &str,
    output_json: bool,
) -> Result<(), CommandError> {
    let parse_output = parse_source_file(file_path_string, version, |_| ())?;
    let msgb = parse_graph_builder(msgb_path_string)?;

    let functions = Functions::stdlib();
    let variables = Variables::new();
    let execution_config = ExecutionConfig::new(&functions, &variables);

    let tree = parse_output.create_tree_cursor();
    let graph = msgb.execute(&tree, &execution_config, &NoCancellation)?;

    if output_json {
        graph.display_json(None)?;
    } else {
        print!("{}", graph.pretty_print());
    }

    Ok(())
}

fn parse_source_file<F>(
    file_path_string: &str,
    version: Version,
    run_before_checking: F,
) -> Result<ParseOutput, CommandError>
where
    F: FnOnce(&ParseOutput),
{
    let file_path = PathBuf::from(&file_path_string)
        .canonicalize()
        .map_err(|_| CommandError::FileNotFound(file_path_string.to_string()))?;

    let input = fs::read_to_string(file_path)?;
    let language = Language::new(version)?;
    let parse_output = language.parse(Language::root_kind(), &input);

    run_before_checking(&parse_output);

    if parse_output.is_valid() {
        Ok(parse_output)
    } else {
        const COLOR: bool = true;
        let report = parse_output
            .errors()
            .iter()
            .map(|error| diagnostic::render(error, file_path_string, &input, COLOR))
            .collect::<Vec<_>>()
            .join("\n");
        Err(CommandError::ParseFailed(report))
    }
}

fn parse_graph_builder(msgb_path_string: &str) -> Result<GraphBuilderFile, CommandError> {
    let msgb_path = PathBuf::from(&msgb_path_string)
        .canonicalize()
        .map_err(|_| CommandError::FileNotFound(msgb_path_string.to_string()))?;

    let msgb_source = fs::read_to_string(&msgb_path)?;
    GraphBuilderFile::from_str(&msgb_source).map_err(|parser_error| {
        let error_message = parser_error
            .display_pretty(&msgb_path, &msgb_source)
            .to_string();
        CommandError::ParseFailed(error_message)
    })
}
