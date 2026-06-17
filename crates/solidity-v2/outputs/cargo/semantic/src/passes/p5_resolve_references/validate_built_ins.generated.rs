// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::ops::Range;

use slang_solidity_v2_common::diagnostics::kinds::resolution::{
    IncompatibleBuiltInTarget, IncompatibleBuiltInVersion,
};
use slang_solidity_v2_common::evm_targets::{EvmTarget, EvmTargetSpecifier};
use slang_solidity_v2_common::versions::{LanguageVersion, LanguageVersionSpecifier};

use super::Pass;
use crate::built_ins::InternalBuiltIn;

impl Pass<'_> {
    /// Emits diagnostics if `built_in` is not compatible with the pass's
    /// configured language version and/or EVM target. `range` locates the
    /// diagnostic at the referencing identifier.
    #[allow(clippy::too_many_lines)]
    pub(super) fn validate_built_in(&mut self, built_in: InternalBuiltIn, range: &Range<usize>) {
        match built_in {
            InternalBuiltIn::Blobhash => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }
            InternalBuiltIn::Erc7201 => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_35),
                );
            }
            InternalBuiltIn::AbiEncodeCall => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_11),
                );
            }
            InternalBuiltIn::BlockBasefee => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_7),
                );
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::London));
            }
            InternalBuiltIn::BlockBlobbasefee => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }
            InternalBuiltIn::BlockChainid => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Istanbul));
            }
            InternalBuiltIn::BlockDifficulty => {
                self.check_target(range, EvmTargetSpecifier::till(EvmTarget::Paris));
            }
            InternalBuiltIn::BlockPrevrandao => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_18),
                );
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Paris));
            }
            InternalBuiltIn::AddressCodehash => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Constantinople));
            }
            InternalBuiltIn::AddressDelegatecall => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Homestead));
            }
            InternalBuiltIn::AddressStaticcall => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Byzantium));
            }
            InternalBuiltIn::TypeEnumMin(_) => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
                );
            }
            InternalBuiltIn::TypeEnumMax(_) => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
                );
            }
            InternalBuiltIn::ErrorSelector => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_4),
                );
            }
            InternalBuiltIn::EventSelector => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_15),
                );
            }
            InternalBuiltIn::Unwrap(_) => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
                );
            }
            InternalBuiltIn::Wrap(_) => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
                );
            }
            InternalBuiltIn::YulBasefee => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_7),
                );
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::London));
            }
            InternalBuiltIn::YulBlobbasefee => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }
            InternalBuiltIn::YulBlobhash => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }
            InternalBuiltIn::YulChainid => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Istanbul));
            }
            InternalBuiltIn::YulClz => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_31),
                );
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Osaka));
            }
            InternalBuiltIn::YulCreate2 => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Constantinople));
            }
            InternalBuiltIn::YulDifficulty => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::till(LanguageVersion::V0_8_18),
                );
                self.check_target(range, EvmTargetSpecifier::till(EvmTarget::Paris));
            }
            InternalBuiltIn::YulExtcodehash => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Constantinople));
            }
            InternalBuiltIn::YulMcopy => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }
            InternalBuiltIn::YulPrevrandao => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_18),
                );
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Paris));
            }
            InternalBuiltIn::YulReturndatacopy => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Byzantium));
            }
            InternalBuiltIn::YulReturndatasize => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Byzantium));
            }
            InternalBuiltIn::YulSar => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Constantinople));
            }
            InternalBuiltIn::YulSelfbalance => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Istanbul));
            }
            InternalBuiltIn::YulShl => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Constantinople));
            }
            InternalBuiltIn::YulShr => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Constantinople));
            }
            InternalBuiltIn::YulStaticcall => {
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Byzantium));
            }
            InternalBuiltIn::YulTload => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }
            InternalBuiltIn::YulTstore => {
                self.check_version(
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }

            _ => {}
        }
    }

    fn check_version(&mut self, range: &Range<usize>, compatible_in: LanguageVersionSpecifier) {
        if !compatible_in.contains(self.language_version) {
            self.diagnostics.push(
                self.file_id.to_owned(),
                range.clone(),
                IncompatibleBuiltInVersion { compatible_in },
            );
        }
    }

    fn check_target(&mut self, range: &Range<usize>, compatible_in: EvmTargetSpecifier) {
        if !compatible_in.contains(self.evm_target) {
            self.diagnostics.push(
                self.file_id.to_owned(),
                range.clone(),
                IncompatibleBuiltInTarget { compatible_in },
            );
        }
    }
}
