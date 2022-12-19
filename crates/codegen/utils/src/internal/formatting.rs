use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::PathBuf,
};

use anyhow::{anyhow, bail, Context, Result};

use crate::{commands::run_command, context::CodegenContext};

pub fn format_source_file(
    codegen: &CodegenContext,
    file_path: &PathBuf,
    contents: &str,
) -> Result<String> {
    let header = generate_header(file_path)?;
    let unformatted = format!("{header}\n\n{contents}");

    return match formatters::run(codegen, file_path, &unformatted) {
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
                .context(format!("Failed to format {file_path:?}"));
        }
    };
}

fn backup_raw_file(file_path: &PathBuf, unformatted: &str) -> Result<PathBuf> {
    let backup_file_path = {
        let backup_file_name = file_path
            .file_name()
            .context(format!("Cannot get file name: {file_path:?}"))?;

        let backup_file_hash = {
            let mut hasher = DefaultHasher::new();
            unformatted.hash(&mut hasher);
            hasher.finish().to_string()
        };

        // target/{PROFILE}/build/{CRATE_NAME}-{BUILD_HASH}/out
        let crate_output_dir = std::env::var("OUT_DIR")?;

        PathBuf::from(crate_output_dir)
            .join("codegen/formatter-backup")
            .join(backup_file_hash)
            .join(backup_file_name)
    };

    std::fs::create_dir_all(
        backup_file_path
            .parent()
            .context(format!("Cannot get file parent: {backup_file_path:?}"))?,
    )?;

    std::fs::write(&backup_file_path, &unformatted)
        .context(format!("Failed to write back up to: {backup_file_path:?}"))?;

    return Ok(backup_file_path);
}

fn generate_header(file_path: &PathBuf) -> Result<String> {
    let warning_line =
        "This file is generated automatically by infrastructure scripts. Please don't edit by hand.";

    return match get_extension(file_path)? {
        "ebnf" => Ok(format!("(* {warning_line} *)")),
        "md" => Ok(format!("<!-- {warning_line} -->")),
        "rs" => Ok(format!("// {warning_line}")),
        "ts" => Ok(format!("// {warning_line}")),
        "yml" => Ok(format!("# {warning_line}")),
        ext => bail!("Unsupported extension to generate a header for: {ext}"),
    };
}

fn get_extension<'a>(file_path: &'a PathBuf) -> Result<&'a str> {
    return file_path
        .extension()
        .context(format!("Cannot get extension of file: {file_path:?}"))?
        .to_str()
        .context(format!("Cannot read extension of file: {file_path:?}"));
}

mod formatters {
    use super::*;

    pub fn run(codegen: &CodegenContext, file_path: &PathBuf, contents: &str) -> Result<String> {
        return match get_extension(file_path)? {
            "rs" => run_rustfmt(codegen, file_path, contents),
            "md" | "yml" => run_prettier(codegen, file_path, contents),
            "ebnf" => Ok(contents.to_owned()), // we don't format these files (yet)
            ext => bail!("Unsupported extension to format: {ext}"),
        };
    }

    fn run_rustfmt(
        codegen: &CodegenContext,
        file_path: &PathBuf,
        contents: &str,
    ) -> Result<String> {
        let rustfmt_path = codegen.repo_root.join("bin/rustfmt");
        let rustfmt_path = rustfmt_path.to_str().context("Cannot get rustfmt path")?;

        return run_command(
            &codegen.repo_root,
            &[rustfmt_path, "--emit", "stdout"],
            Some(contents),
        )
        .context(format!("Failed to run rustfmt on file: {file_path:?}"));
    }

    fn run_prettier(
        codegen: &CodegenContext,
        file_path: &PathBuf,
        contents: &str,
    ) -> Result<String> {
        let node_path = codegen.repo_root.join("bin/node");
        let node_path = node_path.to_str().context("Cannot get node path")?;

        let prettier_path = codegen.repo_root.join("node_modules/.bin/prettier");

        if !prettier_path.exists() {
            let setup_path = codegen.repo_root.join("scripts/linting/setup.sh");
            let setup_path = setup_path.to_str().context("Cannot get setup.sh path")?;

            run_command(&codegen.repo_root, &[setup_path], None)
                .context(format!("Failed to run setup script: {setup_path:?}"))?;
        }

        let prettier_path = prettier_path.to_str().context("Cannot get prettier path")?;

        let file_path = file_path
            .to_str()
            .context(format!("Cannot get file path: {file_path:?}"))?;

        return run_command(
            &codegen.repo_root,
            &[
                node_path,
                prettier_path,
                "--stdin-filepath", // used to infer the language, and detect `.prettierrc` options
                file_path,
            ],
            Some(contents),
        )
        .context(format!("Failed to run prettier on file: {file_path:?}"));
    }
}
