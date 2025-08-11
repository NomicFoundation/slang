use std::collections::HashSet;

use anyhow::Result;
use semver::{BuildMetadata, Prerelease};
use slang_solidity::compilation::{AddFileResponse, CompilationUnit, InternalCompilationBuilder};

use crate::dataset::SolidityProject;

pub struct CompilationBuilder {
    project: &'static SolidityProject,
    internal: InternalCompilationBuilder,
    seen_files: HashSet<String>,
}

impl CompilationBuilder {
    pub fn new(project: &'static SolidityProject) -> Result<CompilationBuilder> {
        let mut version = semver::Version::parse(&project.compilation.compiler_version).unwrap();
        version.pre = Prerelease::EMPTY;
        version.build = BuildMetadata::EMPTY;

        Ok(CompilationBuilder {
            project,
            internal: InternalCompilationBuilder::create(version)?,
            seen_files: HashSet::new(),
        })
    }

    pub fn build(&mut self) -> Result<CompilationUnit> {
        self.add_file(&self.project.compilation.get_entrypoint())?;

        let unit = self.internal.build();
        for file in unit.files() {
            assert_eq!(file.errors(), &[]);
        }
        Ok(unit)
    }

    fn add_file(&mut self, filename: &str) -> Result<()> {
        if !self.seen_files.insert(filename.to_string()) {
            return Ok(());
        }

        let source = self.project.file_contents(filename)?;

        assert_ne!(source, "");
        let AddFileResponse { import_paths } = self.internal.add_file(filename.to_string(), source);

        for import_path_cursor in import_paths {
            let import_path = import_path_cursor.node().unparse();
            let import_path = import_path
                .strip_prefix(|c| matches!(c, '"' | '\''))
                .unwrap()
                .strip_suffix(|c| matches!(c, '"' | '\''))
                .unwrap()
                .trim();

            let import_real_name = self.project.resolve_import(filename, import_path)?;
            self.internal.resolve_import(
                filename,
                &import_path_cursor,
                import_real_name.clone(),
            )?;
            self.add_file(&import_real_name)?;
        }

        Ok(())
    }
}
