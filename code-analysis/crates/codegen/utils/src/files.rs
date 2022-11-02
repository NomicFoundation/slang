use std;
use std::path::PathBuf;

use crate::{
    context::CodegenContext,
    formatting::{format_source_file, generate_header},
};

use anyhow::{bail, Context, Result};

impl CodegenContext {
    pub fn read_file(&self, file_path: &PathBuf) -> Result<String> {
        rerun_if_changed(file_path)?;

        return std::fs::read_to_string(file_path)
            .context(format!("Cannot read source file: {file_path:?}"));
    }

    pub fn write_file(&self, file_path: &PathBuf, contents: &str) -> Result<()> {
        rerun_if_changed(file_path)?;

        let parent_dir = file_path
            .parent()
            .context(format!("Cannot get parent directory of: {file_path:?}"))?;

        std::fs::create_dir_all(parent_dir)
            .context(format!("Cannot create parent directory of: {file_path:?}"))?;

        let header = generate_header(file_path)?;
        let contents = format!("{header}\n\n{contents}");

        let formatted = match format_source_file(self, file_path, &contents) {
            Ok(formatted) => formatted,
            Err(error) => {
                std::fs::write(file_path, contents)
                    .context(format!("Cannot write generated file: {file_path:?}"))?;

                bail!(error);
            }
        };

        if file_path.exists() {
            let existing_contents = std::fs::read_to_string(file_path)
                .context(format!("Cannot read existing file: {file_path:?}"))?;

            // To respect Cargo incrementability, don't touch the file if it is already up to date.
            if formatted == existing_contents {
                return Ok(());
            }
        }

        return std::fs::write(file_path, formatted)
            .context(format!("Cannot write generated file: {file_path:?}"));
    }

    pub fn delete_file(&self, file_path: &PathBuf) -> Result<()> {
        rerun_if_changed(file_path)?;

        return match file_path.exists() {
            false => Ok(()),
            true => std::fs::remove_file(file_path)
                .context(format!("Cannot delete generated file: {file_path:?}")),
        };
    }
}

fn rerun_if_changed(file_path: &PathBuf) -> Result<()> {
    let file_path = file_path.to_str().context("Failed to get file path")?;
    println!("cargo:rerun-if-changed={file_path}");
    return Ok(());
}
