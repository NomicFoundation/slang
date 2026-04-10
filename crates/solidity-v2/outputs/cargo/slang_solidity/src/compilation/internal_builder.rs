use std::collections::BTreeMap;
use std::rc::Rc;

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::interner::Interner;
use slang_solidity_v2_ir::ir::{self, NodeId};
use slang_solidity_v2_parser::{ParseOutput, Parser, ParserError};
use slang_solidity_v2_semantic::context::{
    extract_import_paths_from_source_unit, FileId, SemanticContext,
};

use super::file::File;
use super::unit::CompilationUnit;

#[doc(hidden)]
pub struct InternalCompilationBuilder {
    language_version: LanguageVersion,
    interner: Interner,
    files: BTreeMap<FileId, File>,
}

impl InternalCompilationBuilder {
    pub fn create(language_version: LanguageVersion) -> Self {
        Self {
            language_version,
            interner: Interner::new(),
            files: BTreeMap::new(),
        }
    }

    pub fn add_file(
        &mut self,
        id: &str,
        contents: &str,
    ) -> Result<AddFileResponse, Vec<ParserError>> {
        let file_id = self.interner.intern(id);
        if self.files.contains_key(&file_id) {
            // Already added. No need to process it again:
            return Ok(AddFileResponse {
                file_id,
                import_paths: vec![],
            });
        }

        let ParseOutput {
            source_unit,
            errors,
        } = Parser::parse(contents, self.language_version);

        let source_unit = ir::build(&source_unit, &contents, &mut self.interner);
        let import_paths = extract_import_paths_from_source_unit(&source_unit, &self.interner);

        let file = File::new(id.to_string(), file_id, source_unit);
        self.files.insert(file_id, file);

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(AddFileResponse {
            file_id,
            import_paths,
        })
    }

    pub fn resolve_import(
        &mut self,
        source_file_id: FileId,
        node_id: NodeId,
        destination_file: &str,
    ) -> Result<(), ResolveImportError> {
        self.files
            .get_mut(&source_file_id)
            .ok_or_else(|| {
                ResolveImportError::SourceFileNotFound(
                    self.interner.resolve(source_file_id).to_owned(),
                )
            })?
            .add_resolved_import(node_id, self.interner.intern(destination_file));

        Ok(())
    }

    pub fn build(self) -> CompilationUnit {
        let files: Vec<File> = self.files.into_values().collect();
        let interner = Rc::new(self.interner);
        let semantic = SemanticContext::build_from(self.language_version, &files, &interner);

        CompilationUnit::create(self.language_version, files, Rc::new(semantic), interner)
    }
}

#[doc(hidden)]
pub struct AddFileResponse {
    pub file_id: FileId,
    pub import_paths: Vec<(NodeId, String)>,
}

#[derive(thiserror::Error, Debug)]
#[doc(hidden)]
pub enum ResolveImportError {
    #[error(
        "Source file not found: '{0}'. Make sure to add it first, before resolving its imports."
    )]
    SourceFileNotFound(String),
}
