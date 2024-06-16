// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::fs;
use std::path::PathBuf;

use semver::Version;

use super::parse::parse_source_file;
use super::CommandError;
use crate::bindings::{
    ExecutionConfig, File as GraphBuilderFile, Functions, NoCancellation, Variables,
};

pub fn execute(
    file_path_string: &str,
    version: Version,
    msgb_path_string: &str,
    output_json: bool,
    debug: bool,
) -> Result<(), CommandError> {
    let parse_output = parse_source_file(file_path_string, version, |_| ())?;
    let msgb = parse_graph_builder(msgb_path_string)?;

    let functions = Functions::stdlib();
    let variables = Variables::new();
    let mut execution_config = ExecutionConfig::new(&functions, &variables);
    if debug {
        execution_config = execution_config.debug_attributes(
            "_location".into(),
            "_variable".into(),
            "_match".into(),
        );
    }

    let tree = parse_output.create_tree_cursor();
    let graph = msgb.execute(&tree, &execution_config, &NoCancellation)?;

    if output_json {
        graph.display_json(None)?;
    } else {
        print!("{}", graph.pretty_print());
    }

    Ok(())
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
