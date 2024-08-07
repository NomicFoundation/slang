use core::fmt;
use std::cmp::Ordering;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;
use semver::{Version, VersionReq};
use slang_solidity::bindings::{Bindings, Definition};
use slang_solidity::cursor::Cursor;
use slang_solidity::kinds::TerminalKind;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssertionError {
    #[error("Invalid assertion at {0}:{1}")]
    InvalidAssertion(usize, usize),

    #[error("Invalid version requierement at line {0}: `{1}`")]
    InvalidVersionReq(usize, String),

    #[error("Duplicate assertion definition {0}")]
    DuplicateDefinition(String),

    #[error("Failed {failed} of {total} bindings assertions:\n{errors:#?}")]
    FailedAssertions {
        failed: usize,
        total: usize,
        errors: Vec<String>,
    },
}

pub struct Assertions<'a> {
    definitions: HashMap<String, DefinitionAssertion<'a>>,
    references: Vec<ReferenceAssertion<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
struct DefinitionAssertion<'a> {
    id: String,
    cursor: Cursor,
    file: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
struct ReferenceAssertion<'a> {
    id: Option<String>,
    cursor: Cursor,
    file: &'a str,
    version_req: Option<VersionReq>,
}

impl<'a> Assertions<'a> {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            references: Vec::new(),
        }
    }

    fn count(&self) -> usize {
        self.definitions.len() + self.references.len()
    }

    fn insert_definition_assertion(
        &mut self,
        assertion: DefinitionAssertion<'a>,
    ) -> Result<(), AssertionError> {
        let id = &assertion.id;
        if self.definitions.contains_key(id) {
            Err(AssertionError::DuplicateDefinition(id.clone()))
        } else {
            self.definitions.insert(id.clone(), assertion);
            Ok(())
        }
    }

    fn insert_reference_assertion(&mut self, assertion: ReferenceAssertion<'a>) {
        self.references.push(assertion);
    }
}

impl<'a> fmt::Display for DefinitionAssertion<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Assert Definition {id} {cursor}",
            id = self.id,
            cursor = DisplayCursor(&self.cursor, Some(self.file)),
        )
    }
}

impl<'a> fmt::Display for ReferenceAssertion<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assert ")?;
        if let Some(id) = &self.id {
            write!(f, "Reference {id}")?;
        } else {
            write!(f, "Unresolved Reference")?;
        }
        if let Some(version_req) = &self.version_req {
            write!(f, " in versions {version_req}")?;
        }
        write!(f, " {}", DisplayCursor(&self.cursor, Some(self.file)))
    }
}

struct DisplayCursor<'a>(&'a Cursor, Option<&'a str>);

impl<'a> fmt::Display for DisplayCursor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let offset = self.0.text_offset();
        write!(
            f,
            "`{}` at {}:{}:{}",
            self.0.node().unparse(),
            self.1.unwrap_or("<unknown_file>"),
            offset.line + 1,
            offset.column + 1,
        )
    }
}

/// Collects bindings assertions in comments in the parsed source code
/// accessible through the given cursor. The definitionns take the following form:
///
///   uint x;
///   //   ^def:1
///
/// asserts that at the CST node above the caret symbol `^` (same column, not
/// necessarily in the previous line), the identifier `x` should create a
/// binding definition, and assigns it an ID of '1'.
///
/// Conversely, for references:
///
///   return x + y;
///   //         ^ref:! (>= 0.5.0)
///   //     ^ref:1
///
/// asserts that the CST identifier node `x` should be a binding reference which
/// should be declared as a correspoding definition assertion with identifier
/// '1'; and that the CST identifier node `y` should be a binding reference that
/// is unresolved for version at or above 0.5.0.
///
/// For assertion targets that are located at the column where the comment
/// begins the alternative anchor `<` can be used. For example:
///
///   x = y + 1;
///   //  ^ref:2
///   //<ref:1
///
pub fn collect_assertions_into<'a>(
    assertions: &mut Assertions<'a>,
    mut cursor: Cursor,
    file: &'a str,
    version: &Version,
) -> Result<(), AssertionError> {
    loop {
        if cursor
            .node()
            .is_terminal_with_kind(TerminalKind::SingleLineComment)
        {
            match find_assertion_in_comment(&cursor, version, file)? {
                Some(Assertion::Definition(assertion)) => {
                    assertions.insert_definition_assertion(assertion)?;
                }
                Some(Assertion::Reference(assertion)) => {
                    assertions.insert_reference_assertion(assertion);
                }
                None => (),
            }
        }

        if !cursor.go_to_next() {
            break;
        }
    }

    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
enum Assertion<'a> {
    Definition(DefinitionAssertion<'a>),
    Reference(ReferenceAssertion<'a>),
}

static ASSERTION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?<anchor>[\^]|[<])(?<type>ref|def):(?<id>[0-9a-zA-Z_-]+|!)([\t ]*\((?<version>[^)]+)\))?")
        .unwrap()
});

fn find_assertion_in_comment<'a>(
    comment: &Cursor,
    version: &Version,
    file: &'a str,
) -> Result<Option<Assertion<'a>>, AssertionError> {
    let comment_offset = comment.text_offset();
    let comment_col = comment_offset.column;
    let comment_str = comment.node().unparse();

    let Some(captures) = ASSERTION_REGEX.captures(&comment_str) else {
        return Ok(None);
    };

    let assertion_id = captures.name("id").unwrap().as_str();
    let assertion_type = captures.name("type").unwrap().as_str();
    let assertion_anchor = captures.name("anchor").unwrap();
    let assertion_col = comment_col
        + if assertion_anchor.as_str() == "^" {
            assertion_anchor.start()
        } else {
            0
        };
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
                Assertion::Reference(ReferenceAssertion {
                    id,
                    cursor,
                    file,
                    version_req,
                })
            }
            "def" => Assertion::Definition(DefinitionAssertion {
                id: assertion_id.to_owned(),
                cursor,
                file,
            }),
            _ => unreachable!("unknown assertion type"),
        };
        Ok(Some(assertion))
    } else {
        // Assertion target may not be parseable with the current version
        if let Some(version_req) = version_req {
            if !version_req.matches(version) {
                return Ok(None);
            }
        }
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

/// Checks that the given `assertions` are fulfilled in the given `bindings` for
/// the indicated `version`. Only references can have version requirements, and
/// the absence of a requirement means the assertion should hold for all
/// language versions.
///
pub fn check_assertions(
    bindings: &Bindings,
    assertions: &Assertions<'_>,
    version: &Version,
) -> Result<usize, AssertionError> {
    let mut failures: Vec<String> = Vec::new();

    check_definitions(bindings, assertions.definitions.values(), &mut failures);
    check_references(
        bindings,
        version,
        assertions.references.iter(),
        &assertions.definitions,
        &mut failures,
    );

    let count = assertions.count();
    if failures.is_empty() {
        Ok(count)
    } else {
        Err(AssertionError::FailedAssertions {
            total: count,
            failed: failures.len(),
            errors: failures,
        })
    }
}

fn check_definitions<'a>(
    bindings: &Bindings,
    definitions: impl Iterator<Item = &'a DefinitionAssertion<'a>>,
    failures: &mut Vec<String>,
) {
    for assertion in definitions {
        if let Err(failure) = find_definition(bindings, assertion) {
            failures.push(failure);
        }
    }
}

fn find_definition<'a>(
    bindings: &'a Bindings,
    assertion: &DefinitionAssertion<'_>,
) -> Result<Definition<'a>, String> {
    let DefinitionAssertion { cursor, .. } = assertion;

    let Some(definition) = bindings.definition_at(cursor) else {
        return Err(format!("{assertion} failed: not found"));
    };

    Ok(definition)
}

fn check_references<'a>(
    bindings: &Bindings,
    version: &Version,
    references: impl Iterator<Item = &'a ReferenceAssertion<'a>>,
    definitions: &HashMap<String, DefinitionAssertion<'_>>,
    failures: &mut Vec<String>,
) {
    for assertion in references {
        if let Err(failure) = check_reference_assertion(bindings, definitions, version, assertion) {
            failures.push(failure);
        }
    }
}

fn check_reference_assertion(
    bindings: &Bindings,
    definitions: &HashMap<String, DefinitionAssertion<'_>>,
    version: &Version,
    assertion: &ReferenceAssertion<'_>,
) -> Result<(), String> {
    let resolution = find_and_resolve_reference(bindings, assertion)?;

    let ReferenceAssertion {
        id, version_req, ..
    } = assertion;

    let version_matches = if let Some(version_req) = version_req {
        version_req.matches(version)
    } else {
        true
    };

    match (version_matches, id) {
        (true, None) => {
            if let Some(resolved_handle) = resolution {
                let resolved_cursor = resolved_handle.get_cursor().unwrap();
                let resolved_file = resolved_handle.get_file();
                return Err(format!(
                    "{assertion} failed: unexpected resolution to {resolved} (should not have resolved)",
                    resolved = DisplayCursor(&resolved_cursor, resolved_file)
                ));
            }
        }
        (true, Some(_)) => {
            let Some(resolved_handle) = resolution else {
                return Err(format!("{assertion} failed: did not resolve"));
            };
            let resolved_cursor = resolved_handle.get_cursor().unwrap();
            let expected_handle = lookup_referenced_definition(bindings, definitions, assertion)?;
            let expected_cursor = expected_handle.get_cursor().unwrap();
            if expected_cursor != resolved_cursor {
                return Err(format!(
                    "{assertion} failed: unexpected resolution to {resolved} (should have resolved to {expected})",
                    resolved = DisplayCursor(&resolved_cursor, resolved_handle.get_file()),
                    expected = DisplayCursor(&expected_cursor, expected_handle.get_file()),
                ));
            }
        }
        (false, None) => {
            if resolution.is_none() {
                return Err(format!("{assertion} failed: expected to resolve"));
            }
        }
        (false, Some(_)) => {
            if let Some(resolved_handle) = resolution {
                let resolved_cursor = resolved_handle.get_cursor().unwrap();
                let referenced_handle =
                    lookup_referenced_definition(bindings, definitions, assertion)?;
                let referenced_cursor = referenced_handle.get_cursor().unwrap();
                if referenced_cursor == resolved_cursor {
                    return Err(format!(
                        "{assertion} failed: expected to not resolve to {resolved}",
                        resolved = DisplayCursor(&resolved_cursor, resolved_handle.get_file()),
                    ));
                }
            }
        }
    }

    Ok(())
}

fn find_and_resolve_reference<'a>(
    bindings: &'a Bindings,
    assertion: &ReferenceAssertion<'_>,
) -> Result<Option<Definition<'a>>, String> {
    let ReferenceAssertion { cursor, .. } = assertion;

    let Some(reference) = bindings.reference_at(cursor) else {
        return Err(format!("{assertion} failed: not found"));
    };

    Ok(reference.jump_to_definition())
}

fn lookup_referenced_definition<'a>(
    bindings: &'a Bindings,
    definitions: &HashMap<String, DefinitionAssertion<'_>>,
    assertion: &ReferenceAssertion<'_>,
) -> Result<Definition<'a>, String> {
    let ReferenceAssertion { id, .. } = assertion;
    let Some(id) = id else {
        return Err(format!("{assertion} failed: should not attempt to resolve"));
    };
    let Some(definition) = definitions.get(id) else {
        return Err(format!("{assertion} failed: reference is undefined"));
    };
    find_definition(bindings, definition)
}
