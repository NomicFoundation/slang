use std::sync::Arc;

use ruint::aliases::U256;
use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    StorageLayoutBaseNonInteger, StorageLayoutBaseNotConstant, StorageLayoutBaseOutOfRange,
};
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::{NodeIdentity, TextRange};

use super::{Pass, ScopeFrame};
use crate::binder::{
    Definition, Reference, Resolution, ResolveOptions, ScopeId, Typing, UsingDirective,
    UsingOperator,
};
use crate::built_ins::BuiltInsResolver;
use crate::passes::common::constant_evaluator::{
    evaluate_compile_time_constant, ConstantResolver, EvaluationError,
};
use crate::passes::common::{find_definition_namespace_scope_id, node_location};
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
            .iter()
            .rev()
            .flat_map(
                |ScopeFrame {
                     lexical_scope_id, ..
                 }| {
                    self.binder.get_using_directives_in_scope(*lexical_scope_id)
                },
            )
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
            Typing::This(_) | Typing::Super => {
                // TODO: the contract scope here is not necessarily the current
                // lexical scope; for compilation we should set it to the scope
                // of the contract being compiled, as this will affect the
                // linearisation and hence the result of this `super`
                // resolution. This affects the first parameter to
                // `resolve_in_contract_scope`, not the `node_id` of the
                // resolution option which is always lexical.
                if let Some(scope_id) = self.current_contract_scope_id() {
                    let node_id = self.binder.get_scope_by_id(scope_id).node_id();
                    let options = if matches!(typing, Typing::This(_)) {
                        ResolveOptions::This(node_id)
                    } else {
                        ResolveOptions::Super(node_id)
                    };
                    // TODO(validation) SDR[34]: for `this` resolutions we need to check
                    // that the returned definitions are externally available
                    // (ie. either `external` or `public`)
                    let mut definition_ids = self
                        .binder
                        .resolve_in_contract_scope(scope_id, symbol, options)
                        .get_definition_ids();

                    // Consider active `using` directives for `this`
                    if let Typing::This(receiver_type_id) = typing
                        && matches!(
                            self.types.get_type_by_id(*receiver_type_id),
                            Type::Contract(_)
                        )
                    {
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
                find_definition_namespace_scope_id(self.binder, *definition_id)
                    .map_or(Resolution::Unresolved, |scope_id| {
                        self.binder.resolve_in_scope_as_namespace(scope_id, symbol)
                    })
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

    fn add_attached_functions_for_type(
        &self,
        receiver_type_id: TypeId,
        symbol: &str,
        definition_ids: &mut Vec<NodeId>,
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

        let Typing::Resolved(definition_type_id) = self.binder.node_typing(definition_id) else {
            // definition type cannot be resolved
            return false;
        };
        let Type::Function(function_type) = self.types.get_type_by_id(definition_type_id) else {
            unreachable!("type of function definition is not a function");
        };
        // receiver needs to be convertible to the first parameter type; we use
        // the external call rules because the conversion rules with different
        // locations are more relaxed
        function_type
            .parameter_types
            .first()
            .is_some_and(|type_id| {
                self.types
                    .implicitly_convertible_to_for_external_call(receiver_type_id, *type_id)
            })
    }

    /// Resolves the function a user-defined operator expression invokes and
    /// records it in the binder. The operand must be a user-defined value
    /// type with exactly one bound function taking `operand_count` operands.
    pub(super) fn resolve_operator_function(
        &mut self,
        node_id: NodeId,
        operator: UsingOperator,
        operand: &ir::Expression,
        operand_count: usize,
    ) {
        let Some(operand_type_id) =
            self.typing_of_expression(operand)
                .as_type_id()
                .filter(|&type_id| {
                    matches!(
                        self.types.get_type_by_id(type_id),
                        Type::UserDefinedValue(_)
                    )
                })
        else {
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
        let Typing::Resolved(definition_type_id) = self.binder.node_typing(definition_id) else {
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

    fn resolve_first_modifier(&self, resolution: &Resolution) -> Resolution {
        let definition_ids = resolution.get_definition_ids();
        if definition_ids.is_empty() {
            return Resolution::Unresolved;
        }
        // Find the first definition that is either a modifier or a contract
        // type, as that's how bases in constructors are parsed
        definition_ids
            .into_iter()
            .find(|definition_id| {
                matches!(
                    self.binder.find_definition_by_id(*definition_id),
                    Some(Definition::Modifier(_) | Definition::Contract(_))
                )
            })
            .into()
    }

    pub(super) fn resolve_modifier_invocation(
        &mut self,
        modifier_invocation: &ir::ModifierInvocation,
    ) {
        let identifier_path = &modifier_invocation.name;
        let mut scope_id = self.current_contract_scope_id();
        for (index, identifier) in identifier_path.iter().enumerate() {
            let resolution = if let Some(scope_id) = scope_id {
                let symbol = identifier.unparse();
                if index == 0 {
                    // we use lexical/contract resolution only for the first segment in the path
                    self.resolve_symbol_in_scope(scope_id, symbol)
                } else {
                    self.binder.resolve_in_scope_as_namespace(scope_id, symbol)
                }
            } else {
                Resolution::Unresolved
            };

            // TODO(validation) SDR[35]: the found definition(s) must be modifiers
            // and be in the current contract hierarchy. We could potentially
            // verify that the initial symbol lookup is reachable from the
            // contract only (ie. it's a contract modifier, a modifier in a
            // base, or it's the identifier of a base of the current contract)
            let resolution = if index == identifier_path.len() - 1 {
                // The last element should be a modifier and for valid Solidity
                // it overrides all previous definitions in the hierarchy
                self.resolve_first_modifier(&resolution)
            } else {
                resolution
            };

            let reference = Reference::new(Arc::clone(identifier), resolution.clone());
            self.binder.insert_reference(reference);

            // NOTE: we need to follow symbol aliases to resolve the next scope to use
            scope_id = self
                .binder
                .follow_symbol_aliases(resolution)
                .as_definition_id()
                .and_then(|definition_id| {
                    find_definition_namespace_scope_id(self.binder, definition_id)
                });
        }
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

    /// Resolves and validates a contract's `layout at <expr>` storage base
    /// slot via the constant evaluator, emitting diagnostics on failure.
    pub(super) fn resolve_storage_base_slot(
        &mut self,
        node: &ir::ContractDefinition,
        base_slot_expression: &ir::Expression,
    ) {
        let value = match evaluate_compile_time_constant(
            base_slot_expression,
            self.current_scope_id(),
            self.types,
            &ConstantResolver {
                binder: self.binder,
                // Base slots may reference constants declared anywhere.
                use_site: None,
            },
        ) {
            Ok(value) => value,
            Err(EvaluationError::Diagnostic { kind, expression }) => {
                // Report at the failing operation, falling back to the base
                // slot expression when the evaluator didn't attach one.
                self.push_diagnostic(expression.as_ref().unwrap_or(base_slot_expression), kind);
                return;
            }
            Err(EvaluationError::CouldNotEvaluate) => {
                self.push_diagnostic(base_slot_expression, StorageLayoutBaseNotConstant);
                return;
            }
        };

        // Classify the value: non-integer, then out of range (negative or too large).
        let Some(integer) = value.into_integer() else {
            self.push_diagnostic(base_slot_expression, StorageLayoutBaseNonInteger);
            return;
        };
        let Ok(base_slot) = U256::try_from(&integer) else {
            self.push_diagnostic(
                base_slot_expression,
                StorageLayoutBaseOutOfRange {
                    value: integer.to_string(),
                },
            );
            return;
        };

        // TODO(validation) SDR[743]: When this function is called after `ir::visitor::accept_expression`
        // check if type is implicitly convertable to uint256.

        let Definition::Contract(contract_definition) = self.binder.get_definition_mut(node.id())
        else {
            unreachable!("the definition is not a contract");
        };
        contract_definition.base_slot = Some(base_slot);
    }

    /// Emits `kind` located at `node`.
    fn push_diagnostic(
        &mut self,
        node: &(impl NodeIdentity + TextRange),
        kind: impl Into<DiagnosticKind>,
    ) {
        let (file_id, range) = node_location(node, self.file_node_mapper);
        self.diagnostics.push(file_id, range, kind);
    }
}
