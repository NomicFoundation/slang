use std::str::FromStr;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity::kinds::NonterminalKind;
use slang_solidity::language::Language;
use strum_macros::Display;

use crate::cst_output::generated::VERSION_BREAKS;
use crate::cst_output::renderer::render;

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

    let mut fs = CodegenFileSystem::new(&test_dir)?;

    let input_path = test_dir.join("input.sol");
    let source_id = input_path.strip_repo_root()?.unwrap_str();

    let source = input_path.read_to_string()?;

    let mut last_output = None;

    for version in VERSION_BREAKS {
        let tested_kind = NonterminalKind::from_str(parser_name)
            .unwrap_or_else(|_| panic!("No such parser: {parser_name}"));

        let output = Language::new(version.clone())?.parse(tested_kind, &source);

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
                const COLOR: bool = false;
                slang_solidity::diagnostic::render(error, source_id, &source, COLOR)
            })
            .collect();

        let cursor = output.create_tree_cursor().with_edges();

        let status = if output.is_valid() {
            TestStatus::Success
        } else {
            TestStatus::Failure
        };

        let snapshot = render(&source, &errors, cursor)?;

        let snapshot_path = test_dir
            .join("generated")
            .join(format!("{version}-{status}.yml"));

        fs.write_file(snapshot_path, &snapshot)?;
    }

    Ok(())
}
