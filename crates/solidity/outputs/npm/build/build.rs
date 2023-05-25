use anyhow::Result;
use codegen_schema::types::LanguageDefinition;
use codegen_syntax::SyntaxGeneratorExtensions;
use codegen_utils::context::CodegenContext;
use solidity_language::SolidityLanguageExtensions;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let language = LanguageDefinition::load_solidity()?;

        let output_dir = codegen
            .repo_root
            .join("crates/solidity/outputs/npm/crate/src/generated");

        language.generate_typescript_lib_sources(codegen, &output_dir);

        return Ok(());
    });
}
