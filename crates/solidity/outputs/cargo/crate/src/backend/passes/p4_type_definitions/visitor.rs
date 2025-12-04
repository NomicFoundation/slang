use std::collections::HashMap;
use std::rc::Rc;

use super::Pass;
use crate::backend::binder::{Definition, Reference, Resolution, Scope, Typing, UsingDirective};
use crate::backend::built_ins::BuiltIn;
use crate::backend::ir::ir2_flat_contracts::visitor::Visitor;
use crate::backend::ir::ir2_flat_contracts::{self as input_ir};
use crate::backend::types::{DataLocation, Type};
use crate::cst::TerminalKind;
use crate::utils::versions::VERSION_0_5_0;

impl Visitor for Pass<'_> {
    fn enter_source_unit(&mut self, node: &input_ir::SourceUnit) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_source_unit(&mut self, node: &input_ir::SourceUnit) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);

        let type_id = self.types.register_type(Type::Contract {
            definition_id: node.node_id,
        });
        self.current_receiver_type = Some(type_id);

        if let Some(bases) = self.binder.get_linearised_bases(node.node_id) {
            if !bases.is_empty() {
                self.register_super_types(type_id, &bases.clone());
            }
        }

        true
    }

    fn leave_contract_definition(&mut self, node: &input_ir::ContractDefinition) {
        self.leave_scope_for_node_id(node.node_id);

        self.current_receiver_type = None;
        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);

        let type_id = self.types.register_type(Type::Interface {
            definition_id: node.node_id,
        });
        self.current_receiver_type = Some(type_id);

        if let Some(bases) = self.binder.get_linearised_bases(node.node_id) {
            if !bases.is_empty() {
                self.register_super_types(type_id, &bases.clone());
            }
        }

        true
    }

    fn leave_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.node_id);

        self.current_receiver_type = None;
        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_library_definition(&mut self, node: &input_ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.node_id);

        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn leave_path_import(&mut self, node: &input_ir::PathImport) {
        if node.alias.is_some() {
            self.binder.mark_user_meta_type_node(node.node_id);
        }
    }

    fn enter_import_deconstruction(&mut self, node: &input_ir::ImportDeconstruction) -> bool {
        for symbol in &node.symbols {
            let target_symbol = if let Some(alias) = &symbol.alias {
                alias
            } else {
                &symbol.name
            };
            let resolution = self.binder.resolve_in_scope(
                self.current_contract_or_file_scope_id(),
                &target_symbol.unparse(),
            );
            let reference = Reference::new(Rc::clone(&symbol.name), resolution);
            self.binder.insert_reference(reference);
        }

        false
    }

    fn leave_function_definition(&mut self, node: &input_ir::FunctionDefinition) {
        let default_location = if self.language_version < VERSION_0_5_0 {
            if matches!(
                node.visibility,
                input_ir::FunctionVisibility::External | input_ir::FunctionVisibility::Public
            ) {
                Some(DataLocation::Calldata)
            } else {
                Some(DataLocation::Memory)
            }
        } else {
            None
        };

        // type parameters and return variables
        self.visit_parameters(&node.parameters, default_location);
        if let Some(returns) = &node.returns {
            self.visit_parameters(returns, default_location);
        }

        // now we can compute the function type
        let type_id = self.type_of_function_definition(node, self.current_receiver_type);
        self.binder.set_node_type(node.node_id, type_id);

        if !matches!(node.kind, input_ir::FunctionKind::Modifier) && node.name.is_some() {
            // for non-modifier *named* functions, fill-in parameter types in
            // parameters scope for overload disambiguation in p5
            let parameter_types: Vec<_> = node
                .parameters
                .iter()
                .map(|parameter| self.binder.node_typing(parameter.node_id).as_type_id())
                .collect();

            let Some(parameters_scope_id) = self
                .binder
                .get_parameters_scope_for_definition(node.node_id)
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

    fn leave_function_type(&mut self, node: &input_ir::FunctionType) {
        self.visit_parameters(&node.parameters, None);
        if let Some(returns) = &node.returns {
            self.visit_parameters(returns, None);
        }
    }

    fn leave_try_statement(&mut self, node: &input_ir::TryStatement) {
        if let Some(returns) = &node.returns {
            self.visit_parameters(returns, None);
        }
    }

    fn leave_catch_clause_error(&mut self, node: &input_ir::CatchClauseError) {
        self.visit_parameters(&node.parameters, None);
    }

    fn enter_type_name(&mut self, node: &input_ir::TypeName) -> bool {
        if let input_ir::TypeName::IdentifierPath(identifier_path) = node {
            self.resolve_identifier_path(identifier_path);
            false
        } else {
            true
        }
    }

    fn enter_override_paths(&mut self, items: &input_ir::OverridePaths) -> bool {
        for identifier_path in items {
            self.resolve_identifier_path(identifier_path);
        }
        false
    }

    fn leave_event_definition(&mut self, node: &input_ir::EventDefinition) {
        self.binder.mark_user_meta_type_node(node.node_id);

        // Resolve and collect the types of the parameters, saving them in the
        // scope to use in overload disambiguation when invoking an event as a
        // function
        let mut parameter_types = Vec::new();
        for parameter in &node.parameters {
            // TODO: the data location is not strictly correct, but strings, bytes
            // and structs are allowed as event parameters and they won't type if we
            // pass None here
            let type_id = self.resolve_type_name(&parameter.type_name, Some(DataLocation::Memory));
            self.binder.set_node_type(parameter.node_id, type_id);
            parameter_types.push(type_id);
        }

        let Some(parameters_scope_id) = self
            .binder
            .get_parameters_scope_for_definition(node.node_id)
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

    fn leave_error_definition(&mut self, node: &input_ir::ErrorDefinition) {
        self.binder.mark_user_meta_type_node(node.node_id);

        // Resolve the types of the parameters
        for parameter in &node.parameters {
            // TODO: the data location is not strictly correct, but strings, bytes
            // and structs are allowed as error parameters and they won't type if we
            // pass None here
            let type_id = self.resolve_type_name(&parameter.type_name, Some(DataLocation::Memory));
            self.binder.set_node_type(parameter.node_id, type_id);
        }
    }

    fn leave_state_variable_definition(&mut self, node: &input_ir::StateVariableDefinition) {
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Storage));
        self.binder.set_node_type(node.node_id, type_id);
        // if required, we will compute the type of the getter after all
        // definitions have been typed
    }

    fn leave_constant_definition(&mut self, node: &input_ir::ConstantDefinition) {
        let type_id = self.resolve_type_name(&node.type_name, None);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_variable_declaration_statement(
        &mut self,
        node: &input_ir::VariableDeclarationStatement,
    ) {
        let type_id = if let Some(type_name) = &node.type_name {
            self.resolve_type_name(
                type_name,
                node.storage_location.as_ref().map(Into::into).or_else(|| {
                    if self.language_version < VERSION_0_5_0 {
                        // default data location is storage for variables
                        Some(DataLocation::Storage)
                    } else {
                        None
                    }
                }),
            )
        } else {
            // this is for `var` variables (in Solidity < 0.5.0) we cannot
            // resolve the type at this point (will fixup in p5)
            None
        };
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_struct_definition(&mut self, node: &input_ir::StructDefinition) {
        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn leave_struct_member(&mut self, node: &input_ir::StructMember) {
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Inherited));
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_enum_definition(&mut self, node: &input_ir::EnumDefinition) {
        let type_id = self.types.register_type(Type::Enum {
            definition_id: node.node_id,
        });
        for member in &node.members {
            self.binder.set_node_type(member.id(), Some(type_id));
        }

        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn leave_user_defined_value_type_definition(
        &mut self,
        node: &input_ir::UserDefinedValueTypeDefinition,
    ) {
        self.binder.mark_user_meta_type_node(node.node_id);

        let target_type_id = self.type_of_elementary_type(&node.value_type, None);
        let Definition::UserDefinedValueType(udvt) = self.binder.get_definition_mut(node.node_id)
        else {
            unreachable!("definition in UDVT node is not a UDVT");
        };
        udvt.target_type_id = target_type_id;
    }

    fn enter_using_directive(&mut self, node: &input_ir::UsingDirective) -> bool {
        match &node.clause {
            input_ir::UsingClause::IdentifierPath(identifier_path) => {
                self.resolve_identifier_path(identifier_path);
            }
            input_ir::UsingClause::UsingDeconstruction(using_deconstruction) => {
                for symbol in &using_deconstruction.symbols {
                    self.resolve_identifier_path(&symbol.name);
                }
            }
        }
        // target's TypeName is resolved when entering it
        true
    }

    fn leave_using_directive(&mut self, node: &input_ir::UsingDirective) {
        let directive = match &node.target {
            input_ir::UsingTarget::TypeName(type_name) => {
                let Some(type_id) =
                    // TODO: not sure we should be using DataLocation::Inherited
                    // here, but if we don't provide one we can't register
                    // reference types and hence we cannot get a type_id
                    self.resolve_type_name(type_name, Some(DataLocation::Inherited))
                else {
                    return;
                };
                match &node.clause {
                    input_ir::UsingClause::IdentifierPath(identifier_path) => {
                        let Some(scope_id) =
                            self.find_library_scope_id_for_identifier_path(identifier_path)
                        else {
                            return;
                        };
                        UsingDirective::new_single_type(scope_id, type_id)
                    }
                    input_ir::UsingClause::UsingDeconstruction(using_deconstruction) => {
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

                            symbols.insert(symbol_name.unparse(), definition_ids);

                            if let Some(operator) = &symbol.alias {
                                let terminal = match operator {
                                    input_ir::UsingOperator::Ampersand => TerminalKind::Ampersand,
                                    input_ir::UsingOperator::Asterisk => TerminalKind::Asterisk,
                                    input_ir::UsingOperator::BangEqual => TerminalKind::BangEqual,
                                    input_ir::UsingOperator::Bar => TerminalKind::Bar,
                                    input_ir::UsingOperator::Caret => TerminalKind::Caret,
                                    input_ir::UsingOperator::EqualEqual => TerminalKind::EqualEqual,
                                    input_ir::UsingOperator::GreaterThan => {
                                        TerminalKind::GreaterThan
                                    }
                                    input_ir::UsingOperator::GreaterThanEqual => {
                                        TerminalKind::GreaterThanEqual
                                    }
                                    input_ir::UsingOperator::LessThan => TerminalKind::LessThan,
                                    input_ir::UsingOperator::LessThanEqual => {
                                        TerminalKind::LessThanEqual
                                    }
                                    input_ir::UsingOperator::Minus => TerminalKind::Minus,
                                    input_ir::UsingOperator::Percent => TerminalKind::Percent,
                                    input_ir::UsingOperator::Plus => TerminalKind::Plus,
                                    input_ir::UsingOperator::Slash => TerminalKind::Slash,
                                    input_ir::UsingOperator::Tilde => TerminalKind::Tilde,
                                };
                                operators.insert(terminal, symbol_name.unparse());
                            }
                        }

                        let scope = Scope::new_using(node.node_id, symbols);
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
            input_ir::UsingTarget::Asterisk => {
                let input_ir::UsingClause::IdentifierPath(identifier_path) = &node.clause else {
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

    fn enter_revert_statement(&mut self, node: &input_ir::RevertStatement) -> bool {
        if let Some(identifier_path) = &node.error {
            self.resolve_identifier_path(identifier_path);
        }
        true
    }

    fn leave_revert_statement(&mut self, node: &input_ir::RevertStatement) {
        if let Some(identifier_path) = &node.error {
            let type_id = self.type_of_identifier_path(identifier_path, None);
            self.binder.set_node_type(node.node_id, type_id);
        }
    }

    fn enter_emit_statement(&mut self, node: &input_ir::EmitStatement) -> bool {
        self.resolve_identifier_path(&node.event);
        true
    }

    fn leave_emit_statement(&mut self, node: &input_ir::EmitStatement) {
        let type_id = self.type_of_identifier_path(&node.event, None);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn enter_catch_clause_error(&mut self, node: &input_ir::CatchClauseError) -> bool {
        if let Some(name) = &node.name {
            let resolution = match name.unparse().as_str() {
                "Error" | "Panic" => Resolution::BuiltIn(BuiltIn::ErrorOrPanic),
                _ => Resolution::Unresolved,
            };
            let reference = Reference::new(Rc::clone(name), resolution);
            self.binder.insert_reference(reference);
        }
        true
    }

    fn leave_new_expression(&mut self, node: &input_ir::NewExpression) {
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Memory));
        let typing = type_id.map_or(Typing::Unresolved, Typing::NewExpression);
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_type_expression(&mut self, node: &input_ir::TypeExpression) {
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Memory));
        let typing = type_id.map_or(Typing::Unresolved, |type_id| {
            Typing::BuiltIn(BuiltIn::Type(type_id))
        });
        self.binder.set_node_typing(node.node_id, typing);
    }
}
