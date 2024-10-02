use anyhow::Result;
use slang_solidity::parser::Parser;
use slang_solidity::{bindings, diagnostic};

use crate::generated::VERSION_BREAKS;

#[test]
fn test_built_ins_parse_successfully() -> Result<()> {
    for version in &VERSION_BREAKS {
        let built_ins = bindings::get_built_ins(version);
        let parser = Parser::new(version.clone())?;
        let parse_output = parser.parse(Parser::ROOT_KIND, built_ins);

        let report = parse_output
            .errors()
            .iter()
            .map(|error| diagnostic::render(error, "built-ins", built_ins, false))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            parse_output.is_valid(),
            "Failed to parse built-ins with version {version}: {report}"
        );
    }

    Ok(())
}
