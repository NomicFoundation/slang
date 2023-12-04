use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use cargo_emit::rerun_if_changed;

use crate::{
    cargo::CargoWorkspace,
    codegen::common::file_system::{delete_file, verify_file, write_file},
    github::GitHub,
    paths::{FileWalker, PathExtensions},
};

pub struct CodegenWriteOnly {
    generated_dirs: HashSet<PathBuf>,
    generated_files: HashSet<PathBuf>,
}

impl CodegenWriteOnly {
    pub fn new() -> Result<Self> {
        Ok(Self {
            generated_dirs: HashSet::new(),
            generated_files: HashSet::new(),
        })
    }
}

impl CodegenWriteOnly {
    pub fn write_file(
        &mut self,
        file_path: impl AsRef<Path>,
        contents: impl AsRef<str>,
    ) -> Result<()> {
        let file_path = file_path.as_ref();

        if file_path.strip_repo_root().is_err() {
            bail!("Generated file is outside repository: {file_path:?}");
        }

        if !self.generated_files.insert(file_path.to_owned()) {
            bail!("File was generated twice: {file_path:?}")
        }

        self.generated_dirs.insert(file_path.generated_dir()?);

        return if GitHub::is_running_in_ci() {
            verify_file(file_path, contents.as_ref())
        } else {
            write_file(file_path, contents.as_ref())
        };
    }
}

impl Drop for CodegenWriteOnly {
    fn drop(&mut self) {
        if CargoWorkspace::is_running_inside_build_scripts() {
            for generated_dir in &self.generated_dirs {
                rerun_if_changed!(generated_dir.unwrap_str());
            }
        }

        for generated_dir in &self.generated_dirs {
            for file in FileWalker::from_directory(generated_dir)
                .find_all()
                .unwrap()
            {
                if self.generated_files.contains(&file) {
                    continue;
                }

                if GitHub::is_running_in_ci() {
                    panic!("File was not generated in this context: {file:?}");
                } else {
                    delete_file(&file).expect("Failed to delete orphaned files.");
                }
            }
        }
    }
}
