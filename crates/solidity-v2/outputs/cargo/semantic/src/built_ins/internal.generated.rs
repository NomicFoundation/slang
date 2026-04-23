// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use slang_solidity_v2_common::built_ins::BuiltIn as PublicBuiltIn;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::{LanguageVersion, LanguageVersionSpecifier};

use crate::types::TypeId;

/// This is the internal representation of a built-in resolution and typing.
/// It's richer than the public enum defined in the `common` crate because it
/// contains additional information about the specific built-in use. Eg.
/// `.push()` or `AraryPush` needs to carry the type of the original array to
/// correctly resolve the type in a function call context.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BuiltIn {
    Abi,
    Addmod,
    Assert,
    Blobhash,
    Block,
    Blockhash,
    Ecrecover,
    Gasleft,
    Keccak256,
    Msg,
    Mulmod,
    Require,
    Revert,
    Ripemd160,
    Selfdestruct,
    Sha256,
    Tx,

    AbiDecode,
    AbiEncode,
    AbiEncodeCall,
    AbiEncodePacked,
    AbiEncodeWithSelector,
    AbiEncodeWithSignature,

    BlockBasefee,
    BlockBlobbasefee,
    BlockChainid,
    BlockCoinbase,
    BlockDifficulty,
    BlockGaslimit,
    BlockNumber,
    BlockPrevrandao,
    BlockTimestamp,

    MsgData,
    MsgSender,
    MsgSig,
    MsgValue,

    TxGasPrice,
    TxOrigin,

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

    Type(TypeId),
    TypeName,
    TypeCreationCode,
    TypeRuntimeCode,
    TypeInterfaceId,
    TypeEnumMin(TypeId),
    TypeEnumMax(TypeId),
    TypeMin(TypeId),
    TypeMax(TypeId),

    ArrayPop,
    ArrayPush(TypeId),
    BytesConcat,
    CallOptionGas,
    CallOptionSalt,
    CallOptionValue,
    ErrorOrPanic,
    Length,
    ModifierUnderscore,
    ErrorSelector,
    EventSelector,
    FunctionAddress,
    FunctionSelector,
    StringConcat,
    Unwrap(NodeId),
    Wrap(NodeId),

    YulAdd,
    YulAddmod,
    YulAddress,
    YulAnd,
    YulBalance,
    YulBasefee,
    YulBlobbasefee,
    YulBlobhash,
    YulBlockhash,
    YulByte,
    YulCall,
    YulCallcode,
    YulCalldatacopy,
    YulCalldataload,
    YulCalldatasize,
    YulCaller,
    YulCallvalue,
    YulChainid,
    YulClz,
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
    YulKeccak256,
    YulLog(u8),
    YulLt,
    YulMcopy,
    YulMload,
    YulMod,
    YulMsize,
    YulMstore,
    YulMstore8,
    YulMul,
    YulMulmod,
    YulNot,
    YulNumber,
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
    YulSelfbalance,
    YulSelfdestruct,
    YulSgt,
    YulShl,
    YulShr,
    YulSignextend,
    YulSload,
    YulSlt,
    YulSmod,
    YulSstore,
    YulStaticcall,
    YulStop,
    YulSub,
    YulTimestamp,
    YulTload,
    YulTstore,
    YulXor,

    YulAddressField,
    YulLengthField,
    YulOffset,
    YulSelector,
    YulSlot,
}

impl BuiltIn {
    /// Converts this internal built-in representation to the public enum type
    /// defined in the `common` crate.
    #[allow(clippy::too_many_lines)]
    pub fn to_public(&self) -> PublicBuiltIn {
        match self {
            Self::Abi => PublicBuiltIn::Abi,
            Self::Addmod => PublicBuiltIn::Addmod,
            Self::Assert => PublicBuiltIn::Assert,
            Self::Blobhash => PublicBuiltIn::Blobhash,
            Self::Block => PublicBuiltIn::Block,
            Self::Blockhash => PublicBuiltIn::Blockhash,
            Self::Ecrecover => PublicBuiltIn::Ecrecover,
            Self::Gasleft => PublicBuiltIn::Gasleft,
            Self::Keccak256 => PublicBuiltIn::Keccak256,
            Self::Msg => PublicBuiltIn::Msg,
            Self::Mulmod => PublicBuiltIn::Mulmod,
            Self::Require => PublicBuiltIn::Require,
            Self::Revert => PublicBuiltIn::Revert,
            Self::Ripemd160 => PublicBuiltIn::Ripemd160,
            Self::Selfdestruct => PublicBuiltIn::Selfdestruct,
            Self::Sha256 => PublicBuiltIn::Sha256,
            Self::Tx => PublicBuiltIn::Tx,

            Self::AbiDecode => PublicBuiltIn::AbiDecode,
            Self::AbiEncode => PublicBuiltIn::AbiEncode,
            Self::AbiEncodeCall => PublicBuiltIn::AbiEncodeCall,
            Self::AbiEncodePacked => PublicBuiltIn::AbiEncodePacked,
            Self::AbiEncodeWithSelector => PublicBuiltIn::AbiEncodeWithSelector,
            Self::AbiEncodeWithSignature => PublicBuiltIn::AbiEncodeWithSignature,

            Self::BlockBasefee => PublicBuiltIn::BlockBasefee,
            Self::BlockBlobbasefee => PublicBuiltIn::BlockBlobbasefee,
            Self::BlockChainid => PublicBuiltIn::BlockChainid,
            Self::BlockCoinbase => PublicBuiltIn::BlockCoinbase,
            Self::BlockDifficulty => PublicBuiltIn::BlockDifficulty,
            Self::BlockGaslimit => PublicBuiltIn::BlockGaslimit,
            Self::BlockNumber => PublicBuiltIn::BlockNumber,
            Self::BlockPrevrandao => PublicBuiltIn::BlockPrevrandao,
            Self::BlockTimestamp => PublicBuiltIn::BlockTimestamp,

            Self::MsgData => PublicBuiltIn::MsgData,
            Self::MsgSender => PublicBuiltIn::MsgSender,
            Self::MsgSig => PublicBuiltIn::MsgSig,
            Self::MsgValue => PublicBuiltIn::MsgValue,

            Self::TxGasPrice => PublicBuiltIn::TxGasPrice,
            Self::TxOrigin => PublicBuiltIn::TxOrigin,

            Self::Address => PublicBuiltIn::Address,
            Self::AddressBalance => PublicBuiltIn::AddressBalance,
            Self::AddressCall => PublicBuiltIn::AddressCall,
            Self::AddressCallcode => PublicBuiltIn::AddressCallcode,
            Self::AddressCode => PublicBuiltIn::AddressCode,
            Self::AddressCodehash => PublicBuiltIn::AddressCodehash,
            Self::AddressDelegatecall => PublicBuiltIn::AddressDelegatecall,
            Self::AddressSend => PublicBuiltIn::AddressSend,
            Self::AddressStaticcall => PublicBuiltIn::AddressStaticcall,
            Self::AddressTransfer => PublicBuiltIn::AddressTransfer,

            Self::Type(_) => PublicBuiltIn::Type,
            Self::TypeName => PublicBuiltIn::TypeName,
            Self::TypeCreationCode => PublicBuiltIn::TypeCreationCode,
            Self::TypeRuntimeCode => PublicBuiltIn::TypeRuntimeCode,
            Self::TypeInterfaceId => PublicBuiltIn::TypeInterfaceId,
            Self::TypeEnumMin(_) => PublicBuiltIn::TypeEnumMin,
            Self::TypeEnumMax(_) => PublicBuiltIn::TypeEnumMax,
            Self::TypeMin(_) => PublicBuiltIn::TypeMin,
            Self::TypeMax(_) => PublicBuiltIn::TypeMax,

            Self::ArrayPop => PublicBuiltIn::ArrayPop,
            Self::ArrayPush(_) => PublicBuiltIn::ArrayPush,
            Self::BytesConcat => PublicBuiltIn::BytesConcat,
            Self::CallOptionGas => PublicBuiltIn::CallOptionGas,
            Self::CallOptionSalt => PublicBuiltIn::CallOptionSalt,
            Self::CallOptionValue => PublicBuiltIn::CallOptionValue,
            Self::ErrorOrPanic => PublicBuiltIn::ErrorOrPanic,
            Self::Length => PublicBuiltIn::Length,
            Self::ModifierUnderscore => PublicBuiltIn::ModifierUnderscore,
            Self::ErrorSelector => PublicBuiltIn::ErrorSelector,
            Self::EventSelector => PublicBuiltIn::EventSelector,
            Self::FunctionAddress => PublicBuiltIn::FunctionAddress,
            Self::FunctionSelector => PublicBuiltIn::FunctionSelector,
            Self::StringConcat => PublicBuiltIn::StringConcat,
            Self::Unwrap(_) => PublicBuiltIn::Unwrap,
            Self::Wrap(_) => PublicBuiltIn::Wrap,

            Self::YulAdd => PublicBuiltIn::YulAdd,
            Self::YulAddmod => PublicBuiltIn::YulAddmod,
            Self::YulAddress => PublicBuiltIn::YulAddress,
            Self::YulAnd => PublicBuiltIn::YulAnd,
            Self::YulBalance => PublicBuiltIn::YulBalance,
            Self::YulBasefee => PublicBuiltIn::YulBasefee,
            Self::YulBlobbasefee => PublicBuiltIn::YulBlobbasefee,
            Self::YulBlobhash => PublicBuiltIn::YulBlobhash,
            Self::YulBlockhash => PublicBuiltIn::YulBlockhash,
            Self::YulByte => PublicBuiltIn::YulByte,
            Self::YulCall => PublicBuiltIn::YulCall,
            Self::YulCallcode => PublicBuiltIn::YulCallcode,
            Self::YulCalldatacopy => PublicBuiltIn::YulCalldatacopy,
            Self::YulCalldataload => PublicBuiltIn::YulCalldataload,
            Self::YulCalldatasize => PublicBuiltIn::YulCalldatasize,
            Self::YulCaller => PublicBuiltIn::YulCaller,
            Self::YulCallvalue => PublicBuiltIn::YulCallvalue,
            Self::YulChainid => PublicBuiltIn::YulChainid,
            Self::YulClz => PublicBuiltIn::YulClz,
            Self::YulCodecopy => PublicBuiltIn::YulCodecopy,
            Self::YulCodesize => PublicBuiltIn::YulCodesize,
            Self::YulCoinbase => PublicBuiltIn::YulCoinbase,
            Self::YulCreate => PublicBuiltIn::YulCreate,
            Self::YulCreate2 => PublicBuiltIn::YulCreate2,
            Self::YulDelegatecall => PublicBuiltIn::YulDelegatecall,
            Self::YulDifficulty => PublicBuiltIn::YulDifficulty,
            Self::YulDiv => PublicBuiltIn::YulDiv,
            Self::YulEq => PublicBuiltIn::YulEq,
            Self::YulExp => PublicBuiltIn::YulExp,
            Self::YulExtcodecopy => PublicBuiltIn::YulExtcodecopy,
            Self::YulExtcodehash => PublicBuiltIn::YulExtcodehash,
            Self::YulExtcodesize => PublicBuiltIn::YulExtcodesize,
            Self::YulGas => PublicBuiltIn::YulGas,
            Self::YulGaslimit => PublicBuiltIn::YulGaslimit,
            Self::YulGasprice => PublicBuiltIn::YulGasprice,
            Self::YulGt => PublicBuiltIn::YulGt,
            Self::YulInvalid => PublicBuiltIn::YulInvalid,
            Self::YulIszero => PublicBuiltIn::YulIszero,
            Self::YulKeccak256 => PublicBuiltIn::YulKeccak256,
            Self::YulLog(_) => PublicBuiltIn::YulLog,
            Self::YulLt => PublicBuiltIn::YulLt,
            Self::YulMcopy => PublicBuiltIn::YulMcopy,
            Self::YulMload => PublicBuiltIn::YulMload,
            Self::YulMod => PublicBuiltIn::YulMod,
            Self::YulMsize => PublicBuiltIn::YulMsize,
            Self::YulMstore => PublicBuiltIn::YulMstore,
            Self::YulMstore8 => PublicBuiltIn::YulMstore8,
            Self::YulMul => PublicBuiltIn::YulMul,
            Self::YulMulmod => PublicBuiltIn::YulMulmod,
            Self::YulNot => PublicBuiltIn::YulNot,
            Self::YulNumber => PublicBuiltIn::YulNumber,
            Self::YulOr => PublicBuiltIn::YulOr,
            Self::YulOrigin => PublicBuiltIn::YulOrigin,
            Self::YulPop => PublicBuiltIn::YulPop,
            Self::YulPrevrandao => PublicBuiltIn::YulPrevrandao,
            Self::YulReturn => PublicBuiltIn::YulReturn,
            Self::YulReturndatacopy => PublicBuiltIn::YulReturndatacopy,
            Self::YulReturndatasize => PublicBuiltIn::YulReturndatasize,
            Self::YulRevert => PublicBuiltIn::YulRevert,
            Self::YulSar => PublicBuiltIn::YulSar,
            Self::YulSdiv => PublicBuiltIn::YulSdiv,
            Self::YulSelfbalance => PublicBuiltIn::YulSelfbalance,
            Self::YulSelfdestruct => PublicBuiltIn::YulSelfdestruct,
            Self::YulSgt => PublicBuiltIn::YulSgt,
            Self::YulShl => PublicBuiltIn::YulShl,
            Self::YulShr => PublicBuiltIn::YulShr,
            Self::YulSignextend => PublicBuiltIn::YulSignextend,
            Self::YulSload => PublicBuiltIn::YulSload,
            Self::YulSlt => PublicBuiltIn::YulSlt,
            Self::YulSmod => PublicBuiltIn::YulSmod,
            Self::YulSstore => PublicBuiltIn::YulSstore,
            Self::YulStaticcall => PublicBuiltIn::YulStaticcall,
            Self::YulStop => PublicBuiltIn::YulStop,
            Self::YulSub => PublicBuiltIn::YulSub,
            Self::YulTimestamp => PublicBuiltIn::YulTimestamp,
            Self::YulTload => PublicBuiltIn::YulTload,
            Self::YulTstore => PublicBuiltIn::YulTstore,
            Self::YulXor => PublicBuiltIn::YulXor,

            Self::YulAddressField => PublicBuiltIn::YulAddressField,
            Self::YulLengthField => PublicBuiltIn::YulLengthField,
            Self::YulOffset => PublicBuiltIn::YulOffset,
            Self::YulSelector => PublicBuiltIn::YulSelector,
            Self::YulSlot => PublicBuiltIn::YulSlot,
        }
    }

    // Version ranges for built-ins introduced or deprecated in the supported versions

    pub(super) const BLOBHASH_VERSIONS: LanguageVersionSpecifier = LanguageVersionSpecifier::From {
        from: LanguageVersion::V0_8_24,
    };
    pub(super) const ABI_ENCODE_CALL_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_11,
        };
    pub(super) const BLOCK_BASEFEE_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_7,
        };
    pub(super) const BLOCK_BLOBBASEFEE_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_24,
        };
    pub(super) const BLOCK_PREVRANDAO_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_18,
        };
    pub(super) const TYPE_ENUM_MIN_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_8,
        };
    pub(super) const TYPE_ENUM_MAX_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_8,
        };
    pub(super) const ERROR_SELECTOR_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_4,
        };
    pub(super) const EVENT_SELECTOR_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_15,
        };
    pub(super) const FUNCTION_ADDRESS_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_2,
        };
    pub(super) const UNWRAP_VERSIONS: LanguageVersionSpecifier = LanguageVersionSpecifier::From {
        from: LanguageVersion::V0_8_8,
    };
    pub(super) const WRAP_VERSIONS: LanguageVersionSpecifier = LanguageVersionSpecifier::From {
        from: LanguageVersion::V0_8_8,
    };
    pub(super) const YUL_BASEFEE_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_7,
        };
    pub(super) const YUL_BLOBBASEFEE_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_24,
        };
    pub(super) const YUL_BLOBHASH_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_24,
        };
    pub(super) const YUL_CLZ_VERSIONS: LanguageVersionSpecifier = LanguageVersionSpecifier::From {
        from: LanguageVersion::V0_8_31,
    };
    pub(super) const YUL_DIFFICULTY_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::Till {
            till: LanguageVersion::V0_8_18,
        };
    pub(super) const YUL_MCOPY_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_24,
        };
    pub(super) const YUL_PREVRANDAO_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_18,
        };
    pub(super) const YUL_TLOAD_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_24,
        };
    pub(super) const YUL_TSTORE_VERSIONS: LanguageVersionSpecifier =
        LanguageVersionSpecifier::From {
            from: LanguageVersion::V0_8_24,
        };
}
