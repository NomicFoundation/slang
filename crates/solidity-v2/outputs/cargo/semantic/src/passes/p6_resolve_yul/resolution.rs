use super::Pass;
use crate::binder::{Definition, Resolution, ScopeId};
use crate::built_ins::BuiltInsResolver;
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
        let resolution = filter_overriden_definitions(
            self.binder,
            self.types,
            self.binder.resolve_in_scope(scope_id, symbol),
        );

        let is_solidity_function = |node_id| {
            matches!(
                self.binder.find_definition_by_id(node_id),
                Some(Definition::Function(_))
            )
        };
        let shadows_built_in = match &resolution {
            Resolution::Definition(node_id) => is_solidity_function(*node_id),
            Resolution::Ambiguous(node_ids) => {
                node_ids.iter().copied().all(is_solidity_function)
            }
            _ => false,
        };

        if resolution == Resolution::Unresolved || shadows_built_in {
            let built_in: Resolution = BuiltInsResolver::lookup_yul_global(symbol).into();
            if built_in != Resolution::Unresolved {
                return built_in;
            }
        }
        resolution
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
