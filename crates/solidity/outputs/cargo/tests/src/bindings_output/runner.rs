use std::fs;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use metaslang_graph_builder::graph::Graph;
use slang_solidity::cst::KindTypes;
use slang_solidity::parser::{ParseOutput, Parser};

use super::graph::graphviz::render as render_graphviz_graph;
use super::graph::mermaid::render as render_mermaid_graph;
use super::renderer::render_bindings;
use crate::bindings::create_bindings;
use crate::generated::VERSION_BREAKS;
use crate::multi_part_file::{split_multi_file, Part};

pub(crate) struct ParsedPart<'a> {
    pub path: &'a str,
    pub contents: &'a str,
    pub parse_output: ParseOutput,
    pub graph: Graph<KindTypes>,
}

pub fn run(group_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("bindings_output")
        .join(group_name)
        .join(test_name);
    let target_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("target/bindings_output")
        .join(group_name)
        .join(test_name);

    let mut fs = CodegenFileSystem::new(&test_dir)?;

    let input_path = test_dir.join("input.sol");
    let contents = input_path.read_to_string()?;

    let mut last_graph_output = None;
    let mut last_bindings_output = None;

    for version in &VERSION_BREAKS {
        let parser = Parser::create(version.clone())?;
        let mut bindings = create_bindings(version)?;

        let mut parsed_parts: Vec<ParsedPart<'_>> = Vec::new();
        let multi_part = split_multi_file(&contents);
        for Part {
            name: path,
            contents,
        } in &multi_part.parts
        {
            let parse_output = parser.parse(Parser::ROOT_KIND, contents);
            let graph =
                bindings.add_user_file_returning_graph(path, parse_output.create_tree_cursor());
            parsed_parts.push(ParsedPart {
                path,
                contents,
                parse_output,
                graph,
            });
        }

        if let Some(context) = multi_part.context {
            let context_definition = bindings
                .lookup_definition_by_name(context)
                .expect("context definition to be found")
                .to_handle();
            bindings.set_context(&context_definition);
        }

        let (bindings_output, all_resolved) = render_bindings(&bindings, &parsed_parts)?;

        let parse_success = parsed_parts.iter().all(|part| part.parse_output.is_valid());
        let status = if parse_success && all_resolved {
            "success"
        } else {
            "failure"
        };

        // Render graph outputs unless we're on CI and only if the RENDER_GRAPHS
        // environment variable is set.
        if !GitHub::is_running_in_ci() && std::env::var("RENDER_GRAPHS").is_ok() {
            fs::create_dir_all(&target_dir)?;

            let graph_output = render_mermaid_graph(&parsed_parts);
            match last_graph_output {
                Some(ref last) if last == &graph_output => (),
                _ => {
                    let snapshot_path = target_dir.join(format!("{version}-{status}.mmd"));

                    fs::write(snapshot_path, &graph_output)?;
                    last_graph_output = Some(graph_output);

                    let dot_output = render_graphviz_graph(&parsed_parts);
                    let dot_output_path = target_dir.join(format!("{version}-{status}.dot"));
                    fs::write(dot_output_path, &dot_output)?;
                }
            };
        }

        match last_bindings_output {
            Some(ref last) if last == &bindings_output => (),
            _ => {
                let snapshot_path = test_dir
                    .join("generated")
                    .join(format!("{version}-{status}.txt"));

                fs.write_file(snapshot_path, &bindings_output)?;
                last_bindings_output = Some(bindings_output);
            }
        }
    }

    Ok(())
}
