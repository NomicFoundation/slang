use anyhow::Result;
use cargo_emit::rerun_if_changed;

use codegen_grammar::Grammar;
use codegen_parser_generator::code_generator::CodeGenerator;
use infra_utils::{cargo::CargoWorkspace, paths::PathExtensions};
use solidity_language::GrammarConstructorDslV2;

// Instead of the soure crate calling codegen APIs directly in the build script, it invokes this binary, which in turn
// calls the codegen APIs (and hence why it's emitting `cargo:` directives).
// This indirection is needed because:
//
// 1) We want to run codegen, even if there are git conflicts in output files (Cargo fails to build in that case).
// 2) In order to avoid build graph deadlocks, this binary uses its own local `target` folder here.
// 3) We want to avoid having dependencies from the source crate to codegen crates.
//
fn main() -> Result<()> {
    // Generate files in the source crate:
    {
        let grammar = solidity_language::LANGUAGE_DEF.with(Grammar::from_dsl_v2);

        let crate_dir = CargoWorkspace::locate_source_crate("slang_solidity")?;

        CodeGenerator::write_source(&crate_dir.join("src/generated"), &grammar)?;
    }

    // This build script is not directly depended on by the caller source crate, so we need to manually
    // instruct Cargo on our invocation to rebuild if any of our dependencies (or we) change:
    for crate_name in &[
        env!("CARGO_CRATE_NAME"),
        "codegen_grammar",
        "codegen_parser_generator",
        "solidity_language",
    ] {
        let crate_dir = CargoWorkspace::locate_source_crate(crate_name)?;

        // This is not accurate, but it's good enough for now.
        rerun_if_changed!(crate_dir.join("Cargo.toml").unwrap_str());
        rerun_if_changed!(crate_dir.join("src").unwrap_str());
        // Emitting the `rerun-if-changed` for non-existent files always causes a rebuild, so first check if a build
        // script event exists. It's worth noting that adding/removing one will trip this up.
        if crate_dir.join("build.rs").exists() {
            rerun_if_changed!(crate_dir.join("build.rs").unwrap_str());
        }
    }

    Ok(())
}
