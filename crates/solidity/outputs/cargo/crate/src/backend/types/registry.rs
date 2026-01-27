use std::collections::HashMap;

use indexmap::IndexSet;
use semver::Version;

use super::{DataLocation, FunctionType, LiteralKind, Type, TypeId};
use crate::utils::versions::VERSION_0_5_0;

/// The `TypeRegistry` stores an index of registered types, both elementary
/// types and user defined types. Each type is given a `TypeId` for efficient
/// storage, lookup and comparison. The registry also provides direct access to
/// some common elementary types required to type most built-ins functions and
/// some kinds of expressions (eg. the boolean type).
pub struct TypeRegistry {
    types: IndexSet<Type>,
    super_types: HashMap<TypeId, Vec<TypeId>>,
    // implicit conversion rules are version dependant
    language_version: Version,

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
    string_type_id: TypeId,
    uint256_type_id: TypeId,
    uint8_type_id: TypeId,
    void_type_id: TypeId,
}

impl TypeRegistry {
    #[allow(clippy::similar_names)]
    pub fn new(language_version: Version) -> Self {
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
            language_version,

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

    #[allow(clippy::too_many_lines)]
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

            (
                Type::Literal(
                    LiteralKind::Zero
                    | LiteralKind::DecimalInteger
                    // TODO: rationals cannot be always converted to integers,
                    // unless their fractional part is zero. But for now and
                    // without further information about the literal number
                    // itself, assume it can.
                    | LiteralKind::Rational
                    | LiteralKind::HexInteger { .. },
                ),
                Type::Integer { .. },
            ) => {
                // TODO(validation): check that the rational can fit in the given integer type
                true
            }

            (
                Type::Literal(
                    LiteralKind::Zero
                    | LiteralKind::DecimalInteger
                    | LiteralKind::Rational
                    | LiteralKind::HexInteger { .. },
                ),
                Type::Literal(LiteralKind::Rational),
            ) |
            (
                Type::Literal(
                    LiteralKind::Zero
                    | LiteralKind::DecimalInteger
                    | LiteralKind::HexInteger { .. },
                ),
                Type::Literal(LiteralKind::DecimalInteger),
            ) |
            (
                Type::Literal(
                    LiteralKind::Zero
                    | LiteralKind::HexInteger { .. },
                ),
                Type::Literal(LiteralKind::HexInteger { .. }),
            ) => true,

            (Type::Integer { .. }, Type::Literal(_)) => false,

            (
                Type::Literal(LiteralKind::HexString { .. } | LiteralKind::String { .. }),
                Type::String { location } | Type::Bytes { location },
            ) if *location == DataLocation::Memory || *location == DataLocation::Calldata => true,

            (Type::Literal(LiteralKind::Zero), Type::ByteArray { .. }) => true,
            (
                Type::Literal(
                    LiteralKind::HexInteger { bytes }
                    | LiteralKind::HexString { bytes }
                    | LiteralKind::String { bytes },
                ),
                Type::ByteArray { width },
            ) if *bytes == *width => true,

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
                Type::FixedSizeArray {
                    element_type: from_element_type,
                    location: from_location,
                    size: from_size,
                },
                Type::FixedSizeArray {
                    element_type: to_element_type,
                    location: to_location,
                    size: to_size,
                },
            ) => {
                // conversion rules are strict and only allow changing data
                // location (from storage/calldata to memory)
                *from_size == *to_size
                    && *from_element_type == *to_element_type
                    && from_location.implicitly_convertible_to(*to_location)
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

            (Type::Contract { .. } | Type::Interface { .. }, Type::Address { payable: false }) => {
                // Contract references are implicitly convertible to `address`
                self.language_version < VERSION_0_5_0
            },

            // TODO: add more implicit conversion rules
            _ => false,
        }
    }

    // Similar to `implicitly_convertible_to` above, but with relaxed rules for
    // data location
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
                Type::FixedSizeArray {
                    element_type: from_element_type,
                    size: from_size,
                    ..
                },
                Type::FixedSizeArray {
                    element_type: to_element_type,
                    size: to_size,
                    ..
                },
            ) => from_size == to_size && from_element_type == to_element_type,

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
            Type::FixedSizeArray {
                element_type, size, ..
            } => {
                let element_type = self.find_canonical_type_id(*element_type)?;
                Type::FixedSizeArray {
                    element_type,
                    size: *size,
                    location: DataLocation::Inherited,
                }
            }
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
                implicit_receiver_type: None,
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
            | Type::Literal(_)
            | Type::Mapping { .. }
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
            Type::FixedSizeArray {
                element_type, size, ..
            } => Type::FixedSizeArray {
                element_type: self.register_type_id_with_data_location(element_type, location),
                size,
                location,
            },
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
            | Type::Literal(_)
            | Type::Mapping { .. }
            | Type::UserDefinedValue { .. }
            | Type::Void => type_,
        };
        self.register_type(type_with_location)
    }

    // Return a type that can be stored in the EVM. In short, convert literal
    // types into the appropriate "real" type
    pub fn reified_type(&mut self, type_id: TypeId) -> TypeId {
        let Type::Literal(kind) = self.get_type_by_id(type_id) else {
            return type_id;
        };
        match kind {
            // TODO: implementing these cases requires access to the value
            // itself, to fit the number in the smallest possible type. Eg. solc
            // will convert 1, 2, 3, etc into uint8 and 1.2 into ufixed8x1
            LiteralKind::Zero
            | LiteralKind::Rational
            | LiteralKind::DecimalInteger
            | LiteralKind::HexInteger { .. } => self.uint256(),
            LiteralKind::HexString { .. } | LiteralKind::String { .. } => self.string(),
            LiteralKind::Address => self.address(),
        }
    }

    // Returns true if a function type overrides another
    pub(crate) fn function_type_overrides(
        &self,
        ftype: &FunctionType,
        other: &FunctionType,
    ) -> bool {
        if ftype.parameter_types.len() != other.parameter_types.len()
            || ftype.return_type != other.return_type
        {
            return false;
        }
        if ftype.parameter_types == other.parameter_types {
            return true;
        }
        // TODO(validation): check that return parameters match

        // check if all parameter types are equal except maybe in the data
        // location: memory can override calldata as the location of a parameter
        ftype
            .parameter_types
            .iter()
            .zip(other.parameter_types.iter())
            .all(|(ptype_left, ptype_right)| {
                if ptype_left == ptype_right {
                    return true;
                }
                let type_left = self.get_type_by_id(*ptype_left);
                let type_right = self.get_type_by_id(*ptype_right);
                match (type_left, type_right) {
                    (
                        Type::Array {
                            element_type: element_type_left,
                            location: location_left,
                        },
                        Type::Array {
                            element_type: element_type_right,
                            location: location_right,
                        },
                    ) => {
                        element_type_left == element_type_right
                            && location_left.overrides(*location_right)
                    }
                    (
                        Type::FixedSizeArray {
                            element_type: element_type_left,
                            size: size_left,
                            location: location_left,
                        },
                        Type::FixedSizeArray {
                            element_type: element_type_right,
                            size: size_right,
                            location: location_right,
                        },
                    ) => {
                        element_type_left == element_type_right
                            && size_left == size_right
                            && location_left.overrides(*location_right)
                    }
                    (
                        Type::Bytes {
                            location: location_left,
                        },
                        Type::Bytes {
                            location: location_right,
                        },
                    )
                    | (
                        Type::String {
                            location: location_left,
                        },
                        Type::String {
                            location: location_right,
                        },
                    ) => location_left.overrides(*location_right),
                    (
                        Type::Struct {
                            definition_id: definition_id_left,
                            location: location_left,
                        },
                        Type::Struct {
                            definition_id: definition_id_right,
                            location: location_right,
                        },
                    ) => {
                        definition_id_left == definition_id_right
                            && location_left.overrides(*location_right)
                    }
                    _ => {
                        // anything else is not compatible because it should have
                        // the same type_id, or is not a valid type for a parameter
                        false
                    }
                }
            })
    }

    pub(crate) fn type_id_is_function_and_overrides(
        &self,
        type_id: TypeId,
        other_type_id: TypeId,
    ) -> bool {
        if type_id == other_type_id {
            return true;
        }
        let type_ = self.get_type_by_id(type_id);
        let other_type = self.get_type_by_id(other_type_id);
        match (type_, other_type) {
            (Type::Function(ftype), Type::Function(other)) => {
                self.function_type_overrides(ftype, other)
            }
            _ => false,
        }
    }
}

// Convenience accessors for pre-defined types
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
