use anyhow::Result;
use semver::Version;

use slang_solidity::{
    language::Language,
    syntax::nodes::{ProductionKind, RuleKind, TokenKind},
};

#[test]
fn cursor_api() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}")?;

    let mut contract_names = Vec::new();
    let mut cursor = parse_output.parse_tree().cursor();
    while let Some(_rule_node) = cursor.find_rule_with_kind(&[RuleKind::ContractDefinition]) {
        // You have to make sure you return the cursor to original position
        if cursor.go_to_first_child() {
            while let Some(token_node) = cursor.find_token_with_kind(&[TokenKind::Identifier]) {
                contract_names.push(token_node.text.clone());
                cursor.go_to_next_non_descendent();
            }
            cursor.go_to_parent();
        }

        cursor.go_to_next_non_descendent();
        // if you wanted to continue to recurse into the
        // children of the contract definition
        // you would call `cursor.go_to_next()`
    }
    assert!(matches!(&contract_names[..], [single] if single == "Foo"));

    return Ok(());
}

#[test]
fn cursor_api_using_spawn() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}")?;

    let mut contract_names = Vec::new();
    let mut cursor = parse_output.parse_tree().cursor();
    while let Some(_rule_node) = cursor.find_rule_with_kind(&[RuleKind::ContractDefinition]) {
        // `.spawn()` creates a new cursor without the path history, which is cheaper
        // than `.clone()`, which copies the path history.
        // Do this when you don't want to worry about restoring the position of the
        // existing cursor.
        let mut child_cursor = cursor.spawn();
        if child_cursor.go_to_first_child() {
            // We don't recurse because we just want to check the immediate children
            while let Some(token_node) = child_cursor.find_token_with_kind(&[TokenKind::Identifier])
            {
                contract_names.push(token_node.text.clone());
                child_cursor.go_to_next_non_descendent();
            }
        }

        cursor.go_to_next_non_descendent();
        // if you wanted to continue to recurse into the
        // children of the contract definition
        // you would call `cursor.go_to_next()`
    }
    assert!(matches!(&contract_names[..], [single] if single == "Foo"));

    return Ok(());
}

#[test]
fn cursor_api_using_iter() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}")?;

    let mut contract_names = Vec::new();
    let mut cursor = parse_output.parse_tree().cursor();
    while let Some(_rule_node) = cursor.find_rule_with_kind(&[RuleKind::ContractDefinition]) {
        if let Some(token_node) = _rule_node
            .children
            .iter()
            .find_map(|node| node.as_token_with_kind(&[TokenKind::Identifier]))
        {
            contract_names.push(token_node.text.clone());
        }

        cursor.go_to_next_non_descendent();
        // if you wanted to continue to recurse into the
        // children of the contract definition
        // you would call `cursor.go_to_next()`
    }
    assert!(matches!(&contract_names[..], [single] if single == "Foo"));

    return Ok(());
}
