use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::Parser as V2Parser;
use solidity_v2_language::SolidityDefinition;
use solidity_v2_testing_utils::reporting::diagnostic;

pub fn run(parser_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?
        .join("cst_output")
        .join(parser_name)
        .join(test_name);

    let input_path = test_dir.join("input.sol");
    let source_id = input_path.strip_repo_root()?.unwrap_str().to_owned();
    let source = input_path.read_to_string()?;

    let mut fs = CodegenFileSystem::default();

    let mut last_output = None;

    let tested_versions: Vec<LanguageVersion> = SolidityDefinition::create()
        .collect_all_breaking_versions()
        .into_iter()
        .map(|v| LanguageVersion::try_from(v).unwrap())
        .collect();

    for &lang_version in &tested_versions {
        let v2_output = V2Parser::parse(&source, lang_version);

        match last_output {
            // Skip this version if it produces the same output.
            // Note: comparing objects cheaply before expensive serialization.
            Some(ref last) if last == &v2_output => continue,
            _ => {
                let (status, content) = match &v2_output {
                    Ok(parsed_cst) => {
                        // Print structured CST
                        ("success", format!("{parsed_cst:#?}\n"))
                    }
                    Err(err) => {
                        // We don't care about the errors for now, we just write them
                        let e = diagnostic::render(err, &source_id, &source, false);
                        ("failure", format!("{e}\n"))
                    }
                };

                let snapshot_path = test_dir
                    .join("generated")
                    .join(format!("{lang_version}-{status}.txt"));

                fs.write_file_raw(&snapshot_path, content)?;
                last_output = Some(v2_output);
            }
        }
    }

    Ok(())
}
