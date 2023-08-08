use anyhow::Result;
use infra_utils::{cargo::CargoWorkspace, commands::Command};

fn main() -> Result<()> {
    // Reference here to make sure we recompile when that changes:
    use solidity_npm_build as _;

    execute_codegen_for_local_development()?;

    napi_build::setup();

    return Ok(());
}

fn execute_codegen_for_local_development() -> Result<()> {
    let crate_dir = CargoWorkspace::locate_source_crate("solidity_npm_build")?;

    return Command::new("cargo")
        .arg("run")
        .property("--bin", "solidity_npm_build")
        .current_dir(crate_dir) // Run in that directory, using its own 'target' dir to prevent locking.
        .run();
}
