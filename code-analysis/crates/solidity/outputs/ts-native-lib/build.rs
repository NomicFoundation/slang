extern crate napi_build;
use codegen_ebnf::GrammarEBNFExtensions;
use codegen_parser::GrammarChumskyExtensions;
use codegen_schema::Grammar;
use codegen_utils::context::CodegenContext;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    napi_build::setup();
    let repo_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?)
        .join("../../../../..")
        .canonicalize()?;

    CodegenContext::with_context(repo_dir, |codegen| {
        // Load Manifest
        let grammar = Grammar::from_manifest(
            codegen,
            &codegen
                .repo_dir
                .join("code-analysis/crates/solidity/inputs/schema/manifest.yml"),
        );

        // Generate Parser
        grammar.generate_chumsky(
            codegen,
            &codegen
                .repo_dir
                .join("code-analysis/crates/solidity/outputs/ts-native-lib/src/generated"),
        );

        // Generate EBNF Grammar
        grammar.generate_ebnf(
            codegen,
            &codegen
                .repo_dir
                .join("code-analysis/crates/solidity/outputs/ts-native-lib/src/generated"),
        );

        return Ok(());
    })?;

    return Ok(());
}
