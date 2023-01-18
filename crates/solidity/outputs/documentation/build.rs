use codegen_schema::grammar::Grammar;
use codegen_spec::GrammarSpecGeneratorExtensions;
use codegen_utils::context::CodegenContext;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    CodegenContext::with_context(|codegen| {
        let grammar_file = codegen
            .repo_root
            .join("crates/solidity/inputs/schema/grammar/manifest.yml");

        let output_dir = codegen.repo_root.join("documentation");

        let grammar = Grammar::from_manifest(codegen, &grammar_file);
        grammar.generate_spec(codegen, &output_dir);
        return Ok(());
    })?;

    return Ok(());
}
