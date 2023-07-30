mod commands;
mod toolchains;
mod utils;

use clap::Parser;

use crate::{commands::AppController, utils::Terminal};

fn main() {
    let std_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        // Print panic to stderr first, including any backtraces.
        std_hook(info);

        // A lot of external build commands can exit without clearly indicating success or failure.
        // This acts as a catch all to make sure they don't go unnoticed.
        Terminal::failure();
    }));

    AppController::parse().execute().unwrap();
    Terminal::success();
}
