// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::BTreeSet;
use std::path::PathBuf;

use semver::Version;
use stack_graphs::graph::StackGraph;
use stack_graphs::partial::PartialPaths;
use stack_graphs::stitching::{ForwardPartialPathStitcher, GraphEdgeCandidates, StitcherConfig};

use crate::bindings::stack_graph::{self, Builder, StackGraphLanguage};
use crate::bindings::Variables;

pub fn execute(
    file_path_string: &str,
    version: Version,
    msgb_path_string: &str,
) -> Result<(), super::CommandError> {
    let parse_output = super::parse::parse_source_file(file_path_string, version, |_| ())?;
    let msgb = super::build_graph::parse_graph_builder(msgb_path_string)?;
    let sgl = StackGraphLanguage::new(msgb);

    let mut stack_graph = StackGraph::new();
    let file_path = PathBuf::from(file_path_string);
    let root_path = file_path.parent().unwrap();
    let file_name = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .expect("Source file has a name");
    let file = stack_graph.get_or_create_file(file_name);
    let tree_cursor = parse_output.create_tree_cursor();
    let mut globals = Variables::new();
    globals
        .add(
            stack_graph::ROOT_PATH_VAR.into(),
            root_path.to_str().unwrap().into(),
        )
        .expect("failed to add ROOT_PATH variable");
    globals
        .add(
            stack_graph::FILE_PATH_VAR.into(),
            file_path.to_str().unwrap().into(),
        )
        .expect("failed to add FILE_PATH variable");

    let mut builder = sgl.builder_into_stack_graph(&mut stack_graph, file, tree_cursor);
    builder.build(&globals, &stack_graph::NoCancellation)?;

    print_defs_and_refs(&builder);
    resolve_refs(&stack_graph);

    Ok(())
}

fn print_defs_and_refs(builder: &Builder<'_>) {
    let stack_graph = &builder.stack_graph;
    for handle in stack_graph.iter_nodes() {
        let node = &stack_graph[handle];
        let syntax_node_ref = builder.node_handle_to_syntax_ref(handle);

        if node.is_definition() {
            println!(
                "Node #{node} is definition",
                node = node.display(stack_graph),
            );
        } else if node.is_reference() {
            println!(
                "Node #{node} is reference",
                node = node.display(stack_graph)
            );
        }
        if let Some(syntax_ref) = syntax_node_ref {
            let cursor = &builder.graph[*syntax_ref];
            println!("{:?}", cursor.text_range());
        }
    }
}

fn resolve_refs(stack_graph: &StackGraph) {
    let mut paths = PartialPaths::new();
    let mut results = BTreeSet::new();
    let references = stack_graph
        .iter_nodes()
        .filter(|handle| stack_graph[*handle].is_reference())
        .collect::<Vec<_>>();
    for reference in &references {
        println!(
            "Found ref: {reference}",
            reference = reference.display(stack_graph)
        );
    }
    ForwardPartialPathStitcher::find_all_complete_partial_paths(
        &mut GraphEdgeCandidates::new(stack_graph, &mut paths, None),
        references,
        StitcherConfig::default(),
        &stack_graphs::NoCancellation,
        |graph, paths, path| {
            results.insert(path.display(graph, paths).to_string());
        },
    )
    .expect("should never be cancelled");

    for result in &results {
        println!("Found path: {result}");
    }
}
