use anyhow::{bail, Result};
use slang_solidity::backend::passes::{compile, CompilationOutput};
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit};
use slang_solidity::utils::LanguageFacts;

// This module allows to easily build a one-file compilation unit from a str,
// and generates the compilation output from the backend pipeline.

const MAIN_ID: &str = "MAIN-ID";

pub(crate) struct OneFileConfig {
    file_content: String,
}

impl OneFileConfig {
    pub(crate) fn new(content: &str) -> OneFileConfig {
        OneFileConfig {
            file_content: content.to_owned(),
        }
    }
}

impl CompilationBuilderConfig for OneFileConfig {
    type Error = anyhow::Error;

    fn read_file(&mut self, file_id: &str) -> Result<Option<String>> {
        assert_eq!(file_id, MAIN_ID);
        Ok(Some(self.file_content.clone()))
    }

    fn resolve_import(
        &mut self,
        _source_file_id: &str,
        _import_path_cursor: &slang_solidity::cst::Cursor,
    ) -> Result<Option<String>> {
        bail!("Not expecting an import")
    }
}

pub(crate) fn build_one_file_compilation_unit(content: &str) -> Result<CompilationUnit> {
    let config = OneFileConfig::new(content);
    let mut builder = CompilationBuilder::create(LanguageFacts::LATEST_VERSION, config)?;

    builder.add_file(MAIN_ID)?;

    Ok(builder.build())
}

pub(crate) fn compile_one_file(content: &str) -> Result<CompilationOutput> {
    let unit = build_one_file_compilation_unit(content)?;
    let data = compile(unit);
    assert_eq!(1, data.files.len());
    Ok(data)
}
