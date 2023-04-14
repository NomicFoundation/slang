pub mod syntax;

mod _supress_binary_dependencies_ {
    // Below are dependencies used by the binary `main.rs`, but not here.
    // However, we need to add a fake usage below to supress Cargo warnings about unused dependencies.
    // This is a known issue, and we should remove this hack once there is a better solution from Cargo.
    // https://github.com/rust-lang/cargo/issues/1982
    use anyhow as _;
    use clap as _;
    use serde_json as _;
    #[cfg(test)]
    use solidity_cargo_build as _;
    use thiserror as _;
}
