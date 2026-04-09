use std::fmt::Write;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::validation::validate_syntax_version;
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
        .collect_breaking_versions()
        .into_iter()
        .map(|v| LanguageVersion::try_from(v).unwrap())
        .collect();

    for &lang_version in &tested_versions {
        let v2_parse = V2Parser::parse(&source, lang_version);

        // TODO(v2): these tests should really go through 'CompilationUnit' once it is ready.
        // This way, we won't have to call individual validation APIs.
        // All errors should be collected during the compilation unit construction.
        let validation_errors = match &v2_parse {
            Ok(parsed_cst) => validate_syntax_version(parsed_cst, lang_version),
            Err(_) => Vec::new(),
        };

        let v2_output = (v2_parse, validation_errors);

        match last_output {
            // Skip this version if it produces the same output.
            // Note: comparing objects cheaply before expensive serialization.
            Some(ref last) if last == &v2_output => continue,
            _ => {
                let (status, content) = match &v2_output {
                    (Ok(parsed_cst), validation_errors) if validation_errors.is_empty() => {
                        // Print the structured CST:
                        ("success", format!("{parsed_cst:#?}\n"))
                    }
                    (Ok(parsed_cst), validation_errors) => {
                        // Print validation errors, followed by the structured CST:
                        let mut s = String::new();
                        for err in validation_errors {
                            let rendered = diagnostic::render(err, &source_id, &source, false);
                            writeln!(s, "{rendered}\n").unwrap();
                        }

                        writeln!(s, "\n{parsed_cst:#?}").unwrap();

                        ("failure", s)
                    }
                    (Err(err), _) => {
                        // No structured CST to print. Just print the error:
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
