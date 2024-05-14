use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use cargo_emit::{rerun_if_changed, warning};

use crate::cargo::CargoWorkspace;
use crate::codegen::formatting::format_source_file;
use crate::github::GitHub;
use crate::paths::{FileWalker, PathExtensions};

pub struct CodegenFileSystem {
    input_dir: PathBuf,
    generated_dirs: HashSet<PathBuf>,
    generated_files: HashSet<PathBuf>,
}

impl CodegenFileSystem {
    pub fn new(input_dir: impl Into<PathBuf>) -> Result<Self> {
        let input_dir = input_dir.into();

        let input_dir = input_dir
            .canonicalize()
            .with_context(|| format!("Directory doesn't exist: {input_dir:?}"))?;

        Ok(Self {
            input_dir,
            generated_dirs: HashSet::new(),
            generated_files: HashSet::new(),
        })
    }

    pub fn read_file(&mut self, file_path: impl AsRef<Path>) -> Result<String> {
        let file_path = file_path.as_ref();

        if !file_path.starts_with(&self.input_dir) {
            bail!(
                "File path {:?} is not under input directory {:?}",
                file_path,
                self.input_dir
            );
        }

        file_path.read_to_string()
    }

    pub fn write_file(
        &mut self,
        file_path: impl AsRef<Path>,
        contents: impl AsRef<str>,
    ) -> Result<()> {
        let file_path = file_path.as_ref();

        self.mark_generated_file(file_path)?;

        return if GitHub::is_running_in_ci() {
            verify_contents(file_path, contents.as_ref())
        } else {
            write_contents(file_path, contents.as_ref())
        };
    }

    pub fn mark_generated_file(&mut self, file_path: impl AsRef<Path>) -> Result<()> {
        let file_path = file_path.as_ref();

        if file_path.strip_repo_root().is_err() {
            bail!("Generated file is outside repository: {file_path:?}");
        }

        if !self.generated_files.insert(file_path.to_owned()) {
            bail!("File was generated twice: {file_path:?}")
        }

        self.generated_dirs.insert(file_path.generated_dir()?);

        Ok(())
    }

    pub fn copy_file(
        &mut self,
        source_path: impl AsRef<Path>,
        destination_path: impl AsRef<Path>,
    ) -> Result<()> {
        // Go through read_file() API, to record the correct metadata for it.
        let contents = self.read_file(source_path)?;

        // Go through write_file() API, to only touch/update the file if it changed.
        self.write_file(destination_path, contents)?;

        Ok(())
    }
}

impl Drop for CodegenFileSystem {
    fn drop(&mut self) {
        if CargoWorkspace::is_running_inside_build_scripts() {
            rerun_if_changed!(self.input_dir.unwrap_str());

            for generated_dir in &self.generated_dirs {
                rerun_if_changed!(generated_dir.unwrap_str());
            }
        }

        for generated_dir in &self.generated_dirs {
            for file_path in FileWalker::from_directory(generated_dir)
                .find_all()
                .unwrap()
            {
                if self.generated_files.contains(&file_path) {
                    continue;
                }

                if GitHub::is_running_in_ci() {
                    panic!("File was not generated in this context: {file_path:?}");
                } else {
                    std::fs::remove_file(&file_path)
                        .with_context(|| format!("Failed to delete source file: {file_path:?}"))
                        .unwrap();
                }
            }
        }
    }
}

fn write_contents(file_path: &Path, contents: &str) -> Result<()> {
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

fn verify_contents(file_path: &Path, contents: &str) -> Result<()> {
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
