use std::path::Path;

use anyhow::{Context, Result};

use crate::commands::Command;
use crate::paths::PathExtensions;

pub fn format_source_file(file_path: &Path, contents: &str) -> Result<String> {
    match run_formatter(file_path, contents) {
        Ok(formatted) => Ok(formatted),
        Err(formatter_error) => {
            // Still write the unformatted version to disk, to be able to debug what went wrong:
            file_path.write_string(contents)?;

            Err(formatter_error).context(format!("Failed to format {file_path:?}"))
        }
    }
}

pub fn generate_header(file_path: &Path) -> Option<String> {
    let warning_line =
        "This file is generated automatically by infrastructure scripts. Please don't edit by hand.";

    Some(match (file_path.unwrap_name(), file_path.unwrap_ext()) {
        (_, "ebnf") => format!("(* {warning_line} *)"),
        (_, "html" | "md") => format!("<!-- {warning_line} -->"),
        (_, "dot" | "js" | "mts" | "rs" | "sol" | "ts" | "wit" | "lalrpop") => {
            format!("// {warning_line}")
        }
        (_, "yml" | "txt") => format!("# {warning_line}"),
        (_, "mmd") => format!("%% {warning_line}"),

        // Does not support comments:
        (_, "json") => return None,

        _ => panic!("Unsupported path to generate a header for: {file_path:?}"),
    })
}

fn run_formatter(file_path: &Path, contents: &str) -> Result<String> {
    match (file_path.unwrap_name(), file_path.unwrap_ext()) {
        // We have formatters available for these:
        (_, "js" | "json" | "mts" | "ts") => run_prettier(file_path, contents),
        (_, "rs") => run_rustfmt(contents),

        // No formatters available for these yet:
        (_, "wit" | "lalrpop") => Ok(contents.to_owned()),

        _ => panic!("Unsupported path to format: {file_path:?}"),
    }
}

fn run_prettier(file_path: &Path, contents: &str) -> Result<String> {
    Command::new("prettier")
        .property(
            // used to infer the language, and detect `.prettierrc` options
            "--stdin-filepath",
            file_path.unwrap_str(),
        )
        .evaluate_with_input(contents)
}

fn run_rustfmt(contents: &str) -> Result<String> {
    Command::new("rustfmt")
        .arg(format!("+{}", env!("RUST_NIGHTLY_VERSION")))
        .property("--emit", "stdout")
        .evaluate_with_input(contents)
}
