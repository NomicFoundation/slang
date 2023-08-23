use anyhow::Result;
use infra_utils::{commands::Command, github::GitHub};

pub fn setup_cargo() -> Result<()> {
    return if GitHub::is_running_in_ci() {
        Command::new("cargo").arg("fetch").flag("--locked").run()
    } else {
        Command::new("cargo").arg("fetch").run()
    };
}
