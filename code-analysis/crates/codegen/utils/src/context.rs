use std::{collections::HashSet, path::PathBuf};

use anyhow::{bail, Result};

use crate::files;

pub struct CodegenContext {
    generated_dirs: HashSet<PathBuf>,
    generated_files: HashSet<PathBuf>,
    check_only: bool,

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
            check_only: std::env::var("SLANG_CODEGEN_CHECK_ONLY").is_ok(),

            repo_dir,
        };

        operation(&mut context)?;

        context.post_validation()?;

        return Ok(());
    }

    pub fn read_file(&self, file_path: &PathBuf) -> Result<String> {
        files::rerun_if_changed(file_path)?;

        return files::read_file(file_path);
    }

    pub fn write_file(&mut self, file_path: &PathBuf, contents: &str) -> Result<()> {
        if !file_path.starts_with(&self.repo_dir) {
            bail!("Generated file is outside repository: {file_path:?}");
        }

        if !self.generated_files.insert(file_path.to_owned()) {
            bail!("File was generated twice: {file_path:?}")
        }

        if let Some(generated_dir) = self.get_generated_dir(file_path) {
            if self.generated_dirs.insert(generated_dir.to_owned()) {
                files::rerun_if_changed(&generated_dir)?;
            }
        } else {
            bail!("All generated files should be under a 'generated' parent dir: {file_path:?}");
        }

        return if self.check_only {
            files::verify_file(self, file_path, contents)
        } else {
            files::write_file(self, file_path, contents)
        };
    }

    pub fn collect_files_recursively(&self, parent_dir: &PathBuf) -> Result<Vec<PathBuf>> {
        files::rerun_if_changed(parent_dir)?;

        let mut files = vec![];
        files::collect_files_recursively(parent_dir, &mut files)?;

        return Ok(files);
    }

    pub fn get_generated_dir<'a>(&self, file_path: &PathBuf) -> Option<PathBuf> {
        let generated_index = file_path.iter().enumerate().find(|p| p.1 == "generated")?.0;

        let generated_dir = file_path
            .iter()
            .take(generated_index + 1)
            .collect::<PathBuf>();

        return Some(generated_dir);
    }

    fn post_validation(&self) -> Result<()> {
        let mut files = vec![];
        for generated_dir in &self.generated_dirs {
            files::collect_files_recursively(generated_dir, &mut files)?;
        }

        for file in &files {
            if !self.generated_files.contains(file) {
                if self.check_only {
                    bail!("File was not generated in this context: {file:?}");
                } else {
                    files::delete_file(file)?;
                }
            }
        }

        return Ok(());
    }
}
