use super::Pass;
use crate::binder::{Resolution, ScopeId};
use crate::built_ins::{BuiltInsResolver, is_built_in_available};
use crate::passes::common::filter_overriden_definitions;

impl Pass<'_> {
    // Records a Solidity definition referenced from within the assembly block.
    // Yul definitions (functions/parameters/variables) and non-`Definition`
    // resolutions (built-ins, ambiguous, unresolved) are ignored, and each
    // Solidity definition is recorded at most once.
    pub(super) fn record_solidity_reference(&mut self, resolution: &Resolution) {
        let Resolution::Definition(node_id) = resolution else {
            return;
        };
        let is_solidity = self
            .binder
            .find_definition_by_id(*node_id)
            .is_some_and(|definition| !definition.is_yul());
        if is_solidity && !self.solidity_references.contains(node_id) {
            self.solidity_references.push(*node_id);
        }
    }

    // This is a "top-level" resolution method for symbols in a Yul context.
    pub(super) fn resolve_symbol_in_yul_scope(
        &self,
        scope_id: ScopeId,
        symbol: &str,
    ) -> Resolution {
        let built_in = BuiltInsResolver::lookup_yul_global(symbol);

        // An *available* built-in resolves first, since strictly speaking its
        // name is a reserved keyword. Declaring it is reported as a built-in
        // redeclaration, so no user definition can legally be in scope under
        // that name anyway.
        if built_in.is_some_and(|built_in| {
            is_built_in_available(built_in, self.language_version, self.evm_target)
        }) {
            return built_in.into();
        }

        // A built-in that isn't available for the current version/target doesn't
        // reserve its name, so it must not shadow a user definition: before
        // Cancun, `let mcopy := 1` is a legal declaration and a later `mcopy`
        // refers to that variable, not to the built-in.
        //
        // Falling back to the unavailable built-in keeps the more precise
        // "introduced in <version/target>" diagnostic for a reference that has
        // no declaration in scope.
        filter_overriden_definitions(
            self.binder,
            self.types,
            self.binder.resolve_in_scope(scope_id, symbol),
        )
        .or_else(|| built_in.into())
    }

    pub(super) fn resolve_symbol_in_enclosing_solidity_scope(&self, symbol: &str) -> Resolution {
        filter_overriden_definitions(
            self.binder,
            self.types,
            self.binder
                .resolve_in_scope(self.current_solidity_scope_id(), symbol),
        )
    }

    pub(super) fn resolve_yul_suffix(
        &self,
        symbol: &str,
        parent_resolution: &Resolution,
    ) -> Resolution {
        match parent_resolution {
            Resolution::Definition(node_id) => {
                if let Some(definition) = self.binder.find_definition_by_id(*node_id) {
                    BuiltInsResolver::lookup_yul_suffix(definition, symbol).into()
                } else {
                    Resolution::Unresolved
                }
            }
            Resolution::Unresolved | Resolution::Ambiguous(_) | Resolution::BuiltIn(_) => {
                Resolution::Unresolved
            }
        }
    }
}
