use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::paths::FileWalker;
use semver::Version;
use slang_solidity::bindings::graph_builder::{
    ExecutionConfig, File as GraphBuilderFile, Functions, NoCancellation, Variables,
};
use slang_solidity::language::Language;
use std::fs::{self, create_dir_all};

#[test]
pub fn run_all() -> Result<()> {
    let data_dir =
        CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?.join("graph_output");

    for file in FileWalker::from_directory(data_dir).find(["*.sol"])? {
        run(file.file_name().unwrap().to_str().unwrap())?;
    }

    Ok(())
}

fn run(file_name: &str) -> Result<()> {
    let data_dir =
        CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?.join("graph_output");
    let input_path = data_dir.join(file_name);
    let input = fs::read_to_string(input_path)?;
    // TODO: de-hardcode this and parse with different versions?
    let language = Language::new(Version::new(0, 8, 22))?;
    let parse_output = language.parse(Language::ROOT_KIND, &input);
    assert!(parse_output.is_valid());

    let msgb_path =
        CargoWorkspace::locate_source_crate("solidity_stack_graph")?.join("stack-graphs.msgb");
    let msgb_source = fs::read_to_string(&msgb_path)?;
    let graph_builder = GraphBuilderFile::from_str(&msgb_source)?;

    let functions = Functions::stdlib();
    let variables = Variables::new();
    let execution_config = ExecutionConfig::new(&functions, &variables);
    let tree = parse_output.create_tree_cursor();
    let graph = graph_builder.execute(&tree, &execution_config, &NoCancellation)?;

    let output_dir = data_dir.join("generated");
    create_dir_all(&output_dir)?;

    let output_path = output_dir.join(format!("{file_name}.graph"));

    fs::write(output_path, format!("{}", graph.pretty_print()))?;

    Ok(())
}
