use std::fmt;
use std::fs::{self, create_dir_all};
use std::path::PathBuf;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::functions::Functions;
use metaslang_graph_builder::graph::{Graph, Value};
use metaslang_graph_builder::{ExecutionConfig, NoCancellation, Variables};
use slang_solidity::assertions::{check_assertions, collect_assertions};
use slang_solidity::bindings;
use slang_solidity::cst::KindTypes;
use slang_solidity::language::Language;
use slang_solidity::parse_output::ParseOutput;

pub fn run(group_name: &str, file_name: &str) -> Result<()> {
    let data_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("bindings")
        .join(group_name);
    let input_path = data_dir.join(file_name);
    let input = fs::read_to_string(&input_path)?;

    // TODO: de-hardcode this and parse with different versions?
    let version = Language::SUPPORTED_VERSIONS.last().unwrap();
    let language = Language::new(version.clone())?;

    let parse_output = language.parse(Language::ROOT_KIND, &input);
    assert!(parse_output.is_valid());

    let output_dir = data_dir.join("generated");
    let output_path = output_dir.join(format!("{file_name}.mmd"));
    create_dir_all(&output_dir)?;
    output_graph(&parse_output, &output_path)?;

    let mut bindings = bindings::create(version.clone());
    bindings.add_file(
        input_path.to_str().unwrap(),
        parse_output.create_tree_cursor(),
    );

    let assertions = collect_assertions(parse_output.create_tree_cursor())?;
    check_assertions(&bindings, &assertions)?;

    Ok(())
}

fn output_graph(parse_output: &ParseOutput, output_path: &PathBuf) -> Result<()> {
    let graph_builder = File::from_str(bindings::get_binding_rules())?;

    let tree = parse_output.create_tree_cursor();
    let mut graph = Graph::new();
    let root_node = graph.add_graph_node();
    let variable_name = "ROOT_NODE".to_string();
    graph[root_node]
        .attributes
        .add("__variable".into(), variable_name)
        .unwrap();

    let functions = Functions::stdlib();
    let mut variables = Variables::new();
    variables.add("ROOT_NODE".into(), root_node.into())?;
    let execution_config = ExecutionConfig::new(&functions, &variables).debug_attributes(
        "__location".into(),
        "__variable".into(),
        "__match".into(),
    );

    graph_builder.execute_into(&mut graph, &tree, &execution_config, &NoCancellation)?;

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
                    .and_then(|source| source.as_syntax_node_ref().ok());
                let location = gn.attributes.get("__location");

                let node_label = format!(
                    "\"`**{node_label}** @{source}\n{variable}\n{location}`\"",
                    source = source.map(|s| s.location()).unwrap_or_default(),
                    variable = gn.attributes.get("__variable").unwrap_or(&Value::Null),
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
