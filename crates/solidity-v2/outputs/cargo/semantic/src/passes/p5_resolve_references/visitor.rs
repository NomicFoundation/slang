use std::sync::Arc;

use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::visitor::Visitor;
use slang_solidity_v2_ir::ir::NodeIdentity;

use super::Pass;
use crate::binder::{Reference, Resolution, Typing};
use crate::built_ins::InternalBuiltIn;
use crate::passes::common::filter_overriden_definitions;
use crate::types::{
    AddressType, ArrayType, DataLocation, FixedSizeArrayType, MappingType, MetaType, Number,
    TupleType, Type, UserMetaType,
};

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
        for member in node.members.iter() {
            ir::visitor::accept_contract_member(member, self);
        }
        self.leave_scope_for_node_id(node.id());

        // But any reference in the inheritance types and the storage layout
        // specifier should resolve in the parent scope
        for inheritance_type in node.inheritance_types.iter() {
            ir::visitor::accept_inheritance_type(inheritance_type, self);
        }
        if let Some(ref storage_layout) = node.storage_layout {
            // TODO(validation) SDR[56]: check that the expression is not binding constant variables up until 0.8.31
            // https://www.soliditylang.org/blog/2025/12/03/solidity-0.8.31-release-announcement

            // TODO: call `resolve_storage_base_slot` first for now, because the
            // constant evaluator handles type casts.
            self.resolve_storage_base_slot(node, storage_layout);
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
        // TODO(validation) SDR[51]: for modifier kind, they are not allowed inside
        // interfaces since 0.8.8

        for modifier_invocation in node.attributes.modifier_invocations.iter() {
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

    fn enter_expression(&mut self, node: &ir::Expression) -> bool {
        match node {
            ir::Expression::Identifier(identifier) => {
                let symbol = identifier.unparse();
                let resolution = if symbol == "_" && self.is_in_modifier_scope() {
                    Resolution::BuiltIn(InternalBuiltIn::ModifierUnderscore)
                } else {
                    let scope_id = self.current_scope_id();
                    let resolution = self.resolve_symbol_in_scope(scope_id, symbol);
                    filter_overriden_definitions(self.binder, self.types, resolution)
                };

                // Set the typing for the `Identifier` node.
                // The resolution may point to an imported symbol, so we need to
                // follow through in order to get to the actual typing.
                let followed_resolution = self.binder.follow_symbol_aliases(resolution.clone());
                let typing = self.typing_of_resolution(&followed_resolution);
                self.binder.set_node_typing(identifier.id(), typing);

                // Finally, create the reference for the identifier.
                let reference = Reference::new(Arc::clone(identifier), resolution);
                self.binder.insert_reference(reference);
            }
            // An elementary type keyword in expression position denotes a
            // type, so it types as the meta-type of that type (eg. `uint`
            // types as `type(uint256)`). This is handled here rather than in
            // `leave_elementary_type` because elementary types in type
            // positions (which vastly outnumber expression uses) don't need a
            // typing registered.
            ir::Expression::ElementaryType(elementary_type) => {
                let typing = self.meta_typing_of(Self::type_of_elementary_type(elementary_type));
                let node_id = elementary_type
                    .node_id()
                    .expect("ElementaryType should have a node id");
                self.binder.set_node_typing(node_id, typing);
            }
            // A standalone `payable` (the operand of a `payable(x)` cast)
            // denotes the `address payable` type, so it types as its
            // meta-type.
            ir::Expression::PayableKeyword(payable_keyword) => {
                let typing = self.meta_typing_of(Type::Address(AddressType { is_payable: true }));
                self.binder.set_node_typing(payable_keyword.id(), typing);
            }
            _ => {}
        }
        true
    }

    fn leave_hex_number_expression(&mut self, node: &ir::HexNumberExpression) {
        let type_id = Self::hex_number_literal_kind(node)
            .map(|kind| self.types.register_type(Type::Literal(kind)));
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_decimal_number_expression(&mut self, node: &ir::DecimalNumberExpression) {
        let type_id = Number::from_decimal_number_expression(node).map(|number| {
            self.types
                .register_type(Type::Literal(number.to_literal_kind()))
        });
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_string_expression(&mut self, node: &ir::StringExpression) {
        let type_id = self
            .types
            .register_type(Self::type_of_string_expression(node));
        let node_id = node
            .node_id()
            .expect("StringExpression should have a NodeId");
        self.binder.set_node_type(node_id, Some(type_id));
    }

    fn leave_assignment_expression(&mut self, node: &ir::AssignmentExpression) {
        let type_id = self.typing_of_expression(&node.left_operand).as_type_id();
        // TODO(validation) SDR[59]: check that the type of right_operand can be applied
        // to the left by means of the operator
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_conditional_expression(&mut self, node: &ir::ConditionalExpression) {
        let true_type_id = self
            .typing_of_expression(&node.true_expression)
            .as_type_id();
        let false_type_id = self
            .typing_of_expression(&node.false_expression)
            .as_type_id();

        // TODO(validation) SDR[47]: both true_expression and false_expression should
        // have the compatible types
        //
        // The ternary takes the mobile type of both branches before computing
        // their common type.
        let type_id = match (true_type_id, false_type_id) {
            (Some(true_type_id), Some(false_type_id)) => {
                let true_mobile = self.types.compute_mobile_type(true_type_id);
                let false_mobile = self.types.compute_mobile_type(false_type_id);
                match (true_mobile, false_mobile) {
                    (Some(true_mobile), Some(false_mobile)) => {
                        self.types.common_type(true_mobile, false_mobile)
                    }
                    _ => None,
                }
            }
            _ => None,
        };
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_or_expression(&mut self, node: &ir::OrExpression) {
        // TODO(validation) SDR[57]: check that both operands are boolean
        self.binder
            .set_node_type(node.id(), Some(self.types.boolean()));
    }

    fn leave_and_expression(&mut self, node: &ir::AndExpression) {
        // TODO(validation) SDR[57]: check that both operands are boolean
        self.binder
            .set_node_type(node.id(), Some(self.types.boolean()));
    }

    fn leave_equality_expression(&mut self, node: &ir::EqualityExpression) {
        // TODO(validation) SDR[55]: check that both operands have a compatible type
        self.binder
            .set_node_type(node.id(), Some(self.types.boolean()));
    }

    fn leave_inequality_expression(&mut self, node: &ir::InequalityExpression) {
        // TODO(validation) SDR[55]: check that both operands have a compatible type
        self.binder
            .set_node_type(node.id(), Some(self.types.boolean()));
    }

    fn leave_bitwise_or_expression(&mut self, node: &ir::BitwiseOrExpression) {
        let type_id = self.type_of_binary_operator_expression(
            &node.left_operand,
            &node.right_operand,
            |l, r| l.bit_or(r),
        );
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_bitwise_xor_expression(&mut self, node: &ir::BitwiseXorExpression) {
        let type_id = self.type_of_binary_operator_expression(
            &node.left_operand,
            &node.right_operand,
            |l, r| l.bit_xor(r),
        );
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_bitwise_and_expression(&mut self, node: &ir::BitwiseAndExpression) {
        let type_id = self.type_of_binary_operator_expression(
            &node.left_operand,
            &node.right_operand,
            |l, r| l.bit_and(r),
        );
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_shift_expression(&mut self, node: &ir::ShiftExpression) {
        // TODO(validation) SDR[54]: check that the left operand is an integer and the
        // right operand is an _unsigned_ integer
        let type_id = self.type_of_left_typed_binary_operator_expression(
            &node.left_operand,
            &node.right_operand,
            |l, r| match &node.operator {
                ir::ShiftExpressionOperator::LessThanLessThan(_) => l.shl(r),
                ir::ShiftExpressionOperator::GreaterThanGreaterThan(_) => l.shr(r),
                ir::ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(_) => None,
            },
        );
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_additive_expression(&mut self, node: &ir::AdditiveExpression) {
        let type_id = self.type_of_binary_operator_expression(
            &node.left_operand,
            &node.right_operand,
            |l, r| match &node.operator {
                ir::AdditiveExpressionOperator::Plus(_) => Some(l.add(r)),
                ir::AdditiveExpressionOperator::Minus(_) => Some(l.sub(r)),
            },
        );
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_multiplicative_expression(&mut self, node: &ir::MultiplicativeExpression) {
        let type_id = self.type_of_binary_operator_expression(
            &node.left_operand,
            &node.right_operand,
            |l, r| match &node.operator {
                ir::MultiplicativeExpressionOperator::Asterisk(_) => Some(l.mul(r)),
                ir::MultiplicativeExpressionOperator::Slash(_) => l.div(r),
                ir::MultiplicativeExpressionOperator::Percent(_) => l.rem(r),
            },
        );
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_exponentiation_expression(&mut self, node: &ir::ExponentiationExpression) {
        let type_id = self.type_of_left_typed_binary_operator_expression(
            &node.left_operand,
            &node.right_operand,
            |l, r| l.pow(r),
        );
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_postfix_expression(&mut self, node: &ir::PostfixExpression) {
        // TODO(validation) SDR[52]: check that the operand is an integer
        let type_id = self.typing_of_expression(&node.operand).as_type_id();
        self.binder.set_node_type(node.id(), type_id);
    }

    fn leave_prefix_expression(&mut self, node: &ir::PrefixExpression) {
        let type_id = self.type_of_prefix_expression(node);
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
            for item in node.items.iter() {
                let type_id = item
                    .expression
                    .as_ref()
                    .and_then(|expression| self.typing_of_expression(expression).as_type_id())
                    .unwrap_or(self.types.void());
                types.push(type_id);
            }
            let type_id = self.types.register_type(Type::Tuple(TupleType { types }));
            Typing::Resolved(type_id)
        };
        self.binder.set_node_typing(node.id(), typing);
    }

    fn leave_member_access_expression(&mut self, node: &ir::MemberAccessExpression) {
        // we need to resolve the identifier at this point that we already have
        // typing information of the operand expression
        let operand_typing = self.typing_of_expression(&node.operand);
        let member_resolution =
            self.resolve_symbol_in_typing(&operand_typing, node.member.unparse());
        let resolution = filter_overriden_definitions(self.binder, self.types, member_resolution);

        // If the operand is either `this` or a contract/interface reference
        // type, then resolve the member typing as a contract member. This
        // handles public getters and visibility changes for public methods.
        let typing = if self.typing_is_contract_reference(&operand_typing) {
            self.typing_of_resolution_as_contract_member(&resolution)
        } else {
            self.typing_of_resolution(&resolution)
        };

        // Special cases: see `adjust_member_access_type_for_operand`
        let typing = match typing {
            Typing::Resolved(type_id) => Typing::Resolved(
                self.adjust_member_access_type_for_operand(type_id, &operand_typing),
            ),
            Typing::This(type_id) => {
                Typing::This(self.adjust_member_access_type_for_operand(type_id, &operand_typing))
            }
            Typing::Undetermined(type_ids) => Typing::Undetermined(
                type_ids
                    .into_iter()
                    .map(|type_id| {
                        self.adjust_member_access_type_for_operand(type_id, &operand_typing)
                    })
                    .collect(),
            ),
            Typing::Unresolved | Typing::BuiltIn(_) | Typing::NewExpression(_) | Typing::Super => {
                typing
            }
        };

        // Store the typing
        self.binder.set_node_typing(node.id(), typing);

        // And create the reference for the member identifier.
        let reference = Reference::new(Arc::clone(&node.member), resolution);
        self.binder.insert_reference(reference);
    }

    fn leave_index_access_expression(&mut self, node: &ir::IndexAccessExpression) {
        let typing = match self.typing_of_expression(&node.operand) {
            Typing::Resolved(operand_type_id) => {
                let range_access = node.end.is_some();
                match self.types.get_type_by_id(operand_type_id) {
                    Type::Array(ArrayType { element_type, .. })
                    | Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
                        // TODO(validation) SDR[58]: for fixed-size arrays, if the range
                        // is resolvable at compile time we should check for out
                        // of bounds accesses.
                        // TODO(validation) SDR[46]: array slices should only be
                        // implemented for calldata arrays (as of 0.8.33).
                        // TODO(validation) SDR[50]: we should return a new
                        // (intermediate) type for array slices.
                        if range_access {
                            Typing::Resolved(operand_type_id)
                        } else {
                            Typing::Resolved(*element_type)
                        }
                    }
                    Type::ByteArray(_) => {
                        if range_access {
                            Typing::Unresolved
                        } else {
                            Typing::Resolved(self.types.bytes1())
                        }
                    }
                    Type::Bytes(_) => {
                        if range_access {
                            Typing::Resolved(operand_type_id)
                        } else {
                            Typing::Resolved(self.types.bytes1())
                        }
                    }
                    Type::Mapping(MappingType { value_type_id, .. }) => {
                        if range_access {
                            Typing::Unresolved
                        } else {
                            Typing::Resolved(*value_type_id)
                        }
                    }
                    // Indexing a meta-type creates the meta-type of an array,
                    // eg. the `uint[]` in `abi.decode(data, (uint[]))` or the
                    // fixed-size `uint[3]`.
                    Type::MetaType(MetaType {
                        type_id: element_type,
                    }) => self.meta_typing_of(Type::Array(ArrayType {
                        element_type: *element_type,
                        location: DataLocation::Memory,
                    })),
                    // Indexing a user meta-type likewise creates the meta-type
                    // of an array (eg. `MyStruct[]`).
                    Type::UserMetaType(UserMetaType { definition_id }) => {
                        let definition_id = *definition_id;
                        if let Some(operand_type) = self.type_of_definition(definition_id) {
                            let element_type = self.types.register_type(operand_type);
                            self.meta_typing_of(Type::Array(ArrayType {
                                element_type,
                                location: DataLocation::Memory,
                            }))
                        } else {
                            Typing::Unresolved
                        }
                    }
                    _ => {
                        // TODO(validation) SDR[45]: the operand is not indexable
                        Typing::Unresolved
                    }
                }
            }
            _ => Typing::Unresolved,
        };
        self.binder.set_node_typing(node.id(), typing);
    }

    fn leave_array_expression(&mut self, node: &ir::ArrayExpression) {
        let typing = if node.items.is_empty() {
            // TODO(validation) SDR[1736]: an empty array literal cannot be typed
            Typing::Unresolved
        } else if let Some(type_id) = self.type_of_array_expression(node) {
            Typing::Resolved(type_id)
        } else {
            // TODO(validation) SDR[48]: all expressions in the array should have a type
            // and fit in the type dictated by the first element
            Typing::Unresolved
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
        for option in node.options.iter() {
            let identifier = &option.name;
            let resolution =
                crate::built_ins::BuiltInsResolver::lookup_call_option(identifier.unparse()).into();
            let reference = Reference::new(Arc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
        true
    }

    fn leave_call_options_expression(&mut self, node: &ir::CallOptionsExpression) {
        let operand_typing = self.typing_of_expression(&node.operand);

        // Pre-applying call options (eg. `foo{value: 3}`) partially applies the
        // function.
        let typing = match operand_typing.as_type_id() {
            Some(type_id) => match self.types.get_type_by_id(type_id) {
                Type::Function(function_type) => {
                    let function_type = function_type.clone();
                    Typing::Resolved(self.types.partially_apply_function_type(function_type))
                }
                _ => operand_typing,
            },
            None => operand_typing,
        };
        self.binder.set_node_typing(node.id(), typing);
    }

    fn visit_true_keyword(&mut self, node: &ir::TrueKeyword) {
        self.binder
            .set_node_typing(node.id(), Typing::Resolved(self.types.boolean()));
    }

    fn visit_false_keyword(&mut self, node: &ir::FalseKeyword) {
        self.binder
            .set_node_typing(node.id(), Typing::Resolved(self.types.boolean()));
    }

    fn visit_super_keyword(&mut self, node: &ir::SuperKeyword) {
        self.binder.set_node_typing(node.id(), Typing::Super);
    }

    fn visit_this_keyword(&mut self, node: &ir::ThisKeyword) {
        // `this` is a special keyword that resolves to the current contract or library type
        if let Some(scope_id) = self.current_contract_scope_id() {
            let scope = self.binder.get_scope_by_id(scope_id);
            let node_id = scope.node_id();
            let type_ = self
                .type_of_definition(node_id)
                .expect("the scope of `this` should be a contract or library definition");
            let type_id = self.types.register_type(type_);

            self.binder
                .set_node_typing(node.id(), Typing::This(type_id));
        } else {
            // TODO(validation) SDR[1473]: `this` cannot be used outside of a contract context
            // The error 1473 doesn't directly match to this, but because of how we resolve
            // `this` it flows through here.
        }
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
                        .follow_symbol_aliases(reference.resolution.clone())
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

    fn enter_yul_block(&mut self, _node: &ir::YulBlock) -> bool {
        // All Yul is collected and resolved in `p6_resolve_yul`; this pass does
        // not resolve Yul references, so skip the assembly body entirely.
        false
    }
}
