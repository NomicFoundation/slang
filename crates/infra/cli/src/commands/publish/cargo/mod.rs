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

const USER_FACING_CRATES: &[&str] = &[
    // Sorted by dependency order (from dependencies to dependents):
    "metaslang_cst",
    "metaslang_graph_builder",
    "metaslang_bindings",
    "slang_solidity",
];

pub fn publish_cargo(dry_run: DryRun) -> Result<()> {
    let mut changeset = TemporaryChangeset::new(
        "infra/cargo-publish",
        "prepare Cargo packages for publishing",
    )?;

    let mut changed_crates = vec![];

    for crate_name in USER_FACING_CRATES {
        if prepare_for_publish(crate_name, &mut changeset)? {
            changed_crates.push(crate_name);
        }
    }

    if changed_crates.is_empty() {
        println!("No crates to publish.");
        return Ok(());
    }

    update_cargo_lock(&mut changeset)?;

    changeset.commit_changes()?;

    for crate_name in &changed_crates {
        run_cargo_publish(crate_name, dry_run)?;
    }

    changeset.revert_changes()?;

    Ok(())
}

fn prepare_for_publish(crate_name: &str, changeset: &mut TemporaryChangeset) -> Result<bool> {
    if let Ok(published_version) = CargoWorkspace::published_version(crate_name) {
        println!("Published version of {crate_name}: {published_version}");

        let local_version = CargoWorkspace::local_version()?;
        println!("Local version: {local_version}");

        if local_version == published_version {
            println!("Skipping crate {crate_name}, since the local version is already published.");
            return Ok(false);
        }
    } else {
        println!("No published version found for crate {crate_name}.");
    }

    let crate_dir = &CargoWorkspace::locate_source_crate(crate_name)?;

    let cargo_toml = crate_dir.join("Cargo.toml");
    if strip_publish_markers(&cargo_toml)? {
        changeset.expect_change(&cargo_toml);
    }

    let build_rs = crate_dir.join("build.rs");
    if build_rs.exists() {
        std::fs::remove_file(&build_rs)?;
        changeset.expect_change(&build_rs);
    }

    Ok(true)
}

fn strip_publish_markers(cargo_toml_path: &Path) -> Result<bool> {
    let old_contents = std::fs::read_to_string(cargo_toml_path)?;

    let new_contents = old_contents
        .lines()
        .filter(|line| !line.contains("__REMOVE_THIS_LINE_DURING_CARGO_PUBLISH__"))
        .chain(once(""))
        .join("\n");

    let contents_changed = new_contents != old_contents;
    if contents_changed {
        std::fs::write(cargo_toml_path, new_contents)?;
    }

    Ok(contents_changed)
}

fn update_cargo_lock(changeset: &mut TemporaryChangeset) -> Result<()> {
    let cargo_lock_path = Path::repo_path("Cargo.lock");
    let old_contents = std::fs::read_to_string(&cargo_lock_path)?;

    Command::new("cargo").arg("check").run()?;

    let new_contents = std::fs::read_to_string(&cargo_lock_path)?;

    if new_contents != old_contents {
        changeset.expect_change(&cargo_lock_path);
    }

    Ok(())
}

fn run_cargo_publish(crate_name: &str, dry_run: DryRun) -> Result<()> {
    let mut command = Command::new("cargo")
        .arg("publish")
        .property("--package", crate_name)
        .flag("--all-features");

    if dry_run.is_yes() || !GitHub::is_running_in_ci() {
        println!(
            "Attempting a dry run, since we are not running in CI or a dry run was requested."
        );
        command = command.flag("--dry-run");
    }

    command.run()
}
