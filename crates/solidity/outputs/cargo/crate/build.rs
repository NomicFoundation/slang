//! This build script is only used for local development.
//! It is removed when publishing to crates.io.

use anyhow::Result;
use codegen_grammar::{Grammar, GrammarConstructorDslV2};
use codegen_parser_generator::{AstModel, RustGenerator};
use infra_utils::cargo::CargoWorkspace;
use solidity_language::SolidityDefinition;

fn main() -> Result<()> {
    let language = SolidityDefinition::create();
    let grammar = Grammar::from_dsl_v2(&language);
    let ast_model = AstModel::create(&language);

    RustGenerator::generate(
        &grammar,
        &ast_model,
        &CargoWorkspace::locate_source_crate("slang_solidity")?.join("src/generated"),
    )
}
