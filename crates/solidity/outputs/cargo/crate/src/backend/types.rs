use indexmap::IndexSet;

use crate::cst::NodeId;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TypeId(usize);

pub struct TypeRegistry {
    types: IndexSet<Type>,

    // Pre-defined core types
    address_type_id: TypeId,
    address_payable_type_id: TypeId,
    boolean_type_id: TypeId,
    boolean_bytes_tuple_type_id: TypeId,
    bytes_type_id: TypeId,
    bytes20_type_id: TypeId,
    bytes32_type_id: TypeId,
    bytes4_type_id: TypeId,
    rational_type_id: TypeId,
    string_type_id: TypeId,
    uint256_type_id: TypeId,
    uint8_type_id: TypeId,
    void_type_id: TypeId,
}

impl TypeRegistry {
    #[allow(clippy::similar_names)]
    fn new() -> Self {
        let mut types = IndexSet::new();
        let (address_type, _) = types.insert_full(Type::Address { payable: false });
        let (address_payable_type, _) = types.insert_full(Type::Address { payable: true });
        let (boolean_type, _) = types.insert_full(Type::Boolean);
        let (bytes_type, _) = types.insert_full(Type::Bytes {
            location: DataLocation::Memory,
        });
        let (bytes20_type, _) = types.insert_full(Type::ByteArray { width: 20 });
        let (bytes32_type, _) = types.insert_full(Type::ByteArray { width: 32 });
        let (bytes4_type, _) = types.insert_full(Type::ByteArray { width: 4 });
        let (rational_type, _) = types.insert_full(Type::Rational);
        let (string_type, _) = types.insert_full(Type::String {
            location: DataLocation::Memory,
        });
        let (uint256_type, _) = types.insert_full(Type::Integer {
            signed: false,
            bits: 256,
        });
        let (uint8_type, _) = types.insert_full(Type::Integer {
            signed: false,
            bits: 8,
        });
        let (void_type, _) = types.insert_full(Type::Void);

        let (boolean_bytes_tuple_type, _) = types.insert_full(Type::Tuple {
            types: vec![TypeId(boolean_type), TypeId(bytes_type)],
        });

        Self {
            types,

            address_type_id: TypeId(address_type),
            address_payable_type_id: TypeId(address_payable_type),
            boolean_type_id: TypeId(boolean_type),
            boolean_bytes_tuple_type_id: TypeId(boolean_bytes_tuple_type),
            bytes_type_id: TypeId(bytes_type),
            bytes20_type_id: TypeId(bytes20_type),
            bytes32_type_id: TypeId(bytes32_type),
            bytes4_type_id: TypeId(bytes4_type),
            rational_type_id: TypeId(rational_type),
            string_type_id: TypeId(string_type),
            uint256_type_id: TypeId(uint256_type),
            uint8_type_id: TypeId(uint8_type),
            void_type_id: TypeId(void_type),
        }
    }

    pub fn find_type(&self, type_: &Type) -> Option<TypeId> {
        self.types.get_index_of(type_).map(TypeId)
    }

    pub fn register_type(&mut self, type_: Type) -> TypeId {
        let (index, _) = self.types.insert_full(type_);
        TypeId(index)
    }

    pub fn get_type_by_id(&self, type_id: TypeId) -> &Type {
        self.types.get_index(type_id.0).unwrap()
    }

    pub fn implicitly_convertible_to(&self, from_type_id: TypeId, to_type_id: TypeId) -> bool {
        if from_type_id == to_type_id {
            return true;
        }
        let from_type = self.get_type_by_id(from_type_id);
        let to_type = self.get_type_by_id(to_type_id);

        match (from_type, to_type) {
            (
                Type::Address {
                    payable: from_payable,
                },
                Type::Address { .. },
            ) => *from_payable,

            (
                Type::Integer {
                    signed: from_signed,
                    bits: from_bits,
                },
                Type::Integer {
                    signed: to_signed,
                    bits: to_bits,
                },
            ) => {
                if from_signed == to_signed {
                    from_bits <= to_bits
                } else if *from_signed {
                    false
                } else {
                    from_bits < to_bits
                }
            }

            (Type::Rational, Type::Integer { .. }) => {
                // TODO(validation): check that the rational can fit in the given integer type
                true
            }

            (Type::Integer { .. }, Type::Rational) => false,

            // TODO: add more implicit conversion rules
            _ => unimplemented!(
                "implicitly converting from {from_type:?} to {to_type:?} not supported"
            ),
        }
    }
}

impl TypeRegistry {
    pub fn address(&self) -> TypeId {
        self.address_type_id
    }
    pub fn address_payable(&self) -> TypeId {
        self.address_payable_type_id
    }
    pub fn boolean(&self) -> TypeId {
        self.boolean_type_id
    }
    pub fn boolean_bytes_tuple(&self) -> TypeId {
        self.boolean_bytes_tuple_type_id
    }
    pub fn bytes(&self) -> TypeId {
        self.bytes_type_id
    }
    pub fn bytes20(&self) -> TypeId {
        self.bytes20_type_id
    }
    pub fn bytes32(&self) -> TypeId {
        self.bytes32_type_id
    }
    pub fn bytes4(&self) -> TypeId {
        self.bytes4_type_id
    }
    pub fn rational(&self) -> TypeId {
        self.rational_type_id
    }
    pub fn string(&self) -> TypeId {
        self.string_type_id
    }
    pub fn uint256(&self) -> TypeId {
        self.uint256_type_id
    }
    pub fn uint8(&self) -> TypeId {
        self.uint8_type_id
    }
    pub fn void(&self) -> TypeId {
        self.void_type_id
    }
}

impl TypeRegistry {
    pub fn iter_types(&self) -> impl Iterator<Item = (TypeId, &Type)> {
        (0usize..).map(TypeId).zip(self.types.iter())
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Address {
        payable: bool,
    },
    Array {
        element_type: TypeId,
        location: DataLocation,
        // TODO: handle fixed-size array types?
    },
    Boolean,
    ByteArray {
        width: u32,
    },
    Bytes {
        location: DataLocation,
    },
    Contract {
        definition_id: NodeId,
    },
    Enum {
        definition_id: NodeId,
    },
    FixedPointNumber {
        signed: bool,
        bits: u32,
        precision_bits: u32,
    },
    Function {
        definition_id: Option<NodeId>, // this may point to a FunctionDefinition
        parameter_types: Vec<TypeId>,
        return_type: TypeId,
        external: bool,
        kind: FunctionTypeKind,
    },
    Integer {
        signed: bool,
        bits: u32,
    },
    Interface {
        definition_id: NodeId,
    },
    Mapping {
        key_type_id: TypeId,
        value_type_id: TypeId,
    },
    Rational,
    String {
        location: DataLocation,
    },
    Struct {
        definition_id: NodeId,
        location: DataLocation,
    },
    Tuple {
        types: Vec<TypeId>,
    },
    UserDefinedValue {
        definition_id: NodeId,
    },
    Void,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DataLocation {
    Memory,
    Storage,
    Calldata,

    // This applies to struct fields of reference types, where the data location is the struct's.
    Inherited,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FunctionTypeKind {
    Pure,
    View,
    Payable,
}

impl Type {
    pub fn data_location(&self) -> Option<DataLocation> {
        match self {
            Self::Array { location, .. }
            | Self::Bytes { location }
            | Self::String { location }
            | Self::Struct { location, .. } => Some(*location),
            Self::Mapping { .. } => Some(DataLocation::Storage),
            _ => None,
        }
    }

    #[must_use]
    pub fn with_data_location(&self, data_location: Option<DataLocation>) -> Self {
        match self {
            Self::Array {
                element_type,
                location,
            } => Self::Array {
                element_type: *element_type,
                location: data_location.unwrap_or(*location),
            },
            Self::Bytes { location } => Self::Bytes {
                location: data_location.unwrap_or(*location),
            },
            Self::String { location } => Self::String {
                location: data_location.unwrap_or(*location),
            },
            Self::Struct {
                definition_id,
                location,
            } => Self::Struct {
                definition_id: *definition_id,
                location: data_location.unwrap_or(*location),
            },
            _ => self.clone(),
        }
    }

    #[must_use]
    pub fn canonicalize(&self) -> Self {
        match self {
            Type::Array { element_type, .. } => Type::Array {
                element_type: *element_type,
                location: DataLocation::Inherited,
            },
            Type::Bytes { .. } => Type::Bytes {
                location: DataLocation::Inherited,
            },
            Type::String { .. } => Type::String {
                location: DataLocation::Inherited,
            },
            Type::Struct { definition_id, .. } => Type::Struct {
                definition_id: *definition_id,
                location: DataLocation::Inherited,
            },
            Type::Function {
                parameter_types,
                return_type,
                external,
                kind,
                ..
            } => Type::Function {
                definition_id: None,
                parameter_types: parameter_types.clone(),
                return_type: *return_type,
                external: *external,
                kind: *kind,
            },

            Type::Address { .. }
            | Type::Boolean
            | Type::ByteArray { .. }
            | Type::Contract { .. }
            | Type::Enum { .. }
            | Type::FixedPointNumber { .. }
            | Type::Integer { .. }
            | Type::Interface { .. }
            | Type::Mapping { .. }
            | Type::Rational
            | Type::Tuple { .. }
            | Type::UserDefinedValue { .. }
            | Type::Void => self.clone(),
        }
    }
}

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
