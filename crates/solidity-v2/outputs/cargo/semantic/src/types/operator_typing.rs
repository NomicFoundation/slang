use num_traits::Signed;

use super::literals::numbers::within_integer_range;
use super::{IntegerType, LiteralKind, Number, Type, TypeId, TypeRegistry};

impl TypeRegistry {
    /// Result type of an arithmetic or bitwise binary operator, dispatched on
    /// the left operand. `op` combines the two values when both are literals.
    pub(crate) fn type_of_binary_operator(
        &mut self,
        left: TypeId,
        right: TypeId,
        op: fn(&Number, &Number) -> Option<Number>,
    ) -> Option<TypeId> {
        match self.get_type_by_id(left).clone() {
            Type::Integer(integer) => self.integer_type_of_binary_operator(&integer, right),
            Type::Literal(
                literal @ (LiteralKind::Integer { .. } | LiteralKind::Rational { .. }),
            ) => self.literal_type_of_binary_operator(&literal, right, op),
            _ => None,
        }
    }

    /// Result type of a shift or exponentiation operator, dispatched on the left
    /// operand. The result follows the left operand, and the right operand must
    /// be a nonnegative unsigned integer amount or exponent.
    pub(crate) fn type_of_left_typed_binary_operator(
        &mut self,
        left: TypeId,
        right: TypeId,
        op: fn(&Number, &Number) -> Option<Number>,
    ) -> Option<TypeId> {
        match self.get_type_by_id(left).clone() {
            Type::Integer(integer) => {
                self.integer_type_of_left_typed_binary_operator(&integer, right)
            }
            Type::Literal(
                literal @ (LiteralKind::Integer { .. } | LiteralKind::Rational { .. }),
            ) => self.literal_type_of_left_typed_binary_operator(&literal, right, op),
            _ => None,
        }
    }

    /// Result type of unary minus. Signed integers and literals keep their own
    /// type, and unary minus on an unsigned integer has no result type.
    pub(crate) fn type_of_negation(&mut self, operand: TypeId) -> Option<TypeId> {
        match self.get_type_by_id(operand).clone() {
            // Only signed integers can be negated.
            Type::Integer(IntegerType { is_signed, .. }) => is_signed.then_some(operand),
            // A literal negates to its own type.
            Type::Literal(kind @ (LiteralKind::Integer { .. } | LiteralKind::Rational { .. })) => {
                let negated = Number::from_literal_kind(&kind)?.negate();
                Some(self.register_type(Type::Literal(negated.to_literal_kind())))
            }
            _ => None,
        }
    }

    /// Result type of bitwise not. Integers keep their type, and a literal folds.
    /// A fractional rational has no result, since bitwise not is integer only.
    pub(crate) fn type_of_bitwise_not(&mut self, operand: TypeId) -> Option<TypeId> {
        match self.get_type_by_id(operand).clone() {
            Type::Integer(_) => Some(operand),
            Type::Literal(kind @ (LiteralKind::Integer { .. } | LiteralKind::Rational { .. })) => {
                let complement = Number::from_literal_kind(&kind)?.bit_not()?;
                Some(self.register_type(Type::Literal(complement.to_literal_kind())))
            }
            _ => None,
        }
    }

    /// Arithmetic or bitwise operator with an integer left operand. The result
    /// is the common type of the two operands.
    fn integer_type_of_binary_operator(
        &mut self,
        left: &IntegerType,
        right: TypeId,
    ) -> Option<TypeId> {
        // Only integers and numeric literals are valid operands.
        if !matches!(
            self.get_type_by_id(right),
            Type::Integer(_)
                | Type::Literal(LiteralKind::Integer { .. } | LiteralKind::Rational { .. })
        ) {
            return None;
        }
        let left_id = self.register_type(Type::Integer(left.clone()));
        self.common_type(left_id, right)
    }

    /// Shift or exponentiation with an integer left operand. The right operand
    /// must be a valid amount or exponent, and the result is the left operand's
    /// type.
    fn integer_type_of_left_typed_binary_operator(
        &mut self,
        left: &IntegerType,
        right: TypeId,
    ) -> Option<TypeId> {
        if self.is_valid_shift_or_exponent_amount(right) {
            Some(self.register_type(Type::Integer(left.clone())))
        } else {
            None
        }
    }

    /// Arithmetic or bitwise operator with a number literal left operand.
    fn literal_type_of_binary_operator(
        &mut self,
        left: &LiteralKind,
        right: TypeId,
        op: fn(&Number, &Number) -> Option<Number>,
    ) -> Option<TypeId> {
        let value = Number::from_literal_kind(left)?;
        match self.get_type_by_id(right).clone() {
            // Literal meets a concrete integer. The literal must be whole and fit
            // some integer type. Then we run the operator again with the common
            // type as the left operand.
            Type::Integer(_) => {
                if !within_integer_range(value.as_integer()?) {
                    return None;
                }
                let left_id = self.register_type(Type::Literal(left.clone()));
                let common = self.common_type(left_id, right)?;
                self.type_of_binary_operator(common, right, op)
            }
            // Both operands are literals, so fold the value.
            Type::Literal(kind @ (LiteralKind::Integer { .. } | LiteralKind::Rational { .. })) => {
                let right_value = Number::from_literal_kind(&kind)?;
                let result = op(&value, &right_value)?;
                Some(self.register_type(Type::Literal(result.to_literal_kind())))
            }
            _ => None,
        }
    }

    /// Shift or exponentiation with a number literal left operand.
    fn literal_type_of_left_typed_binary_operator(
        &mut self,
        left: &LiteralKind,
        right: TypeId,
        op: fn(&Number, &Number) -> Option<Number>,
    ) -> Option<TypeId> {
        let value = Number::from_literal_kind(left)?;
        match self.get_type_by_id(right).clone() {
            // Literal base with a concrete integer amount or exponent. The base
            // must be whole and fit some integer type. The result is int256 or
            // uint256 by the base's sign.
            Type::Integer(_) => {
                if !within_integer_range(value.as_integer()?) {
                    return None;
                }
                if !self.is_valid_shift_or_exponent_amount(right) {
                    return None;
                }
                let is_signed = value.is_negative();
                Some(self.register_type(Type::Integer(IntegerType {
                    is_signed,
                    bits: 256,
                })))
            }
            // Both operands are literals, so fold the value.
            Type::Literal(kind @ (LiteralKind::Integer { .. } | LiteralKind::Rational { .. })) => {
                let right_value = Number::from_literal_kind(&kind)?;
                let result = op(&value, &right_value)?;
                Some(self.register_type(Type::Literal(result.to_literal_kind())))
            }
            _ => None,
        }
    }

    /// Whether `amount` is a valid shift amount or exponent. It must be an
    /// unsigned integer, or a nonnegative whole literal that fits some integer
    /// type.
    /// Only the amount (the right operand) is checked here. Callers invoke this
    /// once the base (the left operand) is already known to be integer-valued,
    /// which is a concrete integer type, or a whole integer literal that fits
    /// one.
    fn is_valid_shift_or_exponent_amount(&self, amount: TypeId) -> bool {
        match self.get_type_by_id(amount) {
            Type::Integer(IntegerType { is_signed, .. }) => !is_signed,
            Type::Literal(LiteralKind::Integer { value }) => {
                !value.is_negative() && within_integer_range(value)
            }
            _ => false,
        }
    }
}
