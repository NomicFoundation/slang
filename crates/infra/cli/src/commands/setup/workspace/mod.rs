use anyhow::Result;
use infra_utils::{commands::Command, github::GitHub};

pub fn setup_workspace() -> Result<()> {
    if !GitHub::is_running_in_ci() {
        warm_up_ide_tools()?;
    }

    Ok(())
}

/// Warm up IDE tools in case it was a fresh installation.
fn warm_up_ide_tools() -> Result<()> {
    Command::new("rust-analyzer").flag("--version").run()?;
    Command::new("rust-src").flag("--version").run()?;

    Ok(())
}
