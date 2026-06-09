use slang_solidity_v2_common::diagnostics::DiagnosticCollection;

use crate::binder::Binder;

mod constant_cycles;

/// This pass hosts analyses that emit diagnostics over the fully resolved
/// program, ie. they can only run after all references have been recorded.
/// It produces no data for later consumption.
pub fn run(binder: &Binder, diagnostics: &mut DiagnosticCollection) {
    constant_cycles::detect_constant_value_dependency_cycles(binder, diagnostics);
}
