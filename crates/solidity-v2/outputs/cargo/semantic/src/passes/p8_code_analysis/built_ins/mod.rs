#[path = "validate_built_ins.generated.rs"]
mod validate_built_ins;

use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;
use validate_built_ins::BuiltInValidator;

use crate::binder::{Binder, Resolution};
use crate::context::FileNodeMapper;

/// Iterates over all resolved references and validates any that resolve to a
/// built-in against the configured language version and EVM target.
pub(crate) fn validate_built_in_references(
    binder: &Binder,
    language_version: LanguageVersion,
    evm_target: EvmTarget,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    let mut validator =
        BuiltInValidator::new(language_version, evm_target, file_node_mapper, diagnostics);

    for (node_id, reference) in binder.references() {
        if let Resolution::BuiltIn(built_in) = &reference.resolution {
            validator.validate(*built_in, *node_id, &reference.identifier.range);
        }
    }
}
