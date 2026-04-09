use std::rc::Rc;

use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, ImportDefinition, Reference, Resolution, ScopeId};

/// Resolves an `IdentifierPath` starting from the given scope, creating
/// `Reference`s for all its elements. It will follow through
/// contracts/interfaces/libraries as well as imports and treat them as
/// namespaces. Returns the resolution of the last reference.
pub(super) fn resolve_identifier_path_in_scope(
    binder: &mut Binder,
    identifier_path: &ir::IdentifierPath,
    starting_scope_id: ScopeId,
) -> Resolution {
    let mut scope_id = Some(starting_scope_id);
    let mut use_lexical_resolution = true;
    let mut last_resolution: Resolution = Resolution::Unresolved;

    for identifier in identifier_path {
        let symbol = identifier.string_id;
        let resolution = if let Some(scope_id) = scope_id {
            if use_lexical_resolution {
                binder.resolve_in_scope(scope_id, symbol)
            } else {
                binder.resolve_in_scope_as_namespace(scope_id, symbol)
            }
        } else {
            Resolution::Unresolved
        };

        let reference = Reference::new(Rc::clone(identifier), resolution.clone());
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
