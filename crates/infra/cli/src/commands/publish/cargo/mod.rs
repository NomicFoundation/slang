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

const USER_FACING_CRATES: &[&str] = &["metaslang_cst", "slang_solidity"];

pub fn publish_cargo(dry_run: DryRun) -> Result<()> {
    let mut changeset = TemporaryChangeset::new(
        "infra/cargo-publish",
        "prepare Cargo packages for publishing",
    )?;

    let local_version = CargoWorkspace::local_version()?;
    println!("Local version: {local_version}");

    let mut changed_crates = vec![];
    {
        for user_facing_crate in USER_FACING_CRATES {
            if let Ok(published_version) = CargoWorkspace::published_version(user_facing_crate) {
                println!("Published version of {user_facing_crate}: {published_version}");

                if local_version == published_version {
                    println!("Skipping crate {user_facing_crate}, since the local version is already published.");
                    continue;
                }
            } else {
                println!("No published version found for crate {user_facing_crate}.");
            }

            changed_crates.push(user_facing_crate);

            let crate_dir = &CargoWorkspace::locate_source_crate(user_facing_crate)?;

            {
                let cargo_toml = crate_dir.join("Cargo.toml");
                if strip_publish_markers(&cargo_toml)? {
                    changeset.expect_change(&cargo_toml);
                }
            }

            {
                let build_rs = crate_dir.join("build.rs");
                if build_rs.exists() {
                    std::fs::remove_file(&build_rs)?;
                    changeset.expect_change(&build_rs);
                }
            }
        }

        if changed_crates.is_empty() {
            println!("No crates to publish.");
            return Ok(());
        }
    }

    {
        let path = Path::repo_path("Cargo.lock");
        let old_contents = std::fs::read_to_string(&path)?;

        for user_facing_crate in &changed_crates {
            update_cargo_lock(user_facing_crate)?;
        }

        let new_contents = std::fs::read_to_string(&path)?;
        if old_contents != new_contents {
            changeset.expect_change(&path);
        }
    }

    changeset.commit_changes()?;

    for user_facing_crate in &changed_crates {
        run_cargo_publish(user_facing_crate, dry_run)?;
    }

    changeset.revert_changes()?;

    Ok(())
}

fn strip_publish_markers(cargo_toml: &Path) -> Result<bool> {
    let contents = std::fs::read_to_string(cargo_toml)?;

    let new_contents = contents
        .lines()
        .filter(|line| !line.contains("__REMOVE_THIS_LINE_DURING_CARGO_PUBLISH__"))
        .chain(once(""))
        .join("\n");
    let contents_changed = new_contents != contents;

    if contents_changed {
        std::fs::write(cargo_toml, new_contents)?;
    }

    Ok(contents_changed)
}

fn update_cargo_lock(user_facing_crate: &str) -> Result<()> {
    let mut command = Command::new("cargo").arg("check");
    command = command.property("--package", user_facing_crate.to_owned());
    command.run()
}

fn run_cargo_publish(user_facing_crate: &str, dry_run: DryRun) -> Result<()> {
    let mut command = Command::new("cargo").arg("publish");
    command = command.property("--package", user_facing_crate.to_owned());
    command = command.flag("--all-features");

    if dry_run.is_yes() || !GitHub::is_running_in_ci() {
        println!(
            "Attempting a dry run, since we are not running in CI or a dry run was requested."
        );
        command = command.flag("--dry-run");
    }

    command.run()
}
