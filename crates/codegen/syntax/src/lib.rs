mod legacy;
mod nodes;
mod templates;

use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::LanguageDefinitionRef;
use codegen_utils::context::CodegenContext;

use crate::{
    legacy::language::PrivateSyntaxGeneratorExtensions, nodes::generate_syntax_nodes_mod,
    templates::compile_templates,
};

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
        let tera = compile_templates(codegen)?;

        generate_syntax_nodes_mod(
            self,
            &tera,
            codegen,
            &paths.src_dir.join(paths.syntax_nodes_mod),
        )?;

        return Ok(());
    }
}
