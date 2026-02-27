mod binder;
mod bindings_output;
mod common;
pub mod cst_output;

use std::path::Path;

use anyhow::Result;
use common::generate_version_breaks;
use language_definition::model::Language;

use crate::binder::generate_binder_tests;
use crate::bindings_output::generate_bindings_output_tests;
use crate::cst_output::generate_cst_output_tests;

pub trait TestingGeneratorExtensions {
    fn generate_version_breaks(&self, output_dir: &Path) -> Result<()>;
    fn generate_bindings_output_tests(&self, snapshots_dir: &Path, output_dir: &Path)
        -> Result<()>;
    fn generate_cst_output_tests(&self, snapshots_dir: &Path, output_dir: &Path) -> Result<()>;
    fn generate_binder_tests(&self, snapshots_dir: &Path, output_dir: &Path) -> Result<()>;
}

impl TestingGeneratorExtensions for Language {
    fn generate_version_breaks(&self, output_dir: &Path) -> Result<()> {
        generate_version_breaks(self, output_dir)
    }
    fn generate_bindings_output_tests(&self, data_dir: &Path, output_dir: &Path) -> Result<()> {
        generate_bindings_output_tests(data_dir, output_dir)
    }
    fn generate_cst_output_tests(&self, data_dir: &Path, output_dir: &Path) -> Result<()> {
        generate_cst_output_tests(data_dir, output_dir)
    }
    fn generate_binder_tests(&self, data_dir: &Path, output_dir: &Path) -> Result<()> {
        generate_binder_tests(data_dir, output_dir)
    }
}
