use std::str::FromStr;

use num_bigint::BigInt;
use num_integer::Integer as _;
use num_rational::BigRational;
use num_traits::cast::ToPrimitive;
use num_traits::{Num, One, Signed, Zero};
use slang_solidity_v2_ir::ir;

use super::LiteralKind;

/// Returns the integer value of a decimal number literal, or `None` for
/// rationals that do not reduce to an integer after unit multiplication.
pub fn integer_value_of_decimal_number_expression(
    decimal_number_expression: &ir::DecimalNumberExpression,
) -> Option<BigInt> {
    ConstantValue::from_decimal_number_expression(decimal_number_expression)?.into_integer()
}

/// Returns the integer value of a hex number literal, or `None` if the
/// literal cannot be parsed.
pub fn integer_value_of_hex_number_expression(
    hex_number_expression: &ir::HexNumberExpression,
) -> Option<BigInt> {
    ConstantValue::from_hex_number_expression(hex_number_expression)?.into_integer()
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ConstantValue {
    Integer(BigInt),
    Rational(BigRational),
}

impl ConstantValue {
    pub(crate) fn from_hex_number_expression(
        hex_number_expression: &ir::HexNumberExpression,
    ) -> Option<Self> {
        let hex = hex_number_expression.literal.unparse();
        // Skip `0x` prefix and parse the hexadecimal number.
        // `BigInt::from_str_radix` can handle `_` separators.
        let value = BigInt::from_str_radix(&hex[2..], 16).ok()?;
        Some(Self::Integer(value))
    }

    pub(crate) fn from_decimal_number_expression(
        decimal_number_expression: &ir::DecimalNumberExpression,
    ) -> Option<Self> {
        let mut decimal = decimal_number_expression.literal.unparse().to_owned();
        decimal.retain(|c| c != '_');
        let (mantissa, exponent) = match decimal.split_once(['e', 'E']) {
            Some((m, e)) => (m, e.parse::<i64>().ok()?),
            None => (decimal.as_str(), 0),
        };
        let (int_part, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let numerator = BigInt::from_str(&format!("{int_part}{fraction}")).ok()?;
        let unit = decimal_number_expression
            .unit
            .as_ref()
            .copied()
            .map_or_else(|| BigInt::from(1u32), number_unit_multiplier);
        let scaled_numerator = numerator * unit;
        let decimal_shift = exponent - i64::try_from(fraction.len()).ok()?;
        let denominator =
            BigInt::from(10u32).pow(u32::try_from(decimal_shift.unsigned_abs()).ok()?);
        if decimal_shift >= 0 {
            Some(Self::Integer(scaled_numerator * denominator))
        } else if scaled_numerator.is_multiple_of(&denominator) {
            Some(Self::Integer(scaled_numerator / denominator))
        } else {
            Some(Self::from_rational(BigRational::new(
                scaled_numerator,
                denominator,
            )))
        }
    }

    pub(crate) fn from_literal_kind(kind: &LiteralKind) -> Option<Self> {
        match kind {
            LiteralKind::Integer(value) | LiteralKind::HexInteger { value, .. } => {
                Some(Self::Integer(value.clone()))
            }
            LiteralKind::Rational(value) => Some(Self::Rational(value.clone())),
            LiteralKind::HexString { .. } | LiteralKind::String { .. } | LiteralKind::Address => {
                None
            }
        }
    }

    pub(crate) fn into_integer(self) -> Option<BigInt> {
        match self {
            Self::Integer(value) => Some(value),
            Self::Rational(_) => None,
        }
    }

    pub(crate) fn as_integer(&self) -> Option<&BigInt> {
        match self {
            Self::Integer(value) => Some(value),
            Self::Rational(_) => None,
        }
    }

    pub(crate) fn as_usize(&self) -> Option<usize> {
        self.as_integer()?.to_usize()
    }

    pub(crate) fn to_literal_kind(&self) -> LiteralKind {
        match self {
            Self::Integer(value) => LiteralKind::Integer(value.clone()),
            Self::Rational(value) => LiteralKind::Rational(value.clone()),
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        match self {
            Self::Integer(value) => value.is_zero(),
            Self::Rational(value) => value.is_zero(),
        }
    }

    fn to_rational(&self) -> BigRational {
        match self {
            Self::Integer(value) => BigRational::from(value.clone()),
            Self::Rational(value) => value.clone(),
        }
    }

    /// Constructs from a `BigRational`, normalising to `Integer` when the
    /// denominator reduces to one.
    fn from_rational(value: BigRational) -> Self {
        if value.is_integer() {
            Self::Integer(value.to_integer())
        } else {
            Self::Rational(value)
        }
    }

    pub(crate) fn negate(&self) -> Self {
        match self {
            Self::Integer(value) => Self::Integer(-value),
            Self::Rational(value) => Self::Rational(-value.clone()),
        }
    }

    pub(crate) fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs + rhs),
            _ => Self::from_rational(self.to_rational() + other.to_rational()),
        }
    }

    pub(crate) fn sub(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs - rhs),
            _ => Self::from_rational(self.to_rational() - other.to_rational()),
        }
    }

    pub(crate) fn mul(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs * rhs),
            _ => Self::from_rational(self.to_rational() * other.to_rational()),
        }
    }

    /// Solidity literal arithmetic uses rational division even between two
    /// integer literals — `5 / 2` yields the rational `5/2`, not the integer
    /// `2`. Only division by zero returns `None`.
    pub(crate) fn div(&self, other: &Self) -> Option<Self> {
        if other.is_zero() {
            // TODO(validation): division by zero
            return None;
        }
        Some(Self::from_rational(
            self.to_rational() / other.to_rational(),
        ))
    }

    pub(crate) fn rem(&self, other: &Self) -> Option<Self> {
        if other.is_zero() {
            // TODO(validation): division by zero
            return None;
        }
        match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Some(Self::Integer(lhs % rhs)),
            // TODO(validation): Modulo on rationals is not supported.
            _ => None,
        }
    }

    pub(crate) fn pow(&self, exponent: &Self) -> Option<Self> {
        let exponent = exponent.as_integer()?;
        let exp_abs = exponent.abs().to_u32()?;
        let raised = match self {
            Self::Integer(base) => Self::Integer(base.pow(exp_abs)),
            Self::Rational(base) => {
                let exp_i32 = i32::try_from(exp_abs).ok()?;
                Self::from_rational(base.pow(exp_i32))
            }
        };
        if exponent.is_negative() {
            if raised.is_zero() {
                // TODO(validation): division by zero
                return None;
            }
            Some(Self::from_rational(
                BigRational::one() / raised.to_rational(),
            ))
        } else {
            Some(raised)
        }
    }

    pub(crate) fn bit_or(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Some(Self::Integer(lhs | rhs)),
            _ => None,
        }
    }

    pub(crate) fn bit_xor(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Some(Self::Integer(lhs ^ rhs)),
            _ => None,
        }
    }

    pub(crate) fn bit_and(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Some(Self::Integer(lhs & rhs)),
            _ => None,
        }
    }

    pub(crate) fn shl(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Some(Self::Integer(lhs << rhs.to_u32()?)),
            _ => None,
        }
    }

    pub(crate) fn shr(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Some(Self::Integer(lhs >> rhs.to_u32()?)),
            _ => None,
        }
    }
}

fn number_unit_multiplier(unit: ir::NumberUnit) -> BigInt {
    match unit {
        ir::NumberUnit::WeiKeyword | ir::NumberUnit::SecondsKeyword => BigInt::from(1u32),
        ir::NumberUnit::GweiKeyword => BigInt::from(10u64.pow(9)),
        ir::NumberUnit::EtherKeyword => BigInt::from(10u64.pow(18)),
        ir::NumberUnit::MinutesKeyword => BigInt::from(60u32),
        ir::NumberUnit::HoursKeyword => BigInt::from(60u32 * 60),
        ir::NumberUnit::DaysKeyword => BigInt::from(60u32 * 60 * 24),
        ir::NumberUnit::WeeksKeyword => BigInt::from(60u32 * 60 * 24 * 7),
    }
}
