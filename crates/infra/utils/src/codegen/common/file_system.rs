use std::path::Path;

use anyhow::{bail, Context, Result};
use cargo_emit::warning;

use crate::{
    cargo::CargoWorkspace, codegen::common::formatting::format_source_file, paths::PathExtensions,
};

pub fn delete_file(file_path: &Path) -> Result<()> {
    std::fs::remove_file(file_path)
        .with_context(|| format!("Failed to delete source file: {file_path:?}"))
}

pub fn write_file(file_path: &Path, contents: &str) -> Result<()> {
    std::fs::create_dir_all(file_path.unwrap_parent())
        .with_context(|| format!("Cannot create parent directory of: {file_path:?}"))?;

    let formatted = format_source_file(file_path, contents)?;

    // To respect Cargo incrementability, don't touch the file if it is already up to date.
    if file_path.exists() && formatted == file_path.read_to_string()? {
        return Ok(());
    }

    if CargoWorkspace::is_running_inside_build_scripts() {
        warning!("Updating: {}", file_path.strip_repo_root()?.unwrap_str());
    }

    file_path.write_string(formatted)
}

pub fn verify_file(file_path: &Path, contents: &str) -> Result<()> {
    let formatted = format_source_file(file_path, contents)?;

    if !file_path.exists() {
        bail!("Generated file does not exist: {file_path:?}");
    }

    let existing_contents = file_path.read_to_string()?;

    similar_asserts::assert_eq!(
        formatted,
        existing_contents,
        "Generated file is out of date: {file_path:?}"
    );

    Ok(())
}
