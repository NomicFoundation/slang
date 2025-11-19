use infra_utils::commands::Command;
use infra_utils::github::GitHub;

pub fn setup_npm() {
    let mut command = Command::new("pnpm").arg("install");

    if GitHub::is_running_in_ci() {
        command = command.flag("--frozen-lockfile");
    }

    command.run();
}
