use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::structure::DuplicateAbicoderSpecifier;
use slang_solidity_v2_common::diagnostics::kinds::syntax::{
    UnrecognizedExperimentalFeature, UnsupportedAbicoderV1, UnsupportedExperimentalSmtChecker,
    UnsupportedExperimentalSolidity,
};
use slang_solidity_v2_common::utils::strings::strip_string_literal_quotes;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes as input;
use slang_solidity_v2_cst::structured_cst::text_range::TextRange;

use crate::ir::builder::{CstToIrBuilder, Source};
use crate::ir::nodes as output;

impl<S: Source> CstToIrBuilder<'_, S> {
    pub(super) fn build_experimental_pragma(
        &mut self,
        source: &input::ExperimentalPragma,
    ) -> output::ExperimentalPragma {
        let id = self.next_id(output::NodeKind::ExperimentalPragma);
        let range = source.calculate_text_range().unwrap_or_default();
        let feature = self.build_experimental_feature(&source.feature);

        match feature {
            output::ExperimentalFeature::SMTChecker
            | output::ExperimentalFeature::Solidity
            | output::ExperimentalFeature::Unrecognized => {
                // Unsupported/unrecognized features. Skip reporting redundant diagnostics.
            }
            output::ExperimentalFeature::ABIEncoderV2 => {
                if self.is_abicoder_specified {
                    self.report(&source.feature, DuplicateAbicoderSpecifier);
                } else {
                    self.is_abicoder_specified = true;
                }
            }
        }

        Arc::new(output::ExperimentalPragmaStruct { id, range, feature })
    }

    fn build_experimental_feature(
        &mut self,
        source: &input::ExperimentalFeature,
    ) -> output::ExperimentalFeature {
        match source {
            input::ExperimentalFeature::ABIEncoderV2Keyword(_) => {
                output::ExperimentalFeature::ABIEncoderV2
            }
            input::ExperimentalFeature::SMTCheckerKeyword(node) => {
                self.report(node, UnsupportedExperimentalSmtChecker);
                output::ExperimentalFeature::SMTChecker
            }
            input::ExperimentalFeature::SolidityKeyword(node) => {
                if self.language_version >= LanguageVersion::V0_8_21 {
                    self.report(node, UnsupportedExperimentalSolidity);
                }
                output::ExperimentalFeature::Solidity
            }
            input::ExperimentalFeature::PragmaStringLiteral(literal) => {
                let text = self.unparse_range(literal.range.clone());
                match strip_string_literal_quotes(&text) {
                    "ABIEncoderV2" => output::ExperimentalFeature::ABIEncoderV2,
                    "SMTChecker" => {
                        self.report(literal, UnsupportedExperimentalSmtChecker);
                        output::ExperimentalFeature::SMTChecker
                    }
                    "solidity" if self.language_version >= LanguageVersion::V0_8_21 => {
                        self.report(literal, UnsupportedExperimentalSolidity);
                        output::ExperimentalFeature::Solidity
                    }
                    _ => {
                        self.report(literal, UnrecognizedExperimentalFeature);
                        output::ExperimentalFeature::Unrecognized
                    }
                }
            }
        }
    }

    pub(super) fn build_abicoder_pragma(
        &mut self,
        source: &input::AbicoderPragma,
    ) -> output::AbicoderPragma {
        let id = self.next_id(output::NodeKind::AbicoderPragma);
        let range = source.calculate_text_range().unwrap_or_default();
        let version = self.build_abicoder_version(&source.version);

        match version {
            output::AbicoderVersion::V1 => {
                // V1 is unsupported. Skip reporting redundant diagnostics.
            }
            output::AbicoderVersion::V2 => {
                if self.is_abicoder_specified {
                    self.report(&source.version, DuplicateAbicoderSpecifier);
                } else {
                    self.is_abicoder_specified = true;
                }
            }
        }

        Arc::new(output::AbicoderPragmaStruct { id, range, version })
    }

    fn build_abicoder_version(
        &mut self,
        source: &input::AbicoderVersion,
    ) -> output::AbicoderVersion {
        match source {
            input::AbicoderVersion::AbicoderV1Keyword(node) => {
                self.report(node, UnsupportedAbicoderV1);
                output::AbicoderVersion::V1
            }
            input::AbicoderVersion::AbicoderV2Keyword(_) => output::AbicoderVersion::V2,
        }
    }
}
