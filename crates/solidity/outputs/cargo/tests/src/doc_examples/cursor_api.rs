use anyhow::Result;
use semver::Version;

use slang_solidity::{
    cst::Node,
    kinds::{ProductionKind, RuleKind, TokenKind},
    language::Language,
};

#[test]
fn cursor_api() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}");

    let mut contract_names = Vec::new();
    let mut cursor = parse_output.create_tree_cursor();
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

    Ok(())
}

#[test]
fn cursor_api_using_spawn() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}");

    let mut contract_names = Vec::new();
    let mut cursor = parse_output.create_tree_cursor();
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

    Ok(())
}

#[test]
fn cursor_api_using_iter() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}");

    let mut contract_names = Vec::new();
    let mut cursor = parse_output.create_tree_cursor();
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

    Ok(())
}

#[test]
fn cursor_api_using_iter_combinators() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}");

    let cursor = parse_output.create_tree_cursor();

    let contract_names: Vec<_> = cursor
        .filter_map(|node| {
            let node = node.as_rule_with_kind(&[RuleKind::ContractDefinition])?;
            let name = node
                .children
                .iter()
                .find_map(|node| node.as_token_with_kind(&[TokenKind::Identifier]))?;

            Some(name.text.clone())
        })
        .collect();

    assert_eq!(contract_names, &["Foo"]);

    Ok(())
}

#[test]
#[allow(clippy::redundant_pattern_matching)]
fn cursor_as_iter() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}");

    let mut cursor = parse_output.create_tree_cursor();
    assert_eq!(
        cursor.node().as_rule().unwrap().kind,
        RuleKind::ContractDefinition
    );

    macro_rules! assert_next_is {
        ($pattern:pat $(if $guard:expr)? $(,)?) => {
            assert!(matches!(cursor.next(), $pattern $(if $guard)?));
        };
    }

    assert_next_is!(Some(Node::Rule(rule)) if rule.kind == RuleKind::ContractDefinition);
    {
        assert_next_is!(Some(Node::Token(token)) if token.kind == TokenKind::ContractKeyword);
        assert_next_is!(Some(Node::Rule(rule)) if rule.kind == RuleKind::LeadingTrivia);
        assert_next_is!(Some(Node::Token(token)) if token.kind == TokenKind::Whitespace);
        assert_next_is!(Some(Node::Token(token)) if token.kind == TokenKind::Identifier && token.text == "Foo");
        assert_next_is!(Some(Node::Rule(rule)) if rule.kind == RuleKind::LeadingTrivia);
        assert_next_is!(Some(Node::Token(token)) if token.kind == TokenKind::Whitespace);
        assert_next_is!(Some(Node::Token(token)) if token.kind == TokenKind::OpenBrace);
        assert_next_is!(Some(Node::Token(token)) if token.kind == TokenKind::CloseBrace);
    }
    assert_next_is!(None);

    Ok(())
}
