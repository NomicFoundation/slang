mod legacy;

pub mod language;
pub mod syntax;

// Below are dependencies used by the CLI "main.rs", but not the API `lib.rs`.
// However, we need to add a fake usage to suppress Cargo warnings about unused dependencies.
// This is a known issue, and we should remove this hack once there is a better solution from Cargo.
// https://github.com/rust-lang/cargo/issues/1982
#[cfg(feature = "cli")]
mod supress_cli_dependencies {
    use anyhow as _;
    use clap as _;
    use serde_json as _;
}

// Make sure codegen runs before building for tests.
#[cfg(test)]
use solidity_cargo_build as _;
