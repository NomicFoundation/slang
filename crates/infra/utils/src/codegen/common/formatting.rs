use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};

use crate::{cargo::CargoWorkspace, commands::Command, paths::PathExtensions};

pub fn format_source_file(file_path: &Path, contents: &str) -> Result<String> {
    let header = generate_header(file_path);
    let unformatted = format!("{header}\n\n{contents}");

    return match run_formatter(file_path, &unformatted) {
        Ok(formatted) => Ok(formatted),
        Err(formatter_error) => {
            // Try to backup the unformatted version to disk, to be able to debug what went wrong.
            let backup_error = match backup_raw_file(file_path, &unformatted) {
                Ok(backup_file_path) => {
                    anyhow!("The raw unformatted version was backed up to: {backup_file_path:?}")
                }
                Err(backup_error) => backup_error.context("Failed to back up unformatted version"),
            };

            return Err(formatter_error)
                .context(backup_error)
                .with_context(|| format!("Failed to format {file_path:?}"));
        }
    };
}

fn backup_raw_file(file_path: &Path, unformatted: &str) -> Result<PathBuf> {
    let backup_file_path = {
        let backup_file_hash = {
            let mut hasher = DefaultHasher::new();
            unformatted.hash(&mut hasher);
            hasher.finish().to_string()
        };

        CargoWorkspace::locate_source_crate("infra_utils")?
            .join("target/formatter_backup")
            .join(backup_file_hash)
            .join(file_path.unwrap_name())
    };

    std::fs::create_dir_all(backup_file_path.unwrap_parent())?;

    backup_file_path.write_string(unformatted)?;

    return Ok(backup_file_path);
}

fn generate_header(file_path: &Path) -> String {
    let warning_line =
        "This file is generated automatically by infrastructure scripts. Please don't edit by hand.";

    return match get_extension(file_path) {
        "json" => format!(""),
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
    return Command::new("rustfmt")
        .property("--emit", "stdout")
        .evaluate_with_input(contents);
}
