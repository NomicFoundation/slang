use std::collections::HashMap;

use indexmap::IndexSet;
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use slang_solidity_v2_ir::ir;

use super::{DataLocation, FunctionType, LiteralKind, Type, TypeId};
use crate::types::ImplicitlyConvertible;

/// Number of bits required to hold `value` as a Solidity integer of the given
/// signedness:
/// - unsigned: exactly `value.bits()` (with at least 1, so zero still has a
///   positive width).
/// - signed positive: `value.bits() + 1` (one extra bit for the sign).
/// - signed negative: `(-value - 1).bits() + 1` (two's-complement width).
fn integer_bits_required(value: &BigInt, signed: bool) -> u32 {
    if !signed {
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
fn integer_literal_fits(value: &BigInt, signed: bool, bits: u32) -> bool {
    if !signed && value.is_negative() {
        return false;
    }
    integer_bits_required(value, signed) <= bits
}

fn smallest_integer_type_to_fit(values: &[&BigInt]) -> Option<Type> {
    if values.is_empty() {
        return None;
    }
    let signed = values.iter().all(|value| value.is_negative());
    let bits = values.iter().fold(0u32, |acc, value| {
        acc.max(integer_bits_required(value, signed))
    });

    if bits > 256 {
        // TODO(validation): the integers don't fit in the EVM
        return None;
    }
    let bits = bits.next_multiple_of(8).max(8);
    Some(Type::Integer { signed, bits })
}

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
    string_type_id: TypeId,
    uint256_type_id: TypeId,
    uint8_type_id: TypeId,
    void_type_id: TypeId,
}

impl Default for TypeRegistry {
    #[allow(clippy::similar_names)]
    fn default() -> Self {
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
}

impl TypeRegistry {
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
                Type::Literal(LiteralKind::Integer(value) | LiteralKind::HexInteger { value, .. }),
                Type::Integer { signed, bits },
            ) => integer_literal_fits(value, *signed, *bits),

            // Non-integer rational literals never implicitly convert to an
            // integer type — if a rational reduced to an integer it would have
            // been normalised to `LiteralKind::Integer` at construction time.
            (Type::Literal(LiteralKind::Rational(_)), Type::Integer { .. }) => false,

            // TODO: Rational -> FixedPointNumber once v2 models implicit
            // conversion of rational literals to fixed-point types.
            (Type::Integer { .. }, Type::Literal(_)) => false,

            (
                Type::Literal(LiteralKind::HexString { .. } | LiteralKind::String { .. }),
                Type::String { location } | Type::Bytes { location },
            ) if *location == DataLocation::Memory || *location == DataLocation::Calldata => true,

            // Zero (any source — decimal, hex, or folded) is always
            // implicitly convertible to any byte-array type.
            (
                Type::Literal(LiteralKind::Integer(value) | LiteralKind::HexInteger { value, .. }),
                Type::ByteArray { .. },
            ) if value.is_zero() => true,

            // Non-zero hex-source literals convert to `bytesN` of exactly
            // matching source byte width. Plain `Integer` literals (decimal
            // source or any folded result) do NOT — folding any operation
            // erases the hex provenance and disables this conversion.
            (Type::Literal(LiteralKind::HexInteger { bytes, .. }), Type::ByteArray { width })
                if *bytes == *width =>
            {
                true
            }

            (
                Type::Literal(LiteralKind::HexString { bytes } | LiteralKind::String { bytes }),
                Type::ByteArray { width },
            ) if *bytes == *width as usize => true,

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
                // This is full equality except for visibility and mutability
                // which can be converted to, and definition_id which can differ
                from_function_type
                    .visibility
                    .implicitly_convertible_to(to_function_type.visibility)
                    && from_function_type
                        .mutability
                        .implicitly_convertible_to(to_function_type.mutability)
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

    // Changes a function type to have external visibility and any parameters
    // normalized for that (ie. `calldata` location is changed to `memory`)
    pub fn externalize_function_type(&mut self, function_type: FunctionType) -> FunctionType {
        FunctionType {
            visibility: ir::FunctionVisibility::External,
            parameter_types: function_type
                .parameter_types
                .into_iter()
                .map(|parameter_type_id| {
                    let parameter_type = self.get_type_by_id(parameter_type_id);
                    if parameter_type
                        .data_location()
                        .is_some_and(|location| location == DataLocation::Calldata)
                    {
                        self.register_type_with_data_location(
                            parameter_type.clone(),
                            DataLocation::Memory,
                        )
                    } else {
                        parameter_type_id
                    }
                })
                .collect(),
            ..function_type
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
                visibility,
                mutability,
                ..
            }) => Type::Function(FunctionType {
                definition_id: None,
                implicit_receiver_type: None,
                parameter_types: parameter_types.clone(),
                return_type: *return_type,
                visibility: *visibility,
                mutability: *mutability,
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

    /// Returns the smallest `Type::Integer { signed, bits }` that holds both
    /// `a` and `b` (with `bits` rounded up to a multiple of 8, in the range
    /// 8..=256). Used to unify two distinct integer literal values when one
    /// is not implicitly convertible to the other (e.g. in `cond ? 0 : 1`).
    pub fn common_integer_literal_type(&mut self, a: &BigInt, b: &BigInt) -> Option<TypeId> {
        smallest_integer_type_to_fit(&[a, b]).map(|type_| self.register_type(type_))
    }

    // Return a type that can be stored in the EVM. In short, convert literal
    // types into the appropriate "real" type
    pub fn reified_type(&mut self, type_id: TypeId) -> Option<TypeId> {
        let Type::Literal(kind) = self.get_type_by_id(type_id) else {
            return Some(type_id);
        };
        match kind {
            LiteralKind::Integer(value) | LiteralKind::HexInteger { value, .. } => {
                smallest_integer_type_to_fit(&[value]).map(|type_| self.register_type(type_))
            }
            LiteralKind::Rational(_) => {
                // TODO: narrow the rational type to the smallest fixed/ufixed
                // available (eg. 1.2 -> ufixed8x1). For now, default to uint256
                // to preserve current behaviour.
                Some(self.uint256())
            }
            LiteralKind::HexString { .. } | LiteralKind::String { .. } => Some(self.string()),
            LiteralKind::Address => Some(self.address()),
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

        // In general, parameter types must match exactly for functions to
        // override others.
        if ftype.parameter_types == other.parameter_types {
            true

        // The exception is if the `other` function is external which allows
        // changing the data location of parameters from `memory` to `calldata`
        // (or viceversa) if the visibility of `ftype` is external or public.
        } else if matches!(other.visibility, ir::FunctionVisibility::External)
            && matches!(
                ftype.visibility,
                ir::FunctionVisibility::External | ir::FunctionVisibility::Public
            )
        {
            ftype
                .parameter_types
                .iter()
                .zip(other.parameter_types.iter())
                .all(|(ptype_left, ptype_right)| {
                    self.parameter_type_overrides_in_external_function(*ptype_left, *ptype_right)
                })
        } else {
            // Parameter types don't match: `ftype` *does not* override `other`.
            false
        }
    }

    // Returns true if a the `left` type can override the `right` type in an
    // external function signature. External functions allow changing data
    // location from `memory` to `calldata` and viceversa.
    fn parameter_type_overrides_in_external_function(&self, left: TypeId, right: TypeId) -> bool {
        if left == right {
            return true;
        }
        let type_left = self.get_type_by_id(left);
        let type_right = self.get_type_by_id(right);
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
                location_left.overrides_in_external_function(*location_right)
                    && self.parameter_type_overrides_in_external_function(
                        *element_type_left,
                        *element_type_right,
                    )
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
                size_left == size_right
                    && location_left.overrides_in_external_function(*location_right)
                    && self.parameter_type_overrides_in_external_function(
                        *element_type_left,
                        *element_type_right,
                    )
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
            ) => location_left.overrides_in_external_function(*location_right),
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
                    && location_left.overrides_in_external_function(*location_right)
            }
            _ => {
                // anything else is not compatible because it should have
                // the same type_id, or is not a valid type for a parameter
                false
            }
        }
    }

    pub fn type_id_is_function_and_overrides(
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
