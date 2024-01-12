use std::path::Path;

use anyhow::Result;
use infra_utils::commands::Command;
use infra_utils::git::TemporaryChangeset;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;

pub fn publish_lock_files() -> Result<()> {
    let head_branch = "infra/update-lock-files";

    let mut changeset = TemporaryChangeset::new(head_branch, "update lock files after release")?;

    Command::new("npm")
        .arg("install")
        .flag("--package-lock-only")
        .run()?;

    let local_changes = Command::new("git")
        .arg("status")
        .flag("--short")
        .evaluate()?
        .trim()
        .to_owned();

    if local_changes.is_empty() {
        println!("No changes to lock files.");
        return Ok(());
    }

    if !GitHub::is_running_in_ci() {
        println!("Skipping the update, since we are not running in CI.");
        return Ok(());
    }

    changeset.expect_change(Path::repo_path("package-lock.json"));

    changeset.commit_changes()?;

    Command::new("git")
        .arg("push")
        .flag("--force")
        .property("--set-upstream", "origin")
        .arg(head_branch)
        .run()?;

    Command::new("gh")
        .args(["pr", "create"])
        .flag("--fill")
        .property("--base", "main")
        .property("--head", head_branch)
        .run()?;

    changeset.revert_changes()?;

    Ok(())
}
