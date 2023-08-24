use std::{env::var, process::Command};

fn main() {
    // Codegen does not need to run in production (when users install the crate).
    // We only run that step during local development and in CI.
    if var("IS_RUNNING_IN_NOMIC_FOUNDATION_SLANG_REPO").is_ok() {
        execute_codegen_for_local_development();
    }
}

fn execute_codegen_for_local_development() {
    let success = Command::new("infra")
        .args(["run", "solidity_cargo_build"])
        .spawn()
        .expect("Expected Cargo to spawn successfully.")
        .wait()
        .expect("Cargo failed to start.")
        .success();

    assert!(success, "Failed to run codegen.");
}
