use slang_solidity_v2_common::nodes::NodeId;

use super::{Type, TypeId, TypeRegistry};
use crate::binder::{Binder, Definition, Typing};
use crate::types::{DataLocation, FunctionType, LiteralKind};

/// Returns a human-readable name for a [`Typing`], used in diagnostic messages
/// that need to describe operand types that may not be a fully resolved
/// [`Type`] (e.g. `super`, event/error/struct references, meta-types).
pub(crate) fn typing_display_name(
    types: &TypeRegistry,
    binder: &Binder,
    typing: &Typing,
) -> String {
    match typing {
        Typing::Resolved(type_id) => type_display(types, binder, *type_id),
        Typing::This => "this".to_string(),
        Typing::Super => "super".to_string(),
        Typing::UserMetaType(definition_id) => match binder.find_definition_by_id(*definition_id) {
            Some(Definition::Event(_)) => {
                format!(
                    "event {}",
                    definition_canonical_name(binder, *definition_id)
                )
            }
            Some(Definition::Error(_)) => {
                format!(
                    "error {}",
                    definition_canonical_name(binder, *definition_id)
                )
            }
            Some(Definition::Contract(_)) => format!(
                "contract {}",
                definition_canonical_name(binder, *definition_id)
            ),
            Some(Definition::Interface(_)) => format!(
                "interface {}",
                definition_canonical_name(binder, *definition_id)
            ),
            Some(Definition::Library(_)) => format!(
                "library {}",
                definition_canonical_name(binder, *definition_id)
            ),
            Some(Definition::Struct(_)) => format!(
                "struct {}",
                definition_canonical_name(binder, *definition_id)
            ),
            Some(Definition::Enum(_)) => {
                format!("enum {}", definition_canonical_name(binder, *definition_id))
            }
            Some(Definition::UserDefinedValueType(_)) => {
                definition_canonical_name(binder, *definition_id)
            }
            _ => unreachable!("unexpected user meta type"),
        },
        Typing::MetaType(meta_type) => {
            format!("type({})", type_display_impl(types, binder, meta_type))
        }
        _ => "<unresolved>".to_string(),
    }
}

pub(crate) fn type_display(types: &TypeRegistry, binder: &Binder, type_id: TypeId) -> String {
    type_display_impl(types, binder, types.get_type_by_id(type_id))
}

#[allow(clippy::too_many_lines)]
fn type_display_impl(type_registry: &TypeRegistry, binder: &Binder, type_: &Type) -> String {
    match type_ {
        Type::Address { payable } => {
            if *payable {
                "address payable".to_string()
            } else {
                "address".to_string()
            }
        }
        Type::Array {
            element_type,
            location,
        } => format!(
            "{element_type}[] {location}",
            element_type = type_display(type_registry, binder, *element_type),
            location = data_location_display(*location),
        ),
        Type::Boolean => "bool".to_string(),
        Type::ByteArray { width } => format!("bytes{width}"),
        Type::Bytes { location } => {
            format!(
                "bytes {location}",
                location = data_location_display(*location)
            )
        }
        Type::FixedPointNumber {
            signed,
            bits,
            precision_bits,
        } => {
            format!(
                "{signed}fixed{bits}x{precision_bits}",
                signed = if *signed { "" } else { "u" }
            )
        }
        Type::FixedSizeArray {
            element_type,
            size,
            location,
        } => format!(
            "{element_type}[{size}] {location}",
            element_type = type_display(type_registry, binder, *element_type),
            location = data_location_display(*location),
        ),
        Type::Function(FunctionType {
            parameter_types,
            return_type,
            ..
        }) => {
            format!(
                "function ({parameters}) returns {returns}",
                parameters = parameter_types
                    .iter()
                    .map(|type_id| type_display(type_registry, binder, *type_id))
                    .collect::<Vec<_>>()
                    .join(", "),
                returns = type_display(type_registry, binder, *return_type),
            )
        }
        Type::Integer { signed, bits } => {
            format!("{signed}int{bits}", signed = if *signed { "" } else { "u" })
        }
        Type::Literal(kind) => match kind {
            LiteralKind::Integer { value } => format!("lit-integer({value})"),
            LiteralKind::HexInteger { value, bytes } => {
                format!("lit-hex({value}, {bytes})")
            }
            LiteralKind::Rational { value } => format!("lit-rational({value})"),
            LiteralKind::HexString { bytes } => format!("lit-hexstring({bytes})"),
            LiteralKind::String { bytes } => format!("lit-string({bytes})"),
            LiteralKind::Address => "lit-address".to_string(),
        },
        Type::Mapping {
            key_type_id,
            value_type_id,
        } => {
            format!(
                "{key} => {value}",
                key = type_display(type_registry, binder, *key_type_id),
                value = type_display(type_registry, binder, *value_type_id)
            )
        }
        Type::String { location } => {
            format!(
                "string {location}",
                location = data_location_display(*location)
            )
        }
        Type::Struct {
            definition_id,
            location,
        } => {
            format!(
                "{name} {location}",
                name = definition_canonical_name(binder, *definition_id),
                location = data_location_display(*location)
            )
        }
        Type::Tuple { types } => {
            format!(
                "({types})",
                types = types
                    .iter()
                    .map(|type_id| type_display(type_registry, binder, *type_id))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        Type::Contract { definition_id }
        | Type::Enum { definition_id }
        | Type::Interface { definition_id }
        | Type::UserDefinedValue { definition_id } => {
            definition_canonical_name(binder, *definition_id)
        }
        Type::Void => "void".to_string(),
    }
}

pub(crate) fn definition_canonical_name(binder: &Binder, definition_id: NodeId) -> String {
    binder
        .find_definition_by_id(definition_id)
        .unwrap()
        .identifier()
        .unparse()
        .to_string()
}

fn data_location_display(location: DataLocation) -> &'static str {
    match location {
        DataLocation::Memory => "memory",
        DataLocation::Storage => "storage",
        DataLocation::Calldata => "calldata",
        DataLocation::Inherited => "(inherited)",
    }
}
