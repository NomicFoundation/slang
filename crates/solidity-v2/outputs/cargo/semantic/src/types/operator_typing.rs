use num_traits::Signed;

use super::literals::numbers::within_integer_range;
use super::{IntegerType, LiteralKind, Number, Type, TypeId, TypeRegistry};

/// A binary operator on numbers that can be evaluated at compile time.
#[derive(Clone, Copy, Debug)]
pub(crate) enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitOr,
    BitXor,
    BitAnd,
    Shl,
    Shr,
    Pow,
}

impl BinaryOperator {
    /// Applies the operator to two literal numbers. Returns `None` when the
    /// operation is not defined for the values (eg. division by zero, or a
    /// bitwise operator on a fractional rational).
    pub(crate) fn fold(self, lhs: &Number, rhs: &Number) -> Option<Number> {
        match self {
            Self::Add => Some(lhs.add(rhs)),
            Self::Sub => Some(lhs.sub(rhs)),
            Self::Mul => Some(lhs.mul(rhs)),
            Self::Div => lhs.div(rhs),
            Self::Rem => lhs.rem(rhs),
            Self::BitOr => lhs.bit_or(rhs),
            Self::BitXor => lhs.bit_xor(rhs),
            Self::BitAnd => lhs.bit_and(rhs),
            Self::Shl => lhs.shl(rhs),
            Self::Shr => lhs.shr(rhs),
            Self::Pow => lhs.pow(rhs),
        }
    }

    /// Shifts and exponentiation type by the left operand alone. The other
    /// operators take the common type of both operands.
    fn is_left_typed(self) -> bool {
        matches!(self, Self::Shl | Self::Shr | Self::Pow)
    }

    /// The operator as written in source (e.g. `/`), used in diagnostics.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Rem => "%",
            Self::BitOr => "|",
            Self::BitXor => "^",
            Self::BitAnd => "&",
            Self::Shl => "<<",
            Self::Shr => ">>",
            Self::Pow => "**",
        }
    }
}

/// A unary operator on numbers that can be evaluated at compile time.
#[derive(Clone, Copy, Debug)]
pub(crate) enum UnaryOperator {
    Negate,
    BitNot,
}

impl UnaryOperator {
    /// Applies the operator to a literal number. Returns `None` when the
    /// operation is not defined for the value (bitwise not of a fractional
    /// rational).
    pub(crate) fn fold(self, value: &Number) -> Option<Number> {
        match self {
            Self::Negate => Some(value.negate()),
            Self::BitNot => value.bit_not(),
        }
    }
}

impl TypeRegistry {
    /// The type both operands of a binary operator reconcile to.
    ///
    /// `Self::common_type` mobilises *one* side and tests the other operand's
    /// *raw* type against it (solc's `Type::commonType` rule), so a raw literal
    /// is measured against a concrete operand as-is — eg. `int8 x; x == 5`
    /// keeps `5` raw and reconciles to `int8`, not to `5`'s own mobile type
    /// `uint8`. When neither raw type fits the other's mobile type, both sides
    /// are mobilised and retried, recovering eg. two address literals whose raw
    /// literal types don't inter-convert but whose mobile `address` types do.
    pub(crate) fn common_operand_type(&mut self, left: TypeId, right: TypeId) -> Option<TypeId> {
        // Fast path: operands that already share a type reconcile to its mobile
        // type. The `common_type`/mobilise dance below would only rediscover
        // that, at the cost of extra `Type` clones and convertibility checks.
        if left == right {
            return self.compute_mobile_type(left);
        }
        if let Some(common) = self.common_type(left, right) {
            return Some(common);
        }
        let left_mobile = self.compute_mobile_type(left)?;
        let right_mobile = self.compute_mobile_type(right)?;
        if left_mobile == left && right_mobile == right {
            // Nothing mobilised, so `common_type` already failed on these.
            return None;
        }
        // Equivalent to `common_type(left_mobile, right_mobile)`, but mobile
        // types are idempotent (`mobile(mobile(t)) == mobile(t)`), so skip
        // re-mobilising both sides — just apply the convertibility rule.
        if self.implicitly_convertible_to(right_mobile, left_mobile) {
            return Some(left_mobile);
        }
        if self.implicitly_convertible_to(left_mobile, right_mobile) {
            return Some(right_mobile);
        }
        None
    }

    /// Result type of a binary operator, dispatched on the left operand. For
    /// shifts and exponentiation the result follows the left operand, and the
    /// right operand must be a nonnegative unsigned integer amount or
    /// exponent. The other operators result in the common type of the two
    /// operands. When both operands are literals, the operator is folded and
    /// the result is the literal type holding the folded value.
    pub(crate) fn type_of_binary_operator(
        &mut self,
        left: TypeId,
        right: TypeId,
        op: BinaryOperator,
    ) -> Option<TypeId> {
        match self.get_type_by_id(left).clone() {
            Type::Integer(integer) => {
                if op.is_left_typed() {
                    self.integer_type_of_left_typed_binary_operator(&integer, right)
                } else {
                    self.integer_type_of_binary_operator(&integer, right)
                }
            }
            Type::Literal(
                literal @ (LiteralKind::Integer { .. } | LiteralKind::Rational { .. }),
            ) => {
                if op.is_left_typed() {
                    self.literal_type_of_left_typed_binary_operator(&literal, right, op)
                } else {
                    self.literal_type_of_binary_operator(&literal, right, op)
                }
            }
            _ => None,
        }
    }

    /// Result type of a unary operator, dispatched on the operand. An integer
    /// operand keeps its type, except that negating an unsigned integer has no
    /// result type. When the operand is a literal, the operator is folded and
    /// the result is the literal type holding the folded value.
    pub(crate) fn type_of_unary_operator(
        &mut self,
        operand: TypeId,
        op: UnaryOperator,
    ) -> Option<TypeId> {
        match self.get_type_by_id(operand).clone() {
            Type::Integer(IntegerType { is_signed, .. }) => match op {
                // Only signed integers can be negated.
                UnaryOperator::Negate => is_signed.then_some(operand),
                UnaryOperator::BitNot => Some(operand),
            },
            Type::Literal(kind @ (LiteralKind::Integer { .. } | LiteralKind::Rational { .. })) => {
                let result = op.fold(&Number::from_literal_kind(&kind)?)?;
                Some(self.register_type(Type::Literal(result.to_literal_kind())))
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
        op: BinaryOperator,
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
                let result = op.fold(&value, &right_value)?;
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
        op: BinaryOperator,
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
                let result = op.fold(&value, &right_value)?;
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
