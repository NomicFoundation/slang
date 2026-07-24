// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::ops::Range;

use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::resolution::{
    IncompatibleBuiltInTarget, IncompatibleBuiltInVersion,
};
use slang_solidity_v2_common::evm_targets::{EvmTarget, EvmTargetSpecifier};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::{LanguageVersion, LanguageVersionSpecifier};

use crate::built_ins::InternalBuiltIn;
use crate::context::FileNodeMapper;

pub(super) struct BuiltInValidator<'a> {
    language_version: LanguageVersion,
    evm_target: EvmTarget,
    file_node_mapper: &'a FileNodeMapper,
    diagnostics: &'a mut DiagnosticCollection,
}

impl<'a> BuiltInValidator<'a> {
    pub(super) fn new(
        language_version: LanguageVersion,
        evm_target: EvmTarget,
        file_node_mapper: &'a FileNodeMapper,
        diagnostics: &'a mut DiagnosticCollection,
    ) -> Self {
        Self {
            language_version,
            evm_target,
            file_node_mapper,
            diagnostics,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn validate(
        &mut self,
        built_in: InternalBuiltIn,
        node_id: NodeId,
        range: &Range<usize>,
    ) {
        match built_in {
            InternalBuiltIn::Blobhash => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }
            InternalBuiltIn::Erc7201 => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_35),
                );
            }
            InternalBuiltIn::AbiEncodeCall => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_11),
                );
            }
            InternalBuiltIn::BlockBasefee => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_7),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::from(EvmTarget::London));
            }
            InternalBuiltIn::BlockBlobbasefee => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }
            InternalBuiltIn::BlockChainid => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Istanbul),
                );
            }
            InternalBuiltIn::BlockDifficulty => {
                self.check_target(node_id, range, EvmTargetSpecifier::till(EvmTarget::Paris));
            }
            InternalBuiltIn::BlockPrevrandao => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_18),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::from(EvmTarget::Paris));
            }
            InternalBuiltIn::AddressCodehash => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Constantinople),
                );
            }
            InternalBuiltIn::AddressDelegatecall => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Homestead),
                );
            }
            InternalBuiltIn::AddressStaticcall => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Byzantium),
                );
            }
            InternalBuiltIn::TypeEnumMin(_) => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
                );
            }
            InternalBuiltIn::TypeEnumMax(_) => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
                );
            }
            InternalBuiltIn::ErrorSelector => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_4),
                );
            }
            InternalBuiltIn::EventSelector => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_15),
                );
            }
            InternalBuiltIn::Unwrap(_) => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
                );
            }
            InternalBuiltIn::Wrap(_) => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
                );
            }
            InternalBuiltIn::YulBasefee => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_7),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::from(EvmTarget::London));
            }
            InternalBuiltIn::YulBlobbasefee => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }
            InternalBuiltIn::YulBlobhash => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }
            InternalBuiltIn::YulChainid => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Istanbul),
                );
            }
            InternalBuiltIn::YulClz => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_31),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::from(EvmTarget::Osaka));
            }
            InternalBuiltIn::YulCreate2 => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Constantinople),
                );
            }
            InternalBuiltIn::YulDifficulty => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::till(LanguageVersion::V0_8_18),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::till(EvmTarget::Paris));
            }
            InternalBuiltIn::YulExtcodehash => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Constantinople),
                );
            }
            InternalBuiltIn::YulMcopy => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }
            InternalBuiltIn::YulPrevrandao => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_18),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::from(EvmTarget::Paris));
            }
            InternalBuiltIn::YulReturndatacopy => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Byzantium),
                );
            }
            InternalBuiltIn::YulReturndatasize => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Byzantium),
                );
            }
            InternalBuiltIn::YulSar => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Constantinople),
                );
            }
            InternalBuiltIn::YulSelfbalance => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Istanbul),
                );
            }
            InternalBuiltIn::YulShl => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Constantinople),
                );
            }
            InternalBuiltIn::YulShr => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Constantinople),
                );
            }
            InternalBuiltIn::YulStaticcall => {
                self.check_target(
                    node_id,
                    range,
                    EvmTargetSpecifier::from(EvmTarget::Byzantium),
                );
            }
            InternalBuiltIn::YulTload => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }
            InternalBuiltIn::YulTstore => {
                self.check_version(
                    node_id,
                    range,
                    LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
                );
                self.check_target(node_id, range, EvmTargetSpecifier::from(EvmTarget::Cancun));
            }

            _ => {}
        }
    }

    fn check_version(
        &mut self,
        node_id: NodeId,
        range: &Range<usize>,
        compatible_in: LanguageVersionSpecifier,
    ) {
        if !compatible_in.contains(self.language_version) {
            let file_id = self.file_node_mapper.file_id_from_node_id(node_id);
            self.diagnostics.push(
                file_id.to_owned(),
                range.clone(),
                IncompatibleBuiltInVersion { compatible_in },
            );
        }
    }

    fn check_target(
        &mut self,
        node_id: NodeId,
        range: &Range<usize>,
        compatible_in: EvmTargetSpecifier,
    ) {
        if !compatible_in.contains(self.evm_target) {
            let file_id = self.file_node_mapper.file_id_from_node_id(node_id);
            self.diagnostics.push(
                file_id.to_owned(),
                range.clone(),
                IncompatibleBuiltInTarget { compatible_in },
            );
        }
    }
}
