use std::fs;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use slang_solidity::assertions::{check_assertions, collect_assertions};
use slang_solidity::bindings;
use slang_solidity::language::Language;

pub fn run(group_name: &str, file_name: &str) -> Result<()> {
    let data_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("bindings_assertions")
        .join(group_name);
    let input_path = data_dir.join(file_name);
    let input = fs::read_to_string(&input_path)?;

    // TODO: de-hardcode this and parse with different versions?
    let version = Language::SUPPORTED_VERSIONS.last().unwrap();
    let language = Language::new(version.clone())?;

    let parse_output = language.parse(Language::ROOT_KIND, &input);
    assert!(parse_output.is_valid());

    let mut bindings = bindings::create(version.clone());
    bindings.add_file(
        input_path.to_str().unwrap(),
        parse_output.create_tree_cursor(),
    );

    let assertions = collect_assertions(parse_output.create_tree_cursor())?;
    check_assertions(&bindings, &assertions)?;

    Ok(())
}
