use super::binder::{Binder, Definition, Typing};
use super::types::{Type, TypeId, TypeRegistry};
use crate::cst::NodeId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltIn {
    ArrayPop,
    ArrayPush(TypeId),
    Balance,
    CallOptionValue,
    CallOptionGas,
    ErrorOrPanic,
    Length,
    ModifierUnderscore,
    Selector,
    Tx,
    TxGasPrice,
    TxOrigin,
    Unwrap(NodeId),
    YulAddress,
    YulOffset,
    YulLength,
    YulSelector,
    YulSlot,
    Wrap(NodeId),
    // TODO: complete this list
}

pub(crate) fn lookup_global_built_in(symbol: &str) -> Option<BuiltIn> {
    match symbol {
        "tx" => Some(BuiltIn::Tx),
        // TODO: add the rest of the built-ins
        _ => None,
    }
}

pub(crate) fn lookup_member_of_built_in(built_in: &BuiltIn, symbol: &str) -> Option<BuiltIn> {
    match built_in {
        BuiltIn::Tx => match symbol {
            "gasprice" => Some(BuiltIn::TxGasPrice),
            "origin" => Some(BuiltIn::TxOrigin),
            _ => None,
        },
        _ => None,
    }
}

pub(crate) fn lookup_built_in_member_of_meta_type(
    definition: &Definition,
    symbol: &str,
) -> Option<BuiltIn> {
    match definition {
        Definition::Error(_) => match symbol {
            "selector" => Some(BuiltIn::Selector),
            _ => None,
        },
        Definition::Event(_) => match symbol {
            "selector" => Some(BuiltIn::Selector),
            _ => None,
        },
        Definition::UserDefinedValueType(udvt) => match symbol {
            "wrap" => Some(BuiltIn::Wrap(udvt.node_id)),
            "unwrap" => Some(BuiltIn::Unwrap(udvt.node_id)),
            _ => None,
        },
        _ => None,
    }
}

pub(crate) fn lookup_built_in_member_of_type(
    parent_type: &Type,
    symbol: &str,
    types: &TypeRegistry,
) -> Option<BuiltIn> {
    match parent_type {
        Type::Address { .. } => match symbol {
            "balance" => Some(BuiltIn::Balance),
            // TODO: add the rest of the address (payable) built-ins
            _ => None,
        },
        Type::Array { element_type, .. } => match symbol {
            "length" => Some(BuiltIn::Length),
            "push" => Some(BuiltIn::ArrayPush(*element_type)),
            _ => None,
        },
        Type::Boolean => None,
        Type::ByteArray { .. } => match symbol {
            "length" => Some(BuiltIn::Length),
            _ => None,
        },
        Type::Bytes { .. } => match symbol {
            "push" => Some(BuiltIn::ArrayPush(types.byte())),
            "pop" => Some(BuiltIn::ArrayPop),
            _ => None,
        },
        Type::Contract { .. } => None,
        Type::Enum { .. } => None,
        Type::FixedPointNumber { .. } => None,
        Type::Function { .. } => {
            // TODO: for external functions, expose `selector`, `address`, etc
            None
        }
        Type::Integer { .. } => None,
        Type::Interface { .. } => None,
        Type::Mapping { .. } => None,
        Type::Rational => None,
        Type::String { .. } => match symbol {
            "length" => Some(BuiltIn::Length),
            _ => None,
        },
        Type::Struct { .. } => None,
        Type::Tuple { .. } => None,
        Type::UserDefinedValue { .. } => None,
        Type::Void => None,
    }
}

pub(crate) fn lookup_yul_built_in_suffix(definition: &Definition, symbol: &str) -> Option<BuiltIn> {
    match definition {
        Definition::StateVariable(_) => match symbol {
            "offset" => Some(BuiltIn::YulOffset),
            "slot" => Some(BuiltIn::YulSlot),
            _ => None,
        },
        Definition::Parameter(_) | Definition::Variable(_) => match symbol {
            "address" => Some(BuiltIn::YulAddress),
            "length" => Some(BuiltIn::YulLength),
            "offset" => Some(BuiltIn::YulOffset),
            "selector" => Some(BuiltIn::YulSelector),
            _ => None,
        },
        _ => None,
    }
}

pub(crate) fn lookup_call_option_built_in(symbol: &str) -> Option<BuiltIn> {
    match symbol {
        "value" => Some(BuiltIn::CallOptionValue),
        "gas" => Some(BuiltIn::CallOptionGas),
        _ => None,
    }
}

pub(crate) fn typing_of_built_in(built_in: &BuiltIn, types: &TypeRegistry) -> Typing {
    match built_in {
        BuiltIn::Balance => Typing::Resolved(types.uint256()),
        BuiltIn::Length => Typing::Resolved(types.uint256()),
        BuiltIn::Selector => Typing::Resolved(types.bytes4()),
        BuiltIn::TxGasPrice => Typing::Resolved(types.uint256()),
        BuiltIn::TxOrigin => Typing::Resolved(types.address()),
        BuiltIn::ArrayPop
        | BuiltIn::ArrayPush(_)
        | BuiltIn::CallOptionGas
        | BuiltIn::CallOptionValue
        | BuiltIn::ErrorOrPanic
        | BuiltIn::ModifierUnderscore
        | BuiltIn::Tx
        | BuiltIn::Wrap(_)
        | BuiltIn::Unwrap(_) => Typing::BuiltIn(*built_in),
        BuiltIn::YulAddress
        | BuiltIn::YulLength
        | BuiltIn::YulOffset
        | BuiltIn::YulSelector
        | BuiltIn::YulSlot => Typing::Unresolved,
    }
}

pub(crate) fn typing_of_built_in_function_call(
    built_in: &BuiltIn,
    argument_typings: &[Typing],
    types: &TypeRegistry,
    binder: &Binder,
) -> Typing {
    match built_in {
        BuiltIn::ArrayPush(type_id) => {
            if argument_typings.is_empty() {
                Typing::Resolved(*type_id)
            } else {
                Typing::Resolved(types.void())
            }
        }
        BuiltIn::ArrayPop => Typing::Resolved(types.void()),
        BuiltIn::Unwrap(definition_id) => {
            let Some(Definition::UserDefinedValueType(udvt)) =
                binder.find_definition_by_id(*definition_id)
            else {
                unreachable!("definition bound to unwrap built-in is not a UDVT");
            };
            if let Some(target_type_id) = udvt.target_type_id {
                Typing::Resolved(target_type_id)
            } else {
                Typing::Unresolved
            }
        }
        BuiltIn::Wrap(definition_id) => {
            let Some(Definition::UserDefinedValueType(_)) =
                binder.find_definition_by_id(*definition_id)
            else {
                unreachable!("definition bound to wrap built-in is not a UDVT");
            };
            Typing::Resolved(
                types
                    .find_type(&Type::UserDefinedValue {
                        definition_id: *definition_id,
                    })
                    .unwrap(),
            )
        }
        _ => {
            // other built-ins cannot be called
            Typing::Unresolved
        }
    }
}
