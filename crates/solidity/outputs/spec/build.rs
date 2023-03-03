use anyhow::Result;
use codegen_schema::types::grammar::Grammar;
use codegen_spec::GrammarSpecGeneratorExtensions;
use codegen_utils::context::CodegenContext;
use solidity_schema::SolidityGrammarExtensions;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let grammar = Grammar::load_solidity()?;
        let output_dir = codegen
            .repo_root
            .join("crates/solidity/outputs/spec/generated");

        return grammar.generate_spec(codegen, &output_dir);
    });
}
