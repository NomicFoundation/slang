use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

use anyhow::{bail, Result};
use codegen_language_definition::model::Language;
use inflector::Inflector;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::{FileWalker, PathExtensions};

pub(crate) fn collect_snapshot_tests(
    data_dir: &Path,
) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut tests = BTreeMap::<String, BTreeSet<String>>::new();

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
            [group_name, test_name, "input.sol"] => {
                tests
                    .entry(group_name.to_owned())
                    .or_default()
                    .insert(test_name.to_owned());
            }
            [.., ".gitattributes"] => {
                /* Some tests depend on having CRLF being explicitly specified as EOL */
            }
            _ => {
                bail!("Invalid test input. Should be in the form of '<tests-dir>/GROUP_NAME/TEST_NAME/input.sol', but found: {file:?}");
            }
        };
    }

    Ok(tests)
}

pub(crate) fn generate_mod_file(
    language: &Language,
    fs: &mut CodegenFileSystem,
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

    fs.write_file(mod_file_path, contents)
}

pub(crate) fn generate_unit_test_file(
    runner_module: &str,
    fs: &mut CodegenFileSystem,
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

            use {runner_module}::run;

            {unit_tests_str}
        "
    );

    fs.write_file(unit_test_file_path, contents)
}

pub(crate) fn generate_version_breaks(
    language: &Language,
    input_dir: &Path,
    output_dir: &Path,
) -> Result<()> {
    let mut fs = CodegenFileSystem::new(input_dir)?;

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

            pub const VERSION_BREAKS: [Version; {version_breaks_len}] = [
                {version_breaks_str}
            ];
        ",
    );

    let mod_file_path = output_dir.join("mod.rs");
    fs.write_file(mod_file_path, contents)?;

    Ok(())
}
