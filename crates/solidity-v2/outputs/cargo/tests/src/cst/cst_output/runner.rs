use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::validation::validate_syntax_version;
use slang_solidity_v2_parser::Parser as V2Parser;
use solidity_v2_language::SolidityDefinition;

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
        .collect_syntax_breaking_versions()
        .into_iter()
        .map(|v| LanguageVersion::try_from(v).unwrap())
        .collect();

    for &lang_version in &tested_versions {
        let v2_parse = V2Parser::parse(&source, lang_version);

        // TODO(v2): these tests should really go through 'CompilationUnit' once it is ready.
        // This way, we won't have to call individual validation APIs.
        // All errors should be collected during the compilation unit construction.
        let validation_errors = if v2_parse.errors.is_empty() {
            validate_syntax_version(&v2_parse.source_unit, lang_version)
        } else {
            Vec::new()
        };

        let v2_output = (v2_parse, validation_errors);

        match last_output {
            // Skip this version if it produces the same output.
            // Note: comparing objects cheaply before expensive serialization.
            Some(ref last) if last == &v2_output => continue,
            _ => {
                let (status, content) = solidity_v2_testing_utils::cst_renderer::render(
                    &source, &source_id, &v2_output,
                );

                let status = if status { "success" } else { "failure" };

                let snapshot_path = test_dir
                    .join("generated")
                    .join(format!("{lang_version}-{status}.yml"));

                fs.write_file_raw(&snapshot_path, content)?;
                last_output = Some(v2_output);
            }
        }
    }

    Ok(())
}
