use std::fmt;

use anyhow::Result;
use metaslang_bindings::builder::{self, ROOT_NODE_VAR};
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::graph::{Graph, Value};
use metaslang_graph_builder::{ExecutionConfig, NoCancellation, Variables};
use semver::Version;
use slang_solidity::bindings;
use slang_solidity::cst::KindTypes;
use slang_solidity::parse_output::ParseOutput;

const VARIABLE_DEBUG_ATTR: &str = "__variable";
const LOCATION_DEBUG_ATTR: &str = "__location";
const MATCH_DEBUG_ATTR: &str = "__match";

pub(crate) fn render_graph(version: &Version, parse_output: &ParseOutput) -> Result<String> {
    let graph_builder = File::from_str(bindings::get_binding_rules())?;

    let tree = parse_output.create_tree_cursor();
    let mut graph = Graph::new();
    let root_node = graph.add_graph_node();
    graph[root_node]
        .attributes
        .add(VARIABLE_DEBUG_ATTR.into(), ROOT_NODE_VAR.to_string())
        .unwrap();

    let functions = builder::default_functions(version.clone());
    let mut variables = Variables::new();
    variables.add(ROOT_NODE_VAR.into(), root_node.into())?;
    let execution_config = ExecutionConfig::new(&functions, &variables).debug_attributes(
        LOCATION_DEBUG_ATTR.into(),
        VARIABLE_DEBUG_ATTR.into(),
        MATCH_DEBUG_ATTR.into(),
    );

    graph_builder.execute_into(&mut graph, &tree, &execution_config, &NoCancellation)?;

    let note = if parse_output.is_valid() {
        ""
    } else {
        "%% WARNING: Parsing failed, graph may be incomplete\n"
    };
    Ok(format!(
        "{note}{graph}",
        graph = print_graph_as_mermaid(&graph)
    ))
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
