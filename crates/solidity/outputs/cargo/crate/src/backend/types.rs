use std::collections::HashMap;

use indexmap::IndexSet;

use crate::cst::NodeId;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TypeId(usize);

/// The `TypeRegistry` stores an index of registered types, both elementary
/// types and user defined types. Each type is given a `TypeId` for efficient
/// storage, lookup and comparison. The registry also provides direct access to
/// some common elementary types required to type most built-ins functions and
/// some kinds of expressions (eg. the boolean type).
pub struct TypeRegistry {
    types: IndexSet<Type>,
    super_types: HashMap<TypeId, Vec<TypeId>>,

    // Pre-defined core types
    address_type_id: TypeId,
    address_payable_type_id: TypeId,
    boolean_type_id: TypeId,
    boolean_bytes_tuple_type_id: TypeId,
    bytes_calldata_type_id: TypeId,
    bytes_memory_type_id: TypeId,
    bytes1_type_id: TypeId,
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
        let (bytes_calldata_type, _) = types.insert_full(Type::Bytes {
            location: DataLocation::Calldata,
        });
        let (bytes_memory_type, _) = types.insert_full(Type::Bytes {
            location: DataLocation::Memory,
        });
        let (bytes1_type, _) = types.insert_full(Type::ByteArray { width: 1 });
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
            types: vec![TypeId(boolean_type), TypeId(bytes_memory_type)],
        });

        Self {
            types,
            super_types: HashMap::new(),

            address_type_id: TypeId(address_type),
            address_payable_type_id: TypeId(address_payable_type),
            boolean_type_id: TypeId(boolean_type),
            boolean_bytes_tuple_type_id: TypeId(boolean_bytes_tuple_type),
            bytes_calldata_type_id: TypeId(bytes_calldata_type),
            bytes_memory_type_id: TypeId(bytes_memory_type),
            bytes1_type_id: TypeId(bytes1_type),
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

    pub fn register_super_types(&mut self, type_id: TypeId, super_types: Vec<TypeId>) {
        self.super_types.insert(type_id, super_types);
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

            (
                Type::Array {
                    element_type: from_element_type,
                    location: from_location,
                },
                Type::Array {
                    element_type: to_element_type,
                    location: to_location,
                },
            ) => {
                from_location.implicitly_convertible_to(*to_location)
                    && self.implicitly_convertible_to(*from_element_type, *to_element_type)
            }

            (
                Type::Struct {
                    definition_id: from_definition,
                    location: from_location,
                },
                Type::Struct {
                    definition_id: to_definition,
                    location: to_location,
                },
            ) => {
                from_location.implicitly_convertible_to(*to_location)
                    && from_definition == to_definition
            }

            (
                Type::Bytes {
                    location: from_location,
                },
                Type::Bytes {
                    location: to_location,
                },
            )
            | (
                Type::String {
                    location: from_location,
                },
                Type::String {
                    location: to_location,
                },
            ) => from_location.implicitly_convertible_to(*to_location),

            (Type::Function(from_function_type), Type::Function(to_function_type)) => {
                // This is full equality except for definition_id which can differ
                from_function_type.external == to_function_type.external
                    && from_function_type
                        .kind
                        .implicitly_convertible_to(to_function_type.kind)
                    && from_function_type.parameter_types == to_function_type.parameter_types
                    && from_function_type.return_type == to_function_type.return_type
            }

            (Type::Contract { .. }, Type::Contract { .. } | Type::Interface { .. })
            | (Type::Interface { .. }, Type::Interface { .. }) => self
                .super_types
                .get(&from_type_id)
                .is_some_and(|super_types| super_types.contains(&to_type_id)),

            // TODO: add more implicit conversion rules
            _ => false,
        }
    }

    pub fn implicitly_convertible_to_for_external_call(
        &self,
        from_type_id: TypeId,
        to_type_id: TypeId,
    ) -> bool {
        if from_type_id == to_type_id {
            return true;
        }
        let from_type = self.get_type_by_id(from_type_id);
        let to_type = self.get_type_by_id(to_type_id);

        // TODO(validation): we're assuming here that for external calls every
        // location is implicitly convertible to any other (although
        // reallistically the targets can be memory and calldata only). Verify
        // this assumption.
        match (from_type, to_type) {
            (
                Type::Array {
                    element_type: from_element_type,
                    ..
                },
                Type::Array {
                    element_type: to_element_type,
                    ..
                },
            ) => self
                .implicitly_convertible_to_for_external_call(*from_element_type, *to_element_type),

            (
                Type::Struct {
                    definition_id: from_definition,
                    ..
                },
                Type::Struct {
                    definition_id: to_definition,
                    ..
                },
            ) => from_definition == to_definition,

            (Type::Bytes { .. }, Type::Bytes { .. })
            | (Type::String { .. }, Type::String { .. }) => true,

            _ => self.implicitly_convertible_to(from_type_id, to_type_id),
        }
    }

    pub fn find_canonical_type_id(&self, type_id: TypeId) -> Option<TypeId> {
        let canonical_type = match self.get_type_by_id(type_id) {
            Type::Array { element_type, .. } => {
                let element_type = self.find_canonical_type_id(*element_type)?;
                Type::Array {
                    element_type,
                    location: DataLocation::Inherited,
                }
            }
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
            Type::Function(FunctionType {
                parameter_types,
                return_type,
                external,
                kind,
                ..
            }) => Type::Function(FunctionType {
                definition_id: None,
                parameter_types: parameter_types.clone(),
                return_type: *return_type,
                external: *external,
                kind: *kind,
            }),

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
            | Type::Void => return Some(type_id),
        };
        self.find_type(&canonical_type)
    }

    fn register_type_id_with_data_location(
        &mut self,
        type_id: TypeId,
        location: DataLocation,
    ) -> TypeId {
        let type_ = self.get_type_by_id(type_id).clone();
        self.register_type_with_data_location(type_, location)
    }

    pub fn register_type_with_data_location(
        &mut self,
        type_: Type,
        location: DataLocation,
    ) -> TypeId {
        let type_with_location = match type_ {
            Type::Array { element_type, .. } => Type::Array {
                element_type: self.register_type_id_with_data_location(element_type, location),
                location,
            },
            Type::Bytes { .. } => Type::Bytes { location },
            Type::String { .. } => Type::String { location },
            Type::Struct { definition_id, .. } => Type::Struct {
                definition_id,
                location,
            },
            Type::Tuple { types } => {
                let types_with_location = types
                    .iter()
                    .map(|id| self.register_type_id_with_data_location(*id, location))
                    .collect();
                Type::Tuple {
                    types: types_with_location,
                }
            }
            Type::Address { .. }
            | Type::Boolean
            | Type::ByteArray { .. }
            | Type::Contract { .. }
            | Type::Enum { .. }
            | Type::FixedPointNumber { .. }
            | Type::Function(_)
            | Type::Integer { .. }
            | Type::Interface { .. }
            | Type::Mapping { .. }
            | Type::Rational
            | Type::UserDefinedValue { .. }
            | Type::Void => type_,
        };
        self.register_type(type_with_location)
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
    pub fn bytes_calldata(&self) -> TypeId {
        self.bytes_calldata_type_id
    }
    pub fn bytes_memory(&self) -> TypeId {
        self.bytes_memory_type_id
    }
    pub fn bytes1(&self) -> TypeId {
        self.bytes1_type_id
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
    Function(FunctionType),
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FunctionType {
    pub definition_id: Option<NodeId>, // this may point to a FunctionDefinition
    pub parameter_types: Vec<TypeId>,
    pub return_type: TypeId,
    // TODO: a bool is not sufficient in some corner cases and we need to
    // distinguish between public and external
    pub external: bool,
    pub kind: FunctionTypeKind,
}

impl FunctionType {
    pub(crate) fn overrides(&self, other: &Self) -> bool {
        self.parameter_types == other.parameter_types && self.return_type == other.return_type
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DataLocation {
    Memory,
    Storage,
    Calldata,

    // This applies to struct fields of reference types, where the data location is the struct's.
    Inherited,
}

impl DataLocation {
    pub fn implicitly_convertible_to(&self, target: Self) -> bool {
        match (self, target) {
            (from, to) if *from == to => true,
            (DataLocation::Storage | DataLocation::Calldata, DataLocation::Memory) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FunctionTypeKind {
    Pure,
    View,
    NonPayable,
    Payable,
}

impl FunctionTypeKind {
    pub fn implicitly_convertible_to(&self, target: Self) -> bool {
        matches!(
            (self, target),
            // pure converts to view or non-payable
            (
                FunctionTypeKind::Pure,
                FunctionTypeKind::Pure | FunctionTypeKind::View | FunctionTypeKind::NonPayable,
            )
                // view converts to non-payable
                | (
                FunctionTypeKind::View,
                FunctionTypeKind::View | FunctionTypeKind::NonPayable
            )
                // non-payable does not implicitly convert to any other kind
                | (FunctionTypeKind::NonPayable, FunctionTypeKind::NonPayable)
                // payable converts to non-payable
                | (
                    FunctionTypeKind::Payable,
                    FunctionTypeKind::Payable | FunctionTypeKind::NonPayable,
                )
        )
    }
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

    pub fn is_inherited_location(&self) -> bool {
        self.data_location()
            .is_some_and(|location| location == DataLocation::Inherited)
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
