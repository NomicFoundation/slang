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
    generate_completions(&[Shell::Zsh, Shell::Bash])
}

/// Generate auto-completions for the shell.
fn generate_completions(shells: &[Shell]) -> Result<()> {
    let mut codegen = Codegen::write_only()?;

    let mut command = CLI::command();
    command.build();

    for shell in shells {
        let mut buffer = vec![];

        shell.generate(&command, &mut buffer);

        let crate_dir = CargoWorkspace::locate_source_crate("infra_cli")?;

        codegen.write_file(
            crate_dir.join(format!("generated/infra.{shell}-completions")),
            String::from_utf8(buffer)?,
        )?;
    }

    Ok(())
}
