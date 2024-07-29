use std::path::Path;

use anyhow::{Context, Result};
use clap::CommandFactory;
use clap_complete::Shell;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;

pub fn setup_shell_completions() -> Result<()> {
    if GitHub::is_running_in_ci() {
        println!("No need to generate shell completions in CI.");
        return Ok(());
    }

    println!("Generating shell completions...");
    println!();

    let shell_completions_dir = Path::repo_path("target/shell-completions");
    std::fs::create_dir_all(&shell_completions_dir)
        .context("Failed to create shell completions directory")?;

    let mut cmd = crate::commands::Cli::command();
    let bin_name = cmd.get_bin_name().expect("Set by #[command]").to_owned();

    for shell in [Shell::Bash, Shell::Zsh] {
        let mut out_buf = vec![];
        clap_complete::generate(shell, &mut cmd, &bin_name, &mut out_buf);

        let file = shell_completions_dir.join(format!("infra.{shell}"));
        std::fs::write(&file, out_buf).context("Couldn't write a shell completion file")?;

        let canonicalized = file.canonicalize()?;
        let stripped = canonicalized.strip_repo_root()?;
        println!("Generated {shell} completions at {}", stripped.display());
    }

    Ok(())
}
