use super::binder::{Binder, Definition, Typing};
use super::types::{
    AddressType, ArrayType, BytesType, DataLocation, LiteralKind, Type, TypeId, TypeRegistry,
    UserDefinedValueType,
};

#[path = "internal.generated.rs"]
mod internal;

pub use internal::InternalBuiltIn;

pub(crate) struct BuiltInsResolver<'a> {
    binder: &'a Binder,
    types: &'a TypeRegistry,
}

impl<'a> BuiltInsResolver<'a> {
    pub(crate) fn new(binder: &'a Binder, types: &'a TypeRegistry) -> Self {
        Self { binder, types }
    }

    pub(crate) fn lookup_global(symbol: &str) -> Option<InternalBuiltIn> {
        match symbol {
            "abi" => Some(InternalBuiltIn::Abi),
            "addmod" => Some(InternalBuiltIn::Addmod),
            "assert" => Some(InternalBuiltIn::Assert),
            "blobhash" => Some(InternalBuiltIn::Blobhash),
            "block" => Some(InternalBuiltIn::Block),
            "blockhash" => Some(InternalBuiltIn::Blockhash),
            "ecrecover" => Some(InternalBuiltIn::Ecrecover),
            "erc7201" => Some(InternalBuiltIn::Erc7201),
            "gasleft" => Some(InternalBuiltIn::Gasleft),
            "keccak256" => Some(InternalBuiltIn::Keccak256),
            "msg" => Some(InternalBuiltIn::Msg),
            "mulmod" => Some(InternalBuiltIn::Mulmod),
            "require" => Some(InternalBuiltIn::Require),
            "revert" => Some(InternalBuiltIn::Revert),
            "ripemd160" => Some(InternalBuiltIn::Ripemd160),
            "selfdestruct" => Some(InternalBuiltIn::Selfdestruct),
            "sha256" => Some(InternalBuiltIn::Sha256),
            "tx" => Some(InternalBuiltIn::Tx),
            _ => None,
        }
    }

    pub(crate) fn lookup_yul_global(symbol: &str) -> Option<InternalBuiltIn> {
        // TODO(validation) SDR[18]: Yul built-in function names should be considered reserved words.
        // They're currently parsed as Identifiers to facilitate their manipulation.
        match symbol {
            "add" => Some(InternalBuiltIn::YulAdd),
            "addmod" => Some(InternalBuiltIn::YulAddmod),
            "address" => Some(InternalBuiltIn::YulAddress),
            "and" => Some(InternalBuiltIn::YulAnd),
            "balance" => Some(InternalBuiltIn::YulBalance),
            "basefee" => Some(InternalBuiltIn::YulBasefee),
            "blobbasefee" => Some(InternalBuiltIn::YulBlobbasefee),
            "blobhash" => Some(InternalBuiltIn::YulBlobhash),
            "blockhash" => Some(InternalBuiltIn::YulBlockhash),
            "byte" => Some(InternalBuiltIn::YulByte),
            "callcode" => Some(InternalBuiltIn::YulCallcode),
            "calldatacopy" => Some(InternalBuiltIn::YulCalldatacopy),
            "calldataload" => Some(InternalBuiltIn::YulCalldataload),
            "calldatasize" => Some(InternalBuiltIn::YulCalldatasize),
            "caller" => Some(InternalBuiltIn::YulCaller),
            "call" => Some(InternalBuiltIn::YulCall),
            "callvalue" => Some(InternalBuiltIn::YulCallvalue),
            "chainid" => Some(InternalBuiltIn::YulChainid),
            "clz" => Some(InternalBuiltIn::YulClz),
            "codecopy" => Some(InternalBuiltIn::YulCodecopy),
            "codesize" => Some(InternalBuiltIn::YulCodesize),
            "coinbase" => Some(InternalBuiltIn::YulCoinbase),
            "create" => Some(InternalBuiltIn::YulCreate),
            "create2" => Some(InternalBuiltIn::YulCreate2),
            "delegatecall" => Some(InternalBuiltIn::YulDelegatecall),
            "difficulty" => Some(InternalBuiltIn::YulDifficulty),
            "div" => Some(InternalBuiltIn::YulDiv),
            "eq" => Some(InternalBuiltIn::YulEq),
            "exp" => Some(InternalBuiltIn::YulExp),
            "extcodecopy" => Some(InternalBuiltIn::YulExtcodecopy),
            "extcodehash" => Some(InternalBuiltIn::YulExtcodehash),
            "extcodesize" => Some(InternalBuiltIn::YulExtcodesize),
            "gas" => Some(InternalBuiltIn::YulGas),
            "gaslimit" => Some(InternalBuiltIn::YulGaslimit),
            "gasprice" => Some(InternalBuiltIn::YulGasprice),
            "gt" => Some(InternalBuiltIn::YulGt),
            "invalid" => Some(InternalBuiltIn::YulInvalid),
            "iszero" => Some(InternalBuiltIn::YulIszero),
            "keccak256" => Some(InternalBuiltIn::YulKeccak256),
            "log0" | "log1" | "log2" | "log3" | "log4" => {
                let arity = symbol.as_bytes()[3] - b'0';
                Some(InternalBuiltIn::YulLog(arity))
            }
            "lt" => Some(InternalBuiltIn::YulLt),
            "mcopy" => Some(InternalBuiltIn::YulMcopy),
            "mload" => Some(InternalBuiltIn::YulMload),
            "mod" => Some(InternalBuiltIn::YulMod),
            "msize" => Some(InternalBuiltIn::YulMsize),
            "mstore8" => Some(InternalBuiltIn::YulMstore8),
            "mstore" => Some(InternalBuiltIn::YulMstore),
            "mul" => Some(InternalBuiltIn::YulMul),
            "mulmod" => Some(InternalBuiltIn::YulMulmod),
            "not" => Some(InternalBuiltIn::YulNot),
            "number" => Some(InternalBuiltIn::YulNumber),
            "or" => Some(InternalBuiltIn::YulOr),
            "origin" => Some(InternalBuiltIn::YulOrigin),
            "pop" => Some(InternalBuiltIn::YulPop),
            "prevrandao" => Some(InternalBuiltIn::YulPrevrandao),
            "return" => Some(InternalBuiltIn::YulReturn),
            "returndatacopy" => Some(InternalBuiltIn::YulReturndatacopy),
            "returndatasize" => Some(InternalBuiltIn::YulReturndatasize),
            "revert" => Some(InternalBuiltIn::YulRevert),
            "sar" => Some(InternalBuiltIn::YulSar),
            "sdiv" => Some(InternalBuiltIn::YulSdiv),
            "selfbalance" => Some(InternalBuiltIn::YulSelfbalance),
            "selfdestruct" => Some(InternalBuiltIn::YulSelfdestruct),
            "sgt" => Some(InternalBuiltIn::YulSgt),
            "shl" => Some(InternalBuiltIn::YulShl),
            "shr" => Some(InternalBuiltIn::YulShr),
            "signextend" => Some(InternalBuiltIn::YulSignextend),
            "sload" => Some(InternalBuiltIn::YulSload),
            "slt" => Some(InternalBuiltIn::YulSlt),
            "smod" => Some(InternalBuiltIn::YulSmod),
            "sstore" => Some(InternalBuiltIn::YulSstore),
            "staticcall" => Some(InternalBuiltIn::YulStaticcall),
            "stop" => Some(InternalBuiltIn::YulStop),
            "sub" => Some(InternalBuiltIn::YulSub),
            "timestamp" => Some(InternalBuiltIn::YulTimestamp),
            "tload" => Some(InternalBuiltIn::YulTload),
            "tstore" => Some(InternalBuiltIn::YulTstore),
            "xor" => Some(InternalBuiltIn::YulXor),

            _ => None,
        }
    }

    pub(crate) fn lookup_member_of(
        &self,
        built_in: &InternalBuiltIn,
        symbol: &str,
    ) -> Option<InternalBuiltIn> {
        match built_in {
            InternalBuiltIn::Abi => match symbol {
                "decode" => Some(InternalBuiltIn::AbiDecode),
                "encode" => Some(InternalBuiltIn::AbiEncode),
                "encodeCall" => Some(InternalBuiltIn::AbiEncodeCall),
                "encodePacked" => Some(InternalBuiltIn::AbiEncodePacked),
                "encodeWithSelector" => Some(InternalBuiltIn::AbiEncodeWithSelector),
                "encodeWithSignature" => Some(InternalBuiltIn::AbiEncodeWithSignature),
                _ => None,
            },
            InternalBuiltIn::Block => match symbol {
                "basefee" => Some(InternalBuiltIn::BlockBasefee),
                "blobbasefee" => Some(InternalBuiltIn::BlockBlobbasefee),
                "chainid" => Some(InternalBuiltIn::BlockChainid),
                "coinbase" => Some(InternalBuiltIn::BlockCoinbase),
                "difficulty" => Some(InternalBuiltIn::BlockDifficulty),
                "gaslimit" => Some(InternalBuiltIn::BlockGaslimit),
                "number" => Some(InternalBuiltIn::BlockNumber),
                "prevrandao" => Some(InternalBuiltIn::BlockPrevrandao),
                "timestamp" => Some(InternalBuiltIn::BlockTimestamp),
                _ => None,
            },
            InternalBuiltIn::Tx => match symbol {
                "gasprice" => Some(InternalBuiltIn::TxGasPrice),
                "origin" => Some(InternalBuiltIn::TxOrigin),
                _ => None,
            },
            InternalBuiltIn::Type(type_id) => match self.types.get_type_by_id(*type_id) {
                Type::Contract(_) | Type::Library(_) => match symbol {
                    "name" => Some(InternalBuiltIn::TypeName),
                    "creationCode" => Some(InternalBuiltIn::TypeCreationCode),
                    "runtimeCode" => Some(InternalBuiltIn::TypeRuntimeCode),
                    _ => None,
                },
                Type::Interface(_) => match symbol {
                    "name" => Some(InternalBuiltIn::TypeName),
                    "interfaceId" => Some(InternalBuiltIn::TypeInterfaceId),
                    _ => None,
                },
                Type::Enum(_) => match symbol {
                    "min" => Some(InternalBuiltIn::TypeEnumMin(*type_id)),
                    "max" => Some(InternalBuiltIn::TypeEnumMax(*type_id)),
                    _ => None,
                },
                Type::Integer(_) => match symbol {
                    "min" => Some(InternalBuiltIn::TypeMin(*type_id)),
                    "max" => Some(InternalBuiltIn::TypeMax(*type_id)),
                    _ => None,
                },
                _ => None,
            },
            InternalBuiltIn::Msg => match symbol {
                "data" => Some(InternalBuiltIn::MsgData),
                "sender" => Some(InternalBuiltIn::MsgSender),
                "sig" => Some(InternalBuiltIn::MsgSig),
                "value" => Some(InternalBuiltIn::MsgValue),
                _ => None,
            },
            _ => None,
        }
    }

    pub(crate) fn lookup_member_of_user_definition(
        definition: &Definition,
        symbol: &str,
    ) -> Option<InternalBuiltIn> {
        match definition {
            Definition::Error(_) => match symbol {
                "selector" => Some(InternalBuiltIn::ErrorSelector),
                _ => None,
            },
            Definition::Event(_) => match symbol {
                "selector" => Some(InternalBuiltIn::EventSelector),
                _ => None,
            },
            Definition::UserDefinedValueType(_) => match symbol {
                "wrap" => Some(InternalBuiltIn::Wrap(definition.node_id())),
                "unwrap" => Some(InternalBuiltIn::Unwrap(definition.node_id())),
                _ => None,
            },
            Definition::Function(_) => match symbol {
                "selector" => Some(InternalBuiltIn::FunctionSelector),
                _ => None,
            },
            _ => None,
        }
    }

    pub(crate) fn lookup_member_of_meta_type(
        parent_type: &Type,
        symbol: &str,
    ) -> Option<InternalBuiltIn> {
        match parent_type {
            Type::Bytes(_) => match symbol {
                "concat" => Some(InternalBuiltIn::BytesConcat),
                _ => None,
            },
            Type::String(_) => match symbol {
                "concat" => Some(InternalBuiltIn::StringConcat),
                _ => None,
            },
            _ => None,
        }
    }

    fn lookup_member_of_address(symbol: &str, payable: bool) -> Option<InternalBuiltIn> {
        match symbol {
            "balance" => Some(InternalBuiltIn::AddressBalance),
            "code" => Some(InternalBuiltIn::AddressCode),
            "codehash" => Some(InternalBuiltIn::AddressCodehash),
            "call" => Some(InternalBuiltIn::AddressCall),
            "callcode" if !payable => Some(InternalBuiltIn::AddressCallcode),
            "delegatecall" => Some(InternalBuiltIn::AddressDelegatecall),
            "send" if payable => Some(InternalBuiltIn::AddressSend),
            "staticcall" => Some(InternalBuiltIn::AddressStaticcall),
            "transfer" if payable => Some(InternalBuiltIn::AddressTransfer),
            _ => None,
        }
    }

    pub(crate) fn lookup_member_of_type_id(
        &self,
        type_id: TypeId,
        symbol: &str,
    ) -> Option<InternalBuiltIn> {
        let type_ = self.types.get_type_by_id(type_id);
        match type_ {
            Type::Address(AddressType { is_payable }) => {
                Self::lookup_member_of_address(symbol, *is_payable)
            }
            Type::Array(ArrayType { element_type, .. }) => match symbol {
                "length" => Some(InternalBuiltIn::Length),
                "pop" => Some(InternalBuiltIn::ArrayPop),
                "push" => Some(InternalBuiltIn::ArrayPush(*element_type)),
                _ => None,
            },
            Type::Boolean => None,
            Type::ByteArray(_) => match symbol {
                "length" => Some(InternalBuiltIn::Length),
                _ => None,
            },
            Type::Bytes(BytesType { location }) => match symbol {
                "length" => Some(InternalBuiltIn::Length),
                "pop" if *location == DataLocation::Storage => Some(InternalBuiltIn::ArrayPop),
                "push" if *location == DataLocation::Storage => {
                    Some(InternalBuiltIn::ArrayPush(self.types.uint8()))
                }
                _ => None,
            },
            Type::Contract(_) | Type::Interface(_) | Type::Library(_) => None,
            Type::Enum(_) => None,
            Type::FixedPointNumber(_) => None,
            Type::FixedSizeArray(_) => match symbol {
                "length" => Some(InternalBuiltIn::Length),
                _ => None,
            },
            Type::Function(ftype) => {
                if ftype.is_externally_visible() {
                    match symbol {
                        "address" => Some(InternalBuiltIn::FunctionAddress),
                        "selector" => Some(InternalBuiltIn::FunctionSelector),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Type::Integer(_) => None,
            Type::Literal(LiteralKind::Address { .. }) => {
                Self::lookup_member_of_address(symbol, false)
            }
            Type::Literal(_) => None,
            Type::Mapping(_) => None,
            Type::String(_) => match symbol {
                "length" => Some(InternalBuiltIn::Length),
                _ => None,
            },
            Type::Struct(_) => None,
            Type::Tuple(_) => None,
            Type::UserDefinedValue(_) => None,
            Type::Void => None,
        }
    }

    pub(crate) fn lookup_yul_suffix(
        definition: &Definition,
        symbol: &str,
    ) -> Option<InternalBuiltIn> {
        match definition {
            Definition::StateVariable(_) => match symbol {
                "offset" => Some(InternalBuiltIn::YulOffset),
                "slot" => Some(InternalBuiltIn::YulSlot),
                _ => None,
            },
            Definition::Parameter(_) | Definition::Variable(_) => match symbol {
                "address" => Some(InternalBuiltIn::YulAddressField),
                "length" => Some(InternalBuiltIn::YulLengthField),
                "offset" => Some(InternalBuiltIn::YulOffset),
                "selector" => Some(InternalBuiltIn::YulSelector),
                "slot" => Some(InternalBuiltIn::YulSlot),
                _ => None,
            },
            _ => None,
        }
    }

    pub(crate) fn lookup_call_option(symbol: &str) -> Option<InternalBuiltIn> {
        match symbol {
            "gas" => Some(InternalBuiltIn::CallOptionGas),
            "salt" => Some(InternalBuiltIn::CallOptionSalt),
            "value" => Some(InternalBuiltIn::CallOptionValue),
            _ => None,
        }
    }

    pub(crate) fn typing_of(&self, built_in: &InternalBuiltIn) -> Typing {
        match built_in {
            // variables and members
            InternalBuiltIn::Address | InternalBuiltIn::FunctionAddress => {
                Typing::Resolved(self.types.address())
            }
            InternalBuiltIn::AddressBalance => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::AddressCode => Typing::Resolved(self.types.bytes_memory()),
            InternalBuiltIn::AddressCodehash => Typing::Resolved(self.types.bytes32()),
            InternalBuiltIn::BlockBasefee => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::BlockBlobbasefee => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::BlockChainid => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::BlockCoinbase => Typing::Resolved(self.types.address_payable()),
            InternalBuiltIn::BlockDifficulty => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::BlockGaslimit => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::BlockNumber => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::BlockPrevrandao => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::BlockTimestamp => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::Length => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::MsgData => Typing::Resolved(self.types.bytes_calldata()),
            InternalBuiltIn::MsgSender => Typing::Resolved(self.types.address()),
            InternalBuiltIn::MsgSig => Typing::Resolved(self.types.bytes4()),
            InternalBuiltIn::MsgValue => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::ErrorSelector
            | InternalBuiltIn::EventSelector
            | InternalBuiltIn::FunctionSelector => Typing::Resolved(self.types.bytes4()),
            InternalBuiltIn::TxGasPrice => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::TxOrigin => Typing::Resolved(self.types.address()),
            InternalBuiltIn::TypeName => Typing::Resolved(self.types.string_memory()),
            InternalBuiltIn::TypeCreationCode => Typing::Resolved(self.types.bytes_memory()),
            InternalBuiltIn::TypeRuntimeCode => Typing::Resolved(self.types.bytes_memory()),
            InternalBuiltIn::TypeInterfaceId => Typing::Resolved(self.types.bytes4()),
            InternalBuiltIn::TypeEnumMin(type_id) | InternalBuiltIn::TypeMin(type_id) => {
                Typing::Resolved(*type_id)
            }
            InternalBuiltIn::TypeEnumMax(type_id) | InternalBuiltIn::TypeMax(type_id) => {
                Typing::Resolved(*type_id)
            }

            // built-in functions and types
            InternalBuiltIn::Abi
            | InternalBuiltIn::AbiDecode
            | InternalBuiltIn::AbiEncode
            | InternalBuiltIn::AbiEncodeCall
            | InternalBuiltIn::AbiEncodePacked
            | InternalBuiltIn::AbiEncodeWithSelector
            | InternalBuiltIn::AbiEncodeWithSignature
            | InternalBuiltIn::AddressCall
            | InternalBuiltIn::AddressCallcode
            | InternalBuiltIn::AddressDelegatecall
            | InternalBuiltIn::AddressSend
            | InternalBuiltIn::AddressStaticcall
            | InternalBuiltIn::AddressTransfer
            | InternalBuiltIn::Addmod
            | InternalBuiltIn::ArrayPop
            | InternalBuiltIn::ArrayPush(_)
            | InternalBuiltIn::Assert
            | InternalBuiltIn::Blobhash
            | InternalBuiltIn::Block
            | InternalBuiltIn::Blockhash
            | InternalBuiltIn::BytesConcat
            | InternalBuiltIn::Ecrecover
            | InternalBuiltIn::Erc7201
            | InternalBuiltIn::Gasleft
            | InternalBuiltIn::Keccak256
            | InternalBuiltIn::ModifierUnderscore
            | InternalBuiltIn::Mulmod
            | InternalBuiltIn::Msg
            | InternalBuiltIn::Require
            | InternalBuiltIn::Revert
            | InternalBuiltIn::Ripemd160
            | InternalBuiltIn::Selfdestruct
            | InternalBuiltIn::Sha256
            | InternalBuiltIn::StringConcat
            | InternalBuiltIn::Tx
            | InternalBuiltIn::Wrap(_)
            | InternalBuiltIn::Unwrap(_) => Typing::BuiltIn(*built_in),

            // others
            _ => Typing::Unresolved,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn typing_of_function_call(
        &self,
        built_in: &InternalBuiltIn,
        argument_typings: &[Typing],
    ) -> Typing {
        match built_in {
            InternalBuiltIn::AbiDecode => {
                // Special case: this is handled in the resolution pass because
                // we need to register the types given as parameters
                Typing::Unresolved
            }
            InternalBuiltIn::AbiEncode => Typing::Resolved(self.types.bytes_memory()),
            InternalBuiltIn::AbiEncodeCall => Typing::Resolved(self.types.bytes_memory()),
            InternalBuiltIn::AbiEncodePacked => Typing::Resolved(self.types.bytes_memory()),
            InternalBuiltIn::AbiEncodeWithSelector => Typing::Resolved(self.types.bytes_memory()),
            InternalBuiltIn::AbiEncodeWithSignature => Typing::Resolved(self.types.bytes_memory()),
            InternalBuiltIn::Addmod => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::AddressCall => Typing::Resolved(self.types.boolean_bytes_tuple()),
            InternalBuiltIn::AddressCallcode => Typing::Resolved(self.types.boolean_bytes_tuple()),
            InternalBuiltIn::AddressDelegatecall => {
                Typing::Resolved(self.types.boolean_bytes_tuple())
            }
            InternalBuiltIn::AddressStaticcall => {
                Typing::Resolved(self.types.boolean_bytes_tuple())
            }
            InternalBuiltIn::ArrayPush(type_id) => {
                if argument_typings.is_empty() {
                    Typing::Resolved(*type_id)
                } else {
                    Typing::Resolved(self.types.void())
                }
            }
            InternalBuiltIn::ArrayPop => Typing::Resolved(self.types.void()),
            InternalBuiltIn::Assert => Typing::Resolved(self.types.void()),
            InternalBuiltIn::Blobhash => Typing::Resolved(self.types.bytes32()),
            InternalBuiltIn::Blockhash => Typing::Resolved(self.types.bytes32()),
            InternalBuiltIn::BytesConcat => Typing::Resolved(self.types.bytes_memory()),
            InternalBuiltIn::Ecrecover => Typing::Resolved(self.types.address()),
            InternalBuiltIn::Erc7201 => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::Gasleft => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::Keccak256 => Typing::Resolved(self.types.bytes32()),
            InternalBuiltIn::Mulmod => Typing::Resolved(self.types.uint256()),
            InternalBuiltIn::Require => Typing::Resolved(self.types.void()),
            InternalBuiltIn::Revert => Typing::Resolved(self.types.void()),
            InternalBuiltIn::Ripemd160 => Typing::Resolved(self.types.bytes20()),
            InternalBuiltIn::Selfdestruct => Typing::Resolved(self.types.void()),
            InternalBuiltIn::Sha256 => Typing::Resolved(self.types.bytes32()),
            InternalBuiltIn::StringConcat => Typing::Resolved(self.types.string_memory()),
            InternalBuiltIn::Unwrap(definition_id) => {
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
            InternalBuiltIn::Wrap(definition_id) => {
                let Some(Definition::UserDefinedValueType(_)) =
                    self.binder.find_definition_by_id(*definition_id)
                else {
                    unreachable!("definition bound to wrap built-in is not a UDVT");
                };
                Typing::Resolved(
                    self.types
                        .find_type(&Type::UserDefinedValue(UserDefinedValueType {
                            definition_id: *definition_id,
                        }))
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
