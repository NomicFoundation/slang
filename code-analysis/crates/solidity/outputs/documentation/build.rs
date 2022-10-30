use codegen_schema::Grammar;
use codegen_spec::GrammarSpecGeneratorExtensions;
use codegen_utils::context::CodegenContext;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let repo_dir = crate_dir.join("../../../../..").canonicalize()?;
    let grammar_file = repo_dir.join("code-analysis/crates/solidity/inputs/schema/manifest.yml");
    let output_dir = repo_dir.join("documentation");

    CodegenContext::with_context(repo_dir.clone(), |codegen| {
        let grammar = Grammar::from_manifest(codegen, &grammar_file);
        grammar.generate_spec(codegen, &output_dir);
        return Ok(());
    })?;

    return Ok(());
}
