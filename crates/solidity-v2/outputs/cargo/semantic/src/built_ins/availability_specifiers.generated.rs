// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use slang_solidity_v2_common::evm_targets::{EvmTarget, EvmTargetSpecifier};
use slang_solidity_v2_common::versions::{LanguageVersion, LanguageVersionSpecifier};

use crate::built_ins::InternalBuiltIn;

/// The language version and EVM target ranges `built_in` is defined for, taken
/// from `enabled` / `evm_enabled` in the language definition. `None` on an axis
/// means the built-in is unrestricted along it.
#[allow(clippy::too_many_lines)]
pub(crate) fn built_in_specifiers(
    built_in: InternalBuiltIn,
) -> (Option<LanguageVersionSpecifier>, Option<EvmTargetSpecifier>) {
    match built_in {
        InternalBuiltIn::Blobhash => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_24)),
            Some(EvmTargetSpecifier::from(EvmTarget::Cancun)),
        ),
        InternalBuiltIn::Erc7201 => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_35)),
            None,
        ),
        InternalBuiltIn::AbiEncodeCall => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_11)),
            None,
        ),
        InternalBuiltIn::BlockBasefee => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_7)),
            Some(EvmTargetSpecifier::from(EvmTarget::London)),
        ),
        InternalBuiltIn::BlockBlobbasefee => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_24)),
            Some(EvmTargetSpecifier::from(EvmTarget::Cancun)),
        ),
        InternalBuiltIn::BlockChainid => {
            (None, Some(EvmTargetSpecifier::from(EvmTarget::Istanbul)))
        }
        InternalBuiltIn::BlockPrevrandao => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_18)),
            None,
        ),
        InternalBuiltIn::AddressCodehash => (
            None,
            Some(EvmTargetSpecifier::from(EvmTarget::Constantinople)),
        ),
        InternalBuiltIn::AddressDelegatecall => {
            (None, Some(EvmTargetSpecifier::from(EvmTarget::Homestead)))
        }
        InternalBuiltIn::AddressStaticcall => {
            (None, Some(EvmTargetSpecifier::from(EvmTarget::Byzantium)))
        }
        InternalBuiltIn::TypeEnumMin(_) => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_8)),
            None,
        ),
        InternalBuiltIn::TypeEnumMax(_) => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_8)),
            None,
        ),
        InternalBuiltIn::ErrorSelector => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_4)),
            None,
        ),
        InternalBuiltIn::EventSelector => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_15)),
            None,
        ),
        InternalBuiltIn::Unwrap(_) => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_8)),
            None,
        ),
        InternalBuiltIn::Wrap(_) => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_8)),
            None,
        ),
        InternalBuiltIn::YulBasefee => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_7)),
            Some(EvmTargetSpecifier::from(EvmTarget::London)),
        ),
        InternalBuiltIn::YulBlobbasefee => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_24)),
            Some(EvmTargetSpecifier::from(EvmTarget::Cancun)),
        ),
        InternalBuiltIn::YulBlobhash => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_24)),
            Some(EvmTargetSpecifier::from(EvmTarget::Cancun)),
        ),
        InternalBuiltIn::YulChainid => (None, Some(EvmTargetSpecifier::from(EvmTarget::Istanbul))),
        InternalBuiltIn::YulClz => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_31)),
            Some(EvmTargetSpecifier::from(EvmTarget::Osaka)),
        ),
        InternalBuiltIn::YulCreate2 => (
            None,
            Some(EvmTargetSpecifier::from(EvmTarget::Constantinople)),
        ),
        InternalBuiltIn::YulDifficulty => (None, Some(EvmTargetSpecifier::till(EvmTarget::Paris))),
        InternalBuiltIn::YulExtcodehash => (
            None,
            Some(EvmTargetSpecifier::from(EvmTarget::Constantinople)),
        ),
        InternalBuiltIn::YulMcopy => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_24)),
            Some(EvmTargetSpecifier::from(EvmTarget::Cancun)),
        ),
        InternalBuiltIn::YulPrevrandao => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_18)),
            Some(EvmTargetSpecifier::from(EvmTarget::Paris)),
        ),
        InternalBuiltIn::YulReturndatacopy => {
            (None, Some(EvmTargetSpecifier::from(EvmTarget::Byzantium)))
        }
        InternalBuiltIn::YulReturndatasize => {
            (None, Some(EvmTargetSpecifier::from(EvmTarget::Byzantium)))
        }
        InternalBuiltIn::YulSar => (
            None,
            Some(EvmTargetSpecifier::from(EvmTarget::Constantinople)),
        ),
        InternalBuiltIn::YulSelfbalance => {
            (None, Some(EvmTargetSpecifier::from(EvmTarget::Istanbul)))
        }
        InternalBuiltIn::YulShl => (
            None,
            Some(EvmTargetSpecifier::from(EvmTarget::Constantinople)),
        ),
        InternalBuiltIn::YulShr => (
            None,
            Some(EvmTargetSpecifier::from(EvmTarget::Constantinople)),
        ),
        InternalBuiltIn::YulStaticcall => {
            (None, Some(EvmTargetSpecifier::from(EvmTarget::Byzantium)))
        }
        InternalBuiltIn::YulTload => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_24)),
            Some(EvmTargetSpecifier::from(EvmTarget::Cancun)),
        ),
        InternalBuiltIn::YulTstore => (
            Some(LanguageVersionSpecifier::from(LanguageVersion::V0_8_24)),
            Some(EvmTargetSpecifier::from(EvmTarget::Cancun)),
        ),

        _ => (None, None),
    }
}
