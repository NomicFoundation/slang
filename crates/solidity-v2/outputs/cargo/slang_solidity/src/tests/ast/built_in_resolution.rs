use super::fixtures;
use crate::ast::{self, BuiltIn};

#[test]
fn test_resolve_to_built_in() {
    let unit = fixtures::Counter::build_compilation_unit();

    // Collect all identifiers that resolve to built-ins from the ownable.sol file,
    // which contains `msg.sender` and `require`.
    let ownable_ast = unit.file(&"ownable.sol".into()).unwrap().ast();

    let mut built_in_collector = BuiltInCollector::default();
    ast::visitor::accept_source_unit(&ownable_ast, &mut built_in_collector);

    assert!(
        built_in_collector
            .found
            .contains(&("msg".to_string(), BuiltIn::Msg)),
        "expected Msg built-in, found: {:?}",
        built_in_collector.found,
    );
    assert!(
        built_in_collector
            .found
            .contains(&("sender".to_string(), BuiltIn::MsgSender)),
        "expected MsgSender built-in, found: {:?}",
        built_in_collector.found,
    );
    assert!(
        built_in_collector
            .found
            .contains(&("require".to_string(), BuiltIn::Require)),
        "expected Require built-in, found: {:?}",
        built_in_collector.found,
    );
}

#[derive(Default)]
struct BuiltInCollector {
    found: Vec<(String, BuiltIn)>,
}

impl ast::visitor::Visitor for BuiltInCollector {
    fn visit_identifier(&mut self, node: &ast::Identifier) {
        if let Some(built_in) = node.resolve_to_built_in() {
            self.found.push((node.name(), built_in));
        }
    }
}
