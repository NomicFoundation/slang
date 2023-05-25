use anyhow::Result;
use codegen_schema::types::LanguageDefinition;
use codegen_testing::TestingGeneratorExtensions;
use codegen_utils::context::CodegenContext;
use solidity_language::SolidityLanguageExtensions;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let language = LanguageDefinition::load_solidity()?;

        language.generate_cst_output_tests(
            codegen,
            &codegen
                .repo_root
                .join("crates/solidity/testing/snapshots/cst_output"),
            &codegen
                .repo_root
                .join("crates/solidity/outputs/cargo/tests/src/cst_output/generated"),
        )?;

        return Ok(());
    });
}
