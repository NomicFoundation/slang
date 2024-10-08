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

    match (file_path.unwrap_name(), file_path.unwrap_ext()) {
        // Known names:
        (".gitignore", _) => format!("# {warning_line}"),

        // Known extensions:
        (_, "ebnf") => format!("(* {warning_line} *)"),
        (_, "json") => String::new(),
        (_, "html" | "md") => format!("<!-- {warning_line} -->"),
        (_, "dot" | "js" | "mts" | "rs" | "ts" | "wit") => format!("// {warning_line}"),
        (_, "yml" | "txt") => format!("# {warning_line}"),
        (_, "mmd") => format!("%% {warning_line}"),

        _ => panic!("Unsupported path to generate a header for: {file_path:?}"),
    }
}

fn run_formatter(file_path: &Path, contents: &str) -> Result<String> {
    match (file_path.unwrap_name(), file_path.unwrap_ext()) {
        // We have formatters available for these:
        (_, "js" | "json" | "mts" | "ts") => run_prettier(file_path, contents),
        (_, "rs") => run_rustfmt(contents),

        // No formatters available for these yet:
        (".gitignore", _) => Ok(contents.to_owned()),
        (_, "dot" | "ebnf" | "mmd" | "txt" | "wit") => Ok(contents.to_owned()),

        // We already generate formatted content for these, so no need to run expensive formatting:
        (_, "html" | "md" | "yml") => Ok(contents.to_owned()),

        _ => panic!("Unsupported path to format: {file_path:?}"),
    }
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
