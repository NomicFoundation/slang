use anyhow::Result;
use semver::Version;
use slang_solidity::kinds::{RuleKind, TokenKind};
use slang_solidity::language::Language;

const SOURCE: &str = "
    contract Foo {}
    contract Bar {}
    contract Baz {}
";

#[test]
fn using_cursor_api() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(RuleKind::SourceUnit, SOURCE);

    let mut contract_names = Vec::new();
    let mut cursor = parse_output.create_tree_cursor();

    while cursor.go_to_next_rule_with_kinds(&[RuleKind::ContractDefinition]) {
        // You have to make sure you return the cursor to original position
        assert!(cursor.go_to_first_child());
        assert!(cursor.go_to_next_token_with_kinds(&[TokenKind::Identifier]));

        let token_node = cursor.node();
        contract_names.push(token_node.as_token().unwrap().text.clone());

        assert!(cursor.go_to_parent());
    }

    assert_eq!(contract_names, &["Foo", "Bar", "Baz"]);
    Ok(())
}

#[test]
fn using_spawn() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(RuleKind::SourceUnit, SOURCE);

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
    Ok(())
}

#[test]
fn using_iter() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(RuleKind::SourceUnit, SOURCE);

    let mut contract_names = Vec::new();
    let mut cursor = parse_output.create_tree_cursor();

    while cursor.go_to_next_rule_with_kinds(&[RuleKind::ContractDefinition]) {
        let rule_node = cursor.node();
        if let Some(token_node) = rule_node
            .as_rule()
            .unwrap()
            .children
            .iter()
            .find_map(|(_name, node)| node.as_token_with_kind(&[TokenKind::Identifier]))
        {
            contract_names.push(token_node.text.clone());
        }
    }

    assert_eq!(contract_names, &["Foo", "Bar", "Baz"]);
    Ok(())
}

#[test]
fn using_iter_combinators() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(RuleKind::SourceUnit, SOURCE);

    let contract_names: Vec<_> = parse_output
        .create_tree_cursor()
        .filter_map(|node| {
            let node = node.as_rule_with_kind(&[RuleKind::ContractDefinition])?;
            let contract_name = node
                .children
                .iter()
                .find_map(|(_name, node)| node.as_token_with_kind(&[TokenKind::Identifier]))?;

            Some(contract_name.text.clone())
        })
        .collect();

    assert_eq!(contract_names, &["Foo", "Bar", "Baz"]);
    Ok(())
}

#[test]
fn using_iter_with_node_names() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(RuleKind::SourceUnit, SOURCE);

    let names: Vec<_> = parse_output
        .create_tree_cursor()
        .with_names()
        .filter_map(|(name, node)| (name == "name").then_some(node))
        .filter_map(|node| node.as_token_with_kind(&[TokenKind::Identifier]).cloned())
        .map(|node| node.text.clone())
        .collect();

    assert_eq!(names, &["Foo", "Bar", "Baz"]);
    Ok(())
}
