use super::{DataLocation, Type};

// Type instantiation from language keywords, eg. `uint64`
impl Type {
    pub fn from_bytes_keyword(keyword: &str, data_location: Option<DataLocation>) -> Option<Self> {
        let width = keyword.strip_prefix("bytes").unwrap().parse::<u32>();
        if let Ok(width) = width {
            Some(Self::ByteArray { width })
        } else {
            data_location.map(|data_location| Self::Bytes {
                location: data_location,
            })
        }
    }

    pub fn from_int_keyword(keyword: &str) -> Self {
        let bits = keyword
            .strip_prefix("int")
            .unwrap()
            .parse::<u32>()
            .unwrap_or(256);
        Self::Integer { signed: true, bits }
    }

    pub fn from_uint_keyword(keyword: &str) -> Self {
        let bits = keyword
            .strip_prefix("uint")
            .unwrap()
            .parse::<u32>()
            .unwrap_or(256);
        Self::Integer {
            signed: false,
            bits,
        }
    }

    pub fn from_fixed_keyword(keyword: &str) -> Self {
        let mut parts = keyword
            .strip_prefix("fixed")
            .unwrap()
            .split('x')
            .map(|part| part.parse::<u32>().unwrap());
        let bits = parts.next().unwrap();
        let precision_bits = parts.next().unwrap_or(0);
        Self::FixedPointNumber {
            signed: true,
            bits,
            precision_bits,
        }
    }

    pub fn from_ufixed_keyword(keyword: &str) -> Self {
        let mut parts = keyword
            .strip_prefix("ufixed")
            .unwrap()
            .split('x')
            .map(|part| part.parse::<u32>().unwrap());
        let bits = parts.next().unwrap();
        let precision_bits = parts.next().unwrap_or(0);
        Self::FixedPointNumber {
            signed: false,
            bits,
            precision_bits,
        }
    }
}
