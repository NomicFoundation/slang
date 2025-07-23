use semver::Version;

use super::binder::{Binder, Definition, Typing};
use super::types::{Type, TypeId, TypeRegistry};
use crate::cst::NodeId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltIn {
    Addmod,
    ArrayPop,
    ArrayPush(TypeId),
    Assert,
    Balance,
    Blobhash,
    Blockhash,
    CallOptionValue,
    CallOptionGas,
    Ecrecover,
    ErrorOrPanic,
    Gasleft,
    Keccak256,
    Length,
    Log(u8),
    ModifierUnderscore,
    Msg,
    MsgData,
    MsgGas,
    MsgSender,
    MsgSig,
    MsgValue,
    Mulmod,
    Require,
    Revert,
    Ripemd160,
    Selector,
    Selfdestruct,
    Sha256,
    Sha3,
    Suicide,
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
const VERSION_0_8_0: Version = Version::new(0, 8, 0);
const VERSION_0_8_8: Version = Version::new(0, 8, 8);
const VERSION_0_8_24: Version = Version::new(0, 8, 24);

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
            "addmod" => Some(BuiltIn::Addmod),
            "assert" => Some(BuiltIn::Assert),
            "blobhash" => {
                if self.language_version >= VERSION_0_8_24 {
                    Some(BuiltIn::Blobhash)
                } else {
                    None
                }
            }
            "blockhash" => {
                if self.language_version >= VERSION_0_4_22 {
                    Some(BuiltIn::Blockhash)
                } else {
                    None
                }
            }
            "ecrecover" => Some(BuiltIn::Ecrecover),
            "gasleft" => Some(BuiltIn::Gasleft),
            "keccak256" => Some(BuiltIn::Keccak256),
            "log0" | "log1" | "log2" | "log3" | "log4" => {
                if self.language_version < VERSION_0_8_0 {
                    let arity = symbol.as_bytes()[3] - b'0';
                    Some(BuiltIn::Log(arity))
                } else {
                    None
                }
            }
            "msg" => Some(BuiltIn::Msg),
            "mulmod" => Some(BuiltIn::Mulmod),
            "require" => Some(BuiltIn::Require),
            "revert" => Some(BuiltIn::Revert),
            "ripemd160" => Some(BuiltIn::Ripemd160),
            "selfdestruct" => Some(BuiltIn::Selfdestruct),
            "sha256" => Some(BuiltIn::Sha256),
            "sha3" => {
                if self.language_version < VERSION_0_5_0 {
                    Some(BuiltIn::Sha3)
                } else {
                    None
                }
            }
            "suicide" => {
                if self.language_version < VERSION_0_5_0 {
                    Some(BuiltIn::Suicide)
                } else {
                    None
                }
            }
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
            BuiltIn::Addmod
            | BuiltIn::ArrayPop
            | BuiltIn::ArrayPush(_)
            | BuiltIn::Assert
            | BuiltIn::Blobhash
            | BuiltIn::Blockhash
            | BuiltIn::Ecrecover
            | BuiltIn::Gasleft
            | BuiltIn::Keccak256
            | BuiltIn::Log(_)
            | BuiltIn::ModifierUnderscore
            | BuiltIn::Mulmod
            | BuiltIn::Msg
            | BuiltIn::Require
            | BuiltIn::Revert
            | BuiltIn::Ripemd160
            | BuiltIn::Selfdestruct
            | BuiltIn::Sha256
            | BuiltIn::Sha3
            | BuiltIn::Suicide
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
            BuiltIn::Addmod => Typing::Resolved(self.types.uint256()),
            BuiltIn::ArrayPush(type_id) => {
                if argument_typings.is_empty() {
                    Typing::Resolved(*type_id)
                } else {
                    Typing::Resolved(self.types.void())
                }
            }
            BuiltIn::ArrayPop => Typing::Resolved(self.types.void()),
            BuiltIn::Assert => Typing::Resolved(self.types.void()),
            BuiltIn::Blobhash => Typing::Resolved(self.types.bytes32()),
            BuiltIn::Blockhash => Typing::Resolved(self.types.bytes32()),
            BuiltIn::Ecrecover => Typing::Resolved(self.types.address()),
            BuiltIn::Gasleft => Typing::Resolved(self.types.uint256()),
            BuiltIn::Keccak256 => Typing::Resolved(self.types.bytes32()),
            BuiltIn::Log(_) => Typing::Resolved(self.types.void()),
            BuiltIn::Mulmod => Typing::Resolved(self.types.uint256()),
            BuiltIn::Require => Typing::Resolved(self.types.void()),
            BuiltIn::Revert => Typing::Resolved(self.types.void()),
            BuiltIn::Ripemd160 => Typing::Resolved(self.types.bytes20()),
            BuiltIn::Selfdestruct => Typing::Resolved(self.types.void()),
            BuiltIn::Sha256 => Typing::Resolved(self.types.bytes32()),
            BuiltIn::Sha3 => Typing::Resolved(self.types.bytes32()),
            BuiltIn::Suicide => Typing::Resolved(self.types.void()),
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
