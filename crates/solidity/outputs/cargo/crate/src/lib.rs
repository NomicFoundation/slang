mod generated;
pub use generated::*;

// Below are dependencies used by the CLI "main.rs", but not the API `lib.rs`.
// However, we need to add a fake usage to suppress Cargo warnings about unused dependencies.
// This is a known issue, and we should remove this hack once there is a better solution from Cargo.
// https://github.com/rust-lang/cargo/issues/1982
#[cfg(feature = "cli")]
mod supress_cli_dependencies {
    use {anyhow as _, clap as _, serde_json as _};
}

// Make sure the NAPI crate is rebuilt when the Rust crate is updated.
// Unfortunately, the NAPI types cannot be enabled in 'slang_solidity' directly, as 'napi build' crashes
// when they are imported into other crates, so 'solidity_npm_crate' cannot have any dependants.
#[cfg(feature = "slang_napi_interfaces")]
mod supress_napi_dependencies {
    use slang_solidity as _;
}
