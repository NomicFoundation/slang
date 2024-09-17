use std::path::PathBuf;

use anyhow::Result;
use metaslang_graph_builder::ast::File;
use slang_solidity::cst::KindTypes;
use slang_solidity::language::Language;
use slang_solidity::{bindings, diagnostic};

use crate::generated::VERSION_BREAKS;

#[test]
fn test_binding_rules_parse_successfully() {
    let binding_rules = bindings::get_binding_rules();
    let graph_builder = File::<KindTypes>::from_str(binding_rules);

    assert!(
        graph_builder.is_ok(),
        "Parsing binding rules failed:\n{}",
        graph_builder
            .err()
            .map(|err| err
                .display_pretty(&PathBuf::from("rules.msgb"), binding_rules)
                .to_string())
            .unwrap_or_default()
    );
}

#[test]
fn test_built_ins_parse_successfully() -> Result<()> {
    for version in &VERSION_BREAKS {
        let built_ins = bindings::get_built_ins(version);
        let language = Language::new(version.clone())?;
        let parse_output = language.parse(Language::ROOT_KIND, built_ins);

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
