use num_bigint::BigInt;

use crate::{ast, define_fixture};

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
        self.values.push(node.integer_value());
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
