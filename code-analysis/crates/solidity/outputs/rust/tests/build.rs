use codegen_schema::Grammar;
use codegen_testing::GrammarTestingGeneratorExtensions;
use codegen_utils::context::CodegenContext;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    CodegenContext::with_context(|codegen| {
        // Load Manifest
        let grammar = Grammar::from_manifest(
            codegen,
            &codegen
                .repo_root
                .join("code-analysis/crates/solidity/inputs/schema/manifest.yml"),
        );

        // Generate Tests
        grammar.generate_cst_output_tests(
            codegen,
            &codegen
                .repo_root
                .join("code-analysis/crates/solidity/test_data/cst_output"),
            &codegen
                .repo_root
                .join("code-analysis/crates/solidity/outputs/rust/tests/src/cst_output/generated"),
        )?;

        return Ok(());
    })?;

    return Ok(());
}
