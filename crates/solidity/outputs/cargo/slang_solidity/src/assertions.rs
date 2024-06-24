use core::fmt;
use std::cmp::Ordering;
use std::collections::HashMap;

use regex::Regex;
use semver::{Version, VersionReq};
use thiserror::Error;

use crate::bindings::Bindings;
use crate::cursor::Cursor;
use crate::query::Query;

#[derive(Debug, Error)]
pub enum AssertionError {
    #[error("Invalid assertion at {0}:{1}")]
    InvalidAssertion(usize, usize),

    #[error("Invalid version requierement at line {0}: `{1}`")]
    InvalidVersionReq(usize, String),

    #[error("Duplicate assertion definition {0}")]
    DuplicateDefinition(String),

    #[error("Failed {failed} of {total} bindings assertions: {errors:?}")]
    FailedAssertions {
        failed: usize,
        total: usize,
        errors: Vec<String>,
    },
}

pub fn check_assertions(
    bindings: &Bindings,
    assertions: &Assertions,
    version: &Version,
) -> Result<(), AssertionError> {
    let mut count = 0;
    let mut failures: Vec<String> = Vec::new();

    for assertion in assertions.definitions.values() {
        count += 1;

        let Assertion::Definition { id: _, cursor } = assertion else {
            unreachable!("{assertion} is not a definition assertion");
        };

        let Some(handle) = bindings.cursor_to_handle(cursor) else {
            failures.push(format!("{assertion} failed: not found"));
            continue;
        };
        if !handle.is_definition() {
            failures.push(format!("{assertion} failed: not a definition"));
            continue;
        }
    }

    for assertion in &assertions.references {
        count += 1;

        let Assertion::Reference {
            id,
            cursor,
            version_req,
        } = assertion
        else {
            unreachable!("{assertion} is not a reference assertion");
        };

        let expect_success = if let Some(version_req) = version_req {
            version_req.matches(version)
        } else {
            true
        };

        let Some(handle) = bindings.cursor_to_handle(cursor) else {
            failures.push(format!("{assertion} failed: not found"));
            continue;
        };
        if !handle.is_reference() {
            failures.push(format!("{assertion} failed: not a reference"));
            continue;
        }

        let Some(def_handle) = handle.jump_to_definition() else {
            // couldn't jump to definition
            if id.is_some() && expect_success {
                // but a binding resolution was expected
                failures.push(format!("{assertion} failed: not resolved"));
            }
            continue;
        };
        let Some(id) = id else {
            if expect_success {
                // expected an unresolved reference
                failures.push(format!(
                    "{assertion} failed: reference did resolve to {}",
                    DisplayCursor(&def_handle.get_cursor().unwrap())
                ));
            }
            continue;
        };

        let Some(Assertion::Definition {
            id: _,
            cursor: def_cursor,
        }) = assertions.definitions.get(id)
        else {
            failures.push(format!(
                "{assertion} failed: definition assertion not found"
            ));
            continue;
        };
        if let Some(ref_def_cursor) = def_handle.get_cursor() {
            if ref_def_cursor != *def_cursor {
                if expect_success {
                    failures.push(format!(
                        "{assertion} failed: resolved to unexpected {}",
                        DisplayCursor(&ref_def_cursor)
                    ));
                }
                continue;
            }
        } else {
            failures.push(format!(
                "{assertion} failed: jumped to definition did not resolve to a cursor"
            ));
            continue;
        }
        if !expect_success {
            failures.push(format!(
                "{assertion} succeeded but was expected to fail in version {version}"
            ));
        }
    }

    if failures.is_empty() {
        println!("{count} binding assertions OK");
        Ok(())
    } else {
        Err(AssertionError::FailedAssertions {
            total: count,
            failed: failures.len(),
            errors: failures,
        })
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
    Definition {
        id: String,
        cursor: Cursor,
    },
    Reference {
        id: Option<String>,
        cursor: Cursor,
        version_req: Option<VersionReq>,
    },
}

impl fmt::Display for Assertion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assert ")?;
        let cursor = match self {
            Self::Definition { id, cursor } => {
                write!(f, "Definition {id}")?;
                cursor
            }
            Self::Reference {
                id,
                cursor,
                version_req,
            } => {
                if let Some(id) = id {
                    write!(f, "Reference {id}")?;
                } else {
                    write!(f, "Unresolved Reference")?;
                }
                if let Some(version_req) = version_req {
                    write!(f, " in versions {version_req}")?;
                }
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
    let assertion_regex =
        Regex::new(r"[\^](?<type>ref|def):(?<id>[0-9a-zA-Z_-]+|!)([\t ]*\((?<version>[^)]+)\))?")
            .unwrap();
    let comment_offset = comment.text_offset();
    let comment_col = comment_offset.column;
    let comment_str = comment.node().unparse();

    let Some(captures) = assertion_regex.captures(&comment_str) else {
        return Ok(None);
    };

    let assertion_id = captures.name("id").unwrap().as_str();
    let assertion_type = captures.name("type").unwrap().as_str();
    let assertion_col = comment_col + captures.get(0).unwrap().start();
    let version_req = match captures.name("version") {
        Some(version) => {
            let Ok(version_req) = VersionReq::parse(version.as_str()) else {
                return Err(AssertionError::InvalidVersionReq(
                    comment_offset.line + 1,
                    version.as_str().to_owned(),
                ));
            };
            Some(version_req)
        }
        None => None,
    };

    if let Some(cursor) = search_asserted_node_backwards(comment.clone(), assertion_col) {
        let assertion = match assertion_type {
            "ref" => {
                let id = if assertion_id == "!" {
                    // this should be an unresolved reference
                    None
                } else {
                    Some(assertion_id.to_owned())
                };
                Assertion::Reference {
                    id,
                    cursor,
                    version_req,
                }
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

        // Skip over trivia, to allow defining multiple assertions for the same
        // line of code in multiple single line comments
        if cursor.node().is_trivia() {
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
