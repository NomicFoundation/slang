use std::fmt;

use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::nodes::NodeId;

use super::SemanticContext;
use crate::binder::Definition;
use crate::types::{
    ArraySliceType, ArrayType, ByteArrayType, ContractType, DataLocation, EnumType, ErrorType,
    EventType, FixedPointNumberType, FixedSizeArrayType, FunctionType, FunctionTypeMutability,
    IntegerType, InterfaceType, LibraryType, MappingType, MetaType, StructType, TupleType, Type,
    TypeId, UserDefinedValueType, UserMetaType,
};

/// How [`SemanticContext::write_type_abi_name`] spells a type. They differ only on these types:
///
/// | Type                   | `Selector`   | `LibrarySelector`  | `Json`       | `LibraryJson` |
/// |------------------------|--------------|--------------------|--------------|---------------|
/// | enum                   | `uint8`      | `C.E`              | `uint8`      | `C.E`         |
/// | contract, interface    | `address`    | `C`                | `address`    | `C`           |
/// | struct                 | `(T1,T2)`    | `C.S`              | `tuple`      | `tuple`       |
/// | storage reference      | as above     | ` storage` suffix  | as above     | as above      |
/// | slice                  | as its array | `T[] slice`        | as its array | as its array  |
/// | mapping, other non-ABI | no name      | internal name      | no name      | no name       |
///
/// In all of them an array is spelled through its element, and a user-defined value type as the
/// type it wraps.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AbiTypeSpelling {
    /// The canonical form a selector is hashed from.
    Selector,
    /// The form a library function's selector is hashed from.
    LibrarySelector,
    /// The JSON ABI `type`.
    Json,
    /// A library function's JSON ABI `type`, matching its selector's names.
    LibraryJson,
}

/// Why [`SemanticContext::write_type_abi_name`] could not write a type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AbiNameError {
    /// The writer failed.
    Write(fmt::Error),
    /// The spelling has no name for the type, e.g. a mapping or a recursive struct in the
    /// canonical form.
    NoAbiName(TypeId),
    /// A node the type depends on has no type or definition.
    Unresolved(NodeId),
}

impl From<fmt::Error> for AbiNameError {
    fn from(error: fmt::Error) -> Self {
        Self::Write(error)
    }
}

impl fmt::Display for AbiNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Write(_) => write!(f, "writing the ABI name failed"),
            Self::NoAbiName(type_id) => write!(f, "type {type_id:?} has no ABI name"),
            Self::Unresolved(node_id) => write!(f, "node {node_id:?} is unresolved"),
        }
    }
}

impl std::error::Error for AbiNameError {}

impl SemanticContext {
    /// Qualifies a nested definition with its enclosing scope, as solc's
    /// `canonicalName` does: `L.S`, `C.E`.
    pub fn definition_canonical_name(&self, definition_id: NodeId) -> String {
        let mut name = String::new();
        self.write_definition_canonical_name(definition_id, &mut name)
            .expect("writing to a String never fails");
        name
    }

    pub fn write_definition_canonical_name(
        &self,
        definition_id: NodeId,
        out: &mut impl fmt::Write,
    ) -> fmt::Result {
        if let Some(enclosing) = self.binder.enclosing_definition_node_id(definition_id) {
            self.write_definition_canonical_name(enclosing, out)?;
            out.write_char('.')?;
        }
        out.write_str(
            self.binder
                .find_definition_by_id(definition_id)
                .unwrap()
                .identifier()
                .unparse(),
        )
    }

    pub fn type_internal_name(&self, type_id: TypeId) -> String {
        let mut name = String::new();
        self.write_type_internal_name(type_id, &mut name)
            .expect("writing to a String never fails");
        name
    }

    pub fn write_type_internal_name(
        &self,
        type_id: TypeId,
        out: &mut impl fmt::Write,
    ) -> fmt::Result {
        match self.types.get_type_by_id(type_id) {
            Type::Address(_) => out.write_str("address"),
            Type::Array(ArrayType { element_type, .. }) => {
                self.write_type_internal_name(*element_type, out)?;
                out.write_str("[]")
            }
            Type::ArraySlice(ArraySliceType { array_type_id }) => {
                self.write_type_internal_name(*array_type_id, out)?;
                out.write_str(" slice")
            }
            Type::Boolean => out.write_str("bool"),
            Type::ByteArray(ByteArrayType { width }) => write!(out, "bytes{width}"),
            Type::Bytes(_) => out.write_str("bytes"),
            Type::FixedPointNumber(FixedPointNumberType {
                is_signed,
                bits,
                decimal_places,
            }) => {
                let prefix = if *is_signed { "fixed" } else { "ufixed" };
                write!(out, "{prefix}{bits}x{decimal_places}")
            }
            Type::FixedSizeArray(FixedSizeArrayType {
                element_type, size, ..
            }) => {
                self.write_type_internal_name(*element_type, out)?;
                write!(out, "[{size}]")
            }
            Type::Function(_) => out.write_str("function"),
            Type::Integer(IntegerType { is_signed, bits }) => {
                let prefix = if *is_signed { "int" } else { "uint" };
                write!(out, "{prefix}{bits}")
            }
            Type::Literal(_) => out.write_str("literal"),
            Type::Mapping(MappingType {
                key_type_id,
                value_type_id,
            }) => {
                out.write_str("mapping(")?;
                self.write_type_internal_name(*key_type_id, out)?;
                out.write_str(" => ")?;
                self.write_type_internal_name(*value_type_id, out)?;
                out.write_char(')')
            }
            Type::String(_) => out.write_str("string"),
            Type::Tuple(TupleType { types }) => {
                out.write_char('(')?;
                for (index, type_id) in types.iter().enumerate() {
                    if index > 0 {
                        out.write_char(',')?;
                    }
                    self.write_type_internal_name(*type_id, out)?;
                }
                out.write_char(')')
            }
            Type::Contract(ContractType { definition_id })
            | Type::Enum(EnumType { definition_id })
            | Type::Interface(InterfaceType { definition_id })
            | Type::Library(LibraryType { definition_id })
            | Type::Struct(StructType { definition_id, .. })
            | Type::UserDefinedValue(UserDefinedValueType { definition_id }) => {
                self.write_definition_canonical_name(*definition_id, out)
            }
            Type::Error(ErrorType { definition_id }) => {
                out.write_str("error(")?;
                self.write_definition_canonical_name(*definition_id, out)?;
                out.write_char(')')
            }
            Type::Event(EventType { definition_id }) => {
                out.write_str("event(")?;
                self.write_definition_canonical_name(*definition_id, out)?;
                out.write_char(')')
            }
            // Meta-types print in solc's `type(T)` notation.
            Type::MetaType(MetaType { type_id }) => {
                out.write_str("type(")?;
                self.write_type_internal_name(*type_id, out)?;
                out.write_char(')')
            }
            Type::UserMetaType(UserMetaType { definition_id }) => {
                out.write_str("type(")?;
                self.write_definition_canonical_name(*definition_id, out)?;
                out.write_char(')')
            }
        }
    }

    /// The type as solc's `Type::toString(true)` spells it, which is the JSON ABI `internalType`
    /// and the storage layout label: a kind prefix on user-defined types (`struct C.S`,
    /// `enum C.E`, `contract I`), `address payable`, and function types with their parameters,
    /// mutability, `external` and returns; never a data location. Everything else spells as
    /// [`Self::type_internal_name`].
    pub fn type_abi_internal_name(&self, type_id: TypeId) -> String {
        // Sized on the ABI benchmarks: most names fit without the buffer regrowing.
        let mut name = String::with_capacity(32);
        self.write_type_abi_internal_name(type_id, &mut name)
            .expect("writing to a String never fails");
        name
    }

    /// Writes the type as [`Self::type_abi_internal_name`] spells it.
    pub fn write_type_abi_internal_name(
        &self,
        type_id: TypeId,
        out: &mut impl fmt::Write,
    ) -> fmt::Result {
        match self.types.get_type_by_id(type_id) {
            Type::Address(address) if address.is_payable => out.write_str("address payable"),
            Type::Array(ArrayType { element_type, .. }) => {
                self.write_type_abi_internal_name(*element_type, out)?;
                out.write_str("[]")
            }
            Type::Contract(ContractType { definition_id })
            | Type::Interface(InterfaceType { definition_id }) => {
                out.write_str("contract ")?;
                self.write_definition_canonical_name(*definition_id, out)
            }
            Type::Enum(EnumType { definition_id }) => {
                out.write_str("enum ")?;
                self.write_definition_canonical_name(*definition_id, out)
            }
            Type::FixedSizeArray(FixedSizeArrayType {
                element_type, size, ..
            }) => {
                self.write_type_abi_internal_name(*element_type, out)?;
                write!(out, "[{size}]")
            }
            Type::Function(function_type) => {
                self.write_function_type_abi_internal_name(function_type, out)
            }
            Type::Mapping(MappingType {
                key_type_id,
                value_type_id,
            }) => {
                out.write_str("mapping(")?;
                self.write_type_abi_internal_name(*key_type_id, out)?;
                out.write_str(" => ")?;
                self.write_type_abi_internal_name(*value_type_id, out)?;
                out.write_char(')')
            }
            Type::Struct(StructType { definition_id, .. }) => {
                out.write_str("struct ")?;
                self.write_definition_canonical_name(*definition_id, out)
            }
            _ => self.write_type_internal_name(type_id, out),
        }
    }

    fn write_type_abi_internal_names(
        &self,
        type_ids: &[TypeId],
        out: &mut impl fmt::Write,
    ) -> fmt::Result {
        for (index, type_id) in type_ids.iter().enumerate() {
            if index > 0 {
                out.write_char(',')?;
            }
            self.write_type_abi_internal_name(*type_id, out)?;
        }
        Ok(())
    }

    /// `function (T1,T2) [pure|view|payable] [external] [returns (R1,R2)]`: `nonpayable` and
    /// `internal` are implied by their absence, as in solc.
    fn write_function_type_abi_internal_name(
        &self,
        function_type: &FunctionType,
        out: &mut impl fmt::Write,
    ) -> fmt::Result {
        out.write_str("function (")?;
        self.write_type_abi_internal_names(&function_type.parameter_types, out)?;
        out.write_char(')')?;
        out.write_str(match function_type.mutability {
            FunctionTypeMutability::Pure => " pure",
            FunctionTypeMutability::View => " view",
            FunctionTypeMutability::Payable => " payable",
            FunctionTypeMutability::NonPayable => "",
        })?;
        if function_type.is_externally_visible() {
            out.write_str(" external")?;
        }
        if let Type::Tuple(TupleType { types }) =
            self.types.get_type_by_id(function_type.return_type)
        {
            // An empty tuple is not printed
            if !types.is_empty() {
                out.write_str(" returns (")?;
                self.write_type_abi_internal_names(types, out)?;
                out.write_char(')')?;
            }
        } else {
            out.write_str(" returns (")?;
            self.write_type_abi_internal_name(function_type.return_type, out)?;
            out.write_char(')')?;
        }
        Ok(())
    }

    /// Whether the type has an ABI representation, i.e. [`Self::write_type_abi_name`] can write
    /// it in every spelling: a struct only when all its members do.
    pub fn has_abi_type(&self, type_id: TypeId) -> bool {
        self.has_abi_type_impl(type_id, &mut Set::default())
    }

    fn has_abi_type_impl(&self, type_id: TypeId, visited_structs: &mut Set<NodeId>) -> bool {
        match self.types.get_type_by_id(type_id) {
            Type::Address(_)
            | Type::Boolean
            | Type::ByteArray(_)
            | Type::Bytes(_)
            | Type::Contract(_)
            | Type::Enum(_)
            | Type::FixedPointNumber(_)
            | Type::Function(_)
            | Type::Integer(_)
            | Type::Interface(_)
            | Type::String(_) => true,
            Type::Array(ArrayType { element_type, .. })
            | Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
                self.has_abi_type_impl(*element_type, visited_structs)
            }
            Type::ArraySlice(ArraySliceType { array_type_id }) => {
                self.has_abi_type_impl(*array_type_id, visited_structs)
            }
            Type::UserDefinedValue(UserDefinedValueType { definition_id }) => self
                .user_defined_value_target_type_id(*definition_id)
                .is_some_and(|target_type_id| {
                    self.has_abi_type_impl(target_type_id, visited_structs)
                }),
            Type::Struct(StructType { definition_id, .. }) => {
                // A recursive struct has no ABI representation.
                if !visited_structs.insert(*definition_id) {
                    return false;
                }
                let Some(Definition::Struct(struct_definition)) =
                    self.binder.find_definition_by_id(*definition_id)
                else {
                    return false;
                };
                let has_abi_type = struct_definition.ir_node.members.iter().all(|member| {
                    self.binder
                        .node_typing(member.id())
                        .as_type_id()
                        .is_some_and(|member_type_id| {
                            self.has_abi_type_impl(member_type_id, visited_structs)
                        })
                });
                visited_structs.remove(definition_id);
                has_abi_type
            }
            Type::Error(_)
            | Type::Event(_)
            | Type::Library(_)
            | Type::Literal(_)
            | Type::Mapping(_)
            | Type::MetaType(_)
            | Type::Tuple(_)
            | Type::UserMetaType(_) => false,
        }
    }

    /// Writes the type as `spelling` spells it in an ABI signature.
    pub fn write_type_abi_name(
        &self,
        type_id: TypeId,
        spelling: AbiTypeSpelling,
        out: &mut impl fmt::Write,
    ) -> Result<(), AbiNameError> {
        self.write_type_abi_name_impl(type_id, spelling, out, &mut Set::default())?;
        if matches!(spelling, AbiTypeSpelling::LibrarySelector)
            && self.types.get_type_by_id(type_id).data_location() == Some(DataLocation::Storage)
        {
            out.write_str(" storage")?;
        }
        Ok(())
    }

    fn write_type_abi_name_impl(
        &self,
        type_id: TypeId,
        spelling: AbiTypeSpelling,
        out: &mut impl fmt::Write,
        visited_structs: &mut Set<NodeId>,
    ) -> Result<(), AbiNameError> {
        let names_user_types = matches!(
            spelling,
            AbiTypeSpelling::LibrarySelector | AbiTypeSpelling::LibraryJson
        );
        let names_any_type = matches!(spelling, AbiTypeSpelling::LibrarySelector);
        let is_json = matches!(
            spelling,
            AbiTypeSpelling::Json | AbiTypeSpelling::LibraryJson
        );
        match self.types.get_type_by_id(type_id) {
            Type::Array(ArrayType { element_type, .. }) => {
                self.write_type_abi_name_impl(*element_type, spelling, out, visited_structs)?;
                out.write_str("[]")?;
            }
            Type::FixedSizeArray(FixedSizeArrayType {
                element_type, size, ..
            }) => {
                self.write_type_abi_name_impl(*element_type, spelling, out, visited_structs)?;
                write!(out, "[{size}]")?;
            }
            // A user-defined value type is spelled as the type it wraps, which is always elementary.
            Type::UserDefinedValue(UserDefinedValueType { definition_id }) => {
                let target_type_id = self
                    .user_defined_value_target_type_id(*definition_id)
                    .ok_or(AbiNameError::Unresolved(*definition_id))?;
                self.write_type_abi_name_impl(target_type_id, spelling, out, visited_structs)?;
            }
            Type::Contract(ContractType { definition_id })
            | Type::Enum(EnumType { definition_id })
            | Type::Interface(InterfaceType { definition_id })
                if names_user_types =>
            {
                self.write_definition_canonical_name(*definition_id, out)?;
            }
            Type::Struct(StructType { definition_id, .. }) if names_any_type => {
                self.write_definition_canonical_name(*definition_id, out)?;
            }
            Type::Struct(_) if is_json => out.write_str("tuple")?,
            Type::Address(_) | Type::Contract(_) | Type::Interface(_) => {
                out.write_str("address")?;
            }
            Type::Boolean => out.write_str("bool")?,
            Type::ByteArray(ByteArrayType { width }) => write!(out, "bytes{width}")?,
            Type::Bytes(_) => out.write_str("bytes")?,
            Type::Enum(_) => out.write_str("uint8")?,
            Type::FixedPointNumber(FixedPointNumberType {
                is_signed,
                bits,
                decimal_places,
            }) => {
                let prefix = if *is_signed { "fixed" } else { "ufixed" };
                write!(out, "{prefix}{bits}x{decimal_places}")?;
            }
            // Every function type is spelled `function`, although only an external one encodes.
            Type::Function(_) => out.write_str("function")?,
            Type::Integer(IntegerType { is_signed, bits }) => {
                let prefix = if *is_signed { "int" } else { "uint" };
                write!(out, "{prefix}{bits}")?;
            }
            Type::String(_) => out.write_str("string")?,
            // A slice encodes exactly like the array it slices.
            Type::ArraySlice(ArraySliceType { array_type_id }) if !names_any_type => {
                self.write_type_abi_name_impl(*array_type_id, spelling, out, visited_structs)?;
            }
            Type::Struct(StructType { definition_id, .. }) => {
                self.write_struct_abi_tuple(
                    type_id,
                    *definition_id,
                    spelling,
                    out,
                    visited_structs,
                )?;
            }
            // Only a library function's signature names these, by their internal name: a mapping
            // keeps a user-defined value type by name.
            Type::ArraySlice(_)
            | Type::Error(_)
            | Type::Event(_)
            | Type::Library(_)
            | Type::Literal(_)
            | Type::Mapping(_)
            | Type::MetaType(_)
            | Type::Tuple(_)
            | Type::UserMetaType(_) => {
                if !names_any_type {
                    return Err(AbiNameError::NoAbiName(type_id));
                }
                self.write_type_internal_name(type_id, out)?;
            }
        }
        Ok(())
    }

    /// Writes a struct as the tuple `(T1,T2)` of its members' types.
    fn write_struct_abi_tuple(
        &self,
        type_id: TypeId,
        definition_id: NodeId,
        spelling: AbiTypeSpelling,
        out: &mut impl fmt::Write,
        visited_structs: &mut Set<NodeId>,
    ) -> Result<(), AbiNameError> {
        // Recursive structs are not valid Solidity, but guard against cycles to avoid unbounded
        // recursion if malformed types reach this point.
        if !visited_structs.insert(definition_id) {
            return Err(AbiNameError::NoAbiName(type_id));
        }
        let Some(Definition::Struct(struct_definition)) =
            self.binder.find_definition_by_id(definition_id)
        else {
            return Err(AbiNameError::Unresolved(definition_id));
        };
        out.write_char('(')?;
        for (index, member) in struct_definition.ir_node.members.iter().enumerate() {
            if index > 0 {
                out.write_char(',')?;
            }
            let member_type_id = self
                .binder
                .node_typing(member.id())
                .as_type_id()
                .ok_or(AbiNameError::Unresolved(member.id()))?;
            self.write_type_abi_name_impl(member_type_id, spelling, out, visited_structs)?;
        }
        visited_structs.remove(&definition_id);
        out.write_char(')')?;
        Ok(())
    }

    pub(super) fn user_defined_value_target_type_id(
        &self,
        definition_id: NodeId,
    ) -> Option<TypeId> {
        let Definition::UserDefinedValueType(user_defined_value) =
            self.binder.find_definition_by_id(definition_id)?
        else {
            return None;
        };
        user_defined_value.target_type_id
    }
}
