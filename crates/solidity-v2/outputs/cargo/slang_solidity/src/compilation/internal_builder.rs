use std::collections::BTreeMap;
use std::rc::Rc;

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeId, NodeIdGenerator};
use slang_solidity_v2_parser::{ParseOutput, Parser, ParserError};
use slang_solidity_v2_semantic::context::{extract_import_paths_from_source_unit, SemanticContext};

use super::file::File;
use super::unit::CompilationUnit;

#[doc(hidden)]
pub struct InternalCompilationBuilder {
    language_version: LanguageVersion,
    files: BTreeMap<String, File>,
    id_generator: NodeIdGenerator,
}

impl InternalCompilationBuilder {
    pub fn create(language_version: LanguageVersion) -> Self {
        Self {
            language_version,
            files: BTreeMap::new(),
            id_generator: NodeIdGenerator::default(),
        }
    }

    pub fn add_file(
        &mut self,
        id: String,
        contents: &str,
    ) -> Result<AddFileResponse, Vec<ParserError>> {
        if self.files.contains_key(&id) {
            // Already added. No need to process it again:
            return Ok(AddFileResponse {
                import_paths: vec![],
            });
        }

        let ParseOutput {
            source_unit,
            errors,
        } = Parser::parse(contents, self.language_version);

        let source_unit = ir::build(&source_unit, &contents, &mut self.id_generator);
        let import_paths = extract_import_paths_from_source_unit(&source_unit);

        let file = File::new(id.clone(), source_unit);
        self.files.insert(id, file);

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(AddFileResponse { import_paths })
    }

    pub fn resolve_import(
        &mut self,
        source_file_id: &str,
        node_id: NodeId,
        destination_file_id: String,
    ) -> Result<(), ResolveImportError> {
        self.files
            .get_mut(source_file_id)
            .ok_or_else(|| ResolveImportError::SourceFileNotFound(source_file_id.to_owned()))?
            .add_resolved_import(node_id, destination_file_id);

        Ok(())
    }

    pub fn build(self) -> CompilationUnit {
        let files: Vec<File> = self.files.into_values().collect();
        let semantic = SemanticContext::build_from(self.language_version, &files);

        CompilationUnit::create(self.language_version, files, Rc::new(semantic))
    }
}

#[doc(hidden)]
pub struct AddFileResponse {
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
