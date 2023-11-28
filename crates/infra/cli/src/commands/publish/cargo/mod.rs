use anyhow::Result;
use infra_utils::{cargo::CargoWorkspace, commands::Command, github::GitHub};

const USER_FACING_CRATE: &str = "slang_solidity";

pub fn publish_cargo() -> Result<()> {
    let local_version = CargoWorkspace::local_version()?;
    println!("Local version: {local_version}");

    let published_version = CargoWorkspace::published_version(USER_FACING_CRATE)?;
    println!("Published version: {published_version}");

    if local_version == published_version {
        println!("Skipping crate, since the local version is already published.");
        return Ok(());
    }

    let mut command = Command::new("cargo")
        .arg("publish")
        .property("--package", USER_FACING_CRATE)
        .flag("--all-features");

    if !GitHub::is_running_in_ci() {
        println!("Attempting a dry run, since we are not running in CI.");
        command = command.flag("--dry-run");
    }

    return command.run();
}
