use anyhow::Result;
use codegen_schema::types::grammar::Grammar;
use codegen_testing::GrammarTestingGeneratorExtensions;
use codegen_utils::context::CodegenContext;
use solidity_schema::SolidityGrammarExtensions;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let grammar = Grammar::load_solidity()?;

        grammar.generate_cst_output_tests(
            codegen,
            &codegen
                .repo_root
                .join("crates/solidity/testing/snapshots/cst_output"),
            &codegen
                .repo_root
                .join("crates/solidity/outputs/rust/tests/src/cst_output/generated"),
        )?;

        return Ok(());
    });
}
