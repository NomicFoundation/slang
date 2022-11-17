use codegen_schema::Grammar;
use codegen_spec::GrammarSpecExtensions;
use codegen_utils::context::CodegenContext;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let repo_dir = crate_dir.join("../../../../..").canonicalize()?;

    CodegenContext::with_context(repo_dir.clone(), |codegen| {
        // Load Manifest
        let grammar = Grammar::from_manifest(
            codegen,
            &repo_dir.join("code-analysis/crates/solidity/inputs/schema/manifest.yml"),
        );

        // Generate Spec
        grammar.generate_spec(codegen, &crate_dir);

        return Ok(());
    })?;

    return Ok(());
}
