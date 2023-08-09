mod legacy;
mod nodes;

use std::path::Path;

use anyhow::Result;
use codegen_schema::types::LanguageDefinitionRef;

use crate::{legacy::language::PrivateSyntaxGeneratorExtensions, nodes::generate_syntax_nodes_mod};

pub trait SyntaxGeneratorExtensions {
    fn generate_legacy_rust_lib_sources(&self, crate_dir: &Path) -> Result<()>;

    fn generate_legacy_napi_lib_sources(&self, crate_dir: &Path) -> Result<()>;

    fn generate_syntax_lib_sources(&self, crate_dir: &Path) -> Result<()>;
}

impl SyntaxGeneratorExtensions for LanguageDefinitionRef {
    fn generate_legacy_rust_lib_sources(&self, crate_dir: &Path) -> Result<()> {
        return self
            .create_code_generator()
            .write_rust_lib_sources(self, &crate_dir.join("src/legacy/generated"));
    }

    fn generate_legacy_napi_lib_sources(&self, crate_dir: &Path) -> Result<()> {
        return self
            .create_code_generator()
            .write_napi_lib_sources(self, &crate_dir.join("src/legacy/generated"));
    }

    fn generate_syntax_lib_sources(&self, crate_dir: &Path) -> Result<()> {
        generate_syntax_nodes_mod(self, &crate_dir.join("src/syntax/nodes/generated"))?;

        return Ok(());
    }
}
