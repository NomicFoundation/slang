mod grammar;
mod markdown;
mod navigation;
mod reference;
mod snippets;

use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::grammar::Grammar;
use codegen_utils::context::CodegenContext;

use crate::{
    grammar::{generate_grammar_dir, generate_supported_versions_page},
    navigation::NavigationEntry,
    reference::generate_reference_dir,
    snippets::Snippets,
};

pub trait GrammarSpecGeneratorExtensions {
    fn generate_spec(&self, codegen: &mut CodegenContext, output_dir: &PathBuf) -> Result<()>;
}

impl GrammarSpecGeneratorExtensions for Grammar {
    fn generate_spec(&self, codegen: &mut CodegenContext, output_dir: &PathBuf) -> Result<()> {
        let snippets = Snippets::new(self, output_dir);
        snippets.write_files(codegen)?;

        let root_entry = NavigationEntry::Directory {
            title: format!("{title} Specification", title = self.title),
            path: "public".to_owned(),
            children: vec![
                generate_supported_versions_page(self),
                generate_grammar_dir(self, &snippets, &codegen.repo_root),
                generate_reference_dir(self, &codegen.repo_root),
            ],
        };

        return root_entry.write_files(codegen, output_dir);
    }
}
