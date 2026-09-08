use std::sync::Arc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, DefinitionIds, Reference, Resolution, ScopeId, Typing};
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

/// Whether `overriding` overrides `overridden`: they share a name and their
/// signatures are in an override relationship, or they are unnamed functions
/// of the same kind, whatever their signatures: a contract dispatches at most
/// one fallback and one receive function, so the kind alone identifies them.
pub(crate) fn function_overrides(
    binder: &Binder,
    types: &TypeRegistry,
    overriding: &ir::FunctionDefinition,
    overridden: &ir::FunctionDefinition,
) -> bool {
    match (&overriding.name, &overridden.name) {
        (None, None) => overriding.kind == overridden.kind,
        (Some(name), Some(other_name)) => {
            debug_assert!(
                overriding.kind == overridden.kind && overriding.kind == ir::FunctionKind::Regular,
                "compared functions are both regular"
            );
            if name.unparse() != other_name.unparse() {
                return false;
            }
            let overriding_type_id = binder.node_typing(overriding.id()).as_type_id();
            let overridden_type_id = binder.node_typing(overridden.id()).as_type_id();
            match (overriding_type_id, overridden_type_id) {
                (Some(overriding_type_id), Some(overridden_type_id)) => {
                    types.type_id_is_function_and_overrides(overriding_type_id, overridden_type_id)
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Whether a bare-name reference to `function` dispatches to its most-derived
/// override rather than to the declaration itself: a `virtual` contract
/// function or modifier, or an interface function, which is implicitly
/// virtual. A free or library function, or a library modifier, never is,
/// whatever it is marked.
pub(crate) fn has_virtual_semantics(binder: &Binder, function: &ir::FunctionDefinition) -> bool {
    match binder
        .enclosing_definition_node_id(function.id())
        .and_then(|id| binder.find_definition_by_id(id))
    {
        Some(Definition::Contract(_)) => function.attributes.is_virtual,
        Some(Definition::Interface(_)) => true,
        _ => false,
    }
}

/// Whether the modifier-list entry `name`, naming `modifier`, dispatches to
/// the most-derived override of its name: a bare name of a modifier with
/// virtual semantics. A qualified name like `A.m` runs the declaration; both
/// can resolve to the same declaration, so only the path's segments tell them
/// apart.
pub(crate) fn modifier_dispatches_virtually(
    binder: &Binder,
    name: &ir::IdentifierPath,
    modifier: &ir::FunctionDefinition,
) -> bool {
    name.len() == 1 && has_virtual_semantics(binder, modifier)
}

/// The most-derived function overriding `function` among `functions`, the
/// linearised functions of the contract being compiled, or `None` when nothing
/// there overrides it: the declaration is then the target, as for a virtual
/// function whose only implementation is itself and for an interface member
/// the contract leaves unimplemented.
///
/// `function` must have virtual semantics, which the caller establishes: the
/// search compares signatures, not specifiers, so a non-virtual declaration
/// would match a same-signature derived function that does not override it.
pub(crate) fn most_derived_override<'a>(
    binder: &Binder,
    types: &TypeRegistry,
    functions: &'a [ir::FunctionDefinition],
    function: &ir::FunctionDefinition,
) -> Option<&'a ir::FunctionDefinition> {
    functions
        .iter()
        .find(|candidate| function_overrides(binder, types, candidate, function))
}

/// The most-derived modifier of `modifier`'s name among `bases`, the
/// linearisation of the contract being compiled, or `None` when no contract
/// there declares one. The search is by name alone, since modifiers cannot
/// overload.
///
/// `modifier` must have virtual semantics, which the caller establishes: the
/// search compares names, not specifiers, so a declaration without them would
/// be resolved to a same-named modifier that does not override it.
pub(crate) fn most_derived_modifier<'a>(
    binder: &'a Binder,
    bases: &[NodeId],
    modifier: &ir::FunctionDefinition,
) -> Option<&'a ir::FunctionDefinition> {
    let name = modifier
        .name
        .as_ref()
        .expect("modifiers are named")
        .unparse();
    bases.iter().find_map(|base_id| {
        let Definition::Contract(base) = binder.find_definition_by_id(*base_id)? else {
            return None;
        };
        base.ir_node.members.iter().find_map(|member| match member {
            ir::ContractMember::FunctionDefinition(candidate)
                if matches!(candidate.kind, ir::FunctionKind::Modifier)
                    && candidate
                        .name
                        .as_ref()
                        .is_some_and(|identifier| identifier.unparse() == name) =>
            {
                Some(candidate)
            }
            _ => None,
        })
    })
}

/// The part of the linearisation `bases` that `super` written in `anchor_id`
/// searches: the bases after the anchor, or `None` when the anchor is not one.
pub(crate) fn bases_after(bases: &[NodeId], anchor_id: NodeId) -> Option<&[NodeId]> {
    let anchor_position = bases.iter().position(|base| *base == anchor_id)?;
    Some(&bases[anchor_position + 1..])
}

/// The first implementation of `function` among `bases`, the linearisation of
/// the contract being compiled after the `super` anchor, or `None` when
/// nothing there implements it: the declaration is then the target, as for an
/// interface member the hierarchy leaves unimplemented.
pub(crate) fn super_override<'a>(
    binder: &'a Binder,
    types: &TypeRegistry,
    bases: &[NodeId],
    function: &ir::FunctionDefinition,
) -> Option<&'a ir::FunctionDefinition> {
    bases.iter().find_map(|base_id| {
        let members = match binder.find_definition_by_id(*base_id)? {
            Definition::Contract(base) => &base.ir_node.members[..],
            Definition::Interface(base) => &base.ir_node.members[..],
            _ => return None,
        };
        members.iter().find_map(|member| match member {
            ir::ContractMember::FunctionDefinition(candidate)
                if matches!(candidate.kind, ir::FunctionKind::Regular)
                    && candidate.body.is_some()
                    && function_overrides(binder, types, candidate, function) =>
            {
                Some(candidate)
            }
            _ => None,
        })
    })
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
