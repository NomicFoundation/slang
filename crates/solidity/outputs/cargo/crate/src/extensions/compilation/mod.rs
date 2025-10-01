use crate::cst::{Cursor, Query};

pub struct ImportPathsExtractor {
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
        cursor
            .query(self.queries.clone())
            .flat_map(|query_match| query_match.capture("variant").unwrap().1)
            .collect()
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
