// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::BTreeMap;
use std::rc::Rc;

use semver::Version;

use crate::compilation::{CompilationUnit, File};
use crate::cst::{Cursor, Query};
use crate::parser::{Parser, ParserInitializationError};

pub struct InternalCompilationBuilder {
    parser: Parser,
    files: BTreeMap<String, File>,
}

#[derive(thiserror::Error, Debug)]
pub enum CompilationInitializationError {
    #[error(transparent)]
    ParserInitialization(#[from] ParserInitializationError),
}

impl InternalCompilationBuilder {
    pub fn create(language_version: Version) -> Result<Self, CompilationInitializationError> {
        let parser = match Parser::create(language_version) {
            Ok(parser) => parser,
            Err(error) => {
                return Err(error.into());
            }
        };

        Ok(Self {
            parser,
            files: BTreeMap::new(),
        })
    }

    pub fn add_file(
        &mut self,
        id: String,
        contents: &str,
    ) -> Result<AddFileResponse, AddFileError> {
        let parse_output = self.parser.parse(Parser::ROOT_KIND, contents);

        let import_paths = extract_import_paths(parse_output.create_tree_cursor())
            .map_err(AddFileError::Internal)?;

        let file = File::new(id.clone(), parse_output.tree().clone());
        self.files.insert(id, file);

        Ok(AddFileResponse { import_paths })
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
pub enum AddFileError {
    #[error("Internal Error: {0}")]
    Internal(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ResolveImportError {
    #[error(
        "Source file not found: '{0}'. Make sure to add it first, before resolving its imports."
    )]
    SourceFileNotFound(String),
}

fn extract_import_paths(cursor: Cursor) -> Result<Vec<Cursor>, String> {
    let mut import_paths = Vec::new();

    for query_match in cursor.query(vec![
        Query::parse(
            "[PathImport
                path: [StringLiteral
                    @variant ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
                ]
            ]",
        )
        .map_err(|e| e.to_string())?,
        Query::parse(
            "[NamedImport
                path: [StringLiteral
                    @variant ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
                ]
            ]",
        )
        .map_err(|e| e.to_string())?,
        Query::parse(
            "[ImportDeconstruction
                path: [StringLiteral
                    @variant ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
                ]
            ]",
        )
        .map_err(|e| e.to_string())?,
    ]) {
        for (match_name, _, cursors) in query_match.captures() {
            if match_name != "variant" {
                return Err(format!("Unexpected match name: {match_name}"));
            }

            import_paths.extend(cursors);
        }
    }

    Ok(import_paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::LanguageFacts;

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
        match LanguageFacts::NAME {
            "Solidity" => {
                // Run the test only for Solidity:
            }
            "CodegenRuntime" | "Testlang" => {
                return;
            }
            other => {
                panic!("Unexpected language name: {other}");
            }
        };

        let parser = Parser::create(Version::new(0, 8, 0)).unwrap();
        let parse_output = parser.parse(Parser::ROOT_KIND, source);

        let actual: Vec<_> = extract_import_paths(parse_output.create_tree_cursor())
            .unwrap()
            .into_iter()
            .map(|cursor| cursor.node().unparse())
            .collect();

        assert_eq!(actual, expected.to_vec());
    }
}
