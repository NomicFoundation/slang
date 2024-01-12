pub mod commands;
pub mod toolchains;
pub mod utils;

pub use crate::commands::CLI;
pub use crate::utils::Terminal;

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <CLI as clap::CommandFactory>::command().debug_assert();
}
