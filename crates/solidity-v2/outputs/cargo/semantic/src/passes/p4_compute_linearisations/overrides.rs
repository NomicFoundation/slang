//! The override check. Validates each member of a contract or interface
//! against the inherited members it overrides.

use std::ops::Range;

use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    MissingOverrideSpecifier, OverrideChangesModifierSignature, OverrideChangesStateMutability,
    OverrideParameterLocationDiffers, OverrideReturnLocationDiffers, OverrideReturnTypesDiffer,
    OverrideVisibilityDiffers, OverridingNonVirtualMember, OverridingPublicStateVariable,
    PublicStateVariableOverridesNonExternal, UnimplementedOverrideOfImplemented,
};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;
use smallvec::SmallVec;

use super::HierarchyChecker;
use crate::binder::{Binder, Definition};
use crate::passes::common::{Callable, overrides};
use crate::types::{FunctionType, Type, TypeRegistry};

impl<'a> HierarchyChecker<'a> {
    /// Checks the members of this type against the inherited members they
    /// override.
    pub(super) fn check_overrides(&mut self) {
        let Some(bases) = direct_bases(self.binder, self.definition_id) else {
            return;
        };

        // Nothing to check if the type has no overridable members.
        let mut members = Overridable::members_of(self.binder, self.definition_id).peekable();
        if members.peek().is_none() {
            return;
        }

        // Collect the members of each direct base once, nearest definition
        // first.
        let base_members: SmallVec<[Vec<Overridable<'a>>; 2]> = bases
            .iter()
            .filter_map(|base_id| self.binder.get_linearised_bases(*base_id))
            .map(|linearised_bases| {
                linearised_bases
                    .iter()
                    .flat_map(|type_id| Overridable::members_of(self.binder, *type_id))
                    .collect()
            })
            .collect();
        if base_members.is_empty() {
            return;
        }
        for overriding in members {
            for overridden in self.overridden_by(&overriding, &base_members) {
                self.check_override(&overriding, &overridden);
            }
        }
    }

    /// Finds the members `overriding` overrides. Each direct base contributes
    /// its nearest definition with the same signature, and a definition reached
    /// through two bases is returned once.
    fn overridden_by(
        &self,
        overriding: &Overridable<'a>,
        base_members: &[Vec<Overridable<'a>>],
    ) -> SmallVec<[Overridable<'a>; 2]> {
        let mut overridden: SmallVec<[Overridable<'a>; 2]> = SmallVec::new();
        for members in base_members {
            let nearest = members.iter().find(|candidate| {
                overrides(
                    self.binder,
                    self.types,
                    overriding.callable(),
                    candidate.callable(),
                )
            });
            if let Some(found) = nearest
                && !overridden
                    .iter()
                    .any(|member| member.node_id() == found.node_id())
            {
                overridden.push(*found);
            }
        }
        overridden
    }

    /// Reports the rules `overriding` breaks by overriding `overridden`.
    #[allow(clippy::too_many_lines)]
    fn check_override(&mut self, overriding: &Overridable<'a>, overridden: &Overridable<'a>) {
        if overriding.is_modifier()
            && let (Some(overriding_type), Some(overridden_type)) = (
                overriding.function_type(self.binder, self.types),
                overridden.function_type(self.binder, self.types),
            )
            && overriding_type.parameter_types != overridden_type.parameter_types
        {
            self.report(overriding, OverrideChangesModifierSignature);
        }

        // From 0.8.8 the specifier is optional when overriding an interface
        // function.
        if !overriding.has_override_specifier()
            && (self.language_version < LanguageVersion::V0_8_8
                || !overridden.is_interface_function())
        {
            self.report(
                overriding,
                MissingOverrideSpecifier {
                    kind: overriding.describe().to_owned(),
                },
            );
        }

        match overridden {
            Overridable::StateVariable(_) => {
                self.report(overridden, OverridingPublicStateVariable);
            }
            Overridable::Function { .. } | Overridable::Modifier(_) => {
                if !overridden.is_virtual() {
                    self.report(
                        overridden,
                        OverridingNonVirtualMember {
                            kind: overridden.describe().to_owned(),
                        },
                    );
                }
            }
        }

        if let Overridable::StateVariable(_) = overriding {
            if overridden.visibility() != ir::FunctionVisibility::External {
                self.report(overriding, PublicStateVariableOverridesNonExternal);
            }
        } else if overriding.visibility() != overridden.visibility()
            // Widening an external function to public is the one allowed change.
            && !(overridden.visibility() == ir::FunctionVisibility::External
                && overriding.visibility() == ir::FunctionVisibility::Public)
        {
            self.report(
                overriding,
                OverrideVisibilityDiffers {
                    kind: overriding.describe().to_owned(),
                },
            );
        }

        // Before 0.8.5 this only applies when the overridden member is a function.
        if !overriding.is_implemented()
            && overridden.is_implemented()
            && (overridden.is_function() || self.language_version >= LanguageVersion::V0_8_5)
        {
            self.report(
                overriding,
                UnimplementedOverrideOfImplemented {
                    kind: overriding.describe().to_owned(),
                },
            );
        }

        // The remaining rules compare two functions, or a getter with a
        // function.
        if !overridden.is_function() {
            return;
        }

        // A fallback has no fixed signature, so its parameters and return
        // values are not compared.
        if overriding.function_kind() != ir::FunctionKind::Fallback
            && let (Some(overriding_type), Some(overridden_type)) = (
                overriding.function_type(self.binder, self.types),
                overridden.function_type(self.binder, self.types),
            )
        {
            let return_types_differ = !self.types.type_overrides_in_external_function(
                overriding_type.return_type,
                overridden_type.return_type,
            );
            if return_types_differ {
                self.report(
                    overriding,
                    OverrideReturnTypesDiffer {
                        kind: overriding.describe().to_owned(),
                    },
                );
            }

            // Two functions match even if a parameter or return value is
            // calldata in one and memory in the other. From 0.8.14 that is only
            // allowed when the overridden function is external.
            if self.language_version >= LanguageVersion::V0_8_14
                && overriding.is_function()
                && !return_types_differ
                && overridden.visibility() != ir::FunctionVisibility::External
            {
                if overriding_type.parameter_types != overridden_type.parameter_types {
                    self.report(overriding, OverrideParameterLocationDiffers);
                }
                if overriding_type.return_type != overridden_type.return_type {
                    self.report(overriding, OverrideReturnLocationDiffers);
                }
            }
        }

        let overriding_mutability = overriding.mutability();
        let overridden_mutability = overridden.mutability();
        if overriding_mutability != overridden_mutability
            && (mutability_rank(overriding_mutability) > mutability_rank(overridden_mutability)
                || overridden_mutability == ir::FunctionMutability::Payable)
        {
            self.report(
                overriding,
                OverrideChangesStateMutability {
                    kind: overriding.describe().to_owned(),
                    from: mutability_name(overridden_mutability).to_owned(),
                    to: mutability_name(overriding_mutability).to_owned(),
                },
            );
        }
    }

    fn report(&mut self, member: &Overridable<'_>, kind: impl Into<DiagnosticKind>) {
        let file_id = self
            .file_node_mapper
            .file_id_from_node_id(member.node_id())
            .to_owned();
        self.diagnostics.push(file_id, member.range(), kind);
    }
}

/// The direct bases of a contract or interface, or `None` for any other type.
fn direct_bases(binder: &Binder, definition_id: NodeId) -> Option<&[NodeId]> {
    match binder.find_definition_by_id(definition_id) {
        Some(Definition::Contract(contract)) => contract.bases.as_deref(),
        Some(Definition::Interface(interface)) => interface.bases.as_deref(),
        _ => None,
    }
}

/// A member that can take part in overriding. A function, modifier or public
/// state variable, viewed through the properties the override rules compare.
#[derive(Clone, Copy)]
enum Overridable<'a> {
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
    fn members_of(binder: &'a Binder, type_id: NodeId) -> impl Iterator<Item = Self> + 'a {
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
    fn of(member: &'a ir::ContractMember, in_interface: bool) -> Option<Self> {
        match member {
            ir::ContractMember::FunctionDefinition(definition) => match definition.kind {
                ir::FunctionKind::Regular
                | ir::FunctionKind::Fallback
                | ir::FunctionKind::Receive => Some(Self::Function {
                    definition,
                    in_interface,
                }),
                ir::FunctionKind::Modifier => Some(Self::Modifier(definition)),
                ir::FunctionKind::Constructor => None,
            },
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

    fn node_id(&self) -> NodeId {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => definition.id(),
            Self::StateVariable(state_variable) => state_variable.id(),
        }
    }

    fn describe(&self) -> &'static str {
        match self {
            Self::Function { .. } => "function",
            Self::Modifier(_) => "modifier",
            Self::StateVariable(_) => "public state variable",
        }
    }

    /// The function kind. A getter is a regular function.
    fn function_kind(&self) -> ir::FunctionKind {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => definition.kind,
            Self::StateVariable(_) => ir::FunctionKind::Regular,
        }
    }

    fn is_modifier(&self) -> bool {
        matches!(self, Self::Modifier(_))
    }

    fn is_function(&self) -> bool {
        matches!(self, Self::Function { .. })
    }

    fn is_interface_function(&self) -> bool {
        matches!(
            self,
            Self::Function {
                in_interface: true,
                ..
            }
        )
    }

    fn has_override_specifier(&self) -> bool {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => {
                definition.attributes.override_specifier.is_some()
            }
            Self::StateVariable(state_variable) => {
                state_variable.attributes.override_specifier.is_some()
            }
        }
    }

    fn is_virtual(&self) -> bool {
        match self {
            Self::Function {
                definition,
                in_interface,
            } => definition.attributes.is_virtual || *in_interface,
            Self::Modifier(definition) => definition.attributes.is_virtual,
            Self::StateVariable(_) => false,
        }
    }

    fn visibility(&self) -> ir::FunctionVisibility {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => {
                definition.attributes.visibility
            }
            Self::StateVariable(_) => ir::FunctionVisibility::External,
        }
    }

    fn mutability(&self) -> ir::FunctionMutability {
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

    fn is_implemented(&self) -> bool {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => {
                definition.body.is_some()
            }
            Self::StateVariable(_) => true,
        }
    }

    /// The declared function type, or the getter's type for a state variable.
    /// `None` when it couldn't be computed, which is reported elsewhere.
    fn function_type(
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
            unreachable!("type of a callable or getter is not a function");
        };
        Some(function_type)
    }

    fn range(&self) -> Range<usize> {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => {
                definition.signature_text_range()
            }
            Self::StateVariable(state_variable) => state_variable.range.clone(),
        }
    }

    /// The member as the shared override relation sees it.
    fn callable(&self) -> &'a dyn Callable {
        match self {
            Self::Function { definition, .. } | Self::Modifier(definition) => *definition,
            Self::StateVariable(state_variable) => *state_variable,
        }
    }
}

/// Orders mutabilities from strictest to loosest.
fn mutability_rank(mutability: ir::FunctionMutability) -> u8 {
    match mutability {
        ir::FunctionMutability::Pure => 0,
        ir::FunctionMutability::View => 1,
        ir::FunctionMutability::NonPayable => 2,
        ir::FunctionMutability::Payable => 3,
    }
}

fn mutability_name(mutability: ir::FunctionMutability) -> &'static str {
    match mutability {
        ir::FunctionMutability::Pure => "pure",
        ir::FunctionMutability::View => "view",
        ir::FunctionMutability::NonPayable => "nonpayable",
        ir::FunctionMutability::Payable => "payable",
    }
}
