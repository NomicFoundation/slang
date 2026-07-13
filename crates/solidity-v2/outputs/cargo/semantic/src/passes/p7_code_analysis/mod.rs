mod built_ins;
mod cycle_detection;
mod fallback_functions;
mod receive_functions;

use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::binder::Binder;
use crate::context::FileNodeMapper;
use crate::types::TypeRegistry;

/// This pass hosts analyses that emit diagnostics over the fully resolved
/// program, ie. they can only run after all references have been recorded.
/// It produces no data for later consumption.
pub fn run(
    binder: &Binder,
    language_version: LanguageVersion,
    evm_target: EvmTarget,
    file_node_mapper: &FileNodeMapper,
    types: &TypeRegistry,
    diagnostics: &mut DiagnosticCollection,
) {
    cycle_detection::run(binder, types, file_node_mapper, diagnostics);

    built_ins::validate_built_in_references(
        binder,
        language_version,
        evm_target,
        file_node_mapper,
        diagnostics,
    );

    fallback_functions::check_fallback_functions(binder, types, file_node_mapper, diagnostics);
    receive_functions::check_receive_functions(binder, types, file_node_mapper, diagnostics);
}
