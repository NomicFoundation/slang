use std::path::Path;

use anyhow::Result;
use infra_utils::paths::PathExtensions;

#[test]
fn using_the_cursor() -> Result<()> {
    // --8<-- [start:imports]
    use semver::Version;
    use slang_solidity::kinds::{FieldName, RuleKind, TokenKind};
    use slang_solidity::language::Language;
    use slang_solidity::text_index::TextRangeExtensions;
    // --8<-- [end:imports]

    let input_path = Path::repo_path("documentation/public/user-guide/inputs/using-the-cursor.sol");
    let input_path = input_path.unwrap_str();

    let source = std::fs::read_to_string(input_path)?;
    let source = source.trim();

    // --8<-- [start:parse-input]
    let language = Language::new(Version::parse("0.8.0")?)?;

    let parse_output = language.parse(RuleKind::SourceUnit, source);
    // --8<-- [end:parse-input]

    {
        // --8<-- [start:listing-contract-names]
        let mut contracts = Vec::new();

        let mut cursor = parse_output.create_tree_cursor();

        while cursor.go_to_next_rule_with_kind(RuleKind::ContractDefinition) {
            assert!(cursor.go_to_first_child());
            assert!(cursor.go_to_next_token_with_kind(TokenKind::Identifier));

            let token_node = cursor.node();
            contracts.push(token_node.as_token().unwrap().text.clone());

            // You have to make sure you return the cursor to its original position:
            assert!(cursor.go_to_parent());
        }

        assert_eq!(contracts, &["Foo", "Bar", "Baz"]);
        // --8<-- [end:listing-contract-names]
    }

    {
        // --8<-- [start:visiting-sub-tree]
        let mut contracts = Vec::new();

        let mut cursor = parse_output.create_tree_cursor();

        while cursor.go_to_next_rule_with_kind(RuleKind::ContractDefinition) {
            let mut child_cursor = cursor.spawn();
            assert!(child_cursor.go_to_next_token_with_kind(TokenKind::Identifier));

            let token_node = child_cursor.node();
            contracts.push(token_node.as_token().unwrap().text.clone());
        }

        assert_eq!(contracts, &["Foo", "Bar", "Baz"]);
        // --8<-- [end:visiting-sub-tree]
    }

    {
        // --8<-- [start:accessing-node-positions]
        let mut contracts = Vec::new();

        let mut cursor = parse_output.create_tree_cursor();

        while cursor.go_to_next_rule_with_kind(RuleKind::ContractDefinition) {
            let range = cursor.text_range().utf8();
            let text = cursor.node().as_rule().unwrap().clone().unparse();

            contracts.push((range, text.trim().to_owned()));
        }

        assert_eq!(
            contracts,
            &[
                (0..16, "contract Foo {}".to_string()),
                (16..32, "contract Bar {}".to_string()),
                (32..47, "contract Baz {}".to_string()),
            ]
        );
        // --8<-- [end:accessing-node-positions]
    }

    {
        // --8<-- [start:using-iterator-api]
        let cursor = parse_output.create_tree_cursor();

        let identifiers: Vec<_> = cursor
            .filter_map(|node| node.as_token_with_kind(TokenKind::Identifier).cloned())
            .map(|identifier| identifier.text.clone())
            .collect();

        assert_eq!(identifiers, &["Foo", "Bar", "Baz"]);
        // --8<-- [end:using-iterator-api]
    }

    {
        // --8<-- [start:using-named-cursors]
        let cursor = parse_output.create_tree_cursor();

        let identifiers: Vec<_> = cursor
            .with_names()
            .filter(|node| node.name == Some(FieldName::Name))
            .filter_map(|node| node.as_token_with_kind(TokenKind::Identifier).cloned())
            .map(|identifier| identifier.text.clone())
            .collect();

        assert_eq!(identifiers, &["Foo", "Bar", "Baz"]);
        // --8<-- [end:using-named-cursors]
    }

    Ok(())
}
