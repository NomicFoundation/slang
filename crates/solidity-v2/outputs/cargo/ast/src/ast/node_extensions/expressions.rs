use num_bigint::BigInt;
use num_rational::BigRational;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::binder::Typing;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{literals, LiteralKind, Type};

use super::super::{
    DecimalNumberExpressionStruct, FunctionCallExpressionStruct, HexNumberExpressionStruct,
    StringExpression,
};

impl StringExpression {
    /// Returns the concatenated decoded string value as bytes.
    pub fn value(&self) -> Vec<u8> {
        match self {
            StringExpression::StringLiterals(literals) => {
                literals::value_of_string_literals(literals)
            }
            StringExpression::HexStringLiterals(literals) => {
                literals::value_of_hex_string_literals(literals)
            }
            StringExpression::UnicodeStringLiterals(literals) => {
                literals::value_of_unicode_string_literals(literals)
            }
        }
    }
}

impl DecimalNumberExpressionStruct {
    /// Returns the integer value of this literal, or `None` for rationals
    /// that do not reduce to an integer after unit multiplication.
    pub fn integer_value(&self) -> Option<BigInt> {
        integer_value_of_node(&self.semantic, self.ir_node.id())
    }

    /// Returns the rational value of this literal, or `None` if the literal
    /// cannot be evaluated. Integer literals lift to a rational with
    /// denominator one.
    pub fn rational_value(&self) -> Option<BigRational> {
        rational_value_of_node(&self.semantic, self.ir_node.id())
    }
}

impl HexNumberExpressionStruct {
    /// Returns the integer value of this literal, or `None` if the literal
    /// cannot be evaluated (e.g. a malformed hex digit sequence).
    pub fn integer_value(&self) -> Option<BigInt> {
        integer_value_of_node(&self.semantic, self.ir_node.id())
    }
}

/// Returns the integer value carried by a number-literal node's type, when
/// the type is an integer-shaped literal. Rational literals return `None`.
fn integer_value_of_node(semantic: &SemanticContext, node_id: NodeId) -> Option<BigInt> {
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
fn rational_value_of_node(semantic: &SemanticContext, node_id: NodeId) -> Option<BigRational> {
    let type_id = semantic.binder().node_typing(node_id).as_type_id()?;
    match semantic.types().get_type_by_id(type_id) {
        Type::Literal(LiteralKind::Integer(value) | LiteralKind::HexInteger { value, .. }) => {
            Some(BigRational::from(value.clone()))
        }
        Type::Literal(LiteralKind::Rational(value)) => Some(value.clone()),
        _ => None,
    }
}

impl FunctionCallExpressionStruct {
    /// Returns `true` if this call is a type conversion (e.g. `uint256(x)`,
    /// `address(y)`) rather than a function call.
    pub fn is_type_conversion(&self) -> bool {
        match &self.ir_node.operand {
            ir::Expression::ElementaryType(_) | ir::Expression::PayableKeyword => true,
            ir::Expression::Identifier(terminal) => matches!(
                self.semantic.binder().node_typing(terminal.id()),
                Typing::MetaType(_) | Typing::UserMetaType(_)
            ),
            ir::Expression::MemberAccessExpression(mae) => matches!(
                self.semantic.binder().node_typing(mae.id()),
                Typing::MetaType(_) | Typing::UserMetaType(_)
            ),
            _ => false,
        }
    }
}
