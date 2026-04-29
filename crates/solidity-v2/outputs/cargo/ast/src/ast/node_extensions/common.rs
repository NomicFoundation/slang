use num_bigint::BigInt;
use num_rational::BigRational;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{LiteralKind, Type};

/// Returns the integer value carried by a number-literal node's type, when
/// the type is an integer-shaped literal. Rational literals return `None`.
pub(crate) fn integer_value_of_node(semantic: &SemanticContext, node_id: NodeId) -> Option<BigInt> {
    let type_id = semantic.binder().node_typing(node_id).as_type_id()?;
    match semantic.types().get_type_by_id(type_id) {
        Type::Literal(LiteralKind::Integer(value) | LiteralKind::HexInteger { value, .. }) => {
            Some(value.clone())
        }
        _ => None,
    }
}

/// Returns the rational value carried by a number-literal node's type. Integer
/// literals are returned as a rational with denominator one.
pub(crate) fn rational_value_of_node(
    semantic: &SemanticContext,
    node_id: NodeId,
) -> Option<BigRational> {
    let type_id = semantic.binder().node_typing(node_id).as_type_id()?;
    match semantic.types().get_type_by_id(type_id) {
        Type::Literal(LiteralKind::Integer(value) | LiteralKind::HexInteger { value, .. }) => {
            Some(BigRational::from(value.clone()))
        }
        Type::Literal(LiteralKind::Rational(value)) => Some(value.clone()),
        _ => None,
    }
}
