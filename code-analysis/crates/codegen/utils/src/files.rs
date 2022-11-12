use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};

use crate::{context::CodegenContext, formatting::format_source_file};

pub fn read_file(file_path: &PathBuf) -> Result<String> {
    return std::fs::read_to_string(file_path)
        .context(format!("Cannot read source file: {file_path:?}"));
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

pub fn check_for_extra_files(current_dir: &Path, generated_files: &HashSet<PathBuf>) -> Result<()> {
    if !current_dir.metadata()?.is_dir() {
        bail!("Path is not a directory: {current_dir:?}");
    }

    for child in current_dir.read_dir()? {
        let child = child?;
        let child_path = child.path();

        if child.metadata()?.is_file() {
            if !generated_files.contains(&child.path()) {
                bail!("Extra file in generated dir: {child_path:?}");
            }
        } else {
            check_for_extra_files(&child_path, generated_files)?;
        }
    }

    return Ok(());
}

pub fn get_generated_dir<'a>(path: &'a Path) -> Result<&'a Path> {
    return if path.is_dir() && path.ends_with("generated") {
        Ok(path)
    } else {
        path.parent()
            .context(format!("Failed to find generated parent dir of: {path:?}"))
            .and_then(get_generated_dir)
    };
}
