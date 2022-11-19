use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};

use crate::files::{self, get_generated_dir};

pub struct CodegenContext {
    generated_dirs: HashSet<PathBuf>,
    generated_files: HashSet<PathBuf>,
    validate_generated_files: bool,

    pub repo_dir: PathBuf,
}

impl CodegenContext {
    pub fn with_context(
        repo_dir: PathBuf,
        operation: impl FnOnce(&mut Self) -> Result<()>,
    ) -> Result<()> {
        let mut context = Self {
            generated_dirs: HashSet::new(),
            generated_files: HashSet::new(),
            validate_generated_files: std::env::var("SLANG_VALIDATE_GENERATED_FILES").is_ok(),

            repo_dir,
        };

        operation(&mut context)?;

        context.post_validation()?;

        return Ok(());
    }

    pub fn read_file(&self, file_path: &PathBuf) -> Result<String> {
        self.rerun_if_changed(file_path)?;

        return files::read_file(file_path);
    }

    pub fn write_file(&mut self, file_path: &PathBuf, contents: &str) -> Result<()> {
        if !file_path.starts_with(&self.repo_dir) {
            bail!("Generated file is outside repository: {file_path:?}");
        }

        if let Ok(generated_dir) = get_generated_dir(file_path) {
            self.rerun_if_changed(generated_dir)?;
            self.generated_dirs.insert(generated_dir.to_owned());
        } else {
            bail!("All generated files should be under a 'generated' parent dir: {file_path:?}");
        }

        if !self.generated_files.insert(file_path.to_owned()) {
            bail!("File was generated twice: {file_path:?}")
        }

        return if self.validate_generated_files {
            files::verify_file(self, file_path, contents)
        } else {
            files::write_file(self, file_path, contents)
        };
    }

    fn post_validation(&self) -> Result<()> {
        for generated_dir in &self.generated_dirs {
            files::check_for_extra_files(generated_dir, &self.generated_files)?;
        }

        return Ok(());
    }

    pub fn rerun_if_changed(&self, file_path: &Path) -> Result<()> {
        println!(
            "cargo:rerun-if-changed={value}",
            value = file_path.to_str().context("Failed to get file path")?
        );

        return Ok(());
    }
}
