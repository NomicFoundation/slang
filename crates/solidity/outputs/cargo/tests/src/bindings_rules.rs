use metaslang_graph_builder::ast::File;
use slang_solidity::bindings;
use slang_solidity::cst::KindTypes;

#[test]
fn test_bindings_rules_parsing() {
    let graph_builder = File::<KindTypes>::from_str(bindings::get_binding_rules());

    assert!(graph_builder.is_ok());
}
