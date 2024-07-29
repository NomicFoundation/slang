use std::fs;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use semver::Version;
use slang_solidity::language::Language;
use slang_solidity::{bindings, diagnostic};

use super::assertions::{check_assertions, collect_assertions};
use crate::generated::VERSION_BREAKS;

pub fn run(group_name: &str, test_name: &str) -> Result<()> {
    let file_name = format!("{test_name}.sol");
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

    let mut bindings = bindings::create(version.clone());
    bindings.add_file(file_path, parse_output.create_tree_cursor());

    let assertions = collect_assertions(parse_output.create_tree_cursor(), version)?;

    let result = check_assertions(&bindings, &assertions, version);

    if !parse_output.is_valid() {
        let report = parse_output
            .errors()
            .iter()
            .map(|error| diagnostic::render(error, file_path, file_contents, false))
            .collect::<Vec<_>>()
            .join("\n");
        eprintln!("\nParse errors for version {version}\nFile: {file_path}\n{report}");
    }

    match result {
        Ok(count) => {
            assert!(count > 0, "No assertions found with version {version}");
            println!("Version {version}, {count} assertions OK");
        }
        Err(err) => {
            panic!("Failed bindings assertions in version {version}:\n{err}");
        }
    }

    Ok(())
}
