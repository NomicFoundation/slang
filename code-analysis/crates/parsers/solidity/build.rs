use codegen_ebnf::GrammarEBNFExtensions;
use codegen_parser::GrammarChumskyExtensions;
use codegen_schema::Grammar;
use codegen_spec::GrammarSpecExtensions;
use codegen_utils::context::CodegenContext;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let this_crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).canonicalize()?;
    let repo_dir = this_crate_dir.join("../../../..").canonicalize()?;

    CodegenContext::with_context(repo_dir, |codegen| {
        // Load Manifest
        let manifest_input = this_crate_dir.join("grammar/manifest.yml");
        let grammar = Grammar::from_manifest(codegen, &manifest_input);

        // Generate Grammar
        let grammar_dir = this_crate_dir.join("src/generated");
        grammar.generate_chumsky(codegen, &grammar_dir);
        grammar.generate_ebnf(codegen, &grammar_dir.join("derived.ebnf"));

        // Generate Spec
        let documentation_dir = codegen.repo_dir.join("documentation");
        grammar.generate_spec(codegen, &documentation_dir);

        return Ok(());
    })?;

    return Ok(());
}
