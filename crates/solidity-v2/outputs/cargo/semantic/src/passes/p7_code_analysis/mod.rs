mod built_ins;
mod constant_cycles;

use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::binder::Binder;
use crate::context::FileNodeMapper;

/// This pass hosts analyses that emit diagnostics over the fully resolved
/// program, ie. they can only run after all references have been recorded.
/// It produces no data for later consumption.
pub fn run(
    binder: &Binder,
    language_version: LanguageVersion,
    evm_target: EvmTarget,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    constant_cycles::detect_constant_value_dependency_cycles(binder, diagnostics);

    built_ins::validate_built_in_references(
        binder,
        language_version,
        evm_target,
        file_node_mapper,
        diagnostics,
    );
}
