use std::rc::Rc;

use super::Pass;
use crate::backend::binder::{Reference, Resolution, Typing};
use crate::backend::ir::ir2_flat_contracts::visitor::Visitor;
use crate::backend::ir::ir2_flat_contracts::{self as input_ir};
use crate::backend::types::{DataLocation, LiteralKind, Type};
use crate::cst::TerminalKind;
use crate::utils::versions::{VERSION_0_5_0, VERSION_0_7_0};

impl Visitor for Pass {
    fn enter_source_unit(&mut self, node: &input_ir::SourceUnit) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_source_unit(&mut self, node: &input_ir::SourceUnit) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        // Push the contract scope to visit the contract members
        self.enter_scope_for_node_id(node.node_id);
        for member in &node.members {
            input_ir::visitor::accept_contract_member(member, self);
        }
        self.leave_scope_for_node_id(node.node_id);

        // But any reference in the inheritance types and the storage layout
        // specifier should resolve in the parent scope
        for inheritance_type in &node.inheritance_types {
            input_ir::visitor::accept_inheritance_type(inheritance_type, self);
        }
        if let Some(ref storage_layout) = node.storage_layout {
            input_ir::visitor::accept_expression(storage_layout, self);
        }

        // We already visited the contract definition's children above to be
        // able to better control the scope stack, so don't recurse again
        false
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_library_definition(&mut self, node: &input_ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_function_definition(&mut self, node: &input_ir::FunctionDefinition) -> bool {
        // TODO(validation): for modifier kind, they are not allowed inside
        // interfaces since 0.8.8

        for modifier_invocation in &node.modifier_invocations {
            self.resolve_modifier_invocation(modifier_invocation);
        }

        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_function_definition(&mut self, node: &input_ir::FunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_block(&mut self, node: &input_ir::Block) -> bool {
        if self.language_version >= VERSION_0_5_0 {
            self.enter_scope_for_node_id(node.node_id);
        }
        true
    }

    fn leave_block(&mut self, node: &input_ir::Block) {
        if self.language_version >= VERSION_0_5_0 {
            self.leave_scope_for_node_id(node.node_id);
        }
    }

    fn enter_for_statement(&mut self, node: &input_ir::ForStatement) -> bool {
        if self.language_version >= VERSION_0_5_0 {
            self.enter_scope_for_node_id(node.node_id);
        }
        true
    }

    fn leave_for_statement(&mut self, node: &input_ir::ForStatement) {
        if self.language_version >= VERSION_0_5_0 {
            self.leave_scope_for_node_id(node.node_id);
        }
    }

    fn enter_yul_block(&mut self, node: &input_ir::YulBlock) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_yul_block(&mut self, node: &input_ir::YulBlock) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_expression(&mut self, node: &input_ir::Expression) -> bool {
        if let input_ir::Expression::Identifier(identifier) = node {
            let scope_id = self.current_scope_id();
            let resolution = self.filter_overriden_definitions(
                self.resolve_symbol_in_scope(scope_id, &identifier.unparse()),
            );
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
        true
    }

    fn leave_hex_number_expression(&mut self, node: &input_ir::HexNumberExpression) {
        let kind = Self::hex_number_literal_kind(node);
        let type_id = self.types.register_type(Type::Literal(kind));
        self.binder.set_node_type(node.node_id, Some(type_id));
    }

    fn leave_decimal_number_expression(&mut self, node: &input_ir::DecimalNumberExpression) {
        let type_ = if node.unit.is_none() && node.literal.unparse() == "0" {
            Type::Literal(LiteralKind::Zero)
        } else {
            Type::Literal(LiteralKind::DecimalInteger)
        };
        let type_id = self.types.register_type(type_);
        self.binder.set_node_type(node.node_id, Some(type_id));
    }

    fn leave_string_expression(&mut self, node: &input_ir::StringExpression) {
        let type_id = self
            .types
            .register_type(Self::type_of_string_expression(node));
        let node_id = Self::string_expression_node_id(node);
        self.binder.set_node_type(node_id, Some(type_id));
    }

    fn leave_assignment_expression(&mut self, node: &input_ir::AssignmentExpression) {
        let type_id = self.typing_of_expression(&node.left_operand).as_type_id();
        // TODO(validation): check that the type of right_operand can be applied
        // to the left by means of the operator
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_conditional_expression(&mut self, node: &input_ir::ConditionalExpression) {
        let true_type_id = self
            .typing_of_expression(&node.true_expression)
            .as_type_id();
        let false_type_id = self
            .typing_of_expression(&node.false_expression)
            .as_type_id();

        // TODO(validation): both true_expression and false_expression should
        // have the compatible types
        let type_id = match (true_type_id, false_type_id) {
            (Some(true_type_id), Some(false_type_id)) => {
                if self
                    .types
                    .implicitly_convertible_to(false_type_id, true_type_id)
                {
                    Some(true_type_id)
                } else if self
                    .types
                    .implicitly_convertible_to(true_type_id, false_type_id)
                {
                    Some(false_type_id)
                } else {
                    None
                }
            }
            _ => None,
        };
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_or_expression(&mut self, _node: &input_ir::OrExpression) {
        // TODO(validation): check that both operands are boolean
    }

    fn leave_and_expression(&mut self, _node: &input_ir::AndExpression) {
        // TODO(validation): check that both operands are boolean
    }

    fn leave_equality_expression(&mut self, _node: &input_ir::EqualityExpression) {
        // TODO(validation): check that both operands have a compatible type
    }

    fn leave_inequality_expression(&mut self, _node: &input_ir::InequalityExpression) {
        // TODO(validation): check that both operands have a compatible type
    }

    fn leave_bitwise_or_expression(&mut self, node: &input_ir::BitwiseOrExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_bitwise_xor_expression(&mut self, node: &input_ir::BitwiseXorExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_bitwise_and_expression(&mut self, node: &input_ir::BitwiseAndExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_shift_expression(&mut self, node: &input_ir::ShiftExpression) {
        let type_id = self.typing_of_expression(&node.left_operand).as_type_id();
        // TODO(validation): check that the left operand is an integer and the
        // right operand is an _unsigned_ integer
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_additive_expression(&mut self, node: &input_ir::AdditiveExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_multiplicative_expression(&mut self, node: &input_ir::MultiplicativeExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_exponentiation_expression(&mut self, node: &input_ir::ExponentiationExpression) {
        let mut type_id = self.typing_of_expression(&node.left_operand).as_type_id();
        // TODO(validation): check that the left operand is an integer and the
        // right operand is an _unsigned_ integer
        if type_id.is_some_and(|type_id| self.types.get_type_by_id(type_id).is_literal_number()) {
            // if the base is a rational but the exponent is not, then the result is uint256
            if self
                .typing_of_expression(&node.right_operand)
                .as_type_id()
                .is_none_or(|exponent_type| {
                    !self.types.get_type_by_id(exponent_type).is_literal_number()
                })
            {
                type_id = Some(self.types.uint256());
            }
        }
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_postfix_expression(&mut self, node: &input_ir::PostfixExpression) {
        // TODO(validation): check that the operand is an integer
        let type_id = self.typing_of_expression(&node.operand).as_type_id();
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_prefix_expression(&mut self, node: &input_ir::PrefixExpression) {
        let type_id = match node.operator.kind {
            TerminalKind::PlusPlus
            | TerminalKind::Plus
            | TerminalKind::MinusMinus
            | TerminalKind::Minus
            | TerminalKind::Tilde => {
                // TODO(validation): check that the operand is integer
                self.typing_of_expression(&node.operand).as_type_id()
            }
            TerminalKind::Bang => {
                // TODO(validation): check that the operand is boolean
                Some(self.types.boolean())
            }
            TerminalKind::DeleteKeyword => Some(self.types.void()),
            _ => None,
        };
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_tuple_expression(&mut self, node: &input_ir::TupleExpression) {
        let typing = if node.items.len() == 1 {
            node.items
                .first()
                .unwrap()
                .expression
                .as_ref()
                .map_or(Typing::Unresolved, |expression| {
                    self.typing_of_expression(expression)
                })
        } else {
            let mut types = Vec::new();
            for item in &node.items {
                let type_id = item
                    .expression
                    .as_ref()
                    .and_then(|expression| self.typing_of_expression(expression).as_type_id())
                    .unwrap_or(self.types.void());
                types.push(type_id);
            }
            let type_id = self.types.register_type(Type::Tuple { types });
            Typing::Resolved(type_id)
        };
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_member_access_expression(&mut self, node: &input_ir::MemberAccessExpression) {
        // we need to resolve the identifier at this point that we already have
        // typing information of the operand expression
        let operand_typing = self.typing_of_expression(&node.operand);
        let resolution = self.filter_overriden_definitions(
            self.resolve_symbol_in_typing(&operand_typing, &node.member.unparse()),
        );

        // Special case: if the operand is either `this` or a contract/interface
        // reference type, then try to type the member as a getter
        let mut typing = if self.typing_is_contract_reference(&operand_typing) {
            self.typing_of_resolution_as_getter(&resolution)
        } else {
            self.typing_of_resolution(&resolution)
        };

        // Special case: If the type is a reference type with location
        // "inherited", we use the operand's location for the resulting typing
        if let Some(type_id) = typing.as_type_id() {
            let type_ = self.types.get_type_by_id(type_id);
            if type_.is_inherited_location() {
                if let Some(operand_location) = operand_typing
                    .as_type_id()
                    .and_then(|type_id| self.types.get_type_by_id(type_id).data_location())
                {
                    let type_id_with_location = self
                        .types
                        .register_type_with_data_location(type_.clone(), operand_location);
                    typing = Typing::Resolved(type_id_with_location);
                }
            }
        }

        // store the typing
        self.binder.set_node_typing(node.node_id, typing);

        let reference = Reference::new(Rc::clone(&node.member), resolution);
        self.binder.insert_reference(reference);
    }

    fn leave_index_access_expression(&mut self, node: &input_ir::IndexAccessExpression) {
        let typing = match self.typing_of_expression(&node.operand) {
            Typing::Resolved(operand_type_id) => {
                let range_access = node.end.is_some();
                let operand_type = self.types.get_type_by_id(operand_type_id);
                match operand_type {
                    Type::Array { element_type, .. } => {
                        if range_access {
                            Typing::Resolved(operand_type_id)
                        } else {
                            Typing::Resolved(*element_type)
                        }
                    }
                    Type::ByteArray { .. } => {
                        if range_access {
                            Typing::Unresolved
                        } else {
                            Typing::Resolved(self.types.bytes1())
                        }
                    }
                    Type::Bytes { .. } => {
                        if range_access {
                            Typing::Resolved(operand_type_id)
                        } else {
                            Typing::Resolved(self.types.bytes1())
                        }
                    }
                    Type::Mapping { value_type_id, .. } => {
                        if range_access {
                            Typing::Unresolved
                        } else {
                            Typing::Resolved(*value_type_id)
                        }
                    }
                    _ => {
                        // TODO(validation): the operand is not indexable
                        Typing::Unresolved
                    }
                }
            }
            Typing::MetaType(operand_type) => {
                // indexing a meta-type creates a new meta-type of the array
                let operand_type_id = self.types.register_type(operand_type);
                Typing::MetaType(Type::Array {
                    element_type: operand_type_id,
                    location: DataLocation::Memory,
                })
            }
            Typing::UserMetaType(definition_id) => {
                // indexing a user meta-type creates a new meta-type of the array
                if let Some(operand_type) = self.type_of_definition(definition_id) {
                    let operand_type_id = self.types.register_type(operand_type);
                    Typing::MetaType(Type::Array {
                        element_type: operand_type_id,
                        location: DataLocation::Memory,
                    })
                } else {
                    Typing::Unresolved
                }
            }
            _ => Typing::Unresolved,
        };
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_array_expression(&mut self, node: &input_ir::ArrayExpression) {
        let typing = if node.items.is_empty() {
            Typing::Unresolved
        } else {
            // TODO(validation): all expressions in the array should have the
            // same (or implicitly convertible) types
            if let Some(element_type) = self
                .typing_of_expression(node.items.first().unwrap())
                .as_type_id()
            {
                let element_type = self.types.reified_type(element_type);
                let type_id = self.types.register_type(Type::Array {
                    element_type,
                    location: DataLocation::Memory,
                });
                Typing::Resolved(type_id)
            } else {
                Typing::Unresolved
            }
        };
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_function_call_expression(&mut self, node: &input_ir::FunctionCallExpression) {
        let typing = match &node.arguments {
            input_ir::ArgumentsDeclaration::PositionalArguments(positional_arguments) => {
                self.typing_of_function_call_with_positional_arguments(node, positional_arguments)
            }
            input_ir::ArgumentsDeclaration::NamedArguments(named_arguments) => {
                self.typing_of_function_call_with_named_arguments(node, named_arguments)
            }
        };
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn enter_call_options_expression(&mut self, node: &input_ir::CallOptionsExpression) -> bool {
        for option in &node.options {
            let identifier = &option.name;
            let resolution = self
                .built_ins_resolver()
                .lookup_call_option(identifier.unparse().as_str())
                .into();
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
        true
    }

    fn leave_call_options_expression(&mut self, node: &input_ir::CallOptionsExpression) {
        let typing = self.typing_of_expression(&node.operand);
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn enter_yul_path(&mut self, items: &input_ir::YulPath) -> bool {
        if items.is_empty() {
            return false;
        }

        let scope_id = self.current_scope_id();
        let consumed_identifiers = if self.language_version >= VERSION_0_7_0 {
            let identifier = &items[0];
            let resolution = self.resolve_symbol_in_yul_scope(scope_id, &identifier.unparse());
            let reference = Reference::new(Rc::clone(identifier), resolution.clone());
            self.binder.insert_reference(reference);

            if items.len() > 1 {
                let suffix = &items[1];
                let resolution = self.resolve_yul_suffix(&suffix.unparse(), &resolution);
                let reference = Reference::new(Rc::clone(suffix), resolution);
                self.binder.insert_reference(reference);
            }

            2
        } else {
            let identifier = &items[0];
            let identifier_text = identifier.unparse();
            let resolution = self.resolve_symbol_in_yul_scope(scope_id, &identifier_text);
            let resolution = if resolution == Resolution::Unresolved {
                // if the identifier cannot be resolved as-is, try to extract a
                // suffix (ie. `_slot`, `_offset`, etc.) and resolve for it
                if let Some((prefix, suffix)) = identifier_text.rsplit_once('_') {
                    let resolution = self.resolve_symbol_in_yul_scope(scope_id, prefix);
                    self.resolve_yul_suffix(suffix, &resolution)
                } else {
                    resolution
                }
            } else {
                resolution
            };

            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);

            1
        };

        // any remaining identifiers cannot be resolved, but we still want to
        // emit a reference for each of them
        for identifier in items.iter().skip(consumed_identifiers) {
            self.binder.insert_reference(Reference::new(
                Rc::clone(identifier),
                Resolution::Unresolved,
            ));
        }

        false
    }

    fn enter_yul_stack_assignment_statement(
        &mut self,
        node: &input_ir::YulStackAssignmentStatement,
    ) -> bool {
        let identifier = &node.variable;
        let scope_id = self.current_scope_id();
        let resolution = self.resolve_symbol_in_yul_scope(scope_id, &identifier.unparse());
        let reference = Reference::new(Rc::clone(identifier), resolution);
        self.binder.insert_reference(reference);

        false
    }

    fn enter_yul_for_statement(&mut self, node: &input_ir::YulForStatement) -> bool {
        // Visit the initialization block first
        input_ir::visitor::accept_yul_block(&node.initialization, self);

        // Visit the rest of the children, but in the scope of the
        // initialization block. This is mainly so references in the condition
        // can resolve against the initialization block. The iterator and body
        // should be already properly linked from construction.
        self.enter_scope_for_node_id(node.initialization.node_id);
        input_ir::visitor::accept_yul_expression(&node.condition, self);
        input_ir::visitor::accept_yul_block(&node.iterator, self);
        input_ir::visitor::accept_yul_block(&node.body, self);
        self.leave_scope_for_node_id(node.initialization.node_id);

        // We already visited our children
        false
    }

    fn enter_tuple_deconstruction_statement(
        &mut self,
        node: &input_ir::TupleDeconstructionStatement,
    ) -> bool {
        if node.var_keyword {
            // this is a (deprecated) variable declaration, not assignment
            return true;
        }

        for element in &node.elements {
            let Some(member) = &element.member else {
                continue;
            };
            match member {
                input_ir::TupleMember::TypedTupleMember(_) => {
                    // this is a declaration, not a reference, so nothing left to do in this pass
                }
                input_ir::TupleMember::UntypedTupleMember(untyped_tuple_member) => {
                    let identifier = &untyped_tuple_member.name;
                    let scope_id = self.current_scope_id();
                    let resolution = self.filter_overriden_definitions(
                        self.resolve_symbol_in_scope(scope_id, &identifier.unparse()),
                    );
                    let reference = Reference::new(Rc::clone(identifier), resolution);
                    self.binder.insert_reference(reference);
                }
            }
        }

        true
    }

    fn leave_emit_statement(&mut self, node: &input_ir::EmitStatement) {
        let event_reference_id = node.event.last().unwrap().id();
        let event_resolution = self
            .binder
            .find_reference_by_identifier_node_id(event_reference_id)
            .map(|reference| &reference.resolution);
        match &node.arguments {
            input_ir::ArgumentsDeclaration::PositionalArguments(positional_arguments) => {
                // For positional arguments, resolve ambiguity of overloads only
                if let Some(Resolution::Ambiguous(definition_ids)) = event_resolution {
                    let argument_typings =
                        self.collect_positional_argument_typings(positional_arguments);
                    if let Some(candidate) = self.lookup_event_matching_positional_arguments(
                        definition_ids,
                        &argument_typings,
                    ) {
                        // update resolved definition
                        self.binder
                            .fixup_reference(event_reference_id, Resolution::Definition(candidate));
                    }
                }
            }
            input_ir::ArgumentsDeclaration::NamedArguments(named_arguments) => {
                // For named arguments, we need to resolve ambiguity and the named arguments
                let definition_id = match event_resolution {
                    Some(Resolution::Ambiguous(definition_ids)) => {
                        let argument_typings = self.collect_named_argument_typings(named_arguments);
                        let candidate = self.lookup_event_matching_named_arguments(
                            definition_ids,
                            &argument_typings,
                        );
                        if let Some(candidate) = candidate {
                            // update resolved definition
                            self.binder.fixup_reference(
                                event_reference_id,
                                Resolution::Definition(candidate),
                            );
                        }
                        candidate
                    }
                    Some(Resolution::Definition(definition_id)) => Some(*definition_id),
                    _ => None,
                };
                // resolve names in named arguments
                self.resolve_named_arguments(named_arguments, definition_id);
            }
        }
    }

    fn leave_revert_statement(&mut self, node: &input_ir::RevertStatement) {
        if let input_ir::ArgumentsDeclaration::NamedArguments(named_arguments) = &node.arguments {
            let definition_id = node
                .error
                .as_ref()
                .and_then(|error| {
                    self.binder
                        .find_reference_by_identifier_node_id(error.last().unwrap().id())
                })
                .and_then(|reference| reference.resolution.as_definition_id());
            self.resolve_named_arguments(named_arguments, definition_id);
        }
    }

    fn leave_variable_declaration_statement(
        &mut self,
        node: &input_ir::VariableDeclarationStatement,
    ) {
        if self.language_version >= VERSION_0_5_0 {
            // in Solidity >= 0.5.0 we need to update the scope so further
            // resolutions can access this variable definition
            self.replace_scope_for_node_id(node.node_id);
            // NOTE: ensure following code does not need to perform resolution
        }

        if matches!(
            node.variable_type,
            input_ir::VariableDeclarationType::VarKeyword
        ) {
            // update the type of the variable with the type of the expression (if available)
            if let Some(value) = &node.value {
                let typing = self
                    .typing_of_expression(value)
                    .as_type_id()
                    .map_or(Typing::Unresolved, |type_id| {
                        Typing::Resolved(self.types.reified_type(type_id))
                    });
                self.binder.fixup_node_typing(node.node_id, typing);
            }
        }
    }

    fn leave_tuple_deconstruction_statement(
        &mut self,
        node: &input_ir::TupleDeconstructionStatement,
    ) {
        if self.language_version >= VERSION_0_5_0 {
            // in Solidity >= 0.5.0 we need to update the scope so further
            // resolutions can access these variable definitions
            self.replace_scope_for_node_id(node.node_id);
            // NOTE: ensure following code does not need to perform resolution
        }

        if !node.var_keyword {
            return;
        }

        let typing = self.typing_of_expression(&node.expression);
        let Typing::Resolved(tuple_type_id) = typing else {
            // we can't fixup typing if the expression failed to type
            return;
        };
        let types = if let Type::Tuple { types } = self.types.get_type_by_id(tuple_type_id) {
            types.clone()
        } else {
            // the resolved type is not a tuple
            vec![tuple_type_id]
        };

        // fixup typing of `var` declarations
        for (element, element_type_id) in node.elements.iter().zip(types) {
            let Some(member) = &element.member else {
                continue;
            };
            if let input_ir::TupleMember::UntypedTupleMember(untyped_tuple_member) = member {
                let typing = Typing::Resolved(self.types.reified_type(element_type_id));
                self.binder
                    .fixup_node_typing(untyped_tuple_member.node_id, typing);
            }
        }
    }
}
