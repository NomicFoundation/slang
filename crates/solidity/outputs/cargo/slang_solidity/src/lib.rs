mod generated;
mod user_defined;

pub use generated::*;

// Below are dependencies used by the CLI "main.rs", but not the API `lib.rs`.
// However, we need to add a fake usage to suppress Cargo warnings about unused dependencies.
// This is a known issue, and we should remove this hack once there is a better solution from Cargo.
// https://github.com/rust-lang/cargo/issues/1982
#[cfg(feature = "cli")]
mod supress_cli_dependencies {
    use {anyhow as _, ariadne as _, clap as _, serde_json as _};
}
