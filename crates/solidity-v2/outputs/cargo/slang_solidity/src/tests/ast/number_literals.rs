use num_bigint::BigInt;
use num_rational::BigRational;

use crate::ast::{self, Number};
use crate::define_fixture;

define_fixture!(
    NumberLiterals,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract NumberLiterals {
    // Hex literals
    uint constant HEX_SMALL = 0xff;
    uint constant HEX_MIXED_CASE = 0xDeAdBeEf;
    uint constant HEX_WITH_SEP = 0x1234_5678;
    uint constant HEX_ZERO = 0x0;

    // Address literal: a hex literal of exactly 40 digits is typed as an address.
    address constant ADDRESS_BEEF = 0x000000000000000000000000000000000000beef;

    // Decimal integer literals
    uint constant DEC_ZERO = 0;
    uint constant DEC_INT = 42;
    uint constant DEC_WITH_SEP = 1_000_000;

    // Decimal with scientific notation
    uint constant DEC_EXP_POS = 1e3;
    uint constant DEC_EXP_NEG_INT = 1500e-2;

    // Decimal with units
    uint constant ONE_WEI = 1 wei;
    uint constant ONE_GWEI = 1 gwei;
    uint constant ONE_ETHER = 1 ether;
    uint constant HALF_ETHER = 0.5 ether;
    uint constant ONE_HOUR = 1 hours;

    // Rational decimal literals folded to integers through multiplication
    uint constant FROM_HALF = 0.5 * 4;
    uint constant FROM_ONE_AND_HALF = 1.5 * 2;
}
"#,
);

#[derive(Default)]
struct NumberLiteralCollector {
    hex: Vec<(String, Option<BigInt>, Option<Number>)>,
    decimal: Vec<(String, Option<BigInt>, Option<Number>)>,
}

impl ast::visitor::Visitor for NumberLiteralCollector {
    fn enter_hex_number_expression(&mut self, node: &ast::HexNumberExpression) -> bool {
        let text = node.literal().unparse().to_owned();
        self.hex
            .push((text, node.integer_value(), node.number_value()));
        true
    }

    fn enter_decimal_number_expression(&mut self, node: &ast::DecimalNumberExpression) -> bool {
        let text = node.literal().unparse().to_owned();
        self.decimal
            .push((text, node.integer_value(), node.number_value()));
        true
    }
}

fn integer(value: u128) -> Number {
    Number::Integer(BigInt::from(value))
}

fn rational(numerator: i64, denominator: u64) -> Number {
    Number::Rational(BigRational::new(
        BigInt::from(numerator),
        BigInt::from(denominator),
    ))
}

#[test]
fn test_hex_number_expression_value() {
    let unit = NumberLiterals::build_compilation_unit();
    let ast = unit.file("main.sol").unwrap().ast();

    let mut collector = NumberLiteralCollector::default();
    ast::visitor::accept_source_unit(&ast, &mut collector);

    // Each entry: (literal source text, expected integer value). A 40-digit hex literal is
    // typed as an address, but its integer/number value is still surfaced as an integer.
    let expected: &[(&str, u128)] = &[
        ("0xff", 255),
        ("0xDeAdBeEf", 3_735_928_559),
        ("0x1234_5678", 305_419_896),
        ("0x0", 0),
        ("0x000000000000000000000000000000000000beef", 0xbeef),
    ];

    assert_eq!(
        collector.hex.len(),
        expected.len(),
        "unexpected number of hex literals visited: {:?}",
        collector.hex,
    );

    for ((text, integer_value, number_value), (expected_text, expected_value)) in
        collector.hex.iter().zip(expected.iter())
    {
        assert_eq!(text, expected_text);
        assert_eq!(integer_value, &Some(BigInt::from(*expected_value)));
        assert_eq!(number_value, &Some(integer(*expected_value)));
    }
}

#[test]
fn test_decimal_number_expression_value() {
    let unit = NumberLiterals::build_compilation_unit();
    let ast = unit.file("main.sol").unwrap().ast();

    let mut collector = NumberLiteralCollector::default();
    ast::visitor::accept_source_unit(&ast, &mut collector);

    // Each entry: (literal source text, expected integer value if any, expected number value).
    // The expected integer value is None for rational literals that do not reduce to an integer
    // (with their unit applied).
    let expected: &[(&str, Option<u128>, Number)] = &[
        ("0", Some(0), integer(0)),
        ("42", Some(42), integer(42)),
        ("1_000_000", Some(1_000_000), integer(1_000_000)),
        ("1e3", Some(1_000), integer(1_000)),
        ("1500e-2", Some(15), integer(15)),
        ("1", Some(1), integer(1)),
        ("1", Some(1_000_000_000), integer(1_000_000_000)),
        (
            "1",
            Some(1_000_000_000_000_000_000),
            integer(1_000_000_000_000_000_000),
        ),
        (
            "0.5",
            Some(500_000_000_000_000_000),
            integer(500_000_000_000_000_000),
        ),
        ("1", Some(3_600), integer(3_600)),
        ("0.5", None, rational(1, 2)),
        ("4", Some(4), integer(4)),
        ("1.5", None, rational(3, 2)),
        ("2", Some(2), integer(2)),
    ];

    assert_eq!(
        collector.decimal.len(),
        expected.len(),
        "unexpected number of decimal literals visited: {:?}",
        collector.decimal,
    );

    for ((text, integer_value, number_value), (expected_text, expected_integer, expected_number)) in
        collector.decimal.iter().zip(expected.iter())
    {
        assert_eq!(text, expected_text);
        assert_eq!(
            integer_value,
            &expected_integer.map(BigInt::from),
            "integer_value mismatch for literal {text:?}",
        );
        assert_eq!(
            number_value,
            &Some(expected_number.clone()),
            "number_value mismatch for literal {text:?}",
        );
    }
}

define_fixture!(
    ConstantExpressions,
    file: "main.sol", r#"
contract Test {
    uint256 SUM = ((1 + 2));
    address ADDR = 0x000000000000000000000000000000000000beef;
}
"#,
);

#[test]
fn test_expression_integer_value_and_unwrap_parentheses() {
    let unit = ConstantExpressions::build_compilation_unit();
    let test = unit
        .find_contract_by_name("Test")
        .next()
        .expect("can find Test contract");
    let state_variables = test.state_variables();
    let value_of = |name: &str| {
        state_variables
            .iter()
            .find(|variable| variable.name().name() == name)
            .and_then(|variable| variable.value())
            .unwrap_or_else(|| panic!("`{name}` should have an initializer"))
    };

    // `((1 + 2))` peels to `1 + 2`, a computed constant that folds to 3.
    assert_eq!(
        value_of("SUM").unwrap_parentheses().integer_value(),
        Some(BigInt::from(3)),
    );

    // A 40-digit hex literal is typed as an address; its integer value is surfaced.
    assert_eq!(
        value_of("ADDR").integer_value(),
        Some(BigInt::from(0xbeef_u32))
    );
}
