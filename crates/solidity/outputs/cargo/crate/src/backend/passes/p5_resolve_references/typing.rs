use std::rc::Rc;

use super::{Pass, VERSION_0_5_0, VERSION_0_8_0};
use crate::backend::binder::{Definition, Resolution, Typing};
use crate::backend::built_ins::BuiltIn;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::types::{DataLocation, FunctionType, LiteralKind, Type, TypeId};
use crate::cst::{NodeId, TerminalNode};

impl Pass {
    pub(super) fn typing_of_expression(&self, node: &input_ir::Expression) -> Typing {
        match node {
            input_ir::Expression::AssignmentExpression(assignment_expression) => {
                self.binder.node_typing(assignment_expression.node_id)
            }
            input_ir::Expression::ConditionalExpression(conditional_expression) => {
                self.binder.node_typing(conditional_expression.node_id)
            }
            input_ir::Expression::OrExpression(_)
            | input_ir::Expression::AndExpression(_)
            | input_ir::Expression::EqualityExpression(_)
            | input_ir::Expression::InequalityExpression(_)
            | input_ir::Expression::TrueKeyword
            | input_ir::Expression::FalseKeyword => Typing::Resolved(self.types.boolean()),
            input_ir::Expression::BitwiseOrExpression(bitwise_or_expression) => {
                self.binder.node_typing(bitwise_or_expression.node_id)
            }
            input_ir::Expression::BitwiseXorExpression(bitwise_xor_expression) => {
                self.binder.node_typing(bitwise_xor_expression.node_id)
            }
            input_ir::Expression::BitwiseAndExpression(bitwise_and_expression) => {
                self.binder.node_typing(bitwise_and_expression.node_id)
            }
            input_ir::Expression::ShiftExpression(shift_expression) => {
                self.binder.node_typing(shift_expression.node_id)
            }
            input_ir::Expression::AdditiveExpression(additive_expression) => {
                self.binder.node_typing(additive_expression.node_id)
            }
            input_ir::Expression::MultiplicativeExpression(multiplicative_expression) => {
                self.binder.node_typing(multiplicative_expression.node_id)
            }
            input_ir::Expression::ExponentiationExpression(exponentiation_expression) => {
                self.binder.node_typing(exponentiation_expression.node_id)
            }
            input_ir::Expression::PostfixExpression(postfix_expression) => {
                self.binder.node_typing(postfix_expression.node_id)
            }
            input_ir::Expression::PrefixExpression(prefix_expression) => {
                self.binder.node_typing(prefix_expression.node_id)
            }
            input_ir::Expression::FunctionCallExpression(function_call_expression) => {
                self.binder.node_typing(function_call_expression.node_id)
            }
            input_ir::Expression::CallOptionsExpression(call_options_expression) => {
                self.binder.node_typing(call_options_expression.node_id)
            }
            input_ir::Expression::MemberAccessExpression(member_access_expression) => {
                self.binder.node_typing(member_access_expression.node_id)
            }
            input_ir::Expression::IndexAccessExpression(index_access_expression) => {
                self.binder.node_typing(index_access_expression.node_id)
            }
            input_ir::Expression::NewExpression(new_expression) => {
                self.binder.node_typing(new_expression.node_id)
            }
            input_ir::Expression::TupleExpression(tuple_expression) => {
                self.binder.node_typing(tuple_expression.node_id)
            }
            input_ir::Expression::TypeExpression(type_expression) => {
                self.binder.node_typing(type_expression.node_id)
            }
            input_ir::Expression::ArrayExpression(array_expression) => {
                self.binder.node_typing(array_expression.node_id)
            }
            input_ir::Expression::HexNumberExpression(hex_number_expression) => {
                self.binder.node_typing(hex_number_expression.node_id)
            }
            input_ir::Expression::DecimalNumberExpression(decimal_number_expression) => {
                self.binder.node_typing(decimal_number_expression.node_id)
            }
            input_ir::Expression::StringExpression(string_expression) => self
                .binder
                .node_typing(Self::string_expression_node_id(string_expression)),
            input_ir::Expression::ElementaryType(elementary_type) => {
                Typing::MetaType(Self::type_of_elementary_type(elementary_type))
            }
            input_ir::Expression::Identifier(identifier) => self.typing_of_identifier(identifier),
            input_ir::Expression::PayableKeyword => {
                Typing::MetaType(Type::Address { payable: true })
            }
            input_ir::Expression::ThisKeyword => Typing::This,
            input_ir::Expression::SuperKeyword => Typing::Super,
        }
    }

    fn type_of_elementary_type(elementary_type: &input_ir::ElementaryType) -> Type {
        match elementary_type {
            input_ir::ElementaryType::AddressType(address_type) => Type::Address {
                payable: address_type.payable_keyword.is_some(),
            },
            input_ir::ElementaryType::BytesKeyword(terminal) => {
                Type::from_bytes_keyword(&terminal.unparse(), Some(DataLocation::Memory)).unwrap()
            }
            input_ir::ElementaryType::IntKeyword(terminal) => {
                Type::from_int_keyword(&terminal.unparse())
            }
            input_ir::ElementaryType::UintKeyword(terminal) => {
                Type::from_uint_keyword(&terminal.unparse())
            }
            input_ir::ElementaryType::FixedKeyword(terminal) => {
                Type::from_fixed_keyword(&terminal.unparse())
            }
            input_ir::ElementaryType::UfixedKeyword(terminal) => {
                Type::from_ufixed_keyword(&terminal.unparse())
            }
            input_ir::ElementaryType::BoolKeyword => Type::Boolean,
            input_ir::ElementaryType::ByteKeyword => Type::ByteArray { width: 1 },
            input_ir::ElementaryType::StringKeyword => Type::String {
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

    fn typing_of_identifier(&self, identifier: &Rc<TerminalNode>) -> Typing {
        let resolution = self
            .binder
            .find_reference_by_identifier_node_id(identifier.id())
            .unwrap()
            .resolution
            .clone();
        self.typing_of_resolution(&resolution)
    }

    pub(super) fn type_of_integer_binary_expression(
        &self,
        left_operand: &input_ir::Expression,
        right_operand: &input_ir::Expression,
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

    pub(super) fn typing_of_resolution_as_getter(&self, resolution: &Resolution) -> Typing {
        if let Resolution::Definition(definition_id) = resolution {
            if let Definition::StateVariable(state_var_definition) =
                self.binder.find_definition_by_id(*definition_id).unwrap()
            {
                if let Some(getter_type_id) = state_var_definition.getter_type_id {
                    return Typing::Resolved(getter_type_id);
                }
            }
        }
        self.typing_of_resolution(resolution)
    }

    fn type_id_of_receiver(&self, operand: &input_ir::Expression) -> Option<TypeId> {
        if let input_ir::Expression::MemberAccessExpression(member_access_expression) = operand {
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
                // special case: address(uint160(_)) should type to `address payable` in <0.8.0
                } else if self.language_version < VERSION_0_8_0
                    && self.language_version >= VERSION_0_5_0
                    && matches!(
                        argument_type,
                        Type::Integer {
                            signed: false,
                            bits: 160
                        }
                    )
                    && matches!(target_type, Type::Address { .. })
                {
                    self.types.address_payable()
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
        arguments: &[input_ir::Expression],
    ) -> Vec<Typing> {
        arguments
            .iter()
            .map(|argument| self.typing_of_expression(argument))
            .collect::<Vec<_>>()
    }

    pub(super) fn typing_of_function_call_with_positional_arguments(
        &mut self,
        node: &input_ir::FunctionCallExpression,
        arguments: &[input_ir::Expression],
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
        arguments: &[input_ir::NamedArgument],
    ) -> Vec<(String, Typing)> {
        arguments
            .iter()
            .map(|argument| {
                (
                    argument.name.unparse(),
                    self.typing_of_expression(&argument.value),
                )
            })
            .collect::<Vec<_>>()
    }

    pub(super) fn typing_of_function_call_with_named_arguments(
        &mut self,
        node: &input_ir::FunctionCallExpression,
        arguments: &[input_ir::NamedArgument],
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
fn reference_node_id_for_expression(node: &input_ir::Expression) -> Option<NodeId> {
    match &node {
        input_ir::Expression::MemberAccessExpression(f) => Some(f.member.id()),
        input_ir::Expression::Identifier(f) => Some(f.id()),
        input_ir::Expression::CallOptionsExpression(f) => {
            reference_node_id_for_expression(&f.operand)
        }
        _ => None,
    }
}

/// Typing functions for literals
impl Pass {
    pub(super) fn type_of_string_expression(node: &input_ir::StringExpression) -> Type {
        let kind = match node {
            input_ir::StringExpression::StringLiteral(string_literal) => {
                let size = Self::string_literal_size(string_literal);
                LiteralKind::String {
                    bytes: u32::try_from(size).unwrap(),
                }
            }
            input_ir::StringExpression::StringLiterals(literals) => {
                let size = literals.iter().fold(0usize, |acc, literal| {
                    acc + Self::string_literal_size(literal)
                });
                LiteralKind::String {
                    bytes: u32::try_from(size).unwrap(),
                }
            }
            input_ir::StringExpression::HexStringLiteral(hex_string_literal) => {
                let size = Self::hex_string_literal_bytes_size(hex_string_literal);
                LiteralKind::HexString {
                    bytes: u32::try_from(size).unwrap(),
                }
            }
            input_ir::StringExpression::HexStringLiterals(literals) => {
                let size = literals.iter().fold(0usize, |acc, literal| {
                    acc + Self::hex_string_literal_bytes_size(literal)
                });
                LiteralKind::HexString {
                    bytes: u32::try_from(size).unwrap(),
                }
            }
            input_ir::StringExpression::UnicodeStringLiterals(literals) => {
                let size = literals.iter().fold(0usize, |acc, literal| {
                    acc + Self::unicode_string_literal_size(literal)
                });
                LiteralKind::String {
                    bytes: u32::try_from(size).unwrap(),
                }
            }
        };
        Type::Literal(kind)
    }

    pub(super) fn string_expression_node_id(
        string_expression: &input_ir::StringExpression,
    ) -> NodeId {
        match string_expression {
            input_ir::StringExpression::StringLiteral(string_literal) => {
                Self::string_literal_node_id(string_literal)
            }
            input_ir::StringExpression::StringLiterals(string_literals) => {
                Self::string_literal_node_id(&string_literals[0])
            }
            input_ir::StringExpression::HexStringLiteral(hex_string_literal) => {
                Self::hex_string_literal_node_id(hex_string_literal)
            }
            input_ir::StringExpression::HexStringLiterals(hex_string_literals) => {
                Self::hex_string_literal_node_id(&hex_string_literals[0])
            }
            input_ir::StringExpression::UnicodeStringLiterals(unicode_string_literals) => {
                Self::unicode_string_literal_node_id(&unicode_string_literals[0])
            }
        }
    }

    fn string_literal_node_id(string_literal: &input_ir::StringLiteral) -> NodeId {
        match string_literal {
            input_ir::StringLiteral::SingleQuotedStringLiteral(terminal_node)
            | input_ir::StringLiteral::DoubleQuotedStringLiteral(terminal_node) => {
                terminal_node.id()
            }
        }
    }

    fn string_literal_size(string_literal: &input_ir::StringLiteral) -> usize {
        match string_literal {
            input_ir::StringLiteral::SingleQuotedStringLiteral(terminal_node)
            | input_ir::StringLiteral::DoubleQuotedStringLiteral(terminal_node) => {
                // TODO: consider escaped characters
                terminal_node.unparse().len() - 2
            }
        }
    }

    fn hex_string_literal_node_id(hex_string_literal: &input_ir::HexStringLiteral) -> NodeId {
        match hex_string_literal {
            input_ir::HexStringLiteral::SingleQuotedHexStringLiteral(terminal_node)
            | input_ir::HexStringLiteral::DoubleQuotedHexStringLiteral(terminal_node) => {
                terminal_node.id()
            }
        }
    }

    fn hex_string_literal_bytes_size(hex_string_literal: &input_ir::HexStringLiteral) -> usize {
        match hex_string_literal {
            input_ir::HexStringLiteral::SingleQuotedHexStringLiteral(terminal_node)
            | input_ir::HexStringLiteral::DoubleQuotedHexStringLiteral(terminal_node) => {
                // 5 is the length of the `hex` prefix plus the quotes
                (terminal_node.unparse().len() - 5) / 2
            }
        }
    }

    fn unicode_string_literal_node_id(
        unicode_string_literal: &input_ir::UnicodeStringLiteral,
    ) -> NodeId {
        match unicode_string_literal {
            input_ir::UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(terminal_node)
            | input_ir::UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(terminal_node) => {
                terminal_node.id()
            }
        }
    }

    fn unicode_string_literal_size(string_literal: &input_ir::UnicodeStringLiteral) -> usize {
        match string_literal {
            input_ir::UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(terminal_node)
            | input_ir::UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(terminal_node) => {
                // TODO: actually parse the string
                // 9 is the length of the `unicode` prefix plus quotes
                terminal_node.unparse().len() - 9
            }
        }
    }

    pub(super) fn hex_number_literal_kind(
        hex_number_expression: &input_ir::HexNumberExpression,
    ) -> LiteralKind {
        if hex_number_expression.unit.is_some() {
            // this is deprecated in Solidity >= 0.5.0 anyway
            return LiteralKind::DecimalInteger;
        }
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
