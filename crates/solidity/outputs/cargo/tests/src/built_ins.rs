use anyhow::Result;
use slang_solidity::bindings::built_ins::get_built_ins_contents;
use slang_solidity::diagnostic;
use slang_solidity::parser::Parser;

use crate::generated::VERSION_BREAKS;

#[test]
fn test_built_ins_parse_successfully() -> Result<()> {
    for version in &VERSION_BREAKS {
        let built_ins = get_built_ins_contents(version);
        let parser = Parser::create(version.clone())?;
        let parse_output = parser.parse_file_contents(built_ins);

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
