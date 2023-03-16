mod grammar;
mod markdown;
mod navigation;
mod pages;
mod snippets;

use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::grammar::Grammar;
use codegen_utils::context::CodegenContext;

use crate::{pages::PublicPages, snippets::Snippets};

pub trait GrammarSpecGeneratorExtensions {
    fn generate_spec(&self, codegen: &mut CodegenContext, output_dir: &PathBuf) -> Result<()>;
}

impl GrammarSpecGeneratorExtensions for Grammar {
    fn generate_spec(&self, codegen: &mut CodegenContext, output_dir: &PathBuf) -> Result<()> {
        let snippets = Snippets::new(self, output_dir);
        snippets.write_files(codegen)?;

        let public_pages = PublicPages::new(self, &snippets);
        public_pages.write_files(codegen, output_dir)?;

        return Ok(());
    }
}
