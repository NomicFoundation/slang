use anyhow::{bail, Result};
use infra_utils::{commands::Command, github::GitHub};

pub fn publish_lock_files() -> Result<()> {
    let local_changes = Command::new("git")
        .arg("status")
        .flag("--short")
        .evaluate()?
        .trim()
        .to_owned();

    if !local_changes.is_empty() {
        bail!("Cannot update lock files. Found local changes:\n{local_changes}");
    }

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

    if local_changes != "M package-lock.json" {
        bail!("Unexpected local changes:\n{local_changes}");
    }

    Command::new("git").arg("diff").flag("--cached").run()?;

    if !GitHub::is_running_in_ci() {
        println!("Skipping the update, since we are not running in CI.");
        return Ok(());
    }

    let base_branch = Command::new("git")
        .arg("branch")
        .flag("--show-current")
        .evaluate()?
        .trim()
        .to_owned();

    let remote = "origin";
    let head_branch = "infra/update-lock-files";

    Command::new("git")
        .arg("checkout")
        .property("-b", head_branch)
        .run()?;

    Command::new("git")
        .args(["add", "package-lock.json"])
        .run()?;

    Command::new("git")
        .property("-c", "user.name=github-actions")
        .property("-c", "user.email=github-actions@users.noreply.github.com")
        .arg("commit")
        .property("--message", "update lock files after release")
        .run()?;

    Command::new("git")
        .arg("push")
        .flag("--force")
        .property("--set-upstream", remote)
        .arg(head_branch)
        .run()?;

    Command::new("git")
        .arg("push")
        .flag("--force")
        .property("--set-upstream", remote)
        .arg(head_branch)
        .run()?;

    Command::new("gh")
        .args(["pr", "create"])
        .flag("--fill")
        .property("--base", &base_branch)
        .property("--head", head_branch)
        .run()?;

    Command::new("git").args(["checkout", &base_branch]).run()?;

    Ok(())
}
