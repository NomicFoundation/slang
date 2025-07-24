use semver::Version;

use super::binder::{Binder, Definition, Typing};
use super::types::{Type, TypeId, TypeRegistry};
use crate::cst::NodeId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltIn {
    Abi,
    AbiDecode,
    AbiEncode,
    AbiEncodeCall,
    AbiEncodePacked,
    AbiEncodeWithSelector,
    AbiEncodeWithSignature,
    Addmod,
    AddressBalance,
    AddressCall,
    AddressCallcode,
    AddressCode,
    AddressCodehash,
    AddressDelegatecall,
    AddressSend,
    AddressStaticcall,
    AddressTransfer,
    ArrayPop,
    ArrayPush(TypeId),
    Assert,
    Blobhash,
    Block,
    BlockBasefee,
    BlockBlobbasefee,
    BlockChainid,
    BlockCoinbase,
    BlockDifficulty,
    BlockGaslimit,
    BlockNumber,
    BlockPrevrandao,
    BlockTimestamp,
    Blockhash,
    BytesConcat,
    CallOptionGas,
    CallOptionSalt,
    CallOptionValue,
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
    StringConcat,
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
const VERSION_0_6_0: Version = Version::new(0, 6, 0);
const VERSION_0_8_0: Version = Version::new(0, 8, 0);
const VERSION_0_8_7: Version = Version::new(0, 8, 7);
const VERSION_0_8_8: Version = Version::new(0, 8, 8);
const VERSION_0_8_11: Version = Version::new(0, 8, 11);
const VERSION_0_8_18: Version = Version::new(0, 8, 18);
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
            "abi" => Some(BuiltIn::Abi),
            "addmod" => Some(BuiltIn::Addmod),
            "assert" => Some(BuiltIn::Assert),
            "blobhash" if self.language_version >= VERSION_0_8_24 => Some(BuiltIn::Blobhash),
            "block" => Some(BuiltIn::Block),
            "blockhash" if self.language_version >= VERSION_0_4_22 => Some(BuiltIn::Blockhash),
            "ecrecover" => Some(BuiltIn::Ecrecover),
            "gasleft" => Some(BuiltIn::Gasleft),
            "keccak256" => Some(BuiltIn::Keccak256),
            "log0" | "log1" | "log2" | "log3" | "log4" if self.language_version < VERSION_0_8_0 => {
                let arity = symbol.as_bytes()[3] - b'0';
                Some(BuiltIn::Log(arity))
            }
            "msg" => Some(BuiltIn::Msg),
            "mulmod" => Some(BuiltIn::Mulmod),
            "require" => Some(BuiltIn::Require),
            "revert" => Some(BuiltIn::Revert),
            "ripemd160" => Some(BuiltIn::Ripemd160),
            "selfdestruct" => Some(BuiltIn::Selfdestruct),
            "sha256" => Some(BuiltIn::Sha256),
            "sha3" if self.language_version < VERSION_0_5_0 => Some(BuiltIn::Sha3),
            "suicide" if self.language_version < VERSION_0_5_0 => Some(BuiltIn::Suicide),
            "tx" => Some(BuiltIn::Tx),
            _ => None,
        }
    }

    pub(crate) fn lookup_yul_global(&self, symbol: &str) -> Option<BuiltIn> {
        match symbol {
            "keccak256" if self.language_version >= VERSION_0_4_12 => Some(BuiltIn::YulKeccak256),
            // TODO: add the rest of the built-ins
            _ => None,
        }
    }

    pub(crate) fn lookup_member_of(&self, built_in: &BuiltIn, symbol: &str) -> Option<BuiltIn> {
        match built_in {
            BuiltIn::Abi => match symbol {
                "decode" if self.language_version >= VERSION_0_5_0 => Some(BuiltIn::AbiDecode),
                "encode" if self.language_version >= VERSION_0_4_22 => Some(BuiltIn::AbiEncode),
                "encodeCall" if self.language_version >= VERSION_0_8_11 => {
                    Some(BuiltIn::AbiEncodeCall)
                }
                "encodePacked" if self.language_version >= VERSION_0_4_22 => {
                    Some(BuiltIn::AbiEncodePacked)
                }
                "encodeWithSelector" if self.language_version >= VERSION_0_4_22 => {
                    Some(BuiltIn::AbiEncodeWithSelector)
                }
                "encodeWithSignature" if self.language_version >= VERSION_0_4_22 => {
                    Some(BuiltIn::AbiEncodeWithSignature)
                }
                _ => None,
            },
            BuiltIn::Block => match symbol {
                "basefee" if self.language_version >= VERSION_0_8_7 => Some(BuiltIn::BlockBasefee),
                "blobbasefee" if self.language_version >= VERSION_0_8_24 => {
                    Some(BuiltIn::BlockBlobbasefee)
                }
                "chainid" if self.language_version >= VERSION_0_8_0 => Some(BuiltIn::BlockChainid),
                "coinbase" => Some(BuiltIn::BlockCoinbase),
                "difficulty" => Some(BuiltIn::BlockDifficulty),
                "gaslimit" => Some(BuiltIn::BlockGaslimit),
                "number" => Some(BuiltIn::BlockNumber),
                "prevrandao" if self.language_version >= VERSION_0_8_18 => {
                    Some(BuiltIn::BlockPrevrandao)
                }
                "timestamp" => Some(BuiltIn::BlockTimestamp),
                "blockhash" if self.language_version < VERSION_0_5_0 => Some(BuiltIn::Blockhash),
                _ => None,
            },
            BuiltIn::Tx => match symbol {
                "gasprice" => Some(BuiltIn::TxGasPrice),
                "origin" => Some(BuiltIn::TxOrigin),
                _ => None,
            },
            BuiltIn::Msg => match symbol {
                "data" => Some(BuiltIn::MsgData),
                "gas" if self.language_version < VERSION_0_5_0 => Some(BuiltIn::MsgGas),
                "sender" => Some(BuiltIn::MsgSender),
                "sig" => Some(BuiltIn::MsgSig),
                "value" => Some(BuiltIn::MsgValue),
                _ => None,
            },
            _ => None,
        }
    }

    pub(crate) fn lookup_member_of_user_definition(
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
            Definition::UserDefinedValueType(udvt) if self.language_version >= VERSION_0_8_8 => {
                match symbol {
                    "wrap" => Some(BuiltIn::Wrap(udvt.node_id)),
                    "unwrap" => Some(BuiltIn::Unwrap(udvt.node_id)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn lookup_member_of_meta_type(
        &self,
        parent_type: &Type,
        symbol: &str,
    ) -> Option<BuiltIn> {
        match parent_type {
            Type::Bytes { .. } => match symbol {
                "concat" => Some(BuiltIn::BytesConcat),
                _ => None,
            },
            Type::String { .. } => match symbol {
                "concat" => Some(BuiltIn::StringConcat),
                _ => None,
            },
            _ => None,
        }
    }

    pub(crate) fn lookup_member_of_type(
        &self,
        parent_type: &Type,
        symbol: &str,
    ) -> Option<BuiltIn> {
        match parent_type {
            Type::Address { payable } => {
                if !payable || self.language_version >= VERSION_0_5_0 {
                    match symbol {
                        "balance" => Some(BuiltIn::AddressBalance),
                        "code" if self.language_version >= VERSION_0_8_0 => {
                            Some(BuiltIn::AddressCode)
                        }
                        "codehash" if self.language_version >= VERSION_0_8_0 => {
                            Some(BuiltIn::AddressCodehash)
                        }
                        "call" => Some(BuiltIn::AddressCall),
                        "callcode" if !payable => Some(BuiltIn::AddressCallcode),
                        "delegatecall" => Some(BuiltIn::AddressDelegatecall),
                        "send" => Some(BuiltIn::AddressSend),
                        "staticcall" => Some(BuiltIn::AddressStaticcall),
                        "transfer" => Some(BuiltIn::AddressTransfer),
                        _ => None,
                    }
                } else {
                    None
                }
            }
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
                "length" => Some(BuiltIn::Length),
                "pop" if self.language_version >= VERSION_0_5_0 => Some(BuiltIn::ArrayPop),
                "push" => Some(BuiltIn::ArrayPush(self.types.uint8())),
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
            "gas" => Some(BuiltIn::CallOptionGas),
            "salt" => Some(BuiltIn::CallOptionSalt),
            "value" => Some(BuiltIn::CallOptionValue),
            _ => None,
        }
    }

    pub(crate) fn typing_of(&self, built_in: &BuiltIn) -> Typing {
        match built_in {
            // variables and members
            BuiltIn::AddressBalance => Typing::Resolved(self.types.uint256()),
            BuiltIn::AddressCode => Typing::Resolved(self.types.bytes()),
            BuiltIn::AddressCodehash => Typing::Resolved(self.types.bytes32()),
            BuiltIn::BlockBasefee => Typing::Resolved(self.types.uint256()),
            BuiltIn::BlockBlobbasefee => Typing::Resolved(self.types.uint256()),
            BuiltIn::BlockChainid => Typing::Resolved(self.types.uint256()),
            BuiltIn::BlockCoinbase => Typing::Resolved(self.types.address_payable()),
            BuiltIn::BlockDifficulty => Typing::Resolved(self.types.uint256()),
            BuiltIn::BlockGaslimit => Typing::Resolved(self.types.uint256()),
            BuiltIn::BlockNumber => Typing::Resolved(self.types.uint256()),
            BuiltIn::BlockPrevrandao => Typing::Resolved(self.types.uint256()),
            BuiltIn::BlockTimestamp => Typing::Resolved(self.types.uint256()),
            BuiltIn::Length => Typing::Resolved(self.types.uint256()),
            BuiltIn::MsgGas => Typing::Resolved(self.types.uint256()),
            BuiltIn::MsgData => Typing::Resolved(self.types.bytes()),
            BuiltIn::MsgSender => Typing::Resolved(self.types.address()),
            BuiltIn::MsgSig => Typing::Resolved(self.types.bytes4()),
            BuiltIn::MsgValue => Typing::Resolved(self.types.uint256()),
            BuiltIn::Selector => Typing::Resolved(self.types.bytes4()),
            BuiltIn::TxGasPrice => Typing::Resolved(self.types.uint256()),
            BuiltIn::TxOrigin => Typing::Resolved(self.types.address()),

            // built-in functions and types
            BuiltIn::Abi
            | BuiltIn::AbiDecode
            | BuiltIn::AbiEncode
            | BuiltIn::AbiEncodeCall
            | BuiltIn::AbiEncodePacked
            | BuiltIn::AbiEncodeWithSelector
            | BuiltIn::AbiEncodeWithSignature
            | BuiltIn::AddressCall
            | BuiltIn::AddressCallcode
            | BuiltIn::AddressDelegatecall
            | BuiltIn::AddressSend
            | BuiltIn::AddressStaticcall
            | BuiltIn::AddressTransfer
            | BuiltIn::Addmod
            | BuiltIn::ArrayPop
            | BuiltIn::ArrayPush(_)
            | BuiltIn::Assert
            | BuiltIn::Blobhash
            | BuiltIn::Block
            | BuiltIn::Blockhash
            | BuiltIn::BytesConcat
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
            | BuiltIn::StringConcat
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
            BuiltIn::AbiDecode => {
                // TODO: the return type is a tuple of the types given in the
                // arguments
                Typing::Unresolved
            }
            BuiltIn::AbiEncode => Typing::Resolved(self.types.bytes()),
            BuiltIn::AbiEncodeCall => Typing::Resolved(self.types.bytes()),
            BuiltIn::AbiEncodePacked => Typing::Resolved(self.types.bytes()),
            BuiltIn::AbiEncodeWithSelector => Typing::Resolved(self.types.bytes()),
            BuiltIn::AbiEncodeWithSignature => Typing::Resolved(self.types.bytes()),
            BuiltIn::Addmod => Typing::Resolved(self.types.uint256()),
            BuiltIn::AddressCall => {
                if self.language_version >= VERSION_0_5_0 {
                    Typing::Resolved(self.types.boolean_bytes_tuple())
                } else {
                    Typing::Resolved(self.types.boolean())
                }
            }
            BuiltIn::AddressCallcode if self.language_version >= VERSION_0_5_0 => {
                Typing::Resolved(self.types.boolean_bytes_tuple())
            }
            BuiltIn::AddressDelegatecall => {
                if self.language_version >= VERSION_0_5_0 {
                    Typing::Resolved(self.types.boolean_bytes_tuple())
                } else {
                    Typing::Resolved(self.types.boolean())
                }
            }
            BuiltIn::AddressSend if self.language_version < VERSION_0_8_0 => {
                Typing::Resolved(self.types.boolean())
            }
            BuiltIn::AddressStaticcall if self.language_version >= VERSION_0_5_0 => {
                Typing::Resolved(self.types.boolean_bytes_tuple())
            }
            BuiltIn::AddressTransfer if self.language_version < VERSION_0_8_0 => {
                Typing::Resolved(self.types.void())
            }
            BuiltIn::ArrayPush(type_id) => {
                if argument_typings.is_empty() {
                    if self.language_version >= VERSION_0_6_0 {
                        Typing::Resolved(*type_id)
                    } else {
                        Typing::Unresolved
                    }
                } else if self.language_version >= VERSION_0_6_0 {
                    Typing::Resolved(self.types.void())
                } else {
                    Typing::Resolved(self.types.uint256())
                }
            }
            BuiltIn::ArrayPop => Typing::Resolved(self.types.void()),
            BuiltIn::Assert => Typing::Resolved(self.types.void()),
            BuiltIn::Blobhash => Typing::Resolved(self.types.bytes32()),
            BuiltIn::Blockhash => Typing::Resolved(self.types.bytes32()),
            BuiltIn::BytesConcat => Typing::Resolved(self.types.bytes()),
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
            BuiltIn::StringConcat => Typing::Resolved(self.types.string()),
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
