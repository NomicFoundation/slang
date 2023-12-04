use anyhow::Result;
use infra_utils::{commands::Command, github::GitHub};

pub fn setup_cargo() -> Result<()> {
    let mut command = Command::new("cargo").arg("fetch");

    if GitHub::is_running_in_ci() {
        // In CI, run with '--locked' to make sure `Cargo.lock` is up to date.
        // Don't use '--frozen', because the cache is rarely up to date.
        command = command.flag("--locked");
    }

    command.run()
}
