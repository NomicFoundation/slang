use infra_utils::commands::Command;

pub fn setup_git() {
    Command::new("git")
        .args(["submodule", "update"])
        .flag("--init")
        .flag("--progress")
        .run();
}
