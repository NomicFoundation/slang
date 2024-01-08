use anyhow::Result;
use semver::Version;
use slang_solidity::kinds::{FieldName, RuleKind, TokenKind};
use slang_solidity::language::Language;

const SOURCE: &str = include_str!("cursor_api/base.sol");

#[allow(unused_variables)] // snippet below is included as part of the docs
#[test]
fn create_cursor() -> Result<()> {
    // --8<-- [start:create-cursor]
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(RuleKind::SourceUnit, SOURCE);

    let cursor = parse_output.create_tree_cursor();
    // --8<-- [end:create-cursor]
    Ok(())
}

#[test]
fn node_accessors() -> Result<()> {
    const SOURCE: &str = include_str!("cursor_api/node_accessors.sol");
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(RuleKind::SourceUnit, SOURCE);

    let mut cursor = parse_output.create_tree_cursor();
    // --8<-- [start:example-node-accessors]

    cursor.go_to_next_rule_with_kind(RuleKind::EventParameters);

    let mut parameter_ranges = vec![];
    let mut cursor = cursor.spawn(); // Only visit children of the first event parameters node
    while cursor.go_to_next_rule_with_kind(RuleKind::EventParameter) {
        let text_value = cursor.node().unparse();
        let range = cursor.text_range();
        parameter_ranges.push((text_value, range.start.utf8..range.end.utf8));
    }

    assert_eq!(
        parameter_ranges,
        &[
            ("address indexed src".to_string(), 31..50),
            (" address indexed dst".to_string(), 51..71),
            (" uint256 value".to_string(), 72..86)
        ]
    );
    // --8<-- [end:example-node-accessors]
    Ok(())
}

#[test]
fn using_cursor_api() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(RuleKind::SourceUnit, SOURCE);

    let mut cursor = parse_output.create_tree_cursor();
    // --8<-- [start:example-list-contract-names]
    let mut contract_names = Vec::new();

    while cursor.go_to_next_rule_with_kinds(&[RuleKind::ContractDefinition]) {
        // You have to make sure you return the cursor to original position
        assert!(cursor.go_to_first_child());
        assert!(cursor.go_to_next_token_with_kinds(&[TokenKind::Identifier]));

        let token_node = cursor.node();
        contract_names.push(token_node.as_token().unwrap().text.clone());

        assert!(cursor.go_to_parent());
    }

    assert_eq!(contract_names, &["Foo", "Bar", "Baz"]);
    // --8<-- [end:example-list-contract-names]
    Ok(())
}

#[test]
fn using_spawn() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(RuleKind::SourceUnit, SOURCE);

    // --8<-- [start:example-using-spawn]
    let mut contract_names = Vec::new();
    let mut cursor = parse_output.create_tree_cursor();

    while cursor.go_to_next_rule_with_kinds(&[RuleKind::ContractDefinition]) {
        // `.spawn()` creates a new cursor without the path history, which is cheaper
        // than `.clone()`, which copies the path history.
        // Do this when you don't want to worry about restoring the position of the
        // existing cursor.
        let mut child_cursor = cursor.spawn();
        assert!(child_cursor.go_to_next_token_with_kinds(&[TokenKind::Identifier]));

        let token_node = child_cursor.node();
        contract_names.push(token_node.as_token().unwrap().text.clone());
    }

    assert_eq!(contract_names, &["Foo", "Bar", "Baz"]);
    // --8<-- [end:example-using-spawn]
    Ok(())
}

#[test]
fn using_iter() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(RuleKind::SourceUnit, SOURCE);

    // --8<-- [start:example-using-iter]
    let mut contract_names = Vec::new();
    let mut cursor = parse_output.create_tree_cursor();

    while cursor.go_to_next_rule_with_kinds(&[RuleKind::ContractDefinition]) {
        let rule_node = cursor.node();
        if let Some(token_node) = rule_node
            .as_rule()
            .unwrap()
            .children
            .iter()
            .find_map(|node| node.as_token_with_kind(TokenKind::Identifier))
        {
            contract_names.push(token_node.text.clone());
        }
    }

    assert_eq!(contract_names, &["Foo", "Bar", "Baz"]);
    // --8<-- [end:example-using-iter]
    Ok(())
}

#[test]
fn using_iter_combinators() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(RuleKind::SourceUnit, SOURCE);

    let contract_names: Vec<_> = parse_output
        .create_tree_cursor()
        .filter_map(|node| {
            let node = node.as_rule_with_kind(RuleKind::ContractDefinition)?;
            let contract_name = node
                .children
                .iter()
                .find_map(|node| node.as_token_with_kind(TokenKind::Identifier))?;

            Some(contract_name.text.clone())
        })
        .collect();

    assert_eq!(contract_names, &["Foo", "Bar", "Baz"]);
    Ok(())
}

#[test]
fn using_iter_with_node_names() -> Result<()> {
    // --8<-- [start:example-using-cursor-with-names]
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(RuleKind::SourceUnit, SOURCE);

    let names: Vec<_> = parse_output
        .create_tree_cursor()
        .with_names()
        .filter(|node| node.name == Some(FieldName::Name))
        .filter_map(|node| node.as_token_with_kind(TokenKind::Identifier).cloned())
        .map(|node| node.text.clone())
        .collect();

    assert_eq!(names, &["Foo", "Bar", "Baz"]);
    // --8<-- [end:example-using-cursor-with-names]
    Ok(())
}
