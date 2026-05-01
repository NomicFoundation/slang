use std::rc::Rc;

use num_bigint::BigInt;
use num_rational::BigRational;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::binder::Typing;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{literals, Number};

use super::super::{
    DecimalNumberExpressionStruct, Expression, FunctionCallExpressionStruct,
    HexNumberExpressionStruct, StringExpression,
};

impl Expression {
    /// Returns the literal number carried by this expression's type, if any.
    /// Set for plain number literals and for constant-folded expressions whose
    /// typing pass narrowed them to an integer or rational literal kind.
    pub fn number_value(&self) -> Option<Number> {
        let (node_id, semantic) = self.node_typing_information()?;
        number_value_of_node(semantic, node_id)
    }

    fn node_typing_information(&self) -> Option<(NodeId, &Rc<SemanticContext>)> {
        match self {
            Expression::AssignmentExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::ConditionalExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::OrExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::AndExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::EqualityExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::InequalityExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::BitwiseOrExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::BitwiseXorExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::BitwiseAndExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::ShiftExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::AdditiveExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::MultiplicativeExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::ExponentiationExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::PostfixExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::PrefixExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::FunctionCallExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::CallOptionsExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::MemberAccessExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::IndexAccessExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::NewExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::TupleExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::TypeExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::ArrayExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::HexNumberExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::DecimalNumberExpression(e) => Some((e.ir_node.id(), &e.semantic)),
            Expression::Identifier(e) => Some((e.ir_node.id(), &e.semantic)),
            // We don't have semantic context for these variants
            Expression::PayableKeyword
            | Expression::ThisKeyword
            | Expression::SuperKeyword
            | Expression::TrueKeyword
            | Expression::FalseKeyword
            | Expression::ElementaryType(_)
            | Expression::StringExpression(_) => None,
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

    /// Returns the literal number carried by this node's type, preserving the
    /// integer-vs-rational distinction.
    pub fn number_value(&self) -> Option<Number> {
        number_value_of_node(&self.semantic, self.ir_node.id())
    }
}

impl HexNumberExpressionStruct {
    /// Returns the integer value of this literal, or `None` if the literal
    /// cannot be evaluated (e.g. a malformed hex digit sequence).
    pub fn integer_value(&self) -> Option<BigInt> {
        integer_value_of_node(&self.semantic, self.ir_node.id())
    }

    /// Returns the literal number carried by this node's type. Hex number
    /// expressions only ever fold to integers.
    pub fn number_value(&self) -> Option<Number> {
        number_value_of_node(&self.semantic, self.ir_node.id())
    }
}

/// Returns the literal number carried by a node's type, when the type is a
/// number-shaped literal kind (integer, hex-integer, or rational).
fn number_value_of_node(semantic: &SemanticContext, node_id: NodeId) -> Option<Number> {
    let type_id = semantic.binder().node_typing(node_id).as_type_id()?;
    semantic.types().number_value_of_type_id(type_id)
}

fn integer_value_of_node(semantic: &SemanticContext, node_id: NodeId) -> Option<BigInt> {
    match number_value_of_node(semantic, node_id)? {
        Number::Integer(value) => Some(value),
        Number::Rational(_) => None,
    }
}

fn rational_value_of_node(semantic: &SemanticContext, node_id: NodeId) -> Option<BigRational> {
    match number_value_of_node(semantic, node_id)? {
        Number::Integer(value) => Some(BigRational::from(value)),
        Number::Rational(value) => Some(value),
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
