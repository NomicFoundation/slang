use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use infra_utils::paths::PathExtensions;

#[test]
fn using_the_parser() -> Result<()> {
    // --8<-- [start:imports]
    use semver::Version;
    use slang_solidity::cst::Node;
    use slang_solidity::kinds::{NonTerminalKind, TerminalKind};
    use slang_solidity::language::Language;
    // --8<-- [end:imports]

    let input_path = Path::repo_path("documentation/public/user-guide/inputs/using-the-parser.sol");
    let input_path = input_path.unwrap_str();

    let source = std::fs::read_to_string(input_path)?;
    let source = source.trim();

    // --8<-- [start:parse-input]
    let language = Language::new(Version::parse("0.8.0")?)?;

    let parse_output = language.parse(NonTerminalKind::ContractDefinition, source);
    // --8<-- [end:parse-input]

    // --8<-- [start:print-errors]
    for error in parse_output.errors() {
        eprintln!("{}", error.to_error_report(input_path, source, true));
    }
    // --8<-- [end:print-errors]

    // --8<-- [start:assert-is-valid]
    assert!(parse_output.is_valid());
    // --8<-- [end:assert-is-valid]

    // --8<-- [start:inspect-tree]
    let parse_tree = parse_output.tree();

    let contract = parse_tree.as_nonterminal().unwrap();
    assert_eq!(contract.kind, NonTerminalKind::ContractDefinition);
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
        matches!(&children[5].node, Node::NonTerminal(r) if r.kind == NonTerminalKind::ContractMembers)
    );
    assert!(matches!(&children[6].node, Node::Terminal(t) if t.kind == TerminalKind::CloseBrace));
    // --8<-- [end:inspect-tree]

    // --8<-- [start:unparse-node]
    let contract_source = Rc::clone(contract).unparse();
    assert_eq!(contract_source, "contract Foo {}");
    // --8<-- [end:unparse-node]

    Ok(())
}
