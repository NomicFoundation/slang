use ruint::aliases::U256;
use ruint::uint;

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
    values: Vec<U256>,
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
    let main_ast = unit.file(&"main.sol".into()).unwrap().ast();

    let mut visitor = YulLiteralValues::default();
    ast::visitor::accept_source_unit(&main_ast, &mut visitor);

    // `0x1234` / `42` by integer value, `true` as 1, and `"abc"` packed
    // left-aligned into the 256-bit word.
    let abc = uint!(0x6162630000000000000000000000000000000000000000000000000000000000_U256);
    assert_eq!(
        visitor.values,
        vec![uint!(0x1234_U256), uint!(42_U256), uint!(1_U256), abc],
    );
}
