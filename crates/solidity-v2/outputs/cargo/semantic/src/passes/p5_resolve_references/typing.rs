use ruint::aliases::{U160, U256};
use slang_solidity_v2_common::diagnostics::kinds::type_system::CannotCallViaContractTypeName;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::NodeIdentity;

use super::Pass;
use crate::binder::{Definition, Resolution, Typing};
use crate::passes::common::node_location;
use crate::types::{
    literals, AddressType, ContractType, DataLocation, FixedSizeArrayType, FunctionType,
    FunctionTypeVisibility, IntegerType, LiteralKind, MetaType, Number, StringType, Type, TypeId,
    UserMetaType,
};

impl Pass<'_> {
    /// Registers `inner` and the [`Type::MetaType`] wrapping it, returning the
    /// meta-type as a `Typing::Resolved`.
    pub(super) fn meta_typing_of(&mut self, inner: Type) -> Typing {
        let type_id = self.types.register_type(inner);
        Typing::Resolved(
            self.types
                .register_type(Type::MetaType(MetaType { type_id })),
        )
    }

    pub(super) fn typing_of_expression(&self, node: &ir::Expression) -> Typing {
        // Every expression variant registers its typing in the binder during
        // the pass, so we simply look it up by `NodeId`.
        let node_id = node
            .node_id()
            .expect("expression should have a NodeId to look up its typing");
        self.binder.node_typing(node_id)
    }

    pub(super) fn type_of_elementary_type(elementary_type: &ir::ElementaryType) -> Type {
        match elementary_type {
            ir::ElementaryType::AddressType(address_type) => Type::Address(AddressType {
                is_payable: address_type.is_payable,
            }),
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
            ir::ElementaryType::BoolKeyword(_) => Type::Boolean,
            ir::ElementaryType::StringKeyword(_) => Type::String(StringType {
                location: DataLocation::Memory,
            }),
        }
    }

    pub(super) fn type_of_definition(&self, definition_id: NodeId) -> Option<Type> {
        let definition = self.binder.find_definition_by_id(definition_id)?;
        definition.try_into().ok()
    }

    /// The common type both operands of a comparison reconcile to, through the
    /// shared `TypeRegistry::common_type` the integer-operator and ternary paths
    /// also use. Recorded apart from the comparison's boolean result type, unlike
    /// arithmetic where the reconciled type *is* the result.
    pub(super) fn common_type_of_operands(
        &mut self,
        left_operand: &ir::Expression,
        right_operand: &ir::Expression,
    ) -> Option<TypeId> {
        let left = self.typing_of_expression(left_operand).as_type_id()?;
        let right = self.typing_of_expression(right_operand).as_type_id()?;
        self.types.common_type(left, right)
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

        // TODO(validation) SDR[44]: check that both operands are valid for the operator
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
            // TODO(validation) SDR[43]: the types are not compatible, we should
            // emit an error, or signal our caller
            None
        }
    }

    pub(super) fn type_of_prefix_expression(
        &mut self,
        node: &ir::PrefixExpression,
    ) -> Option<TypeId> {
        match node.operator {
            ir::PrefixExpressionOperator::Minus(_) | ir::PrefixExpressionOperator::Tilde(_) => {
                // Fold `-<constant>` or `~<constant>` by operating on the
                // operand's known number value.
                let operand_type_id = self.typing_of_expression(&node.operand).as_type_id()?;
                if let Some(value) = self.types.number_value_of_type_id(operand_type_id) {
                    let result = match node.operator {
                        ir::PrefixExpressionOperator::Minus(_) => value.negate(),
                        ir::PrefixExpressionOperator::Tilde(_) => value.bit_not()?,
                        _ => unreachable!(),
                    };
                    Some(
                        self.types
                            .register_type(Type::Literal(result.to_literal_kind())),
                    )
                } else {
                    // TODO(validation) SDR[1734]: check that the operand type supports the operator
                    Some(operand_type_id)
                }
            }
            ir::PrefixExpressionOperator::PlusPlus(_)
            | ir::PrefixExpressionOperator::MinusMinus(_) => {
                self.typing_of_expression(&node.operand).as_type_id()
            }
            ir::PrefixExpressionOperator::Bang(_) => {
                // TODO(validation) SDR[49]: check that the operand is boolean
                Some(self.types.boolean())
            }
            ir::PrefixExpressionOperator::DeleteKeyword(_) => Some(self.types.void()),
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
        let element_type = self.types.type_of_array_literal(&item_type_ids)?;
        Some(
            self.types
                .register_type(Type::FixedSizeArray(FixedSizeArrayType {
                    element_type,
                    size: U256::from(array.items.len()),
                    location: DataLocation::Memory,
                })),
        )
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
                Some(self.types.register_type(Type::Integer(IntegerType {
                    is_signed: true,
                    bits: 256,
                })))
            } else {
                Some(self.types.uint256())
            }
        } else {
            // TODO(validation) SDR[1735]: check that the operand types are valid (needs
            // additional parameter or validation at call site)
            Some(left_type_id)
        }
    }

    pub(super) fn typing_of_resolution(&mut self, resolution: &Resolution) -> Typing {
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
            Typing::This(type_id) | Typing::Resolved(type_id) => matches!(
                self.types.get_type_by_id(*type_id),
                Type::Contract(_) | Type::Interface(_)
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

    /// Whether `contract_id` is accessed from a scope that neither is it nor
    /// derives from it (solc's "Foreign" access). The linearisation includes
    /// the contract itself, so containment means local/deriving access.
    pub(crate) fn is_foreign_contract(&self, contract_id: NodeId) -> bool {
        let Some(scope_id) = self.current_contract_scope_id() else {
            return true;
        };
        let current_contract_id = self.binder.get_scope_by_id(scope_id).node_id();
        !self
            .binder
            .get_linearised_bases(current_contract_id)
            .is_some_and(|bases| bases.contains(&contract_id))
    }

    /// Returns the typing of the *receiver* of a call — the operand of the
    /// member access being called (eg. for `a.f(...)`, the typing of `a`).
    /// Returns `None` when the call target is not a member access.
    fn type_id_of_value_receiver(&self, operand: &ir::Expression) -> Option<TypeId> {
        if let ir::Expression::MemberAccessExpression(member_access_expression) = operand {
            let type_id = self
                .typing_of_expression(&member_access_expression.operand)
                .as_type_id()?;
            // A meta-type operand is a namespace qualifier, not
            // a runtime value, so there is no receiver to bind as an implicit
            // first argument during overload resolution.
            if self.types.get_type_by_id(type_id).is_meta_type() {
                return None;
            }
            Some(type_id)
        } else {
            None
        }
    }

    /// Adjusts the type of a member reached through a member access whose
    /// operand types as `operand_typing`:
    /// - reference types with an "inherited" data location take the operand's
    ///   location;
    /// - functions attached via `using for` bind the receiver as their first
    ///   argument, producing a partially applied function;
    /// - function declarations that are not callable through the operand's
    ///   contract/interface type name become the user meta type of their
    ///   definition.
    pub(super) fn adjust_member_access_type_for_operand(
        &mut self,
        type_id: TypeId,
        operand_typing: &Typing,
    ) -> TypeId {
        let type_ = self.types.get_type_by_id(type_id);

        if type_.is_inherited_location() {
            if let Some(operand_location) = operand_typing
                .as_type_id()
                .and_then(|type_id| self.types.get_type_by_id(type_id).data_location())
            {
                let type_ = type_.clone();
                return self
                    .types
                    .register_type_with_data_location(type_, operand_location);
            }
        } else if let Type::Function(function_type) = type_ {
            if let Some(receiver_type_id) = operand_typing.as_type_id() {
                if function_type.implicit_receiver_type.is_none()
                    && function_type.parameter_types.first().is_some_and(|first| {
                        self.types
                            .implicitly_convertible_to_for_external_call(receiver_type_id, *first)
                    })
                {
                    return self
                        .types
                        .partially_apply_function_type(function_type.clone());
                }

                if let Some(declaration_type_id) = self.as_foreign_function_declaration_type(
                    receiver_type_id,
                    function_type.definition_id,
                    function_type.visibility,
                ) {
                    return declaration_type_id;
                }
            }
        }
        type_id
    }

    /// Returns the user meta type of a function's definition when reaching the
    /// function through a contract/interface *type name* (eg. `C.g`) makes it
    /// a non-callable declaration with no mobile type: external functions
    /// always, and public functions of a *foreign* contract. Internal/private
    /// functions reached the same way stay normal callables (qualified base
    /// calls), as do members of library type names (`L.f`).
    fn as_foreign_function_declaration_type(
        &mut self,
        receiver_type_id: TypeId,
        function_definition_id: Option<NodeId>,
        visibility: FunctionTypeVisibility,
    ) -> Option<TypeId> {
        let Type::UserMetaType(UserMetaType { definition_id }) =
            self.types.get_type_by_id(receiver_type_id)
        else {
            return None;
        };
        if !matches!(
            self.binder.find_definition_by_id(*definition_id),
            Some(Definition::Contract(_) | Definition::Interface(_))
        ) {
            return None;
        }

        let is_foreign_and_visible = match visibility {
            FunctionTypeVisibility::External => true,
            FunctionTypeVisibility::Public => self.is_foreign_contract(*definition_id),
            FunctionTypeVisibility::Internal | FunctionTypeVisibility::Private => false,
        };
        if !is_foreign_and_visible {
            return None;
        }

        let definition_id = function_definition_id?;
        Some(
            self.types
                .register_type(Type::UserMetaType(UserMetaType { definition_id })),
        )
    }

    fn typing_of_cast(&mut self, argument_typing: &Typing, target_type_id: TypeId) -> Typing {
        // TODO(validation) SDR[40]: this is a cast to the given type, but we
        // need to verify that the (single) argument is convertible
        match argument_typing.as_type_id() {
            Some(argument_type_id) => {
                // the resulting cast type inherits the data location of the argument
                let argument_type = self.types.get_type_by_id(argument_type_id);
                let type_id = if let Some(data_location) = argument_type.data_location() {
                    let target_type = self.types.get_type_by_id(target_type_id).clone();
                    self.types
                        .register_type_with_data_location(target_type, data_location)
                } else {
                    target_type_id
                };
                Typing::Resolved(type_id)
            }
            None => Typing::Unresolved,
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
            Typing::Unresolved | Typing::This(_) | Typing::Super => {
                // `this` and `super` are not callable
                Typing::Unresolved
            }
            Typing::NewExpression(type_id) => {
                match self.types.get_type_by_id(type_id) {
                    Type::Array(_) | Type::Contract(_) => Typing::Resolved(type_id),
                    _ => {
                        // only contracts can be created with `new`
                        Typing::Unresolved
                    }
                }
            }
            Typing::BuiltIn(built_in) => self
                .built_ins_resolver()
                .typing_of_function_call(&built_in, &argument_typings),

            Typing::Resolved(type_id) => self.typing_of_type_called_with_positional_arguments(
                node,
                type_id,
                &argument_typings,
            ),
            Typing::Undetermined(type_ids) => {
                let receiver_type_id = self.type_id_of_value_receiver(&node.operand);
                let candidate = self.lookup_function_matching_positional_arguments(
                    &type_ids,
                    &argument_typings,
                    receiver_type_id,
                );

                if let Some(candidate_type_id) = candidate {
                    // The operand disambiguates to the selected overload, even
                    // when calling it turns out to be invalid
                    self.fixup_operand_expression(&node.operand, candidate_type_id);

                    self.typing_of_type_called_with_positional_arguments(
                        node,
                        candidate_type_id,
                        &argument_typings,
                    )
                } else {
                    Typing::Unresolved
                }
            }
        }
    }

    /// Narrows an overloaded call operand from its whole candidate set down to
    /// the single selected overload `candidate_type_id`: both the operand's
    /// reference (so it points at the selected declaration) and its recorded
    /// typing (so querying the operand yields the selected overload rather than
    /// the ambiguous `Undetermined` set) are updated.
    fn fixup_operand_expression(&mut self, operand: &ir::Expression, candidate_type_id: TypeId) {
        if let (Some(reference_node_id), Some(definition_id)) = (
            reference_node_id_for_expression(operand),
            self.candidate_definition_id(candidate_type_id),
        ) {
            self.binder
                .fixup_reference(reference_node_id, Resolution::Definition(definition_id));
        }

        if let Some(operand_node_id) = operand.node_id() {
            self.binder
                .update_node_typing(operand_node_id, Typing::Resolved(candidate_type_id));
        }
    }

    /// Types a call whose operand is (or was disambiguated to) `type_id`: a
    /// function value is an actual call, a meta type is a cast (or a struct
    /// construction), and the user meta type of a function is a non-callable
    /// declaration reached through a contract/interface type name.
    fn typing_of_type_called_with_positional_arguments(
        &mut self,
        node: &ir::FunctionCallExpression,
        type_id: TypeId,
        argument_typings: &[Typing],
    ) -> Typing {
        match self.types.get_type_by_id(type_id) {
            Type::Function(FunctionType { return_type, .. }) => Typing::Resolved(*return_type),
            Type::MetaType(MetaType {
                type_id: target_type_id,
            }) => {
                // This is an explicit cast to the (meta-)type, eg. `uint(x)`.
                let target_type_id = *target_type_id;
                if argument_typings.len() == 1 {
                    self.typing_of_cast(&argument_typings[0], target_type_id)
                } else {
                    Typing::Unresolved
                }
            }
            Type::UserMetaType(UserMetaType { definition_id }) => {
                // A cast to the underlying type of the definition (eg.
                // `MyEnum(1)`), or a struct construction. UDVTs are not
                // castable by name (they convert via `wrap`/`unwrap`).
                let definition_id = *definition_id;
                match self.binder.find_definition_by_id(definition_id) {
                    Some(
                        Definition::Contract(_)
                        | Definition::Interface(_)
                        | Definition::Library(_)
                        | Definition::Enum(_)
                        | Definition::Struct(_),
                    ) => {
                        // TODO(validation) SDR[39]: for contract, interface
                        // and library targets the type of the (single)
                        // argument should be an address
                        // TODO(validation) SDR[868]: For enums, only one argument expected
                        // TODO(validation) SDR[1698]: For enums, check the type of the argument is compatible

                        let type_ = self
                            .type_of_definition(definition_id)
                            .expect("definition kind is handled by type_of_definition");
                        Typing::Resolved(self.types.register_type(type_))
                    }
                    Some(Definition::Function(_)) => {
                        // Calling a function referenced through a contract/interface
                        // type name (eg. `C.f()`) is invalid: it's a non-callable
                        // declaration.
                        let (file_id, range) = node_location(node, self.file_node_mapper);

                        self.diagnostics
                            .push(file_id, range, CannotCallViaContractTypeName);
                        Typing::Unresolved
                    }

                    _ => Typing::Unresolved,
                }
            }
            _ => {
                // TODO(validation) SDR[41]: the operand did not resolve to a function
                Typing::Unresolved
            }
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
            Typing::Unresolved | Typing::This(_) | Typing::Super => {
                // `this` and `super` are not callable
                (Typing::Unresolved, None)
            }
            Typing::Resolved(type_id) => {
                self.typing_of_type_called_with_named_arguments(node, type_id)
            }
            Typing::Undetermined(type_ids) => {
                let receiver_type_id = self.type_id_of_value_receiver(&node.operand);
                let argument_typings = self.collect_named_argument_typings(arguments);
                let candidate = self.lookup_function_matching_named_arguments(
                    &type_ids,
                    &argument_typings,
                    receiver_type_id,
                );

                if let Some(candidate_type_id) = candidate {
                    // The operand disambiguates to the selected overload, even
                    // when calling it turns out to be invalid
                    self.fixup_operand_expression(&node.operand, candidate_type_id);

                    self.typing_of_type_called_with_named_arguments(node, candidate_type_id)
                } else {
                    (Typing::Unresolved, None)
                }
            }
            Typing::NewExpression(type_id) => {
                if let Type::Contract(ContractType { definition_id }) =
                    self.types.get_type_by_id(type_id)
                {
                    (Typing::Resolved(type_id), Some(*definition_id))
                } else {
                    // only contracts can be created with `new`
                    (Typing::Unresolved, None)
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

    /// Types a call with named arguments whose operand is (or was
    /// disambiguated to) `type_id`, additionally returning the definition the
    /// operand resolves to (if any). Only function values and struct
    /// constructions are callable this way; the user meta type of a function is
    /// a non-callable declaration reached through a contract/interface type
    /// name.
    fn typing_of_type_called_with_named_arguments(
        &mut self,
        node: &ir::FunctionCallExpression,
        type_id: TypeId,
    ) -> (Typing, Option<NodeId>) {
        match self.types.get_type_by_id(type_id) {
            Type::Function(FunctionType {
                definition_id,
                return_type,
                ..
            }) => (Typing::Resolved(*return_type), *definition_id),
            Type::MetaType(_) => {
                // This is a cast to the given type and is not valid with named arguments
                (Typing::Unresolved, None)
            }
            Type::UserMetaType(UserMetaType { definition_id }) => {
                // Function call with named arguments are only valid in user
                // types of the struct kind, which results in the construction
                // of such struct in memory
                let definition_id = *definition_id;
                match self.binder.find_definition_by_id(definition_id) {
                    Some(Definition::Struct(_)) => {
                        // struct construction
                        let type_ = self
                            .type_of_definition(definition_id)
                            .expect("struct definitions are handled by type_of_definition");
                        let type_id = self.types.register_type(type_);
                        (Typing::Resolved(type_id), Some(definition_id))
                    }
                    Some(Definition::Function(_)) => {
                        // Calling a function via a contract/interface type name is
                        // invalid. Still return the definition so the argument
                        // names resolve against its parameters.
                        let (file_id, range) = node_location(node, self.file_node_mapper);

                        self.diagnostics
                            .push(file_id, range, CannotCallViaContractTypeName);
                        (Typing::Unresolved, Some(definition_id))
                    }

                    _ => (Typing::Unresolved, None),
                }
            }
            _ => {
                // TODO(validation) SDR[41]: the operand did not resolve to a function
                (Typing::Unresolved, None)
            }
        }
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
            // TODO(validation) SDR[38]: verify the address is valid (ie. has a valid checksum)
            // We need at least an implementation of SHA3 to compute the checksum

            // Skip `0x` prefix and parse the hexadecimal number.
            // `U160::from_str_radix` ignores `_` separators.
            let value = U160::from_str_radix(&hex_number[2..], 16).ok()?;
            return Some(LiteralKind::Address { value });
        }
        let value = Number::from_hex_number_expression(hex_number_expression)?
            .into_integer()
            .expect("hex literal must parse to an integer")
            .to_biguint()
            .expect("hex literal must be non-negative");
        // Each pair of hex digits is one byte (with odd digit counts rounded up).
        let bytes = digits.div_ceil(2).max(1);
        Some(LiteralKind::HexInteger { value, bytes })
    }
}
