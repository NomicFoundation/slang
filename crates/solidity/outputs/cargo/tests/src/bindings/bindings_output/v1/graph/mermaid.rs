use std::fmt;

use metaslang_graph_builder::graph::{Graph, GraphNodeRef, Value};
use slang_solidity::cst::KindTypes;

use super::super::runner::ParsedPart;

const VARIABLE_DEBUG_ATTR: &str = "debug_msgb_variable";
const LOCATION_DEBUG_ATTR: &str = "debug_msgb_location";
const MATCH_DEBUG_ATTR: &str = "debug_msgb_match_node";

pub(crate) fn render(parsed_parts: &[ParsedPart<'_>]) -> String {
    let mut result = Vec::new();
    result.push("graph TD".to_string());
    result.push("ROOT_NODE((ROOT NODE))".to_string());

    for (index, part) in parsed_parts.iter().enumerate() {
        let title = if part.parse_output.is_valid() {
            part.path.to_string()
        } else {
            format!("PARSING FAILED: {}", part.path)
        };

        let graph_id = format!("G{index}");
        result.push(
            MermaidSubGraph {
                graph_id,
                graph: &part.graph,
                title,
            }
            .to_string(),
        );
    }

    result.join("\n")
}

struct MermaidSubGraph<'a> {
    graph_id: String,
    graph: &'a Graph<KindTypes>,
    title: String,
}

impl MermaidSubGraph<'_> {
    fn root_node(&self) -> GraphNodeRef {
        self.graph
            .iter_nodes()
            .next()
            .expect("graph should have at least the root node")
    }

    fn node_id(&self, node: GraphNodeRef) -> String {
        if node == self.root_node() {
            // special case: ROOT_NODE
            "ROOT_NODE".to_string()
        } else {
            format!(
                "{graph_id}N{index}",
                graph_id = self.graph_id,
                index = node.index()
            )
        }
    }
}

impl fmt::Display for MermaidSubGraph<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root = self.root_node();

        // we need to print edges connecting to the ROOT NODE outside of the subgraph
        for node in self.graph.iter_nodes() {
            let graph_node = &self.graph[node];
            let node_id = self.node_id(node);

            for (sink, _edge) in graph_node.iter_edges() {
                if node == root || sink == root {
                    writeln!(f, "{node_id} --> {sink_id}", sink_id = self.node_id(sink))?;
                }
            }
        }

        writeln!(f)?;
        writeln!(
            f,
            "subgraph {graph_id} [{title}]",
            graph_id = self.graph_id,
            title = self.title
        )?;

        for node in self.graph.iter_nodes() {
            if node == root {
                // we already rendered the ROOT NODE and all its edges
                continue;
            }

            let graph_node = &self.graph[node];
            let node_label = if let Some(symbol) = graph_node.attributes.get("symbol") {
                symbol.to_string()
            } else {
                format!("{}", node.index())
            };
            let source = graph_node
                .attributes
                .get(MATCH_DEBUG_ATTR)
                .and_then(|source| source.as_syntax_node_ref().ok());
            let location = graph_node.attributes.get(LOCATION_DEBUG_ATTR);

            let node_label = format!(
                "\"`**{node_label}** @{source}\n{variable}\n{location}`\"",
                source = source.map(|s| s.location()).unwrap_or_default(),
                variable = graph_node
                    .attributes
                    .get(VARIABLE_DEBUG_ATTR)
                    .unwrap_or(&Value::Null),
                location = location.unwrap_or(&Value::Null),
            );
            let node_type = graph_node
                .attributes
                .get("type")
                .and_then(|x| x.as_str().ok());
            let node_id = self.node_id(node);

            match node_type {
                Some("push_symbol") => writeln!(f, "\t{node_id}[/{node_label}\\]")?,
                Some("pop_symbol") => writeln!(f, "\t{node_id}[\\{node_label}/]")?,
                _ => writeln!(f, "\t{node_id}[{node_label}]")?,
            }

            for (sink, _edge) in graph_node.iter_edges() {
                if sink == root {
                    // we already rendered the edges going to ROOT NODE
                    continue;
                }
                writeln!(f, "\t{node_id} --> {sink_id}", sink_id = self.node_id(sink))?;
            }
        }

        writeln!(f, "end")?;
        Ok(())
    }
}
