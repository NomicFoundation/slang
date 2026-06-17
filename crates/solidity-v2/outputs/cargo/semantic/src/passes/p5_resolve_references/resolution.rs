use std::sync::Arc;

use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::{Pass, ScopeFrame};
use crate::binder::{
    Definition, Reference, Resolution, ResolveOptions, ScopeId, Typing, UsingDirective,
};
use crate::built_ins::BuiltInsResolver;
use crate::types::{ContractType, InterfaceType, StructType, Type, TypeId};

/// Lexical style resolution of symbols
impl Pass<'_> {
    // This is a "top-level" (ie. not a member access) resolution method
    pub(super) fn resolve_symbol_in_scope(&self, scope_id: ScopeId, symbol: &str) -> Resolution {
        let resolution = self.binder.resolve_in_scope(scope_id, symbol);
        match &resolution {
            Resolution::Unresolved => BuiltInsResolver::lookup_global(symbol).into(),
            Resolution::Ambiguous(definition_ids) => {
                // Try to disambiguate known cases
                let first_id = definition_ids.first().copied().unwrap();
                let first_definition = self.binder.find_definition_by_id(first_id).unwrap();

                match first_definition {
                    Definition::StateVariable(_) => {
                        // TODO(validation) SDR[36]: the state variable should have the
                        // `override` attribute and the rest of the definitions
                        // should be either functions with the correct
                        // signature, state variables or private variables or
                        // constants
                        Resolution::Definition(first_id)
                    }
                    Definition::Constant(_) => {
                        // TODO(validation) SDR[33]: if there are other definitions in
                        // base contracts, they should be marked private and
                        // they should be constants or state variables
                        Resolution::Definition(first_id)
                    }
                    _ => {
                        // TODO(validation) SDR[32]: check that the returned definitions are
                        // all functions (or maybe modifiers?)
                        resolution
                    }
                }
            }
            Resolution::Definition(_) | Resolution::BuiltIn(_) => resolution,
        }
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

    pub(super) fn resolve_symbol_in_typing(&self, typing: &Typing, symbol: &str) -> Resolution {
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
                    if let Typing::This(receiver_type_id) = typing {
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
            Typing::MetaType(type_) => {
                if let Some(built_in) = BuiltInsResolver::lookup_member_of_meta_type(type_, symbol)
                {
                    Resolution::BuiltIn(built_in)
                } else {
                    Resolution::Unresolved
                }
            }
            Typing::UserMetaType(node_id) => {
                // We're trying to resolve a member access expression into a
                // type name, ie. this is a meta-type member access
                let Some(definition) = self.binder.find_definition_by_id(*node_id) else {
                    return Resolution::Unresolved;
                };
                match definition {
                    Definition::Import(import_definition) => {
                        if let Some(scope_id) = import_definition
                            .resolved_file_id
                            .as_ref()
                            .and_then(|file_id| self.binder.scope_id_for_file_id(file_id))
                        {
                            self.binder.resolve_in_scope(scope_id, symbol)
                        } else {
                            Resolution::Unresolved
                        }
                    }
                    Definition::Contract(_)
                    | Definition::Enum(_)
                    | Definition::Interface(_)
                    | Definition::Library(_) => {
                        // this is a "namespace" lookup
                        if let Some(scope_id) = self.binder.scope_id_for_node_id(*node_id) {
                            self.binder.resolve_in_scope_as_namespace(scope_id, symbol)
                        } else {
                            Resolution::Unresolved
                        }
                    }
                    _ => BuiltInsResolver::lookup_member_of_user_definition(definition, symbol)
                        .into(),
                }
            }
            Typing::BuiltIn(built_in) => self
                .built_ins_resolver()
                .lookup_member_of(built_in, symbol)
                .into(),
        }
    }

    fn resolve_symbol_in_type(&self, type_id: TypeId, symbol: &str) -> Resolution {
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
            let ids = self
                .binder
                .resolve_in_scope_as_namespace(scope_id, symbol)
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
        let mut use_contract_lookup = true;
        for (index, identifier) in identifier_path.iter().enumerate() {
            let resolution = if let Some(scope_id) = scope_id {
                let symbol = identifier.unparse();
                if use_contract_lookup {
                    use_contract_lookup = false;
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

            // NOTE: we need to follow symbol aliases to resolve the next scope to use
            scope_id = self
                .binder
                .follow_symbol_aliases(&resolution)
                .as_definition_id()
                .and_then(|definition_id| self.binder.scope_id_for_node_id(definition_id));

            let reference = Reference::new(Arc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
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
}
