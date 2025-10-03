use std::collections::HashSet;
use std::rc::Rc;

use super::{Pass, ScopeFrame};
use crate::backend::binder::{
    Definition, EitherIter, Reference, Resolution, ResolveOptions, ScopeId, Typing, UsingDirective,
};
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::types::{FunctionType, Type, TypeId};
use crate::cst::NodeId;
use crate::utils::versions::{VERSION_0_5_0, VERSION_0_7_0};

/// Lexical style resolution of symbols
impl Pass {
    // This is a "top-level" (ie. not a member access) resolution method
    pub(super) fn resolve_symbol_in_scope(&self, scope_id: ScopeId, symbol: &str) -> Resolution {
        let resolution = self.binder.resolve_in_scope(scope_id, symbol);
        match &resolution {
            Resolution::Unresolved => self.built_ins_resolver().lookup_global(symbol).into(),
            Resolution::Ambiguous(definition_ids) => {
                // Try to disambiguate known cases
                let first_id = definition_ids.first().copied().unwrap();
                let first_definition = self.binder.find_definition_by_id(first_id).unwrap();

                match first_definition {
                    Definition::StateVariable(_) => {
                        // TODO(validation): the state variable should have the
                        // `override` attribute and the rest of the definitions
                        // should be either functions with the correct
                        // signature, state variables or private variables or
                        // constants
                        Resolution::Definition(first_id)
                    }
                    Definition::Constant(_) => {
                        // TODO(validation): if there are other definitions in
                        // base contracts, they should be marked private and
                        // they should be constants or state variables
                        Resolution::Definition(first_id)
                    }
                    _ => {
                        // TODO(validation): check that the returned definitions are
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
                    if self.language_version < VERSION_0_7_0 {
                        // In Solidity < 0.7.0 using directives are inherited in contracts,
                        // so we need to pull any `using` directives in a contract hierarchy
                        // if there are linearisations
                        EitherIter::Left(
                            self.binder
                                .get_using_directives_in_scope_including_inherited(
                                    *lexical_scope_id,
                                ),
                        )
                    } else {
                        EitherIter::Right(
                            self.binder.get_using_directives_in_scope(*lexical_scope_id),
                        )
                    }
                },
            )
            // ... and add the global directives
            .chain(self.binder.get_global_using_directives())
            .filter(move |directive| directive.applies_to(type_id))
    }

    pub(super) fn filter_overriden_definitions(&self, resolution: Resolution) -> Resolution {
        let Resolution::Ambiguous(definition_ids) = resolution else {
            return resolution;
        };
        let mut seen_function_types: Vec<&FunctionType> = Vec::new();
        let mut filtered_definitions = Vec::new();
        for definition_id in definition_ids {
            match self.binder.find_definition_by_id(definition_id).unwrap() {
                Definition::Function(_) => {
                    if let Typing::Resolved(type_id) = self.binder.node_typing(definition_id) {
                        let Type::Function(function_type) = self.types.get_type_by_id(type_id)
                        else {
                            unreachable!("type of function definition is not a function");
                        };
                        if seen_function_types.iter().any(|seen_function_type| {
                            self.types
                                .function_type_overrides(seen_function_type, function_type)
                        }) {
                            // the function type is overriden by some other previously seen definition
                            continue;
                        }
                        seen_function_types.push(function_type);
                    }
                }
                Definition::StateVariable(state_variable) => {
                    // remember the getter type if present to override functions
                    // in bases
                    if let Some(getter_type_id) = state_variable.getter_type_id {
                        let Type::Function(getter_type) = self.types.get_type_by_id(getter_type_id)
                        else {
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

    pub(super) fn resolve_symbol_in_typing(&self, typing: &Typing, symbol: &str) -> Resolution {
        match typing {
            Typing::Unresolved => Resolution::Unresolved,
            Typing::Undetermined(type_ids) => {
                // We cannot use argument-type disambiguation here, so we will
                // use the first result
                // TODO(validation): check that the types are consistent (eg.
                // they are all function types) and that it makes sense to use
                // the first one
                self.resolve_symbol_in_type(type_ids[0], symbol)
            }
            Typing::Resolved(type_id) => self.resolve_symbol_in_type(*type_id, symbol),
            Typing::This | Typing::Super => {
                // TODO: the contract scope here is not necessarily the current
                // lexical scope; for compilation we should set it to the scope
                // of the contract being compiled, as this will affect the
                // linearisation and hence the result of this `super`
                // resolution. This affects the first parameter to
                // `resolve_in_contract_scope`, not the `node_id` of the
                // resolution option which is always lexical.
                if let Some(scope_id) = self.current_contract_scope_id() {
                    let node_id = self.binder.get_scope_by_id(scope_id).node_id();
                    let options = if matches!(typing, Typing::This) {
                        ResolveOptions::This(node_id)
                    } else {
                        ResolveOptions::Super(node_id)
                    };
                    // TODO(validation): for `this` resolutions we need to check
                    // that the returned definitions are externally available
                    // (ie. either `external` or `public`)
                    let mut definition_ids = self
                        .binder
                        .resolve_in_contract_scope(scope_id, symbol, options)
                        .get_definition_ids();

                    if matches!(typing, Typing::This) {
                        // Consider active `using` directives for `this`
                        if let Some(receiver_type_id) = self.types.find_type(&Type::Contract {
                            definition_id: node_id,
                        }) {
                            self.add_attached_functions_for_type(
                                receiver_type_id,
                                symbol,
                                &mut definition_ids,
                            );
                        }
                    }

                    if self.language_version < VERSION_0_5_0 && definition_ids.is_empty() {
                        // In Solidity < 0.5.0, `this` can be used as an address
                        Resolution::from(
                            self.built_ins_resolver()
                                .lookup_member_of_type_id(self.types.address(), symbol),
                        )
                    } else {
                        Resolution::from(definition_ids)
                    }
                } else {
                    Resolution::Unresolved
                }
            }
            Typing::NewExpression(type_id) => {
                // Special case: resolve legacy constructor call options (ie.
                // `(new Lock).value(1)()`)
                if self.language_version < VERSION_0_7_0 {
                    Resolution::from(
                        self.built_ins_resolver()
                            .lookup_member_of_new_type_id(*type_id, symbol),
                    )
                } else {
                    Resolution::Unresolved
                }
            }
            Typing::MetaType(type_) => {
                if let Some(built_in) = self
                    .built_ins_resolver()
                    .lookup_member_of_meta_type(type_, symbol)
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
                    _ => self
                        .built_ins_resolver()
                        .lookup_member_of_user_definition(definition, symbol)
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
            Type::Contract { definition_id, .. } | Type::Interface { definition_id, .. } => {
                let scope_id = self.binder.scope_id_for_node_id(*definition_id).unwrap();
                self.binder
                    .resolve_in_contract_scope(scope_id, symbol, ResolveOptions::External)
                    .or_else(|| {
                        self.built_ins_resolver()
                            .lookup_member_of_type_id(type_id, symbol)
                            .into()
                    })
            }
            Type::Struct { definition_id, .. } => {
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
        let mut seen_ids = HashSet::new();
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
        modifier_invocation: &input_ir::ModifierInvocation,
    ) {
        let identifier_path = &modifier_invocation.name;
        let mut scope_id = self.current_contract_scope_id();
        let mut use_contract_lookup = true;
        for (index, identifier) in identifier_path.iter().enumerate() {
            let resolution = if let Some(scope_id) = scope_id {
                let symbol = identifier.unparse();
                if use_contract_lookup {
                    use_contract_lookup = false;
                    self.resolve_symbol_in_scope(scope_id, &symbol)
                } else {
                    self.binder.resolve_in_scope_as_namespace(scope_id, &symbol)
                }
            } else {
                Resolution::Unresolved
            };

            // TODO(validation): the found definition(s) must be modifiers
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

            scope_id = resolution
                .as_definition_id()
                .and_then(|definition_id| self.binder.scope_id_for_node_id(definition_id));

            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
    }

    pub(super) fn resolve_named_arguments(
        &mut self,
        named_arguments: &[input_ir::NamedArgument],
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
                        .resolve_in_scope_as_namespace(parameters_scope_id, &identifier.unparse())
                });
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
    }

    // This is a "top-level" resolution method for symbols in s Yul context
    pub(super) fn resolve_symbol_in_yul_scope(
        &self,
        scope_id: ScopeId,
        symbol: &str,
    ) -> Resolution {
        let resolution =
            self.filter_overriden_definitions(self.binder.resolve_in_scope(scope_id, symbol));
        if resolution == Resolution::Unresolved {
            self.built_ins_resolver().lookup_yul_global(symbol).into()
        } else {
            resolution
        }
    }

    pub(super) fn resolve_yul_suffix(
        &self,
        symbol: &str,
        parent_resolution: &Resolution,
    ) -> Resolution {
        match parent_resolution {
            Resolution::Definition(node_id) => {
                if let Some(definition) = self.binder.find_definition_by_id(*node_id) {
                    self.built_ins_resolver()
                        .lookup_yul_suffix(definition, symbol)
                        .into()
                } else {
                    Resolution::Unresolved
                }
            }
            Resolution::Unresolved | Resolution::Ambiguous(_) | Resolution::BuiltIn(_) => {
                Resolution::Unresolved
            }
        }
    }
}
