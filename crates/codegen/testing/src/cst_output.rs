use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

use anyhow::{bail, Result};
use codegen_language_definition::model::Language;
use inflector::Inflector;
use infra_utils::codegen::{Codegen, CodegenReadWrite};
use infra_utils::paths::{FileWalker, PathExtensions};

pub fn generate_cst_output_tests(
    language: &Language,
    data_dir: &Path,
    output_dir: &Path,
) -> Result<()> {
    let parser_tests = collect_parser_tests(data_dir)?;

    let mut codegen = Codegen::read_write(data_dir)?;

    generate_mod_file(
        language,
        &mut codegen,
        &output_dir.join("mod.rs"),
        &parser_tests,
    )?;

    for (parser_name, test_names) in &parser_tests {
        generate_unit_test_file(
            &mut codegen,
            parser_name,
            test_names,
            &output_dir.join(format!("{0}.rs", parser_name.to_snake_case())),
        )?;
    }

    Ok(())
}

fn collect_parser_tests(data_dir: &Path) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut parser_tests = BTreeMap::<String, BTreeSet<String>>::new();

    for file in FileWalker::from_directory(data_dir).find_all()? {
        if let Ok(generated_dir) = file.generated_dir() {
            assert!(
                generated_dir.unwrap_parent().join("input.sol").exists(),
                "Each snapshot should have a matching input.sol test file: {file:?}",
            );

            // skip generated files
            continue;
        }

        let parts: Vec<_> = file
            .strip_prefix(data_dir)?
            .iter()
            .map(|p| p.to_str().unwrap())
            .collect();

        match parts[..] {
            [parser_name, test_name, "input.sol"] => {
                parser_tests
                    .entry(parser_name.to_owned())
                    .or_default()
                    .insert(test_name.to_owned());
            }
            [.., ".gitattributes"] => {
                /* Some tests depend on having CRLF being explicitly specified as EOL */
            }
            _ => {
                bail!("Invalid test input. Should be in the form of '<tests-dir>/PARSER_NAME/TEST_NAME/input.sol', but found: {file:?}");
            }
        };
    }

    Ok(parser_tests)
}

fn generate_mod_file(
    language: &Language,
    codegen: &mut CodegenReadWrite,
    mod_file_path: &Path,
    parser_tests: &BTreeMap<String, BTreeSet<String>>,
) -> Result<()> {
    let module_declarations_str =
        parser_tests
            .keys()
            .fold(String::new(), |mut buffer, parser_name| {
                writeln!(buffer, "mod {0};", parser_name.to_snake_case()).unwrap();
                buffer
            });

    let version_breaks = language.collect_breaking_versions();
    let version_breaks_len = version_breaks.len();
    let version_breaks_str = version_breaks
        .iter()
        .fold(String::new(), |mut buffer, version| {
            writeln!(
                buffer,
                "Version::new({}, {}, {}),",
                version.major, version.minor, version.patch
            )
            .unwrap();
            buffer
        });

    let contents = format!(
        "
            use semver::Version;
            {module_declarations_str}

            pub const VERSION_BREAKS: [Version; {version_breaks_len}] = [
                {version_breaks_str}
            ];
        ",
    );

    codegen.write_file(mod_file_path, contents)
}

fn generate_unit_test_file(
    codegen: &mut CodegenReadWrite,
    parser_name: &str,
    test_names: &BTreeSet<String>,
    unit_test_file_path: &Path,
) -> Result<()> {
    let unit_tests_str = test_names
        .iter()
        .fold(String::new(), |mut buffer, test_name| {
            writeln!(
                buffer,
                r#"
                    #[test]
                    fn {test_name}() -> Result<()> {{
                        run("{parser_name}", "{test_name}")
                    }}
                "#
            )
            .unwrap();
            buffer
        });

    let contents = format!(
        "
            use anyhow::Result;

            use crate::cst_output::runner::run;

            {unit_tests_str}
        "
    );

    codegen.write_file(unit_test_file_path, contents)
}
