mod bindings_assertions;
mod bindings_output;
mod common;
mod cst_output;

use std::path::Path;

use anyhow::Result;
use codegen_language_definition::model::Language;
use common::generate_version_breaks;

use crate::bindings_assertions::generate_bindings_assertions_tests;
use crate::bindings_output::generate_bindings_output_tests;
use crate::cst_output::generate_cst_output_tests;

pub trait TestingGeneratorExtensions {
    fn generate_version_breaks(&self, lang_def_dir: &Path, output_dir: &Path) -> Result<()>;
    fn generate_bindings_assertions_tests(
        &self,
        sources_dir: &Path,
        output_dir: &Path,
    ) -> Result<()>;
    fn generate_bindings_output_tests(&self, snapshots_dir: &Path, output_dir: &Path)
        -> Result<()>;
    fn generate_cst_output_tests(&self, snapshots_dir: &Path, output_dir: &Path) -> Result<()>;
}

impl TestingGeneratorExtensions for Language {
    fn generate_version_breaks(&self, lang_def_dir: &Path, output_dir: &Path) -> Result<()> {
        generate_version_breaks(self, lang_def_dir, output_dir)
    }
    fn generate_bindings_assertions_tests(&self, data_dir: &Path, output_dir: &Path) -> Result<()> {
        generate_bindings_assertions_tests(data_dir, output_dir)
    }
    fn generate_bindings_output_tests(&self, data_dir: &Path, output_dir: &Path) -> Result<()> {
        generate_bindings_output_tests(data_dir, output_dir)
    }
    fn generate_cst_output_tests(&self, data_dir: &Path, output_dir: &Path) -> Result<()> {
        generate_cst_output_tests(data_dir, output_dir)
    }
}
