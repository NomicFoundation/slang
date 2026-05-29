use super::fixtures;
use crate::ast;

#[derive(Default)]
struct IdentifierCounter {
    total: usize,
    definitions: usize,
    references: usize,
}

impl ast::visitor::Visitor for IdentifierCounter {
    fn visit_identifier(&mut self, node: &ast::Identifier) {
        if node.is_name_of_definition() {
            self.definitions += 1;
        }
        if node.is_reference() {
            self.references += 1;
        }
        self.total += 1;
    }
}

#[test]
fn test_ast_visitor() {
    let unit = fixtures::Counter::build_compilation_unit();

    let main_ast = unit.file("main.sol").unwrap().ast();

    let mut main_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&main_ast, &mut main_visitor);

    assert_eq!(main_visitor.total, 25);
    assert_eq!(main_visitor.definitions, 9);
    assert_eq!(main_visitor.references, 18);

    let ownable_ast = unit.file("ownable.sol").unwrap().ast();

    let mut ownable_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&ownable_ast, &mut ownable_visitor);

    assert_eq!(ownable_visitor.total, 11);
    assert_eq!(ownable_visitor.definitions, 3);
    assert_eq!(ownable_visitor.references, 8);

    let activatable_ast = unit.file("activatable.sol").unwrap().ast();

    let mut activatable_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&activatable_ast, &mut activatable_visitor);

    assert_eq!(activatable_visitor.total, 31);
    assert_eq!(activatable_visitor.definitions, 10);
    assert_eq!(activatable_visitor.references, 22);
}
