use std::path::Path;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::{
    cargo::CargoWorkspace,
    commands::Command,
    github::GitHub,
    paths::{FileWalker, PathExtensions},
};
use itertools::Itertools;

use crate::utils::{ClapExtensions, OrderedCommand, Terminal};

#[derive(Clone, Debug, Default, Parser)]
pub struct LintController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<LintCommand>,
}

impl LintController {
    pub fn execute(&self) -> Result<()> {
        return LintCommand::execute_in_order(&self.commands);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum LintCommand {
    /// Lints all Rust source files.
    // Automatically applied lints may need to be formatted again, so we run this before formatting.
    Clippy,
    /// Format all Rust source files.
    CargoFmt,
    /// Check for spelling issues in Markdown files.
    Cspell,
    /// Format all non-Rust source files.
    Prettier,
    /// Check for broken links in Markdown files.
    MarkdownLinkCheck,
    /// Check for violations in Markdown files.
    MarkdownLint,
    /// Check for violations in Bash files.
    Shellcheck,
    /// Check for type errors in TypeScript files.
    Tsc,
    /// Check for violations issues in Yaml files.
    Yamllint,
}

impl OrderedCommand for LintCommand {
    fn execute(&self) -> Result<()> {
        Terminal::step(format!("lint {name}", name = self.clap_name()));

        return match self {
            LintCommand::Clippy => run_clippy(),
            LintCommand::CargoFmt => run_cargo_fmt(),
            LintCommand::Cspell => run_cspell(),
            LintCommand::Prettier => run_prettier(),
            LintCommand::MarkdownLinkCheck => run_markdown_link_check(),
            LintCommand::MarkdownLint => run_markdown_lint(),
            LintCommand::Shellcheck => run_shellcheck(),
            LintCommand::Tsc => run_tsc(),
            LintCommand::Yamllint => run_yamllint(),
        };
    }
}

fn run_cargo_fmt() -> Result<()> {
    let mut command = Command::new("cargo-fmt").flag("--all").flag("--verbose");

    if GitHub::is_running_in_ci() {
        command = command.flag("--check");
    }

    return command.run();
}

fn run_clippy() -> Result<()> {
    // To help with gradual adoption, we don't exit on failure and ignore some errors for now.
    let makeshift_config = std::fs::read_to_string(Path::repo_path(".clippy_allowed_lints"))?;
    let allowed_lints = makeshift_config
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .unique();

    let mut clippy = Command::new("cargo")
        .flag("clippy")
        .flag("--")
        .flag("--verbose");

    for lint in allowed_lints {
        clippy = clippy.property("-A", format!("clippy::{}", lint));
    }

    clippy.run()
}

fn run_cspell() -> Result<()> {
    return Command::new("cspell")
        .arg("lint")
        .flag("--show-context")
        .flag("--show-suggestions")
        .flag("--dot")
        .flag("--gitignore")
        .run();
}

fn run_prettier() -> Result<()> {
    return if GitHub::is_running_in_ci() {
        Command::new("prettier").property("--check", ".").run()
    } else {
        Command::new("prettier").property("--write", ".").run()
    };
}

fn run_markdown_link_check() -> Result<()> {
    let config_file = Path::repo_path(".markdown-link-check.json");

    let markdown_files = FileWalker::from_repo_root().find(["**/*.md"])?;

    return Command::new("markdown-link-check")
        .property("--config", config_file.unwrap_str())
        .run_xargs(markdown_files);
}

fn run_markdown_lint() -> Result<()> {
    let markdown_files = FileWalker::from_repo_root()
        .find(["**/*.md"])?
        .into_iter()
        .map(|path| {
            println!("{}", path.display());
            return path;
        });

    let mut command = Command::new("markdownlint").flag("--dot");

    if !GitHub::is_running_in_ci() {
        command = command.flag("--fix");
    }

    return command.run_xargs(markdown_files);
}

fn run_shellcheck() -> Result<()> {
    let bash_files = FileWalker::from_repo_root()
        .find(["scripts/**"])?
        .into_iter()
        .map(|path| {
            println!("{}", path.display());
            return path;
        });

    return Command::new("shellcheck").run_xargs(bash_files);
}

fn run_tsc() -> Result<()> {
    let config_file = Path::repo_path("tsconfig.json");

    return Command::new("tsc")
        .property("--project", config_file.unwrap_str())
        .run();
}

fn run_yamllint() -> Result<()> {
    let config_file = Path::repo_path(".yamllint.yml");

    let yaml_files = FileWalker::from_repo_root()
        .find(["**/*.yml"])?
        .into_iter()
        .map(|path| {
            println!("{}", path.display());
            return path;
        });

    // Run the command next to the Pipfile installing it:
    let crate_dir = CargoWorkspace::locate_source_crate("infra_cli")?;

    return Command::new("python3")
        .property("-m", "pipenv")
        .args(["run", "yamllint"])
        .flag("--strict")
        .property("--config-file", config_file.unwrap_str())
        .current_dir(crate_dir)
        .run_xargs(yaml_files);
}
