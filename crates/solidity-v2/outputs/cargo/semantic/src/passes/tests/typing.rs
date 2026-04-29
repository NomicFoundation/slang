use num_bigint::BigInt;
use num_rational::BigRational;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeIdGenerator};

use super::build_file;
use crate::binder::{Binder, Typing};
use crate::context::SemanticFile;
use crate::passes::{
    p1_collect_definitions, p2_linearise_contracts, p3_type_definitions, p4_resolve_references,
};
use crate::types::{LiteralKind, Type, TypeRegistry};

/// Parses a Solidity expression as the value of a top-level `uint constant`,
/// runs all semantic passes, and returns a clone of the inferred type assigned
/// to that value expression along with the populated `TypeRegistry`.
fn type_of_value_expression(input: &str) -> (Type, TypeRegistry) {
    let source = format!("uint constant x = {input};");
    let mut id_generator = NodeIdGenerator::default();
    let file = build_file("test.sol", &source, &mut id_generator);
    let files = [file];

    let mut binder = Binder::default();
    let mut types = TypeRegistry::default();
    let language_version = LanguageVersion::V0_8_30;

    p1_collect_definitions::run(&files, &mut binder);
    p2_linearise_contracts::run(&files, &mut binder);
    p3_type_definitions::run(&files, &mut binder, &mut types);
    p4_resolve_references::run(&files, &mut binder, &mut types, language_version);

    let value_expr = match files[0].ir_root().members.first().unwrap() {
        ir::SourceUnitMember::ConstantDefinition(definition) => definition.value.as_ref().unwrap(),
        other => panic!("expected ConstantDefinition, got {other:?}"),
    };

    let type_id = match binder.node_typing(expression_node_id(value_expr)) {
        Typing::Resolved(type_id) => type_id,
        other => panic!("expected resolved type for value expression, got {other:?}"),
    };

    (types.get_type_by_id(type_id).clone(), types)
}

/// Returns the `NodeId` of an `ir::Expression`, dispatching across the
/// variants that can appear as the value of a top-level constant in our
/// typing tests.
fn expression_node_id(expr: &ir::Expression) -> NodeId {
    match expr {
        ir::Expression::AdditiveExpression(e) => e.id(),
        ir::Expression::MultiplicativeExpression(e) => e.id(),
        ir::Expression::ShiftExpression(e) => e.id(),
        ir::Expression::ExponentiationExpression(e) => e.id(),
        ir::Expression::PrefixExpression(e) => e.id(),
        ir::Expression::DecimalNumberExpression(e) => e.id(),
        ir::Expression::HexNumberExpression(e) => e.id(),
        ir::Expression::TupleExpression(e) => e.id(),
        ir::Expression::ConditionalExpression(e) => e.id(),
        ir::Expression::BitwiseOrExpression(e) => e.id(),
        ir::Expression::BitwiseXorExpression(e) => e.id(),
        ir::Expression::BitwiseAndExpression(e) => e.id(),
        other => panic!("expression variant {other:?} not supported by typing tests"),
    }
}

#[test]
fn test_value_bearing_integer_literal_types() {
    let (type_, _) = type_of_value_expression("127");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer(BigInt::from(127)))
    );

    let (type_, _) = type_of_value_expression("-128");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer(BigInt::from(-128)))
    );

    // Hex literals carry source byte width as `HexInteger`, distinct from
    // decimal `Integer` so the byte-array conversion rule can fire.
    let (type_, _) = type_of_value_expression("0xff");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::HexInteger {
            value: BigInt::from(255),
            bytes: 1,
        })
    );

    // Source byte width is preserved across leading zeros.
    let (type_, _) = type_of_value_expression("0x0012");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::HexInteger {
            value: BigInt::from(18),
            bytes: 2,
        })
    );

    // Folding a hex literal demotes it to a plain `Integer` (provenance lost).
    let (type_, _) = type_of_value_expression("0x10 + 0");
    assert_eq!(type_, Type::Literal(LiteralKind::Integer(BigInt::from(16))));
}

#[test]
fn test_binary_arithmetic_folds_to_narrowed_literal() {
    // Addition.
    let (type_, _) = type_of_value_expression("1 + 1");
    assert_eq!(type_, Type::Literal(LiteralKind::Integer(BigInt::from(2))));

    // Multiplication.
    let (type_, _) = type_of_value_expression("3 * 4");
    assert_eq!(type_, Type::Literal(LiteralKind::Integer(BigInt::from(12))));

    // Power.
    let (type_, _) = type_of_value_expression("2 ** 10");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer(BigInt::from(1024)))
    );

    // Shift.
    let (type_, _) = type_of_value_expression("1 << 32");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer(BigInt::from(1u64 << 32)))
    );

    // Reducible rational arithmetic normalises back to an integer.
    let (type_, _) = type_of_value_expression("1.5 * 2");
    assert_eq!(type_, Type::Literal(LiteralKind::Integer(BigInt::from(3))));

    // Non-reducing rational division stays rational.
    let (type_, _) = type_of_value_expression("5 / 2");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Rational(BigRational::new(
            BigInt::from(5),
            BigInt::from(2)
        )))
    );

    // Negation of a folded constant.
    let (type_, _) = type_of_value_expression("-(1 + 1)");
    assert_eq!(type_, Type::Literal(LiteralKind::Integer(BigInt::from(-2))));
}

#[test]
fn test_implicit_conversion_uses_literal_value() {
    let (_, mut types) = type_of_value_expression("0");

    let int8 = types.register_type(Type::Integer {
        signed: true,
        bits: 8,
    });
    let uint8 = types.register_type(Type::Integer {
        signed: false,
        bits: 8,
    });
    let uint256 = types.register_type(Type::Integer {
        signed: false,
        bits: 256,
    });

    let lit_127 = types.register_type(Type::Literal(LiteralKind::Integer(BigInt::from(127))));
    let lit_128 = types.register_type(Type::Literal(LiteralKind::Integer(BigInt::from(128))));
    let lit_neg_128 = types.register_type(Type::Literal(LiteralKind::Integer(BigInt::from(-128))));
    let lit_neg_129 = types.register_type(Type::Literal(LiteralKind::Integer(BigInt::from(-129))));
    let lit_neg_1 = types.register_type(Type::Literal(LiteralKind::Integer(BigInt::from(-1))));
    let lit_0 = types.register_type(Type::Literal(LiteralKind::Integer(BigInt::from(0))));
    let lit_255 = types.register_type(Type::Literal(LiteralKind::Integer(BigInt::from(255))));
    let lit_256 = types.register_type(Type::Literal(LiteralKind::Integer(BigInt::from(256))));
    let lit_big = types.register_type(Type::Literal(LiteralKind::Integer(BigInt::from(
        12_345_678,
    ))));
    let lit_half = types.register_type(Type::Literal(LiteralKind::Rational(BigRational::new(
        BigInt::from(1),
        BigInt::from(2),
    ))));

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
    let (_, mut types) = type_of_value_expression("0");

    let bytes1 = types.register_type(Type::ByteArray { width: 1 });
    let bytes2 = types.register_type(Type::ByteArray { width: 2 });
    let bytes4 = types.register_type(Type::ByteArray { width: 4 });

    // Hex source literal: byte-width must match the target exactly.
    let hex_0x12 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigInt::from(0x12),
        bytes: 1,
    }));
    let hex_0x0012 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigInt::from(0x12),
        bytes: 2,
    }));
    let hex_0x10203040 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigInt::from(0x1020_3040u32),
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
    let dec_18 = types.register_type(Type::Literal(LiteralKind::Integer(BigInt::from(18))));
    assert!(!types.implicitly_convertible_to(dec_18, bytes1));
    assert!(!types.implicitly_convertible_to(dec_18, bytes2));

    // Zero in any source — decimal, hex of any width, or folded — converts
    // to a byte array of any width.
    let dec_0 = types.register_type(Type::Literal(LiteralKind::Integer(BigInt::from(0))));
    let hex_0x0 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigInt::from(0),
        bytes: 1,
    }));
    let hex_0x0000 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigInt::from(0),
        bytes: 2,
    }));
    assert!(types.implicitly_convertible_to(dec_0, bytes1));
    assert!(types.implicitly_convertible_to(dec_0, bytes4));
    assert!(types.implicitly_convertible_to(hex_0x0, bytes2));
    assert!(types.implicitly_convertible_to(hex_0x0000, bytes4));
}
