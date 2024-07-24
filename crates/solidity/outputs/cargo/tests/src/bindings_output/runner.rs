use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use slang_solidity::language::Language;

use super::graph::render_graph;
use super::renderer::render_bindings;
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
            let graph_output = render_graph(version, &parse_output, &input_path);
            match last_graph_output {
                Some(ref last) if last == &graph_output => (),
                _ => {
                    let snapshot_path = test_dir.join("generated").join(format!("{version}.mmd"));

                    fs.write_file(snapshot_path, &graph_output)?;
                    last_graph_output = Some(graph_output);
                }
            };
        }

        let bindings_output = render_bindings(version, &parse_output, &source, &input_path)?;
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
