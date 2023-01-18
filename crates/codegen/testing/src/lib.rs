mod cst_output;
mod versions;

use std::{collections::BTreeSet, path::PathBuf};

use anyhow::Result;
use codegen_schema::grammar::Grammar;
use codegen_utils::context::CodegenContext;
use semver::Version;

use crate::{cst_output::generate_cst_output_tests, versions::collect_breaking_versions};

pub trait GrammarTestingGeneratorExtensions {
    fn generate_cst_output_tests(
        &self,
        codegen: &mut CodegenContext,
        snapshots_dir: &PathBuf,
        output_dir: &PathBuf,
    ) -> Result<()>;

    fn collect_breaking_versions<'a>(&'a self) -> BTreeSet<&'a Version>;
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

    fn collect_breaking_versions<'a>(&'a self) -> BTreeSet<&'a Version> {
        return collect_breaking_versions(self);
    }
}
