pub mod generated;

mod _supress_binary_dependencies_ {
    // Below are dependencies used by the binary `main.rs`, but not here.
    // However, we need to add a fake usage below to supress Cargo warnings about unused dependencies.
    // This is a known issue, and we should remove this hack once there is a better solution from Cargo.
    use anyhow as _;
    use clap as _;
    use serde_json as _;
    #[cfg(test)]
    use solidity_rust_build as _;
    use thiserror as _;
}
