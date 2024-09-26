use std::fmt;

use metaslang_graph_builder::graph::{Graph, GraphNodeRef};
use slang_solidity::cst::KindTypes;

use super::super::runner::ParsedPart;

const VARIABLE_DEBUG_ATTR: &str = "debug_msgb_variable";

pub(crate) fn render(parsed_parts: &[ParsedPart<'_>]) -> String {
    let mut result = Vec::new();
    result.push("digraph {".to_string());
    result.push("ROOT_NODE".to_string());

    for (index, part) in parsed_parts.iter().enumerate() {
        let title = if part.parse_output.is_valid() {
            part.path.to_string()
        } else {
            format!("PARSING FAILED: {}", part.path)
        };

        let graph_id = format!("G{index}");
        result.push(
            DotSubGraph {
                graph_id,
                graph: &part.graph,
                title,
            }
            .to_string(),
        );
    }
    result.push("}".to_string());

    result.join("\n")
}

struct DotSubGraph<'a> {
    graph_id: String,
    graph: &'a Graph<KindTypes>,
    title: String,
}

impl<'a> DotSubGraph<'a> {
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

impl<'a> fmt::Display for DotSubGraph<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root = self.root_node();

        // we need to print edges connecting to the ROOT NODE outside of the subgraph
        for node in self.graph.iter_nodes() {
            let graph_node = &self.graph[node];
            let node_id = self.node_id(node);

            for (sink, _edge) in graph_node.iter_edges() {
                if node == root || sink == root {
                    writeln!(f, "{node_id} -> {sink_id}", sink_id = self.node_id(sink))?;
                }
            }
        }

        writeln!(f)?;
        writeln!(
            f,
            "subgraph cluster_{graph_id} {{\nlabel = \"{title}\"",
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
            } else if let Some(variable) = graph_node.attributes.get(VARIABLE_DEBUG_ATTR) {
                variable.to_string()
            } else {
                format!("{}", node.index())
            };

            let node_label = format!("\"{node_label}\"");
            let node_type = graph_node
                .attributes
                .get("type")
                .and_then(|x| x.as_str().ok());
            let node_id = self.node_id(node);

            match node_type {
                Some("push_symbol") => {
                    let extra_attrs = if graph_node.attributes.get("is_reference").is_some() {
                        ", penwidth = 2, color = \"limegreen\", fontcolor = \"limegreen\""
                    } else {
                        ", color = \"lightgreen\", fontcolor = \"lightgreen\""
                    };
                    writeln!(
                        f,
                        "\t{node_id} [label = {node_label}, shape = \"invhouse\"{extra_attrs}]"
                    )?;
                }
                Some("pop_symbol") => {
                    let extra_attrs = if graph_node.attributes.get("is_definition").is_some() {
                        ", penwidth = 2, color = \"red\", fontcolor = \"red\""
                    } else {
                        ", color = \"coral\", fontcolor = \"coral\""
                    };
                    writeln!(
                        f,
                        "\t{node_id} [label = {node_label}, shape = \"house\"{extra_attrs}]"
                    )?;
                }
                _ => writeln!(f, "\t{node_id} [label = {node_label}]")?,
            }

            for (sink, _edge) in graph_node.iter_edges() {
                if sink == root {
                    // we already rendered the edges going to ROOT NODE
                    continue;
                }
                writeln!(f, "\t{node_id} -> {sink_id}", sink_id = self.node_id(sink))?;
            }
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}
