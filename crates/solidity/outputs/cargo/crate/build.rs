use std::{env::var, path::PathBuf, process::Command};

fn main() {
    // Codegen does not need to run in production (when users install the crate).
    // We only run that step during local development and in CI.
    if var("IS_RUNNING_IN_NOMIC_FOUNDATION_SLANG_REPO").is_ok() {
        execute_codegen_for_local_development();
    }
}

fn execute_codegen_for_local_development() {
    let repo_root: PathBuf = var("REPO_ROOT")
        .expect("Expected $REPO_ROOT to be defined for build tasks.")
        .into();

    let cargo_bin = repo_root.join("bin/cargo");
    if !cargo_bin.exists() {
        panic!("Cannot locate Cargo binary within the repository.");
    }

    let crate_dir = repo_root.join("crates/solidity/outputs/cargo/build");
    if !crate_dir.exists() {
        panic!("Cannot locate build crate within the repository.");
    }

    let success = Command::new(cargo_bin)
        .args(["run", "--bin", "solidity_cargo_build"])
        .current_dir(crate_dir) // Run in that directory, using its own 'target' dir to prevent locking.
        .spawn()
        .expect("Expected Cargo to spawn successfully.")
        .wait()
        .unwrap()
        .success();

    assert!(success, "Failed to run codegen.");
}
