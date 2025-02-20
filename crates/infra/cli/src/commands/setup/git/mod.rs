use infra_utils::commands::Command;
use infra_utils::github::GitHub;

pub fn setup_git() {
    if GitHub::is_running_in_ci() {
        Command::new("git")
            .arg("config")
            .property("user.name", "github-actions")
            .run();

        Command::new("git")
            .arg("config")
            .property("user.email", "github-actions@users.noreply.github.com")
            .run();
    }

    Command::new("git")
        .args(["submodule", "update"])
        .flag("--init")
        .flag("--progress")
        .run();
}
