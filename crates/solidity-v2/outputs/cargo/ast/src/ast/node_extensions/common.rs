use num_bigint::BigInt;
use num_rational::BigRational;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{Number, Type};

/// Returns the literal number carried by a node's type, when the type is a
/// number-shaped literal kind (integer, hex-integer, or rational).
pub(crate) fn number_value_of_node(semantic: &SemanticContext, node_id: NodeId) -> Option<Number> {
    let type_id = semantic.binder().node_typing(node_id).as_type_id()?;
    match semantic.types().get_type_by_id(type_id) {
        Type::Literal(kind) => Number::from_literal_kind(kind),
        _ => None,
    }
}

pub(crate) fn integer_value_of_node(semantic: &SemanticContext, node_id: NodeId) -> Option<BigInt> {
    match number_value_of_node(semantic, node_id)? {
        Number::Integer(value) => Some(value),
        Number::Rational(_) => None,
    }
}

pub(crate) fn rational_value_of_node(
    semantic: &SemanticContext,
    node_id: NodeId,
) -> Option<BigRational> {
    match number_value_of_node(semantic, node_id)? {
        Number::Integer(value) => Some(BigRational::from(value)),
        Number::Rational(value) => Some(value),
    }
}
