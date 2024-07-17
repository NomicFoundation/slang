use std::fmt;
use std::ops::Range;
use std::path::Path;

use anyhow::Result;
use ariadne::{Color, Config, Label, Report, ReportBuilder, ReportKind, Source};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use metaslang_bindings::builder;
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::graph::{Graph, Value};
use metaslang_graph_builder::{ExecutionConfig, NoCancellation, Variables};
use semver::Version;
use slang_solidity::bindings::{self, Handle};
use slang_solidity::cst::KindTypes;
use slang_solidity::language::Language;
use slang_solidity::parse_output::ParseOutput;

use crate::generated::VERSION_BREAKS;

pub fn run(group_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("bindings_output")
        .join(group_name)
        .join(test_name);

    let mut fs = CodegenFileSystem::new(&test_dir)?;

    let input_path = test_dir.join("input.sol");

    let source = input_path.read_to_string()?;

    let mut last_graph_output = None;
    let mut last_bindings_output = None;

    for version in &VERSION_BREAKS {
        let language = Language::new(version.clone())?;

        let parse_output = language.parse(Language::ROOT_KIND, &source);

        if !GitHub::is_running_in_ci() {
            // Don't run this in CI, since the graph outputs are not committed
            // to the repository and hence we cannot verify their contents,
            // which is what `fs.write_file` does in CI.
            let graph_output = output_graph(version, &parse_output)?;
            match last_graph_output {
                Some(ref last) if last == &graph_output => (),
                _ => {
                    let snapshot_path = test_dir.join("generated").join(format!("{version}.mmd"));

                    fs.write_file(snapshot_path, &graph_output)?;
                    last_graph_output = Some(graph_output);
                }
            };
        }

        let bindings_output = output_bindings(version, &parse_output, &source, &input_path)?;
        match last_bindings_output {
            Some(ref last) if last == &bindings_output => (),
            _ => {
                let snapshot_path = test_dir.join("generated").join(format!("{version}.txt"));

                fs.write_file(snapshot_path, &bindings_output)?;
                last_bindings_output = Some(bindings_output);
            }
        }
    }

    Ok(())
}

const ROOT_NODE_VAR: &str = "ROOT_NODE";
const VERSION_VAR: &str = "VERSION";

const VARIABLE_DEBUG_ATTR: &str = "__variable";
const LOCATION_DEBUG_ATTR: &str = "__location";
const MATCH_DEBUG_ATTR: &str = "__match";

fn output_graph(version: &Version, parse_output: &ParseOutput) -> Result<String> {
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
    variables.add(VERSION_VAR.into(), version.to_string().into())?;
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

fn output_bindings(
    version: &Version,
    parse_output: &ParseOutput,
    source: &str,
    source_path: &Path,
) -> Result<String> {
    let mut bindings = bindings::create(version.clone());
    bindings.add_file(
        source_path.to_str().unwrap(),
        parse_output.create_tree_cursor(),
    );

    let source_id = source_path.strip_repo_root()?.unwrap_str();
    let mut builder: ReportBuilder<'_, (&str, Range<usize>)> = Report::build(
        ReportKind::Custom("References and definitions", Color::Unset),
        source_id,
        0,
    )
    .with_config(Config::default().with_color(false));

    if !parse_output.is_valid() {
        builder = builder.with_note("WARNING: Parsing failed. Results may be incomplete.");
    }

    let mut definitions: Vec<Handle<'_>> = Vec::new();

    for definition in bindings.all_definitions() {
        let Some(cursor) = definition.get_cursor() else {
            continue;
        };

        let range = {
            let range = cursor.text_range();
            let start = source[..range.start.utf8].chars().count();
            let end = source[..range.end.utf8].chars().count();
            start..end
        };

        definitions.push(definition);
        let message = format!("def: {}", definitions.len());
        builder = builder.with_label(Label::new((source_id, range)).with_message(message));
    }

    for reference in bindings.all_references() {
        let Some(cursor) = reference.get_cursor() else {
            continue;
        };

        let range = {
            let range = cursor.text_range();
            let start = source[..range.start.utf8].chars().count();
            let end = source[..range.end.utf8].chars().count();
            start..end
        };

        let definition = reference.jump_to_definition();
        let message = match definition {
            None => "unresolved".to_string(),
            Some(definition) => {
                let def_id = definitions.iter().position(|d| *d == definition).unwrap();
                format!("ref: {}", def_id + 1)
            }
        };

        builder = builder.with_label(Label::new((source_id, range)).with_message(message));
    }

    let report = builder.finish();
    let mut buffer = Vec::new();
    report.write((source_id, Source::from(source)), &mut buffer)?;

    let result = String::from_utf8(buffer)?;
    Ok(result)
}
