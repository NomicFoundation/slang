use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{bail, Result};
use codegen_language_definition::model::Language;
use inflector::Inflector;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::FileWalker;

use crate::common::{generate_mod_file, generate_unit_test_file};

pub fn generate_bindings_assertions_tests(
    language: &Language,
    data_dir: &Path,
    output_dir: &Path,
) -> Result<()> {
    let tests = collect_assertion_tests(data_dir)?;

    let mut fs = CodegenFileSystem::new(data_dir)?;

    generate_mod_file(language, &mut fs, &output_dir.join("mod.rs"), &tests)?;

    for (group_name, test_files) in &tests {
        generate_unit_test_file(
            "crate::bindings_assertions::runner",
            &mut fs,
            group_name,
            test_files,
            &output_dir.join(format!("{0}.rs", group_name.to_snake_case())),
        )?;
    }

    Ok(())
}

fn collect_assertion_tests(data_dir: &Path) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut bindings_tests = BTreeMap::<String, BTreeSet<String>>::new();

    for file in FileWalker::from_directory(data_dir).find(["**/*.sol"])? {
        let parts: Vec<_> = file
            .strip_prefix(data_dir)?
            .iter()
            .map(|p| p.to_str().unwrap())
            .collect();

        match parts[..] {
            [group_name, test_file] => {
                let test_name = test_file.strip_suffix(".sol").unwrap();
                bindings_tests
                    .entry(group_name.to_owned())
                    .or_default()
                    .insert(test_name.to_owned());
            }
            _ => {
                bail!("Invalid test input. Should be in the form of '<tests-dir>/GROUP_NAME/TEST_FILE.sol', but found: {file:?}");
            }
        };
    }

    Ok(bindings_tests)
}
