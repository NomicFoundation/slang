use anyhow::Result;
use codegen_schema::types::grammar::Grammar;
use codegen_syntax::GrammarParserGeneratorExtensions;
use codegen_utils::context::CodegenContext;
use solidity_schema::SolidityGrammarExtensions;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let grammar = Grammar::load_solidity()?;

        let output_dir = codegen
            .repo_root
            .join("crates/solidity/outputs/rust/slang_solidity/src/syntax/generated");

        grammar.generate_rust_lib_sources(codegen, &output_dir);
        return Ok(());
    });
}
