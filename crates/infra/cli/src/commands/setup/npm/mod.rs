use infra_utils::commands::Command;
use infra_utils::github::GitHub;

pub fn setup_npm() {
    if GitHub::is_running_in_ci() {
        Command::new("npm").arg("ci").run();
    } else {
        Command::new("npm").arg("install").run();
    }
}
