//! Literal typing and compile-time constant folding: number and string
//! literals, the arithmetic folded over them, and the folded values consumed as
//! array lengths and storage base slots.

use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;

use super::expression;
use crate::types::{AddressType, LiteralKind, Type};

#[test]
fn test_value_bearing_integer_literal_types() {
    let (type_, _) = expression("127").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(127)
        })
    );

    let (type_, _) = expression("-128").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-128)
        })
    );

    // Hex literals carry source byte width as `HexInteger`, distinct from
    // decimal `Integer` so the byte-array conversion rule can fire.
    let (type_, _) = expression("0xff").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::HexInteger {
            value: BigUint::from(255u32),
            bytes: 1,
        })
    );

    // Source byte width is preserved across leading zeros.
    let (type_, _) = expression("0x0012").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::HexInteger {
            value: BigUint::from(18u32),
            bytes: 2,
        })
    );

    // An odd digit count spans no whole number of bytes: a plain `Integer`.
    let (type_, _) = expression("0x123").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(291)
        })
    );

    // Folding a hex literal demotes it to a plain `Integer` (provenance lost).
    let (type_, _) = expression("0x10 + 0").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(16)
        })
    );
}

#[test]
fn test_address_literal_types_as_address() {
    let address = Some(Type::Address(AddressType { is_payable: false }));
    let (type_, _) =
        expression("0xb701a286753ADe3704e1DcFD93507454A2307641").into_type_and_diagnostic();
    assert_eq!(type_, address);
    let (type_, _) =
        expression("(0xb701a286753ADe3704e1DcFD93507454A2307641)").into_type_and_diagnostic();
    assert_eq!(type_, address);

    // `address` has no arithmetic, so the literal does not fold as a number.
    let (type_, _) =
        expression("0xb701a286753ADe3704e1DcFD93507454A2307641 + 1").into_type_and_diagnostic();
    assert_eq!(type_, None);
}

#[test]
fn test_binary_arithmetic_folds_to_narrowed_literal() {
    // Addition.
    let (type_, _) = expression("1 + 1").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(2)
        })
    );

    // Multiplication.
    let (type_, _) = expression("3 * 4").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(12)
        })
    );

    // Power.
    let (type_, _) = expression("2 ** 10").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(1024)
        })
    );

    // Shift.
    let (type_, _) = expression("1 << 32").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(1u64 << 32)
        })
    );

    // Reducible rational arithmetic normalises back to an integer.
    let (type_, _) = expression("1.5 * 2").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(3)
        })
    );

    // Non-reducing rational division stays rational.
    let (type_, _) = expression("5 / 2").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Rational {
            value: BigRational::new(BigInt::from(5), BigInt::from(2))
        })
    );

    // Negation of a folded constant.
    let (type_, _) = expression("-(1 + 1)").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-2)
        })
    );
}

#[test]
fn test_binary_bitwise_folds_to_literal() {
    // OR
    let (type_, _) = expression("1 | 2").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(3)
        })
    );

    // AND
    let (type_, _) = expression("12 & 10").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(8)
        })
    );

    // XOR
    let (type_, _) = expression("6 ^ 3").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(5)
        })
    );

    // Folding hex operands demotes the result to a plain `Integer`
    // (mirroring the additive folding behaviour).
    let (type_, _) = expression("0xf0 | 0x0f").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(0xff)
        })
    );

    // Bitwise AND with a negative literal: BigInt uses arbitrary-precision
    // two's-complement, so `-1 & 0xff` masks to the low byte.
    let (type_, _) = expression("(-1) & 0xff").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(0xff)
        })
    );

    // Bitwise OR of a folded constant feeds further folding.
    let (type_, _) = expression("(1 | 2) ^ 4").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(7)
        })
    );
}

#[test]
fn test_bitwise_not_folds_to_literal() {
    // ~x = -x - 1 (two's complement on an infinite-precision integer).
    let (type_, _) = expression("~1").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-2)
        })
    );

    let (type_, _) = expression("~0").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-1)
        })
    );

    // Double-complement returns to the original value.
    let (type_, _) = expression("~(-1)").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(0)
        })
    );

    // Folding `~hex` demotes the result to a plain `Integer`.
    let (type_, _) = expression("~0xff").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-256)
        })
    );

    // `~` of a folded constant.
    let (type_, _) = expression("~(1 | 2)").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-4)
        })
    );
}

#[test]
fn test_bitwise_operations_unresolved_for_rationals() {
    // Bitwise binary operators don't apply to non-reducing rationals.
    let (type_, _) = expression("1.5 | 1").into_type();
    assert_eq!(type_, None);

    let (type_, _) = expression("1 & 0.5").into_type();
    assert_eq!(type_, None);

    let (type_, _) = expression("0.5 ^ 0.25").into_type();
    assert_eq!(type_, None);

    // Likewise for the unary bitwise NOT.
    let (type_, _) = expression("~0.5").into_type();
    assert_eq!(type_, None);
}

fn string_literal(value: &[u8]) -> Type {
    Type::Literal(LiteralKind::String {
        value: value.into(),
    })
}

fn hex_string_literal(value: &[u8]) -> Type {
    Type::Literal(LiteralKind::HexString {
        value: value.into(),
    })
}

#[test]
fn test_string_literal_value_with_escapes() {
    // Plain ASCII: one byte per char.
    let (type_, _) = expression(r#""abc""#).into_resolved_type();
    assert_eq!(type_, string_literal(b"abc"));

    // Each `\n`, `\t`, etc. decodes to a single byte.
    let (type_, _) = expression(r#""\n\t\\""#).into_resolved_type();
    assert_eq!(type_, string_literal(b"\n\t\\"));

    // `\xNN` escapes decode to one byte each, regardless of the 4-char source
    // length per escape.
    let (type_, _) = expression(r#""\x41\x42""#).into_resolved_type();
    assert_eq!(type_, string_literal(b"AB"));

    // Line continuations (`\<newline>`) decode to nothing.
    let (type_, _) = expression("\"a\\\nb\"").into_resolved_type();
    assert_eq!(type_, string_literal(b"ab"));

    // Concatenated string literals: the pieces are joined.
    let (type_, _) = expression(r#""abc" "de""#).into_resolved_type();
    assert_eq!(type_, string_literal(b"abcde"));
}

#[test]
fn test_unicode_string_literal_value() {
    // ASCII unicode-string literal: one byte per char.
    let (type_, _) = expression(r#"unicode"abc""#).into_resolved_type();
    assert_eq!(type_, string_literal(b"abc"));

    // Multi-byte UTF-8 passes through as its encoding: `€` is 3 bytes.
    let (type_, _) = expression(r#"unicode"€""#).into_resolved_type();
    assert_eq!(type_, string_literal("€".as_bytes()));

    // `\uNNNN` escapes decode to their UTF-8 encoding:
    // `\u20AC` (€) → 3 bytes, `\u00A2` (¢) → 2 bytes, `\u0024` ($) → 1 byte.
    let (type_, _) = expression(r#"unicode"\u20AC\u00A2\u0024""#).into_resolved_type();
    assert_eq!(type_, string_literal("€¢$".as_bytes()));
}

#[test]
fn test_hex_string_literal_value() {
    // Pairs of hex digits, no separators: one byte per pair.
    let (type_, _) = expression(r#"hex"414243""#).into_resolved_type();
    assert_eq!(type_, hex_string_literal(b"ABC"));

    // Underscore separators don't contribute to the decoded value.
    let (type_, _) = expression(r#"hex"41_42""#).into_resolved_type();
    assert_eq!(type_, hex_string_literal(b"AB"));

    // Concatenated hex string literals: the pieces are joined.
    let (type_, _) = expression(r#"hex"4142" hex"43""#).into_resolved_type();
    assert_eq!(type_, hex_string_literal(b"ABC"));
}
