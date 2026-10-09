//! The identifiers a storage layout keys its type table with, e.g. `t_uint256`,
//! `t_mapping(t_address,t_uint256)` or `t_struct(S)12_storage`.

use std::fmt::Write;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{
    AddressType, ArrayType, ByteArrayType, BytesType, ContractType, DataLocation, EnumType,
    FixedPointNumberType, FixedSizeArrayType, FunctionType, FunctionTypeMutability, IntegerType,
    InterfaceType, MappingType, StringType, StructType, TupleType, Type, TypeId,
    UserDefinedValueType,
};

/// The identifier of `type_id` in a storage layout. A user-defined type is
/// named after its definition and ends with the definition's node id.
pub(crate) fn storage_type_identifier(semantic: &SemanticContext, type_id: TypeId) -> String {
    let mut identifier = String::with_capacity(32);
    write_identifier(semantic, type_id, false, &mut identifier);
    identifier
}

/// Writes the identifier of `type_id`. A reference type is a pointer when it is
/// not in storage, or when it is a parameter or return type of a function type
/// (`is_pointer`).
fn write_identifier(
    semantic: &SemanticContext,
    type_id: TypeId,
    is_pointer: bool,
    out: &mut String,
) {
    match semantic.types().get_type_by_id(type_id) {
        Type::Address(AddressType { is_payable }) => {
            out.push_str(if *is_payable {
                "t_address_payable"
            } else {
                "t_address"
            });
        }
        Type::Array(ArrayType {
            element_type,
            location,
        }) => {
            out.push_str("t_array(");
            write_identifier(semantic, *element_type, false, out);
            out.push_str(")dyn");
            write_location(*location, is_pointer, out);
        }
        Type::Boolean => out.push_str("t_bool"),
        Type::ByteArray(ByteArrayType { width }) => write!(out, "t_bytes{width}").unwrap(),
        Type::Bytes(BytesType { location }) => {
            out.push_str("t_bytes");
            write_location(*location, is_pointer, out);
        }
        Type::Contract(ContractType { definition_id })
        | Type::Interface(InterfaceType { definition_id }) => {
            write_user_defined(semantic, "t_contract", *definition_id, out);
        }
        Type::Enum(EnumType { definition_id }) => {
            write_user_defined(semantic, "t_enum", *definition_id, out);
        }
        Type::FixedPointNumber(FixedPointNumberType {
            is_signed,
            bits,
            decimal_places,
        }) => {
            let sign = if *is_signed { "" } else { "u" };
            write!(out, "t_{sign}fixed{bits}x{decimal_places}").unwrap();
        }
        Type::FixedSizeArray(FixedSizeArrayType {
            element_type,
            location,
            size,
        }) => {
            out.push_str("t_array(");
            write_identifier(semantic, *element_type, false, out);
            write!(out, "){size}").unwrap();
            write_location(*location, is_pointer, out);
        }
        Type::Function(function_type) => write_function(semantic, function_type, out),
        Type::Integer(IntegerType { is_signed, bits }) => {
            let sign = if *is_signed { "" } else { "u" };
            write!(out, "t_{sign}int{bits}").unwrap();
        }
        Type::Mapping(MappingType {
            key_type_id,
            value_type_id,
        }) => {
            out.push_str("t_mapping(");
            write_identifier(semantic, *key_type_id, false, out);
            out.push(',');
            write_identifier(semantic, *value_type_id, false, out);
            out.push(')');
        }
        Type::String(StringType { location }) => {
            out.push_str("t_string");
            write_location(*location, is_pointer, out);
        }
        Type::Struct(StructType {
            definition_id,
            location,
        }) => {
            write_user_defined(semantic, "t_struct", *definition_id, out);
            write_location(*location, is_pointer, out);
        }
        Type::UserDefinedValue(UserDefinedValueType { definition_id }) => {
            write_user_defined(semantic, "t_userDefinedValueType", *definition_id, out);
        }

        // None of these can be stored, nor be a parameter of a stored function
        // type. Name them by their internal name if one reaches a layout.
        Type::ArraySlice(_)
        | Type::Error(_)
        | Type::Event(_)
        | Type::Library(_)
        | Type::Literal(_)
        | Type::MetaType(_)
        | Type::Tuple(_)
        | Type::UserMetaType(_) => {
            out.push_str("t_");
            out.push_str(&semantic.type_internal_name(type_id));
        }
    }
}

/// `{prefix}(Name){node_id}`.
fn write_user_defined(
    semantic: &SemanticContext,
    prefix: &str,
    definition_id: NodeId,
    out: &mut String,
) {
    let name = semantic
        .binder()
        .find_definition_by_id(definition_id)
        .map_or("", |definition| definition.identifier().unparse());
    write!(out, "{prefix}({name}){}", u64::from(definition_id)).unwrap();
}

/// `_storage` for a type in storage, or `_memory_ptr` and `_calldata_ptr`
/// for one that points to memory or calldata.
fn write_location(location: DataLocation, is_pointer: bool, out: &mut String) {
    match location {
        // Only a struct member has an inherited location, and it is stored
        // with its struct.
        DataLocation::Storage | DataLocation::Inherited => {
            out.push_str(if is_pointer {
                "_storage_ptr"
            } else {
                "_storage"
            });
        }
        DataLocation::Memory => out.push_str("_memory_ptr"),
        DataLocation::Calldata => out.push_str("_calldata_ptr"),
    }
}

/// `t_function_{internal|external}_{mutability}(T1,T2)returns(R1,R2)`.
fn write_function(semantic: &SemanticContext, function_type: &FunctionType, out: &mut String) {
    let visibility = if function_type.is_externally_visible() {
        "external"
    } else {
        "internal"
    };
    let mutability = match function_type.mutability {
        FunctionTypeMutability::Pure => "pure",
        FunctionTypeMutability::View => "view",
        FunctionTypeMutability::NonPayable => "nonpayable",
        FunctionTypeMutability::Payable => "payable",
    };
    write!(out, "t_function_{visibility}_{mutability}(").unwrap();
    write_identifiers(semantic, &function_type.parameter_types, out);
    out.push_str(")returns(");
    match semantic.types().get_type_by_id(function_type.return_type) {
        Type::Tuple(TupleType { types }) => write_identifiers(semantic, types, out),
        _ => write_identifier(semantic, function_type.return_type, true, out),
    }
    out.push(')');
}

fn write_identifiers(semantic: &SemanticContext, type_ids: &[TypeId], out: &mut String) {
    for (index, type_id) in type_ids.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        write_identifier(semantic, *type_id, true, out);
    }
}
