use std::path::Path;

use anyhow::{Context, Result};

use crate::commands::Command;
use crate::paths::PathExtensions;

pub fn format_source_file(file_path: &Path, contents: &str) -> Result<String> {
    let header = generate_header(file_path);
    let unformatted = format!("{header}\n\n{contents}");

    match run_formatter(file_path, &unformatted) {
        Ok(formatted) => Ok(formatted),
        Err(formatter_error) => {
            // Still write the unformatted version to disk, to be able to debug what went wrong:
            file_path.write_string(unformatted)?;

            Err(formatter_error).context(format!("Failed to format {file_path:?}"))
        }
    }
}

fn generate_header(file_path: &Path) -> String {
    let warning_line =
        "This file is generated automatically by infrastructure scripts. Please don't edit by hand.";

    return match get_extension(file_path) {
        "json" => String::new(),
        "html" | "md" => format!("<!-- {warning_line} -->"),
        "js" | "rs" | "ts" => format!("// {warning_line}"),
        "yml" | "zsh-completions" => format!("# {warning_line}"),
        ext => panic!("Unsupported extension to generate a header for: {ext}"),
    };
}

fn get_extension(file_path: &Path) -> &str {
    return file_path
        .extension()
        .with_context(|| format!("Cannot get extension of file: {file_path:?}"))
        .unwrap()
        .to_str()
        .with_context(|| format!("Cannot read extension of file: {file_path:?}"))
        .unwrap();
}

fn run_formatter(file_path: &Path, contents: &str) -> Result<String> {
    return match get_extension(file_path) {
        "js" | "json" | "ts" => run_prettier(file_path, contents),
        "rs" => run_rustfmt(contents),
        "html" | "md" | "yml" => {
            // We already generate formatted content for these, so no need to run expensive formatting.
            Ok(contents.to_owned())
        }
        "zsh-completions" => {
            // No formatters available for these (yet).
            Ok(contents.to_owned())
        }
        ext => {
            panic!("Unsupported extension to format: {ext}");
        }
    };
}

fn run_prettier(file_path: &Path, contents: &str) -> Result<String> {
    return Command::new("prettier")
        .property(
            // used to infer the language, and detect `.prettierrc` options
            "--stdin-filepath",
            file_path.unwrap_str(),
        )
        .evaluate_with_input(contents);
}

fn run_rustfmt(contents: &str) -> Result<String> {
    Command::new("rustfmt")
        .arg(format!("+{}", env!("RUST_NIGHTLY_VERSION")))
        .property("--emit", "stdout")
        .evaluate_with_input(contents)
}
