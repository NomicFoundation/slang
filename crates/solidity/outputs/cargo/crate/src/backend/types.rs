use indexmap::IndexSet;

use crate::cst::NodeId;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TypeId(usize);

pub struct TypeRegistry {
    types: IndexSet<Type>,

    // Pre-defined core types
    address_type_id: TypeId,
    boolean_type_id: TypeId,
    byte_type_id: TypeId,
    bytes4_type_id: TypeId,
    rational_type_id: TypeId,
    string_type_id: TypeId,
    uint256_type_id: TypeId,
    void_type_id: TypeId,
}

impl TypeRegistry {
    fn new() -> Self {
        let mut types = IndexSet::new();
        let (address_type, _) = types.insert_full(Type::Address { payable: false });
        let (boolean_type, _) = types.insert_full(Type::Boolean);
        let (byte_type, _) = types.insert_full(Type::Integer {
            signed: false,
            bits: 8,
        });
        let (bytes4_type, _) = types.insert_full(Type::ByteArray { width: 4 });
        let (rational_type, _) = types.insert_full(Type::Rational);
        let (string_type, _) = types.insert_full(Type::String {
            location: DataLocation::Memory,
        });
        let (uint256_type, _) = types.insert_full(Type::Integer {
            signed: false,
            bits: 256,
        });
        let (void_type, _) = types.insert_full(Type::Void);

        Self {
            types,

            address_type_id: TypeId(address_type),
            boolean_type_id: TypeId(boolean_type),
            byte_type_id: TypeId(byte_type),
            bytes4_type_id: TypeId(bytes4_type),
            rational_type_id: TypeId(rational_type),
            string_type_id: TypeId(string_type),
            uint256_type_id: TypeId(uint256_type),
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

    pub fn get_type_by_id(&self, type_id: TypeId) -> Option<&Type> {
        self.types.get_index(type_id.0)
    }

    pub fn implicitly_convertible_to(&self, from_type_id: TypeId, to_type_id: TypeId) -> bool {
        if from_type_id == to_type_id {
            return true;
        }
        let from_type = self.get_type_by_id(from_type_id).unwrap();
        let to_type = self.get_type_by_id(to_type_id).unwrap();

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

            (Type::Rational, Type::Integer { .. }) => true,

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
    pub fn boolean(&self) -> TypeId {
        self.boolean_type_id
    }
    pub fn byte(&self) -> TypeId {
        self.byte_type_id
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
    #[must_use]
    pub fn with_location(&self, location: DataLocation) -> Self {
        match self {
            Type::Array { element_type, .. } => Type::Array {
                element_type: *element_type,
                location,
            },
            Type::Bytes { .. } => Type::Bytes { location },
            Type::String { .. } => Type::String { location },
            Type::Struct { definition_id, .. } => Type::Struct {
                definition_id: *definition_id,
                location,
            },

            Type::Address { .. }
            | Type::Boolean
            | Type::ByteArray { .. }
            | Type::Contract { .. }
            | Type::Enum { .. }
            | Type::FixedPointNumber { .. }
            | Type::Function { .. }
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
