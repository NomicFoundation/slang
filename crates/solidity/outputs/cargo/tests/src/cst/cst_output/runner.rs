use std::str::FromStr;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use semver::Version;
use slang_solidity::cst::NonterminalKind;
use slang_solidity::parser::Parser;
use slang_solidity_v2_parser::temp_testing::V2TesterConstructor;
use strum_macros::Display;

use crate::cst::cst_output::renderer::render;
use crate::cst::generated::VERSION_BREAKS;

#[derive(Display)]
#[strum(serialize_all = "kebab_case")]
enum TestStatus {
    Success,
    Failure,
}

pub fn run(parser_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("cst_output")
        .join(parser_name)
        .join(test_name);

    let mut fs = CodegenFileSystem::default();

    let input_path = test_dir.join("input.sol");
    let source_id = input_path.strip_repo_root()?.unwrap_str();

    let source = input_path.read_to_string()?;

    let mut last_output = None;

    let mut v2_tester = V2TesterConstructor::new_tester(parser_name);

    // TODO(v2): We forde version 0.8.30 to be tested because it's the only version
    // used by the V2 parser.
    // Once other versions are added we should only test breaking versions.
    let mut versions: Vec<Version> = VERSION_BREAKS.to_vec();
    versions.push(Version::new(0, 8, 30));
    versions.sort();
    versions.dedup();

    for version in &versions {
        let tested_kind = NonterminalKind::from_str(parser_name)
            .unwrap_or_else(|_| panic!("No such parser: {parser_name}"));

        let output = Parser::create(version.clone())?.parse_nonterminal(tested_kind, &source);

        // Test against V2
        v2_tester.test_next(&test_dir, &mut fs, source_id, &source, version, &output)?;

        let output = match last_output {
            // Skip this version if it produces the same output.
            // Note: comparing objects cheaply before expensive serialization.
            Some(ref last) if last == &output => continue,
            _ => &*last_output.insert(output),
        };

        assert_eq!(source, output.tree().unparse(), "source round-trip failed.");

        let errors = output
            .errors()
            .iter()
            .map(|error| {
                const COLOR: bool = false;
                slang_solidity::diagnostic::render(error, source_id, &source, COLOR)
            })
            .collect();

        let mut cursor = output.create_tree_cursor();

        let status = if output.is_valid() {
            TestStatus::Success
        } else {
            TestStatus::Failure
        };

        let snapshot = render(&source, &errors, &mut cursor.clone())?;

        cursor.reset();

        assert!(
            cursor.descendants().all(|e| !e.has_default_label()),
            "{snapshot}"
        );

        // Test did not assign a bad edge label, so save the snapshot to the expected file
        let snapshot_path = test_dir
            .join("generated")
            .join(format!("{version}-{status}.yml"));

        fs.write_file_raw(snapshot_path, &snapshot)?;
    }

    Ok(())
}
