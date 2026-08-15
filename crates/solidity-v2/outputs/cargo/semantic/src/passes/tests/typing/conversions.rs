//! Implicit conversion and the common type two operands reconcile to:
//! conditional expressions, array literals, and the widening rules between
//! integers, byte arrays and literals.

use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use ruint::aliases::U256;

use super::{
    register_uint_type, try_type_of_expression, try_type_of_expression_in_context,
    type_of_expression, type_of_expression_in_context,
};
use crate::types::{
    ByteArrayType, DataLocation, FixedSizeArrayType, IntegerType, LiteralKind, MappingType,
    StringType, TupleType, Type,
};

#[test]
fn test_implicit_conversion_uses_literal_value() {
    let (_, mut types) = type_of_expression("0");

    let int8 = types.register_type(Type::Integer(IntegerType {
        is_signed: true,
        bits: 8,
    }));
    let uint8 = types.uint8();
    let uint256 = types.uint256();

    let lit_127 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(127),
    }));
    let lit_128 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(128),
    }));
    let lit_neg_128 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(-128),
    }));
    let lit_neg_129 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(-129),
    }));
    let lit_neg_1 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(-1),
    }));
    let lit_0 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(0),
    }));
    let lit_255 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(255),
    }));
    let lit_256 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(256),
    }));
    let lit_big = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(12_345_678),
    }));
    let lit_half = types.register_type(Type::Literal(LiteralKind::Rational {
        value: BigRational::new(BigInt::from(1), BigInt::from(2)),
    }));

    // Within int8 range (signed):
    assert!(types.implicitly_convertible_to(lit_127, int8));
    assert!(types.implicitly_convertible_to(lit_neg_128, int8));

    // Outside int8 range:
    assert!(!types.implicitly_convertible_to(lit_128, int8));
    assert!(!types.implicitly_convertible_to(lit_neg_129, int8));

    // Within uint8 range (unsigned):
    assert!(types.implicitly_convertible_to(lit_0, uint8));
    assert!(types.implicitly_convertible_to(lit_255, uint8));

    // Outside uint8 range:
    assert!(!types.implicitly_convertible_to(lit_256, uint8));
    assert!(!types.implicitly_convertible_to(lit_neg_1, uint8));

    // Anything within uint256 (any non-negative literal we'd use here):
    assert!(types.implicitly_convertible_to(lit_0, uint256));
    assert!(types.implicitly_convertible_to(lit_big, uint256));

    // Non-integer rationals never convert to integers.
    assert!(!types.implicitly_convertible_to(lit_half, uint8));
    assert!(!types.implicitly_convertible_to(lit_half, int8));
    assert!(!types.implicitly_convertible_to(lit_half, uint256));
}

#[test]
fn test_hex_literal_to_byte_array_conversion() {
    let (_, mut types) = type_of_expression("0");

    let bytes1 = types.register_type(Type::ByteArray(ByteArrayType { width: 1 }));
    let bytes2 = types.register_type(Type::ByteArray(ByteArrayType { width: 2 }));
    let bytes4 = types.register_type(Type::ByteArray(ByteArrayType { width: 4 }));

    // Hex source literal: byte-width must match the target exactly.
    let hex_0x12 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigUint::from(0x12u32),
        bytes: 1,
    }));
    let hex_0x0012 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigUint::from(0x12u32),
        bytes: 2,
    }));
    let hex_0x10203040 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigUint::from(0x1020_3040u32),
        bytes: 4,
    }));

    // Same value, different source widths convert to different byte arrays.
    assert!(types.implicitly_convertible_to(hex_0x12, bytes1));
    assert!(!types.implicitly_convertible_to(hex_0x12, bytes2));
    assert!(types.implicitly_convertible_to(hex_0x0012, bytes2));
    assert!(!types.implicitly_convertible_to(hex_0x0012, bytes1));
    assert!(types.implicitly_convertible_to(hex_0x10203040, bytes4));
    assert!(!types.implicitly_convertible_to(hex_0x10203040, bytes2));

    // Decimal-source integer of the same value does NOT convert to a byte
    // array (provenance matters).
    let dec_18 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(18),
    }));
    assert!(!types.implicitly_convertible_to(dec_18, bytes1));
    assert!(!types.implicitly_convertible_to(dec_18, bytes2));

    // Zero in any source — decimal, hex of any width, or folded — converts
    // to a byte array of any width.
    let dec_0 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(0),
    }));
    let hex_0x0 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigUint::from(0u32),
        bytes: 1,
    }));
    let hex_0x0000 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigUint::from(0u32),
        bytes: 2,
    }));
    assert!(types.implicitly_convertible_to(dec_0, bytes1));
    assert!(types.implicitly_convertible_to(dec_0, bytes4));
    assert!(types.implicitly_convertible_to(hex_0x0, bytes2));
    assert!(types.implicitly_convertible_to(hex_0x0000, bytes4));
}

#[test]
fn test_conditional_expression_unifies_branch_types() {
    // Both branches reify to uint8 — common type is uint8.
    let (type_, _) = type_of_expression("true ? 1 : 2");
    assert_eq!(
        type_,
        Type::Integer(IntegerType {
            is_signed: false,
            bits: 8,
        })
    );

    // uint8 (1) widens to uint16 (256).
    let (type_, _) = type_of_expression("true ? 1 : 256");
    assert_eq!(
        type_,
        Type::Integer(IntegerType {
            is_signed: false,
            bits: 16,
        })
    );

    // int8 (-1) and int8 (1) — common type is int8.
    let (type_, _) = type_of_expression("true ? -1 : -128");
    assert_eq!(
        type_,
        Type::Integer(IntegerType {
            is_signed: true,
            bits: 8,
        })
    );

    // Both branches are string literals — both reify to `string memory`.
    let (type_, _) = type_of_expression(r#"true ? "abc" : "x""#);
    assert_eq!(
        type_,
        Type::String(StringType {
            location: DataLocation::Memory,
        })
    );
}

#[test]
fn test_conditional_expression_unresolved_when_branches_incompatible() {
    // uint8 (1) and int8 (-1): neither converts to the other at the same
    // bit width, so unification fails and the conditional is unresolved.
    let (type_, _) = try_type_of_expression("true ? 1 : -1");
    assert_eq!(type_, None);

    // A non-reducing rational has no `reified` type yet, so any conditional
    // involving one is unresolved.
    let (type_, _) = try_type_of_expression("true ? 0.5 : 1");
    assert_eq!(type_, None);
}

#[test]
fn test_array_literal_unifies_element_types() {
    // Homogeneous uint8 elements.
    let (expr_type, types) = type_of_expression("[1, 2, 3]");
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type,
        size,
        location,
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(3));
    assert_eq!(location, DataLocation::Memory);
    assert_eq!(element_type, types.uint8());

    // Mixed widths widen to the largest required.
    let (expr_type, mut types) = type_of_expression("[1, 256, 3]");
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type, size, ..
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(3));
    assert_eq!(element_type, register_uint_type(&mut types, 16));

    // Negative values force the result to a signed type.
    let (expr_type, mut types) = type_of_expression("[-1, -2]");
    let Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) = expr_type else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(
        element_type,
        types.register_type(Type::Integer(IntegerType {
            is_signed: true,
            bits: 8,
        }))
    );

    // String literal arrays reify each element to `string memory`.
    let (expr_type, types) = type_of_expression(r#"["abc", "x"]"#);
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type, size, ..
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(2));
    assert_eq!(element_type, types.string_memory());
}

#[test]
fn test_array_literal_unresolved_when_elements_incompatible() {
    // uint8 (1) and int8 (-1) cannot be unified (same bit width, opposite sign).
    let (type_, _) = try_type_of_expression("[1, -1]");
    assert_eq!(type_, None);

    // Non-reducing rationals don't reify yet — array unification fails.
    let (type_, _) = try_type_of_expression("[0.5, 1]");
    assert_eq!(type_, None);
}

#[test]
fn test_conditional_expression_unifies_byte_arrays() {
    let (expr_type, types) = type_of_expression("true ? bytes32(0) : bytes32(1)");
    assert_eq!(expr_type, *types.get_type_by_id(types.bytes32()));
}

#[test]
fn test_conditional_expression_widens_byte_arrays() {
    let (expr_type, types) = type_of_expression("true ? bytes20(0) : bytes32(0)");
    assert_eq!(expr_type, *types.get_type_by_id(types.bytes32()));

    let (expr_type, types) = type_of_expression("true ? bytes32(0) : bytes20(0)");
    assert_eq!(expr_type, *types.get_type_by_id(types.bytes32()));
}

#[test]
fn test_array_literal_unifies_byte_array_elements() {
    let (expr_type, types) = type_of_expression("[bytes32(0), bytes32(1)]");
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type,
        size,
        location,
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(2));
    assert_eq!(location, DataLocation::Memory);
    assert_eq!(element_type, types.bytes32());
}

#[test]
fn test_array_literal_unifies_byte_array_and_literal_zero() {
    let (expr_type, types) = type_of_expression("[bytes32(0), 0]");
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type,
        size,
        location,
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(2));
    assert_eq!(location, DataLocation::Memory);
    assert_eq!(element_type, types.bytes32());
}

#[test]
fn test_conditional_expression_does_not_unify_byte_array_and_literal_zero() {
    let (type_, _) = try_type_of_expression("true ? bytes32(0) : 0");
    assert_eq!(type_, None);
}

#[test]
fn test_array_literal_does_not_unify_when_literal_is_first_and_byte_array_follows() {
    // The first element of the array is used to find the common type
    // Matches solc behaviour
    let (type_, _) = try_type_of_expression("[0, bytes32(0)]");
    assert_eq!(type_, None);
}

#[test]
fn test_array_literal_widens_past_first_element_integer_type() {
    let (expr_type, mut types) = type_of_expression("[uint8(0), 256]");
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type, size, ..
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(2));
    assert_eq!(element_type, register_uint_type(&mut types, 16));
}

#[test]
fn test_array_literal_unifies_byte_array_and_matching_hex_literal() {
    let (expr_type, types) = type_of_expression("[bytes1(0x01), 0x01]");
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type, size, ..
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(2));
    assert_eq!(element_type, types.bytes1());
}

#[test]
fn test_conditional_expression_loses_hex_literal_specialness() {
    let (type_, _) = try_type_of_expression("true ? bytes1(0x01) : 0x01");
    assert_eq!(type_, None);
}

#[test]
fn test_conditional_expression_widens_literal_to_concrete_integer() {
    let (expr_type, types) = type_of_expression("true ? uint256(0) : 0");
    assert_eq!(expr_type, *types.get_type_by_id(types.uint256()));

    let (expr_type, types) = type_of_expression("true ? 0 : uint256(0)");
    assert_eq!(expr_type, *types.get_type_by_id(types.uint256()));
}

#[test]
fn test_conditional_expression_unifies_mappings() {
    let (expr_type, types) = try_type_of_expression_in_context(
        "mapping(uint => uint) m1; mapping(uint => uint) m2;",
        "true ? m1 : m2",
    );
    let Some(Type::Mapping(MappingType {
        key_type_id,
        value_type_id,
    })) = expr_type
    else {
        panic!("expected Mapping, got {expr_type:?}");
    };
    assert_eq!(key_type_id, types.uint256());
    assert_eq!(value_type_id, types.uint256());
}

#[test]
fn test_conditional_expression_unifies_literal_tuples() {
    let (expr_type, types) = type_of_expression("true ? (1, 2) : (3, 4)");
    let Type::Tuple(TupleType { types: tuple_types }) = expr_type else {
        panic!("expected Tuple, got {expr_type:?}");
    };

    assert_eq!(tuple_types.len(), 2);
    assert_eq!(tuple_types[0], types.uint8());
    assert_eq!(tuple_types[1], types.uint8());
}

#[test]
fn test_conditional_expression_with_function_call_tuple() {
    // A branch that calls a function returning `(uint256, uint256)` gives the
    // conditional a tuple type. The other branch's elements are unified
    // element-wise against it, widening literal elements (`3`) against the
    // concrete `uint256` coming from `pair()`, matching solc's common tuple
    // type. Every combination below resolves to `(uint256, uint256)`.
    let ctx = "function pair() internal pure returns (uint256, uint256) { return (1, 2); }";

    for expr in [
        "true ? pair() : (uint256(3), uint256(4))",
        "true ? pair() : (3, uint256(4))",
        "true ? pair() : (3, 4)",
        "true ? (uint256(1), uint256(2)) : (3, uint256(4))",
        // Symmetric: the concrete branch on the right also unifies.
        "true ? (3, 4) : pair()",
    ] {
        let (expr_type, types) = type_of_expression_in_context(ctx, expr);
        let Type::Tuple(TupleType { types: tuple_types }) = expr_type else {
            panic!("expected Tuple for `{expr}`, got {expr_type:?}");
        };
        assert_eq!(tuple_types.len(), 2, "arity for `{expr}`");
        assert_eq!(tuple_types[0], types.uint256(), "element 0 for `{expr}`");
        assert_eq!(tuple_types[1], types.uint256(), "element 1 for `{expr}`");
    }

    // No common type: each tuple is "wider" in a different position (element 0
    // on the left, element 1 on the right), so neither converts to the other
    // (matches solc error 1080).
    let expr = "true ? (uint256(1), uint128(2)) : (uint128(3), uint256(4))";
    let (expr_type, _) = try_type_of_expression(expr);
    assert!(expr_type.is_none());
}

#[test]
fn test_mappings_only_unify_on_equal_elements() {
    // Mappings must match on key and value types
    let (expr_type, _) = try_type_of_expression_in_context(
        "mapping(uint => int128) m1; mapping(uint => int256) m2;",
        "true ? m1 : m2",
    );
    assert_eq!(None, expr_type);
}

#[test]
fn test_array_literal_rejects_mapping_element() {
    let (type_, _) = try_type_of_expression_in_context(
        "mapping(uint => uint) m1; mapping(uint => uint) m2;",
        "[m1, m2]",
    );
    assert_eq!(type_, None);
}

#[test]
fn test_array_literal_does_not_unify_byte_array_and_non_zero_literal() {
    let (type_, _) = try_type_of_expression("[bytes32(0), 1]");
    assert_eq!(type_, None);
}

#[test]
fn test_bitwise_or_widens_byte_arrays() {
    let (expr_type, types) = type_of_expression("bytes20(0) | bytes32(0)");
    assert_eq!(expr_type, *types.get_type_by_id(types.bytes32()));

    let (expr_type, types) = type_of_expression("bytes32(0) | bytes20(0)");
    assert_eq!(expr_type, *types.get_type_by_id(types.bytes32()));
}

#[test]
fn test_conditional_expression_unifies_booleans() {
    let (type_, _) = type_of_expression("true ? true : false");
    assert_eq!(type_, Type::Boolean);
}
