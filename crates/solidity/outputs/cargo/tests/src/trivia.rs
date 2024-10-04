use anyhow::Result;
use semver::Version;
use slang_solidity::cst::{Node, NonterminalKind, TerminalKind};
use slang_solidity::parser::Parser;

#[test]
fn end_of_line() -> Result<()> {
    // Only one is valid as a single terminal:
    compare_end_of_lines("\r", &["\r"])?;
    compare_end_of_lines("\n", &["\n"])?;

    // Two of the same are two terminals:
    compare_end_of_lines("\n\n", &["\n", "\n"])?;
    compare_end_of_lines("\r\r", &["\r", "\r"])?;

    // A carriage return followed by a newline is one terminal, but the opposite is not:
    compare_end_of_lines("\r\n", &["\r\n"])?;
    compare_end_of_lines("\n\r", &["\n", "\r"])?;

    Ok(())
}

fn compare_end_of_lines(input: &str, expected: &[&str]) -> Result<()> {
    let version = Version::parse("0.8.0")?;
    let parser = Parser::create(version)?;

    let output = parser.parse(NonterminalKind::SourceUnit, input);
    assert!(output.is_valid());

    let actual = output
        .create_tree_cursor()
        .filter_map(|node| match node {
            Node::Nonterminal(_) => None,

            Node::Terminal(terminal) => {
                assert_eq!(terminal.kind, TerminalKind::EndOfLine);
                Some(terminal.text.clone())
            }
        })
        .collect::<Vec<_>>();

    let expected = expected.to_vec();
    assert_eq!(actual, expected);

    Ok(())
}
