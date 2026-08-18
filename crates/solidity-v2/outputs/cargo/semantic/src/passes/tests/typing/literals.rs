//! Literal typing and compile-time constant folding: number and string
//! literals, the arithmetic folded over them, and the folded values consumed as
//! array lengths and storage base slots.

use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use ruint::aliases::U256;
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    ArrayLengthFractional, ArrayLengthNotConstant, ArrayLengthZero, ConstantArithmeticError,
    IncompatibleConstantOperator, StorageLayoutBaseNotConstant,
};

use super::{contract_base_slot, expression, folded_array_length};
use crate::types::{LiteralKind, Type};

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

#[test]
fn test_string_literal_byte_count_with_escapes() {
    // Plain ASCII: one byte per char.
    let (type_, _) = expression(r#""abc""#).into_resolved_type();
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 3 }));

    // Each `\n`, `\t`, etc. decodes to a single byte.
    let (type_, _) = expression(r#""\n\t\\""#).into_resolved_type();
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 3 }));

    // `\xNN` escapes decode to one byte each, regardless of the 4-char source
    // length per escape.
    let (type_, _) = expression(r#""\x41\x42""#).into_resolved_type();
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 2 }));

    // Line continuations (`\<newline>`) decode to nothing.
    let (type_, _) = expression("\"a\\\nb\"").into_resolved_type();
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 2 }));

    // Concatenated string literals: byte counts add up across pieces.
    let (type_, _) = expression(r#""abc" "de""#).into_resolved_type();
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 5 }));
}

#[test]
fn test_unicode_string_literal_byte_count() {
    // ASCII unicode-string literal: one byte per char.
    let (type_, _) = expression(r#"unicode"abc""#).into_resolved_type();
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 3 }));

    // Multi-byte UTF-8 passes through with its full byte length:
    // `€` is 3 bytes in UTF-8.
    let (type_, _) = expression(r#"unicode"€""#).into_resolved_type();
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 3 }));

    // `\uNNNN` escapes decode to their UTF-8 byte length:
    // `\u20AC` (€) → 3 bytes, `\u00A2` (¢) → 2 bytes, `\u0024` ($) → 1 byte.
    let (type_, _) = expression(r#"unicode"\u20AC\u00A2\u0024""#).into_resolved_type();
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 6 }));
}

#[test]
fn test_hex_string_literal_byte_count() {
    // Pairs of hex digits, no separators: one byte per pair.
    let (type_, _) = expression(r#"hex"414243""#).into_resolved_type();
    assert_eq!(type_, Type::Literal(LiteralKind::HexString { bytes: 3 }));

    // Underscore separators don't contribute to the decoded length.
    let (type_, _) = expression(r#"hex"41_42""#).into_resolved_type();
    assert_eq!(type_, Type::Literal(LiteralKind::HexString { bytes: 2 }));

    // Concatenated hex string literals: byte counts add up across pieces.
    let (type_, _) = expression(r#"hex"4142" hex"43""#).into_resolved_type();
    assert_eq!(type_, Type::Literal(LiteralKind::HexString { bytes: 3 }));
}

#[test]
fn test_array_length_folds_with_typed_constants() {
    let uint256_b = |value: &str| format!("uint256 constant B = {value};");

    // Division by a typed integer constant truncates toward zero (`10 / 3 = 3`).
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[10 / B]"),
        (U256::from(3), None)
    );
    // `7 / 3` truncates to `2`.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[7 / B]"),
        (U256::from(2), None)
    );
    // The fractional intermediate of `(1 / B) * B` is discarded at the typed
    // division, folding to `0` rather than `1`, which is then a zero length.
    assert_eq!(
        folded_array_length(&uint256_b("2"), "uint256[(1 / B) * B]"),
        (U256::ZERO, Some(ArrayLengthZero.into())),
    );
    // Whole literals combine with a typed integer fine: `2 * 7 / 2 = 7`.
    assert_eq!(
        folded_array_length(&uint256_b("2"), "uint256[(B * 7) / 2]"),
        (U256::from(7), None)
    );
    // Exponentiation with a typed base: `3 ** 2 = 9`.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[B ** 2]"),
        (U256::from(9), None)
    );

    // A small integer type widens the result to the literal's mobile type, so
    // `300 / B` (`B: uint8`) is `uint16` and folds to `100` instead of
    // overflowing `uint8`.
    assert_eq!(
        folded_array_length("uint8 constant B = 3;", "uint256[300 / B]"),
        (U256::from(100), None),
    );

    // But when no widening applies, an overflow of the result type is rejected:
    // `A + 255` (`A: uint8`) has common type `uint8` (255 fits `uint8`), and
    // `1 + 255 = 256` does not fit `uint8`, so it is an arithmetic overflow.
    assert_eq!(
        folded_array_length("uint8 constant A = 1;", "uint256[A + 255]"),
        (U256::ZERO, Some(ConstantArithmeticError.into())),
    );
}

#[test]
fn test_array_length_folds_all_literal_arithmetic() {
    // No typed constant: exact rational arithmetic, then the whole result feeds
    // the length. `100 / 8 * 2 = 25` (`100 / 8` is the exact `25/2`).
    assert_eq!(
        folded_array_length("", "uint256[(100 / 8) * 2]"),
        (U256::from(25), None)
    );
    assert_eq!(
        folded_array_length("", "uint256[2 ** 8]"),
        (U256::from(256), None)
    );
    // Lengths above the machine word are valid. The maximum is `2**256 - 1`.
    assert_eq!(
        folded_array_length("", "uint256[2 ** 64]"),
        (U256::from(1) << 64, None)
    );
}

#[test]
fn test_array_length_rejected_inputs_default_to_zero() {
    let uint256_b = |value: &str| format!("uint256 constant B = {value};");

    // A negative literal has no common type with an unsigned integer, so the
    // operator has no result type and is reported as incompatible.
    assert_eq!(
        folded_array_length(&uint256_b("2"), "uint256[(0 - 7) / B]"),
        (
            U256::ZERO,
            Some(
                IncompatibleConstantOperator {
                    operator: "/".to_owned(),
                    left_type: "int_const -7".to_owned(),
                    right_type: "uint256".to_owned(),
                }
                .into()
            ),
        ),
    );
    // `~B` over `uint256` folds to a negative value that overflows the
    // unsigned result type: an arithmetic error, not a plain non-constant
    // length.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[~B]"),
        (U256::ZERO, Some(ConstantArithmeticError.into())),
    );
    // Unary negation of an unsigned integer has no result type.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[-B]"),
        (U256::ZERO, Some(ArrayLengthNotConstant.into())),
    );
    // A literal exceeding 256 bits cannot meet a typed integer, so the
    // operator has no result type and is reported as incompatible.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[(2 ** 256) / B]"),
        (
            U256::ZERO,
            Some(
                IncompatibleConstantOperator {
                    operator: "/".to_owned(),
                    left_type: "int_const 1157...(70 digits omitted)...9936".to_owned(),
                    right_type: "uint256".to_owned(),
                }
                .into()
            ),
        ),
    );
    // All-literal division stays an exact rational, which is not a valid
    // (integer) length.
    assert_eq!(
        folded_array_length("", "uint256[10 / 4]"),
        (U256::ZERO, Some(ArrayLengthFractional.into())),
    );
}

#[test]
fn test_storage_base_slot_evaluation() {
    // A base slot referencing a constant declared *after* the contract resolves,
    // because base slots are evaluated in `p5_resolve_references` once every
    // constant is typed.
    assert_eq!(
        contract_base_slot("contract C layout at N {} uint256 constant N = 42;", "C"),
        (Some(U256::from(42)), None),
    );
    // Backward reference and a plain literal still resolve.
    assert_eq!(
        contract_base_slot("uint256 constant N = 42; contract C layout at N {}", "C"),
        (Some(U256::from(42)), None),
    );
    assert_eq!(
        contract_base_slot("contract C layout at 7 {}", "C"),
        (Some(U256::from(7)), None),
    );
    // A non-integer constant (here `address`) is not foldable to an integer, so
    // it is rejected as a non-constant base slot, even when forward-referenced.
    assert_eq!(
        contract_base_slot(
            "contract C layout at N {} address constant N = address(0);",
            "C",
        ),
        (None, Some(StorageLayoutBaseNotConstant.into())),
    );
}
