use std::sync::Arc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, DefinitionIds, Reference, Resolution, ScopeId, Typing};
use crate::types::{FunctionType, Type, TypeId, TypeRegistry};

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
    let mut resolution = Resolution::Unresolved;

    for (index, identifier) in identifier_path.iter().enumerate() {
        let symbol = identifier.unparse();
        resolution = if let Some(scope_id) = scope_id {
            if index == 0 {
                // we use lexical resolution only in the first segment of the identifier path
                binder.resolve_in_scope(scope_id, symbol)
            } else {
                binder.resolve_in_scope_as_namespace(scope_id, symbol)
            }
        } else {
            Resolution::Unresolved
        };

        let reference = Reference::new(Arc::clone(identifier), resolution.clone());
        binder.insert_reference(reference);

        resolution = binder.follow_symbol_aliases(resolution);

        // Change the resolution scope to be that of the last resolved
        // definition, so we can resolve the next identifier in the path.
        scope_id = resolution
            .as_definition_id()
            .and_then(|definition_id| find_definition_namespace_scope_id(binder, definition_id));
    }
    resolution
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
    let mut filtered_definitions = DefinitionIds::new();
    for definition_id in definition_ids {
        match binder.find_definition_by_id(definition_id).unwrap() {
            Definition::Function(_) => {
                if let &Typing::Resolved(type_id) = binder.node_typing(definition_id) {
                    let Type::Function(function_type) = types.get_type_by_id(type_id) else {
                        unreachable!("type of function definition is not a function");
                    };
                    if seen_function_types.iter().any(|seen_function_type| {
                        types
                            .function_type_overrides_in_hierarchy(seen_function_type, function_type)
                    }) {
                        // the function type is overriden by some other previously seen definition
                        continue;
                    }
                    seen_function_types.push(function_type);
                }
            }
            Definition::StateVariable(state_variable) => {
                // TODO(validation) SDR[36]: the state variable should have the
                // `override` attribute and the rest of the definitions should
                // be either functions with the correct signature, state
                // variables or private variables or constants.
                //
                // Remember the getter type if present to override functions in
                // bases.
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

/// A member that can occupy an override slot. A function, a modifier, or a
/// public state variable through its getter.
pub(crate) trait Callable {
    /// `None` for a fallback or receive.
    fn name(&self) -> Option<&str>;
    /// A getter is a regular function.
    fn kind(&self) -> ir::FunctionKind;
    /// The function or getter type. `None` when it couldn't be computed.
    fn type_id(&self, binder: &Binder) -> Option<TypeId>;
}

impl Callable for ir::FunctionDefinition {
    fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|name| name.unparse())
    }

    fn kind(&self) -> ir::FunctionKind {
        self.kind
    }

    fn type_id(&self, binder: &Binder) -> Option<TypeId> {
        binder.node_typing(self.id()).as_type_id()
    }
}

impl Callable for ir::StateVariableDefinition {
    fn name(&self) -> Option<&str> {
        Some(self.name.unparse())
    }

    fn kind(&self) -> ir::FunctionKind {
        ir::FunctionKind::Regular
    }

    fn type_id(&self, binder: &Binder) -> Option<TypeId> {
        match binder.find_definition_by_id(self.id()) {
            Some(Definition::StateVariable(definition)) => definition.getter_type_id,
            _ => unreachable!("state variable is not registered as a definition"),
        }
    }
}

/// Whether `overriding` takes the override slot of `overridden`. The two match
/// on name and kind. Regular functions and getters also need indistinguishable
/// parameter lists.
pub(crate) fn overrides(
    binder: &Binder,
    types: &TypeRegistry,
    overriding: &dyn Callable,
    overridden: &dyn Callable,
) -> bool {
    if overriding.kind() != overridden.kind() || overriding.name() != overridden.name() {
        return false;
    }
    match overriding.kind() {
        // Modifiers cannot be overloaded, and a fallback or receive has no
        // selector, so name and kind are enough.
        ir::FunctionKind::Modifier | ir::FunctionKind::Fallback | ir::FunctionKind::Receive => true,
        // A member whose parameters aren't all typed is reported on its own.
        // Treating it as matching anything here would invent an override.
        ir::FunctionKind::Regular => match (overriding.type_id(binder), overridden.type_id(binder))
        {
            (Some(overriding_type_id), Some(overridden_type_id)) => types
                .parameter_lists_are_indistinguishable(
                    parameter_types(types, overriding_type_id),
                    parameter_types(types, overridden_type_id),
                ),
            _ => false,
        },
        ir::FunctionKind::Constructor => {
            unreachable!("constructors take no part in overriding")
        }
    }
}

/// The parameter types of the function type `type_id`.
fn parameter_types(types: &TypeRegistry, type_id: TypeId) -> &[TypeId] {
    let Type::Function(function_type) = types.get_type_by_id(type_id) else {
        unreachable!("type of a function or getter is not a function");
    };
    &function_type.parameter_types
}

/// Given a `Definition`'s `NodeId`, find the scope where we should resolve
/// symbols if the definition acts as a namespace. This is typically used when
/// resolving `MemberAccessExpression`s with a `UserMetatype` operand. Returns
/// `None` if the definition cannot be used as a namespace.
pub(crate) fn find_definition_namespace_scope_id(
    binder: &Binder,
    node_id: NodeId,
) -> Option<ScopeId> {
    match binder.find_definition_by_id(node_id)? {
        Definition::Import(import_definition) => import_definition
            .resolved_file_id
            .as_ref()
            .and_then(|file_id| binder.scope_id_for_file_id(file_id)),
        Definition::Contract(_)
        | Definition::Enum(_)
        | Definition::Interface(_)
        | Definition::Library(_) => {
            // this is a "namespace" lookup
            binder.scope_id_for_node_id(node_id)
        }
        _ => None,
    }
}
