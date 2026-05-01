use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::binder::{Definition, Resolution, Typing};
use crate::built_ins::BuiltIn;
use crate::passes::common::node_id_for_expression_typing;
use crate::types::{literals, DataLocation, FunctionType, LiteralKind, Number, Type, TypeId};

impl Pass<'_> {
    pub(super) fn typing_of_expression(&self, node: &ir::Expression) -> Typing {
        match node {
            // These are always typed as boolean
            ir::Expression::OrExpression(_)
            | ir::Expression::AndExpression(_)
            | ir::Expression::EqualityExpression(_)
            | ir::Expression::InequalityExpression(_)
            | ir::Expression::TrueKeyword
            | ir::Expression::FalseKeyword => Typing::Resolved(self.types.boolean()),

            // Special case for elementary types: it's always a meta-type
            ir::Expression::ElementaryType(elementary_type) => {
                Typing::MetaType(Self::type_of_elementary_type(elementary_type))
            }

            // Other special cases
            ir::Expression::PayableKeyword => Typing::MetaType(Type::Address { payable: true }),
            ir::Expression::ThisKeyword => Typing::This,
            ir::Expression::SuperKeyword => Typing::Super,

            // By default, query the binder for registered typing information
            _ => {
                let node_id = node_id_for_expression_typing(node).expect(
                    "typing of expression variant not handled and it doesn't have a NodeId",
                );
                self.binder.node_typing(node_id)
            }
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

    /// Returns the type of an binary operator expression. If both operands are
    /// number literals, applies `op` to fold them into a narrowed literal type;
    /// otherwise falls back to the implicit-convertibility rule between the
    /// operand types.
    pub(super) fn type_of_binary_operator_expression<F>(
        &mut self,
        left_operand: &ir::Expression,
        right_operand: &ir::Expression,
        op: F,
    ) -> Option<TypeId>
    where
        F: FnOnce(&Number, &Number) -> Option<Number>,
    {
        let left_type_id = self.typing_of_expression(left_operand).as_type_id()?;
        let right_type_id = self.typing_of_expression(right_operand).as_type_id()?;

        // If both operands are number constants, fold them using the given operator.
        if let (Some(left_value), Some(right_value)) = (
            self.types.number_value_of_type_id(left_type_id),
            self.types.number_value_of_type_id(right_type_id),
        ) {
            return op(&left_value, &right_value).map(|result| {
                self.types
                    .register_type(Type::Literal(result.to_literal_kind()))
            });
        }

        // TODO(validation): check that both operands are valid for the operator
        // (needs additional parameter or check at the call site)
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
    }

    pub(super) fn type_of_prefix_expression(
        &mut self,
        node: &ir::PrefixExpression,
    ) -> Option<TypeId> {
        match node.expression_prefix_expression_operator {
            ir::Expression_PrefixExpression_Operator::Minus => {
                let operand_type_id = self.typing_of_expression(&node.operand).as_type_id()?;
                // Fold `-literal` (and more generally `-<constant>`) by
                // negating the operand's known number value.
                if let Some(value) = self.types.number_value_of_type_id(operand_type_id) {
                    Some(
                        self.types
                            .register_type(Type::Literal(value.negate().to_literal_kind())),
                    )
                } else {
                    // TODO(validation): check that the operand type supports
                    // negation (ie. uint does not)
                    Some(operand_type_id)
                }
            }
            ir::Expression_PrefixExpression_Operator::PlusPlus
            | ir::Expression_PrefixExpression_Operator::MinusMinus
            | ir::Expression_PrefixExpression_Operator::Tilde => {
                // TODO(validation): check that the operand is integer
                self.typing_of_expression(&node.operand).as_type_id()
            }
            ir::Expression_PrefixExpression_Operator::Bang => {
                // TODO(validation): check that the operand is boolean
                Some(self.types.boolean())
            }
            ir::Expression_PrefixExpression_Operator::DeleteKeyword => Some(self.types.void()),
        }
    }

    pub(super) fn type_of_array_expression(
        &mut self,
        array: &ir::ArrayExpression,
    ) -> Option<TypeId> {
        let mut item_type_ids: Vec<TypeId> = Vec::with_capacity(array.items.len());
        for item in &array.items {
            item_type_ids.push(self.typing_of_expression(item).as_type_id()?);
        }
        let element_type = self.types.common_mobile_type(&item_type_ids)?;
        Some(self.types.register_type(Type::FixedSizeArray {
            element_type,
            size: array.items.len(),
            location: DataLocation::Memory,
        }))
    }

    pub(super) fn type_of_left_typed_binary_operator_expression<F>(
        &mut self,
        left_operand: &ir::Expression,
        right_operand: &ir::Expression,
        op: F,
    ) -> Option<TypeId>
    where
        F: FnOnce(&Number, &Number) -> Option<Number>,
    {
        let left_type_id = self.typing_of_expression(left_operand).as_type_id()?;
        let right_type_id = self.typing_of_expression(right_operand).as_type_id()?;

        let left_value = self.types.number_value_of_type_id(left_type_id);
        let right_value = self.types.number_value_of_type_id(right_type_id);

        if let (Some(left_value), Some(right_value)) = (&left_value, &right_value) {
            // Both constants, so fold them
            op(left_value, right_value).map(|result| {
                self.types
                    .register_type(Type::Literal(result.to_literal_kind()))
            })
        } else if let Some(left_value) = &left_value {
            // For shifts or exponentiations, if the left operand is a literal,
            // the result is either a `uint256` or an `int256` depending on the
            // sign of `left_operand`.
            if left_value.is_negative() {
                Some(self.types.register_type(Type::Integer {
                    signed: true,
                    bits: 256,
                }))
            } else {
                Some(self.types.uint256())
            }
        } else {
            // TODO(validation): check that the operand types are valid (needs
            // additional parameter or validation at call site)
            Some(left_type_id)
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
    ) -> Vec<(String, Typing)> {
        arguments
            .iter()
            .map(|argument| {
                (
                    argument.name.unparse().to_string(),
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
    pub(super) fn type_of_string_expression(node: &ir::StringExpression) -> Type {
        // Hex string literals carry distinct provenance (mirroring `HexInteger`
        // vs `Integer`); regular and unicode strings share `String` since they
        // are indistinguishable once decoded.
        let kind = match node {
            ir::StringExpression::StringLiterals(literals) => {
                let value = literals::value_of_string_literals(literals);
                LiteralKind::String { bytes: value.len() }
            }
            ir::StringExpression::HexStringLiterals(literals) => {
                let value = literals::value_of_hex_string_literals(literals);
                LiteralKind::HexString { bytes: value.len() }
            }
            ir::StringExpression::UnicodeStringLiterals(literals) => {
                let value = literals::value_of_unicode_string_literals(literals);
                LiteralKind::String { bytes: value.len() }
            }
        };
        Type::Literal(kind)
    }

    pub(super) fn hex_number_literal_kind(
        hex_number_expression: &ir::HexNumberExpression,
    ) -> Option<LiteralKind> {
        let mut hex_number = hex_number_expression.literal.unparse().to_owned();
        hex_number.retain(|character| character != '_');
        // Source-text byte width: `0x` prefix is stripped
        let digits = u32::try_from(hex_number.len().saturating_sub(2)).ok()?;
        if digits == 40 {
            // TODO(validation): verify the address is valid (ie. has a valid checksum)
            // We need at least an implementation of SHA3 to compute the checksum
            return Some(LiteralKind::Address);
        }
        let value = Number::from_hex_number_expression(hex_number_expression)?
            .into_integer()
            .expect("hex literal must parse to an integer");
        // Each pair of hex digits is one byte (with odd digit counts rounded up).
        let bytes = digits.div_ceil(2).max(1);
        Some(LiteralKind::HexInteger { value, bytes })
    }
}
