use anyhow::Result;
use cargo_emit::rerun_if_changed;
use codegen_grammar::Grammar;
use codegen_parser_generator::code_generator::CodeGenerator;
use infra_utils::{cargo::CargoWorkspace, paths::PathExtensions};
use solidity_language::GrammarConstructor;

// Instead of the soure crate calling codegen APIs directly, it invokes this binary, which in turn calls the codegen APIs.
// This indirection is needed because:
//
// 1) We want to run codegen, even if there are git conflicts in output files (Cargo fails to build in that case).
// 2) In order to avoid build graph deadlocks, this binary uses its own local `target` folder here.
// 3) We want to avoid having dependencies from the source crate to codegen crates.
//
fn main() -> Result<()> {
    // The parser generator itself affects the build, so make sure we rerun the build script if it changes:
    {
        let codegen_dir = CargoWorkspace::locate_source_crate("codegen_parser_generator")?;

        rerun_if_changed!(codegen_dir.unwrap_str());
    }

    // Generate files in the source crate:

    {
        let grammar = Grammar::new();
        let crate_dir = CargoWorkspace::locate_source_crate("solidity_npm_crate")?;

        CodeGenerator::write_source(&crate_dir.join("src/generated"), grammar)?;
    }

    // Instruct the caller source crate to rebuild itself if the this build crate changes:

    {
        let build_crate_dir = CargoWorkspace::locate_source_crate("solidity_npm_build")?;

        rerun_if_changed!(build_crate_dir.join("Cargo.toml").unwrap_str());
        rerun_if_changed!(build_crate_dir.join("src").unwrap_str());
    }

    return Ok(());
}
