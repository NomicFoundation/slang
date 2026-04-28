use std::str::FromStr;

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::cast::ToPrimitive;
use num_traits::Num;
use slang_solidity_v2_ir::ir;

use super::LiteralKind;

/// Returns the integer value of a decimal number literal, or `None` for
/// rationals that do not reduce to an integer after unit multiplication.
// TODO: support returning rational values (numerator/denominator) once the
// evaluator exposes them.
pub fn integer_value_of_decimal_number_expression(
    decimal_number_expression: &ir::DecimalNumberExpression,
) -> Option<BigInt> {
    ConstantValue::from_decimal_number_expression(decimal_number_expression)
        .map(|ConstantValue::Integer(integer)| integer)
}

/// Returns the integer value of a hex number literal, or `None` if the
/// literal cannot be evaluated (e.g. a malformed hex digit sequence).
// TODO: support returning rational values once the evaluator exposes them.
pub fn integer_value_of_hex_number_expression(
    hex_number_expression: &ir::HexNumberExpression,
) -> Option<BigInt> {
    ConstantValue::from_hex_number_expression(hex_number_expression)
        .map(|ConstantValue::Integer(integer)| integer)
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ConstantValue {
    Integer(BigInt),
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
        } else {
            // TODO: support returning a rational value (numerator/denominator)
            // when the literal does not reduce to an integer.
            scaled_numerator
                .is_multiple_of(&denominator)
                .then(|| Self::Integer(scaled_numerator / denominator))
        }
    }

    pub(crate) fn as_usize(&self) -> Option<usize> {
        let Self::Integer(value) = self;
        value.to_usize()
    }

    pub(crate) fn get_literal_kind(&self) -> LiteralKind {
        let Self::Integer(value) = self;
        let bytes = u32::try_from(value.bits().div_ceil(8)).unwrap().max(1);
        LiteralKind::DecimalInteger {
            bytes,
            signed: false,
        }
    }

    pub(crate) fn get_signed_literal_kind(&self) -> LiteralKind {
        let Self::Integer(magnitude) = self;
        let negated_bits = (magnitude - 1u32).bits() + 1;
        let bytes = u32::try_from(negated_bits.div_ceil(8)).unwrap().max(1);
        LiteralKind::DecimalInteger {
            bytes,
            signed: true,
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
