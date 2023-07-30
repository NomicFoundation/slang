use std::str::FromStr;

use anyhow::Result;
use codegen_utils::context::CodegenContext;
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
    return CodegenContext::with_context(|codegen| {
        let test_dir = codegen
            .repo_root
            .join("crates/solidity/testing/snapshots/cst_output")
            .join(parser_name)
            .join(test_name);

        let input_path = test_dir.join("input.sol");
        let source_id = input_path
            .strip_prefix(&codegen.repo_root)?
            .to_str()
            .unwrap();

        let source = &std::fs::read_to_string(&input_path)?;

        let mut last_snapshot: Option<String> = None;

        for version in VERSION_BREAKS {
            let version = Version::parse(version)?;

            let production_kind = ProductionKind::from_str(parser_name)
                .expect(format!("No such parser: {parser_name}").as_str());

            let (errors, tree, status) = match Language::new(version.to_owned())?
                .parse(production_kind, &source)
            {
                Ok(output) => {
                    let errors = output
                        .errors()
                        .iter()
                        .map(|error| {
                            error.to_error_report(source_id, source, /* with_colour */ false)
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

            let snapshot = CstSnapshots::render(source, &errors, &tree)?;

            if Some(&snapshot) == last_snapshot.as_ref() {
                // Skip versions that produce the same output.
                continue;
            }

            let snapshot_path = test_dir.join(format!(
                "generated/{version}-{status}.yml",
                status = status.as_ref(),
            ));

            codegen.write_file(&snapshot_path, &snapshot)?;

            last_snapshot = Some(snapshot);
        }

        return Ok(());
    });
}
