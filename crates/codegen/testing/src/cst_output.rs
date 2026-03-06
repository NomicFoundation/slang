use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::Result;
use inflector::Inflector;
use infra_utils::codegen::CodegenFileSystem;

use crate::common::{collect_snapshot_tests, generate_mod_file, generate_unit_test_file};

/// Generates test harness files from a pre-built test map.
pub fn generate_cst_output_test_harness(
    runner_module: &str,
    parser_tests: &BTreeMap<String, BTreeSet<String>>,
    output_dir: &Path,
) -> Result<()> {
    let mut fs = CodegenFileSystem::default();

    generate_mod_file(&mut fs, &output_dir.join("mod.rs"), parser_tests)?;

    for (parser_name, test_names) in parser_tests {
        generate_unit_test_file(
            runner_module,
            &mut fs,
            parser_name,
            test_names,
            &output_dir.join(format!("{0}.rs", parser_name.to_snake_case())),
        )?;
    }

    Ok(())
}

pub fn generate_cst_output_tests(data_dir: &Path, output_dir: &Path) -> Result<()> {
    let parser_tests = collect_snapshot_tests(data_dir)?;
    generate_cst_output_test_harness("crate::cst::cst_output::runner", &parser_tests, output_dir)
}
