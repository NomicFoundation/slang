use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::codegen::formatting::{format_source_file, generate_header};
use crate::github::GitHub;
use crate::paths::{FileWalker, PathExtensions};

#[derive(Default)]
pub struct CodegenFileSystem {
    generated_dirs: HashSet<PathBuf>,
    generated_files: HashSet<PathBuf>,
}

impl CodegenFileSystem {
    pub fn write_file_formatted(
        &mut self,
        file_path: impl AsRef<Path>,
        contents: impl AsRef<str>,
    ) -> Result<()> {
        let contents = format_source_file(file_path.as_ref(), contents.as_ref())?;

        self.write_file_raw(file_path, contents)
    }

    pub fn write_file_raw(
        &mut self,
        file_path: impl AsRef<Path>,
        contents: impl AsRef<str>,
    ) -> Result<()> {
        let file_path = file_path.as_ref();
        self.mark_generated_file(file_path)?;

        let contents = if let Some(header) = generate_header(file_path) {
            let contents = contents.as_ref();
            let eol = if contents.contains("\r\n") { "\r\n" } else { "\n" };
            format!("{header}{eol}{eol}{contents}")
        } else {
            contents.as_ref().to_owned()
        };

        if GitHub::is_running_in_ci() {
            verify_contents(file_path, &contents)
        } else {
            write_contents(file_path, &contents)
        }
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
}

impl Drop for CodegenFileSystem {
    fn drop(&mut self) {
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

    // To respect Cargo incrementability, don't touch the file if it is already up to date.
    if file_path.exists() && contents == file_path.read_to_string()? {
        println!("Up to date: {}", file_path.strip_repo_root()?.unwrap_str());
        return Ok(());
    }

    println!("Updating: {}", file_path.strip_repo_root()?.unwrap_str());
    file_path.write_string(contents)
}

fn verify_contents(file_path: &Path, contents: &str) -> Result<()> {
    if !file_path.exists() {
        bail!("Generated file does not exist: {file_path:?}");
    }

    let existing_contents = file_path.read_to_string()?;

    similar_asserts::assert_eq!(
        contents,
        existing_contents,
        "Generated file is out of date: {file_path:?}"
    );

    Ok(())
}
