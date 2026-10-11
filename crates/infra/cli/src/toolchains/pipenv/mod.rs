use infra_utils::commands::Command;

pub struct PipEnv;

impl PipEnv {
    #[must_use]
    pub fn run(name: impl Into<String>) -> Command {
        Command::new("python3")
            .property("-m", "pipenv")
            .args(["run", &name.into()])
    }
}
