use slang_solidity_v2_ir::interner::StringId;
use slang_solidity_v2_ir::ir::{self, NodeId};

use super::Pass;
use crate::binder::{Definition, Resolution, Typing};
use crate::built_ins::BuiltIn;
use crate::types::{DataLocation, FunctionType, LiteralKind, Type, TypeId};

impl Pass<'_> {
    pub(super) fn typing_of_expression(&self, node: &ir::Expression) -> Typing {
        match node {
            ir::Expression::AssignmentExpression(assignment_expression) => {
                self.binder.node_typing(assignment_expression.id())
            }
            ir::Expression::ConditionalExpression(conditional_expression) => {
                self.binder.node_typing(conditional_expression.id())
            }
            ir::Expression::OrExpression(_)
            | ir::Expression::AndExpression(_)
            | ir::Expression::EqualityExpression(_)
            | ir::Expression::InequalityExpression(_)
            | ir::Expression::TrueKeyword
            | ir::Expression::FalseKeyword => Typing::Resolved(self.types.boolean()),
            ir::Expression::BitwiseOrExpression(bitwise_or_expression) => {
                self.binder.node_typing(bitwise_or_expression.id())
            }
            ir::Expression::BitwiseXorExpression(bitwise_xor_expression) => {
                self.binder.node_typing(bitwise_xor_expression.id())
            }
            ir::Expression::BitwiseAndExpression(bitwise_and_expression) => {
                self.binder.node_typing(bitwise_and_expression.id())
            }
            ir::Expression::ShiftExpression(shift_expression) => {
                self.binder.node_typing(shift_expression.id())
            }
            ir::Expression::AdditiveExpression(additive_expression) => {
                self.binder.node_typing(additive_expression.id())
            }
            ir::Expression::MultiplicativeExpression(multiplicative_expression) => {
                self.binder.node_typing(multiplicative_expression.id())
            }
            ir::Expression::ExponentiationExpression(exponentiation_expression) => {
                self.binder.node_typing(exponentiation_expression.id())
            }
            ir::Expression::PostfixExpression(postfix_expression) => {
                self.binder.node_typing(postfix_expression.id())
            }
            ir::Expression::PrefixExpression(prefix_expression) => {
                self.binder.node_typing(prefix_expression.id())
            }
            ir::Expression::FunctionCallExpression(function_call_expression) => {
                self.binder.node_typing(function_call_expression.id())
            }
            ir::Expression::CallOptionsExpression(call_options_expression) => {
                self.binder.node_typing(call_options_expression.id())
            }
            ir::Expression::MemberAccessExpression(member_access_expression) => {
                self.binder.node_typing(member_access_expression.id())
            }
            ir::Expression::IndexAccessExpression(index_access_expression) => {
                self.binder.node_typing(index_access_expression.id())
            }
            ir::Expression::NewExpression(new_expression) => {
                self.binder.node_typing(new_expression.id())
            }
            ir::Expression::TupleExpression(tuple_expression) => {
                self.binder.node_typing(tuple_expression.id())
            }
            ir::Expression::TypeExpression(type_expression) => {
                self.binder.node_typing(type_expression.id())
            }
            ir::Expression::ArrayExpression(array_expression) => {
                self.binder.node_typing(array_expression.id())
            }
            ir::Expression::HexNumberExpression(hex_number_expression) => {
                self.binder.node_typing(hex_number_expression.id())
            }
            ir::Expression::DecimalNumberExpression(decimal_number_expression) => {
                self.binder.node_typing(decimal_number_expression.id())
            }
            ir::Expression::StringExpression(string_expression) => self
                .binder
                .node_typing(Self::string_expression_node_id(string_expression)),
            ir::Expression::ElementaryType(elementary_type) => {
                Typing::MetaType(Self::type_of_elementary_type(elementary_type))
            }
            ir::Expression::Identifier(identifier) => self.typing_of_identifier(identifier),
            ir::Expression::PayableKeyword => Typing::MetaType(Type::Address { payable: true }),
            ir::Expression::ThisKeyword => Typing::This,
            ir::Expression::SuperKeyword => Typing::Super,
        }
    }

    fn type_of_elementary_type(elementary_type: &ir::ElementaryType) -> Type {
        match elementary_type {
            ir::ElementaryType::AddressType(address_type) => Type::Address {
                payable: address_type.payable_keyword,
            },
            ir::ElementaryType::BytesKeyword(terminal) => {
                Type::from_bytes_keyword(terminal.unparse(), Some(DataLocation::Memory)).unwrap()
            }
            ir::ElementaryType::IntKeyword(terminal) => Type::from_int_keyword(terminal.unparse()),
            ir::ElementaryType::UintKeyword(terminal) => {
                Type::from_uint_keyword(terminal.unparse())
            }
            ir::ElementaryType::FixedKeyword(terminal) => {
                Type::from_fixed_keyword(terminal.unparse())
            }
            ir::ElementaryType::UfixedKeyword(terminal) => {
                Type::from_ufixed_keyword(terminal.unparse())
            }
            ir::ElementaryType::BoolKeyword => Type::Boolean,
            ir::ElementaryType::StringKeyword => Type::String {
                location: DataLocation::Memory,
            },
        }
    }

    pub(super) fn type_of_definition(&mut self, definition_id: NodeId) -> Option<Type> {
        let definition = self.binder.find_definition_by_id(definition_id)?;
        match definition {
            Definition::Contract(_) => Some(Type::Contract { definition_id }),
            Definition::Enum(_) => Some(Type::Enum { definition_id }),
            Definition::Interface(_) => Some(Type::Interface { definition_id }),
            Definition::Struct(_) => Some(Type::Struct {
                definition_id,
                location: DataLocation::Memory,
            }),
            Definition::UserDefinedValueType(_) => Some(Type::UserDefinedValue { definition_id }),
            _ => None,
        }
    }

    fn typing_of_identifier(&self, identifier: &ir::Identifier) -> Typing {
        let resolution = &self
            .binder
            .find_reference_by_identifier_node_id(identifier.id())
            .unwrap()
            .resolution;
        // The resolution may point to an imported symbol, so we need to follow
        // through in order to get to the actual typing
        let resolution = self.binder.follow_symbol_aliases(resolution);
        self.typing_of_resolution(&resolution)
    }

    pub(super) fn type_of_integer_binary_expression(
        &self,
        left_operand: &ir::Expression,
        right_operand: &ir::Expression,
    ) -> Option<TypeId> {
        let left_type_id = self.typing_of_expression(left_operand).as_type_id();
        let right_type_id = self.typing_of_expression(right_operand).as_type_id();
        // TODO(validation): check that both operands are indeed integers
        if let (Some(left_type_id), Some(right_type_id)) = (left_type_id, right_type_id) {
            if self
                .types
                .implicitly_convertible_to(right_type_id, left_type_id)
            {
                Some(left_type_id)
            } else if self
                .types
                .implicitly_convertible_to(left_type_id, right_type_id)
            {
                Some(right_type_id)
            } else {
                // TODO(validation): the types are not compatible, we should
                // emit an error, or signal our caller
                None
            }
        } else {
            None
        }
    }

    pub(super) fn typing_of_resolution(&self, resolution: &Resolution) -> Typing {
        match resolution {
            Resolution::Unresolved => Typing::Unresolved,
            Resolution::BuiltIn(built_in) => self.built_ins_resolver().typing_of(built_in),
            Resolution::Definition(definition_id) => self.binder.node_typing(*definition_id),
            Resolution::Ambiguous(definitions) => {
                let mut type_ids = Vec::new();
                for definition_id in definitions {
                    if let Typing::Resolved(type_id) = self.binder.node_typing(*definition_id) {
                        type_ids.push(type_id);
                    }
                }
                Typing::Undetermined(type_ids)
            }
        }
    }

    pub(super) fn typing_is_contract_reference(&self, typing: &Typing) -> bool {
        match typing {
            Typing::This => true,
            Typing::Resolved(type_id) => matches!(
                self.types.get_type_by_id(*type_id),
                Type::Contract { .. } | Type::Interface { .. }
            ),
            _ => false,
        }
    }

    pub(super) fn typing_of_resolution_as_contract_member(
        &mut self,
        resolution: &Resolution,
    ) -> Typing {
        // Check if the target is a state variable, and if it has a getter
        if let Resolution::Definition(definition_id) = resolution {
            if let Definition::StateVariable(state_var_definition) =
                self.binder.find_definition_by_id(*definition_id).unwrap()
            {
                if let Some(getter_type_id) = state_var_definition.getter_type_id {
                    return Typing::Resolved(getter_type_id);
                }
            }
        }

        let mut typing = self.typing_of_resolution(resolution);
        let resolved_type = typing
            .as_type_id()
            .map(|type_id| self.types.get_type_by_id(type_id));

        if let Some(Type::Function(function_type)) = resolved_type {
            // If the resolved type is a function and the operand is either
            // `this` or something of an address type, the function is being
            // used as an external function. If the member is a *public*
            // function, change the expression typing to indicate the
            // external access.
            if function_type.is_externally_visible() {
                let external_function_type =
                    self.types.externalize_function_type(function_type.clone());
                let type_id_with_external_visibility = self
                    .types
                    .register_type(Type::Function(external_function_type));
                typing = Typing::Resolved(type_id_with_external_visibility);
            }
        }

        typing
    }

    fn type_id_of_receiver(&self, operand: &ir::Expression) -> Option<TypeId> {
        if let ir::Expression::MemberAccessExpression(member_access_expression) = operand {
            self.typing_of_expression(&member_access_expression.operand)
                .as_type_id()
        } else {
            None
        }
    }

    fn typing_of_cast(&mut self, argument_typing: &Typing, target_type: Type) -> Typing {
        // TODO(validation): this is a cast to the given type, but we
        // need to verify that the (single) argument is convertible
        match argument_typing {
            Typing::Resolved(argument_type_id) => {
                // the resulting cast type inherits the data location of the argument
                let argument_type = self.types.get_type_by_id(*argument_type_id);
                let type_id = if let Some(data_location) = argument_type.data_location() {
                    self.types
                        .register_type_with_data_location(target_type, data_location)
                } else {
                    self.types.register_type(target_type)
                };
                Typing::Resolved(type_id)
            }
            Typing::This => {
                // special case: "this" can be cast to an address
                if let Type::Address { .. } = target_type {
                    Typing::Resolved(self.types.address())
                } else {
                    Typing::Unresolved
                }
            }
            _ => Typing::Unresolved,
        }
    }

    pub(super) fn collect_positional_argument_typings(
        &self,
        arguments: &[ir::Expression],
    ) -> Vec<Typing> {
        arguments
            .iter()
            .map(|argument| self.typing_of_expression(argument))
            .collect::<Vec<_>>()
    }

    pub(super) fn typing_of_function_call_with_positional_arguments(
        &mut self,
        node: &ir::FunctionCallExpression,
        arguments: &[ir::Expression],
    ) -> Typing {
        let operand_typing = self.typing_of_expression(&node.operand);
        let argument_typings = self.collect_positional_argument_typings(arguments);

        match operand_typing {
            Typing::Unresolved | Typing::This | Typing::Super => {
                // `this` and `super` are not callable
                Typing::Unresolved
            }
            Typing::Resolved(type_id) => {
                if let Type::Function(FunctionType { return_type, .. }) =
                    self.types.get_type_by_id(type_id)
                {
                    Typing::Resolved(*return_type)
                } else {
                    // TODO(validation): the operand did not resolve to a function
                    Typing::Unresolved
                }
            }
            Typing::Undetermined(type_ids) => {
                let receiver_type_id = self.type_id_of_receiver(&node.operand);
                let candidate = self.lookup_function_matching_positional_arguments(
                    &type_ids,
                    &argument_typings,
                    receiver_type_id,
                );

                if let Some(candidate) = candidate {
                    let return_type = candidate.return_type;

                    let reference_node_id = reference_node_id_for_expression(&node.operand);
                    let definition_id = candidate.definition_id;
                    if let (Some(node_id), Some(definition_id)) = (reference_node_id, definition_id)
                    {
                        // TODO: maybe update the typing of the operand as well?
                        self.binder
                            .fixup_reference(node_id, Resolution::Definition(definition_id));
                    }

                    Typing::Resolved(return_type)
                } else {
                    Typing::Unresolved
                }
            }
            Typing::MetaType(type_) => {
                if argument_typings.len() == 1 {
                    self.typing_of_cast(&argument_typings[0], type_)
                } else {
                    Typing::Unresolved
                }
            }
            Typing::NewExpression(type_id) => {
                match self.types.get_type_by_id(type_id) {
                    Type::Array { .. } | Type::Contract { .. } => Typing::Resolved(type_id),
                    _ => {
                        // only contracts can be created with `new`
                        Typing::Unresolved
                    }
                }
            }
            Typing::UserMetaType(node_id) => {
                // Generally this is a cast to the underlying type of the given
                // definition, except for structs for which we need to construct
                // the value in memory
                match self.binder.find_definition_by_id(node_id) {
                    Some(Definition::Contract(_)) => {
                        // TODO(validation): the type of the first argument should be an address
                        let type_id = self.types.register_type(Type::Contract {
                            definition_id: node_id,
                        });
                        Typing::Resolved(type_id)
                    }
                    Some(Definition::Interface(_)) => {
                        // TODO(validation): the type of the first argument should be an address
                        let type_id = self.types.register_type(Type::Interface {
                            definition_id: node_id,
                        });
                        Typing::Resolved(type_id)
                    }
                    Some(Definition::Struct(_)) => {
                        // struct construction
                        let type_id = self.types.register_type(Type::Struct {
                            definition_id: node_id,
                            location: DataLocation::Memory,
                        });
                        Typing::Resolved(type_id)
                    }
                    _ => Typing::Unresolved,
                }
            }
            // Special case: for `abi.decode` we need to register the types
            // given as the second argument and we need a mutable `TypeRegistry`
            // for that.
            Typing::BuiltIn(BuiltIn::AbiDecode) => self.typing_of_abi_decode(&argument_typings),
            Typing::BuiltIn(built_in) => self
                .built_ins_resolver()
                .typing_of_function_call(&built_in, &argument_typings),
        }
    }

    fn typing_of_abi_decode(&mut self, argument_typings: &[Typing]) -> Typing {
        if argument_typings.len() != 2 {
            return Typing::Unresolved;
        }
        match &argument_typings[1] {
            Typing::Resolved(type_id) => {
                // TODO(validation): this only makes sense if type_id is a tuple
                Typing::Resolved(*type_id)
            }
            Typing::UserMetaType(definition_id) => {
                if let Some(type_) = self.type_of_definition(*definition_id) {
                    Typing::Resolved(self.types.register_type(type_))
                } else {
                    Typing::Unresolved
                }
            }
            Typing::MetaType(type_) => Typing::Resolved(self.types.register_type(type_.clone())),
            _ => Typing::Unresolved,
        }
    }

    pub(super) fn collect_named_argument_typings(
        &self,
        arguments: &[ir::NamedArgument],
    ) -> Vec<(StringId, Typing)> {
        arguments
            .iter()
            .map(|argument| {
                (
                    argument.name.string_id,
                    self.typing_of_expression(&argument.value),
                )
            })
            .collect::<Vec<_>>()
    }

    pub(super) fn typing_of_function_call_with_named_arguments(
        &mut self,
        node: &ir::FunctionCallExpression,
        arguments: &[ir::NamedArgument],
    ) -> Typing {
        let operand_typing = self.typing_of_expression(&node.operand);

        let (typing, definition_id) = match operand_typing {
            Typing::Unresolved | Typing::This | Typing::Super => {
                // `this` and `super` are not callable
                (Typing::Unresolved, None)
            }
            Typing::Resolved(type_id) => {
                if let Type::Function(FunctionType {
                    definition_id,
                    return_type,
                    ..
                }) = self.types.get_type_by_id(type_id)
                {
                    (Typing::Resolved(*return_type), *definition_id)
                } else {
                    // TODO(validation): the operand did not resolve to a function
                    (Typing::Unresolved, None)
                }
            }
            Typing::Undetermined(type_ids) => {
                let receiver_type_id = self.type_id_of_receiver(&node.operand);
                let argument_typings = self.collect_named_argument_typings(arguments);
                let candidate = self.lookup_function_matching_named_arguments(
                    &type_ids,
                    &argument_typings,
                    receiver_type_id,
                );

                if let Some(candidate) = candidate {
                    let return_type = candidate.return_type;

                    let reference_node_id = reference_node_id_for_expression(&node.operand);
                    let definition_id = candidate.definition_id;
                    if let (Some(node_id), Some(definition_id)) = (reference_node_id, definition_id)
                    {
                        // TODO: maybe update the typing of the operand as well?
                        self.binder
                            .fixup_reference(node_id, Resolution::Definition(definition_id));
                    }

                    (Typing::Resolved(return_type), definition_id)
                } else {
                    (Typing::Unresolved, None)
                }
            }
            Typing::MetaType(_) => {
                // This is a cast to the given type and is not valid with named arguments
                (Typing::Unresolved, None)
            }
            Typing::NewExpression(type_id) => {
                if let Type::Contract { definition_id } = self.types.get_type_by_id(type_id) {
                    (Typing::Resolved(type_id), Some(*definition_id))
                } else {
                    // only contracts can be created with `new`
                    (Typing::Unresolved, None)
                }
            }
            Typing::UserMetaType(node_id) => {
                // Function call with named arguments are only valid in user
                // types of the struct kind, which results in the construction
                // of such struct in memory
                match self.binder.find_definition_by_id(node_id) {
                    Some(Definition::Struct(_)) => {
                        // struct construction
                        let type_id = self.types.register_type(Type::Struct {
                            definition_id: node_id,
                            location: DataLocation::Memory,
                        });
                        (Typing::Resolved(type_id), Some(node_id))
                    }
                    Some(Definition::Event(_)) => {
                        // this is an event called as a function, which is valid in <0.5.0
                        (Typing::Resolved(self.types.void()), Some(node_id))
                    }
                    _ => (Typing::Unresolved, None),
                }
            }
            Typing::BuiltIn(_) => {
                // built-ins cannot be called with named arguments
                (Typing::Unresolved, None)
            }
        };

        // Reference and resolve named arguments
        self.resolve_named_arguments(arguments, definition_id);

        typing
    }
}

// Given an expression node that resolves to a reference, return the node ID of
// the identifier of the reference. If the expression cannot be traced back to a
// single reference, return `None`.
fn reference_node_id_for_expression(node: &ir::Expression) -> Option<NodeId> {
    match &node {
        ir::Expression::MemberAccessExpression(f) => Some(f.member.id()),
        ir::Expression::Identifier(f) => Some(f.id()),
        ir::Expression::CallOptionsExpression(f) => reference_node_id_for_expression(&f.operand),
        _ => None,
    }
}

/// Typing functions for literals
impl Pass<'_> {
    pub(super) fn string_expression_node_id(node: &ir::StringExpression) -> NodeId {
        match node {
            ir::StringExpression::StringLiterals(strings) => strings[0].id(),
            ir::StringExpression::HexStringLiterals(hex_strings) => hex_strings[0].id(),
            ir::StringExpression::UnicodeStringLiterals(unicode_strings) => unicode_strings[0].id(),
        }
    }

    pub(super) fn type_of_string_expression(node: &ir::StringExpression) -> Type {
        let kind = match node {
            ir::StringExpression::StringLiterals(strings) => {
                let size = strings.iter().fold(0usize, |acc, string| {
                    // TODO: consider escaped characters
                    acc + string.unparse().len() - 2
                });
                LiteralKind::String {
                    bytes: u32::try_from(size).unwrap(),
                }
            }
            ir::StringExpression::HexStringLiterals(hex_strings) => {
                let size = hex_strings.iter().fold(0usize, |acc, hex_string| {
                    // 5 is the length of the `hex` prefix plus the quotes
                    acc + (hex_string.unparse().len() - 5) / 2
                });
                LiteralKind::String {
                    bytes: u32::try_from(size).unwrap(),
                }
            }
            ir::StringExpression::UnicodeStringLiterals(unicode_strings) => {
                let size = unicode_strings.iter().fold(0usize, |acc, unicode_string| {
                    // TODO: actually parse the string
                    // 9 is the length of the `unicode` prefix plus quotes
                    acc + unicode_string.unparse().len() - 9
                });
                LiteralKind::String {
                    bytes: u32::try_from(size).unwrap(),
                }
            }
        };
        Type::Literal(kind)
    }

    pub(super) fn hex_number_literal_kind(
        hex_number_expression: &ir::HexNumberExpression,
    ) -> LiteralKind {
        let hex_number = hex_number_expression.literal.unparse();
        if hex_number.len() == 42 {
            // TODO(validation): verify the address is valid (ie. has a valid checksum)
            // We need at least an implementation of SHA3 to compute the checksum
            LiteralKind::Address
        } else if hex_number == "0x0" {
            LiteralKind::Zero
        } else {
            LiteralKind::HexInteger {
                bytes: u32::try_from((hex_number.len() - 3) / 2 + 1).unwrap(),
            }
        }
    }
}
