use std::path::PathBuf;

use crate::{commands::run_command, context::CodegenContext};

use anyhow::{bail, Context, Result};

pub(crate) fn generate_header(file_path: &PathBuf) -> Result<String> {
    let warning_line = "This file is generated via `cargo build`. Please don't edit by hand.";

    return match get_extension(file_path)? {
        "ebnf" => Ok(format!("(* {warning_line} *)")),
        "md" => Ok(format!("<!-- {warning_line} -->")),
        "rs" => Ok(format!("// {warning_line}")),
        "yml" => Ok(format!("# {warning_line}")),
        ext => bail!("Unsupported extension to generate a header for: {ext}"),
    };
}

pub(crate) fn format_source_file(
    codegen: &CodegenContext,
    file_path: &PathBuf,
    contents: &str,
) -> Result<String> {
    return match get_extension(file_path)? {
        "rs" => run_rustfmt_binary(codegen, file_path, contents),
        "md" | "yml" => run_prettier_binary(codegen, file_path, contents),
        "ebnf" => Ok(contents.to_owned()), // we don't format these files (yet)
        ext => bail!("Unsupported extension to format: {ext}"),
    };
}

fn get_extension<'a>(file_path: &'a PathBuf) -> Result<&'a str> {
    return file_path
        .extension()
        .context(format!("Cannot get extension of file: {file_path:?}"))?
        .to_str()
        .context(format!("Cannot read extension of file: {file_path:?}"));
}

fn run_rustfmt_binary(
    codegen: &CodegenContext,
    file_path: &PathBuf,
    contents: &str,
) -> Result<String> {
    return run_command(
        codegen,
        &vec!["rustfmt", "--emit", "stdout"],
        Some(contents),
    )
    .context(format!("Failed to run rustfmt on file: {file_path:?}"));
}

fn run_prettier_binary(
    codegen: &CodegenContext,
    file_path: &PathBuf,
    contents: &str,
) -> Result<String> {
    let node_path = codegen.repo_dir.join("bin/node");

    let prettier_path = codegen
        .repo_dir
        .join("infrastructure/node_modules/.bin/prettier");

    if !prettier_path.exists() {
        let setup_path = codegen.repo_dir.join("infrastructure/scripts/setup.sh");
        bail!("Failed to find prettier binary at: {prettier_path:?}. Please run {setup_path:?} to install it.");
    }

    return run_command(
        codegen,
        &vec![
            node_path.to_str().context("Failed to get path")?,
            prettier_path.to_str().context("Failed to get path")?,
            "--stdin-filepath", // used to infer the language, and detect `.prettierrc` options
            file_path.to_str().context("Failed to read file path")?,
        ],
        Some(contents),
    )
    .context(format!("Failed to run prettier on file: {file_path:?}"));
}
