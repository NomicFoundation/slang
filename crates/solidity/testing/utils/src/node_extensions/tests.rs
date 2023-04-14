use anyhow::{Context, Result};
use semver::Version;
use slang_solidity::syntax::parser::{Language, ProductionKind};

use crate::node_extensions::NodeExtensions;

#[test]
fn extract_non_trivia() -> Result<()> {
    let source = "x = (
        // comment
        1 +
        // another comment
        2
      )";

    let language = Language::new(Version::parse("0.8.0")?)?;
    let output = language.parse(ProductionKind::Expression, source);

    assert_eq!(output.error_count(), 0);

    let parse_tree = output.parse_tree().context("Expected a parse tree")?;
    let value = parse_tree.extract_non_trivia(source);

    assert_eq!(value, "x=(1+2)");

    return Ok(());
}
