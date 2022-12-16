use anyhow::Result;
use codegen_utils::context::CodegenContext;
use semver::Version;
use solidity_rust_lib::generated::language::Language;
use solidity_testing_utils::parser_snapshots::ParserOutputTestSnapshotExtensions;

use crate::cst_output::generated::BREAKING_VERSIONS;

pub fn run(parser_name: &str, test_name: &str) -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let test_dir = codegen
            .repo_root
            .join("crates/solidity/testing/snapshots/cst_output")
            .join(parser_name)
            .join(test_name);

        let source = &std::fs::read_to_string(&test_dir.join("input.sol"))?;

        let mut last_snapshot: String = String::new();

        for version in BREAKING_VERSIONS {
            let version = Version::parse(version)?;
            let snapshot_path = test_dir.join(format!("generated/{version}.yml"));

            let parser_output = Language::new(version)
                .parse(parser_name, &source)
                .expect(format!("No such parser: {}", parser_name).as_str());

            let current_snapshot = parser_output.to_test_snapshot(source)?;

            if current_snapshot == last_snapshot {
                // Skip versions that produce the same snapshot.
                continue;
            }

            codegen.write_file(&snapshot_path, &current_snapshot)?;

            last_snapshot = current_snapshot;
        }

        return Ok(());
    });
}
