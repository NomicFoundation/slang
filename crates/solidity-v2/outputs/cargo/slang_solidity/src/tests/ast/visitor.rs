use num_bigint::BigInt;

use super::fixtures;
use crate::{ast, define_fixture};

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

define_fixture!(
    YulLiterals,
    file: "main.sol", r#"
contract Test {
    function f() public pure {
        assembly {
            let a := 0x1234
            let b := 42
            let c := true
            let d := "abc"
        }
    }
}
"#,
);

#[derive(Default)]
struct YulLiteralValues {
    values: Vec<BigInt>,
}

impl ast::visitor::Visitor for YulLiteralValues {
    fn enter_yul_literal(&mut self, node: &ast::YulLiteral) -> bool {
        self.values.push(node.value());
        true
    }
}

#[test]
fn test_yul_literal_value() {
    let unit = YulLiterals::build_compilation_unit();
    let main_ast = unit.file("main.sol").unwrap().ast();

    let mut visitor = YulLiteralValues::default();
    ast::visitor::accept_source_unit(&main_ast, &mut visitor);

    // `0x1234` / `42` by integer value, `true` as 1, and `"abc"` packed
    // left-aligned into the 256-bit word.
    let abc = BigInt::parse_bytes(
        b"6162630000000000000000000000000000000000000000000000000000000000",
        16,
    )
    .unwrap();
    assert_eq!(
        visitor.values,
        vec![BigInt::from(0x1234), BigInt::from(42), BigInt::from(1), abc],
    );
}
