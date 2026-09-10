use std::ops::Range;
use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::semantic::OverridableKind;
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

/// A member that can take part in overriding. A function, modifier or public
/// state variable, viewed through the properties the override rules compare.
#[derive(Clone, Copy)]
pub(crate) enum Overridable<'a> {
    /// A regular function, a fallback or a receive.
    Function {
        definition: &'a ir::FunctionDefinition,
        /// Whether the member is declared in an interface. A function declared
        /// in an interface is implicitly `virtual`.
        in_interface: bool,
    },
    Modifier(&'a ir::FunctionDefinition),
    /// A `public` state variable.
    StateVariable(&'a ir::StateVariableDefinition),
}

impl<'a> Overridable<'a> {
    /// The overridable members of the contract or interface `type_id`.
    pub(crate) fn members_of(
        binder: &'a Binder,
        type_id: NodeId,
    ) -> impl Iterator<Item = Self> + 'a {
        let (members, in_interface) = match binder.find_definition_by_id(type_id) {
            Some(Definition::Contract(contract)) => (&contract.ir_node.members[..], false),
            Some(Definition::Interface(interface)) => (&interface.ir_node.members[..], true),
            _ => unreachable!("a type with bases is a contract or an interface"),
        };
        members
            .iter()
            .filter_map(move |member| Self::of(member, in_interface))
    }

    /// The overridable view of `member`, or `None` for members that don't take
    /// part in overriding.
    pub(crate) fn of(member: &'a ir::ContractMember, in_interface: bool) -> Option<Self> {
        match member {
            ir::ContractMember::FunctionDefinition(definition) => {
                Self::of_function(definition, in_interface)
            }
            ir::ContractMember::StateVariableDefinition(state_variable)
                if matches!(
                    state_variable.attributes.visibility,
                    ir::StateVariableVisibility::Public
                ) =>
            {
                Some(Self::StateVariable(state_variable))
            }
            _ => None,
        }
    }

    /// The overridable view of a function or modifier definition, for callers
    /// that hold one directly rather than a contract member. `None` for a
    /// constructor, which takes no part in overriding.
    pub(crate) fn of_function(
        definition: &'a ir::FunctionDefinition,
        in_interface: bool,
    ) -> Option<Self> {
        match definition.kind {
            ir::FunctionKind::Regular | ir::FunctionKind::Fallback | ir::FunctionKind::Receive => {
                Some(Self::Function {
                    definition,
                    in_interface,
                })
            }
            ir::FunctionKind::Modifier => Some(Self::Modifier(definition)),
            ir::FunctionKind::Constructor => None,
        }
    }

    /// Whether this member and `other` occupy the same override slot. They
    /// match on name and function kind, and regular functions and getters also
    /// need indistinguishable parameter lists. The relation is symmetric, so
    /// which of the two overrides the other does not matter here.
    pub(crate) fn overrides(
        &self,
        binder: &'a Binder,
        types: &'a TypeRegistry,
        other: &Self,
    ) -> bool {
        if self.function_kind() != other.function_kind() || self.name() != other.name() {
            return false;
        }
        match self.function_kind() {
            // Modifiers cannot be overloaded, and a fallback or receive has no
            // selector, so name and kind are enough.
            ir::FunctionKind::Modifier | ir::FunctionKind::Fallback | ir::FunctionKind::Receive => {
                true
            }
            // A member whose parameters aren't all typed is reported on its
            // own. Treating it as matching anything here would invent an
            // override.
            ir::FunctionKind::Regular => match (
                self.function_type(binder, types),
                other.function_type(binder, types),
            ) {
                (Some(self_type), Some(other_type)) => types.parameter_lists_are_indistinguishable(
                    &self_type.parameter_types,
                    &other_type.parameter_types,
                ),
                _ => false,
            },
            ir::FunctionKind::Constructor => {
                unreachable!("constructors take no part in overriding")
            }
        }
    }

    pub(crate) fn node_id(&self) -> NodeId {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => definition.id(),
            Self::StateVariable(state_variable) => state_variable.id(),
        }
    }

    /// The name, or `None` for a fallback or receive.
    pub(crate) fn name(&self) -> Option<&'a str> {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => {
                definition.name.as_ref().map(|name| name.unparse())
            }
            Self::StateVariable(state_variable) => Some(state_variable.name.unparse()),
        }
    }

    pub(crate) fn kind(&self) -> OverridableKind {
        match self {
            Self::Function { .. } => OverridableKind::Function,
            Self::Modifier(_) => OverridableKind::Modifier,
            Self::StateVariable(_) => OverridableKind::PublicStateVariable,
        }
    }

    /// The function kind. A getter is a regular function.
    pub(crate) fn function_kind(&self) -> ir::FunctionKind {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => definition.kind,
            Self::StateVariable(_) => ir::FunctionKind::Regular,
        }
    }

    pub(crate) fn is_modifier(&self) -> bool {
        matches!(self, Self::Modifier(_))
    }

    pub(crate) fn is_function(&self) -> bool {
        matches!(self, Self::Function { .. })
    }

    pub(crate) fn is_interface_function(&self) -> bool {
        matches!(
            self,
            Self::Function {
                in_interface: true,
                ..
            }
        )
    }

    pub(crate) fn has_override_specifier(&self) -> bool {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => {
                definition.attributes.override_specifier.is_some()
            }
            Self::StateVariable(state_variable) => {
                state_variable.attributes.override_specifier.is_some()
            }
        }
    }

    pub(crate) fn is_virtual(&self) -> bool {
        match self {
            Self::Function {
                definition,
                in_interface,
            } => definition.attributes.is_virtual || *in_interface,
            Self::Modifier(definition) => definition.attributes.is_virtual,
            Self::StateVariable(_) => false,
        }
    }

    pub(crate) fn visibility(&self) -> ir::FunctionVisibility {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => {
                definition.attributes.visibility
            }
            Self::StateVariable(_) => ir::FunctionVisibility::External,
        }
    }

    pub(crate) fn mutability(&self) -> ir::FunctionMutability {
        match self {
            Self::Function { definition, .. } => definition.attributes.mutability,
            Self::Modifier(_) => unreachable!("modifiers have no state mutability"),
            Self::StateVariable(state_variable) => {
                if matches!(
                    state_variable.attributes.mutability,
                    ir::StateVariableMutability::Constant
                ) {
                    ir::FunctionMutability::Pure
                } else {
                    ir::FunctionMutability::View
                }
            }
        }
    }

    pub(crate) fn is_implemented(&self) -> bool {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => {
                definition.body.is_some()
            }
            Self::StateVariable(_) => true,
        }
    }

    /// The declared function type, or the getter's type for a state variable.
    /// `None` when it couldn't be computed, which is reported elsewhere.
    pub(crate) fn function_type(
        &self,
        binder: &'a Binder,
        types: &'a TypeRegistry,
    ) -> Option<&'a FunctionType> {
        let type_id = match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => {
                binder.node_typing(definition.id()).as_type_id()?
            }
            Self::StateVariable(state_variable) => {
                match binder.find_definition_by_id(state_variable.id()) {
                    Some(Definition::StateVariable(definition)) => definition.getter_type_id?,
                    _ => unreachable!("state variable is not registered as a definition"),
                }
            }
        };
        let Type::Function(function_type) = types.get_type_by_id(type_id) else {
            unreachable!("type of a function or getter is not a function");
        };
        Some(function_type)
    }

    pub(crate) fn range(&self) -> Range<usize> {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => {
                definition.signature_text_range()
            }
            Self::StateVariable(state_variable) => state_variable.range.clone(),
        }
    }
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
