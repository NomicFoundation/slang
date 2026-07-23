//! Exercises `common_operand_type` on the binary operator AST nodes: the type
//! both operands of a binary operator reconcile to before the operator runs,
//! as recorded by the typing pass (solc's `commonType` annotation).

use crate::ast::BinaryOperatorExpression;
use crate::{ast, define_fixture};

define_fixture!(
    CommonOperands,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

type Wrapped is int128;
using {add as +, eq as ==, lt as <} for Wrapped global;

function add(Wrapped a, Wrapped b) pure returns (Wrapped) { return a; }
function eq(Wrapped a, Wrapped b) pure returns (bool) { return true; }
function lt(Wrapped a, Wrapped b) pure returns (bool) { return false; }

contract CommonOperands {
    enum Color { Red, Green }

    function f() internal pure {}
    function g() internal pure {}

    function run(
        uint8 small,
        uint256 big,
        address first,
        address second,
        bytes2 two,
        bytes4 four,
        bool yes,
        bool no,
        Color c1,
        Color c2,
        Wrapped w1,
        Wrapped w2
    ) public pure {
        // Arithmetic and bitwise operands reconcile through their common type,
        // which is also the operator's own type.
        small + big;
        1 + 2;
        small * small;
        big & 0xff;
        two | four;

        // Shifts and exponentiation follow their left operand.
        big << small;
        two << 2;
        big ** small;
        2 ** small;

        // `&&` and `||` reconcile both operands to `bool`.
        yes && no;
        yes || no;

        // Comparison operands reconcile through their common type, while the
        // comparison itself types as `bool`.
        small < big;
        big == 5;
        1 < 2;
        first == second;
        0x1111111111111111111111111111111111111111 ==
            0x2222222222222222222222222222222222222222;
        two < four;
        yes == no;
        c1 == c2;
        c1 < c2;
        f == g;

        // Operators defined through `using {..} for T` reconcile both
        // operands to `T`.
        w1 + w2;
        w1 == w2;
        w1 < w2;
    }
}
"#,
);

#[derive(Default)]
struct CommonOperandCollector {
    common_types: Vec<Option<ast::Type>>,
}

macro_rules! collect_common_operand_type {
    ($($hook:ident: $node:ident),* $(,)?) => {
        $(
            fn $hook(&mut self, node: &ast::$node) -> bool {
                self.common_types.push(node.common_operand_type());
                true
            }
        )*
    };
}

impl ast::visitor::Visitor for CommonOperandCollector {
    collect_common_operand_type!(
        enter_additive_expression: AdditiveExpression,
        enter_and_expression: AndExpression,
        enter_bitwise_and_expression: BitwiseAndExpression,
        enter_bitwise_or_expression: BitwiseOrExpression,
        enter_bitwise_xor_expression: BitwiseXorExpression,
        enter_equality_expression: EqualityExpression,
        enter_exponentiation_expression: ExponentiationExpression,
        enter_inequality_expression: InequalityExpression,
        enter_multiplicative_expression: MultiplicativeExpression,
        enter_or_expression: OrExpression,
        enter_shift_expression: ShiftExpression,
    );
}

#[test]
fn binary_operators_record_their_common_operand_type() {
    let unit = CommonOperands::build_compilation_unit();
    let ast = unit.file(&"main.sol".into()).unwrap().ast();

    let mut collector = CommonOperandCollector::default();
    ast::visitor::accept_source_unit(&ast, &mut collector);

    // The collector visits the binary operators in source order; `next` pops
    // the one matching the given source text (the text is just for readable
    // failures).
    let mut common_types = collector.common_types.into_iter();
    let mut next = |source: &str| {
        common_types
            .next()
            .unwrap_or_else(|| panic!("`{source}` is visited"))
    };

    // Arithmetic: the smaller side widens into the common type, two literals
    // fold into a literal.
    let common = next("small + big").expect("uint8 and uint256 reconcile");
    assert!(matches!(common, ast::Type::Integer(t) if !t.is_signed() && t.bits() == 256));
    let common = next("1 + 2").expect("two literals fold");
    assert!(matches!(common, ast::Type::Literal(_)));
    let common = next("small * small").expect("uint8 operands reconcile");
    assert!(matches!(common, ast::Type::Integer(t) if !t.is_signed() && t.bits() == 8));
    let common = next("big & 0xff").expect("uint256 and a literal reconcile");
    assert!(matches!(common, ast::Type::Integer(t) if !t.is_signed() && t.bits() == 256));
    let common = next("two | four").expect("fixed bytes reconcile");
    assert!(matches!(common, ast::Type::ByteArray(t) if t.width() == 4));

    // Shifts and exponentiation follow their left operand.
    let common = next("big << small").expect("shifts follow the left operand");
    assert!(matches!(common, ast::Type::Integer(t) if !t.is_signed() && t.bits() == 256));
    let common = next("two << 2").expect("fixed bytes support shifts");
    assert!(matches!(common, ast::Type::ByteArray(t) if t.width() == 2));
    let common = next("big ** small").expect("exponentiation follows the left operand");
    assert!(matches!(common, ast::Type::Integer(t) if !t.is_signed() && t.bits() == 256));
    let common = next("2 ** small").expect("a literal base widens to uint256");
    assert!(matches!(common, ast::Type::Integer(t) if !t.is_signed() && t.bits() == 256));

    // `&&` and `||` reconcile both operands to `bool`.
    let common = next("yes && no").expect("boolean conjunction reconciles");
    assert!(matches!(common, ast::Type::Boolean(_)));
    let common = next("yes || no").expect("boolean disjunction reconciles");
    assert!(matches!(common, ast::Type::Boolean(_)));

    // Comparisons record the type the operands are compared at.
    let common = next("small < big").expect("uint8 and uint256 reconcile");
    assert!(matches!(common, ast::Type::Integer(t) if !t.is_signed() && t.bits() == 256));
    let common = next("big == 5").expect("uint256 and a literal reconcile");
    assert!(matches!(common, ast::Type::Integer(t) if !t.is_signed() && t.bits() == 256));
    // Two number literals are mobilised before reconciling.
    let common = next("1 < 2").expect("two small literals reconcile");
    assert!(matches!(common, ast::Type::Integer(t) if !t.is_signed() && t.bits() == 8));
    let common = next("first == second").expect("addresses reconcile");
    assert!(matches!(common, ast::Type::Address(t) if !t.is_payable()));
    let common = next("0x11..11 == 0x22..22").expect("two address literals reconcile");
    assert!(matches!(common, ast::Type::Address(t) if !t.is_payable()));
    let common = next("two < four").expect("fixed bytes reconcile");
    assert!(matches!(common, ast::Type::ByteArray(t) if t.width() == 4));
    let common = next("yes == no").expect("booleans support equality");
    assert!(matches!(common, ast::Type::Boolean(_)));
    let common = next("c1 == c2").expect("identical enums reconcile");
    assert!(matches!(common, ast::Type::Enum(_)));
    let common = next("c1 < c2").expect("enums support ordering");
    assert!(matches!(common, ast::Type::Enum(_)));
    let common = next("f == g").expect("matching function values reconcile");
    assert!(matches!(common, ast::Type::Function(_)));

    // Operators defined through `using {..} for T` reconcile both operands to
    // `T`.
    let common = next("w1 + w2").expect("user-defined addition reconciles");
    assert!(matches!(common, ast::Type::UserDefinedValue(_)));
    let common = next("w1 == w2").expect("user-defined equality reconciles");
    assert!(matches!(common, ast::Type::UserDefinedValue(_)));
    let common = next("w1 < w2").expect("user-defined ordering reconciles");
    assert!(matches!(common, ast::Type::UserDefinedValue(_)));

    assert!(
        common_types.next().is_none(),
        "all binary operators are asserted on"
    );
}
