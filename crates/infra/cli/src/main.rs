#![allow(unused_crate_dependencies)] // dependencies are almost exclusively used by the lib target

use clap::Parser;
use infra_cli::{Terminal, CLI};

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
