use core::fmt;
use std::cmp::Ordering;
use std::collections::HashMap;

use anyhow::Result;
use regex::Regex;
use semver::Version;
use slang_solidity::bindings;
use slang_solidity::cli::commands;
use slang_solidity::cli::commands::CommandError;
use slang_solidity::cursor::Cursor;
use slang_solidity::kinds::TerminalKind;
use slang_solidity::query::Query;
use thiserror::Error;

pub fn execute_check_assertions(file_path_string: &str, version: Version) -> Result<()> {
    let mut bindings = bindings::create(version.clone());
    let parse_output = commands::parse::parse_source_file(file_path_string, version, |_| ())?;
    let tree_cursor = parse_output.create_tree_cursor();

    bindings.add_file(file_path_string, tree_cursor.clone());

    let assertions =
        collect_assertions(tree_cursor).map_err(|e| CommandError::Unknown(e.to_string()))?;
    for assertion in assertions.iter() {
        println!("{assertion}");
    }
    Ok(())
}

#[derive(Debug, Error)]
enum AssertionError {
    #[error("Invalid assertion at {0}:{1}")]
    InvalidAssertion(usize, usize),

    #[error("Duplicate assertion definition {0}")]
    DuplicateDefinition(String),
}

fn collect_assertions(cursor: Cursor) -> Result<Assertions, AssertionError> {
    let mut assertions = Assertions::new();

    let query = Query::parse("@comment [SingleLineComment]").unwrap();
    for result in cursor.query(vec![query]) {
        let captures = result.captures;
        let Some(comment) = captures.get("comment").and_then(|v| v.first()) else {
            continue;
        };

        if let Some(assertion) = find_assertion_in_comment(comment)? {
            assertions.insert_assertion(assertion)?;
        }
    }

    Ok(assertions)
}

struct Assertions {
    definitions: HashMap<String, Assertion>,
    references: Vec<Assertion>,
}

impl Assertions {
    fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            references: Vec::new(),
        }
    }

    fn insert_assertion(&mut self, assertion: Assertion) -> Result<(), AssertionError> {
        match assertion {
            Assertion::Definition { ref id, .. } => {
                if self.definitions.contains_key(id) {
                    Err(AssertionError::DuplicateDefinition(id.clone()))
                } else {
                    self.definitions.insert(id.clone(), assertion);
                    Ok(())
                }
            }
            Assertion::Reference { .. } => {
                self.references.push(assertion);
                Ok(())
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Assertion> {
        self.definitions.values().chain(self.references.iter())
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Assertion {
    Definition { id: String, cursor: Cursor },
    Reference { id: Option<String>, cursor: Cursor },
}

impl fmt::Display for Assertion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assert ")?;
        let cursor = match self {
            Self::Definition { id, cursor } => {
                write!(f, "Definition {id}")?;
                cursor
            }
            Self::Reference { id: None, cursor } => {
                write!(f, "Unresolved Reference",)?;
                cursor
            }
            Self::Reference {
                id: Some(id),
                cursor,
            } => {
                write!(f, "Reference {id}")?;
                cursor
            }
        };
        let offset = cursor.text_offset();
        let range = cursor.text_range();
        write!(
            f,
            " `{}` at {}:{} [{}..{}]",
            cursor.node().unparse(),
            offset.line + 1,
            offset.column + 1,
            range.start,
            range.end,
        )
    }
}

fn find_assertion_in_comment(comment: &Cursor) -> Result<Option<Assertion>, AssertionError> {
    let assertion_regex = Regex::new(r"[\^](ref|def):([0-9a-zA-Z_-]+|!)").unwrap();
    let comment_offset = comment.text_offset();
    let comment_col = comment_offset.column;
    let comment_str = comment.node().unparse();

    let Some(captures) = assertion_regex.captures(&comment_str) else {
        return Ok(None);
    };

    let assertion_id = captures.get(2).unwrap().as_str();
    let assertion_type = captures.get(1).unwrap().as_str();
    let assertion_col = comment_col + captures.get(0).unwrap().start();

    if let Some(cursor) = search_asserted_node_backwards(comment.clone(), assertion_col) {
        let assertion = match assertion_type {
            "ref" => {
                let id = if assertion_id == "!" {
                    // this should be an unresolved reference
                    None
                } else {
                    Some(assertion_id.to_owned())
                };
                Assertion::Reference { id, cursor }
            }
            "def" => Assertion::Definition {
                id: assertion_id.to_owned(),
                cursor,
            },
            _ => unreachable!("unknown assertion type"),
        };
        Ok(Some(assertion))
    } else {
        Err(AssertionError::InvalidAssertion(
            comment_offset.line + 1,
            assertion_col + 1,
        ))
    }
}

fn search_asserted_node_backwards(mut cursor: Cursor, anchor_column: usize) -> Option<Cursor> {
    let starting_line = cursor.text_offset().line;
    while cursor.go_to_previous() {
        // Skip if the cursor is on the same line
        if cursor.text_offset().line == starting_line {
            continue;
        }

        // Skip over trivia and other comments (allows defining multiple
        // assertions for the same line of code in multiple single line
        // comments)
        if cursor.node().is_terminal_with_kinds(&[
            TerminalKind::Whitespace,
            TerminalKind::EndOfLine,
            TerminalKind::SingleLineComment,
        ]) {
            continue;
        }

        let cursor_column = cursor.text_offset().column;
        match cursor_column.cmp(&anchor_column) {
            Ordering::Equal => return Some(cursor),
            Ordering::Greater => continue,
            Ordering::Less => (),
        }

        // Node is not found, and probably the anchor is invalid
        break;
    }
    None
}
