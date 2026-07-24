use num_bigint::BigInt;
use num_traits::Zero;
use slang_solidity_v2_common::collections::{Map, OrderedSet};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;

use super::literals::numbers;
use super::{
    AddressType, ArrayType, ByteArrayType, BytesType, ContractType, DataLocation,
    FixedSizeArrayType, FunctionType, FunctionTypeVisibility, IntegerType, InterfaceType,
    LiteralKind, Number, StringType, StructType, TupleType, Type, TypeId,
};
use crate::types::ImplicitlyConvertible;

/// The `TypeRegistry` stores an index of registered types, both elementary
/// types and user defined types. Each type is given a `TypeId` for efficient
/// storage, lookup and comparison. The registry also provides direct access to
/// some common elementary types required to type most built-ins functions and
/// some kinds of expressions (eg. the boolean type).
pub struct TypeRegistry {
    types: OrderedSet<Type>,
    // This is used to register the _is-subclass_ and _implements_ relationships
    // between contract/interface types. The `NodeId`s correspond to the
    // `definition_id` in the respective `Type` variants.
    super_types: Map<NodeId, Vec<NodeId>>,
    // Some implicit conversion rules are version dependant. The version is
    // threaded in here so we can gate those rules on it.
    language_version: LanguageVersion,

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
    string_memory_type_id: TypeId,
    uint256_type_id: TypeId,
    uint8_type_id: TypeId,
    void_type_id: TypeId,
}

impl TypeRegistry {
    #[allow(clippy::similar_names)]
    pub(crate) fn new(language_version: LanguageVersion) -> Self {
        let mut types = OrderedSet::default();
        let (address_type, _) = types.insert_full(Type::Address(AddressType { is_payable: false }));
        let (address_payable_type, _) =
            types.insert_full(Type::Address(AddressType { is_payable: true }));
        let (boolean_type, _) = types.insert_full(Type::Boolean);
        let (bytes_calldata_type, _) = types.insert_full(Type::Bytes(BytesType {
            location: DataLocation::Calldata,
        }));
        let (bytes_memory_type, _) = types.insert_full(Type::Bytes(BytesType {
            location: DataLocation::Memory,
        }));
        let (bytes1_type, _) = types.insert_full(Type::ByteArray(ByteArrayType { width: 1 }));
        let (bytes20_type, _) = types.insert_full(Type::ByteArray(ByteArrayType { width: 20 }));
        let (bytes32_type, _) = types.insert_full(Type::ByteArray(ByteArrayType { width: 32 }));
        let (bytes4_type, _) = types.insert_full(Type::ByteArray(ByteArrayType { width: 4 }));
        let (string_memory_type, _) = types.insert_full(Type::String(StringType {
            location: DataLocation::Memory,
        }));
        let (uint256_type, _) = types.insert_full(Type::Integer(IntegerType {
            is_signed: false,
            bits: 256,
        }));
        let (uint8_type, _) = types.insert_full(Type::Integer(IntegerType {
            is_signed: false,
            bits: 8,
        }));
        let (void_type, _) = types.insert_full(Type::Void);

        let (boolean_bytes_tuple_type, _) = types.insert_full(Type::Tuple(TupleType {
            types: vec![TypeId(boolean_type), TypeId(bytes_memory_type)],
        }));

        Self {
            types,
            super_types: Map::default(),
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
            string_memory_type_id: TypeId(string_memory_type),
            uint256_type_id: TypeId(uint256_type),
            uint8_type_id: TypeId(uint8_type),
            void_type_id: TypeId(void_type),
        }
    }
}

impl TypeRegistry {
    pub(crate) fn find_type(&self, type_: &Type) -> Option<TypeId> {
        self.types.get_index_of(type_).map(TypeId)
    }

    pub(crate) fn register_type(&mut self, type_: Type) -> TypeId {
        let (index, _) = self.types.insert_full(type_);
        TypeId(index)
    }

    pub(crate) fn register_super_types(&mut self, type_id: TypeId, super_types: &[TypeId]) {
        let type_ = self.get_type_by_id(type_id);
        assert!(
            type_.is_contract_or_interface(),
            "can only register super types of contracts and interfaces",
        );
        let type_node_id = type_.get_definition_id().unwrap();
        let super_type_node_ids = super_types
            .iter()
            .map(|super_type_id| {
                let super_type = self.get_type_by_id(*super_type_id);
                assert!(
                    super_type.is_contract_or_interface(),
                    "super types can only be contracts or interfaces",
                );
                super_type.get_definition_id().unwrap()
            })
            .collect();
        self.super_types.insert(type_node_id, super_type_node_ids);
    }

    pub fn get_type_by_id(&self, type_id: TypeId) -> &Type {
        self.types.get_index(type_id.0).unwrap()
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn implicitly_convertible_to(
        &self,
        from_type_id: TypeId,
        to_type_id: TypeId,
    ) -> bool {
        if from_type_id == to_type_id {
            return true;
        }
        let from_type = self.get_type_by_id(from_type_id);
        let to_type = self.get_type_by_id(to_type_id);

        match (from_type, to_type) {
            (
                Type::Address(AddressType {
                    is_payable: from_payable,
                }),
                Type::Address(_),
            ) => *from_payable,

            (
                Type::Integer(IntegerType {
                    is_signed: from_signed,
                    bits: from_bits,
                }),
                Type::Integer(IntegerType {
                    is_signed: to_signed,
                    bits: to_bits,
                }),
            ) => {
                if from_signed == to_signed {
                    // Same signedness widens (or keeps the same width).
                    from_bits <= to_bits
                } else if *from_signed {
                    // `intN` never implicitly converts to an unsigned type.
                    false
                } else {
                    // `uintN` was implicitly convertible to `intM` for `M > N`
                    // until 0.8.1 disallowed it.
                    // https://github.com/ethereum/solidity/releases/tag/v0.8.1
                    self.language_version < LanguageVersion::V0_8_1 && from_bits < to_bits
                }
            }

            (
                Type::Literal(LiteralKind::Integer { value }),
                Type::Integer(IntegerType { is_signed, bits }),
            ) => numbers::integer_literal_fits(value, *is_signed, *bits),

            (
                Type::Literal(LiteralKind::HexInteger { value, .. }),
                Type::Integer(IntegerType { is_signed, bits }),
            ) => numbers::integer_literal_fits(&BigInt::from(value.clone()), *is_signed, *bits),

            // Non-integer rational literals never implicitly convert to an
            // integer type — if a rational reduced to an integer it would have
            // been normalised to `LiteralKind::Integer` at construction time.
            (Type::Literal(LiteralKind::Rational { .. }), Type::Integer(_)) => false,

            // TODO: Rational -> FixedPointNumber once v2 models implicit
            // conversion of rational literals to fixed-point types.
            (Type::Integer(_), Type::Literal(_)) => false,

            (
                Type::Literal(LiteralKind::HexString { .. } | LiteralKind::String { .. }),
                Type::String(StringType { location }) | Type::Bytes(BytesType { location }),
            ) if *location == DataLocation::Memory || *location == DataLocation::Calldata => true,

            // Zero (any source — decimal, hex, or folded) is always
            // implicitly convertible to any byte-array type.
            (Type::Literal(LiteralKind::Integer { value }), Type::ByteArray(_))
                if value.is_zero() =>
            {
                true
            }
            (Type::Literal(LiteralKind::HexInteger { value, .. }), Type::ByteArray(_))
                if value.is_zero() =>
            {
                true
            }

            // Non-zero hex-source literals convert to `bytesN` of exactly
            // matching source byte width. Plain `Integer` literals (decimal
            // source or any folded result) do NOT — folding any operation
            // erases the hex provenance and disables this conversion.
            (
                Type::Literal(LiteralKind::HexInteger { bytes, .. }),
                Type::ByteArray(ByteArrayType { width }),
            ) if *bytes == *width => true,

            (
                Type::Literal(LiteralKind::HexString { bytes } | LiteralKind::String { bytes }),
                Type::ByteArray(ByteArrayType { width }),
            ) if *bytes == *width as usize => true,

            (
                Type::Array(ArrayType {
                    element_type: from_element_type,
                    location: from_location,
                }),
                Type::Array(ArrayType {
                    element_type: to_element_type,
                    location: to_location,
                }),
            ) => {
                from_location.implicitly_convertible_to(*to_location)
                    && self.implicitly_convertible_to(*from_element_type, *to_element_type)
            }

            (
                Type::FixedSizeArray(FixedSizeArrayType {
                    element_type: from_element_type,
                    location: from_location,
                    size: from_size,
                }),
                Type::FixedSizeArray(FixedSizeArrayType {
                    element_type: to_element_type,
                    location: to_location,
                    size: to_size,
                }),
            ) => {
                // conversion rules are strict and only allow changing data
                // location (from storage/calldata to memory)
                *from_size == *to_size
                    && *from_element_type == *to_element_type
                    && from_location.implicitly_convertible_to(*to_location)
            }

            (
                Type::Struct(StructType {
                    definition_id: from_definition,
                    location: from_location,
                }),
                Type::Struct(StructType {
                    definition_id: to_definition,
                    location: to_location,
                }),
            ) => {
                from_location.implicitly_convertible_to(*to_location)
                    && from_definition == to_definition
            }

            (
                Type::Bytes(BytesType {
                    location: from_location,
                }),
                Type::Bytes(BytesType {
                    location: to_location,
                }),
            )
            | (
                Type::String(StringType {
                    location: from_location,
                }),
                Type::String(StringType {
                    location: to_location,
                }),
            ) => from_location.implicitly_convertible_to(*to_location),

            // `bytesN` widens to `bytesM` when `M >= N`.
            (
                Type::ByteArray(ByteArrayType { width: from_width }),
                Type::ByteArray(ByteArrayType { width: to_width }),
            ) => from_width <= to_width,

            (Type::Function(from_function_type), Type::Function(to_function_type)) => {
                // This is full equality except for visibility and mutability
                // which can be converted to, partially_applied which must be `false`,
                // and definition_id and implicit_receiver_type which can differ.
                //
                // Note: solc checks that the `from` and `to` function types have
                // the same call options or bound argument applied in the same way,
                // but that is not observable from Solidity source code.
                // Therefore we have a stronger check here that has the same effect
                // for users.
                !from_function_type.partially_applied
                    && !to_function_type.partially_applied
                    && from_function_type
                        .visibility
                        .implicitly_convertible_to(to_function_type.visibility)
                    && from_function_type
                        .mutability
                        .implicitly_convertible_to(to_function_type.mutability)
                    && from_function_type.parameter_types == to_function_type.parameter_types
                    && from_function_type.return_type == to_function_type.return_type
            }

            // Contracts and interfaces are implicitly converted to their super types
            (
                Type::Contract(ContractType {
                    definition_id: from_node_id,
                }),
                Type::Contract(ContractType {
                    definition_id: to_node_id,
                })
                | Type::Interface(InterfaceType {
                    definition_id: to_node_id,
                }),
            )
            | (
                Type::Interface(InterfaceType {
                    definition_id: from_node_id,
                }),
                Type::Interface(InterfaceType {
                    definition_id: to_node_id,
                }),
            ) => self
                .super_types
                .get(from_node_id)
                .is_some_and(|super_types| super_types.contains(to_node_id)),

            (
                Type::Tuple(TupleType { types: from_types }),
                Type::Tuple(TupleType { types: to_types }),
            ) => {
                // TODO(validation) SDR[59]: assignment LHS holes (`(a, ) = f()`)
                // are modeled as `Void` tuple elements, which reject any
                // concrete source element here. solc instead skips empty
                // target components (`TupleType::isImplicitlyConvertibleTo`),
                // so once assignments are validated with this rule, holes need
                // a special case
                from_types.len() == to_types.len()
                    && from_types
                        .iter()
                        .zip(to_types.iter())
                        .all(|(from, to)| self.implicitly_convertible_to(*from, *to))
            }

            // TODO: add more implicit conversion rules
            _ => false,
        }
    }

    // Similar to `implicitly_convertible_to` above, but with relaxed rules for
    // data location
    pub(crate) fn implicitly_convertible_to_for_external_call(
        &self,
        from_type_id: TypeId,
        to_type_id: TypeId,
    ) -> bool {
        if from_type_id == to_type_id {
            return true;
        }
        let from_type = self.get_type_by_id(from_type_id);
        let to_type = self.get_type_by_id(to_type_id);

        // TODO(validation) SDR[60]: we're assuming here that for external calls every
        // location is implicitly convertible to any other (although
        // reallistically the targets can be memory and calldata only). Verify
        // this assumption.
        match (from_type, to_type) {
            (
                Type::Array(ArrayType {
                    element_type: from_element_type,
                    ..
                }),
                Type::Array(ArrayType {
                    element_type: to_element_type,
                    ..
                }),
            ) => self
                .implicitly_convertible_to_for_external_call(*from_element_type, *to_element_type),

            (
                Type::FixedSizeArray(FixedSizeArrayType {
                    element_type: from_element_type,
                    size: from_size,
                    ..
                }),
                Type::FixedSizeArray(FixedSizeArrayType {
                    element_type: to_element_type,
                    size: to_size,
                    ..
                }),
            ) => from_size == to_size && from_element_type == to_element_type,

            (
                Type::Struct(StructType {
                    definition_id: from_definition,
                    ..
                }),
                Type::Struct(StructType {
                    definition_id: to_definition,
                    ..
                }),
            ) => from_definition == to_definition,

            (Type::Bytes(_), Type::Bytes(_)) | (Type::String(_), Type::String(_)) => true,

            _ => self.implicitly_convertible_to(from_type_id, to_type_id),
        }
    }

    // Changes a function type to have external visibility and any parameters
    // normalized for that (ie. `calldata` location is changed to `memory`)
    pub(crate) fn externalize_function_type(
        &mut self,
        function_type: FunctionType,
    ) -> FunctionType {
        FunctionType {
            visibility: FunctionTypeVisibility::External,
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

    // Marks a function type as partially applied:
    // - a function from a `using` directive applied on a value
    // - call options have been pre-applied in an external call
    pub(crate) fn partially_apply_function_type(&mut self, function_type: FunctionType) -> TypeId {
        self.register_type(Type::Function(FunctionType {
            partially_applied: true,
            ..function_type
        }))
    }

    pub(crate) fn find_canonical_type_id(&self, type_id: TypeId) -> Option<TypeId> {
        let canonical_type = match self.get_type_by_id(type_id) {
            Type::Array(ArrayType { element_type, .. }) => {
                let element_type = self.find_canonical_type_id(*element_type)?;
                Type::Array(ArrayType {
                    element_type,
                    location: DataLocation::Inherited,
                })
            }
            Type::Bytes(_) => Type::Bytes(BytesType {
                location: DataLocation::Inherited,
            }),
            Type::FixedSizeArray(FixedSizeArrayType {
                element_type, size, ..
            }) => {
                let element_type = self.find_canonical_type_id(*element_type)?;
                Type::FixedSizeArray(FixedSizeArrayType {
                    element_type,
                    size: *size,
                    location: DataLocation::Inherited,
                })
            }
            Type::String(_) => Type::String(StringType {
                location: DataLocation::Inherited,
            }),
            Type::Struct(StructType { definition_id, .. }) => Type::Struct(StructType {
                definition_id: *definition_id,
                location: DataLocation::Inherited,
            }),
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
                partially_applied: false,
            }),

            Type::Address(_)
            | Type::Boolean
            | Type::ByteArray(_)
            | Type::Contract(_)
            | Type::Enum(_)
            | Type::FixedPointNumber(_)
            | Type::Integer(_)
            | Type::Interface(_)
            | Type::Library(_)
            | Type::Literal(_)
            | Type::Mapping(_)
            | Type::MetaType(_)
            | Type::Tuple(_)
            | Type::UserDefinedValue(_)
            | Type::UserMetaType(_)
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

    pub(crate) fn register_type_with_data_location(
        &mut self,
        type_: Type,
        location: DataLocation,
    ) -> TypeId {
        let type_with_location = match type_ {
            Type::Array(ArrayType { element_type, .. }) => Type::Array(ArrayType {
                element_type: self.register_type_id_with_data_location(element_type, location),
                location,
            }),
            Type::Bytes(_) => Type::Bytes(BytesType { location }),
            Type::FixedSizeArray(FixedSizeArrayType {
                element_type, size, ..
            }) => Type::FixedSizeArray(FixedSizeArrayType {
                element_type: self.register_type_id_with_data_location(element_type, location),
                size,
                location,
            }),
            Type::String(_) => Type::String(StringType { location }),
            Type::Struct(StructType { definition_id, .. }) => Type::Struct(StructType {
                definition_id,
                location,
            }),
            Type::Tuple(TupleType { types }) => {
                let types_with_location = types
                    .iter()
                    .map(|id| self.register_type_id_with_data_location(*id, location))
                    .collect();
                Type::Tuple(TupleType {
                    types: types_with_location,
                })
            }
            Type::Address(_)
            | Type::Boolean
            | Type::ByteArray(_)
            | Type::Contract(_)
            | Type::Enum(_)
            | Type::FixedPointNumber(_)
            | Type::Function(_)
            | Type::Integer(_)
            | Type::Interface(_)
            | Type::Library(_)
            | Type::Literal(_)
            | Type::Mapping(_)
            | Type::MetaType(_)
            | Type::UserDefinedValue(_)
            | Type::UserMetaType(_)
            | Type::Void => type_,
        };
        self.register_type(type_with_location)
    }

    /// Computes the mobile type of `type_id` and returns its `TypeId`.
    pub(crate) fn compute_mobile_type(&mut self, type_id: TypeId) -> Option<TypeId> {
        match self.get_type_by_id(type_id).clone() {
            Type::Literal(kind) => {
                let mobile = kind.mobile_type()?;
                Some(self.register_type(mobile))
            }
            Type::Tuple(TupleType { types: element_ids }) => {
                let mobile_ids: Option<Vec<TypeId>> = element_ids
                    .iter()
                    .map(|id| self.compute_mobile_type(*id))
                    .collect();
                Some(self.register_type(Type::Tuple(TupleType { types: mobile_ids? })))
            }
            // A partially applied function (bound first argument or pre-applied
            // call options) doesn't have a mobile type.
            //
            // Matches solc behaviour
            Type::Function(function_type) if function_type.partially_applied => None,
            _ => Some(type_id),
        }
    }

    /// The type both operands can convert into. It takes the mobile type of one
    /// side and checks whether the *raw* other side implicitly converts into it.
    pub(crate) fn common_type(&mut self, left: TypeId, right: TypeId) -> Option<TypeId> {
        if let Some(left_mobile) = self.compute_mobile_type(left)
            && self.implicitly_convertible_to(right, left_mobile)
        {
            return Some(left_mobile);
        }
        if let Some(right_mobile) = self.compute_mobile_type(right)
            && self.implicitly_convertible_to(left, right_mobile)
        {
            return Some(right_mobile);
        }
        None
    }

    /// Return a type that can be stored in the EVM and can hold values of all
    /// the given types. The first element dictates the type class. Returns
    /// `None` if the types cannot be reified, they are not compatible, or they don't belong
    /// in an array literal.
    ///
    /// This is used to unify types of literal arrays.
    /// Only the first element is mobile-typed unconditionally.
    pub(crate) fn type_of_array_literal(&mut self, type_ids: &[TypeId]) -> Option<TypeId> {
        let (first_id, rest) = type_ids.split_first()?;
        if !self.get_type_by_id(*first_id).can_be_array_element() {
            // TODO(validation) SDR[750]: Error if the element type can't be an array element
            return None;
        }
        let mut element_type_id = self.compute_mobile_type(*first_id)?;
        for &item_type_id in rest {
            if self.implicitly_convertible_to(item_type_id, element_type_id) {
                // Item already fits the accumulator
                continue;
            }
            if !self.get_type_by_id(item_type_id).can_be_array_element() {
                // TODO(validation) SDR[750]: Error if the element type can't be an array element
                return None;
            }
            let item_mobile_type_id = self.compute_mobile_type(item_type_id)?;
            if !self.implicitly_convertible_to(element_type_id, item_mobile_type_id) {
                // TODO(validation) SDR[1741,1353]: types are not compatible
                return None;
            }
            element_type_id = item_mobile_type_id;
        }
        Some(element_type_id)
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
        } else if matches!(other.visibility, FunctionTypeVisibility::External)
            && matches!(
                ftype.visibility,
                FunctionTypeVisibility::External | FunctionTypeVisibility::Public
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
                Type::Array(ArrayType {
                    element_type: element_type_left,
                    location: location_left,
                }),
                Type::Array(ArrayType {
                    element_type: element_type_right,
                    location: location_right,
                }),
            ) => {
                location_left.overrides_in_external_function(*location_right)
                    && self.parameter_type_overrides_in_external_function(
                        *element_type_left,
                        *element_type_right,
                    )
            }
            (
                Type::FixedSizeArray(FixedSizeArrayType {
                    element_type: element_type_left,
                    size: size_left,
                    location: location_left,
                }),
                Type::FixedSizeArray(FixedSizeArrayType {
                    element_type: element_type_right,
                    size: size_right,
                    location: location_right,
                }),
            ) => {
                size_left == size_right
                    && location_left.overrides_in_external_function(*location_right)
                    && self.parameter_type_overrides_in_external_function(
                        *element_type_left,
                        *element_type_right,
                    )
            }
            (
                Type::Bytes(BytesType {
                    location: location_left,
                }),
                Type::Bytes(BytesType {
                    location: location_right,
                }),
            )
            | (
                Type::String(StringType {
                    location: location_left,
                }),
                Type::String(StringType {
                    location: location_right,
                }),
            ) => location_left.overrides_in_external_function(*location_right),
            (
                Type::Struct(StructType {
                    definition_id: definition_id_left,
                    location: location_left,
                }),
                Type::Struct(StructType {
                    definition_id: definition_id_right,
                    location: location_right,
                }),
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

    /// Returns the compile-time number value of a type, when it is a
    /// value-bearing literal (integer or rational).
    pub fn number_value_of_type_id(&self, type_id: TypeId) -> Option<Number> {
        match self.get_type_by_id(type_id) {
            Type::Literal(kind) => Number::from_literal_kind(kind),
            _ => None,
        }
    }
}

// Convenience accessors for pre-defined types
impl TypeRegistry {
    pub(crate) fn address(&self) -> TypeId {
        self.address_type_id
    }
    pub(crate) fn address_payable(&self) -> TypeId {
        self.address_payable_type_id
    }
    pub(crate) fn boolean(&self) -> TypeId {
        self.boolean_type_id
    }
    pub(crate) fn boolean_bytes_tuple(&self) -> TypeId {
        self.boolean_bytes_tuple_type_id
    }
    pub(crate) fn bytes_calldata(&self) -> TypeId {
        self.bytes_calldata_type_id
    }
    pub(crate) fn bytes_memory(&self) -> TypeId {
        self.bytes_memory_type_id
    }
    pub(crate) fn bytes1(&self) -> TypeId {
        self.bytes1_type_id
    }
    pub(crate) fn bytes20(&self) -> TypeId {
        self.bytes20_type_id
    }
    pub(crate) fn bytes32(&self) -> TypeId {
        self.bytes32_type_id
    }
    pub(crate) fn bytes4(&self) -> TypeId {
        self.bytes4_type_id
    }
    pub(crate) fn string_memory(&self) -> TypeId {
        self.string_memory_type_id
    }
    pub(crate) fn uint256(&self) -> TypeId {
        self.uint256_type_id
    }
    pub(crate) fn uint8(&self) -> TypeId {
        self.uint8_type_id
    }
    pub(crate) fn void(&self) -> TypeId {
        self.void_type_id
    }
}

#[cfg(test)]
impl TypeRegistry {
    pub(crate) fn iter_types(&self) -> impl Iterator<Item = (TypeId, &Type)> {
        (0usize..).map(TypeId).zip(self.types.iter())
    }
}
