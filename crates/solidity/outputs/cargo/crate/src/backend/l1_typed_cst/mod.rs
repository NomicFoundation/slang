mod generated;

use std::collections::HashMap;

pub use generated::*;

use super::types::{DataLocation, Type, TypeError};
use crate::compilation;

pub struct CompilationUnit {
    pub files: HashMap<String, SourceUnit>,
}

impl CompilationUnit {
    pub fn build(input: &compilation::CompilationUnit) -> Result<Self, String> {
        let mut files = HashMap::new();
        for file in &input.files() {
            files.insert(file.id().into(), builder::build_source_unit(file.tree())?);
        }
        Ok(Self { files })
    }
}

impl ElementaryType {
    pub fn try_to_type(&self) -> Result<Type, TypeError> {
        match self {
            Self::AddressType(address) => {
                let payable = address.payable_keyword.is_some();
                Ok(Type::Address { payable })
            }
            Self::BoolKeyword => Ok(Type::Boolean),
            Self::BytesKeyword(bytes) => {
                if bytes.text == "bytes" {
                    // Cannot convert bytes type without a storage location
                    Err(TypeError::MissingDataLocation)
                } else {
                    let width = bytes.text.strip_prefix("bytes").unwrap().parse().unwrap();
                    Ok(Type::ByteArray { width })
                }
            }
            Self::FixedKeyword(fixed) => {
                let parts: Vec<_> = fixed
                    .text
                    .strip_prefix("fixed")
                    .unwrap()
                    .split('x')
                    .map(|part| part.parse::<u32>().unwrap())
                    .collect();
                let (bits, precision_bits) = if parts.is_empty() {
                    (128, 18)
                } else {
                    (parts[0], parts[1])
                };
                Ok(Type::FixedPointNumber {
                    signed: true,
                    bits,
                    precision_bits,
                })
            }
            Self::UfixedKeyword(ufixed) => {
                let parts: Vec<_> = ufixed
                    .text
                    .strip_prefix("ufixed")
                    .unwrap()
                    .split('x')
                    .map(|part| part.parse::<u32>().unwrap())
                    .collect();
                let (bits, precision_bits) = if parts.is_empty() {
                    (128, 18)
                } else {
                    (parts[0], parts[1])
                };
                Ok(Type::FixedPointNumber {
                    signed: true,
                    bits,
                    precision_bits,
                })
            }
            Self::IntKeyword(int) => {
                let bits = int
                    .text
                    .strip_prefix("int")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(256);
                Ok(Type::Integer { signed: true, bits })
            }
            Self::UintKeyword(uint) => {
                let bits = uint
                    .text
                    .strip_prefix("uint")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(256);
                Ok(Type::Integer {
                    signed: false,
                    bits,
                })
            }
            Self::StringKeyword => {
                // Cannot convert a string type without a storage location
                Err(TypeError::MissingDataLocation)
            }
        }
    }

    pub fn to_type_with_location(&self, location: DataLocation) -> Result<Type, TypeError> {
        match self {
            Self::BytesKeyword(bytes) => {
                if bytes.text == "bytes" {
                    Ok(Type::Bytes { location })
                } else {
                    self.try_to_type()
                }
            }
            Self::StringKeyword => Ok(Type::String { location }),
            _ => self.try_to_type(),
        }
    }
}

impl StorageLocation {
    pub fn to_data_location(&self) -> DataLocation {
        match self {
            Self::CallDataKeyword => DataLocation::Calldata,
            Self::MemoryKeyword => DataLocation::Memory,
            Self::StorageKeyword => DataLocation::Storage,
        }
    }
}
