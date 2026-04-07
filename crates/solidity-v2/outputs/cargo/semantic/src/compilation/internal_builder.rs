use std::collections::BTreeMap;

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeId};
use slang_solidity_v2_parser::{Parser, ParserError};

use super::file::File;
use super::unit::CompilationUnit;
use crate::binder::Binder;
use crate::passes::{
    p1_collect_definitions, p2_linearise_contracts, p3_type_definitions, p4_resolve_references,
};
use crate::types::TypeRegistry;

#[doc(hidden)]
pub struct InternalCompilationBuilder {
    language_version: LanguageVersion,
    files: BTreeMap<String, File>,
}

impl InternalCompilationBuilder {
    pub fn create(language_version: LanguageVersion) -> Self {
        Self {
            language_version,
            files: BTreeMap::new(),
        }
    }

    pub fn add_file(&mut self, id: String, contents: &str) -> Result<AddFileResponse, ParserError> {
        if self.files.contains_key(&id) {
            // Already added. No need to process it again:
            return Ok(AddFileResponse {
                import_paths: vec![],
            });
        }

        let source_unit_cst = Parser::parse(contents, self.language_version)?;
        let source_unit = ir::build(&source_unit_cst, &contents);
        let import_paths = Self::extract_imports_path(&source_unit);

        let file = File::new(id.clone(), source_unit);
        self.files.insert(id, file);

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

    fn extract_imports_path(source_unit: &ir::SourceUnit) -> Vec<(NodeId, String)> {
        let mut import_paths = Vec::new();

        for member in &source_unit.members {
            let ir::SourceUnitMember::ImportClause(import_clause) = member else {
                continue;
            };
            let (node_id, path) = match import_clause {
                ir::ImportClause::PathImport(path_import) => {
                    (path_import.id(), path_import.path.unparse().to_owned())
                }
                ir::ImportClause::ImportDeconstruction(import_deconstruction) => (
                    import_deconstruction.id(),
                    import_deconstruction.path.unparse().to_owned(),
                ),
            };
            import_paths.push((node_id, path));
        }
        import_paths
    }

    pub fn build(self) -> CompilationUnit {
        let mut binder = Binder::new();
        let mut types = TypeRegistry::default();
        let files: Vec<File> = self.files.into_values().collect();

        p1_collect_definitions::run(&files, &mut binder);
        p2_linearise_contracts::run(&files, &mut binder);
        p3_type_definitions::run(&files, &mut binder, &mut types);
        p4_resolve_references::run(&files, &mut binder, &mut types, self.language_version);

        CompilationUnit::create(self.language_version, files, binder, types)
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
