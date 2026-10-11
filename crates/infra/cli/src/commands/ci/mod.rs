use clap::Parser;
use infra_utils::commands::Command;

#[derive(Clone, Debug, Parser)]
pub struct CiController;

impl CiController {
    #[allow(clippy::unused_self)] // For symmetry with other commands
    pub fn execute(&self) {
        // Run all CI steps in order: _SLANG_INFRA_CI_STEPS_ORDERED_ (keep in sync)
        Command::new("task").arg("setup").run();
        Command::new("task").arg("check").run();
        Command::new("task").arg("test").run();
        Command::new("task").arg("lint").run();
    }
}
