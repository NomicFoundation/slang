use std::path::Path;

use anyhow::Result;
use inflector::Inflector;
use infra_utils::codegen::CodegenFileSystem;

use crate::common::{collect_snapshot_tests, generate_mod_file, generate_unit_test_file};

pub fn generate_binder_tests(data_dir: &Path, output_dir: &Path) -> Result<()> {
    let tests = collect_snapshot_tests(data_dir)?;

    let mut fs = CodegenFileSystem::default();

    generate_mod_file(&mut fs, &output_dir.join("mod.rs"), &tests)?;

    for (group_name, test_names) in &tests {
        generate_unit_test_file(
            "crate::binder::runner",
            &mut fs,
            group_name,
            test_names,
            &output_dir.join(format!("{0}.rs", group_name.to_snake_case())),
        )?;
    }

    Ok(())
}
