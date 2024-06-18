use core::fmt;
use std::cmp::Ordering;
use std::collections::HashMap;

use regex::Regex;
use thiserror::Error;

use crate::bindings::Bindings;
use crate::cursor::Cursor;
use crate::kinds::TerminalKind;
use crate::query::Query;

#[derive(Debug, Error)]
pub enum AssertionError {
    #[error("Invalid assertion at {0}:{1}")]
    InvalidAssertion(usize, usize),

    #[error("Duplicate assertion definition {0}")]
    DuplicateDefinition(String),

    #[error("Failed {0} of {1} bindings assertions")]
    FailedAssertions(usize, usize),
}

pub fn check_assertions(
    bindings: &Bindings,
    assertions: &Assertions,
) -> Result<(), AssertionError> {
    let mut count = 0;
    let mut success = 0;

    for assertion in assertions.definitions.values() {
        count += 1;

        let Assertion::Definition { id: _, cursor } = assertion else {
            unreachable!("{assertion} is not a definition assertion");
        };

        let Some(handle) = bindings.cursor_to_handle(cursor) else {
            eprintln!("{assertion} failed: not found");
            continue;
        };
        if !handle.is_definition() {
            eprintln!("{assertion} failed: not a definition");
            continue;
        }

        success += 1;
    }

    for assertion in &assertions.references {
        count += 1;

        let Assertion::Reference { id, cursor } = assertion else {
            unreachable!("{assertion} is not a reference assertion");
        };

        let Some(handle) = bindings.cursor_to_handle(cursor) else {
            eprintln!("{assertion} failed: not found");
            continue;
        };
        if !handle.is_reference() {
            eprintln!("{assertion} failed: not a reference");
            continue;
        }

        let Some(def_handle) = handle.jump_to_definition() else {
            // couldn't jump to definition
            if id.is_some() {
                // but a binding resolution was expected
                eprintln!("{assertion} failed: not resolved");
            } else {
                // and we asserted an unresolved reference -> good
                success += 1;
            }
            continue;
        };
        let Some(id) = id else {
            // expected an unresolved reference
            eprintln!(
                "{assertion} failed: reference did resolve to {}",
                DisplayCursor(&def_handle.get_cursor().unwrap())
            );
            continue;
        };

        let Some(Assertion::Definition {
            id: _,
            cursor: def_cursor,
        }) = assertions.definitions.get(id)
        else {
            eprintln!("{assertion} failed: definition assertion not found");
            continue;
        };
        if let Some(ref_def_cursor) = def_handle.get_cursor() {
            if ref_def_cursor != *def_cursor {
                eprintln!(
                    "{assertion} failed: resolved to unexpected {}",
                    DisplayCursor(&ref_def_cursor)
                );
                continue;
            }
        } else {
            eprintln!("{assertion} failed: jumped to definition did not resolve to a cursor");
            continue;
        }

        success += 1;
    }

    if count > success {
        eprintln!();
        Err(AssertionError::FailedAssertions(count - success, count))
    } else {
        println!("{count} binding assertions OK");
        Ok(())
    }
}

pub fn collect_assertions(cursor: Cursor) -> Result<Assertions, AssertionError> {
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

pub struct Assertions {
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
                write!(f, "Unresolved Reference")?;
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
        write!(f, " {}", DisplayCursor(cursor))
    }
}

struct DisplayCursor<'a>(&'a Cursor);

impl<'a> fmt::Display for DisplayCursor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let offset = self.0.text_offset();
        let range = self.0.text_range();
        write!(
            f,
            "`{}` at {}:{} [{}..{}]",
            self.0.node().unparse(),
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
