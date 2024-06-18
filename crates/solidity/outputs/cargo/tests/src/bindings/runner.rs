use std::fmt;
use std::fs::{self, create_dir_all};

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::paths::FileWalker;
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::functions::Functions;
use metaslang_graph_builder::graph::Graph;
use metaslang_graph_builder::{ExecutionConfig, NoCancellation, Variables};
use slang_solidity::bindings;
use slang_solidity::cst::KindTypes;
use slang_solidity::language::Language;

#[test]
pub fn run_all() -> Result<()> {
    let data_dir =
        CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?.join("bindings");

    for file in FileWalker::from_directory(data_dir).find(["*.sol"])? {
        run(file.file_name().unwrap().to_str().unwrap())?;
    }

    Ok(())
}

fn run(file_name: &str) -> Result<()> {
    let data_dir =
        CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?.join("bindings");
    let input_path = data_dir.join(file_name);
    let input = fs::read_to_string(input_path)?;

    // TODO: de-hardcode this and parse with different versions?
    let latest_version = Language::SUPPORTED_VERSIONS.last().unwrap();
    let language = Language::new(latest_version.clone())?;

    let parse_output = language.parse(Language::ROOT_KIND, &input);
    assert!(parse_output.is_valid());

    let graph_builder = File::from_str(bindings::get_binding_rules())?;

    let functions = Functions::stdlib();
    let variables = Variables::new();
    let execution_config = ExecutionConfig::new(&functions, &variables).debug_attributes(
        "__location".into(),
        "__variable".into(),
        "__match".into(),
    );

    let tree = parse_output.create_tree_cursor();
    let graph = graph_builder.execute(&tree, &execution_config, &NoCancellation)?;

    let output_dir = data_dir.join("generated");
    create_dir_all(&output_dir)?;

    let output_path = output_dir.join(format!("{file_name}.mmd"));
    fs::write(output_path, format!("{}", print_graph_as_mermaid(&graph)))?;

    Ok(())
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
                    .get("__match")
                    .unwrap()
                    .as_syntax_node_ref()
                    .unwrap();
                let location = gn.attributes.get("__location").unwrap();

                let node_label = format!(
                    "\"`**{node_label}** @{source}\n{variable}\n{location}`\"",
                    source = source.location(),
                    variable = gn.attributes.get("__variable").unwrap(),
                    location = location,
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
