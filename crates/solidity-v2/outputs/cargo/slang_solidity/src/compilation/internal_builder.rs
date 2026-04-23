use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes as cst;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_parser::{ParseOutput, Parser, ParserError};
use slang_solidity_v2_semantic::context::{extract_import_paths_from_source_unit, SemanticContext};

use super::file::File;
use super::unit::CompilationUnit;

pub(crate) struct InternalFile {
    source_unit: cst::SourceUnit,
    contents: String,
    resolved_imports: BTreeMap<String, String>,
}

#[doc(hidden)]
pub(crate) struct InternalCompilationBuilder {
    language_version: LanguageVersion,
    files: BTreeMap<String, InternalFile>,
}

impl InternalCompilationBuilder {
    pub fn create(language_version: LanguageVersion) -> Self {
        Self {
            language_version,
            files: BTreeMap::new(),
        }
    }

    pub fn add_file(
        &mut self,
        id: String,
        contents: String,
    ) -> Result<AddFileResponse, Vec<ParserError>> {
        if self.files.contains_key(&id) {
            // Already added. No need to process it again:
            return Ok(AddFileResponse {
                import_paths: BTreeSet::new(),
            });
        }

        let ParseOutput {
            source_unit,
            errors,
        } = Parser::parse(&contents, self.language_version);
        let import_paths = extract_import_paths_from_cst(&source_unit, &contents);

        let internal_file = InternalFile {
            source_unit,
            contents,
            resolved_imports: BTreeMap::new(),
        };
        self.files.insert(id, internal_file);

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(AddFileResponse { import_paths })
    }

    pub fn resolve_import(
        &mut self,
        source_file_id: &str,
        import_path: String,
        destination_file_id: String,
    ) -> Result<(), ResolveImportError> {
        let internal_file = self
            .files
            .get_mut(source_file_id)
            .ok_or_else(|| ResolveImportError::SourceFileNotFound(source_file_id.to_owned()))?;

        internal_file
            .resolved_imports
            .insert(import_path, destination_file_id);

        Ok(())
    }

    fn build_files(self) -> Vec<File> {
        let mut id_generator = ir::NodeIdGenerator::default();
        self.files
            .into_iter()
            .map(|(file_id, internal_file)| {
                let ir_root = ir::build(
                    &internal_file.source_unit,
                    &internal_file.contents,
                    &mut id_generator,
                );
                let import_paths = extract_import_paths_from_source_unit(&ir_root);
                let mut file = File::new(file_id, ir_root);
                for (node_id, import_path) in import_paths {
                    let Some(target_file_id) = internal_file.resolved_imports.get(&import_path)
                    else {
                        continue;
                    };
                    file.add_resolved_import(node_id, target_file_id.clone());
                }
                file
            })
            .collect()
    }

    pub fn build(self) -> CompilationUnit {
        let language_version = self.language_version;
        let files = self.build_files();
        let semantic = SemanticContext::build_from(language_version, &files);

        CompilationUnit::create(language_version, files, Rc::new(semantic))
    }
}

#[doc(hidden)]
pub(crate) struct AddFileResponse {
    pub import_paths: BTreeSet<String>,
}

#[derive(thiserror::Error, Debug)]
#[doc(hidden)]
pub enum ResolveImportError {
    #[error(
        "Source file not found: '{0}'. Make sure to add it first, before resolving its imports."
    )]
    SourceFileNotFound(String),
}

fn extract_import_paths_from_cst(
    source_unit: &cst::SourceUnit,
    contents: &str,
) -> BTreeSet<String> {
    let mut import_paths = BTreeSet::new();

    for member in &source_unit.members.elements {
        let cst::SourceUnitMember::ImportDirective(import_directive) = member else {
            continue;
        };
        let range = match &import_directive.clause {
            cst::ImportClause::PathImport(path_import) => &path_import.path.range,
            cst::ImportClause::NamedImport(named_import) => &named_import.path.range,
            cst::ImportClause::ImportDeconstruction(import_deconstruction) => {
                &import_deconstruction.path.range
            }
        };
        import_paths.insert(contents[range.clone()].to_owned());
    }
    import_paths
}
