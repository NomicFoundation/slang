use std::fmt;
use std::str::FromStr;

use num_bigint::BigInt;
use num_integer::Integer as _;
use num_rational::BigRational;
use num_traits::cast::ToPrimitive;
use num_traits::{Num, One, Signed, Zero};
use slang_solidity_v2_ir::ir;

use crate::types::{FixedPointNumberType, IntegerType, LiteralKind, Type};

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Integer(BigInt),
    Rational(BigRational),
}

impl Number {
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
        // Split off the exponent in-place, leaving `decimal` as the mantissa.
        let exponent = match decimal.find(['e', 'E']) {
            Some(idx) => {
                let exp = decimal[idx + 1..].parse::<i64>().ok()?;
                decimal.truncate(idx);
                exp
            }
            None => 0,
        };
        // Record the fraction width and then strip the `.` so the buffer is
        // pure digits ready to be parsed as the numerator.
        let fraction_len = decimal.find('.').map_or(0, |idx| decimal.len() - idx - 1);
        decimal.retain(|c| c != '.');
        let numerator = BigInt::from_str(&decimal).ok()?;
        let unit = decimal_number_expression
            .unit
            .as_ref()
            .map_or_else(|| BigInt::from(1u32), number_unit_multiplier);
        let scaled_numerator = numerator * unit;
        let decimal_shift = exponent - i64::try_from(fraction_len).ok()?;
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

    pub fn from_literal_kind(kind: &LiteralKind) -> Option<Self> {
        match kind {
            LiteralKind::Address { value } => Some(Self::Integer(value.into())),
            LiteralKind::Integer { value } => Some(Self::Integer(value.clone())),
            LiteralKind::HexInteger { value, .. } => Some(Self::Integer(value.clone().into())),
            LiteralKind::Rational { value } => Some(Self::Rational(value.clone())),
            LiteralKind::HexString { .. } | LiteralKind::String { .. } => None,
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

    pub(crate) fn to_literal_kind(&self) -> LiteralKind {
        match self {
            Self::Integer(value) => LiteralKind::Integer {
                value: value.clone(),
            },
            Self::Rational(value) => LiteralKind::Rational {
                value: value.clone(),
            },
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Self::Integer(value) => value.is_zero(),
            Self::Rational(value) => value.is_zero(),
        }
    }

    pub fn is_negative(&self) -> bool {
        match self {
            Self::Integer(value) => value.is_negative(),
            Self::Rational(value) => value.is_negative(),
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
            // TODO(validation) SDR[1737]: division by zero
            return None;
        }
        Some(Self::from_rational(
            self.to_rational() / other.to_rational(),
        ))
    }

    pub(crate) fn rem(&self, other: &Self) -> Option<Self> {
        if other.is_zero() {
            // TODO(validation) SDR[1737]: division by zero
            return None;
        }
        Some(Self::from_rational(
            self.to_rational() % other.to_rational(),
        ))
    }

    pub(crate) fn pow(&self, exponent: &Self) -> Option<Self> {
        // TODO(validation) SDR[1738]: validate range of exponent
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
                // TODO(validation) SDR[1737]: division by zero
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

    pub(crate) fn bit_not(&self) -> Option<Self> {
        match self {
            Self::Integer(value) => Some(Self::Integer(!value)),
            Self::Rational(_) => None,
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

/// Human-readable form used in diagnostics: `int_const N` for an integer and
/// `rational_const N / D` for a rational. A value whose decimal form exceeds
/// 32 digits is abbreviated to its first and last four digits.
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn readable_digits(value: &BigInt) -> String {
            let digits = value.to_string();
            if digits.len() > 32 {
                let omitted = digits.len() - 8;
                format!(
                    "{}...({omitted} digits omitted)...{}",
                    &digits[..4],
                    &digits[digits.len() - 4..],
                )
            } else {
                digits
            }
        }

        match self {
            Self::Integer(value) => write!(f, "int_const {}", readable_digits(value)),
            Self::Rational(value) => write!(
                f,
                "rational_const {} / {}",
                readable_digits(value.numer()),
                readable_digits(value.denom()),
            ),
        }
    }
}

fn number_unit_multiplier(unit: &ir::NumberUnit) -> BigInt {
    match unit {
        ir::NumberUnit::WeiKeyword(_) | ir::NumberUnit::SecondsKeyword(_) => BigInt::from(1u32),
        ir::NumberUnit::GweiKeyword(_) => BigInt::from(10u64.pow(9)),
        ir::NumberUnit::EtherKeyword(_) => BigInt::from(10u64.pow(18)),
        ir::NumberUnit::MinutesKeyword(_) => BigInt::from(60u32),
        ir::NumberUnit::HoursKeyword(_) => BigInt::from(60u32 * 60),
        ir::NumberUnit::DaysKeyword(_) => BigInt::from(60u32 * 60 * 24),
        ir::NumberUnit::WeeksKeyword(_) => BigInt::from(60u32 * 60 * 24 * 7),
    }
}

/// Number of bits required to hold `value` as a Solidity integer of the given
/// signedness:
/// - unsigned: exactly `value.bits()` (with at least 1, so zero still has a
///   positive width).
/// - signed positive: `value.bits() + 1` (one extra bit for the sign).
/// - signed negative: `(-value - 1).bits() + 1` (two's-complement width).
fn integer_bits_required(value: &BigInt, signed: bool) -> u32 {
    if !signed {
        debug_assert!(!value.is_negative());
        u32::try_from(value.bits()).unwrap().max(1)
    } else if value.is_negative() {
        let magnitude_minus_one = -value - 1u32;
        u32::try_from(magnitude_minus_one.bits()).unwrap() + 1
    } else {
        u32::try_from(value.bits()).unwrap() + 1
    }
}

/// Returns true if `value` fits in the integer type described by `signed` and
/// `bits`. Range is `[-2^(bits-1), 2^(bits-1) - 1]` for signed and
/// `[0, 2^bits - 1]` for unsigned.
pub(crate) fn integer_literal_fits(value: &BigInt, signed: bool, bits: u32) -> bool {
    if !signed && value.is_negative() {
        return false;
    }
    integer_bits_required(value, signed) <= bits
}

/// Whether `value` is within the range a Solidity integer type can hold. Those
/// are at most 256 bits wide, so a larger value is out of range.
pub(crate) fn within_integer_range(value: &BigInt) -> bool {
    integer_bits_required(value, value.is_negative()) <= 256
}

/// Returns true if the rational `value` lies within the range of the integer
/// type described by `signed` and `bits`.
pub(crate) fn rational_literal_fits(value: &BigRational, signed: bool, bits: u32) -> bool {
    integer_literal_fits(&value.floor().to_integer(), signed, bits)
        && integer_literal_fits(&value.ceil().to_integer(), signed, bits)
}

pub(crate) fn smallest_integer_type_to_fit(value: &BigInt) -> Option<Type> {
    let is_signed = value.is_negative();
    let bits = integer_bits_required(value, is_signed);

    if bits > 256 {
        // TODO(validation) SDR[1740]: the integers don't fit in the EVM
        return None;
    }
    let bits = bits.next_multiple_of(8).max(8);
    Some(Type::Integer(IntegerType { is_signed, bits }))
}

/// Returns the smallest fixed-point type that holds `value`.
///
/// For rationals with a finite decimal expansion (`q = 2^a * 5^b` in
/// lowest terms) the natural precision `max(a, b)` gives an exact
/// representation; that's what we pick when it doesn't exceed 80 and the
/// scaled value still fits in 256 bits — so `1/2` becomes `ufixed8x1`,
/// `1/4` becomes `ufixed8x2`, `6/5` becomes `ufixed8x1`, etc.
///
/// Otherwise — periodic fractional part, natural precision over 80, or
/// natural precision yielding a scaled value over 256 bits — we truncate
/// at the largest precision `N` in `[0, 80]` that keeps
/// `floor(|p| * 10^N / q)` within the chosen 256-bit range (`2^256 - 1`
/// unsigned, `2^255` signed). `M` is then the smallest multiple of 8 in
/// `[8, 256]` that holds the scaled value. Returns `None` only when the
/// integer part alone overflows the range, since for any rational a
/// low-enough `N` (down to 0) would otherwise let it fit.
pub(crate) fn smallest_fixed_point_type_to_fit(value: &BigRational) -> Option<Type> {
    const MAX_DECIMAL_PLACES: u32 = 80;

    let is_signed = value.is_negative();
    let numerator_magnitude = value.numer().abs();
    // `BigRational` normalises to a positive denominator, so `denom()` is
    // always > 0 and carries no sign information.
    let denominator = value.denom();

    // Maximum scaled magnitude that fits in 256 bits with the chosen
    // signedness: `2^255` for signed (reachable by `INT256_MIN`), and
    // `2^256 - 1` for unsigned.
    let max_magnitude = if is_signed {
        BigInt::from(1u32) << 255
    } else {
        (BigInt::from(1u32) << 256) - BigInt::from(1u32)
    };

    if &numerator_magnitude / denominator > max_magnitude {
        return None;
    }

    // Factor the denominator as `2^a * 5^b * r`. The natural precision of
    // an exact representation is `max(a, b)`; if `r > 1` the rational has
    // a periodic fractional part and no exact representation exists.
    let five = BigInt::from(5u32);
    let mut remaining_denominator = denominator.clone();
    let factors_of_two: u32 = remaining_denominator
        .trailing_zeros()
        .unwrap_or(0)
        .try_into()
        .unwrap();
    remaining_denominator >>= factors_of_two;
    let mut factors_of_five: u32 = 0;
    while remaining_denominator.is_multiple_of(&five) {
        remaining_denominator /= &five;
        factors_of_five += 1;
    }
    let has_finite_expansion = remaining_denominator.is_one();
    let natural_decimal_places = factors_of_two.max(factors_of_five);

    // The scaled magnitude `floor(|p| * 10^d / q)` fits the 256-bit range
    // iff `|p| * 10^d < (max_magnitude + 1) * q`; both the fast path and
    // the binary-search probe below use this `threshold` comparison.
    let threshold = (max_magnitude + BigInt::from(1u32)) * denominator;

    let build_result = |scaled_magnitude: BigInt, decimal_places: u32| -> Type {
        let scaled = if is_signed {
            -scaled_magnitude
        } else {
            scaled_magnitude
        };
        let bits = integer_bits_required(&scaled, is_signed);
        debug_assert!(bits <= 256, "scaled value must fit 256 bits at this point");
        let bits = bits.next_multiple_of(8).max(8);
        Type::FixedPointNumber(FixedPointNumberType {
            is_signed,
            bits,
            decimal_places,
        })
    };

    // Fast path: a finite expansion whose natural precision is within the
    // 80-place cap and whose scaled magnitude already fits — the common
    // shape for typical decimal literals (`1/2`, `1/4`, `1.2`, …). One
    // `pow` + multiply + compare here saves the binary search's ~7 probes.
    if has_finite_expansion && natural_decimal_places <= MAX_DECIMAL_PLACES {
        let scaled_numerator =
            &numerator_magnitude * BigInt::from(10u32).pow(natural_decimal_places);
        if scaled_numerator < threshold {
            return Some(build_result(
                scaled_numerator / denominator,
                natural_decimal_places,
            ));
        }
    }

    // Slow path: truncate at the largest precision in `[0, MAX_DECIMAL_PLACES]`
    // whose scaled magnitude still fits. Reached when the rational is
    // periodic, when its natural precision exceeds 80 places, or when its
    // natural precision yields a scaled value over 256 bits.
    let mut lower_bound: u32 = 0;
    let mut upper_bound: u32 = MAX_DECIMAL_PLACES;
    while lower_bound < upper_bound {
        let midpoint = lower_bound + (upper_bound - lower_bound).div_ceil(2);
        if &numerator_magnitude * BigInt::from(10u32).pow(midpoint) < threshold {
            lower_bound = midpoint;
        } else {
            upper_bound = midpoint - 1;
        }
    }
    let decimal_places = lower_bound;

    let scaled_magnitude =
        &numerator_magnitude * BigInt::from(10u32).pow(decimal_places) / denominator;
    Some(build_result(scaled_magnitude, decimal_places))
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use num_rational::BigRational;

    use super::{Number, Type, rational_literal_fits, smallest_fixed_point_type_to_fit};
    use crate::types::FixedPointNumberType;

    fn integer(value: i64) -> Number {
        Number::Integer(BigInt::from(value))
    }

    fn rational(numerator: i64, denominator: i64) -> Number {
        Number::Rational(BigRational::new(
            BigInt::from(numerator),
            BigInt::from(denominator),
        ))
    }

    #[test]
    fn rem_on_rationals() {
        assert_eq!(integer(5).rem(&integer(3)), Some(integer(2)));
        // Integer modulo follows the dividend's sign.
        assert_eq!(integer(-7).rem(&integer(3)), Some(integer(-1)));
        // 5.5 % 2 = 1.5
        assert_eq!(rational(11, 2).rem(&integer(2)), Some(rational(3, 2)));
        // 7 % 2.5 = 2 (integer-valued, collapses to Integer)
        assert_eq!(integer(7).rem(&rational(5, 2)), Some(integer(2)));
        // 8.5 % 1.5 = 1
        assert_eq!(rational(17, 2).rem(&rational(3, 2)), Some(integer(1)));
        // Sign follows the dividend, not the divisor.
        assert_eq!(rational(-11, 2).rem(&integer(2)), Some(rational(-3, 2)));
        assert_eq!(rational(11, 2).rem(&integer(-2)), Some(rational(3, 2)));
        // (1/3) % (1/10) = 1/30
        assert_eq!(rational(1, 3).rem(&rational(1, 10)), Some(rational(1, 30)));
        // Division by zero is rejected for both integer and rational divisors.
        assert_eq!(integer(5).rem(&integer(0)), None);
        assert_eq!(rational(11, 2).rem(&integer(0)), None);
    }

    #[test]
    fn rational_literal_fits_rejects_out_of_range_before_truncation() {
        // 255.9 truncates toward zero to the in-range 255, but the rational
        // exceeds uint8's max, so it must be rejected — the range check happens
        // before truncation.
        let over_max = BigRational::new(BigInt::from(2559), BigInt::from(10));
        assert!(!rational_literal_fits(&over_max, false, 8));

        // An in-range fraction is accepted (it truncates toward zero afterwards).
        let in_range = BigRational::new(BigInt::from(1), BigInt::from(2));
        assert!(rational_literal_fits(&in_range, false, 8));

        // Below uint8's min (negative): rejected even though its ceil is 0.
        let below_zero = BigRational::new(BigInt::from(-1), BigInt::from(2));
        assert!(!rational_literal_fits(&below_zero, false, 8));
        // But -0.5 is within int8's range, so signed accepts it.
        assert!(rational_literal_fits(&below_zero, true, 8));

        // Signed lower bound: -128.5 truncates toward zero to the in-range -128,
        // but the rational is below int8's min, so it is rejected.
        let under_min = BigRational::new(BigInt::from(-1285), BigInt::from(10));
        assert!(!rational_literal_fits(&under_min, true, 8));
    }

    #[test]
    fn display_abbreviates_long_integers() {
        assert_eq!(integer(-7).to_string(), "int_const -7");
        // 10^40 has 41 digits, over the 32-digit limit.
        assert_eq!(
            Number::Integer(pow10(40)).to_string(),
            "int_const 1000...(33 digits omitted)...0000"
        );
    }

    #[test]
    fn display_renders_rationals_as_fractions() {
        assert_eq!(rational(5, 2).to_string(), "rational_const 5 / 2");
        // The sign lives on the numerator.
        assert_eq!(rational(-1, 3).to_string(), "rational_const -1 / 3");
        // The fraction is normalised.
        assert_eq!(rational(2, 6).to_string(), "rational_const 1 / 3");
        // A huge numerator is abbreviated like an integer.
        assert_eq!(
            Number::Rational(BigRational::new(pow10(40) + 1, BigInt::from(2))).to_string(),
            "rational_const 1000...(33 digits omitted)...0001 / 2"
        );
    }

    fn fit(numerator: BigInt, denominator: BigInt) -> Option<FixedPointNumberType> {
        match smallest_fixed_point_type_to_fit(&BigRational::new(numerator, denominator))? {
            Type::FixedPointNumber(inner) => Some(inner),
            other => panic!("expected FixedPointNumber, got {other:?}"),
        }
    }

    fn unsigned_fixed(bits: u32, decimal_places: u32) -> FixedPointNumberType {
        FixedPointNumberType {
            is_signed: false,
            bits,
            decimal_places,
        }
    }

    fn signed_fixed(bits: u32, decimal_places: u32) -> FixedPointNumberType {
        FixedPointNumberType {
            is_signed: true,
            bits,
            decimal_places,
        }
    }

    fn pow2(exponent: u32) -> BigInt {
        BigInt::from(2u32).pow(exponent)
    }

    fn pow10(exponent: u32) -> BigInt {
        BigInt::from(10u32).pow(exponent)
    }

    // 1/2 = 0.5: terminating, denom = 2^1, natural d = 1. Scaled = 5
    // (3 bits, rounded to M = 8).
    #[test]
    fn terminating_one_half() {
        assert_eq!(
            fit(BigInt::from(1u32), BigInt::from(2u32)),
            Some(unsigned_fixed(8, 1)),
        );
    }

    // 1/4 = 0.25: terminating, denom = 2^2, natural d = 2. Scaled = 25
    // (5 bits → M = 8).
    #[test]
    fn terminating_one_quarter() {
        assert_eq!(
            fit(BigInt::from(1u32), BigInt::from(4u32)),
            Some(unsigned_fixed(8, 2)),
        );
    }

    // 6/5 = 1.2: terminating, denom = 5^1, natural d = 1. Scaled = 12
    // (4 bits → M = 8).
    #[test]
    fn terminating_one_point_two() {
        assert_eq!(
            fit(BigInt::from(6u32), BigInt::from(5u32)),
            Some(unsigned_fixed(8, 1)),
        );
    }

    // -1/2: terminating negative — natural d = 1, scaled = -5
    // (signed bits required = 4 → M = 8).
    #[test]
    fn terminating_signed_negative_one_half() {
        assert_eq!(
            fit(-BigInt::from(1u32), BigInt::from(2u32)),
            Some(signed_fixed(8, 1)),
        );
    }

    // 1/3: periodic — algorithm falls back to truncation, maximising d
    // within the unsigned 256-bit range. d = 77 since `10^77 / 3` fits
    // but `10^78 / 3` would not.
    #[test]
    fn non_terminating_one_third_truncates() {
        assert_eq!(
            fit(BigInt::from(1u32), BigInt::from(3u32)),
            Some(unsigned_fixed(256, 77)),
        );
    }

    // -1/3: signed variant of the truncation case. The signed bound
    // `2^255` still admits d = 77 (`10^77 / 3 ≈ 3.33 * 10^76 < 2^255`).
    #[test]
    fn non_terminating_signed_negative_one_third_truncates() {
        assert_eq!(
            fit(-BigInt::from(1u32), BigInt::from(3u32)),
            Some(signed_fixed(256, 77)),
        );
    }

    // (2^256 - 1) / 2: terminating with natural d = 1, but the scaled
    // value `5 * (2^256 - 1)` overflows 256 bits. Falls back to d = 0
    // (which truncates the .5 fractional part).
    #[test]
    fn terminating_natural_precision_overflows_falls_back_to_truncation() {
        let numerator = pow2(256) - BigInt::from(1u32);
        assert_eq!(
            fit(numerator, BigInt::from(2u32)),
            Some(unsigned_fixed(256, 0)),
        );
    }

    // 5 / 10^80: terminating, reduces to `1 / (2^80 * 5^79)`, natural
    // d = 80. Scaled = 5, fits in the smallest M = 8.
    #[test]
    fn terminating_natural_precision_at_maximum() {
        assert_eq!(
            fit(BigInt::from(5u32), pow10(80)),
            Some(unsigned_fixed(8, 80)),
        );
    }

    // 1 / 10^100: natural d = 100 exceeds the 80-place cap, so we
    // truncate. At d = 80 the scaled magnitude floors to 0.
    #[test]
    fn terminating_natural_precision_over_cap_truncates_to_zero() {
        assert_eq!(
            fit(BigInt::from(1u32), pow10(100)),
            Some(unsigned_fixed(8, 80)),
        );
    }

    // 2^260 / 3: integer part alone exceeds `2^256 - 1`, so no unsigned
    // fixed-point type can hold this value.
    #[test]
    fn unsigned_integer_part_overflows() {
        assert_eq!(fit(pow2(260), BigInt::from(3u32)), None);
    }

    // -2^257 / 3: signed bound is `2^255`, but `|2^257 / 3| ≈ 7.7 * 10^76`
    // overflows it. The same magnitude as a positive value would fit
    // unsigned, but signedness is fixed by the negative sign.
    #[test]
    fn signed_integer_part_overflows() {
        assert_eq!(fit(-pow2(257), BigInt::from(3u32)), None);
    }
}
