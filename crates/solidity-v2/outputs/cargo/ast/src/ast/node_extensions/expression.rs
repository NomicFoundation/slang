use num_bigint::BigInt;
use slang_solidity_v2_semantic::types::Number;

use super::super::{ElementaryType, Expression, Type};

impl Expression {
    /// Returns the type assigned to this expression by the typing pass, or
    /// `None` when no type was recorded for the underlying node.
    pub fn get_type(&self) -> Option<Type> {
        match self {
            Expression::AssignmentExpression(expression) => expression.get_type(),
            Expression::ConditionalExpression(expression) => expression.get_type(),
            Expression::OrExpression(expression) => expression.get_type(),
            Expression::AndExpression(expression) => expression.get_type(),
            Expression::EqualityExpression(expression) => expression.get_type(),
            Expression::InequalityExpression(expression) => expression.get_type(),
            Expression::BitwiseOrExpression(expression) => expression.get_type(),
            Expression::BitwiseXorExpression(expression) => expression.get_type(),
            Expression::BitwiseAndExpression(expression) => expression.get_type(),
            Expression::ShiftExpression(expression) => expression.get_type(),
            Expression::AdditiveExpression(expression) => expression.get_type(),
            Expression::MultiplicativeExpression(expression) => expression.get_type(),
            Expression::ExponentiationExpression(expression) => expression.get_type(),
            Expression::PostfixExpression(expression) => expression.get_type(),
            Expression::PrefixExpression(expression) => expression.get_type(),
            Expression::FunctionCallExpression(expression) => expression.get_type(),
            Expression::CallOptionsExpression(expression) => expression.get_type(),
            Expression::MemberAccessExpression(expression) => expression.get_type(),
            Expression::IndexAccessExpression(expression) => expression.get_type(),
            Expression::NewExpression(expression) => expression.get_type(),
            Expression::TupleExpression(expression) => expression.get_type(),
            Expression::TypeExpression(expression) => expression.get_type(),
            Expression::ArrayExpression(expression) => expression.get_type(),
            Expression::HexNumberExpression(expression) => expression.get_type(),
            Expression::DecimalNumberExpression(expression) => expression.get_type(),
            Expression::StringExpression(expression) => expression.get_type(),
            Expression::ElementaryType(expression) => expression.get_type(),
            Expression::PayableKeyword(expression) => expression.get_type(),
            Expression::ThisKeyword(expression) => expression.get_type(),
            Expression::SuperKeyword(expression) => expression.get_type(),
            Expression::TrueKeyword(expression) => expression.get_type(),
            Expression::FalseKeyword(expression) => expression.get_type(),
            Expression::Identifier(expression) => expression.get_type(),
        }
    }

    /// The integer value this expression folds to when it is a compile-time
    /// constant of integer type, or `None` otherwise.
    pub fn integer_value(&self) -> Option<BigInt> {
        let Type::Literal(literal_type) = self.get_type()? else {
            return None;
        };
        Number::from_literal_kind(&literal_type.kind())?.into_integer()
    }
}

impl ElementaryType {
    /// Returns the type assigned to this elementary type by the typing pass,
    /// dispatched to the underlying keyword/terminal node.
    pub fn get_type(&self) -> Option<Type> {
        match self {
            ElementaryType::BoolKeyword(keyword) => keyword.get_type(),
            ElementaryType::StringKeyword(keyword) => keyword.get_type(),
            ElementaryType::AddressType(address_type) => address_type.get_type(),
            ElementaryType::BytesKeyword(keyword) => keyword.get_type(),
            ElementaryType::IntKeyword(keyword) => keyword.get_type(),
            ElementaryType::UintKeyword(keyword) => keyword.get_type(),
            ElementaryType::FixedKeyword(keyword) => keyword.get_type(),
            ElementaryType::UfixedKeyword(keyword) => keyword.get_type(),
        }
    }
}
