use std::env::var;
use std::process::Command;

fn main() {
    // Codegen does not need to run during 'napi build' (when cross-compiling).
    // We already run this expensive step during 'cargo check' validation.
    if var("SLANG_SKIP_CODEGEN_DURING_NAPI_BUILD").is_err() {
        execute_codegen_for_local_development();
    }

    napi_build::setup();
}

fn execute_codegen_for_local_development() {
    let success = Command::new("infra")
        .args(["run", "solidity_npm_build"])
        .spawn()
        .expect("Expected Cargo to spawn successfully.")
        .wait()
        .expect("Cargo failed to start.")
        .success();

    assert!(success, "Failed to run codegen.");
}
