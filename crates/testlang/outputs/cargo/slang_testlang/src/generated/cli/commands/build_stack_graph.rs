// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::fs;
use std::path::PathBuf;

use semver::Version;
use stack_graphs::graph::StackGraph;
use thiserror::Error;

use crate::bindings::{
    ExecutionConfig, ExecutionError, File as GraphBuilderFile, Functions, NoCancellation, Variables,
};
use crate::cli::stack_graph::{self, StackGraphLanguage};
use crate::diagnostic;
use crate::language::{Error as LanguageError, Language};
use crate::parse_output::ParseOutput;

pub fn execute(
    file_path_string: &str,
    version: Version,
    msgb_path_string: &str,
) -> Result<(), super::CommandError> {
    let parse_output = super::parse::parse_source_file(file_path_string, version, |_| ())?;
    let msgb = super::build_graph::parse_graph_builder(msgb_path_string)?;
    let sgl = StackGraphLanguage::new(msgb);

    let mut stack_graph = StackGraph::new();
    let file = stack_graph.get_or_create_file(file_path_string);
    let tree_cursor = parse_output.create_tree_cursor();
    let globals = Variables::new();
    sgl.build_stack_graph_into(
        &mut stack_graph,
        file,
        tree_cursor,
        &globals,
        &stack_graph::NoCancellation,
    )?;

    for handle in stack_graph.iter_nodes() {
        let node = &stack_graph[handle];
        if node.is_definition() {
            println!(
                "Node #{node} is definition",
                node = node.display(&stack_graph)
            );
        } else if node.is_reference() {
            println!(
                "Node #{node} is reference",
                node = node.display(&stack_graph)
            );
        }
    }

    Ok(())
}
