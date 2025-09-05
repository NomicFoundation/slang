use std::collections::BTreeMap;
use std::rc::Rc;

use semver::Version;

use crate::compilation::{CompilationUnit, File};
use crate::cst::Cursor;
use crate::extensions::compilation::ImportPathsExtractor;
use crate::parser::{Parser, ParserInitializationError};

#[doc(hidden)]
pub struct InternalCompilationBuilder {
    parser: Parser,
    imports: ImportPathsExtractor,
    files: BTreeMap<String, File>,
}

impl InternalCompilationBuilder {
    pub fn create(language_version: Version) -> Result<Self, ParserInitializationError> {
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

        let parse_output = self.parser.parse_file_contents(contents);

        let import_paths = self.imports.extract(parse_output.create_tree_cursor());

        let file = File::create(id.clone(), parse_output);
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

        CompilationUnit::create(language_version, files)
    }
}

#[doc(hidden)]
pub struct AddFileResponse {
    pub import_paths: Vec<Cursor>,
}

#[derive(thiserror::Error, Debug)]
#[doc(hidden)]
pub enum ResolveImportError {
    #[error(
        "Source file not found: '{0}'. Make sure to add it first, before resolving its imports."
    )]
    SourceFileNotFound(String),
}
