use semver::Version;

use super::binder::{Binder, Definition, Typing};
use super::types::{Type, TypeId, TypeRegistry};
use crate::cst::NodeId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltIn {
    ArrayPop,
    ArrayPush(TypeId),
    Assert,
    Balance,
    Blockhash,
    CallOptionValue,
    CallOptionGas,
    ErrorOrPanic,
    Length,
    ModifierUnderscore,
    Msg,
    MsgData,
    MsgGas,
    MsgSender,
    MsgSig,
    MsgValue,
    Require,
    Selector,
    Tx,
    TxGasPrice,
    TxOrigin,
    Unwrap(NodeId),
    Wrap(NodeId),

    YulAddress,
    YulKeccak256,
    YulOffset,
    YulLength,
    YulSelector,
    YulSlot,
    // TODO: complete this list
}

const VERSION_0_4_12: Version = Version::new(0, 4, 12);
const VERSION_0_4_22: Version = Version::new(0, 4, 22);
const VERSION_0_5_0: Version = Version::new(0, 5, 0);
const VERSION_0_8_8: Version = Version::new(0, 8, 8);

pub(crate) struct BuiltInsResolver<'a> {
    language_version: Version,
    binder: &'a Binder,
    types: &'a TypeRegistry,
}

impl<'a> BuiltInsResolver<'a> {
    pub(crate) fn new(
        language_version: Version,
        binder: &'a Binder,
        types: &'a TypeRegistry,
    ) -> Self {
        Self {
            language_version,
            binder,
            types,
        }
    }

    pub(crate) fn lookup_global(&self, symbol: &str) -> Option<BuiltIn> {
        match symbol {
            "assert" => Some(BuiltIn::Assert),
            "blockhash" => {
                if self.language_version >= VERSION_0_4_22 {
                    Some(BuiltIn::Blockhash)
                } else {
                    None
                }
            }
            "msg" => Some(BuiltIn::Msg),
            "require" => Some(BuiltIn::Require),
            "tx" => Some(BuiltIn::Tx),
            // TODO: add the rest of the built-ins
            _ => None,
        }
    }

    pub(crate) fn lookup_yul_global(&self, symbol: &str) -> Option<BuiltIn> {
        match symbol {
            "keccak256" => {
                if self.language_version >= VERSION_0_4_12 {
                    Some(BuiltIn::YulKeccak256)
                } else {
                    None
                }
            }
            // TODO: add the rest of the built-ins
            _ => None,
        }
    }

    pub(crate) fn lookup_member_of(&self, built_in: &BuiltIn, symbol: &str) -> Option<BuiltIn> {
        match built_in {
            BuiltIn::Tx => match symbol {
                "gasprice" => Some(BuiltIn::TxGasPrice),
                "origin" => Some(BuiltIn::TxOrigin),
                _ => None,
            },
            BuiltIn::Msg => match symbol {
                "data" => Some(BuiltIn::MsgData),
                "gas" => {
                    if self.language_version < VERSION_0_5_0 {
                        Some(BuiltIn::MsgGas)
                    } else {
                        None
                    }
                }
                "sender" => Some(BuiltIn::MsgSender),
                "sig" => Some(BuiltIn::MsgSig),
                "value" => Some(BuiltIn::MsgValue),
                _ => None,
            },
            _ => None,
        }
    }

    pub(crate) fn lookup_member_of_meta_type(
        &self,
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
            Definition::UserDefinedValueType(udvt) => {
                if self.language_version >= VERSION_0_8_8 {
                    match symbol {
                        "wrap" => Some(BuiltIn::Wrap(udvt.node_id)),
                        "unwrap" => Some(BuiltIn::Unwrap(udvt.node_id)),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub(crate) fn lookup_member_of_type(
        &self,
        parent_type: &Type,
        symbol: &str,
    ) -> Option<BuiltIn> {
        match parent_type {
            Type::Address { .. } => match symbol {
                "balance" => Some(BuiltIn::Balance),
                // TODO: add the rest of the address (payable) built-ins
                _ => None,
            },
            Type::Array { element_type, .. } => match symbol {
                "length" => Some(BuiltIn::Length),
                "pop" => Some(BuiltIn::ArrayPop),
                "push" => Some(BuiltIn::ArrayPush(*element_type)),
                _ => None,
            },
            Type::Boolean => None,
            Type::ByteArray { .. } => match symbol {
                "length" => Some(BuiltIn::Length),
                _ => None,
            },
            Type::Bytes { .. } => match symbol {
                "pop" => Some(BuiltIn::ArrayPop),
                "push" => Some(BuiltIn::ArrayPush(self.types.byte())),
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

    #[allow(clippy::unused_self)]
    pub(crate) fn lookup_yul_suffix(
        &self,
        definition: &Definition,
        symbol: &str,
    ) -> Option<BuiltIn> {
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

    #[allow(clippy::unused_self)]
    pub(crate) fn lookup_call_option(&self, symbol: &str) -> Option<BuiltIn> {
        match symbol {
            "value" => Some(BuiltIn::CallOptionValue),
            "gas" => Some(BuiltIn::CallOptionGas),
            _ => None,
        }
    }

    pub(crate) fn typing_of(&self, built_in: &BuiltIn) -> Typing {
        match built_in {
            // variables and members
            BuiltIn::Balance => Typing::Resolved(self.types.uint256()),
            BuiltIn::Length => Typing::Resolved(self.types.uint256()),
            BuiltIn::MsgGas => Typing::Resolved(self.types.uint256()),
            BuiltIn::MsgData => Typing::Resolved(self.types.bytes()),
            BuiltIn::MsgSender => Typing::Resolved(self.types.address()),
            BuiltIn::MsgSig => Typing::Resolved(self.types.bytes4()),
            BuiltIn::MsgValue => Typing::Resolved(self.types.uint256()),
            BuiltIn::Selector => Typing::Resolved(self.types.bytes4()),
            BuiltIn::TxGasPrice => Typing::Resolved(self.types.uint256()),
            BuiltIn::TxOrigin => Typing::Resolved(self.types.address()),

            // built-in functions
            BuiltIn::ArrayPop
            | BuiltIn::ArrayPush(_)
            | BuiltIn::Assert
            | BuiltIn::Blockhash
            | BuiltIn::ModifierUnderscore
            | BuiltIn::Msg
            | BuiltIn::Require
            | BuiltIn::Tx
            | BuiltIn::Wrap(_)
            | BuiltIn::Unwrap(_) => Typing::BuiltIn(*built_in),

            // others
            _ => Typing::Unresolved,
        }
    }

    pub(crate) fn typing_of_function_call(
        &self,
        built_in: &BuiltIn,
        argument_typings: &[Typing],
    ) -> Typing {
        match built_in {
            BuiltIn::ArrayPush(type_id) => {
                if argument_typings.is_empty() {
                    Typing::Resolved(*type_id)
                } else {
                    Typing::Resolved(self.types.void())
                }
            }
            BuiltIn::ArrayPop => Typing::Resolved(self.types.void()),
            BuiltIn::Assert | BuiltIn::Require => Typing::Resolved(self.types.void()),
            BuiltIn::Blockhash => Typing::Resolved(self.types.bytes32()),
            BuiltIn::Unwrap(definition_id) => {
                let Some(Definition::UserDefinedValueType(udvt)) =
                    self.binder.find_definition_by_id(*definition_id)
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
                    self.binder.find_definition_by_id(*definition_id)
                else {
                    unreachable!("definition bound to wrap built-in is not a UDVT");
                };
                Typing::Resolved(
                    self.types
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
}
