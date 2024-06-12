use semver::Version;

use crate::bindings;

pub fn execute(
    file_path_string: &str,
    version: Version,
) -> Result<(), super::CommandError> {
    let mut bindings = bindings::create_for(version.clone());
    let parse_output = super::parse::parse_source_file(file_path_string, version, |_| ())?;
    let tree_cursor = parse_output.create_tree_cursor();

    bindings.add_file(file_path_string, tree_cursor)?;

    // print_defs_and_refs(&bindings);
    // resolve_refs(&bindings);

    Ok(())
}

// fn print_defs_and_refs(bindings: &Bindings) {
//     let stack_graph = &bindings.stack_graph;
//     for handle in stack_graph.iter_nodes() {
//         let node = &stack_graph[handle];
//         let syntax_node_ref = bindings.node_handle_to_syntax_ref(handle);

//         if node.is_definition() {
//             println!(
//                 "Node #{node} is definition",
//                 node = node.display(stack_graph),
//             );
//         } else if node.is_reference() {
//             println!(
//                 "Node #{node} is reference",
//                 node = node.display(stack_graph)
//             );
//         }
//         if let Some(syntax_ref) = syntax_node_ref {
//             let cursor = &bindings.graph[*syntax_ref];
//             println!("{:?}", cursor.text_range());
//         }
//     }
// }

// fn resolve_refs(stack_graph: &StackGraph) {
//     let mut paths = PartialPaths::new();
//     let mut results = BTreeSet::new();
//     let references = stack_graph
//         .iter_nodes()
//         .filter(|handle| stack_graph[*handle].is_reference())
//         .collect::<Vec<_>>();
//     for reference in &references {
//         println!(
//             "Found ref: {reference}",
//             reference = reference.display(stack_graph)
//         );
//     }
//     ForwardPartialPathStitcher::find_all_complete_partial_paths(
//         &mut GraphEdgeCandidates::new(stack_graph, &mut paths, None),
//         references,
//         StitcherConfig::default(),
//         &stack_graphs::NoCancellation,
//         |graph, paths, path| {
//             results.insert(path.display(graph, paths).to_string());
//         },
//     )
//     .expect("should never be cancelled");

//     for result in &results {
//         println!("Found path: {result}");
//     }
// }
