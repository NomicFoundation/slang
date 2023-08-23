pub mod commands;
pub mod toolchains;
pub mod utils;

use clap::Parser;

use crate::{commands::CLI, utils::Terminal};

#[allow(dead_code)] // as it is referenced from 'build.rs' of the same crate.
fn main() {
    let std_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        // Print panic to stderr first, including any backtraces.
        std_hook(info);

        // A lot of external build commands can exit without clearly indicating success or failure.
        // This acts as a catch all to make sure they don't go unnoticed.
        Terminal::failure();
    }));

    CLI::parse().execute().unwrap();
    Terminal::success();
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <CLI as clap::CommandFactory>::command().debug_assert();
}
