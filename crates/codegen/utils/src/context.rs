use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Result};
use walkdir::WalkDir;

use crate::{errors::CodegenErrors, internal::files};

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

            repo_root: PathBuf::from(std::env::var("REPO_ROOT")?),
        };

        if let Err(error) = operation(&mut context) {
            if let Some(errors) = error.downcast_ref::<CodegenErrors>() {
                Self::process_errors(errors);
            } else {
                bail!(error);
            }
        }

        context.validate_orphaned_files()?;
        context.print_cargo_markers();

        return Ok(());
    }

    pub fn track_input_dir(&mut self, path: &PathBuf) {
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
        for input_dir in &self.input_dirs {
            if file_path.starts_with(input_dir) {
                return files::read_file(file_path);
            }
        }

        bail!(
            anyhow!("Input file is not under any tracked directory: {file_path:?}")
                .context("Please use CodegenContext::track_input_dir() to ensure all input files are tracked correctly."),
        );
    }

    pub fn copy_file(&mut self, source_path: &PathBuf, destination_path: &PathBuf) -> Result<()> {
        // Go through read_file() API, to record the correct metadata for it.
        let contents = self.read_file(source_path)?;

        // Go through write_file() API, to only touch/update the file if it changed.
        self.write_file(destination_path, &contents)?;

        return Ok(());
    }

    pub fn write_file(&mut self, file_path: &PathBuf, contents: &str) -> Result<()> {
        if !file_path.starts_with(&self.repo_root) {
            bail!("Generated file is outside repository: {file_path:?}");
        }

        if !self.generated_files.insert(file_path.to_owned()) {
            bail!("File was generated twice: {file_path:?}")
        }

        let generated_dirs = self.get_generated_dir(file_path)?;
        self.generated_dirs.insert(generated_dirs);

        return if self.check_only {
            files::verify_file(self, file_path, contents)
        } else {
            files::write_file(self, file_path, contents)
        };
    }

    pub fn get_generated_dir(&self, file_path: &Path) -> Result<PathBuf> {
        let generated_indexes: Vec<_> = file_path
            .iter()
            .enumerate()
            .filter(|p| p.1 == "generated")
            .collect();

        let generated_index = match generated_indexes.len() {
            0 => bail!("Generated file path should have a 'generated' ancestor dir: {file_path:?}"),
            1 => generated_indexes[0].0,
            _ => bail!("Multiple 'generated' dirs in path: {file_path:?}"),
        };

        let generated_dir: PathBuf = file_path.iter().take(generated_index + 1).collect();
        return Ok(generated_dir);
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

    fn process_errors(errors: &CodegenErrors) -> ! {
        let count = errors.len() as i32;
        assert_ne!(count, 0);

        eprintln!();
        eprintln!("{errors}");

        eprintln!();
        eprintln!("Found {count} validation errors. Aborting.");

        // `process::exit()` instead of `panic!()`. No need to pollute the output with useless stack traces:
        std::process::exit(count);
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
