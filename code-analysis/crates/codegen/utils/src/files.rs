use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::{context::CodegenContext, formatting::format_source_file};

pub fn read_file(file_path: &PathBuf) -> Result<String> {
    return std::fs::read_to_string(file_path)
        .context(format!("Cannot read source file: {file_path:?}"));
}

pub fn delete_file(file_path: &PathBuf) -> Result<()> {
    return std::fs::remove_file(&file_path)
        .context(format!("Failed to delete extra file: {file_path:?}"));
}

pub fn write_file(codegen: &CodegenContext, file_path: &PathBuf, contents: &str) -> Result<()> {
    let parent_dir = file_path
        .parent()
        .context(format!("Cannot get parent directory of: {file_path:?}"))?;

    std::fs::create_dir_all(parent_dir)
        .context(format!("Cannot create parent directory of: {file_path:?}"))?;

    let formatted = format_source_file(codegen, file_path, &contents)?;

    // To respect Cargo incrementability, don't touch the file if it is already up to date.
    if file_path.exists() {
        let existing_contents = std::fs::read_to_string(file_path)
            .context(format!("Cannot read existing file: {file_path:?}"))?;

        if formatted == existing_contents {
            return Ok(());
        }
    }

    return std::fs::write(file_path, formatted)
        .context(format!("Cannot write generated file: {file_path:?}"));
}

pub fn verify_file(codegen: &CodegenContext, file_path: &PathBuf, contents: &str) -> Result<()> {
    let formatted = format_source_file(codegen, file_path, &contents)?;

    if !file_path.exists() {
        bail!("Generated file does not exist: {file_path:?}");
    }

    let existing_contents = std::fs::read_to_string(file_path)
        .context(format!("Cannot read existing file: {file_path:?}"))?;

    if formatted != existing_contents {
        bail!("Generated file is out of date: {file_path:?}");
    }

    return Ok(());
}

pub fn collect_files_recursively(parent_dir: &Path, result: &mut Vec<PathBuf>) -> Result<()> {
    if !parent_dir.metadata()?.is_dir() {
        bail!("Path is not a directory: {parent_dir:?}");
    }

    for child in parent_dir.read_dir()? {
        let child = child?;
        let child_path = child.path();

        if child.metadata()?.is_file() {
            result.push(child_path);
        } else {
            collect_files_recursively(&child_path, result)?;
        }
    }

    return Ok(());
}

pub fn rerun_if_changed(file_path: &PathBuf) -> Result<()> {
    println!(
        "cargo:rerun-if-changed={value}",
        value = file_path.to_str().context("Failed to get file path")?
    );

    return Ok(());
}
