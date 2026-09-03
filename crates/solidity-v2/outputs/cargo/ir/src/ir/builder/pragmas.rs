use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::structure::DuplicateAbicoderSpecifier;
use slang_solidity_v2_common::diagnostics::kinds::syntax::{
    IncompatibleVersionPragma, InvalidVersionSpecifier, UnrecognizedExperimentalFeature,
    UnsupportedAbicoderV1, UnsupportedExperimentalSmtChecker, UnsupportedExperimentalSolidity,
};
use slang_solidity_v2_common::utils::strings::{
    decode_escape_sequences, strip_string_literal_quotes,
};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes as input;
use slang_solidity_v2_cst::structured_cst::text_range::TextRange;

use crate::ir::builder::{CstToIrBuilder, Source};
use crate::ir::node_extensions::VersionPragmaSpecifierExtensions;
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

    pub(super) fn build_version_pragma(
        &mut self,
        source: &input::VersionPragma,
    ) -> output::VersionPragma {
        let id = self.next_id(output::NodeKind::VersionPragma);
        let range = source.calculate_text_range().unwrap_or_default();

        let sets = self.build_version_expression_sets(&source.sets);

        let pragma = Arc::new(output::VersionPragmaStruct { id, range, sets });

        // A literal we could not read admits every version, so a pragma already
        // reported as invalid does not also report an incompatibility.
        if !pragma.sets.is_empty() && !pragma.matches_version(self.language_version) {
            self.report(
                source,
                IncompatibleVersionPragma {
                    language_version: self.language_version,
                },
            );
        }

        pragma
    }

    fn build_version_expression_sets(
        &mut self,
        source: &input::VersionPragmaExpressionSets,
    ) -> output::VersionPragmaExpressionSets {
        source
            .elements
            .iter()
            .map(|set| self.build_version_expression_set(set))
            .collect()
    }

    /// Builds one alternative of a pragma: the expressions written next to each
    /// other, which all have to hold at once.
    fn build_version_expression_set(
        &mut self,
        source: &input::VersionPragmaExpressionSet,
    ) -> output::VersionPragmaExpressionSet {
        let mut comparators = Vec::with_capacity(source.elements.len());

        for expression in &source.elements {
            match expression {
                input::VersionPragmaExpression::VersionPragmaTerm(term) => {
                    comparators.push(self.build_version_comparator(term));
                }
                input::VersionPragmaExpression::VersionPragmaRange(range) => {
                    comparators.push(self.build_version_bound(
                        &range.start,
                        output::VersionPragmaOperator::GreaterThanEqual,
                    ));
                    comparators.push(self.build_version_bound(
                        &range.end,
                        output::VersionPragmaOperator::LessThanEqual,
                    ));
                }
            }
        }

        comparators.into()
    }

    /// Builds one end of a hyphen range as the comparator it stands for.
    fn build_version_bound(
        &mut self,
        source: &input::VersionPragmaSpecifier,
        operator: output::VersionPragmaOperator,
    ) -> output::VersionPragmaComparator {
        let id = self.next_id(output::NodeKind::VersionPragmaComparator);
        let range = source.calculate_text_range().unwrap_or_default();
        let specifier = self.build_version(source);

        Arc::new(output::VersionPragmaComparatorStruct {
            id,
            range,
            operator,
            specifier,
        })
    }

    fn build_version_comparator(
        &mut self,
        source: &input::VersionPragmaTerm,
    ) -> output::VersionPragmaComparator {
        let id = self.next_id(output::NodeKind::VersionPragmaComparator);
        let range = source.calculate_text_range().unwrap_or_default();
        // A literal written with no operator means the same as one written
        // with `=`, so the IR always spells the operator out.
        let operator = source
            .operator
            .as_ref()
            .map_or(output::VersionPragmaOperator::Equal, |operator| {
                Self::build_version_comparator_operator(operator)
            });
        let specifier = self.build_version(&source.specifier);

        Arc::new(output::VersionPragmaComparatorStruct {
            id,
            range,
            operator,
            specifier,
        })
    }

    fn build_version_comparator_operator(
        source: &input::VersionPragmaOperator,
    ) -> output::VersionPragmaOperator {
        match source {
            input::VersionPragmaOperator::PragmaCaret(_) => output::VersionPragmaOperator::Caret,
            input::VersionPragmaOperator::PragmaTilde(_) => output::VersionPragmaOperator::Tilde,
            input::VersionPragmaOperator::PragmaEqual(_) => output::VersionPragmaOperator::Equal,
            input::VersionPragmaOperator::PragmaLessThan(_) => {
                output::VersionPragmaOperator::LessThan
            }
            input::VersionPragmaOperator::PragmaLessThanEqual(_) => {
                output::VersionPragmaOperator::LessThanEqual
            }
            input::VersionPragmaOperator::PragmaGreaterThan(_) => {
                output::VersionPragmaOperator::GreaterThan
            }
            input::VersionPragmaOperator::PragmaGreaterThanEqual(_) => {
                output::VersionPragmaOperator::GreaterThanEqual
            }
        }
    }

    /// Reads a version literal, written either as a run of `.`-separated
    /// specifiers or as a quoted string carrying the same, into [`output::VersionPragmaSpecifier`].
    fn build_version(
        &mut self,
        source: &input::VersionPragmaSpecifier,
    ) -> output::VersionPragmaSpecifier {
        match source {
            input::VersionPragmaSpecifier::VersionPragmaComponents(literal) => {
                let components: Vec<String> = literal
                    .elements
                    .iter()
                    .map(|specifier| self.unparse_range(specifier.range.clone()))
                    .collect();

                self.resolve_version(&components, literal)
            }
            input::VersionPragmaSpecifier::PragmaStringLiteral(literal) => {
                let text = self.unparse_range(literal.range.clone());
                let decoded = decode_escape_sequences(strip_string_literal_quotes(&text));
                let decoded = String::from_utf8_lossy(&decoded);

                let components: Vec<String> = decoded.split('.').map(str::to_owned).collect();

                self.resolve_version(&components, literal)
            }
        }
    }

    /// Resolves the components of a version literal, reporting the literal when
    /// they do not spell out a version. The grammar is looser than the version
    /// syntax it spells out, so a specifier may mix digits and wildcards (`1x`),
    /// and a quoted literal may hold anything at all (`"0.8.beta"`).
    ///
    /// A reported literal still resolves, keeping the components as written so
    /// the surrounding pragma holds what the source said. Those components admit
    /// every version, so the one mistake is not also reported as an incompatible
    /// pragma.
    fn resolve_version(
        &mut self,
        components: &[String],
        source: &dyn TextRange,
    ) -> output::VersionPragmaSpecifier {
        let version: output::VersionPragmaSpecifier = components
            .iter()
            .map(|component| build_version_component(component))
            .collect();

        if !version.is_valid() {
            self.report(
                source,
                InvalidVersionSpecifier {
                    specifier: components.join("."),
                },
            );
        }

        version
    }
}

/// Reads one `.`-separated component of a version literal.
///
/// Leading zeros carry no meaning, so `08` is `8`. A run of wildcard characters
/// collapses into a single wildcard, matching how `solc` reads a literal
/// character by character: `xx` and `**` mean the same as `x`. Anything else —
/// the `1x` of `pragma solidity 1x;`, the `beta` of `pragma solidity "0.8.beta";`
/// — reads as neither, and the specifier holding it is reported.
fn build_version_component(component: &str) -> output::VersionPragmaComponent {
    if component.is_empty() {
        return output::VersionPragmaComponent::Unrecognized;
    }

    if component
        .chars()
        .all(|character| matches!(character, '*' | 'x' | 'X'))
    {
        return output::VersionPragmaComponent::Wildcard;
    }

    component.parse().map_or(
        output::VersionPragmaComponent::Unrecognized,
        output::VersionPragmaComponent::Number,
    )
}
