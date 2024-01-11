use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::Generator;

/// Generate shell completions for this CLI.
#[derive(Clone, Debug, Parser)]
pub struct CompletionController {
    /// The shell to generate the completions for
    #[arg(value_enum)]
    shell: clap_complete::Shell,
}

impl CompletionController {
    #[allow(clippy::unnecessary_wraps)] // for consistency with other commands
    pub fn execute(&self) -> Result<()> {
        let mut command = crate::commands::CLI::command();
        command.build(); // Required to generate completions

        self.shell.generate(&command, &mut std::io::stdout());

        Ok(())
    }
}
