use codegen_schema::Grammar;
use codegen_testing::GrammarTestingGeneratorExtensions;
use codegen_utils::context::CodegenContext;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?)
        .join("../../../../../..")
        .canonicalize()?;

    CodegenContext::with_context(repo_dir, |codegen| {
        // Load Manifest
        let grammar = Grammar::from_manifest(
            codegen,
            &codegen
                .repo_dir
                .join("code-analysis/crates/solidity/inputs/schema/manifest.yml"),
        );

        // Generate Tests
        grammar.generate_cst_output_tests(
            codegen,
            &codegen
                .repo_dir
                .join("code-analysis/crates/solidity/test_data/cst_output"),
            &codegen
                .repo_dir
                .join("code-analysis/crates/solidity/outputs/rust/tests/src/cst_output/generated"),
        )?;

        return Ok(());
    })?;

    return Ok(());
}
