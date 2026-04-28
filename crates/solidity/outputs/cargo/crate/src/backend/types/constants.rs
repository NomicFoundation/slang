use std::str::FromStr;

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::cast::ToPrimitive;
use num_traits::Num;

use super::LiteralKind;
use crate::backend::ir::ir2_flat_contracts::{self as input_ir};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ConstantValue {
    Integer(BigInt),
}

impl ConstantValue {
    pub(crate) fn from_hex_number(
        hex_number_expression: &input_ir::HexNumberExpression,
    ) -> Option<Self> {
        let hex = hex_number_expression.literal.unparse();
        let value = BigInt::from_str_radix(&hex[2..], 16).ok()?;
        let scaled = match &hex_number_expression.unit {
            None => value,
            Some(unit) => value * BigInt::from(unit),
        };
        Some(Self::Integer(scaled))
    }

    pub(crate) fn from_decimal_number(
        decimal_number_expression: &input_ir::DecimalNumberExpression,
    ) -> Option<Self> {
        let mut decimal = decimal_number_expression.literal.unparse();
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
            .map_or(BigInt::from(1u32), BigInt::from);
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

impl From<&input_ir::NumberUnit> for BigInt {
    fn from(value: &input_ir::NumberUnit) -> Self {
        match value {
            input_ir::NumberUnit::WeiKeyword | input_ir::NumberUnit::SecondsKeyword => {
                Self::from(1u32)
            }
            input_ir::NumberUnit::GweiKeyword => Self::from(10u64.pow(9)),
            input_ir::NumberUnit::SzaboKeyword => Self::from(10u64.pow(12)),
            input_ir::NumberUnit::FinneyKeyword => Self::from(10u64.pow(15)),
            input_ir::NumberUnit::EtherKeyword => Self::from(10u64.pow(18)),
            input_ir::NumberUnit::MinutesKeyword => Self::from(60u32),
            input_ir::NumberUnit::HoursKeyword => Self::from(60u32 * 60),
            input_ir::NumberUnit::DaysKeyword => Self::from(60u32 * 60 * 24),
            input_ir::NumberUnit::WeeksKeyword => Self::from(60u32 * 60 * 24 * 7),
            input_ir::NumberUnit::YearsKeyword => Self::from(60u64 * 60 * 24 * 365),
        }
    }
}
