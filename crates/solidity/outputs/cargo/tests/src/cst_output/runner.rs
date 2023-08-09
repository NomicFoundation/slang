use std::str::FromStr;

use anyhow::Result;
use infra_utils::{cargo::CargoWorkspace, codegen::Codegen, paths::PathExtensions};
use semver::Version;
use slang_solidity::{language::Language, syntax::nodes::ProductionKind};
use solidity_testing_utils::cst_snapshots::CstSnapshots;
use strum_macros::AsRefStr;

use crate::cst_output::generated::VERSION_BREAKS;

#[derive(AsRefStr)]
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
        let version = Version::parse(version)?;

        let production_kind = ProductionKind::from_str(parser_name)
            .expect(format!("No such parser: {parser_name}").as_str());

        let output = Language::new(version.to_owned())?.parse(production_kind, &source);

        if Some(&output) == last_output.as_ref() {
            // Skip this version if it produces the same output.
            // Note: comparing objects cheaply before expensive serialization.
            continue;
        }

        last_output = Some(output);

        let (errors, tree, status) =
            match Language::new(version.to_owned())?.parse(production_kind, &source) {
                Ok(output) => {
                    let errors = output
                        .errors()
                        .iter()
                        .map(|error| {
                            error.to_error_report(source_id, &source, /* with_colour */ false)
                        })
                        .collect();

                    let tree = Some(output.parse_tree());

                    let status = if output.is_valid() {
                        TestStatus::Success
                    } else {
                        TestStatus::Failure
                    };

                    (errors, tree, status)
                }
                Err(error) => {
                    let errors = vec![format!("{error:#?}")];
                    let tree = None;
                    let status = TestStatus::Failure;

                    (errors, tree, status)
                }
            };

        let snapshot = CstSnapshots::render(&source, &errors, &tree)?;

        let snapshot_path = test_dir
            .join("generated")
            .join(format!("{version}-{status}.yml", status = status.as_ref()));

        codegen.write_file(snapshot_path, &snapshot)?;
    }

    return Ok(());
}
