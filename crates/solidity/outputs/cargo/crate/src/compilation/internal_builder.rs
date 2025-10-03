use std::collections::BTreeMap;
use std::rc::Rc;

use semver::Version;

use crate::compilation::{CompilationUnit, File};
use crate::cst::{Cursor, Query};
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

struct ImportPathsExtractor {
    queries: Vec<Query>,
}

impl ImportPathsExtractor {
    pub fn new() -> Self {
        Self {
            queries: [
                "[PathImport
                    path: [StringLiteral
                        @variant ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
                    ]
                ]",
                "[NamedImport
                    path: [StringLiteral
                        @variant ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
                    ]
                ]",
                "[ImportDeconstruction
                    path: [StringLiteral
                        @variant ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
                    ]
                ]",
            ]
            .into_iter()
            .map(|text| Query::create(text).unwrap())
            .collect(),
        }
    }

    pub fn extract(&self, cursor: Cursor) -> Vec<Cursor> {
        let mut cursors = Vec::new();

        for query_match in cursor.query(self.queries.clone()) {
            for cursor in query_match.capture("variant").unwrap().cursors() {
                cursors.push(cursor.clone());
            }
        }

        cursors
    }
}

#[cfg(test)]
mod tests {
    use semver::Version;

    use crate::parser::Parser;

    #[test]
    pub fn path_import() {
        run(
            r#"
                import "foo-double";
                import "bar-double" as bar;

                import 'foo-single';
                import 'bar-single' as bar;

            "#,
            &[
                "\"foo-double\"",
                "\"bar-double\"",
                "\'foo-single\'",
                "\'bar-single\'",
            ],
        );
    }

    #[test]
    pub fn named_import() {
        run(
            r#"
                import * as foo from "foo-double";

                import * as foo from 'foo-single';
            "#,
            &["\"foo-double\"", "\'foo-single\'"],
        );
    }

    #[test]
    pub fn import_deconstruction() {
        run(
            r#"
                import {a, b} from "foo-double";

                import {a, b} from 'foo-single';
            "#,
            &["\"foo-double\"", "\'foo-single\'"],
        );
    }

    fn run(source: &str, expected: &[&str]) {
        let parser = Parser::create(Version::new(0, 8, 0)).unwrap();
        let parse_output = parser.parse_file_contents(source);

        let imports = super::ImportPathsExtractor::new();

        let actual: Vec<_> = imports
            .extract(parse_output.create_tree_cursor())
            .into_iter()
            .map(|cursor| cursor.node().unparse())
            .collect();

        assert_eq!(actual, expected.to_vec());
    }
}
