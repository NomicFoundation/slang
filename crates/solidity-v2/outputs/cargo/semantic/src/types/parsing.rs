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
        Self::Integer(IntegerType { is_signed: true, bits })
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
        let mut parts = keyword
            .strip_prefix("fixed")
            .unwrap()
            .split('x')
            .map(|part| part.parse::<u32>().unwrap());
        let bits = parts.next().unwrap();
        let decimal_places = parts.next().unwrap_or(0);
        Self::FixedPointNumber(FixedPointNumberType {
            is_signed: true,
            bits,
            decimal_places,
        })
    }

    pub fn from_ufixed_keyword(keyword: &str) -> Self {
        let mut parts = keyword
            .strip_prefix("ufixed")
            .unwrap()
            .split('x')
            .map(|part| part.parse::<u32>().unwrap());
        let bits = parts.next().unwrap();
        let decimal_places = parts.next().unwrap_or(0);
        Self::FixedPointNumber(FixedPointNumberType {
            is_signed: false,
            bits,
            decimal_places,
        })
    }
}
