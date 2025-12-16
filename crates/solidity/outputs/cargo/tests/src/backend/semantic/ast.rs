use anyhow::Result;
use slang_solidity::backend::semantic::ast;

use crate::backend::sample::build_compilation_unit;

#[test]
fn test_ast_visitor() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let main_ast = semantic.get_file_ast_root("main.sol").unwrap();

    let mut visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&main_ast, &mut visitor);

    assert_eq!(visitor.total, 25);
    assert_eq!(visitor.definitions, 9);
    assert_eq!(visitor.references, 18);

    Ok(())
}

#[derive(Default)]
struct IdentifierCounter {
    total: usize,
    definitions: usize,
    references: usize,
}

impl ast::visitor::Visitor for IdentifierCounter {
    fn visit_identifier(&mut self, node: &ast::Identifier) {
        if node.is_definition() {
            self.definitions += 1;
        }
        if node.is_reference() {
            self.references += 1;
        }
        self.total += 1;
    }
}
