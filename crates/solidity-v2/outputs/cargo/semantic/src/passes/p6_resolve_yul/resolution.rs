use super::Pass;
use crate::binder::{Definition, Resolution, ScopeId, Typing};
use crate::built_ins::BuiltInsResolver;
use crate::context::SemanticFile;
use crate::types::{FunctionType, Type};

impl<F: SemanticFile> Pass<'_, F> {
    // This is a "top-level" resolution method for symbols in a Yul context.
    pub(super) fn resolve_symbol_in_yul_scope(
        &self,
        scope_id: ScopeId,
        symbol: &str,
    ) -> Resolution {
        let resolution =
            self.filter_overriden_definitions(self.binder.resolve_in_scope(scope_id, symbol));
        if resolution == Resolution::Unresolved {
            BuiltInsResolver::lookup_yul_global(symbol).into()
        } else {
            resolution
        }
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

    // When a Yul identifier resolves to an ambiguous set of Solidity functions
    // (overloads/virtuals), drop the ones overridden by a previously seen
    // definition. This is the only place the pass reads typing information,
    // hence the dependency on `p3_type_definitions` having run.
    fn filter_overriden_definitions(&self, resolution: Resolution) -> Resolution {
        let Resolution::Ambiguous(definition_ids) = resolution else {
            return resolution;
        };
        let mut seen_function_types: Vec<&FunctionType> = Vec::new();
        let mut filtered_definitions = Vec::new();
        for definition_id in definition_ids {
            match self.binder.find_definition_by_id(definition_id).unwrap() {
                Definition::Function(_) => {
                    if let Typing::Resolved(type_id) = self.binder.node_typing(definition_id) {
                        let Type::Function(function_type) = self.types.get_type_by_id(type_id)
                        else {
                            unreachable!("type of function definition is not a function");
                        };
                        if seen_function_types.iter().any(|seen_function_type| {
                            self.types
                                .function_type_overrides(seen_function_type, function_type)
                        }) {
                            // the function type is overriden by some other previously seen definition
                            continue;
                        }
                        seen_function_types.push(function_type);
                    }
                }
                Definition::StateVariable(state_variable) => {
                    // remember the getter type if present to override functions
                    // in bases
                    if let Some(getter_type_id) = state_variable.getter_type_id {
                        let Type::Function(getter_type) = self.types.get_type_by_id(getter_type_id)
                        else {
                            unreachable!("getter function type is not a function")
                        };
                        seen_function_types.push(getter_type);
                    }
                }
                _ => {}
            }
            filtered_definitions.push(definition_id);
        }
        Resolution::from(filtered_definitions)
    }
}
