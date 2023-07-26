use anyhow::Result;
use codegen_grammar::Grammar;
use codegen_parser_generator::code_generator::CodeGenerator;
use infra_utils::cargo::CargoWorkspace;
use solidity_language::GrammarConstructor;

// Instead of the soure crate calling codegen APIs directly, it invokes this binary, which in turn calls the codegen APIs.
// This indirection is needed because:
//
// 1) We want to run codegen, even if there are git conflicts in output files (Cargo fails to build in that case).
// 2) In order to avoid build graph deadlocks, this binary uses its own local `target` folder here.
// 3) We want to avoid having dependencies from the source crate to codegen crates.
//
fn main() -> Result<()> {
    let crate_dir = CargoWorkspace::locate_source_crate("solidity_npm_crate")?;
    CodeGenerator::write_source(&crate_dir.join("src/generated"), Grammar::new())?;
    return Ok(());
}
