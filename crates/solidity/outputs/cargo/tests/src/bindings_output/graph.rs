use std::fmt;
use std::path::Path;

use metaslang_graph_builder::graph::{Graph, Value};
use semver::Version;
use slang_solidity::bindings;
use slang_solidity::cst::KindTypes;
use slang_solidity::parse_output::ParseOutput;

const VARIABLE_DEBUG_ATTR: &str = "debug_msgb_variable";
const LOCATION_DEBUG_ATTR: &str = "debug_msgb_location";
const MATCH_DEBUG_ATTR: &str = "debug_msgb_match_node";

pub(crate) fn render_graph(
    version: &Version,
    parse_output: &ParseOutput,
    source_path: &Path,
) -> String {
    let mut bindings = bindings::create(version.clone());
    let tree = parse_output.create_tree_cursor();
    let graph = bindings.add_file_returning_graph(source_path.to_str().unwrap(), tree);

    let note = if parse_output.is_valid() {
        ""
    } else {
        "%% WARNING: Parsing failed, graph may be incomplete\n"
    };
    format!("{note}{graph}", graph = print_graph_as_mermaid(&graph))
}

fn print_graph_as_mermaid(graph: &Graph<KindTypes>) -> impl fmt::Display + '_ {
    struct DisplayGraph<'a>(&'a Graph<KindTypes>);

    impl<'a> fmt::Display for DisplayGraph<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let graph = self.0;
            writeln!(f, "graph TD")?;
            for node in graph.iter_nodes() {
                let gn = &graph[node];
                let node_label = if let Some(symbol) = gn.attributes.get("symbol") {
                    symbol.to_string()
                } else {
                    format!("{}", node.index())
                };
                let source = gn
                    .attributes
                    .get(MATCH_DEBUG_ATTR)
                    .and_then(|source| source.as_syntax_node_ref().ok());
                let location = gn.attributes.get(LOCATION_DEBUG_ATTR);

                let node_label = format!(
                    "\"`**{node_label}** @{source}\n{variable}\n{location}`\"",
                    source = source.map(|s| s.location()).unwrap_or_default(),
                    variable = gn
                        .attributes
                        .get(VARIABLE_DEBUG_ATTR)
                        .unwrap_or(&Value::Null),
                    location = location.unwrap_or(&Value::Null),
                );
                let node_type = gn.attributes.get("type").and_then(|x| x.as_str().ok());
                match node_type {
                    Some("push_symbol") => writeln!(f, "\tN{}[/{}\\]", node.index(), node_label)?,
                    Some("pop_symbol") => writeln!(f, "\tN{}[\\{}/]", node.index(), node_label)?,
                    _ => writeln!(f, "\tN{}[{}]", node.index(), node_label)?,
                }
                for (sink, _edge) in gn.iter_edges() {
                    writeln!(f, "\tN{} --> N{}", node.index(), sink.index())?;
                }
            }
            Ok(())
        }
    }

    DisplayGraph(graph)
}
