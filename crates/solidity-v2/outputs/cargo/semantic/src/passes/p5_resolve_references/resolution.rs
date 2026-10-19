use std::sync::Arc;

use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::diagnostics::kinds::resolution::MemberNotFound;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::binder::{
    Definition, DefinitionIds, Reference, Resolution, ResolveOptions, ScopeId, Typing,
    UsingDirective, UsingOperator,
};
use crate::built_ins::BuiltInsResolver;
use crate::passes::common::{
    find_definition_namespace_scope_id, resolve_identifier_path_as_expression_in_scope,
    resolve_identifier_path_as_type_in_scope,
};
use crate::types::{ContractType, InterfaceType, StructType, Type, TypeId, UserMetaType};

/// Lexical style resolution of symbols
impl Pass<'_> {
    // This is a "top-level" (ie. not a member access) resolution method
    pub(super) fn resolve_symbol_in_scope(&self, scope_id: ScopeId, symbol: &str) -> Resolution {
        self.binder
            .resolve_in_scope(scope_id, symbol)
            .or_else(|| BuiltInsResolver::lookup_global(symbol).into())
    }

    fn active_using_directives_for_type(
        &self,
        type_id: TypeId,
    ) -> impl Iterator<Item = &UsingDirective> {
        // Compute the canonical type: this handles cases where the given type
        // is context dependent:
        // - If the type is a reference type, we need to compute the id of the
        // location independent type (using DataLocation::Inherited). If it
        // doesn't exist, proceed with the given value, but we won't find any
        // type-specific directives, only those applicable to all types (ie.
        // `using L for *`)
        // - If the type is a function type, it may have an associated
        // definition ID from the function definition where it is derived from.
        let type_id = self
            .types
            .find_canonical_type_id(type_id)
            .unwrap_or(type_id);

        // consider using directives in the scope stack
        self.scope_stack
            .lexical_scopes()
            .flat_map(|scope_id| self.binder.get_using_directives_in_scope(scope_id))
            // ... and add the global directives
            .chain(self.binder.get_global_using_directives())
            .filter(move |directive| directive.applies_to(type_id))
    }

    pub(super) fn resolve_symbol_in_typing(&mut self, typing: &Typing, symbol: &str) -> Resolution {
        match typing {
            Typing::Unresolved => Resolution::Unresolved,
            Typing::Undetermined(type_ids) => {
                // We cannot use argument-type disambiguation here, so we will
                // use the first result
                // TODO(validation) SDR[37]: check that the types are consistent (eg.
                // they are all function types) and that it makes sense to use
                // the first one
                self.resolve_symbol_in_type(type_ids[0], symbol)
            }
            Typing::Resolved(type_id) => self.resolve_symbol_in_type(*type_id, symbol),
            Typing::Super(enclosing_contract) => {
                // This resolves `super` against the linearisation of the contract it is
                // written in; which function runs depends on the contract being compiled,
                // see `SemanticContext::resolve_super`.
                let scope_id = self
                    .binder
                    .scope_id_for_node_id(*enclosing_contract)
                    .expect("a contract has a scope");
                self.binder.resolve_in_contract_scope(
                    scope_id,
                    symbol,
                    ResolveOptions::Super(*enclosing_contract),
                )
            }
            Typing::This(receiver_type_id) => {
                if let Some(scope_id) = self.current_contract_scope_id() {
                    let node_id = self.binder.get_scope_by_id(scope_id).node_id();
                    // TODO(validation) SDR[34]: for `this` resolutions we need to check
                    // that the returned definitions are externally available
                    // (ie. either `external` or `public`)
                    let mut definition_ids = self
                        .binder
                        .resolve_in_contract_scope(scope_id, symbol, ResolveOptions::This(node_id))
                        .get_definition_ids();

                    // Consider active `using` directives for `this`
                    if matches!(
                        self.types.get_type_by_id(*receiver_type_id),
                        Type::Contract(_)
                    ) {
                        self.add_attached_functions_for_type(
                            *receiver_type_id,
                            symbol,
                            &mut definition_ids,
                        );
                    }

                    Resolution::from(definition_ids)
                } else {
                    Resolution::Unresolved
                }
            }
            Typing::NewExpression(_type_id) => {
                // No legacy constructor call options in >= 0.8.0
                Resolution::Unresolved
            }
            Typing::BuiltIn(built_in) => self
                .built_ins_resolver()
                .lookup_member_of(built_in, symbol)
                .into(),
        }
    }

    fn resolve_symbol_in_type(&mut self, type_id: TypeId, symbol: &str) -> Resolution {
        let type_ = self.types.get_type_by_id(type_id);

        // Resolve direct members of the type first
        let mut definition_ids = match type_ {
            Type::Contract(ContractType { definition_id })
            | Type::Interface(InterfaceType { definition_id }) => {
                // A `Type::Library` doesn't belong here, since values of type `library` (ie `this`)
                // can't be used to access to library members.
                let scope_id = self.binder.scope_id_for_node_id(*definition_id).unwrap();
                self.binder
                    .resolve_in_contract_scope(scope_id, symbol, ResolveOptions::External)
                    .or_else(|| {
                        self.built_ins_resolver()
                            .lookup_member_of_type_id(type_id, symbol)
                            .into()
                    })
            }
            Type::Struct(StructType { definition_id, .. }) => {
                let scope_id = self.binder.scope_id_for_node_id(*definition_id).unwrap();
                self.binder.resolve_in_scope_as_namespace(scope_id, symbol)
            }
            Type::UserMetaType(UserMetaType { definition_id }) => {
                let definition_id = *definition_id;
                let resolution = find_definition_namespace_scope_id(self.binder, definition_id)
                    .map_or(Resolution::Unresolved, |scope_id| {
                        self.binder
                            .resolve_in_scope_as_expression_member(scope_id, symbol)
                    });
                if self.is_foreign_contract_or_interface(definition_id) {
                    let visible: DefinitionIds = resolution
                        .get_definition_ids()
                        .into_iter()
                        .filter(|id| self.is_reachable_from_outside_hierarchy(*id))
                        .collect();
                    Resolution::from(visible)
                } else {
                    resolution
                }
            }
            _ => Resolution::Unresolved,
        }
        .get_definition_ids();

        // Next, consider active `using` directives in the current context
        self.add_attached_functions_for_type(type_id, symbol, &mut definition_ids);

        Resolution::from(definition_ids).or_else(|| {
            // If still unresolved, try with a built-in
            self.built_ins_resolver()
                .lookup_member_of_type_id(type_id, symbol)
                .into()
        })
    }

    fn is_foreign_contract_or_interface(&self, definition_id: NodeId) -> bool {
        matches!(
            self.binder.find_definition_by_id(definition_id),
            Some(Definition::Contract(_) | Definition::Interface(_))
        ) && self.is_foreign_contract(definition_id)
    }

    /// Whether a member of a contract or interface can be reached through its
    /// name from outside the contract's hierarchy: types, errors and
    /// externally visible functions can, events only from 0.8.21, and
    /// variables and internal functions never.
    fn is_reachable_from_outside_hierarchy(&self, definition_id: NodeId) -> bool {
        match self.binder.find_definition_by_id(definition_id) {
            Some(definition @ Definition::Function(_)) => definition.is_externally_visible(),
            Some(Definition::Event(_)) => self.types.language_version() >= LanguageVersion::V0_8_21,
            Some(
                Definition::StateVariable(_) | Definition::Constant(_) | Definition::Modifier(_),
            ) => false,
            _ => true,
        }
    }

    fn add_attached_functions_for_type(
        &self,
        receiver_type_id: TypeId,
        symbol: &str,
        definition_ids: &mut DefinitionIds,
    ) {
        let active_directives = self.active_using_directives_for_type(receiver_type_id);
        let mut seen_ids = Set::default();
        for directive in active_directives {
            let scope_id = directive.get_scope_id();
            let resolution = self.binder.resolve_in_scope_as_namespace(scope_id, symbol);
            let ids = self
                .binder
                .follow_symbol_aliases(resolution)
                .get_definition_ids();
            for id in &ids {
                // Avoid returning duplicate definition IDs. That may happen
                // if equivalent `using` directives are active at some point
                // (eg. inherited through a base in older Solidity)
                // Filter the resolved definitions to only include
                // functions whose first parameter is of our type (or
                // implicitly convertible to it)
                if seen_ids.insert(*id)
                    && self.is_function_with_receiver_type(*id, receiver_type_id)
                {
                    definition_ids.push(*id);
                }
            }
        }
    }

    fn is_function_with_receiver_type(
        &self,
        definition_id: NodeId,
        receiver_type_id: TypeId,
    ) -> bool {
        if !self
            .binder
            .find_definition_by_id(definition_id)
            .is_some_and(|definition| matches!(definition, Definition::Function(_)))
        {
            // definition is not a function
            return false;
        }

        let &Typing::Resolved(definition_type_id) = self.binder.node_typing(definition_id) else {
            // definition type cannot be resolved
            return false;
        };
        let Type::Function(function_type) = self.types.get_type_by_id(definition_type_id) else {
            unreachable!("type of function definition is not a function");
        };
        // receiver needs to be convertible to the first parameter type, which
        // an externally visible library function receives through the ABI
        function_type
            .parameter_types
            .first()
            .is_some_and(|type_id| {
                self.types.implicitly_convertible_to_for_call(
                    receiver_type_id,
                    *type_id,
                    function_type.is_externally_visible(),
                )
            })
    }

    /// Resolves the function a user-defined operator expression invokes and
    /// records it in the binder. The operand must be a user-defined value
    /// type with exactly one bound function taking `operand_count` operands.
    pub(super) fn resolve_operator_function(
        &mut self,
        node_id: NodeId,
        operator: UsingOperator,
        operand_type_id: Option<TypeId>,
        operand_count: usize,
    ) {
        let Some(operand_type_id) = operand_type_id.filter(|&type_id| {
            matches!(
                self.types.get_type_by_id(type_id),
                Type::UserDefinedValue(_)
            )
        }) else {
            return;
        };

        let mut candidates = Vec::new();
        for directive in self.active_using_directives_for_type(operand_type_id) {
            let UsingDirective::SingleTypeOperator {
                operator_mapping, ..
            } = directive
            else {
                continue;
            };
            for &definition_id in operator_mapping.get(&operator).into_iter().flatten() {
                if !candidates.contains(&definition_id)
                    && self.is_operator_function(definition_id, operand_type_id, operand_count)
                {
                    candidates.push(definition_id);
                }
            }
        }

        if let [definition_id] = candidates[..] {
            self.binder.set_operator_function(node_id, definition_id);
        }
    }

    /// Whether the definition is a function taking exactly `operand_count`
    /// parameters of the operand's type. A `-` bound to both a unary and a
    /// binary function disambiguates by the parameter count.
    fn is_operator_function(
        &self,
        definition_id: NodeId,
        operand_type_id: TypeId,
        operand_count: usize,
    ) -> bool {
        let Some(Definition::Function(_)) = self.binder.find_definition_by_id(definition_id) else {
            return false;
        };
        let &Typing::Resolved(definition_type_id) = self.binder.node_typing(definition_id) else {
            return false;
        };
        let Type::Function(function_type) = self.types.get_type_by_id(definition_type_id) else {
            unreachable!("type of function definition is not a function");
        };
        function_type.parameter_types.len() == operand_count
            && function_type
                .parameter_types
                .iter()
                .all(|parameter_type_id| *parameter_type_id == operand_type_id)
    }

    pub(super) fn resolve_modifier_invocation(
        &mut self,
        modifier_invocation: &ir::ModifierInvocation,
    ) {
        // Resolved before entering the function's scope, so this starts at
        // the enclosing contract, or at the file for a free function.
        let identifier_path = &modifier_invocation.name;
        let path_resolution = resolve_identifier_path_as_type_in_scope(
            self.binder,
            identifier_path,
            self.current_scope_id(),
        );
        // The overrides of a virtual modifier are narrowed down below, but
        // any other set of declarations names no single modifier.
        let is_modifier_override_set = match &path_resolution.resolution {
            Resolution::Ambiguous(definition_ids) => definition_ids.iter().all(|definition_id| {
                matches!(
                    self.binder.find_definition_by_id(*definition_id),
                    Some(Definition::Modifier(_))
                )
            }),
            _ => false,
        };
        if let Some((identifier, kind)) =
            path_resolution.unresolved_identifier(identifier_path, is_modifier_override_set)
        {
            self.push_diagnostic(identifier, kind);
        }

        // TODO(validation) SDR[35]: the found definition(s) must be modifiers
        // and be in the current contract hierarchy. We could potentially
        // verify that the initial symbol lookup is reachable from the
        // contract only (ie. it's a contract modifier, a modifier in a
        // base, or it's the identifier of a base of the current contract)

        // A virtual modifier resolves to all of its overrides, most derived
        // first. Keep that one: which override runs is decided per compiled
        // contract, see `SemanticContext::resolve_modifier`.
        if is_modifier_override_set
            && let Resolution::Ambiguous(definition_ids) = &path_resolution.resolution
        {
            let name = identifier_path
                .last()
                .expect("an identifier path has at least one identifier");
            self.binder
                .fixup_reference(name.id(), Resolution::Definition(definition_ids[0]));
        }
    }

    /// Resolves the event of an `emit` or the error of a `revert`, reporting
    /// its first identifier that names nothing, or else a last identifier
    /// that isn't reachable through the contract qualifying it.
    pub(super) fn resolve_emit_or_revert_target(
        &mut self,
        identifier_path: &ir::IdentifierPath,
        last_may_be_ambiguous: bool,
    ) {
        let path_resolution = resolve_identifier_path_as_expression_in_scope(
            self.binder,
            identifier_path,
            self.current_scope_id(),
        );
        if let Some((identifier, kind)) =
            path_resolution.unresolved_identifier(identifier_path, last_may_be_ambiguous)
        {
            self.push_diagnostic(identifier, kind);
        } else if let Some(member) = self.unreachable_qualified_member(identifier_path) {
            self.binder
                .fixup_reference(member.id(), Resolution::Unresolved);
            let name = member.unparse().to_owned();
            self.push_diagnostic(member, MemberNotFound { name });
        }
    }

    /// The last identifier of `identifier_path` when it names a member of a
    /// contract or interface that isn't reachable through that contract's
    /// name from here, eg. the `E` of `emit D.E()` outside `D`'s hierarchy.
    fn unreachable_qualified_member<'a>(
        &self,
        identifier_path: &'a ir::IdentifierPath,
    ) -> Option<&'a ir::Identifier> {
        let [.., qualifier, member] = identifier_path.as_ref() else {
            return None;
        };
        let qualifier_id = self
            .followed_resolution_of_reference(qualifier.id())?
            .as_definition_id()?;
        if !self.is_foreign_contract_or_interface(qualifier_id) {
            return None;
        }
        let member_ids = self
            .followed_resolution_of_reference(member.id())?
            .get_definition_ids();
        let is_unreachable = !member_ids.is_empty()
            && member_ids
                .iter()
                .all(|member_id| !self.is_reachable_from_outside_hierarchy(*member_id));
        is_unreachable.then_some(member)
    }

    /// The resolution of the declaration named by the reference at
    /// `identifier_node_id`, with symbol aliases followed.
    pub(super) fn followed_resolution_of_reference(
        &self,
        identifier_node_id: NodeId,
    ) -> Option<Resolution> {
        self.binder
            .find_reference_by_identifier_node_id(identifier_node_id)
            .map(|reference| {
                self.binder
                    .follow_symbol_aliases(reference.resolution.clone())
            })
    }

    pub(super) fn resolve_named_arguments(
        &mut self,
        named_arguments: &[ir::NamedArgument],
        definition_id: Option<NodeId>,
    ) {
        let parameters_scope_id = definition_id.and_then(|definition_id| {
            self.binder
                .get_parameters_scope_for_definition(definition_id)
        });

        for named_argument in named_arguments {
            let identifier = &named_argument.name;
            let resolution =
                parameters_scope_id.map_or(Resolution::Unresolved, |parameters_scope_id| {
                    self.binder
                        .resolve_in_scope_as_namespace(parameters_scope_id, identifier.unparse())
                });
            let reference = Reference::new(Arc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
    }
}
