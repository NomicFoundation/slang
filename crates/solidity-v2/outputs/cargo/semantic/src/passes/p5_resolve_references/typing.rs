use ruint::aliases::{U160, U256};
use slang_solidity_v2_common::diagnostics::kinds::resolution::{
    AmbiguousReference, MemberNotFound, NoMatchingCallableDeclaration,
};
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    CannotCallViaContractTypeName, ExpressionNotCallable,
};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::NodeIdentity;

use super::Pass;
use super::disambiguation::OverloadMatch;
use crate::binder::{Definition, Resolution, Typing};
use crate::built_ins::BuiltInCallError;
use crate::passes::common::node_location;
use crate::types::{
    AddressType, ArraySliceType, ArrayType, ContractType, DataLocation, FixedSizeArrayType,
    FunctionType, FunctionTypeVisibility, IntegerType, LiteralKind, MetaType, Number, StringType,
    Type, TypeId, UserMetaType, literals,
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

    /// The typing of `node` where its value is used, which is everywhere but
    /// the callee of a call. Nothing can select from an overload set here, so
    /// one that reaches this point is sunk to `Unresolved` rather than handed
    /// on undetermined.
    #[inline]
    pub(super) fn typing_of_expression(&mut self, node: &ir::Expression) -> Typing {
        match self.typing_of_callee_expression(node) {
            Typing::Undetermined(_) => {
                self.report_ambiguous_reference(node);
                Typing::Unresolved
            }
            typing => typing,
        }
    }

    /// The typing `node` registered, which for a callee may be the whole
    /// overload set: the call's arguments are what select from it.
    pub(super) fn typing_of_callee_expression(&self, node: &ir::Expression) -> Typing {
        // Every expression variant registers its typing in the binder during
        // the pass, so we simply look it up by `NodeId`.
        let node_id = node
            .node_id()
            .expect("expression should have a NodeId to look up its typing");
        self.binder.node_typing(node_id)
    }

    /// Narrows an overload lookup for a call down to the selected candidate,
    /// reporting when the arguments select none or several of them. A
    /// candidate is returned whenever there is one, so an ambiguous call can
    /// still be typed and its reference still points somewhere.
    pub(super) fn select_overload<T>(
        &mut self,
        operand: &ir::Expression,
        overload_match: OverloadMatch<T>,
    ) -> Option<T> {
        let identifier = reference_identifier_for_expression(operand)
            .expect("Overloaded operand cannot be traced back to identifier");
        match overload_match {
            OverloadMatch::None => {
                // Ruled out by the arguments: on a member that reads as
                // the operand's type not providing it, on a bare name as
                // a failed declaration lookup.
                if matches!(operand, ir::Expression::MemberAccessExpression(_)) {
                    let name = identifier.unparse().to_owned();
                    self.push_diagnostic(identifier, MemberNotFound { name });
                } else {
                    self.push_diagnostic(identifier, NoMatchingCallableDeclaration);
                }
                None
            }
            OverloadMatch::Unique(selected) => Some(selected),
            OverloadMatch::Ambiguous(selected) => {
                self.report_ambiguous_identifier(identifier);
                Some(selected)
            }
        }
    }

    /// [`Self::select_overload`] for an emitted event, whose name is an
    /// identifier path rather than an expression.
    pub(super) fn select_event_overload<T>(
        &mut self,
        identifier: &ir::Identifier,
        overload_match: OverloadMatch<T>,
    ) -> Option<T> {
        match overload_match {
            OverloadMatch::None => {
                self.push_diagnostic(identifier, NoMatchingCallableDeclaration);
                None
            }
            OverloadMatch::Unique(selected) => Some(selected),
            OverloadMatch::Ambiguous(selected) => {
                self.report_ambiguous_identifier(identifier);
                Some(selected)
            }
        }
    }

    /// Reports `node` as a reference that matched more than one declaration.
    /// Kept out of line so that reporting, which is rare, does not stop
    /// [`Self::typing_of_expression`] from inlining into its many callers.
    #[cold]
    #[inline(never)]
    fn report_ambiguous_reference(&mut self, node: &ir::Expression) {
        if let Some(identifier) = reference_identifier_for_expression(node) {
            self.report_ambiguous_identifier(identifier);
        }
    }

    /// The diagnostic is located at the identifier the reference was made
    /// through, which is also where its name comes from.
    fn report_ambiguous_identifier(&mut self, identifier: &ir::Identifier) {
        let name = identifier.unparse().to_owned();
        self.push_diagnostic(identifier, AmbiguousReference { name });
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

    /// Records the common type both operands of a comparison reconcile to
    /// before the comparison runs.
    pub(super) fn record_comparison_common_operand_type(
        &mut self,
        node_id: NodeId,
        left_typing: &Typing,
        right_typing: &Typing,
    ) {
        let common = left_typing
            .as_type_id()
            .zip(right_typing.as_type_id())
            .and_then(|(left, right)| self.types.common_operand_type(left, right));
        self.binder.set_common_operand_type(node_id, common);
    }

    /// Returns the type of an binary operator expression. If both operands are
    /// number literals, applies `op` to fold them into a narrowed literal type;
    /// otherwise falls back to the implicit-convertibility rule between the
    /// operand types.
    pub(super) fn type_of_binary_operator_expression<F>(
        &mut self,
        left_typing: &Typing,
        right_typing: &Typing,
        op: F,
    ) -> Option<TypeId>
    where
        F: FnOnce(&Number, &Number) -> Option<Number>,
    {
        let left_type_id = left_typing.as_type_id()?;
        let right_type_id = right_typing.as_type_id()?;

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
        //
        // TODO(validation) SDR[43]: a `None` here means the types are not
        // compatible; we should emit an error, or signal our caller.
        self.types.common_type(left_type_id, right_type_id)
    }

    pub(super) fn type_of_prefix_expression(
        &mut self,
        node: &ir::PrefixExpression,
        operand_typing: &Typing,
    ) -> Option<TypeId> {
        match node.operator {
            ir::PrefixExpressionOperator::Minus(_) | ir::PrefixExpressionOperator::Tilde(_) => {
                // Fold `-<constant>` or `~<constant>` by operating on the
                // operand's known number value.
                let operand_type_id = operand_typing.as_type_id()?;
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
            | ir::PrefixExpressionOperator::MinusMinus(_) => operand_typing.as_type_id(),
            ir::PrefixExpressionOperator::Bang(_) => {
                // TODO(validation) SDR[49]: check that the operand is boolean
                Some(self.types.boolean())
            }
            ir::PrefixExpressionOperator::DeleteKeyword(_) => Some(self.types.void()),
        }
    }

    /// Types a range index (`a[i:j]`) whose operand is the array `array_type_id`
    /// in `location`. Only dynamically-sized calldata arrays can be sliced, so a
    /// calldata operand yields a [`Type::ArraySlice`] wrapping it and anything
    /// else is left unresolved.
    pub(super) fn slice_typing(&mut self, array_type_id: TypeId, location: DataLocation) -> Typing {
        if location == DataLocation::Calldata {
            Typing::Resolved(
                self.types
                    .register_type(Type::ArraySlice(ArraySliceType { array_type_id })),
            )
        } else {
            // TODO(validation) SDR[46]: slicing a non-calldata array is invalid.
            Typing::Unresolved
        }
    }

    /// Unlike an array type name's length, a constant is not a length here: in
    /// expression position solc folds literals only, and rejects `uint[N]`.
    pub(super) fn typing_of_indexed_meta_type(
        &mut self,
        element_type: TypeId,
        size: Option<&ir::Expression>,
    ) -> Typing {
        let Some(size) = size else {
            return self.meta_typing_of(Type::Array(ArrayType {
                element_type,
                location: DataLocation::Memory,
            }));
        };
        let Some(size) = self.literal_array_size(size) else {
            // TODO(validation): a non-literal index is not an array length
            return Typing::Unresolved;
        };
        self.meta_typing_of(Type::FixedSizeArray(FixedSizeArrayType {
            element_type,
            size,
            location: DataLocation::Memory,
        }))
    }

    /// The length an array index denotes, or `None` unless it is a non-negative
    /// integer literal that fits a `U256`. The value is taken from the
    /// expression type to benefit from the constant folding of literals.
    fn literal_array_size(&mut self, size: &ir::Expression) -> Option<U256> {
        let type_id = self.typing_of_expression(size).as_type_id()?;
        let value = self.types.number_value_of_type_id(type_id)?;
        U256::try_from(value.as_integer()?).ok()
    }

    pub(super) fn type_of_array_expression(
        &mut self,
        array: &ir::ArrayExpression,
    ) -> Option<TypeId> {
        let mut item_type_ids: Vec<TypeId> = Vec::with_capacity(array.items.len());
        for item in array.items.iter() {
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
        left_typing: &Typing,
        right_typing: &Typing,
        op: F,
    ) -> Option<TypeId>
    where
        F: FnOnce(&Number, &Number) -> Option<Number>,
    {
        let left_type_id = left_typing.as_type_id()?;
        let right_type_id = right_typing.as_type_id()?;

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
        // Check if the target is a state variable with a getter or a function
        // with an externalized type; the member is accessed through that type.
        if let Resolution::Definition(definition_id) = resolution
            && let Some(member_type_id) =
                match self.binder.find_definition_by_id(*definition_id).unwrap() {
                    Definition::StateVariable(state_var_definition) => {
                        state_var_definition.getter_type_id
                    }
                    Definition::Function(function_definition) => {
                        function_definition.externalized_type_id
                    }
                    _ => None,
                }
        {
            return Typing::Resolved(member_type_id);
        }

        let mut typing = self.typing_of_resolution(resolution);

        // If the resolved type is a function and the operand is either
        // `this` or something of an address type, the function is being
        // used as an external function: change the expression typing to
        // indicate the external access.
        if let Some(type_id) = typing.as_type_id()
            && let Type::Function(function_type) = self.types.get_type_by_id(type_id)
            && function_type.is_externally_visible()
        {
            typing = Typing::Resolved(self.types.externalize_function_type(type_id));
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
    fn type_id_of_value_receiver(&mut self, operand: &ir::Expression) -> Option<TypeId> {
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
        } else if let Type::Function(function_type) = type_
            && let Some(receiver_type_id) = operand_typing.as_type_id()
        {
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
        &mut self,
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
        let operand_typing = self.typing_of_callee_expression(&node.operand);
        let argument_typings = self.collect_positional_argument_typings(arguments);

        match operand_typing {
            Typing::Unresolved => {
                // The callee failed to resolve, which is already reported: it
                // is only uncallable as a consequence.
                Typing::Unresolved
            }
            Typing::This(_) | Typing::Super => {
                // `this` and `super` name a contract instance, not a callable
                self.report_operand_not_callable(node);
                Typing::Unresolved
            }
            Typing::NewExpression(type_id) => {
                match self.types.get_type_by_id(type_id) {
                    // `new` creates a contract, a dynamic array (`new T[](n)`),
                    // or a dynamic `bytes`/`string` (`new bytes(n)`), yielding a
                    // value of that type.
                    Type::Array(_) | Type::Contract(_) | Type::Bytes(_) | Type::String(_) => {
                        Typing::Resolved(type_id)
                    }
                    _ => Typing::Unresolved,
                }
            }
            Typing::BuiltIn(built_in) => {
                match self
                    .built_ins_resolver()
                    .type_of_function_call(&built_in, &argument_typings)
                {
                    Ok(type_id) => Typing::Resolved(type_id),
                    Err(error) => {
                        self.report_built_in_call_error(node, error);
                        Typing::Unresolved
                    }
                }
            }

            Typing::Resolved(type_id) => self.typing_of_type_called_with_positional_arguments(
                node,
                type_id,
                &argument_typings,
            ),
            Typing::Undetermined(type_ids) => {
                let receiver_type_id = self.type_id_of_value_receiver(&node.operand);
                let overload_match = self.lookup_function_matching_positional_arguments(
                    &type_ids,
                    &argument_typings,
                    receiver_type_id,
                );
                let candidate = self.select_overload(&node.operand, overload_match);

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
        if let (Some(identifier), Some(definition_id)) = (
            reference_identifier_for_expression(operand),
            self.candidate_definition_id(candidate_type_id),
        ) {
            self.binder
                .fixup_reference(identifier.id(), Resolution::Definition(definition_id));
        }

        if let Some(operand_node_id) = operand.node_id() {
            self.binder
                .update_node_typing(operand_node_id, Typing::Resolved(candidate_type_id));
        }
    }

    /// Types a call whose operand is (or was disambiguated to) `type_id`: a
    /// function value is an actual call, a meta type is a cast (or a struct
    /// construction), and the user meta type of a function is a non-callable
    /// declaration reached through a contract/interface type name. Anything
    /// else, including a modifier and the alias of an imported file, is not
    /// callable at all.
    fn typing_of_type_called_with_positional_arguments(
        &mut self,
        node: &ir::FunctionCallExpression,
        type_id: TypeId,
        argument_typings: &[Typing],
    ) -> Typing {
        match self.types.get_type_by_id(type_id) {
            Type::Function(FunctionType {
                definition_id,
                return_type,
                ..
            }) => {
                let (definition_id, return_type) = (*definition_id, *return_type);
                if self.is_modifier_definition(definition_id) {
                    self.report_operand_not_callable(node);
                    Typing::Unresolved
                } else {
                    Typing::Resolved(return_type)
                }
            }
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
                    Some(Definition::Event(_)) => {
                        // TODO: an event invocation has to be prefixed by
                        // `emit`. The name *is* callable, so this is not a
                        // callability error. OTOH we don't have a `Type`
                        // variant for events to be able to type this yet. See
                        // SDR[950].
                        Typing::Unresolved
                    }
                    Some(Definition::Error(_)) => {
                        // TODO: An error construction has no value type of its own,
                        // except as an assertion parameter. Similar to the
                        // event case, we don't have a `Type` variant to
                        // represent the error yet. See SDR[1504]
                        Typing::Unresolved
                    }
                    Some(Definition::UserDefinedValueType(_)) => {
                        // TODO(validation) SDR[1698]: a UDVT is callable
                        // syntactically but not castable by name, so this is a
                        // disallowed conversion rather than a callability
                        // error.
                        Typing::Unresolved
                    }
                    Some(_) => {
                        // Report any other definitions as expression not
                        // callable. In practice only path imports can ever
                        // reach here, but it's a safe default.
                        self.report_operand_not_callable(node);
                        Typing::Unresolved
                    }
                    None => {
                        unreachable!("Invalid user meta type; not linking a definition");
                    }
                }
            }
            _ => {
                // The operand is a value of some other type, which is not
                // callable (eg. `1(2)`, or a mapping, which is indexed).
                self.report_operand_not_callable(node);
                Typing::Unresolved
            }
        }
    }

    pub(super) fn collect_named_argument_typings(
        &mut self,
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
        let operand_typing = self.typing_of_callee_expression(&node.operand);

        let (typing, definition_id) = match operand_typing {
            Typing::Unresolved => {
                // The callee failed to resolve, which is already reported: it
                // is only uncallable as a consequence.
                (Typing::Unresolved, None)
            }
            Typing::This(_) | Typing::Super => {
                // `this` and `super` name a contract instance, not a callable
                self.report_operand_not_callable(node);
                (Typing::Unresolved, None)
            }
            Typing::Resolved(type_id) => {
                self.typing_of_type_called_with_named_arguments(node, type_id)
            }
            Typing::Undetermined(type_ids) => {
                let receiver_type_id = self.type_id_of_value_receiver(&node.operand);
                let argument_typings = self.collect_named_argument_typings(arguments);
                let overload_match = self.lookup_function_matching_named_arguments(
                    &type_ids,
                    &argument_typings,
                    receiver_type_id,
                );
                let candidate = self.select_overload(&node.operand, overload_match);

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
    /// operand resolves to (if any). Only function values, struct constructions
    /// and error constructions are callable this way; the user meta types of a
    /// function and of an event are non-callable declarations, reached through
    /// a contract/interface type name and outside an `emit` statement
    /// respectively. Anything else, including a modifier and the alias of an
    /// imported file, is not callable at all.
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
            }) => {
                let (definition_id, return_type) = (*definition_id, *return_type);
                if self.is_modifier_definition(definition_id) {
                    self.report_operand_not_callable(node);
                    // The definition is still returned so the argument names
                    // resolve against the modifier's parameters.
                    (Typing::Unresolved, definition_id)
                } else {
                    (Typing::Resolved(return_type), definition_id)
                }
            }
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
                    Some(Definition::Error(_)) => {
                        // TODO: An error construction has no value type of its
                        // own, matching the positional form. Return the
                        // definition so the argument names resolve against its
                        // parameters.
                        (Typing::Unresolved, Some(definition_id))
                    }
                    Some(Definition::Event(_)) => {
                        // TODO: Likewise, an event invocation has no value type of
                        // its own; return the definition so the argument names
                        // resolve against its parameters.  See SDR[950]: an
                        // event invocation has to be prefixed by `emit`.
                        (Typing::Unresolved, Some(definition_id))
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
                    Some(_) => {
                        // Report any other definitions as expression not
                        // callable through named arguments. In practice only
                        // path imports can ever reach here, but it's a safe
                        // default.
                        self.report_operand_not_callable(node);
                        (Typing::Unresolved, None)
                    }
                    None => {
                        unreachable!("Invalid user meta type; not linking a definition");
                    }
                }
            }
            _ => {
                // The operand is a value of some other type, which is not
                // callable (eg. `3({a: 1})`).
                self.report_operand_not_callable(node);
                (Typing::Unresolved, None)
            }
        }
    }

    /// Reports the callee of a call as not callable.
    fn report_operand_not_callable(&mut self, node: &ir::FunctionCallExpression) {
        let (file_id, range) = node_location(node, self.file_node_mapper);
        self.diagnostics.push(file_id, range, ExpressionNotCallable);
    }

    /// Reports a built-in call that produced no result type, at the call site.
    /// The failures that carry no diagnostic are silent by design: either the
    /// check is not implemented yet, or what the call depends on already
    /// reported its own failure.
    fn report_built_in_call_error(
        &mut self,
        node: &ir::FunctionCallExpression,
        error: BuiltInCallError,
    ) {
        match error {
            BuiltInCallError::Diagnostic(kind) => self.push_diagnostic(node, kind),
            BuiltInCallError::NotReportedYet | BuiltInCallError::UnresolvedDependency => {}
        }
    }

    /// Whether `definition_id` is a modifier. A modifier has a function type so
    /// that its invocation can be checked against its parameters, but it is not
    /// callable from an expression.
    fn is_modifier_definition(&self, definition_id: Option<NodeId>) -> bool {
        definition_id
            .and_then(|definition_id| self.binder.find_definition_by_id(definition_id))
            .is_some_and(|definition| matches!(definition, Definition::Modifier(_)))
    }
}

// Given an expression node that resolves to a reference, return the identifier
// the reference was made through. If the expression cannot be traced back to a
// single reference, return `None`.
fn reference_identifier_for_expression(node: &ir::Expression) -> Option<&ir::Identifier> {
    match &node {
        ir::Expression::MemberAccessExpression(f) => Some(&f.member),
        ir::Expression::Identifier(f) => Some(f),
        ir::Expression::CallOptionsExpression(f) => reference_identifier_for_expression(&f.operand),
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
