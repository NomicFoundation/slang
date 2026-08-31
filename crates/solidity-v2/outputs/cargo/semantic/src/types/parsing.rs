use super::{ByteArrayType, BytesType, DataLocation, FixedPointNumberType, IntegerType, Type};

// Type instantiation from language keywords, eg. `uint64`
impl Type {
    pub fn from_bytes_keyword(keyword: &str, data_location: Option<DataLocation>) -> Option<Self> {
        let width = keyword.strip_prefix("bytes").unwrap().parse::<u32>();
        if let Ok(width) = width {
            Some(Self::ByteArray(ByteArrayType { width }))
        } else {
            data_location.map(|data_location| {
                Self::Bytes(BytesType {
                    location: data_location,
                })
            })
        }
    }

    pub fn from_int_keyword(keyword: &str) -> Self {
        let bits = keyword
            .strip_prefix("int")
            .unwrap()
            .parse::<u32>()
            .unwrap_or(256);
        Self::Integer(IntegerType {
            is_signed: true,
            bits,
        })
    }

    pub fn from_uint_keyword(keyword: &str) -> Self {
        let bits = keyword
            .strip_prefix("uint")
            .unwrap()
            .parse::<u32>()
            .unwrap_or(256);
        Self::Integer(IntegerType {
            is_signed: false,
            bits,
        })
    }

    pub fn from_fixed_keyword(keyword: &str) -> Self {
        let (bits, decimal_places) =
            parse_fixed_point_suffix(keyword.strip_prefix("fixed").unwrap());
        Self::FixedPointNumber(FixedPointNumberType {
            is_signed: true,
            bits,
            decimal_places,
        })
    }

    pub fn from_ufixed_keyword(keyword: &str) -> Self {
        let (bits, decimal_places) =
            parse_fixed_point_suffix(keyword.strip_prefix("ufixed").unwrap());
        Self::FixedPointNumber(FixedPointNumberType {
            is_signed: false,
            bits,
            decimal_places,
        })
    }
}

/// Bare `fixed`/`ufixed` (an empty suffix) is an alias for the `128x18` variant.
const DEFAULT_FIXED_POINT_BITS: u32 = 128;
const DEFAULT_FIXED_POINT_DECIMAL_PLACES: u32 = 18;

/// Splits the `MxN` part of a `fixed`/`ufixed` keyword into bits and decimal places.
fn parse_fixed_point_suffix(suffix: &str) -> (u32, u32) {
    let Some((bits, decimal_places)) = suffix.split_once('x') else {
        return (DEFAULT_FIXED_POINT_BITS, DEFAULT_FIXED_POINT_DECIMAL_PLACES);
    };
    (bits.parse().unwrap(), decimal_places.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::types::{FixedPointNumberType, IntegerType, Type};

    fn fixed_point(is_signed: bool, bits: u32, decimal_places: u32) -> Type {
        Type::FixedPointNumber(FixedPointNumberType {
            is_signed,
            bits,
            decimal_places,
        })
    }

    #[test]
    fn integer_keywords() {
        assert_eq!(
            Type::from_int_keyword("int"),
            Type::Integer(IntegerType {
                is_signed: true,
                bits: 256
            })
        );
        assert_eq!(
            Type::from_uint_keyword("uint8"),
            Type::Integer(IntegerType {
                is_signed: false,
                bits: 8
            })
        );
    }

    #[test]
    fn fixed_point_keywords() {
        // A bare keyword is an alias for the `128x18` variant.
        assert_eq!(
            Type::from_fixed_keyword("fixed"),
            fixed_point(true, 128, 18)
        );
        assert_eq!(
            Type::from_ufixed_keyword("ufixed"),
            fixed_point(false, 128, 18)
        );

        assert_eq!(
            Type::from_fixed_keyword("fixed128x18"),
            Type::from_fixed_keyword("fixed")
        );
        assert_eq!(
            Type::from_ufixed_keyword("ufixed8x0"),
            fixed_point(false, 8, 0)
        );
        assert_eq!(
            Type::from_fixed_keyword("fixed184x80"),
            fixed_point(true, 184, 80)
        );
    }
}
