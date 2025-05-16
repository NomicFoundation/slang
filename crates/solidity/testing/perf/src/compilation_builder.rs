use std::collections::HashSet;
use std::path::PathBuf;

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

        self.add_file(&entrypoint)?;

        Ok(self.internal.build())
    }

    fn add_file(&mut self, filename: &str) -> Result<()> {
        if !self.seen_files.insert(filename.into()) {
            return Ok(());
        }

        let source = self.project_root.join(filename).read_to_string()?;

        let AddFileResponse { import_paths } = self.internal.add_file(filename.into(), &source);

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
