mod cst_output;

use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::grammar::Grammar;
use codegen_utils::context::CodegenContext;

use crate::cst_output::generate_cst_output_tests;

pub trait GrammarTestingGeneratorExtensions {
    fn generate_cst_output_tests(
        &self,
        codegen: &mut CodegenContext,
        snapshots_dir: &PathBuf,
        output_dir: &PathBuf,
    ) -> Result<()>;
}

impl GrammarTestingGeneratorExtensions for Grammar {
    fn generate_cst_output_tests(
        &self,
        codegen: &mut CodegenContext,
        data_dir: &PathBuf,
        output_dir: &PathBuf,
    ) -> Result<()> {
        return generate_cst_output_tests(self, codegen, data_dir, output_dir);
    }
}
