//! Override dispatch: which declaration a name runs when a contract is the one
//! being compiled. The rules read the binder's definitions and typings against
//! the type registry, so they are grouped on a borrow of both rather than
//! threaded through every call.

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::types::{TypeId, TypeRegistry};

/// What a bare-name reference to a function runs in a contract: a function, or
/// the getter of a public state variable that overrides it.
pub enum VirtualTarget<'a> {
    Function(&'a ir::FunctionDefinition),
    Getter(&'a ir::StateVariableDefinition),
}

/// A public state variable's generated getter, reduced to what we need to tell
/// whether it overrides a function. Just its name and its function type.
pub(crate) struct GetterSlot<'a> {
    name: &'a str,
    type_id: TypeId,
}

pub(crate) struct Overrides<'a> {
    binder: &'a Binder,
    types: &'a TypeRegistry,
}

impl<'a> Overrides<'a> {
    pub(crate) fn new(binder: &'a Binder, types: &'a TypeRegistry) -> Self {
        Self { binder, types }
    }

    /// Whether `overriding` overrides `overridden`: they are of the same kind
    /// and either share a name with signatures in an override relationship, or
    /// are both unnamed, whatever their signatures: a contract dispatches at
    /// most one fallback and one receive function, so the kind alone
    /// identifies them.
    pub(crate) fn function_overrides(
        &self,
        overriding: &ir::FunctionDefinition,
        overridden: &ir::FunctionDefinition,
    ) -> bool {
        if overriding.kind != overridden.kind {
            return false;
        }
        match (&overriding.name, &overridden.name) {
            (None, None) => true,
            (Some(name), Some(other_name)) => {
                if name.unparse() != other_name.unparse() {
                    return false;
                }
                let overriding_type_id = self.binder.node_typing(overriding.id()).as_type_id();
                let overridden_type_id = self.binder.node_typing(overridden.id()).as_type_id();
                match (overriding_type_id, overridden_type_id) {
                    (Some(overriding_type_id), Some(overridden_type_id)) => self
                        .types
                        .type_id_is_function_and_overrides(overriding_type_id, overridden_type_id),
                    _ => false,
                }
            }
            _ => false,
        }
    }

    /// Whether a bare-name reference to `function` dispatches to its
    /// most-derived override rather than to the declaration itself: a
    /// `virtual` contract function, or an interface member, which is
    /// implicitly virtual. A free or library function never is, whatever it is
    /// marked.
    pub(crate) fn has_virtual_semantics(&self, function: &ir::FunctionDefinition) -> bool {
        if function.name.is_none() {
            return function.attributes.is_virtual;
        }
        match self
            .binder
            .enclosing_definition_node_id(function.id())
            .and_then(|id| self.binder.find_definition_by_id(id))
        {
            Some(Definition::Contract(_)) => function.attributes.is_virtual,
            Some(Definition::Interface(_)) => true,
            _ => false,
        }
    }

    /// The most-derived function overriding `function` among `functions`, the
    /// linearised functions of the contract being compiled, or `None` when
    /// nothing there overrides it, as for an interface member the contract
    /// leaves unimplemented or a function a getter overrides. p4 has already
    /// dropped every function a more-derived one overrides, so at most one of
    /// `functions` can match.
    ///
    /// `function` must have virtual semantics, which the caller establishes:
    /// the search compares signatures, not specifiers, so a non-virtual
    /// declaration would match a same-signature derived function that does not
    /// override it.
    pub(crate) fn virtual_target<'f>(
        &self,
        functions: &'f [ir::FunctionDefinition],
        function: &ir::FunctionDefinition,
    ) -> Option<&'f ir::FunctionDefinition> {
        functions
            .iter()
            .find(|candidate| self.function_overrides(candidate, function))
    }

    /// The public state variable among `state_variables`, the linearised state
    /// variables of the contract being compiled, whose getter overrides
    /// `function`, or `None` when none does.
    pub(crate) fn overriding_getter<'s>(
        &self,
        state_variables: &'s [ir::StateVariableDefinition],
        function: &ir::FunctionDefinition,
    ) -> Option<&'s ir::StateVariableDefinition> {
        state_variables.iter().find(|state_variable| {
            self.getter_slot(state_variable)
                .is_some_and(|getter| self.getter_overrides(&getter, function))
        })
    }

    /// The modifier `name` that the contract being compiled runs: the one its
    /// most-derived base among `bases` declares, or `None` when none does.
    /// Modifiers cannot overload, so the name identifies one.
    pub(crate) fn modifier_target(
        &self,
        bases: &[NodeId],
        name: &str,
    ) -> Option<&'a ir::FunctionDefinition> {
        bases.iter().find_map(|base_id| {
            let Definition::Contract(base) = self
                .binder
                .find_definition_by_id(*base_id)
                .expect("a linearised base is a definition")
            else {
                return None;
            };
            base.ir_node.members.iter().find_map(|member| match member {
                ir::ContractMember::FunctionDefinition(candidate)
                    if matches!(candidate.kind, ir::FunctionKind::Modifier)
                        && candidate
                            .name
                            .as_ref()
                            .is_some_and(|candidate_name| candidate_name.unparse() == name) =>
                {
                    Some(candidate)
                }
                _ => None,
            })
        })
    }

    /// The first implementation of `function` among `bases`, the linearisation
    /// of the contract being compiled after the `super` enclosing contract, or `None` when
    /// nothing there implements it: the declaration is then the target, as for
    /// an interface member the hierarchy leaves unimplemented.
    pub(crate) fn super_target(
        &self,
        bases: &[NodeId],
        function: &ir::FunctionDefinition,
    ) -> Option<&'a ir::FunctionDefinition> {
        bases.iter().find_map(|base_id| {
            let members = match self
                .binder
                .find_definition_by_id(*base_id)
                .expect("a linearised base is a definition")
            {
                Definition::Contract(base) => &base.ir_node.members[..],
                _ => return None,
            };
            members.iter().find_map(|member| match member {
                ir::ContractMember::FunctionDefinition(candidate)
                    if matches!(candidate.kind, ir::FunctionKind::Regular)
                        && candidate.body.is_some()
                        && self.function_overrides(candidate, function) =>
                {
                    Some(candidate)
                }
                _ => None,
            })
        })
    }

    /// The [`GetterSlot`] for `state_variable`, or `None` when it has no
    /// getter. Only public state variables have a getter, and `getter_type_id`
    /// is set only for those, so unwrapping it below is what filters the
    /// non-public ones out.
    pub(crate) fn getter_slot<'s>(
        &self,
        state_variable: &'s ir::StateVariableDefinition,
    ) -> Option<GetterSlot<'s>> {
        match self.binder.find_definition_by_id(state_variable.id()) {
            Some(Definition::StateVariable(definition)) => Some(GetterSlot {
                name: state_variable.name.unparse(),
                type_id: definition.getter_type_id?,
            }),
            _ => None,
        }
    }

    /// Whether `getter` overrides `function`. True when they share a name and
    /// the getter's type can override the function's. The getter version of
    /// [`Overrides::function_overrides`].
    pub(crate) fn getter_overrides(
        &self,
        getter: &GetterSlot<'_>,
        function: &ir::FunctionDefinition,
    ) -> bool {
        function
            .name
            .as_ref()
            .is_some_and(|name| name.unparse() == getter.name)
            && self
                .binder
                .node_typing(function.id())
                .as_type_id()
                .is_some_and(|function_type_id| {
                    self.types
                        .type_id_is_function_and_overrides(getter.type_id, function_type_id)
                })
    }
}
