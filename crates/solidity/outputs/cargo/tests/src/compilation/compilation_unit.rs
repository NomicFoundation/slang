use anyhow::Result;
use metaslang_bindings::PathResolver;
use semver::Version;
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit};

use crate::compilation::resolver::TestsPathResolver;
use crate::utils::multi_part_file::{split_multi_file, MultiPart};

struct MultiPartBuildConfig<'a> {
    resolver: TestsPathResolver,
    multi_part: &'a MultiPart<'a>,
}

impl<'a> MultiPartBuildConfig<'a> {
    fn new(multi_part: &'a MultiPart<'a>) -> Self {
        Self {
            resolver: TestsPathResolver {},
            multi_part,
        }
    }
}

impl CompilationBuilderConfig for MultiPartBuildConfig<'_> {
    type Error = anyhow::Error;

    fn read_file(&mut self, file_id: &str) -> std::result::Result<Option<String>, Self::Error> {
        Ok(self
            .multi_part
            .parts
            .iter()
            .find(|part| part.name == file_id)
            .map(|part| part.contents.to_owned()))
    }

    fn resolve_import(
        &mut self,
        source_file_id: &str,
        import_path_cursor: &slang_solidity::cst::Cursor,
    ) -> std::result::Result<Option<String>, Self::Error> {
        Ok(self
            .resolver
            .resolve_path(source_file_id, import_path_cursor))
    }
}

pub(crate) fn build_compilation_unit_from_multi_part_file(
    version: &Version,
    contents: &str,
) -> Result<CompilationUnit> {
    let multi_part = split_multi_file(contents);
    let mut builder =
        CompilationBuilder::new(version.clone(), MultiPartBuildConfig::new(&multi_part))?;

    for part in &multi_part.parts {
        builder.add_file(part.name)?;
    }

    Ok(builder.build())
}
