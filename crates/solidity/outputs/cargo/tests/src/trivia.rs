use anyhow::Result;
use semver::Version;
use slang_solidity::cst::Node;
use slang_solidity::kinds::{RuleKind, TokenKind};
use slang_solidity::language::Language;

#[test]
fn end_of_line() -> Result<()> {
    // Only one is valid as a single token:
    compare_end_of_lines("\r", &["\r"])?;
    compare_end_of_lines("\n", &["\n"])?;

    // Two of the same are two tokens:
    compare_end_of_lines("\n\n", &["\n", "\n"])?;
    compare_end_of_lines("\r\r", &["\r", "\r"])?;

    // A carriage return followed by a newline is one token, but the opposite is not:
    compare_end_of_lines("\r\n", &["\r\n"])?;
    compare_end_of_lines("\n\r", &["\n", "\r"])?;

    Ok(())
}

fn compare_end_of_lines(input: &str, expected: &[&str]) -> Result<()> {
    let version = Version::parse("0.8.0")?;
    let language = &Language::new(version)?;

    let output = language.parse(RuleKind::SourceUnit, input);
    assert!(output.is_valid());

    let actual = output
        .create_tree_cursor()
        .filter_map(|node| match node {
            Node::Rule(_) => None,

            Node::Token(token) => {
                assert_eq!(token.kind, TokenKind::EndOfLine);
                Some(token.text.clone())
            }
        })
        .collect::<Vec<_>>();

    let expected = expected.to_vec();
    assert_eq!(actual, expected);

    Ok(())
}
