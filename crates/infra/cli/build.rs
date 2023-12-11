use anyhow::Result;
use clap::CommandFactory;
use clap_complete::generate;
use clap_complete::shells::Zsh;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::Codegen;

// Need this hack to import the source crate as a module:
#[path = "./src/main.rs"]
mod crate_main;
pub use crate_main::*;

fn main() -> Result<()> {
    generate_zsh_completions()?;

    Ok(())
}

/// Generate auto-completions for the shell.
fn generate_zsh_completions() -> Result<()> {
    let mut buffer = vec![];
    let mut command = <crate_main::commands::CLI as CommandFactory>::command();

    generate(Zsh, &mut command, "infra", &mut buffer);

    let crate_dir = CargoWorkspace::locate_source_crate("infra_cli")?;

    Codegen::write_only()?.write_file(
        crate_dir.join("generated/infra.zsh-completions"),
        String::from_utf8(buffer)?,
    )
}
