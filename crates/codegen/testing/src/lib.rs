mod cst_output;

use std::path::Path;

use anyhow::Result;
use codegen_language_definition::model::Language;

use crate::cst_output::generate_cst_output_tests;

pub trait TestingGeneratorExtensions {
    fn generate_cst_output_tests(&self, snapshots_dir: &Path, output_dir: &Path) -> Result<()>;
}

impl TestingGeneratorExtensions for Language {
    fn generate_cst_output_tests(&self, data_dir: &Path, output_dir: &Path) -> Result<()> {
        generate_cst_output_tests(self, data_dir, output_dir)
    }
}
