use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{bail, Result};
use inflector::Inflector;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::FileWalker;

use crate::common::{collect_snapshot_tests, generate_mod_file, generate_unit_test_file};

pub fn generate_cst_output_tests(data_dir: &Path, output_dir: &Path) -> Result<()> {
    let parser_tests = collect_snapshot_tests(data_dir)?;

    let mut fs = CodegenFileSystem::default();

    generate_mod_file(&mut fs, &output_dir.join("mod.rs"), &parser_tests)?;

    for (parser_name, test_names) in &parser_tests {
        generate_unit_test_file(
            "crate::cst::cst_output::runner",
            &mut fs,
            parser_name,
            test_names,
            &output_dir.join(format!("{0}.rs", parser_name.to_snake_case())),
        )?;
    }

    Ok(())
}

// @Claude: I don't like this method, it's too copy like from `generate_cst_output_tests`. Can't you have `migrate_v1_tests_to_v2` return the list of tests to generate, and then generate the harness code?
/// Like `generate_cst_output_tests`, but walks a directory of generated inputs
/// (does not skip files under `generated/` paths).
pub fn generate_cst_output_tests_for_generated_inputs(
    data_dir: &Path,
    output_dir: &Path,
) -> Result<()> {
    let parser_tests = collect_generated_input_tests(data_dir)?;

    let mut fs = CodegenFileSystem::default();

    generate_mod_file(&mut fs, &output_dir.join("mod.rs"), &parser_tests)?;

    for (parser_name, test_names) in &parser_tests {
        generate_unit_test_file(
            "crate::cst::cst_output::runner",
            &mut fs,
            parser_name,
            test_names,
            &output_dir.join(format!("{0}.rs", parser_name.to_snake_case())),
        )?;
    }

    Ok(())
}

fn collect_generated_input_tests(data_dir: &Path) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut tests = BTreeMap::<String, BTreeSet<String>>::new();

    for file in FileWalker::from_directory(data_dir).find(["**/input.sol"])? {
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
            _ => {
                bail!("Invalid test input. Should be '<data_dir>/GROUP_NAME/TEST_NAME/input.sol', but found: {file:?}");
            }
        }
    }

    Ok(tests)
}
