use std::fs;
use std::rc::Rc;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use metaslang_graph_builder::graph::Graph;
use slang_solidity::bindings;
use slang_solidity::cst::KindTypes;
use slang_solidity::parser::{ParseOutput, Parser};

use super::graph::graphviz::render as render_graphviz_graph;
use super::graph::mermaid::render as render_mermaid_graph;
use super::renderer::render_bindings;
use crate::compilation::resolver::TestsPathResolver;
use crate::cst::generated::VERSION_BREAKS;
use crate::utils::multi_part_file::{split_multi_file, Part};

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

    let mut fs = CodegenFileSystem::default();

    let input_path = test_dir.join("input.sol");
    let contents = input_path.read_to_string()?;

    let mut last_graph_output = None;
    let mut last_bindings_output = None;

    for version in &VERSION_BREAKS {
        let parser = Parser::create(version.clone())?;
        let mut builder =
            bindings::create_with_resolver(version.clone(), Rc::new(TestsPathResolver {}));

        let mut parsed_parts: Vec<ParsedPart<'_>> = Vec::new();
        let multi_part = split_multi_file(&contents);
        for Part {
            name: path,
            contents,
        } in &multi_part.parts
        {
            let parse_output = parser.parse_file_contents(contents);
            let graph =
                builder.add_user_file_returning_graph(path, parse_output.create_tree_cursor());
            parsed_parts.push(ParsedPart {
                path,
                contents,
                parse_output,
                graph,
            });
        }

        let binding_graph = builder.build();
        let (bindings_output, all_resolved) = render_bindings(&binding_graph, &parsed_parts)?;

        let parse_success = parsed_parts.iter().all(|part| part.parse_output.is_valid());
        let status = if parse_success && all_resolved {
            "success"
        } else {
            "failure"
        };

        // Render graph outputs only if the __SLANG_BINDINGS_RENDER_GRAPHS__
        // environment variable is set.
        if std::env::var("__SLANG_BINDINGS_RENDER_GRAPHS__").is_ok() {
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
            }
        }

        match last_bindings_output {
            Some(ref last) if last == &bindings_output => (),
            _ => {
                let snapshot_path = test_dir
                    .join("generated")
                    .join(format!("{version}-{status}.txt"));

                fs.write_file_raw(snapshot_path, &bindings_output)?;
                last_bindings_output = Some(bindings_output);
            }
        }
    }

    Ok(())
}
