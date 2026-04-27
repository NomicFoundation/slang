use std::fmt;

use metaslang_graph_builder::graph::{Graph, GraphNodeRef};
use slang_solidity::cst::KindTypes;

use super::super::runner::ParsedPart;

const VARIABLE_DEBUG_ATTR: &str = "debug_msgb_variable";
const LOCATION_DEBUG_ATTR: &str = "debug_msgb_location";

pub(crate) fn render(parsed_parts: &[ParsedPart<'_>]) -> String {
    let mut result = Vec::new();
    result.push("digraph {".to_string());

    // special nodes, outside of any source part
    result.push("ROOT_NODE".to_string());
    result.push(
        "JUMP_TO_SCOPE_NODE [label = \"\", shape = circle, style = filled, color = purple]"
            .to_string(),
    );

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

fn special_node(node: GraphNodeRef) -> bool {
    node.index() <= 1
}

impl DotSubGraph<'_> {
    fn node_id(&self, node: GraphNodeRef) -> String {
        let index = node.index();
        if index == 0 {
            // special case: ROOT_NODE
            "ROOT_NODE".to_string()
        } else if index == 1 {
            // special case: JUMP_TO_SCOPE_NODE
            "JUMP_TO_SCOPE_NODE".to_string()
        } else {
            format!("{graph_id}N{index}", graph_id = self.graph_id,)
        }
    }

    fn node_label(&self, node: GraphNodeRef, node_type: Option<&str>) -> String {
        let graph_node = &self.graph[node];
        let mut node_label = if let Some(symbol) = graph_node.attributes.get("symbol") {
            format!("{symbol} [{index}]", index = node.index())
        } else if let Some(variable) = graph_node.attributes.get(VARIABLE_DEBUG_ATTR) {
            format!("{variable} [{index}]", index = node.index())
        } else {
            format!("[{index}]", index = node.index())
        };
        match node_type {
            Some("push_scoped_symbol") => {
                node_label += " \u{25ef}";
            }
            Some("pop_scoped_symbol") => {
                node_label += " \u{2b24}";
            }
            _ => {
                if graph_node.attributes.get("is_exported").is_some() {
                    // shorten the label to fit in a circle
                    if let Some(dot_index) = node_label.find('.') {
                        node_label = String::from(&node_label[dot_index + 1..dot_index + 4]);
                    } else if node_label.len() > 3 {
                        node_label = String::from(&node_label[..3]);
                    }
                }
            }
        }
        node_label
    }
}

impl fmt::Display for DotSubGraph<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // we need to print edges connecting to the special nodes outside of the subgraph
        for node in self.graph.iter_nodes() {
            let graph_node = &self.graph[node];
            let node_id = self.node_id(node);

            for (sink, _edge) in graph_node.iter_edges() {
                if special_node(node) || special_node(sink) {
                    writeln!(f, "{node_id} -> {sink_id}", sink_id = self.node_id(sink))?;
                }
            }
        }

        writeln!(f)?;
        writeln!(
            f,
            "subgraph cluster_{graph_id} {{",
            graph_id = self.graph_id
        )?;
        writeln!(f, "label = \"{title}\"", title = self.title)?;

        for node in self.graph.iter_nodes() {
            if special_node(node) {
                // we already rendered the special nodes all its edges
                continue;
            }

            let graph_node = &self.graph[node];
            let node_type = graph_node
                .attributes
                .get("type")
                .and_then(|x| x.as_str().ok());
            let node_label = self.node_label(node, node_type);
            let node_id = self.node_id(node);

            if let Some(location) = graph_node.attributes.get(LOCATION_DEBUG_ATTR) {
                writeln!(f, "\t// {location}")?;
            }
            match node_type {
                Some("push_symbol" | "push_scoped_symbol") => {
                    let extra_attrs = if graph_node.attributes.get("is_reference").is_some() {
                        ", penwidth = 2, color = limegreen, fontcolor = limegreen"
                    } else {
                        ", color = lightgreen, fontcolor = lightgreen, style = dashed"
                    };
                    writeln!(
                        f,
                        "\t{node_id} [label = \"{node_label}\", shape = invhouse{extra_attrs}]"
                    )?;
                    if let Some(scope) = graph_node
                        .attributes
                        .get("scope")
                        .and_then(|scope| scope.as_graph_node_ref().ok())
                    {
                        writeln!(
                            f,
                            "\t{node_id} -> {scope_id} [style = dashed]",
                            scope_id = self.node_id(scope)
                        )?;
                    }
                }
                Some("pop_symbol" | "pop_scoped_symbol") => {
                    let extra_attrs = if graph_node.attributes.get("is_definition").is_some() {
                        ", penwidth = 2, color = red, fontcolor = red"
                    } else {
                        ", color = coral, fontcolor = coral, style = dashed"
                    };
                    writeln!(
                        f,
                        "\t{node_id} [label = \"{node_label}\", shape = house{extra_attrs}]"
                    )?;
                }
                Some("drop_scopes") => {
                    writeln!(f, "\t{node_id} [label = \"DROP\", shape = box]")?;
                }
                _ => {
                    let extra_attrs = if graph_node.attributes.get("is_exported").is_some() {
                        ", shape = circle, width = 0.5, penwidth = 2, fixedsize = true, color = purple"
                    } else {
                        ""
                    };
                    writeln!(f, "\t{node_id} [label = \"{node_label}\"{extra_attrs}]")?;
                }
            }

            for (sink, edge) in graph_node.iter_edges() {
                if special_node(sink) {
                    // we already rendered the edges going to special nodes
                    continue;
                }

                if let Some(location) = edge.attributes.get(LOCATION_DEBUG_ATTR) {
                    writeln!(f, "\t// {location}")?;
                }
                writeln!(f, "\t{node_id} -> {sink_id}", sink_id = self.node_id(sink))?;
            }
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}
