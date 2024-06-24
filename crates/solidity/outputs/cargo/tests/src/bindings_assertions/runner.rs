use std::fs;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use semver::Version;
use slang_solidity::assertions::{check_assertions, collect_assertions};
use slang_solidity::bindings;
use slang_solidity::language::Language;

use super::generated::VERSION_BREAKS;

pub fn run(group_name: &str, file_name: &str) -> Result<()> {
    let data_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("bindings_assertions")
        .join(group_name);
    let input_path = data_dir.join(file_name);
    let input = fs::read_to_string(&input_path)?;

    for version in &VERSION_BREAKS {
        check_assertions_with_version(version, input_path.to_str().unwrap(), &input)?;
    }
    Ok(())
}

fn check_assertions_with_version(
    version: &Version,
    file_path: &str,
    file_contents: &str,
) -> Result<()> {
    let language = Language::new(version.clone())?;

    let parse_output = language.parse(Language::ROOT_KIND, file_contents);
    assert!(parse_output.is_valid());

    let mut bindings = bindings::create(version.clone());
    bindings.add_file(file_path, parse_output.create_tree_cursor());

    let assertions = collect_assertions(parse_output.create_tree_cursor())?;

    let result = check_assertions(&bindings, &assertions, version);

    assert!(
        result.is_ok(),
        "Failed bindings assertions in version {version}:\n{errors}",
        errors = result.err().map(|x| x.to_string()).unwrap_or_default(),
    );

    Ok(())
}
