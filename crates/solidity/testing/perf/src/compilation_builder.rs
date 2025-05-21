use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::Result;
use infra_utils::paths::PathExtensions;
use semver::Version;
use slang_solidity::compilation::{AddFileResponse, CompilationUnit, InternalCompilationBuilder};

use crate::import_resolver::resolve_import;

pub struct CompilationBuilder {
    project_root: PathBuf,
    entrypoint: String,
    internal: InternalCompilationBuilder,
    seen_files: HashSet<String>,
}

// a poor-man's copy of crates/solidity/testing/sourcify/src/compilation_builder.rs
// TODO: abstract the functionality that is common to both
// NOTE: here we deal with a real file system, unlike in sourcify.
impl CompilationBuilder {
    pub fn new(
        language_version: Version,
        project_root: PathBuf,
        entrypoint: String,
    ) -> Result<CompilationBuilder> {
        Ok(CompilationBuilder {
            project_root,
            entrypoint,
            internal: InternalCompilationBuilder::create(language_version)?,
            seen_files: HashSet::new(),
        })
    }

    pub fn build(mut self) -> Result<CompilationUnit> {
        let entrypoint = self.entrypoint.clone();

        // Sanitize the import string: remove leading slashes
        let filename = self
            .project_root
            .join(entrypoint.trim_start_matches('/'))
            .canonicalize()
            .unwrap_or_else(|_| panic!("Can't find {entrypoint}"));

        self.add_file(filename.to_string_lossy().as_ref())?;

        Ok(self.internal.build())
    }

    fn add_file(&mut self, filename: &str) -> Result<()> {
        if !self.seen_files.insert(filename.to_string()) {
            return Ok(());
        }

        let source = Path::new(filename).read_to_string()?;

        let AddFileResponse { import_paths } =
            self.internal.add_file(filename.to_string(), &source);

        for import_path in import_paths {
            let import_path = import_path.node().unparse();
            let import_path = import_path
                .strip_prefix(|c| matches!(c, '"' | '\''))
                .unwrap()
                .strip_suffix(|c| matches!(c, '"' | '\''))
                .unwrap()
                .trim();

            let import_real_name = resolve_import(&self.project_root, filename, import_path)?;
            self.add_file(&import_real_name)?;
        }

        Ok(())
    }
}
