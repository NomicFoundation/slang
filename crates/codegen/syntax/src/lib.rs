mod legacy;

use codegen_schema::types::LanguageDefinitionRef;
use codegen_utils::context::CodegenContext;

use crate::legacy::language::PrivateSyntaxGeneratorExtensions;

pub trait SyntaxGeneratorExtensions {
    fn generate_legacy_rust_lib_sources(
        &self,
        codegen: &mut CodegenContext,
        output_dir: &std::path::PathBuf,
    );

    fn generate_legacy_napi_lib_sources(
        &self,
        codegen: &mut CodegenContext,
        output_dir: &std::path::PathBuf,
    );
}

impl SyntaxGeneratorExtensions for LanguageDefinitionRef {
    fn generate_legacy_rust_lib_sources(
        &self,
        codegen: &mut CodegenContext,
        output_dir: &std::path::PathBuf,
    ) {
        self.create_code_generator()
            .write_rust_lib_sources(self, codegen, output_dir);
    }

    fn generate_legacy_napi_lib_sources(
        &self,
        codegen: &mut CodegenContext,
        output_dir: &std::path::PathBuf,
    ) {
        self.create_code_generator()
            .write_napi_lib_sources(self, codegen, output_dir);
    }
}
