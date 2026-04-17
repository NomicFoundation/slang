use slang_solidity_v2_common::versions::LanguageVersion;

use super::binder::{Binder, Definition, Typing};
use super::types::{DataLocation, LiteralKind, Type, TypeId, TypeRegistry};

#[path = "internal.generated.rs"]
mod internal;

pub use internal::BuiltIn;

pub(crate) struct BuiltInsResolver<'a> {
    language_version: LanguageVersion,
    binder: &'a Binder,
    types: &'a TypeRegistry,
}

impl<'a> BuiltInsResolver<'a> {
    pub(crate) fn new(
        language_version: LanguageVersion,
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
            "blobhash" if self.language_version >= BuiltIn::BLOBHASH_FROM => {
                Some(BuiltIn::Blobhash)
            }
            "block" => Some(BuiltIn::Block),
            "blockhash" => Some(BuiltIn::Blockhash),
            "ecrecover" => Some(BuiltIn::Ecrecover),
            "gasleft" => Some(BuiltIn::Gasleft),
            "keccak256" => Some(BuiltIn::Keccak256),
            "msg" => Some(BuiltIn::Msg),
            "mulmod" => Some(BuiltIn::Mulmod),
            "require" => Some(BuiltIn::Require),
            "revert" => Some(BuiltIn::Revert),
            "ripemd160" => Some(BuiltIn::Ripemd160),
            "selfdestruct" => Some(BuiltIn::Selfdestruct),
            "sha256" => Some(BuiltIn::Sha256),
            "tx" => Some(BuiltIn::Tx),
            _ => None,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn lookup_yul_global(&self, symbol: &str) -> Option<BuiltIn> {
        // TODO(validation): Yul built-in function names should be considered reserved words.
        // They're currently parsed as Identifiers to facilitate their manipulation.
        match symbol {
            "add" => Some(BuiltIn::YulAdd),
            "addmod" => Some(BuiltIn::YulAddmod),
            "address" => Some(BuiltIn::YulAddress),
            "and" => Some(BuiltIn::YulAnd),
            "balance" => Some(BuiltIn::YulBalance),
            "basefee" if self.language_version >= BuiltIn::YUL_BASEFEE_FROM => {
                Some(BuiltIn::YulBasefee)
            }
            "blobbasefee" if self.language_version >= BuiltIn::YUL_BLOBBASEFEE_FROM => {
                Some(BuiltIn::YulBlobbasefee)
            }
            "blobhash" if self.language_version >= BuiltIn::YUL_BLOBHASH_FROM => {
                Some(BuiltIn::YulBlobhash)
            }
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
            "clz" if self.language_version >= BuiltIn::YUL_CLZ_FROM => Some(BuiltIn::YulClz),
            "codecopy" => Some(BuiltIn::YulCodecopy),
            "codesize" => Some(BuiltIn::YulCodesize),
            "coinbase" => Some(BuiltIn::YulCoinbase),
            "create" => Some(BuiltIn::YulCreate),
            "create2" => Some(BuiltIn::YulCreate2),
            "delegatecall" => Some(BuiltIn::YulDelegatecall),
            "difficulty" if self.language_version < BuiltIn::YUL_DIFFICULTY_TILL => {
                Some(BuiltIn::YulDifficulty)
            }
            "div" => Some(BuiltIn::YulDiv),
            "eq" => Some(BuiltIn::YulEq),
            "exp" => Some(BuiltIn::YulExp),
            "extcodecopy" => Some(BuiltIn::YulExtcodecopy),
            "extcodehash" => Some(BuiltIn::YulExtcodehash),
            "extcodesize" => Some(BuiltIn::YulExtcodesize),
            "gas" => Some(BuiltIn::YulGas),
            "gaslimit" => Some(BuiltIn::YulGaslimit),
            "gasprice" => Some(BuiltIn::YulGasprice),
            "gt" => Some(BuiltIn::YulGt),
            "invalid" => Some(BuiltIn::YulInvalid),
            "iszero" => Some(BuiltIn::YulIszero),
            "keccak256" => Some(BuiltIn::YulKeccak256),
            "log0" | "log1" | "log2" | "log3" | "log4" => {
                let arity = symbol.as_bytes()[3] - b'0';
                Some(BuiltIn::YulLog(arity))
            }
            "lt" => Some(BuiltIn::YulLt),
            "mcopy" if self.language_version >= BuiltIn::YUL_MCOPY_FROM => Some(BuiltIn::YulMcopy),
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
            "prevrandao" if self.language_version >= BuiltIn::YUL_PREVRANDAO_FROM => {
                Some(BuiltIn::YulPrevrandao)
            }
            "return" => Some(BuiltIn::YulReturn),
            "returndatacopy" => Some(BuiltIn::YulReturndatacopy),
            "returndatasize" => Some(BuiltIn::YulReturndatasize),
            "revert" => Some(BuiltIn::YulRevert),
            "sar" => Some(BuiltIn::YulSar),
            "sdiv" => Some(BuiltIn::YulSdiv),
            "selfbalance" => Some(BuiltIn::YulSelfbalance),
            "selfdestruct" => Some(BuiltIn::YulSelfdestruct),
            "sgt" => Some(BuiltIn::YulSgt),
            "shl" => Some(BuiltIn::YulShl),
            "shr" => Some(BuiltIn::YulShr),
            "signextend" => Some(BuiltIn::YulSignextend),
            "sload" => Some(BuiltIn::YulSload),
            "slt" => Some(BuiltIn::YulSlt),
            "smod" => Some(BuiltIn::YulSmod),
            "sstore" => Some(BuiltIn::YulSstore),
            "staticcall" => Some(BuiltIn::YulStaticcall),
            "stop" => Some(BuiltIn::YulStop),
            "sub" => Some(BuiltIn::YulSub),
            "timestamp" => Some(BuiltIn::YulTimestamp),
            "tload" if self.language_version >= BuiltIn::YUL_TLOAD_FROM => Some(BuiltIn::YulTload),
            "tstore" if self.language_version >= BuiltIn::YUL_TSTORE_FROM => {
                Some(BuiltIn::YulTstore)
            }
            "xor" => Some(BuiltIn::YulXor),

            _ => None,
        }
    }

    pub(crate) fn lookup_member_of(&self, built_in: &BuiltIn, symbol: &str) -> Option<BuiltIn> {
        match built_in {
            BuiltIn::Abi => match symbol {
                "decode" => Some(BuiltIn::AbiDecode),
                "encode" => Some(BuiltIn::AbiEncode),
                "encodeCall" if self.language_version >= BuiltIn::ABI_ENCODE_CALL_FROM => {
                    Some(BuiltIn::AbiEncodeCall)
                }
                "encodePacked" => Some(BuiltIn::AbiEncodePacked),
                "encodeWithSelector" => Some(BuiltIn::AbiEncodeWithSelector),
                "encodeWithSignature" => Some(BuiltIn::AbiEncodeWithSignature),
                _ => None,
            },
            BuiltIn::Block => match symbol {
                "basefee" if self.language_version >= BuiltIn::BLOCK_BASEFEE_FROM => {
                    Some(BuiltIn::BlockBasefee)
                }
                "blobbasefee" if self.language_version >= BuiltIn::BLOCK_BLOBBASEFEE_FROM => {
                    Some(BuiltIn::BlockBlobbasefee)
                }
                "chainid" => Some(BuiltIn::BlockChainid),
                "coinbase" => Some(BuiltIn::BlockCoinbase),
                "difficulty" => Some(BuiltIn::BlockDifficulty),
                "gaslimit" => Some(BuiltIn::BlockGaslimit),
                "number" => Some(BuiltIn::BlockNumber),
                "prevrandao" if self.language_version >= BuiltIn::BLOCK_PREVRANDAO_FROM => {
                    Some(BuiltIn::BlockPrevrandao)
                }
                "timestamp" => Some(BuiltIn::BlockTimestamp),
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
                    "creationCode" => Some(BuiltIn::TypeCreationCode),
                    "runtimeCode" => Some(BuiltIn::TypeRuntimeCode),
                    "interfaceId" => Some(BuiltIn::TypeInterfaceId),
                    _ => None,
                },
                Type::Interface { .. } => match symbol {
                    "name" => Some(BuiltIn::TypeName),
                    "interfaceId" => Some(BuiltIn::TypeInterfaceId),
                    _ => None,
                },
                Type::Enum { .. } => match symbol {
                    "min" if self.language_version >= BuiltIn::TYPE_ENUM_MIN_FROM => {
                        Some(BuiltIn::TypeEnumMin(*type_id))
                    }
                    "max" if self.language_version >= BuiltIn::TYPE_ENUM_MAX_FROM => {
                        Some(BuiltIn::TypeEnumMax(*type_id))
                    }
                    _ => None,
                },
                Type::Integer { .. } => match symbol {
                    "min" => Some(BuiltIn::TypeMin(*type_id)),
                    "max" => Some(BuiltIn::TypeMax(*type_id)),
                    _ => None,
                },
                _ => None,
            },
            BuiltIn::Msg => match symbol {
                "data" => Some(BuiltIn::MsgData),
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
                "selector" if self.language_version >= BuiltIn::ERROR_SELECTOR_FROM => {
                    Some(BuiltIn::ErrorSelector)
                }
                _ => None,
            },
            Definition::Event(_) => match symbol {
                "selector" if self.language_version >= BuiltIn::EVENT_SELECTOR_FROM => {
                    Some(BuiltIn::EventSelector)
                }
                _ => None,
            },
            Definition::UserDefinedValueType(_) => match symbol {
                "wrap" if self.language_version >= BuiltIn::WRAP_FROM => {
                    Some(BuiltIn::Wrap(definition.node_id()))
                }
                "unwrap" if self.language_version >= BuiltIn::UNWRAP_FROM => {
                    Some(BuiltIn::Unwrap(definition.node_id()))
                }
                _ => None,
            },
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

    fn lookup_member_of_address(symbol: &str, payable: bool) -> Option<BuiltIn> {
        match symbol {
            "balance" => Some(BuiltIn::AddressBalance),
            "code" => Some(BuiltIn::AddressCode),
            "codehash" => Some(BuiltIn::AddressCodehash),
            "call" => Some(BuiltIn::AddressCall),
            "callcode" if !payable => Some(BuiltIn::AddressCallcode),
            "delegatecall" => Some(BuiltIn::AddressDelegatecall),
            "send" if payable => Some(BuiltIn::AddressSend),
            "staticcall" => Some(BuiltIn::AddressStaticcall),
            "transfer" if payable => Some(BuiltIn::AddressTransfer),
            _ => None,
        }
    }

    pub(crate) fn lookup_member_of_type_id(
        &self,
        type_id: TypeId,
        symbol: &str,
    ) -> Option<BuiltIn> {
        let type_ = self.types.get_type_by_id(type_id);
        match type_ {
            Type::Address { payable } => Self::lookup_member_of_address(symbol, *payable),
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
            Type::Bytes { location } => match symbol {
                "length" => Some(BuiltIn::Length),
                "pop" if *location == DataLocation::Storage => Some(BuiltIn::ArrayPop),
                "push" if *location == DataLocation::Storage => {
                    Some(BuiltIn::ArrayPush(self.types.uint8()))
                }
                _ => None,
            },
            Type::Contract { .. } | Type::Interface { .. } => None,
            Type::Enum { .. } => None,
            Type::FixedPointNumber { .. } => None,
            Type::FixedSizeArray { .. } => match symbol {
                "length" => Some(BuiltIn::Length),
                _ => None,
            },
            Type::Function(ftype) => {
                if ftype.is_externally_visible() {
                    match symbol {
                        "address" if self.language_version >= BuiltIn::FUNCTION_ADDRESS_FROM => {
                            Some(BuiltIn::FunctionAddress)
                        }
                        "selector" => Some(BuiltIn::FunctionSelector),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Type::Integer { .. } => None,
            Type::Literal(LiteralKind::Address) => Self::lookup_member_of_address(symbol, false),
            Type::Literal(_) => None,
            Type::Mapping { .. } => None,
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

    pub(crate) fn lookup_call_option(symbol: &str) -> Option<BuiltIn> {
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
            BuiltIn::Address | BuiltIn::FunctionAddress => Typing::Resolved(self.types.address()),
            BuiltIn::AddressBalance => Typing::Resolved(self.types.uint256()),
            BuiltIn::AddressCode => Typing::Resolved(self.types.bytes_memory()),
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
            BuiltIn::MsgData => Typing::Resolved(self.types.bytes_calldata()),
            BuiltIn::MsgSender => Typing::Resolved(self.types.address()),
            BuiltIn::MsgSig => Typing::Resolved(self.types.bytes4()),
            BuiltIn::MsgValue => Typing::Resolved(self.types.uint256()),
            BuiltIn::ErrorSelector | BuiltIn::EventSelector | BuiltIn::FunctionSelector => {
                Typing::Resolved(self.types.bytes4())
            }
            BuiltIn::TxGasPrice => Typing::Resolved(self.types.uint256()),
            BuiltIn::TxOrigin => Typing::Resolved(self.types.address()),
            BuiltIn::TypeName => Typing::Resolved(self.types.string()),
            BuiltIn::TypeCreationCode => Typing::Resolved(self.types.bytes_memory()),
            BuiltIn::TypeRuntimeCode => Typing::Resolved(self.types.bytes_memory()),
            BuiltIn::TypeInterfaceId => Typing::Resolved(self.types.bytes4()),
            BuiltIn::TypeEnumMin(type_id) | BuiltIn::TypeMin(type_id) => Typing::Resolved(*type_id),
            BuiltIn::TypeEnumMax(type_id) | BuiltIn::TypeMax(type_id) => Typing::Resolved(*type_id),

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
            | BuiltIn::ModifierUnderscore
            | BuiltIn::Mulmod
            | BuiltIn::Msg
            | BuiltIn::Require
            | BuiltIn::Revert
            | BuiltIn::Ripemd160
            | BuiltIn::Selfdestruct
            | BuiltIn::Sha256
            | BuiltIn::StringConcat
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
                // Special case: this is handled in the resolution pass because
                // we need to register the types given as parameters
                Typing::Unresolved
            }
            BuiltIn::AbiEncode => Typing::Resolved(self.types.bytes_memory()),
            BuiltIn::AbiEncodeCall => Typing::Resolved(self.types.bytes_memory()),
            BuiltIn::AbiEncodePacked => Typing::Resolved(self.types.bytes_memory()),
            BuiltIn::AbiEncodeWithSelector => Typing::Resolved(self.types.bytes_memory()),
            BuiltIn::AbiEncodeWithSignature => Typing::Resolved(self.types.bytes_memory()),
            BuiltIn::Addmod => Typing::Resolved(self.types.uint256()),
            BuiltIn::AddressCall => Typing::Resolved(self.types.boolean_bytes_tuple()),
            BuiltIn::AddressCallcode => Typing::Resolved(self.types.boolean_bytes_tuple()),
            BuiltIn::AddressDelegatecall => Typing::Resolved(self.types.boolean_bytes_tuple()),
            BuiltIn::AddressStaticcall => Typing::Resolved(self.types.boolean_bytes_tuple()),
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
            BuiltIn::BytesConcat => Typing::Resolved(self.types.bytes_memory()),
            BuiltIn::Ecrecover => Typing::Resolved(self.types.address()),
            BuiltIn::Gasleft => Typing::Resolved(self.types.uint256()),
            BuiltIn::Keccak256 => Typing::Resolved(self.types.bytes32()),
            BuiltIn::Mulmod => Typing::Resolved(self.types.uint256()),
            BuiltIn::Require => Typing::Resolved(self.types.void()),
            BuiltIn::Revert => Typing::Resolved(self.types.void()),
            BuiltIn::Ripemd160 => Typing::Resolved(self.types.bytes20()),
            BuiltIn::Selfdestruct => Typing::Resolved(self.types.void()),
            BuiltIn::Sha256 => Typing::Resolved(self.types.bytes32()),
            BuiltIn::StringConcat => Typing::Resolved(self.types.string()),
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
