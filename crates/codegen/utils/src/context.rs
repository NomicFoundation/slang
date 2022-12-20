use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use walkdir::WalkDir;

use crate::internal::files::{self, calculate_repo_root};

pub struct CodegenContext {
    input_dirs: HashSet<PathBuf>,
    generated_dirs: HashSet<PathBuf>,
    generated_files: HashSet<PathBuf>,
    check_only: bool,

    pub repo_root: PathBuf,
}

impl CodegenContext {
    pub fn with_context(operation: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {
        let mut context = Self {
            input_dirs: HashSet::new(),
            generated_dirs: HashSet::new(),
            generated_files: HashSet::new(),
            check_only: std::env::var("CI").is_ok(),

            repo_root: calculate_repo_root()?,
        };

        operation(&mut context)?;

        context.validate_orphaned_files()?;
        context.print_cargo_markers();

        return Ok(());
    }

    pub fn mark_input_dir(&mut self, path: &PathBuf) {
        // Skip if same path (or a parent) is already marked
        for input_dir in &self.input_dirs {
            if path.starts_with(input_dir) {
                return;
            }
        }

        // Remove existing children of input_path
        self.input_dirs.retain(|input_dir| {
            return !input_dir.starts_with(path);
        });

        self.input_dirs.insert(path.to_owned());
    }

    pub fn read_file(&mut self, file_path: &PathBuf) -> Result<String> {
        self.mark_input_dir(&file_path.parent().unwrap().to_path_buf());

        return files::read_file(file_path);
    }

    pub fn write_file(&mut self, file_path: &PathBuf, contents: &str) -> Result<()> {
        if !file_path.starts_with(&self.repo_root) {
            bail!("Generated file is outside repository: {file_path:?}");
        }

        if !self.generated_files.insert(file_path.to_owned()) {
            bail!("File was generated twice: {file_path:?}")
        }

        if let Some(generated_dir) = self.get_generated_dir(file_path) {
            self.generated_dirs.insert(generated_dir.to_owned());
        } else {
            bail!("All generated files should be under a 'generated' parent dir: {file_path:?}");
        }

        return if self.check_only {
            files::verify_file(self, file_path, contents)
        } else {
            files::write_file(self, file_path, contents)
        };
    }

    pub fn get_generated_dir(&self, file_path: &Path) -> Option<PathBuf> {
        let generated_index = file_path.iter().enumerate().find(|p| p.1 == "generated")?.0;
        let generated_dir: PathBuf = file_path.iter().take(generated_index + 1).collect();
        return Some(generated_dir);
    }

    fn validate_orphaned_files(&self) -> Result<()> {
        for generated_dir in &self.generated_dirs {
            for entry in WalkDir::new(generated_dir) {
                let entry = entry?;
                if entry.file_type().is_dir() {
                    continue;
                }

                let file = entry.into_path();
                if self.generated_files.contains(&file) {
                    continue;
                }

                if self.check_only {
                    bail!("File was not generated in this context: {file:?}");
                } else {
                    files::delete_file(&file)?;
                }
            }
        }

        return Ok(());
    }

    fn print_cargo_markers(&self) {
        if std::env::var("RUSTC").is_err() {
            // This environment variable is only set for Cargo build tasks.
            // When it is missing, it means we are running from tests, and we shouldn't print these markers.
            return;
        }

        for input_dir in &self.input_dirs {
            println!("cargo:rerun-if-changed={}", input_dir.to_str().unwrap());
        }

        for generated_dir in &self.generated_dirs {
            println!("cargo:rerun-if-changed={}", generated_dir.to_str().unwrap());
        }
    }
}
