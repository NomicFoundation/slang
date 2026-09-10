//! The override check. Validates each member of a contract or interface
//! against the inherited members it overrides.

use slang_solidity_v2_common::collections::Map;
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    MissingOverrideSpecifier, OverrideChangesModifierSignature, OverrideChangesStateMutability,
    OverrideParameterLocationDiffers, OverrideReturnLocationDiffers, OverrideReturnTypesDiffer,
    OverrideVisibilityDiffers, OverridingNonVirtualMember, OverridingPublicStateVariable,
    PublicStateVariableOverridesNonExternal, StateMutability, UnimplementedOverrideOfImplemented,
};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;
use smallvec::SmallVec;

use super::HierarchyChecker;
use crate::binder::{Binder, Definition};
use crate::passes::common::Overridable;

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

        // Index the members each direct base declares or inherits, nearest
        // definition first within a name.
        let base_members: SmallVec<[BaseMembers<'a>; 2]> = bases
            .iter()
            .filter_map(|base_id| self.binder.get_linearised_bases(*base_id))
            .map(|linearised_bases| {
                let mut by_name = BaseMembers::default();
                for member in linearised_bases
                    .iter()
                    .flat_map(|type_id| Overridable::members_of(self.binder, *type_id))
                {
                    by_name.entry(member.name()).or_default().push(member);
                }
                by_name
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
        base_members: &[BaseMembers<'a>],
    ) -> SmallVec<[Overridable<'a>; 2]> {
        let name = overriding.name();
        let mut overridden: SmallVec<[Overridable<'a>; 2]> = SmallVec::new();
        for members in base_members {
            let nearest = members.get(&name).and_then(|candidates| {
                candidates
                    .iter()
                    .find(|candidate| overriding.overrides(self.binder, self.types, candidate))
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
                    kind: overriding.kind(),
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
                            kind: overridden.kind(),
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
            self.report(overriding, OverrideVisibilityDiffers);
        }

        // Before 0.8.5 this only applies when the overridden member is a function.
        if !overriding.is_implemented()
            && overridden.is_implemented()
            && (overridden.is_function() || self.language_version >= LanguageVersion::V0_8_5)
        {
            self.report(
                overriding,
                UnimplementedOverrideOfImplemented {
                    kind: overriding.kind(),
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
                        kind: overriding.kind(),
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
                    kind: overriding.kind(),
                    from: state_mutability(overridden_mutability),
                    to: state_mutability(overriding_mutability),
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

/// The overridable members of one direct base's hierarchy, grouped by name and
/// nearest definition first within each name. `None` is the name of a fallback
/// or receive.
type BaseMembers<'a> = Map<Option<&'a str>, SmallVec<[Overridable<'a>; 1]>>;

/// Orders mutabilities from strictest to loosest.
fn mutability_rank(mutability: ir::FunctionMutability) -> u8 {
    match mutability {
        ir::FunctionMutability::Pure => 0,
        ir::FunctionMutability::View => 1,
        ir::FunctionMutability::NonPayable => 2,
        ir::FunctionMutability::Payable => 3,
    }
}

fn state_mutability(mutability: ir::FunctionMutability) -> StateMutability {
    match mutability {
        ir::FunctionMutability::Pure => StateMutability::Pure,
        ir::FunctionMutability::View => StateMutability::View,
        ir::FunctionMutability::NonPayable => StateMutability::NonPayable,
        ir::FunctionMutability::Payable => StateMutability::Payable,
    }
}
