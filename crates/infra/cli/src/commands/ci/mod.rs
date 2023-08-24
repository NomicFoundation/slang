use anyhow::Result;
use clap::Parser;

use crate::commands::{
    check::CheckController, lint::LintController, setup::SetupController, test::TestController,
};

#[derive(Clone, Debug, Parser)]
pub struct CiController;

impl CiController {
    pub fn execute(&self) -> Result<()> {
        // Run all CI steps in order: _SLANG_INFRA_CI_STEPS_ORDERED_ (keep in sync)

        SetupController::default().execute()?;
        CheckController::default().execute()?;
        TestController::default().execute()?;
        LintController::default().execute()?;

        return Ok(());
    }
}
