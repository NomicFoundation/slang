use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

use anyhow::{bail, Result};
use codegen_language_definition::model::Language;
use inflector::Inflector;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::FileWalker;

pub fn generate_bindings_tests(
    _language: &Language,
    data_dir: &Path,
    output_dir: &Path,
) -> Result<()> {
    let bindings_tests = collect_bindings_tests(data_dir)?;

    let mut fs = CodegenFileSystem::new(data_dir)?;

    generate_mod_file(&mut fs, &output_dir.join("mod.rs"), &bindings_tests)?;

    for (group_name, test_files) in &bindings_tests {
        generate_unit_test_file(
            &mut fs,
            group_name,
            test_files,
            &output_dir.join(format!("{0}.rs", group_name.to_snake_case())),
        )?;
    }

    Ok(())
}

fn collect_bindings_tests(data_dir: &Path) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut bindings_tests = BTreeMap::<String, BTreeSet<String>>::new();

    for file in FileWalker::from_directory(data_dir).find(["**/*.sol"])? {
        let parts: Vec<_> = file
            .strip_prefix(data_dir)?
            .iter()
            .map(|p| p.to_str().unwrap())
            .collect();

        match parts[..] {
            [group_name, test_file] => {
                bindings_tests
                    .entry(group_name.to_owned())
                    .or_default()
                    .insert(test_file.to_owned());
            }
            _ => {
                bail!("Invalid test input. Should be in the form of '<tests-dir>/GROUP_NAME/TEST_FILE.sol', but found: {file:?}");
            }
        };
    }

    Ok(bindings_tests)
}

fn generate_mod_file(
    fs: &mut CodegenFileSystem,
    mod_file_path: &Path,
    bindings_tests: &BTreeMap<String, BTreeSet<String>>,
) -> Result<()> {
    let module_declarations_str =
        bindings_tests
            .keys()
            .fold(String::new(), |mut buffer, group_name| {
                writeln!(buffer, "mod {0};", group_name.to_snake_case()).unwrap();
                buffer
            });

    let contents = format!(
        "
            {module_declarations_str}
        ",
    );

    fs.write_file(mod_file_path, contents)
}

fn generate_unit_test_file(
    fs: &mut CodegenFileSystem,
    group_name: &str,
    test_files: &BTreeSet<String>,
    unit_test_file_path: &Path,
) -> Result<()> {
    let unit_tests_str = test_files
        .iter()
        .fold(String::new(), |mut buffer, test_file| {
            let test_name = test_file.strip_suffix(".sol").unwrap();
            writeln!(
                buffer,
                r#"
                    #[test]
                    fn {test_name}() -> Result<()> {{
                        run("{group_name}", "{test_file}")
                    }}
                "#
            )
            .unwrap();
            buffer
        });

    let contents = format!(
        "
            use anyhow::Result;

            use crate::bindings::runner::run;

            {unit_tests_str}
        "
    );

    fs.write_file(unit_test_file_path, contents)
}
