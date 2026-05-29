use num_bigint::BigInt;
use num_rational::BigRational;

use super::fixtures;
use crate::ast::{self, Number};

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
    let unit = fixtures::NumberLiterals::build_compilation_unit();
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
    let unit = fixtures::NumberLiterals::build_compilation_unit();
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
