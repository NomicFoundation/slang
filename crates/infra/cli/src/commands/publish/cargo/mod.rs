use std::iter::once;
use std::path::Path;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::git::TemporaryChangeset;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use itertools::Itertools;

use crate::commands::publish::DryRun;

const USER_FACING_CRATE: &str = "slang_solidity";

pub fn publish_cargo(dry_run: DryRun) -> Result<()> {
    let local_version = CargoWorkspace::local_version()?;
    println!("Local version: {local_version}");

    let published_version = CargoWorkspace::published_version(USER_FACING_CRATE)?;
    println!("Published version: {published_version}");

    if local_version == published_version {
        println!("Skipping crate, since the local version is already published.");
        return Ok(());
    }

    let crate_dir = &CargoWorkspace::locate_source_crate(USER_FACING_CRATE)?;

    let mut changeset = TemporaryChangeset::new(
        "infra/cargo-publish",
        "prepare Cargo packages for publishing",
    )?;

    {
        let cargo_toml = crate_dir.join("Cargo.toml");
        strip_publish_markers(&cargo_toml)?;
        changeset.expect_change(&cargo_toml);
    }

    {
        let build_rs = crate_dir.join("build.rs");
        std::fs::remove_file(&build_rs)?;
        changeset.expect_change(&build_rs);
    }

    {
        update_cargo_lock()?;
        changeset.expect_change(Path::repo_path("Cargo.lock"));
    }

    changeset.commit_changes()?;

    run_cargo_publish(dry_run)?;

    changeset.revert_changes()?;

    Ok(())
}

fn strip_publish_markers(cargo_toml: &Path) -> Result<()> {
    let contents = std::fs::read_to_string(cargo_toml)?;

    let contents = contents
        .lines()
        .filter(|line| !line.contains("__REMOVE_THIS_LINE_DURING_CARGO_PUBLISH__"))
        .chain(once(""))
        .join("\n");

    std::fs::write(cargo_toml, contents)?;

    Ok(())
}

fn update_cargo_lock() -> Result<()> {
    Command::new("cargo")
        .arg("check")
        .property("--package", USER_FACING_CRATE)
        .run()
}

fn run_cargo_publish(dry_run: DryRun) -> Result<()> {
    let mut command = Command::new("cargo")
        .arg("publish")
        .property("--package", USER_FACING_CRATE)
        .flag("--all-features");

    if dry_run.is_yes() || !GitHub::is_running_in_ci() {
        println!(
            "Attempting a dry run, since we are not running in CI or a dry run was requested."
        );
        command = command.flag("--dry-run");
    }

    command.run()
}
