use anyhow::Result;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;

pub fn setup_npm() -> Result<()> {
    if GitHub::is_running_in_ci() {
        Command::new("npm").arg("install").flag("--ci").run()
    } else {
        Command::new("npm").arg("install").run()
    }
}
