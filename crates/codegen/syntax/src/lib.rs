mod legacy;
mod nodes;

use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::LanguageDefinitionRef;
use codegen_utils::context::CodegenContext;

use crate::{legacy::language::PrivateSyntaxGeneratorExtensions, nodes::generate_syntax_nodes_mod};

pub struct SyntaxGeneratorPaths {
    pub src_dir: PathBuf,
    pub syntax_nodes_mod: &'static str,
}

pub trait SyntaxGeneratorExtensions {
    fn generate_legacy_rust_lib_sources(&self, codegen: &mut CodegenContext, output_dir: &PathBuf);

    fn generate_legacy_napi_lib_sources(&self, codegen: &mut CodegenContext, output_dir: &PathBuf);

    fn generate_syntax_lib_sources(
        &self,
        codegen: &mut CodegenContext,
        paths: &SyntaxGeneratorPaths,
    ) -> Result<()>;
}

impl SyntaxGeneratorExtensions for LanguageDefinitionRef {
    fn generate_legacy_rust_lib_sources(&self, codegen: &mut CodegenContext, output_dir: &PathBuf) {
        self.create_code_generator()
            .write_rust_lib_sources(self, codegen, output_dir);
    }

    fn generate_legacy_napi_lib_sources(&self, codegen: &mut CodegenContext, output_dir: &PathBuf) {
        self.create_code_generator()
            .write_napi_lib_sources(self, codegen, output_dir);
    }

    fn generate_syntax_lib_sources(
        &self,
        codegen: &mut CodegenContext,
        paths: &SyntaxGeneratorPaths,
    ) -> Result<()> {
        generate_syntax_nodes_mod(self, codegen, &paths.src_dir.join(paths.syntax_nodes_mod))?;

        return Ok(());
    }
}
