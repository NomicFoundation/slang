use std::collections::BTreeMap;
use std::rc::Rc;

use semver::Version;

use crate::compilation::{CompilationUnit, File};
use crate::cst::Cursor;
use crate::extensions::compilation::ImportPathsExtractor;
use crate::parser::{Parser, ParserInitializationError};

pub struct InternalCompilationBuilder {
    parser: Parser,
    imports: ImportPathsExtractor,
    files: BTreeMap<String, File>,
}

#[derive(thiserror::Error, Debug)]
pub enum CompilationInitializationError {
    #[error(transparent)]
    ParserInitialization(#[from] ParserInitializationError),
}

impl InternalCompilationBuilder {
    pub fn create(language_version: Version) -> Result<Self, CompilationInitializationError> {
        let parser = Parser::create(language_version)?;

        Ok(Self {
            parser,
            imports: ImportPathsExtractor::new(),
            files: BTreeMap::new(),
        })
    }

    pub fn add_file(&mut self, id: String, contents: &str) -> AddFileResponse {
        if self.files.contains_key(&id) {
            // Already added. No need to process it again:
            return AddFileResponse {
                import_paths: vec![],
            };
        }

        let parse_output = self.parser.parse(Parser::ROOT_KIND, contents);

        let import_paths = self.imports.extract(parse_output.create_tree_cursor());

        let file = File::new(id.clone(), parse_output.tree().clone());
        self.files.insert(id, file);

        AddFileResponse { import_paths }
    }

    pub fn resolve_import(
        &mut self,
        source_file_id: &str,
        import_path: &Cursor,
        destination_file_id: String,
    ) -> Result<(), ResolveImportError> {
        self.files
            .get_mut(source_file_id)
            .ok_or_else(|| ResolveImportError::SourceFileNotFound(source_file_id.to_owned()))?
            .resolve_import(import_path, destination_file_id);

        Ok(())
    }

    pub fn build(&self) -> CompilationUnit {
        let language_version = self.parser.language_version().to_owned();

        let files = self
            .files
            .iter()
            .map(|(id, file)| (id.to_owned(), Rc::new(file.to_owned())))
            .collect();

        CompilationUnit::new(language_version, files)
    }
}

pub struct AddFileResponse {
    pub import_paths: Vec<Cursor>,
}

#[derive(thiserror::Error, Debug)]
pub enum ResolveImportError {
    #[error(
        "Source file not found: '{0}'. Make sure to add it first, before resolving its imports."
    )]
    SourceFileNotFound(String),
}
