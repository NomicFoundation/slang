use anyhow::Result;
use semver::{BuildMetadata, Prerelease};
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit};

use crate::dataset::SolidityProject;

struct ProjectConfig<'a> {
    project: &'a SolidityProject,
}

impl CompilationBuilderConfig for ProjectConfig<'_> {
    type Error = anyhow::Error;

    fn read_file(&mut self, file_id: &str) -> Result<Option<String>> {
        self.project
            .file_contents(file_id)
            .map(|content| Some(content.to_owned()))
    }

    fn resolve_import(
        &mut self,
        source_file_id: &str,
        import_path_cursor: &slang_solidity::cst::Cursor,
    ) -> Result<Option<String>> {
        let import_path = import_path_cursor.node().unparse();
        let import_path = import_path
            .strip_prefix(|c| matches!(c, '"' | '\''))
            .unwrap()
            .strip_suffix(|c| matches!(c, '"' | '\''))
            .unwrap()
            .trim();

        self.project
            .resolve_import(source_file_id, import_path)
            .map(Some)
    }
}

pub fn create_compilation_unit(project: &SolidityProject) -> Result<CompilationUnit> {
    let mut version = semver::Version::parse(&project.compilation.compiler_version).unwrap();
    version.pre = Prerelease::EMPTY;
    version.build = BuildMetadata::EMPTY;

    let mut builder = CompilationBuilder::new(version, ProjectConfig { project })?;

    builder.add_file(&project.compilation.get_entrypoint())?;

    Ok(builder.build())
}
