use anyhow::Result;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;

pub fn setup_git() -> Result<()> {
    if !GitHub::is_running_in_ci() {
        println!("No need to modify local dev environments.");
        return Ok(());
    }

    Command::new("git")
        .arg("config")
        .property("user.name", "github-actions")
        .run()?;

    Command::new("git")
        .arg("config")
        .property("user.email", "github-actions@users.noreply.github.com")
        .run()?;

    Ok(())
}
