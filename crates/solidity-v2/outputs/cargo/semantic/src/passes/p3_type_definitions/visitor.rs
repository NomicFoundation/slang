use std::collections::HashMap;
use std::rc::Rc;

use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::visitor::Visitor;

use super::evaluator::evaluate_compile_time_uint_constant;
use super::Pass;
use crate::binder::{Definition, Reference, Resolution, Scope, Typing, UsingDirective};
use crate::built_ins::BuiltIn;
use crate::types::{DataLocation, Type};

impl Visitor for Pass<'_> {
    fn enter_source_unit(&mut self, node: &ir::SourceUnit) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_source_unit(&mut self, node: &ir::SourceUnit) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_contract_definition(&mut self, node: &ir::ContractDefinition) -> bool {
        self.enter_scope_for_node_id(node.id());

        let type_id = self.types.register_type(Type::Contract {
            definition_id: node.id(),
        });
        self.current_receiver_type = Some(type_id);

        if let Some(bases) = self.binder.get_linearised_bases(node.id()) {
            if !bases.is_empty() {
                self.register_super_types(type_id, &bases.clone());
            }
        }

        true
    }

    fn leave_contract_definition(&mut self, node: &ir::ContractDefinition) {
        self.leave_scope_for_node_id(node.id());

        self.current_receiver_type = None;
        self.binder.mark_user_meta_type_node(node.id());

        if let Some(base_slot_expression) = &node.storage_layout {
            // TODO(validation): if the base slot expression cannot be computed
            // at this time, it's not a compile time constant and hence it's an
            // error
            if let Some(base_slot) = evaluate_compile_time_uint_constant(
                base_slot_expression,
                self.current_contract_or_file_scope_id(),
                self,
            ) {
                let Definition::Contract(contract_definition) =
                    self.binder.get_definition_mut(node.id())
                else {
                    unreachable!("the definition is not a contract");
                };
                contract_definition.base_slot = Some(base_slot);
            }
        }
    }

    fn enter_interface_definition(&mut self, node: &ir::InterfaceDefinition) -> bool {
        self.enter_scope_for_node_id(node.id());

        let type_id = self.types.register_type(Type::Interface {
            definition_id: node.id(),
        });
        self.current_receiver_type = Some(type_id);

        if let Some(bases) = self.binder.get_linearised_bases(node.id()) {
            if !bases.is_empty() {
                self.register_super_types(type_id, &bases.clone());
            }
        }

        true
    }

    fn leave_interface_definition(&mut self, node: &ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.id());

        self.current_receiver_type = None;
        self.binder.mark_user_meta_type_node(node.id());
    }

    fn enter_library_definition(&mut self, node: &ir::LibraryDefinition) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_library_definition(&mut self, node: &ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.id());

        self.binder.mark_user_meta_type_node(node.id());
    }

    fn leave_path_import(&mut self, node: &ir::PathImport) {
        if node.alias.is_some() {
            self.binder.mark_user_meta_type_node(node.id());
        }
    }

    fn enter_import_deconstruction(&mut self, node: &ir::ImportDeconstruction) -> bool {
        for symbol in &node.symbols {
            // find the associated definition to get the imported file ID
            let Some(Definition::ImportedSymbol(imported_symbol)) =
                self.binder.find_definition_by_id(symbol.id())
            else {
                unreachable!("expected to find definition associated to imported symbol");
            };
            // now we can get the target scope ID
            let scope_id = imported_symbol
                .resolved_file_id
                .as_ref()
                .and_then(|file_id| self.binder.scope_id_for_file_id(file_id));

            let resolution = scope_id.map_or(Resolution::Unresolved, |scope_id| {
                self.binder
                    .resolve_in_scope(scope_id, symbol.name.string_id)
            });
            let reference = Reference::new(Rc::clone(&symbol.name), resolution);
            self.binder.insert_reference(reference);
        }

        false
    }

    fn leave_function_definition(&mut self, node: &ir::FunctionDefinition) {
        // type parameters and return variables
        self.visit_parameters(&node.parameters);
        if let Some(returns) = &node.returns {
            self.visit_parameters(returns);
        }

        // now we can compute the function type
        let type_id = self.type_of_function_definition(node, self.current_receiver_type);
        self.binder.set_node_type(node.id(), type_id);

        if !matches!(node.kind, ir::FunctionKind::Modifier) && node.name.is_some() {
            // for non-modifier *named* functions, fill-in parameter types in
            // parameters scope for overload disambiguation in p5
            let parameter_types: Vec<_> = node
                .parameters
                .iter()
                .map(|parameter| self.binder.node_typing(parameter.id()).as_type_id())
                .collect();

            let Some(parameters_scope_id) =
                self.binder.get_parameters_scope_for_definition(node.id())
            else {
                unreachable!("FunctionDefinition does not have associated parameters scope");
            };
            let Scope::Parameters(ref mut parameters_scope) =
                self.binder.get_scope_mut(parameters_scope_id)
            else {
                unreachable!("scope is not a ParametersScope");
            };
            parameters_scope.set_parameter_types(&parameter_types);
        }
    }

    fn leave_function_type(&mut self, node: &ir::FunctionType) {
        self.visit_parameters(&node.parameters);
        if let Some(returns) = &node.returns {
            self.visit_parameters(returns);
        }
    }

    fn leave_try_statement(&mut self, node: &ir::TryStatement) {
        if let Some(returns) = &node.returns {
            self.visit_parameters(returns);
        }
    }

    fn leave_catch_clause_error(&mut self, node: &ir::CatchClauseError) {
        self.visit_parameters(&node.parameters);
    }

    fn enter_type_name(&mut self, node: &ir::TypeName) -> bool {
        if let ir::TypeName::IdentifierPath(identifier_path) = node {
            self.resolve_identifier_path(identifier_path);
            false
        } else {
            true
        }
    }

    fn enter_override_paths(&mut self, items: &ir::OverridePaths) -> bool {
        for identifier_path in items {
            self.resolve_identifier_path(identifier_path);
        }
        false
    }

    fn leave_event_definition(&mut self, node: &ir::EventDefinition) {
        self.binder.mark_user_meta_type_node(node.id());

        // Resolve and collect the types of the parameters, saving them in the
        // scope to use in overload disambiguation when invoking an event as a
        // function
        let mut parameter_types = Vec::new();
        for parameter in &node.parameters {
            // TODO: the data location is not strictly correct, but strings, bytes
            // and structs are allowed as event parameters and they won't type if we
            // pass None here
            let type_id = self.resolve_type_name(&parameter.type_name, Some(DataLocation::Memory));
            self.binder.set_node_type(parameter.id(), type_id);
            parameter_types.push(type_id);
        }

        let Some(parameters_scope_id) = self.binder.get_parameters_scope_for_definition(node.id())
        else {
            unreachable!("EventDefinition does not have associated parameters scope");
        };
        let Scope::Parameters(ref mut parameters_scope) =
            self.binder.get_scope_mut(parameters_scope_id)
        else {
            unreachable!("scope is not a ParametersScope");
        };
        parameters_scope.set_parameter_types(&parameter_types);
    }

    fn leave_error_definition(&mut self, node: &ir::ErrorDefinition) {
        self.binder.mark_user_meta_type_node(node.id());

        // Resolve the types of the parameters
        for parameter in &node.parameters {
            // TODO: the data location is not strictly correct, but strings, bytes
            // and structs are allowed as error parameters and they won't type if we
            // pass None here
            let type_id = self.resolve_type_name(&parameter.type_name, Some(DataLocation::Memory));
            self.binder.set_node_type(parameter.id(), type_id);
        }
    }

    fn leave_state_variable_definition(&mut self, node: &ir::StateVariableDefinition) {
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Storage));
        self.binder.set_node_type(node.id(), type_id);
        // if required, we will compute the type of the getter after all
        // definitions have been typed
    }

    fn leave_constant_definition(&mut self, node: &ir::ConstantDefinition) {
        let type_id = self.resolve_type_name(&node.type_name, None);
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_variable_declaration_statement(&mut self, node: &ir::VariableDeclarationStatement) {
        match &node.target {
            ir::VariableDeclarationTarget::SingleTypedDeclaration(single) => {
                let type_id = self.resolve_type_name(
                    &single.declaration.type_name,
                    single.declaration.storage_location.as_ref().map(Into::into),
                );
                self.binder.set_node_type(single.declaration.id(), type_id);
            }
            ir::VariableDeclarationTarget::MultiTypedDeclaration(multi) => {
                for element in &multi.elements {
                    if let Some(declaration) = &element.member {
                        let type_id = self.resolve_type_name(
                            &declaration.type_name,
                            declaration.storage_location.as_ref().map(Into::into),
                        );
                        self.binder.set_node_type(declaration.id(), type_id);
                    }
                }
            }
        }
    }

    fn leave_struct_definition(&mut self, node: &ir::StructDefinition) {
        self.binder.mark_user_meta_type_node(node.id());
    }

    fn leave_struct_member(&mut self, node: &ir::StructMember) {
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Inherited));
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_enum_definition(&mut self, node: &ir::EnumDefinition) {
        let type_id = self.types.register_type(Type::Enum {
            definition_id: node.id(),
        });
        for member in &node.members {
            self.binder.set_node_type(member.id(), Some(type_id));
        }

        self.binder.mark_user_meta_type_node(node.id());
    }

    fn leave_user_defined_value_type_definition(
        &mut self,
        node: &ir::UserDefinedValueTypeDefinition,
    ) {
        self.binder.mark_user_meta_type_node(node.id());

        let target_type_id = self.type_of_elementary_type(&node.value_type, None);
        let Definition::UserDefinedValueType(udvt) = self.binder.get_definition_mut(node.id())
        else {
            unreachable!("definition in UDVT node is not a UDVT");
        };
        udvt.target_type_id = target_type_id;
    }

    fn enter_using_directive(&mut self, node: &ir::UsingDirective) -> bool {
        match &node.clause {
            ir::UsingClause::IdentifierPath(identifier_path) => {
                self.resolve_identifier_path(identifier_path);
            }
            ir::UsingClause::UsingDeconstruction(using_deconstruction) => {
                for symbol in &using_deconstruction.symbols {
                    self.resolve_identifier_path(&symbol.name);
                }
            }
        }
        // target's TypeName is resolved when entering it
        true
    }

    fn leave_using_directive(&mut self, node: &ir::UsingDirective) {
        let directive = match &node.target {
            ir::UsingTarget::TypeName(type_name) => {
                let Some(type_id) =
                    // TODO: not sure we should be using DataLocation::Inherited
                    // here, but if we don't provide one we can't register
                    // reference types and hence we cannot get a type_id
                    self.resolve_type_name(type_name, Some(DataLocation::Inherited))
                else {
                    return;
                };
                match &node.clause {
                    ir::UsingClause::IdentifierPath(identifier_path) => {
                        let Some(scope_id) =
                            self.find_library_scope_id_for_identifier_path(identifier_path)
                        else {
                            return;
                        };
                        UsingDirective::new_single_type(scope_id, type_id)
                    }
                    ir::UsingClause::UsingDeconstruction(using_deconstruction) => {
                        let mut symbols = HashMap::new();
                        let mut operators = HashMap::new();

                        for symbol in &using_deconstruction.symbols {
                            let symbol_name = symbol.name.last().unwrap();
                            let definition_ids = self
                                .binder
                                .find_reference_by_identifier_node_id(symbol_name.id())
                                .map_or(Vec::new(), |reference| {
                                    reference.resolution.get_definition_ids()
                                });
                            // TODO(validation): *all* definitions should point to functions

                            symbols.insert(symbol_name.string_id, definition_ids);

                            if let Some(operator) = &symbol.alias {
                                operators.insert(*operator, symbol_name.string_id);
                            }
                        }

                        let scope = Scope::new_using(node.id(), symbols);
                        let scope_id = self.binder.insert_scope(scope);

                        if operators.is_empty() {
                            UsingDirective::new_single_type(scope_id, type_id)
                        } else {
                            UsingDirective::new_single_type_with_operators(
                                scope_id, type_id, operators,
                            )
                        }
                    }
                }
            }
            ir::UsingTarget::Asterisk => {
                let ir::UsingClause::IdentifierPath(identifier_path) = &node.clause else {
                    // only libraries can be attached to all types
                    return;
                };
                let Some(scope_id) =
                    self.find_library_scope_id_for_identifier_path(identifier_path)
                else {
                    // the identifier path does not point to a valid library
                    return;
                };
                UsingDirective::new_all(scope_id)
            }
        };

        if node.global_keyword {
            self.binder.insert_global_using_directive(directive);
        } else {
            // TODO(validation): `using` directives are not allowed in interfaces since 0.7.1
            let current_scope_id = self.current_contract_or_file_scope_id();
            self.binder
                .insert_using_directive_in_scope(directive, current_scope_id);
        }
    }

    fn enter_revert_statement(&mut self, node: &ir::RevertStatement) -> bool {
        self.resolve_identifier_path(&node.error);
        true
    }

    fn leave_revert_statement(&mut self, node: &ir::RevertStatement) {
        let type_id = self.type_of_identifier_path(&node.error, None);
        self.binder.set_node_type(node.id(), type_id);
    }

    fn enter_emit_statement(&mut self, node: &ir::EmitStatement) -> bool {
        self.resolve_identifier_path(&node.event);
        true
    }

    fn leave_emit_statement(&mut self, node: &ir::EmitStatement) {
        let type_id = self.type_of_identifier_path(&node.event, None);
        self.binder.set_node_type(node.id(), type_id);
    }

    fn enter_catch_clause_error(&mut self, node: &ir::CatchClauseError) -> bool {
        if let Some(name) = &node.name {
            let resolution = match name.unparse() {
                "Error" | "Panic" => Resolution::BuiltIn(BuiltIn::ErrorOrPanic),
                _ => Resolution::Unresolved,
            };
            let reference = Reference::new(Rc::clone(name), resolution);
            self.binder.insert_reference(reference);
        }
        true
    }

    fn leave_new_expression(&mut self, node: &ir::NewExpression) {
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Memory));
        let typing = type_id.map_or(Typing::Unresolved, Typing::NewExpression);
        self.binder.set_node_typing(node.id(), typing);
    }

    fn leave_type_expression(&mut self, node: &ir::TypeExpression) {
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Memory));
        let typing = type_id.map_or(Typing::Unresolved, |type_id| {
            Typing::BuiltIn(BuiltIn::Type(type_id))
        });
        self.binder.set_node_typing(node.id(), typing);
    }
}
