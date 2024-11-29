use std::path::PathBuf;

use metaslang_graph_builder::ast::File;
use slang_solidity::bindings;
use slang_solidity::cst::KindTypes;

#[test]
fn test_user_binding_rules_parse_successfully() {
    let binding_rules = bindings::get_user_binding_rules();
    let graph_builder = File::<KindTypes>::from_str(binding_rules);

    assert!(
        graph_builder.is_ok(),
        "Parsing user binding rules failed:\n{}",
        graph_builder
            .err()
            .map(|err| err
                .display_pretty(&PathBuf::from("user-rules.msgb"), binding_rules)
                .to_string())
            .unwrap_or_default()
    );
}

#[test]
fn test_system_binding_rules_parse_successfully() {
    let binding_rules = bindings::get_system_binding_rules();
    let graph_builder = File::<KindTypes>::from_str(binding_rules);

    assert!(
        graph_builder.is_ok(),
        "Parsing system binding rules failed:\n{}",
        graph_builder
            .err()
            .map(|err| err
                .display_pretty(&PathBuf::from("system-rules.msgb"), binding_rules)
                .to_string())
            .unwrap_or_default()
    );
}
