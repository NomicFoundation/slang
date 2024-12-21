use std::path::Path;

use anyhow::Result;
use infra_utils::paths::PathExtensions;

#[test]
fn using_the_parser() -> Result<()> {
    // --8<-- [start:imports]
    use semver::Version;
    use slang_solidity::cst::{Node, NonterminalKind, TerminalKind};
    use slang_solidity::parser::Parser;
    // --8<-- [end:imports]

    let input_path = Path::repo_path("documentation/public/user-guide/inputs/using-the-parser.sol");
    let input_path = input_path.unwrap_str();

    let source = std::fs::read_to_string(input_path)?;
    let source = source.trim();

    // --8<-- [start:parse-input]
    let parser = Parser::create(Version::parse("0.8.0")?)?;

    let parse_output = parser.parse(NonterminalKind::ContractDefinition, source);
    // --8<-- [end:parse-input]

    // --8<-- [start:print-errors]
    for error in parse_output.errors() {
        eprintln!(
            "Error at byte offset {offset}: {message}",
            offset = error.text_range().start.utf8,
            message = error.message()
        );
    }
    // --8<-- [end:print-errors]

    // --8<-- [start:assert-is-valid]
    assert!(parse_output.is_valid());
    // --8<-- [end:assert-is-valid]

    // --8<-- [start:inspect-tree]
    let contract = parse_output.tree();

    assert_eq!(contract.kind, NonterminalKind::ContractDefinition);
    assert_eq!(contract.children.len(), 7);

    let children = &contract.children;
    assert!(
        matches!(&children[0].node, Node::Terminal(t) if t.kind == TerminalKind::ContractKeyword)
    );
    assert!(matches!(&children[1].node, Node::Terminal(t) if t.kind == TerminalKind::Whitespace));
    assert!(matches!(&children[2].node, Node::Terminal(t) if t.kind == TerminalKind::Identifier));
    assert!(matches!(&children[3].node, Node::Terminal(t) if t.kind == TerminalKind::Whitespace));
    assert!(matches!(&children[4].node, Node::Terminal(t) if t.kind == TerminalKind::OpenBrace));
    assert!(
        matches!(&children[5].node, Node::Nonterminal(r) if r.kind == NonterminalKind::ContractMembers)
    );
    assert!(matches!(&children[6].node, Node::Terminal(t) if t.kind == TerminalKind::CloseBrace));
    // --8<-- [end:inspect-tree]

    // --8<-- [start:unparse-node]
    assert_eq!(contract.unparse(), "contract Foo {}");
    // --8<-- [end:unparse-node]

    Ok(())
}
