use std::collections::HashSet;
use std::rc::Rc;

use super::{Pass, ScopeFrame};
use crate::binder::{
    Definition, Reference, Resolution, ResolveOptions, ScopeId, Typing, UsingDirective,
};
use crate::ir::{self, NodeId};
use crate::types::{FunctionType, Type, TypeId};

/// Lexical style resolution of symbols
impl Pass<'_> {
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
                    Definition::StateVariable(_) => Resolution::Definition(first_id),
                    Definition::Constant(_) => Resolution::Definition(first_id),
                    _ => resolution,
                }
            }
            Resolution::Definition(_) | Resolution::BuiltIn(_) => resolution,
        }
    }

    fn active_using_directives_for_type(
        &self,
        type_id: TypeId,
    ) -> impl Iterator<Item = &UsingDirective> {
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
                            continue;
                        }
                        seen_function_types.push(function_type);
                    }
                }
                Definition::StateVariable(state_variable) => {
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
            Typing::Undetermined(type_ids) => self.resolve_symbol_in_type(type_ids[0], symbol),
            Typing::Resolved(type_id) => self.resolve_symbol_in_type(*type_id, symbol),
            Typing::This | Typing::Super => {
                if let Some(scope_id) = self.current_contract_scope_id() {
                    let node_id = self.binder.get_scope_by_id(scope_id).node_id();
                    let options = if matches!(typing, Typing::This) {
                        ResolveOptions::This(node_id)
                    } else {
                        ResolveOptions::Super(node_id)
                    };
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
            return false;
        }

        let Typing::Resolved(definition_type_id) = self.binder.node_typing(definition_id) else {
            return false;
        };
        let Type::Function(function_type) = self.types.get_type_by_id(definition_type_id) else {
            unreachable!("type of function definition is not a function");
        };
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

            let resolution = if index == identifier_path.len() - 1 {
                self.resolve_first_modifier(&resolution)
            } else {
                resolution
            };

            scope_id = self
                .binder
                .follow_symbol_aliases(&resolution)
                .as_definition_id()
                .and_then(|definition_id| self.binder.scope_id_for_node_id(definition_id));

            let reference = Reference::new(Rc::clone(identifier), resolution);
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
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
    }

    // This is a "top-level" resolution method for symbols in a Yul context
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
