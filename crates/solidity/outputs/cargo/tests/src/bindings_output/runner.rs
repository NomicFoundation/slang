use std::sync::Arc;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use metaslang_graph_builder::graph::Graph;
use slang_solidity::bindings;
use slang_solidity::cst::KindTypes;
use slang_solidity::parser::{ParseOutput, Parser};

use super::graph::{render_dot_graph, render_graph};
use super::renderer::render_bindings;
use crate::generated::VERSION_BREAKS;
use crate::multi_part_file::{split_multi_file, Part};
use crate::resolver::TestsPathResolver;

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

    let mut fs = CodegenFileSystem::new(&test_dir)?;

    let input_path = test_dir.join("input.sol");
    let contents = input_path.read_to_string()?;

    let mut last_graph_output = None;
    let mut last_bindings_output = None;

    for version in &VERSION_BREAKS {
        let parser = Parser::new(version.clone())?;
        let mut bindings =
            bindings::create_with_resolver(version.clone(), Arc::new(TestsPathResolver {}));
        let mut parsed_parts: Vec<ParsedPart<'_>> = Vec::new();

        let multi_part = split_multi_file(&contents);
        for Part {
            name: path,
            contents,
        } in &multi_part.parts
        {
            let parse_output = parser.parse(Parser::ROOT_KIND, contents);
            let graph = bindings.add_file_returning_graph(path, parse_output.create_tree_cursor());
            parsed_parts.push(ParsedPart {
                path,
                contents,
                parse_output,
                graph,
            });
        }
        let parse_success = parsed_parts.iter().all(|part| part.parse_output.is_valid());
        let parse_status = if parse_success { "success" } else { "failure" };

        if let Some(context) = multi_part.context {
            let context_definition = bindings
                .lookup_definition_by_name(context)
                .expect("context definition to be found")
                .to_handle();
            bindings.set_context(&context_definition);
        }

        if !GitHub::is_running_in_ci() {
            // Don't run this in CI, since the graph outputs are not committed
            // to the repository and hence we cannot verify their contents,
            // which is what `fs.write_file` does in CI.
            let graph_output = render_graph(&parsed_parts);
            match last_graph_output {
                Some(ref last) if last == &graph_output => (),
                _ => {
                    let snapshot_path = test_dir
                        .join("generated")
                        .join(format!("{version}-{parse_status}.mmd"));

                    fs.write_file(snapshot_path, &graph_output)?;
                    last_graph_output = Some(graph_output);

                    let dot_output = render_dot_graph(&parsed_parts);
                    let dot_output_path = test_dir
                        .join("generated")
                        .join(format!("{version}-{parse_status}.dot"));
                    fs.write_file(dot_output_path, &dot_output)?;
                }
            };
        }

        let bindings_output = render_bindings(&bindings, &parsed_parts)?;
        match last_bindings_output {
            Some(ref last) if last == &bindings_output => (),
            _ => {
                let snapshot_path = test_dir
                    .join("generated")
                    .join(format!("{version}-{parse_status}.txt"));

                fs.write_file(snapshot_path, &bindings_output)?;
                last_bindings_output = Some(bindings_output);
            }
        }
    }

    Ok(())
}
