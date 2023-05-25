use anyhow::Result;
use codegen_schema::types::LanguageDefinition;
use codegen_spec::SpecGeneratorExtensions;
use codegen_utils::context::CodegenContext;
use solidity_language::SolidityLanguageExtensions;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let language = LanguageDefinition::load_solidity()?;
        let output_dir = codegen
            .repo_root
            .join("crates/solidity/outputs/spec/generated");

        return language.generate_spec(codegen, &output_dir);
    });
}
