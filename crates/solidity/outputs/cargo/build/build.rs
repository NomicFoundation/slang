use anyhow::Result;
use codegen_schema::types::LanguageDefinition;
use codegen_syntax::SyntaxGeneratorExtensions;
use codegen_utils::context::CodegenContext;
use solidity_language::SolidityLanguageExtensions;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let language = LanguageDefinition::load_solidity()?;

        let src_dir = codegen
            .repo_root
            .join("crates/solidity/outputs/cargo/crate/src");

        language.generate_legacy_rust_lib_sources(codegen, &src_dir.join("legacy/generated"));

        return Ok(());
    });
}
