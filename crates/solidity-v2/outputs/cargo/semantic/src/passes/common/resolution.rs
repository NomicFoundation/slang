use std::sync::Arc;

use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, ImportDefinition, Reference, Resolution, ScopeId, Typing};
use crate::types::{FunctionType, Type, TypeRegistry};

/// Resolves an `IdentifierPath` starting from the given scope, creating
/// `Reference`s for all its elements. It will follow through
/// contracts/interfaces/libraries as well as imports and treat them as
/// namespaces. Returns the resolution of the last reference.
pub(crate) fn resolve_identifier_path_in_scope(
    binder: &mut Binder,
    identifier_path: &ir::IdentifierPath,
    starting_scope_id: ScopeId,
) -> Resolution {
    let mut scope_id = Some(starting_scope_id);
    let mut use_lexical_resolution = true;
    let mut last_resolution: Resolution = Resolution::Unresolved;

    for identifier in identifier_path {
        let symbol = identifier.unparse();
        let resolution = if let Some(scope_id) = scope_id {
            if use_lexical_resolution {
                binder.resolve_in_scope(scope_id, symbol)
            } else {
                binder.resolve_in_scope_as_namespace(scope_id, symbol)
            }
        } else {
            Resolution::Unresolved
        };

        let reference = Reference::new(Arc::clone(identifier), resolution.clone());
        binder.insert_reference(reference);

        // Unless we used namespace resolution and in order to continue
        // resolving the identifier path, we should ensure we've followed
        // through any symbol alias (ie. import deconstruction symbol). This
        // is not needed for namespaced resolution because there cannot be
        // import directives inside contracts, interfaces or libraries which
        // changes the lookup mode (see below).
        let resolution = if use_lexical_resolution {
            binder.follow_symbol_aliases(&resolution)
        } else {
            resolution
        };

        // recurse into file scopes pointed by the resolved definition
        // to resolve the next identifier in the path
        scope_id = resolution
            .as_definition_id()
            .and_then(|node_id| binder.find_definition_by_id(node_id))
            .and_then(|definition| match definition {
                Definition::Import(ImportDefinition {
                    resolved_file_id, ..
                }) => resolved_file_id
                    .as_ref()
                    .and_then(|resolved_file_id| binder.scope_id_for_file_id(resolved_file_id)),
                Definition::Contract(_) | Definition::Interface(_) | Definition::Library(_) => {
                    use_lexical_resolution = false;
                    binder.scope_id_for_node_id(definition.node_id())
                }
                _ => None,
            });

        last_resolution = resolution;
    }
    last_resolution
}

/// When a symbol resolves to an ambiguous set of Solidity functions
/// (overloads/virtuals), drop the ones overridden by a previously seen
/// definition. This reads function typing information, so it requires
/// `p3_type_definitions` to have run.
pub(crate) fn filter_overriden_definitions(
    binder: &Binder,
    types: &TypeRegistry,
    resolution: Resolution,
) -> Resolution {
    // TODO: it may be possible/desirable to use information procured in
    // `p4_compute_linearisations` here.
    let Resolution::Ambiguous(definition_ids) = resolution else {
        return resolution;
    };
    let mut seen_function_types: Vec<&FunctionType> = Vec::new();
    let mut filtered_definitions = Vec::new();
    for definition_id in definition_ids {
        match binder.find_definition_by_id(definition_id).unwrap() {
            Definition::Function(_) => {
                if let Typing::Resolved(type_id) = binder.node_typing(definition_id) {
                    let Type::Function(function_type) = types.get_type_by_id(type_id) else {
                        unreachable!("type of function definition is not a function");
                    };
                    if seen_function_types.iter().any(|seen_function_type| {
                        types.function_type_overrides(seen_function_type, function_type)
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
                    let Type::Function(getter_type) = types.get_type_by_id(getter_type_id) else {
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
