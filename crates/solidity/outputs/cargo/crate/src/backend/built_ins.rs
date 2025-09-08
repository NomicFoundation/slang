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
    Address,
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
    LegacyCallOptionGas(Option<TypeId>),
    LegacyCallOptionValue(Option<TypeId>),
    LegacyCallOptionValueNew(TypeId),
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
    Now,
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
    Type(TypeId),
    TypeName,
    TypeCreationCode,
    TypeRuntimeCode,
    TypeInterfaceId,
    TypeMin(TypeId),
    TypeMax(TypeId),
    Unwrap(NodeId),
    Wrap(NodeId),

    YulAdd,
    YulAddmod,
    YulAddress,
    YulAddressField,
    YulAnd,
    YulBalance,
    YulBasefee,
    YulBlobbasefee,
    YulBlobhash,
    YulBlockhash,
    YulByte,
    YulCallcode,
    YulCalldatacopy,
    YulCalldataload,
    YulCalldatasize,
    YulCaller,
    YulCall,
    YulCallvalue,
    YulChainid,
    YulCodecopy,
    YulCodesize,
    YulCoinbase,
    YulCreate,
    YulCreate2,
    YulDelegatecall,
    YulDifficulty,
    YulDiv,
    YulEq,
    YulExp,
    YulExtcodecopy,
    YulExtcodehash,
    YulExtcodesize,
    YulGas,
    YulGaslimit,
    YulGasprice,
    YulGt,
    YulInvalid,
    YulIszero,
    YulJump,
    YulJumpi,
    YulKeccak256,
    YulLengthField,
    YulLog(u8),
    YulLt,
    YulMcopy,
    YulMload,
    YulMod,
    YulMsize,
    YulMstore8,
    YulMstore,
    YulMul,
    YulMulmod,
    YulNot,
    YulNumber,
    YulOffset,
    YulOr,
    YulOrigin,
    YulPop,
    YulPrevrandao,
    YulReturn,
    YulReturndatacopy,
    YulReturndatasize,
    YulRevert,
    YulSar,
    YulSdiv,
    YulSelector,
    YulSelfbalance,
    YulSelfdestruct,
    YulSgt,
    YulSha3,
    YulShl,
    YulShr,
    YulSignextend,
    YulSload,
    YulSlot,
    YulSlt,
    YulSmod,
    YulSstore,
    YulStaticcall,
    YulStop,
    YulSub,
    YulSuicide,
    YulTimestamp,
    YulTload,
    YulTstore,
    YulXor,
}

const VERSION_0_4_12: Version = Version::new(0, 4, 12);
const VERSION_0_4_22: Version = Version::new(0, 4, 22);
const VERSION_0_5_0: Version = Version::new(0, 5, 0);
const VERSION_0_5_3: Version = Version::new(0, 5, 3);
const VERSION_0_6_0: Version = Version::new(0, 6, 0);
const VERSION_0_6_2: Version = Version::new(0, 6, 2);
const VERSION_0_6_7: Version = Version::new(0, 6, 7);
const VERSION_0_6_8: Version = Version::new(0, 6, 8);
const VERSION_0_7_0: Version = Version::new(0, 7, 0);
const VERSION_0_8_0: Version = Version::new(0, 8, 0);
const VERSION_0_8_4: Version = Version::new(0, 8, 4);
const VERSION_0_8_7: Version = Version::new(0, 8, 7);
const VERSION_0_8_8: Version = Version::new(0, 8, 8);
const VERSION_0_8_11: Version = Version::new(0, 8, 11);
const VERSION_0_8_15: Version = Version::new(0, 8, 15);
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
            "now" if self.language_version < VERSION_0_7_0 => Some(BuiltIn::Now),
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
            "add" => Some(BuiltIn::YulAdd),
            "addmod" => Some(BuiltIn::YulAddmod),
            "address" => Some(BuiltIn::YulAddress),
            "and" => Some(BuiltIn::YulAnd),
            "balance" => Some(BuiltIn::YulBalance),
            "basefee" if self.language_version >= VERSION_0_8_7 => Some(BuiltIn::YulBasefee),
            "blobbasefee" if self.language_version >= VERSION_0_8_24 => {
                Some(BuiltIn::YulBlobbasefee)
            }
            "blobhash" if self.language_version >= VERSION_0_8_24 => Some(BuiltIn::YulBlobhash),
            "blockhash" => Some(BuiltIn::YulBlockhash),
            "byte" => Some(BuiltIn::YulByte),
            "callcode" => Some(BuiltIn::YulCallcode),
            "calldatacopy" => Some(BuiltIn::YulCalldatacopy),
            "calldataload" => Some(BuiltIn::YulCalldataload),
            "calldatasize" => Some(BuiltIn::YulCalldatasize),
            "caller" => Some(BuiltIn::YulCaller),
            "call" => Some(BuiltIn::YulCall),
            "callvalue" => Some(BuiltIn::YulCallvalue),
            "chainid" => Some(BuiltIn::YulChainid),
            "codecopy" => Some(BuiltIn::YulCodecopy),
            "codesize" => Some(BuiltIn::YulCodesize),
            "coinbase" => Some(BuiltIn::YulCoinbase),
            "create" => Some(BuiltIn::YulCreate),
            "create2" if self.language_version >= VERSION_0_4_12 => Some(BuiltIn::YulCreate2),
            "delegatecall" => Some(BuiltIn::YulDelegatecall),
            "difficulty" if self.language_version < VERSION_0_8_18 => Some(BuiltIn::YulDifficulty),
            "div" => Some(BuiltIn::YulDiv),
            "eq" => Some(BuiltIn::YulEq),
            "exp" => Some(BuiltIn::YulExp),
            "extcodecopy" => Some(BuiltIn::YulExtcodecopy),
            "extcodehash" if self.language_version >= VERSION_0_5_0 => {
                Some(BuiltIn::YulExtcodehash)
            }
            "extcodesize" => Some(BuiltIn::YulExtcodesize),
            "gas" => Some(BuiltIn::YulGas),
            "gaslimit" => Some(BuiltIn::YulGaslimit),
            "gasprice" => Some(BuiltIn::YulGasprice),
            "gt" => Some(BuiltIn::YulGt),
            "invalid" => Some(BuiltIn::YulInvalid),
            "iszero" => Some(BuiltIn::YulIszero),
            "jump" if self.language_version < VERSION_0_5_0 => Some(BuiltIn::YulJump),
            "jumpi" if self.language_version < VERSION_0_5_0 => Some(BuiltIn::YulJumpi),
            "keccak256" if self.language_version >= VERSION_0_4_12 => Some(BuiltIn::YulKeccak256),
            "log0" | "log1" | "log2" | "log3" | "log4" => {
                let arity = symbol.as_bytes()[3] - b'0';
                Some(BuiltIn::YulLog(arity))
            }
            "lt" => Some(BuiltIn::YulLt),
            "mcopy" if self.language_version >= VERSION_0_8_24 => Some(BuiltIn::YulMcopy),
            "mload" => Some(BuiltIn::YulMload),
            "mod" => Some(BuiltIn::YulMod),
            "msize" => Some(BuiltIn::YulMsize),
            "mstore8" => Some(BuiltIn::YulMstore8),
            "mstore" => Some(BuiltIn::YulMstore),
            "mul" => Some(BuiltIn::YulMul),
            "mulmod" => Some(BuiltIn::YulMulmod),
            "not" => Some(BuiltIn::YulNot),
            "number" => Some(BuiltIn::YulNumber),
            "or" => Some(BuiltIn::YulOr),
            "origin" => Some(BuiltIn::YulOrigin),
            "pop" => Some(BuiltIn::YulPop),
            "prevrandao" if self.language_version >= VERSION_0_8_18 => Some(BuiltIn::YulPrevrandao),
            "return" => Some(BuiltIn::YulReturn),
            "returndatacopy" => Some(BuiltIn::YulReturndatacopy),
            "returndatasize" => Some(BuiltIn::YulReturndatasize),
            "revert" => Some(BuiltIn::YulRevert),
            "sar" => Some(BuiltIn::YulSar),
            "sdiv" => Some(BuiltIn::YulSdiv),
            "selfbalance" => Some(BuiltIn::YulSelfbalance),
            "selfdestruct" => Some(BuiltIn::YulSelfdestruct),
            "sgt" => Some(BuiltIn::YulSgt),
            "sha3" if self.language_version < VERSION_0_5_0 => Some(BuiltIn::YulSha3),
            "shl" => Some(BuiltIn::YulShl),
            "shr" => Some(BuiltIn::YulShr),
            "signextend" => Some(BuiltIn::YulSignextend),
            "sload" => Some(BuiltIn::YulSload),
            "slt" => Some(BuiltIn::YulSlt),
            "smod" => Some(BuiltIn::YulSmod),
            "sstore" => Some(BuiltIn::YulSstore),
            "staticcall" if self.language_version >= VERSION_0_4_12 => Some(BuiltIn::YulStaticcall),
            "stop" => Some(BuiltIn::YulStop),
            "sub" => Some(BuiltIn::YulSub),
            "suicide" if self.language_version < VERSION_0_5_0 => Some(BuiltIn::YulSuicide),
            "timestamp" => Some(BuiltIn::YulTimestamp),
            "tload" if self.language_version >= VERSION_0_8_24 => Some(BuiltIn::YulTload),
            "tstore" if self.language_version >= VERSION_0_8_24 => Some(BuiltIn::YulTstore),
            "xor" => Some(BuiltIn::YulXor),

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
            BuiltIn::Type(type_id) => match self.types.get_type_by_id(*type_id) {
                Type::Contract { .. } => match symbol {
                    "name" => Some(BuiltIn::TypeName),
                    "creationCode" if self.language_version >= VERSION_0_5_3 => {
                        Some(BuiltIn::TypeCreationCode)
                    }
                    "runtimeCode" if self.language_version >= VERSION_0_5_3 => {
                        Some(BuiltIn::TypeRuntimeCode)
                    }
                    "interfaceId" if self.language_version >= VERSION_0_6_7 => {
                        Some(BuiltIn::TypeInterfaceId)
                    }
                    _ => None,
                },
                Type::Interface { .. } => match symbol {
                    "name" => Some(BuiltIn::TypeName),
                    "interfaceId" if self.language_version >= VERSION_0_6_7 => {
                        Some(BuiltIn::TypeInterfaceId)
                    }
                    _ => None,
                },
                Type::Enum { .. } if self.language_version >= VERSION_0_8_8 => match symbol {
                    "min" => Some(BuiltIn::TypeMin(*type_id)),
                    "max" => Some(BuiltIn::TypeMax(*type_id)),
                    _ => None,
                },
                Type::Integer { .. } if self.language_version >= VERSION_0_6_8 => match symbol {
                    "min" => Some(BuiltIn::TypeMin(*type_id)),
                    "max" => Some(BuiltIn::TypeMax(*type_id)),
                    _ => None,
                },
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
            BuiltIn::AddressCall if self.language_version < VERSION_0_7_0 => match symbol {
                "gas" => Some(BuiltIn::LegacyCallOptionGas(None)),
                "value" => Some(BuiltIn::LegacyCallOptionValue(None)),
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
                "selector" if self.language_version >= VERSION_0_8_4 => Some(BuiltIn::Selector),
                _ => None,
            },
            Definition::Event(_) => match symbol {
                "selector" if self.language_version >= VERSION_0_8_15 => Some(BuiltIn::Selector),
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

    fn lookup_member_of_address(&self, symbol: &str, payable: bool) -> Option<BuiltIn> {
        if !payable || self.language_version >= VERSION_0_5_0 {
            match symbol {
                "balance" => Some(BuiltIn::AddressBalance),
                "code" if self.language_version >= VERSION_0_8_0 => Some(BuiltIn::AddressCode),
                "codehash" if self.language_version >= VERSION_0_8_0 => {
                    Some(BuiltIn::AddressCodehash)
                }
                "call" => Some(BuiltIn::AddressCall),
                "callcode" if !payable => Some(BuiltIn::AddressCallcode),
                "delegatecall" => Some(BuiltIn::AddressDelegatecall),
                "send" if payable || self.language_version < VERSION_0_5_0 => {
                    Some(BuiltIn::AddressSend)
                }
                "staticcall" if self.language_version >= VERSION_0_5_0 => {
                    Some(BuiltIn::AddressStaticcall)
                }
                "transfer" if payable || self.language_version < VERSION_0_8_0 => {
                    Some(BuiltIn::AddressTransfer)
                }
                _ => None,
            }
        } else {
            None
        }
    }

    pub(crate) fn lookup_member_of_type_id(
        &self,
        type_id: TypeId,
        symbol: &str,
    ) -> Option<BuiltIn> {
        let type_ = self.types.get_type_by_id(type_id);
        match type_ {
            Type::Address { payable } => self.lookup_member_of_address(symbol, *payable),
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
            Type::Contract { .. } | Type::Interface { .. } => {
                if self.language_version < VERSION_0_5_0 {
                    self.lookup_member_of_address(symbol, false)
                } else {
                    None
                }
            }
            Type::Enum { .. } => None,
            Type::FixedPointNumber { .. } => None,
            Type::Function { external, .. } => {
                if *external {
                    match symbol {
                        "address" => Some(BuiltIn::Address),
                        "selector" => Some(BuiltIn::Selector),
                        "gas" if self.language_version < VERSION_0_7_0 => {
                            Some(BuiltIn::LegacyCallOptionGas(Some(type_id)))
                        }
                        "value" if self.language_version < VERSION_0_7_0 => {
                            Some(BuiltIn::LegacyCallOptionValue(Some(type_id)))
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Type::Integer { .. } => None,
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
                "address" => Some(BuiltIn::YulAddressField),
                "length" => Some(BuiltIn::YulLengthField),
                "offset" => Some(BuiltIn::YulOffset),
                "selector" => Some(BuiltIn::YulSelector),
                "slot" => Some(BuiltIn::YulSlot),
                _ => None,
            },
            _ => None,
        }
    }

    pub(crate) fn lookup_call_option(&self, symbol: &str) -> Option<BuiltIn> {
        match symbol {
            "gas" => Some(BuiltIn::CallOptionGas),
            "salt" if self.language_version >= VERSION_0_6_2 => Some(BuiltIn::CallOptionSalt),
            "value" => Some(BuiltIn::CallOptionValue),
            _ => None,
        }
    }

    pub(crate) fn lookup_member_of_new_type_id(
        &self,
        type_id: TypeId,
        symbol: &str,
    ) -> Option<BuiltIn> {
        match symbol {
            "value" if self.language_version < VERSION_0_7_0 => {
                Some(BuiltIn::LegacyCallOptionValueNew(type_id))
            }
            _ => None,
        }
    }

    pub(crate) fn typing_of(&self, built_in: &BuiltIn) -> Typing {
        match built_in {
            // variables and members
            BuiltIn::Address => Typing::Resolved(self.types.address()),
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
            BuiltIn::MsgSender => {
                if self.language_version >= VERSION_0_5_0 && self.language_version < VERSION_0_8_0 {
                    Typing::Resolved(self.types.address_payable())
                } else {
                    Typing::Resolved(self.types.address())
                }
            }
            BuiltIn::MsgSig => Typing::Resolved(self.types.bytes4()),
            BuiltIn::MsgValue => Typing::Resolved(self.types.uint256()),
            BuiltIn::Now => Typing::Resolved(self.types.uint256()),
            BuiltIn::Selector => Typing::Resolved(self.types.bytes4()),
            BuiltIn::TxGasPrice => Typing::Resolved(self.types.uint256()),
            BuiltIn::TxOrigin => {
                if self.language_version >= VERSION_0_8_0 {
                    Typing::Resolved(self.types.address())
                } else {
                    Typing::Resolved(self.types.address_payable())
                }
            }
            BuiltIn::TypeName => Typing::Resolved(self.types.string()),
            BuiltIn::TypeCreationCode => Typing::Resolved(self.types.bytes()),
            BuiltIn::TypeRuntimeCode => Typing::Resolved(self.types.bytes()),
            BuiltIn::TypeInterfaceId => Typing::Resolved(self.types.bytes4()),
            BuiltIn::TypeMin(type_id) => Typing::Resolved(*type_id),
            BuiltIn::TypeMax(type_id) => Typing::Resolved(*type_id),

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
            | BuiltIn::LegacyCallOptionGas(_)
            | BuiltIn::LegacyCallOptionValue(_)
            | BuiltIn::LegacyCallOptionValueNew(_)
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

    #[allow(clippy::too_many_lines)]
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
            BuiltIn::LegacyCallOptionGas(type_id) | BuiltIn::LegacyCallOptionValue(type_id) => {
                if let Some(type_id) = type_id {
                    Typing::Resolved(*type_id)
                } else if self.language_version < VERSION_0_7_0 {
                    Typing::BuiltIn(BuiltIn::AddressCall)
                } else {
                    Typing::Unresolved
                }
            }
            BuiltIn::LegacyCallOptionValueNew(type_id) => Typing::NewExpression(*type_id),
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
