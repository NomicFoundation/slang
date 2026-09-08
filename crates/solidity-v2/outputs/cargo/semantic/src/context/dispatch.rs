use std::sync::Arc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::SemanticContext;
use crate::binder::{Binder, Definition, Scope};
use crate::passes::common::Overridable;
use crate::types::TypeRegistry;

/// The function or public state variable getter selected by virtual dispatch.
pub enum VirtualTarget<'a> {
    Function(&'a ir::FunctionDefinition),
    Getter(&'a ir::StateVariableDefinition),
}

impl SemanticContext {
    /// Resolves `function` in code compiled into `contract_id`. Virtual members
    /// select their most-derived override, which can be a public variable's
    /// getter. Nonvirtual members and declarations with no override resolve to
    /// themselves, including unimplemented members of abstract contracts.
    ///
    /// Returns `None` unless `contract_id` identifies a linearised contract and
    /// `function` is a function or modifier declared in its hierarchy in this
    /// context. Constructors and free or library functions are not accepted.
    pub fn resolve_virtual<'a>(
        &'a self,
        contract_id: NodeId,
        function: &'a ir::FunctionDefinition,
    ) -> Option<VirtualTarget<'a>> {
        let (bases, member) = self.dispatch_member(contract_id, function)?;
        if !member.is_virtual() {
            return Some(VirtualTarget::Function(function));
        }
        if member.is_modifier() {
            return Some(VirtualTarget::Function(
                modifier_target(&self.binder, &self.types, bases, member)
                    .expect("a virtual modifier is at least its own target"),
            ));
        }
        if let Some(target) = function_target(
            &self.binder,
            &self.types,
            self.linearised_functions(contract_id),
            member,
        ) {
            return Some(VirtualTarget::Function(target));
        }
        let getter = bases
            .iter()
            .flat_map(|base| Overridable::members_of(&self.binder, *base))
            .find_map(|candidate| match candidate {
                Overridable::StateVariable(variable)
                    if candidate.overrides(&self.binder, &self.types, &member) =>
                {
                    Some(variable)
                }
                _ => None,
            });
        Some(match getter {
            Some(variable) => VirtualTarget::Getter(variable),
            None => VirtualTarget::Function(function),
        })
    }

    /// Resolves `super.f` for `function`, written in `enclosing_contract` and
    /// compiled into `contract_id`. Searches after the enclosing contract in
    /// the compiled contract's linearisation, skipping bodiless declarations.
    ///
    /// Returns `None` if the inputs do not belong to that contract's hierarchy,
    /// `function` is not a regular function from this context, or no matching
    /// implementation follows. A super target is always a function, never a getter.
    pub fn resolve_super<'a>(
        &'a self,
        contract_id: NodeId,
        function: &'a ir::FunctionDefinition,
        enclosing_contract: NodeId,
    ) -> Option<&'a ir::FunctionDefinition> {
        let (bases, member) = self.dispatch_member(contract_id, function)?;
        if member.function_kind() != ir::FunctionKind::Regular
            || !matches!(
                self.binder.find_definition_by_id(enclosing_contract),
                Some(Definition::Contract(_))
            )
        {
            return None;
        }
        super_target(&self.binder, &self.types, bases, enclosing_contract, member)
    }

    /// Resolves the modifier-list entry `invocation` in code compiled into
    /// `contract_id`. A bare name selects the most-derived modifier of that
    /// name in the hierarchy; a qualified name like `A.m` runs the declaration
    /// it names, as does a nonvirtual one.
    ///
    /// Returns `None` when the entry names a base rather than a modifier, as a
    /// constructor's base-argument list does, or when the modifier it names is
    /// not declared in `contract_id`'s hierarchy in this context.
    pub fn resolve_modifier(
        &self,
        contract_id: NodeId,
        invocation: &ir::ModifierInvocation,
    ) -> Option<&ir::FunctionDefinition> {
        let definition_id =
            self.resolve_reference_identifier_to_definition_id(invocation.name.last()?.id())?;
        let Definition::Modifier(modifier) = self.binder.find_definition_by_id(definition_id)?
        else {
            return None;
        };
        let (bases, member) = self.dispatch_member(contract_id, &modifier.ir_node)?;
        if invocation.name.len() > 1 || !member.is_virtual() {
            return Some(&modifier.ir_node);
        }
        modifier_target(&self.binder, &self.types, bases, member)
    }

    fn dispatch_member<'a>(
        &'a self,
        contract_id: NodeId,
        function: &ir::FunctionDefinition,
    ) -> Option<(&'a [NodeId], Overridable<'a>)> {
        if !matches!(
            self.binder.find_definition_by_id(contract_id),
            Some(Definition::Contract(_))
        ) {
            return None;
        }
        let bases = self
            .binder
            .get_linearised_bases(contract_id)
            .expect("p2 linearises every contract");
        let scope_id = self.binder.scope_id_for_node_id(function.id())?;
        let parent_scope_id = match self.binder.get_scope_by_id(scope_id) {
            Scope::Function(function_scope) => function_scope.parent_scope_id,
            Scope::Modifier(modifier_scope) => modifier_scope.parent_scope_id,
            _ => return None,
        };
        let enclosing_node_id = self.binder.get_scope_by_id(parent_scope_id).node_id();
        if !bases.contains(&enclosing_node_id) {
            return None;
        }
        let member =
            Overridable::members_of(&self.binder, enclosing_node_id).find(
                |member| match member {
                    Overridable::Function { definition, .. }
                    | Overridable::Modifier(definition) => Arc::ptr_eq(definition, function),
                    Overridable::StateVariable(_) => false,
                },
            )?;
        Some((bases, member))
    }
}

/// Finds the function occupying a slot in the already flattened function list.
pub(crate) fn function_target<'a>(
    binder: &'a Binder,
    types: &'a TypeRegistry,
    functions: &'a [ir::FunctionDefinition],
    member: Overridable<'a>,
) -> Option<&'a ir::FunctionDefinition> {
    functions.iter().find(|candidate| {
        Overridable::of_function(candidate, false)
            .expect("a linearised function is never a constructor")
            .overrides(binder, types, &member)
    })
}

/// Finds the most-derived modifier occupying the requested slot.
pub(crate) fn modifier_target<'a>(
    binder: &'a Binder,
    types: &'a TypeRegistry,
    bases: &[NodeId],
    member: Overridable<'a>,
) -> Option<&'a ir::FunctionDefinition> {
    bases
        .iter()
        .flat_map(|base| Overridable::members_of(binder, *base))
        .find_map(|candidate| match candidate {
            Overridable::Modifier(definition) if candidate.overrides(binder, types, &member) => {
                Some(definition)
            }
            _ => None,
        })
}

/// Finds the nearest implemented function after `enclosing_contract` in the
/// linearisation `bases`; `None` when the enclosing contract is not in it.
pub(crate) fn super_target<'a>(
    binder: &'a Binder,
    types: &'a TypeRegistry,
    bases: &[NodeId],
    enclosing_contract: NodeId,
    member: Overridable<'a>,
) -> Option<&'a ir::FunctionDefinition> {
    let position = bases.iter().position(|base| *base == enclosing_contract)?;
    bases[position + 1..]
        .iter()
        .flat_map(|base| Overridable::members_of(binder, *base))
        .find_map(|candidate| match candidate {
            Overridable::Function { definition, .. }
                if candidate.is_implemented() && candidate.overrides(binder, types, &member) =>
            {
                Some(definition)
            }
            _ => None,
        })
}
