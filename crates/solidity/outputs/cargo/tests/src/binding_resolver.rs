use std::rc::Rc;

use anyhow::Result;
use semver::Version;
use slang_solidity::bindings;
use slang_solidity::cst::{Cursor, Query};
use slang_solidity::parser::Parser;

use crate::resolver::TestsPathResolver;

const TEST_VERSION: Version = Version::new(0, 8, 26);

const INPUT_FILE: &str = r##"
contract Base {}
contract Middle is Base {}
contract Test is Base, Middle {}
"##;

fn find_first_match(root_cursor: Cursor, query_string: &str, capture_name: &str) -> Result<Cursor> {
    let query = Query::create(query_string)?;
    let query_match = root_cursor.query(vec![query]).next();
    let query_match = query_match.expect("query to succeed");
    Ok(query_match.captures[capture_name]
        .first()
        .expect("query to capture identifier")
        .clone())
}

fn find_all_matches(
    root_cursor: Cursor,
    query_string: &str,
    capture_name: &str,
) -> Result<Vec<Cursor>> {
    let query = Query::create(query_string)?;
    let mut results = Vec::new();
    for query_match in root_cursor.query(vec![query]) {
        let cursor = query_match.captures[capture_name]
            .first()
            .expect("query to capture identifier");
        results.push(cursor.clone());
    }
    Ok(results)
}

#[test]
fn test_resolve_references_from_definition() -> Result<()> {
    let version = TEST_VERSION;
    let parser = Parser::create(version.clone())?;
    let mut builder =
        bindings::create_with_resolver(version.clone(), Rc::new(TestsPathResolver {}))?;

    let parse_output = parser.parse_file_contents(INPUT_FILE);
    builder.add_user_file("input.sol", parse_output.create_tree_cursor());

    let binding_graph = builder.build();
    let root_cursor = parse_output.create_tree_cursor();

    // "Base" identifier
    let base_cursor = find_first_match(
        root_cursor.clone(),
        "[ContractDefinition @identifier [\"Base\"]]",
        "identifier",
    )?;
    let base_definition = binding_graph
        .definition_at(&base_cursor)
        .expect("Base definition to be found");
    let base_references = base_definition.references();
    assert_eq!(2, base_references.len());

    let base_ref_cursors = find_all_matches(
        root_cursor.clone(),
        "[IdentifierPath @identifier [\"Base\"]]",
        "identifier",
    )?;
    for base_ref in &base_references {
        assert!(base_ref_cursors.contains(base_ref.get_cursor()));
    }

    // "Middle" identifier
    let middle_cursor = find_first_match(
        root_cursor.clone(),
        "[ContractDefinition @identifier [\"Middle\"]]",
        "identifier",
    )?;
    let middle_definition = binding_graph
        .definition_at(&middle_cursor)
        .expect("Middle definition to be found");
    let middle_references = middle_definition.references();
    assert_eq!(1, middle_references.len());

    let middle_ref_cursors = find_all_matches(
        root_cursor.clone(),
        "[IdentifierPath @identifier [\"Middle\"]]",
        "identifier",
    )?;
    for middle_ref in &middle_references {
        assert!(middle_ref_cursors.contains(middle_ref.get_cursor()));
    }

    // "Test" identifier
    let test_cursor = find_first_match(
        root_cursor.clone(),
        "[ContractDefinition @identifier [\"Test\"]]",
        "identifier",
    )?;
    let test_definition = binding_graph
        .definition_at(&test_cursor)
        .expect("Test definition to be found");
    let test_references = test_definition.references();
    assert_eq!(0, test_references.len());

    let test_ref_cursors = find_all_matches(
        root_cursor.clone(),
        "[IdentifierPath @identifier [\"Test\"]]",
        "identifier",
    )?;
    assert!(test_ref_cursors.is_empty());

    Ok(())
}
