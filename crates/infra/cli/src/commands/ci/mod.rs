use anyhow::Result;
use clap::Parser;
use infra_utils::commands::Command;

use crate::commands::check::CheckController;
use crate::commands::setup::SetupController;
use crate::commands::test::TestController;

#[derive(Clone, Debug, Parser)]
pub struct CiController;

impl CiController {
    #[allow(clippy::unused_self)] // For symmetry with other commands
    pub fn execute(&self) -> Result<()> {
        // Note: setup is ran implicitly during devcontainer initialization:
        SetupController::default().execute()?;

        // Run all CI steps in order: _SLANG_INFRA_CI_STEPS_ORDERED_ (keep in sync)
        CheckController::default().execute()?;
        TestController::default().execute();
        Command::new("task").arg("lint").run();

        Ok(())
    }
}
