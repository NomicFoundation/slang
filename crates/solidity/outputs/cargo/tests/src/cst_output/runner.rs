use std::str::FromStr;

use anyhow::Result;
use infra_utils::{cargo::CargoWorkspace, codegen::Codegen, paths::PathExtensions};
use slang_solidity::{kinds::ProductionKind, language::Language};
use solidity_testing_utils::cst_snapshots::CstSnapshots;
use strum_macros::Display;

use crate::cst_output::generated::VERSION_BREAKS;

#[derive(Display)]
#[strum(serialize_all = "kebab_case")]
enum TestStatus {
    Success,
    Failure,
}

pub fn run(parser_name: &str, test_name: &str) -> Result<()> {
    let mut codegen = Codegen::write_only()?;

    let test_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("cst_output")
        .join(parser_name)
        .join(test_name);

    let input_path = test_dir.join("input.sol");
    let source_id = input_path.strip_repo_root()?.unwrap_str();

    let source = input_path.read_to_string()?;

    let mut last_output = None;

    for version in VERSION_BREAKS {
        let production_kind = ProductionKind::from_str(parser_name)
            .expect(format!("No such parser: {parser_name}").as_str());

        let output = Language::new(version.clone())?.parse(production_kind, &source);

        let output = match last_output {
            // Skip this version if it produces the same output.
            // Note: comparing objects cheaply before expensive serialization.
            Some(ref last) if last == &output => continue,
            _ => &*last_output.insert(output),
        };

        let errors = output
            .errors()
            .iter()
            .map(|error| {
                error.to_error_report(source_id, &source, /* with_color */ false)
            })
            .collect();

        let tree = Some(output.parse_tree());

        let status = if output.is_valid() {
            TestStatus::Success
        } else {
            TestStatus::Failure
        };

        let snapshot = CstSnapshots::render(&source, &errors, &tree)?;

        let snapshot_path = test_dir
            .join("generated")
            .join(format!("{version}-{status}.yml"));

        codegen.write_file(snapshot_path, &snapshot)?;
    }

    return Ok(());
}
