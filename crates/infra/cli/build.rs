use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{Generator, Shell};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::Codegen;

// We use the `clap_complete` pattern of sharing the same crate entry point between
// the main lib/bin target and the build script to generate the completions at compile-time.
// See <https://docs.rs/clap_complete/latest/clap_complete/generator/fn.generate_to.html>.
include!("src/lib.rs");

fn main() -> Result<()> {
    generate_completions(Shell::Zsh)
}

/// Generate auto-completions for the shell.
fn generate_completions(shell: Shell) -> Result<()> {
    let mut buffer = vec![];

    let mut command = CLI::command();
    command.build();

    shell.generate(&command, &mut buffer);

    let crate_dir = CargoWorkspace::locate_source_crate("infra_cli")?;

    Codegen::write_only()?.write_file(
        crate_dir.join(format!("generated/infra.{shell}-completions")),
        String::from_utf8(buffer)?,
    )
}
