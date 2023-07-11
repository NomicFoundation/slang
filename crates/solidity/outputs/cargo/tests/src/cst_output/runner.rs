use std::str::FromStr;

use anyhow::Result;
use codegen_utils::context::CodegenContext;
use semver::Version;
use slang_solidity::{
    language::{Error, Language},
    syntax::{nodes::ProductionKind, parser::ParseOutput},
};
use solidity_testing_utils::cst_snapshots::ParseOutputTestSnapshotExtensions;

use crate::cst_output::generated::VERSION_BREAKS;

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

        let mut last_output: Option<ParseOutput> = None;

        for version in VERSION_BREAKS {
            let version = Version::parse(version)?;

            let production_kind = ProductionKind::from_str(parser_name)
                .expect(format!("No such parser: {parser_name}").as_str());

            let output = match Language::new(version.to_owned())?.parse(production_kind, &source) {
                Ok(output) => output,
                Err(error) => match error {
                    Error::InvalidProductionVersion(_) => {
                        continue; // Skip versions that this production is not defined in.
                    }
                    _ => panic!("Unexpected error: {:?}", error),
                },
            };

            if let Some(last_output) = &last_output {
                if &output == last_output {
                    // Skip versions that produce the same output.
                    continue;
                }
            }

            let test_status = if output.is_valid() {
                "success"
            } else {
                "failure"
            };

            let snapshot_path = test_dir.join(format!("generated/{version}-{test_status}.yml"));
            let snapshot = output.to_test_snapshot(source_id, source)?;
            codegen.write_file(&snapshot_path, &snapshot)?;

            last_output = Some(output);
        }

        return Ok(());
    });
}
