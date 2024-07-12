use std::fmt::{self, Display};
use std::fs;
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use infra_utils::paths::PathExtensions;
use semver::Version;
use slang_solidity::bindings::{self, Handle};
use slang_solidity::language::Language;
use slang_solidity::resolver::SolidityPathResolver;
use stack_graphs::graph::{Node, PopSymbolNode, PushSymbolNode, StackGraph};

use crate::multi_part_file::split_multi_file;

#[test]
fn test_bindings_are_resolved_across_imported_files() -> Result<()> {
    let version = Version::parse("0.8.22")?;
    let language = Language::new(version.clone())?;

    let input_path = Path::repo_path("crates/solidity/outputs/cargo/tests/fixtures/input.sol");
    let contents = fs::read_to_string(input_path)?;
    let parts = split_multi_file(&contents);
    assert_eq!(3, parts.len());

    let mut bindings =
        bindings::create_with_resolver(version.clone(), Arc::new(SolidityPathResolver {}));
    for (path, contents) in &parts {
        let parsed = language.parse(Language::ROOT_KIND, contents);
        bindings.add_file(path, parsed.create_tree_cursor());
    }

    let mermaid_path =
        Path::repo_path("crates/solidity/outputs/cargo/tests/fixtures/stackgraph.mmd");
    let mermaid_contents = print_stack_graph_as_mermaid(&bindings.stack_graph);
    fs::write(mermaid_path, mermaid_contents.to_string())?;

    for reference in bindings.all_references() {
        let definition = reference.jump_to_definition();
        assert!(
            definition.is_some(),
            "expected all references to be resolved, but {} failed",
            DisplayHandle(reference),
        );

        let definition = definition.unwrap();
        println!(
            "{reference} ----> {definition}",
            reference = DisplayHandle(reference),
            definition = DisplayHandle(definition),
        );
    }

    Ok(())
}

struct DisplayHandle<'a>(Handle<'a>);

impl<'a> Display for DisplayHandle<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.0.get_file().unwrap();
        let cursor = self.0.get_cursor().unwrap();
        let offset = cursor.text_offset();

        write!(
            f,
            "{file}:{line}:{column} -> {text}",
            line = offset.line + 1,
            column = offset.column,
            text = cursor.node().unparse(),
        )
    }
}

fn print_stack_graph_as_mermaid(stack_graph: &StackGraph) -> impl fmt::Display + '_ {
    struct DisplayStackGraph<'a>(&'a StackGraph);

    impl<'a> fmt::Display for DisplayStackGraph<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let stack_graph = self.0;
            writeln!(f, "graph TD")?;
            for node_handle in stack_graph.iter_nodes() {
                match &stack_graph[node_handle] {
                    Node::PopSymbol(PopSymbolNode { symbol, .. }) => {
                        writeln!(
                            f,
                            "\tN{}[\\{}/]",
                            node_handle.as_u32(),
                            &stack_graph[*symbol]
                        )?;
                    }
                    Node::PushSymbol(PushSymbolNode { symbol, .. }) => {
                        writeln!(
                            f,
                            "\tN{}[/{}\\]",
                            node_handle.as_u32(),
                            &stack_graph[*symbol]
                        )?;
                    }
                    Node::Root(_) => {
                        writeln!(f, "\tN{}[ROOT]", node_handle.as_u32())?;
                    }
                    _ => {
                        writeln!(f, "\tN{}", node_handle.as_u32())?;
                    }
                }

                for edge_handle in stack_graph.outgoing_edges(node_handle) {
                    writeln!(
                        f,
                        "\tN{} --> N{}",
                        edge_handle.source.as_u32(),
                        edge_handle.sink.as_u32()
                    )?;
                }
            }
            Ok(())
        }
    }

    DisplayStackGraph(stack_graph)
}
