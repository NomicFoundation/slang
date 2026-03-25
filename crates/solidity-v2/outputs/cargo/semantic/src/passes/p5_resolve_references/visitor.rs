use std::rc::Rc;

use super::Pass;
use crate::binder::{Reference, Resolution, Typing};
use crate::ir;
use crate::ir::visitor::Visitor;
use crate::ir::Expression_PrefixExpression_Operator;
use crate::types::{DataLocation, LiteralKind, Type};

impl Visitor for Pass<'_> {
    fn enter_source_unit(&mut self, node: &ir::SourceUnit) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_source_unit(&mut self, node: &ir::SourceUnit) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_contract_definition(&mut self, node: &ir::ContractDefinition) -> bool {
        // Push the contract scope to visit the contract members
        self.enter_scope_for_node_id(node.id());
        for member in &node.members {
            ir::visitor::accept_contract_member(member, self);
        }
        self.leave_scope_for_node_id(node.id());

        // But any reference in the inheritance types and the storage layout
        // specifier should resolve in the parent scope
        for inheritance_type in &node.inheritance_types {
            ir::visitor::accept_inheritance_type(inheritance_type, self);
        }
        if let Some(ref storage_layout) = node.storage_layout {
            ir::visitor::accept_expression(storage_layout, self);
        }

        // We already visited the contract definition's children above to be
        // able to better control the scope stack, so don't recurse again
        false
    }

    fn enter_interface_definition(&mut self, node: &ir::InterfaceDefinition) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_interface_definition(&mut self, node: &ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_library_definition(&mut self, node: &ir::LibraryDefinition) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_library_definition(&mut self, node: &ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_function_definition(&mut self, node: &ir::FunctionDefinition) -> bool {
        for modifier_invocation in &node.modifier_invocations {
            self.resolve_modifier_invocation(modifier_invocation);
        }

        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_function_definition(&mut self, node: &ir::FunctionDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_block(&mut self, node: &ir::Block) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_block(&mut self, node: &ir::Block) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_for_statement(&mut self, node: &ir::ForStatement) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_for_statement(&mut self, node: &ir::ForStatement) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_yul_block(&mut self, node: &ir::YulBlock) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_yul_block(&mut self, node: &ir::YulBlock) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_yul_function_definition(&mut self, node: &ir::YulFunctionDefinition) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_yul_function_definition(&mut self, node: &ir::YulFunctionDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_expression(&mut self, node: &ir::Expression) -> bool {
        if let ir::Expression::Identifier(identifier) = node {
            let scope_id = self.current_scope_id();
            let resolution = self.filter_overriden_definitions(
                self.resolve_symbol_in_scope(scope_id, identifier.unparse()),
            );
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
        true
    }

    fn leave_hex_number_expression(&mut self, node: &ir::HexNumberExpression) {
        let kind = Self::hex_number_literal_kind(node);
        let type_id = self.types.register_type(Type::Literal(kind));
        self.binder.set_node_type(node.id(), Some(type_id));
    }

    fn leave_decimal_number_expression(&mut self, node: &ir::DecimalNumberExpression) {
        let type_ = if node.unit.is_none() && node.literal.unparse() == "0" {
            Type::Literal(LiteralKind::Zero)
        } else {
            Type::Literal(LiteralKind::DecimalInteger)
        };
        let type_id = self.types.register_type(type_);
        self.binder.set_node_type(node.id(), Some(type_id));
    }

    fn leave_string_expression(&mut self, node: &ir::StringExpression) {
        let type_id = self
            .types
            .register_type(Self::type_of_string_expression(node));
        let node_id = Self::string_expression_node_id(node);
        self.binder.set_node_type(node_id, Some(type_id));
    }

    fn leave_assignment_expression(&mut self, node: &ir::AssignmentExpression) {
        let type_id = self.typing_of_expression(&node.left_operand).as_type_id();
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_conditional_expression(&mut self, node: &ir::ConditionalExpression) {
        let true_type_id = self
            .typing_of_expression(&node.true_expression)
            .as_type_id();
        let false_type_id = self
            .typing_of_expression(&node.false_expression)
            .as_type_id();

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
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_or_expression(&mut self, _node: &ir::OrExpression) {
        // TODO(validation): check that both operands are boolean
    }

    fn leave_and_expression(&mut self, _node: &ir::AndExpression) {
        // TODO(validation): check that both operands are boolean
    }

    fn leave_equality_expression(&mut self, _node: &ir::EqualityExpression) {
        // TODO(validation): check that both operands have a compatible type
    }

    fn leave_inequality_expression(&mut self, _node: &ir::InequalityExpression) {
        // TODO(validation): check that both operands have a compatible type
    }

    fn leave_bitwise_or_expression(&mut self, node: &ir::BitwiseOrExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_bitwise_xor_expression(&mut self, node: &ir::BitwiseXorExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_bitwise_and_expression(&mut self, node: &ir::BitwiseAndExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_shift_expression(&mut self, node: &ir::ShiftExpression) {
        let type_id = self.typing_of_expression(&node.left_operand).as_type_id();
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_additive_expression(&mut self, node: &ir::AdditiveExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_multiplicative_expression(&mut self, node: &ir::MultiplicativeExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_exponentiation_expression(&mut self, node: &ir::ExponentiationExpression) {
        let mut type_id = self.typing_of_expression(&node.left_operand).as_type_id();
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
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_postfix_expression(&mut self, node: &ir::PostfixExpression) {
        let type_id = self.typing_of_expression(&node.operand).as_type_id();
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_prefix_expression(&mut self, node: &ir::PrefixExpression) {
        let type_id = match node.expression_prefix_expression_operator {
            Expression_PrefixExpression_Operator::PlusPlus
            | Expression_PrefixExpression_Operator::MinusMinus
            | Expression_PrefixExpression_Operator::Minus
            | Expression_PrefixExpression_Operator::Tilde => {
                // TODO(validation): check that the operand is integer
                self.typing_of_expression(&node.operand).as_type_id()
            }
            Expression_PrefixExpression_Operator::Bang => {
                // TODO(validation): check that the operand is boolean
                Some(self.types.boolean())
            }
            Expression_PrefixExpression_Operator::DeleteKeyword => Some(self.types.void()),
        };
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_tuple_expression(&mut self, node: &ir::TupleExpression) {
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
        self.binder.set_node_typing(node.id(), typing);
    }

    fn leave_member_access_expression(&mut self, node: &ir::MemberAccessExpression) {
        // we need to resolve the identifier at this point that we already have
        // typing information of the operand expression
        let operand_typing = self.typing_of_expression(&node.operand);
        let resolution = self.filter_overriden_definitions(
            self.resolve_symbol_in_typing(&operand_typing, node.member.unparse()),
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
        self.binder.set_node_typing(node.id(), typing);

        let reference = Reference::new(Rc::clone(&node.member), resolution);
        self.binder.insert_reference(reference);
    }

    fn leave_index_access_expression(&mut self, node: &ir::IndexAccessExpression) {
        let typing = match self.typing_of_expression(&node.operand) {
            Typing::Resolved(operand_type_id) => {
                let range_access = node.end.is_some();
                let operand_type = self.types.get_type_by_id(operand_type_id);
                match operand_type {
                    Type::Array { element_type, .. }
                    | Type::FixedSizeArray { element_type, .. } => {
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
                    _ => Typing::Unresolved,
                }
            }
            Typing::MetaType(operand_type) => {
                let operand_type_id = self.types.register_type(operand_type);
                Typing::MetaType(Type::Array {
                    element_type: operand_type_id,
                    location: DataLocation::Memory,
                })
            }
            Typing::UserMetaType(definition_id) => {
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
        self.binder.set_node_typing(node.id(), typing);
    }

    fn leave_array_expression(&mut self, node: &ir::ArrayExpression) {
        let typing = if node.items.is_empty() {
            Typing::Unresolved
        } else {
            if let Some(element_type) = self
                .typing_of_expression(node.items.first().unwrap())
                .as_type_id()
            {
                let element_type = self.types.reified_type(element_type);
                let type_id = self.types.register_type(Type::FixedSizeArray {
                    element_type,
                    size: node.items.len(),
                    location: DataLocation::Memory,
                });
                Typing::Resolved(type_id)
            } else {
                Typing::Unresolved
            }
        };
        self.binder.set_node_typing(node.id(), typing);
    }

    fn leave_function_call_expression(&mut self, node: &ir::FunctionCallExpression) {
        let typing = match &node.arguments {
            ir::ArgumentsDeclaration::PositionalArguments(positional_arguments) => {
                self.typing_of_function_call_with_positional_arguments(node, positional_arguments)
            }
            ir::ArgumentsDeclaration::NamedArguments(named_arguments) => {
                self.typing_of_function_call_with_named_arguments(node, named_arguments)
            }
        };
        self.binder.set_node_typing(node.id(), typing);
    }

    fn enter_call_options_expression(&mut self, node: &ir::CallOptionsExpression) -> bool {
        for option in &node.options {
            let identifier = &option.name;
            let resolution =
                crate::built_ins::BuiltInsResolver::lookup_call_option(identifier.unparse()).into();
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
        true
    }

    fn leave_call_options_expression(&mut self, node: &ir::CallOptionsExpression) {
        let typing = self.typing_of_expression(&node.operand);
        self.binder.set_node_typing(node.id(), typing);
    }

    fn enter_yul_path(&mut self, items: &ir::YulPath) -> bool {
        if items.is_empty() {
            return false;
        }

        let scope_id = self.current_scope_id();
        let identifier = &items[0];
        let resolution = self.resolve_symbol_in_yul_scope(scope_id, identifier.unparse());
        let reference = Reference::new(Rc::clone(identifier), resolution.clone());
        self.binder.insert_reference(reference);

        if items.len() > 1 {
            let suffix = &items[1];
            let resolution = self.resolve_yul_suffix(suffix.unparse(), &resolution);
            let reference = Reference::new(Rc::clone(suffix), resolution);
            self.binder.insert_reference(reference);
        }

        let consumed_identifiers = 2;

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

    fn enter_yul_for_statement(&mut self, node: &ir::YulForStatement) -> bool {
        // Visit the initialization block first
        ir::visitor::accept_yul_block(&node.initialization, self);

        // Visit the rest of the children, but in the scope of the
        // initialization block. This is mainly so references in the condition
        // can resolve against the initialization block. The iterator and body
        // should be already properly linked from construction.
        self.enter_scope_for_node_id(node.initialization.id());
        ir::visitor::accept_yul_expression(&node.condition, self);
        ir::visitor::accept_yul_block(&node.iterator, self);
        ir::visitor::accept_yul_block(&node.body, self);
        self.leave_scope_for_node_id(node.initialization.id());

        // We already visited our children
        false
    }

    fn leave_emit_statement(&mut self, node: &ir::EmitStatement) {
        let event_reference_id = node.event.last().unwrap().id();
        let event_resolution = self
            .binder
            .find_reference_by_identifier_node_id(event_reference_id)
            .map(|reference| &reference.resolution);
        match &node.arguments {
            ir::ArgumentsDeclaration::PositionalArguments(positional_arguments) => {
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
            ir::ArgumentsDeclaration::NamedArguments(named_arguments) => {
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

    fn leave_revert_statement(&mut self, node: &ir::RevertStatement) {
        if let ir::ArgumentsDeclaration::NamedArguments(named_arguments) = &node.arguments {
            let definition_id = self
                .binder
                .find_reference_by_identifier_node_id(node.error.last().unwrap().id())
                .and_then(|reference| {
                    // Follow symbol aliases as the error type argument to
                    // revert could be an imported symbol
                    self.binder
                        .follow_symbol_aliases(&reference.resolution)
                        .as_definition_id()
                });
            self.resolve_named_arguments(named_arguments, definition_id);
        }
    }

    fn leave_variable_declaration_statement(&mut self, node: &ir::VariableDeclarationStatement) {
        // update the scope so further resolutions can access this variable definition
        self.replace_scope_for_node_id(node.id());
        // NOTE: ensure following code does not need to perform resolution
    }
}
