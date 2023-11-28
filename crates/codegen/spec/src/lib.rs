//! Generates a human-readable specification for the [`LanguageDefinitionRef`].
//!
//! At the time of writing, the generated pages include:
//! - A list of supported versions
//! - A grammar page for each version
//! - A reference page for each version
//!
//! and the auxiliary snippet files included by the grammar mkdocs pages.
//!
//! Exposes a [`SpecGeneratorExtensions`] trait that generates all the pages in a given [`CodegenContext`].
mod grammar;
mod markdown;
mod navigation;
mod reference;
mod snippets;

use std::path::Path;

use anyhow::Result;
use codegen_schema::types::LanguageDefinitionRef;
use infra_utils::codegen::Codegen;

use crate::{
    grammar::{generate_grammar_dir, generate_supported_versions_page},
    navigation::NavigationEntry,
    reference::generate_reference_dir,
    snippets::Snippets,
};

/// Extension trait for [`LanguageDefinitionRef`] that generates the specification files.
pub trait SpecGeneratorExtensions {
    /// Generates the specification files in `output_dir`.
    fn generate_spec(&self, output_dir: &Path) -> Result<()>;
}

impl SpecGeneratorExtensions for LanguageDefinitionRef {
    fn generate_spec(&self, output_dir: &Path) -> Result<()> {
        let mut codegen = Codegen::write_only()?;

        let snippets = Snippets::new(self, output_dir);
        snippets.write_files(&mut codegen)?;

        let root_entry = NavigationEntry::Directory {
            title: format!("{title} Specification", title = self.title),
            path: "public".to_owned(),
            children: vec![
                generate_supported_versions_page(self),
                generate_grammar_dir(self, &snippets),
                generate_reference_dir(self),
            ],
        };

        return root_entry.write_files(&mut codegen, output_dir);
    }
}
