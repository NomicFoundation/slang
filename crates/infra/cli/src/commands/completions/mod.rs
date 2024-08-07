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
    pub fn execute(&self) {
        let mut command = crate::commands::Cli::command();
        command.build(); // Required to generate completions

        self.shell.generate(&command, &mut std::io::stdout());
    }
}
