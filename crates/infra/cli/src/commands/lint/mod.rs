use std::path::Path;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::{FileWalker, PathExtensions};
use infra_utils::terminal::Terminal;

use crate::toolchains::pipenv::PipEnv;
use crate::utils::{ClapExtensions, OrderedCommand};

#[derive(Clone, Debug, Default, Parser)]
pub struct LintController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<LintCommand>,
}

impl LintController {
    pub fn execute(&self) -> Result<()> {
        LintCommand::execute_in_order(&self.commands)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum LintCommand {
    /// Check for spelling issues in Markdown files.
    Cspell,
    /// Format all non-Rust source files.
    Prettier,
    /// Check for broken links in Markdown files.
    MarkdownLinkCheck,
    /// Check for violations in Markdown files.
    MarkdownLint,
    /// Format all Rust source files.
    Rustfmt,
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

        match self {
            LintCommand::Cspell => run_cspell(),
            LintCommand::Prettier => run_prettier(),
            LintCommand::MarkdownLinkCheck => run_markdown_link_check()?,
            LintCommand::MarkdownLint => run_markdown_lint()?,
            LintCommand::Rustfmt => run_rustfmt(),
            LintCommand::Shellcheck => run_shellcheck()?,
            LintCommand::Tsc => run_tsc(),
            LintCommand::Yamllint => run_yamllint()?,
        };

        Ok(())
    }
}

fn run_cspell() {
    Command::new("cspell")
        .arg("lint")
        .flag("--show-context")
        .flag("--show-suggestions")
        .flag("--dot")
        .flag("--gitignore")
        .run();
}

fn run_prettier() {
    if GitHub::is_running_in_ci() {
        Command::new("prettier").property("--check", ".").run();
    } else {
        Command::new("prettier").property("--write", ".").run();
    }
}

fn run_markdown_link_check() -> Result<()> {
    let config_file = Path::repo_path(".markdown-link-check.json");

    let markdown_files = FileWalker::from_repo_root().find(["**/*.md"])?;

    Command::new("markdown-link-check")
        .property("--config", config_file.unwrap_str())
        .run_xargs(markdown_files);

    Ok(())
}

fn run_markdown_lint() -> Result<()> {
    let markdown_files = FileWalker::from_repo_root()
        .find(["**/*.md"])?
        .inspect(|path| println!("{}", path.display()));

    let mut command = Command::new("markdownlint").flag("--dot");

    if !GitHub::is_running_in_ci() {
        command = command.flag("--fix");
    }

    command.run_xargs(markdown_files);

    Ok(())
}

fn run_rustfmt() {
    let mut command = Command::new("cargo-fmt")
        .arg(format!("+{}", env!("RUST_NIGHTLY_VERSION")))
        .flag("--all")
        .flag("--verbose");

    if GitHub::is_running_in_ci() {
        command = command.flag("--check");
    }

    command.run();
}

fn run_shellcheck() -> Result<()> {
    let bash_files = FileWalker::from_repo_root()
        .find(["scripts/**"])?
        .inspect(|path| println!("{}", path.display()));

    Command::new("shellcheck").run_xargs(bash_files);

    Ok(())
}

fn run_tsc() {
    let root_project = Path::repo_path("tsconfig.json");

    Command::new("tsc")
        .property("--build", root_project.unwrap_str())
        .run();
}

fn run_yamllint() -> Result<()> {
    let config_file = Path::repo_path(".yamllint.yml");

    let yaml_files = FileWalker::from_repo_root()
        .find(["**/*.yml"])?
        .inspect(|path| println!("{}", path.display()));

    PipEnv::run("yamllint")
        .flag("--strict")
        .property("--config-file", config_file.unwrap_str())
        .run_xargs(yaml_files);

    Ok(())
}
