use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

use anyhow::{bail, Result};
use inflector::Inflector;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::{FileWalker, PathExtensions};
use language_definition::model::Language;
use semver::Version;

pub(crate) fn collect_snapshot_tests(
    data_dir: &Path,
) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut tests = BTreeMap::<String, BTreeSet<String>>::new();

    for file in FileWalker::from_directory(data_dir).find_all()? {
        if let Ok(generated_dir) = file.generated_dir() {
            assert!(
                generated_dir.unwrap_parent().join("input.sol").exists()
                    // handle binder outputs which are placed inside a `binder`
                    // dir of the snapshot
                    || generated_dir
                        .unwrap_parent()
                        .unwrap_parent()
                        .join("input.sol")
                        .exists(),
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
        }
    }

    Ok(tests)
}

pub(crate) fn generate_mod_file(
    fs: &mut CodegenFileSystem,
    mod_file_path: &Path,
    parser_tests: &BTreeMap<String, BTreeSet<String>>,
) -> Result<()> {
    let contents = parser_tests
        .keys()
        .fold(String::new(), |mut buffer, parser_name| {
            writeln!(buffer, "mod {0};", parser_name.to_snake_case()).unwrap();
            buffer
        });

    fs.write_file_raw(mod_file_path, contents)
}

pub(crate) fn generate_unit_test_file(
    runner_module: &str,
    fs: &mut CodegenFileSystem,
    test_dir_name: &str,
    test_names: &BTreeSet<String>,
    unit_test_file_path: &Path,
) -> Result<()> {
    let mut contents = String::new();

    writeln!(contents, "use anyhow::Result;")?;
    writeln!(contents)?;
    writeln!(contents, "use {runner_module}::run;")?;
    writeln!(contents)?;
    writeln!(contents, "const T: &str = \"{test_dir_name}\";")?;

    for test_name in test_names {
        writeln!(contents)?;
        writeln!(contents, "#[test]")?;
        writeln!(contents, "fn {test_name}() -> Result<()> {{")?;
        writeln!(contents, "    run(T, \"{test_name}\")")?;
        writeln!(contents, "}}")?;
    }

    fs.write_file_raw(unit_test_file_path, contents)
}

pub(crate) fn generate_version_breaks(language: &Language, output_dir: &Path) -> Result<()> {
    let mut fs = CodegenFileSystem::default();

    let versions = language.collect_all_breaking_versions();
    let versions_len = versions.len();

    let mut contents = String::new();

    writeln!(contents, "use semver::Version;")?;
    writeln!(contents)?;
    writeln!(
        contents,
        "pub const VERSION_BREAKS: [Version; {versions_len}] = ["
    )?;

    for Version {
        major,
        minor,
        patch,
        ..
    } in versions
    {
        writeln!(contents, "    Version::new({major}, {minor}, {patch}),")?;
    }

    writeln!(contents, "];")?;

    let mod_file_path = output_dir.join("mod.rs");
    fs.write_file_raw(mod_file_path, contents)?;

    Ok(())
}
