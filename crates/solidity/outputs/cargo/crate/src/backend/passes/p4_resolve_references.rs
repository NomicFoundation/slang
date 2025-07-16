use std::collections::HashMap;
use std::rc::Rc;

use semver::Version;

use super::p3_type_definitions::Output as Input;
use crate::backend::binder::{
    Binder, BuiltIn, Definition, Reference, Resolution, ScopeId, Typing, UsingDirective,
};
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::types::{DataLocation, Type, TypeId, TypeRegistry};
use crate::compilation::CompilationUnit;
use crate::cst::{NodeId, TerminalKind, TerminalNode};

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
    pub types: TypeRegistry,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let compilation_unit = input.compilation_unit;
    let mut pass = Pass::new(
        input.binder,
        input.types,
        compilation_unit.language_version(),
    );
    for source_unit in files.values() {
        pass.visit_file(source_unit);
    }
    let binder = pass.binder;
    let types = pass.types;

    Output {
        compilation_unit,
        files,
        binder,
        types,
    }
}

const VERSION_0_5_0: Version = Version::new(0, 5, 0);

struct Pass {
    language_version: Version,
    scope_stack: Vec<ScopeId>,
    binder: Binder,
    types: TypeRegistry,
}

impl Pass {
    fn new(binder: Binder, types: TypeRegistry, language_version: &Version) -> Self {
        Self {
            language_version: language_version.clone(),
            scope_stack: Vec::new(),
            binder,
            types,
        }
    }

    fn visit_file(&mut self, source_unit: &input_ir::SourceUnit) {
        assert!(self.scope_stack.is_empty());
        input_ir::visitor::accept_source_unit(source_unit, self);
        assert!(self.scope_stack.is_empty());
    }

    fn enter_scope_for_node_id(&mut self, node_id: NodeId) {
        let scope_id = self.binder.scope_id_for_node_id(node_id).unwrap();
        self.scope_stack.push(scope_id);
    }

    fn leave_scope_for_node_id(&mut self, node_id: NodeId) {
        let Some(current_scope_id) = self.scope_stack.pop() else {
            unreachable!("attempt to pop an empty scope stack");
        };
        assert_eq!(
            current_scope_id,
            self.binder.scope_id_for_node_id(node_id).unwrap()
        );
    }

    fn current_scope_id(&self) -> ScopeId {
        self.scope_stack.last().copied().unwrap()
    }

    fn lookup_global_built_in(symbol: &str) -> Option<BuiltIn> {
        match symbol {
            "tx" => Some(BuiltIn::Tx),
            // TODO: add the rest of the built-ins
            _ => None,
        }
    }

    // This is a "top-level" (ie. not a member access) resolution method
    fn resolve_symbol_in_scope(&self, scope_id: ScopeId, symbol: &str) -> Resolution {
        // TODO: we need to do hierarchy lookups for contracts and interfaces if
        // we're in the scope of a contract/interface/library. This we probably
        // cannot delegate to the binder.
        let resolution = self.binder.resolve_in_scope(scope_id, symbol);
        if resolution == Resolution::Unresolved {
            if let Some(built_in) = Self::lookup_global_built_in(symbol) {
                Resolution::BuiltIn(built_in)
            } else {
                Resolution::Unresolved
            }
        } else {
            resolution
        }
    }

    fn active_using_directives_for_type(&self, type_id: TypeId) -> Vec<&UsingDirective> {
        // If the type is a reference type, we need to compute the id of the
        // location independent type (using DataLocation::Inherited). If it
        // doesn't exist, proceed with the given value, but we won't find any
        // type-specific directives, only those applicable to all types (ie.
        // `using L for *`)
        let location_independent_type = self
            .types
            .get_type_by_id(type_id)
            .unwrap()
            .with_location(DataLocation::Inherited);
        let type_id = self
            .types
            .find_type(&location_independent_type)
            .unwrap_or(type_id);

        let mut directives = Vec::new();
        for scope_id in self.scope_stack.iter().rev() {
            let scope = self.binder.get_scope_by_id(*scope_id);
            let scope_directives = scope.get_using_directives();
            directives.extend(
                scope_directives
                    .iter()
                    .filter(|directive| directive.applies_to(type_id)),
            );
        }
        let global_directives = self.binder.get_global_using_directives();
        directives.extend(
            global_directives
                .iter()
                .filter(|directive| directive.applies_to(type_id)),
        );
        directives
    }

    fn resolve_symbol_in_typing(&self, typing: &Typing, symbol: &str) -> Resolution {
        match typing {
            Typing::Unresolved | Typing::Undetermined(_) => Resolution::Unresolved,
            Typing::Resolved(type_id) => {
                let resolution = self.resolve_symbol_in_type(*type_id, symbol);
                if resolution == Resolution::Unresolved {
                    // Consider active `using` directives in the current context
                    let active_directives = self.active_using_directives_for_type(*type_id);
                    let mut definition_ids = Vec::new();
                    for directive in &active_directives {
                        let scope_id = directive.get_scope_id();
                        let ids = self
                            .binder
                            .resolve_in_scope_as_namespace(scope_id, symbol)
                            .get_definition_ids();
                        // TODO: filter the resolved definitions to only include
                        // functions whose first parameter is of our type (or
                        // implicitly convertible to it)
                        definition_ids.extend(ids);
                    }
                    Resolution::from(definition_ids)
                } else {
                    resolution
                }
            }
            Typing::This | Typing::Super => {
                // TODO: restrict lookup to the scope of the enclosing contract,
                // but extend the search to the bases contracts/interfaces using
                // the linearisation information
                Resolution::Unresolved
            }
            Typing::MetaType(node_id) => {
                // We're trying to resolve a member access expression into a
                // type name, ie. this is a meta-type member access
                let Some(definition) = self.binder.find_definition_by_id(*node_id) else {
                    return Resolution::Unresolved;
                };
                match definition {
                    Definition::Error(_) => Resolution::BuiltIn(BuiltIn::ErrorType),
                    Definition::Event(_) => Resolution::BuiltIn(BuiltIn::EventType),
                    Definition::Import(import_definition) => {
                        if let Some(scope_id) = import_definition
                            .resolved_file_id
                            .as_ref()
                            .and_then(|file_id| self.binder.scope_id_for_file_id(file_id))
                        {
                            self.binder.resolve_in_scope(scope_id, symbol)
                        } else {
                            Resolution::Unresolved
                        }
                    }
                    Definition::Contract(_)
                    | Definition::Enum(_)
                    | Definition::Interface(_)
                    | Definition::Library(_) => {
                        // this is a "namespace" lookup
                        if let Some(scope_id) = self.binder.scope_id_for_node_id(*node_id) {
                            self.binder.resolve_in_scope_as_namespace(scope_id, symbol)
                        } else {
                            Resolution::Unresolved
                        }
                    }
                    _ => Resolution::Unresolved,
                }
            }
            Typing::BuiltIn(built_in) => match built_in {
                BuiltIn::ErrorType => match symbol {
                    "selector" => Resolution::BuiltIn(BuiltIn::Selector),
                    _ => Resolution::Unresolved,
                },
                BuiltIn::EventType => match symbol {
                    "selector" => Resolution::BuiltIn(BuiltIn::Selector),
                    _ => Resolution::Unresolved,
                },
                BuiltIn::Tx => match symbol {
                    "gasprice" => Resolution::BuiltIn(BuiltIn::TxGasPrice),
                    "origin" => Resolution::BuiltIn(BuiltIn::TxOrigin),
                    _ => Resolution::Unresolved,
                },
                _ => Resolution::Unresolved,
            },
        }
    }

    fn resolve_symbol_in_type(&self, type_id: TypeId, symbol: &str) -> Resolution {
        let type_ = self.types.get_type_by_id(type_id).unwrap();
        match type_ {
            Type::Address { .. } => match symbol {
                "balance" => Resolution::BuiltIn(BuiltIn::Balance),
                // TODO: add the rest of the address (payable) built-ins
                _ => Resolution::Unresolved,
            },
            Type::Array { element_type, .. } => match symbol {
                "length" => Resolution::BuiltIn(BuiltIn::Length),
                "push" => Resolution::BuiltIn(BuiltIn::ArrayPush(*element_type)),
                _ => Resolution::Unresolved,
            },
            Type::Boolean => Resolution::Unresolved,
            Type::ByteArray { .. } => match symbol {
                "length" => Resolution::BuiltIn(BuiltIn::Length),
                _ => Resolution::Unresolved,
            },
            Type::Bytes { .. } => match symbol {
                "push" => Resolution::BuiltIn(BuiltIn::ArrayPush(self.types.byte())),
                "pop" => Resolution::BuiltIn(BuiltIn::ArrayPop),
                _ => Resolution::Unresolved,
            },
            Type::Contract { .. } => {
                // TODO: expose the public contract methods
                Resolution::Unresolved
            }
            Type::Enum { .. } => Resolution::Unresolved,
            Type::FixedPointNumber { .. } => Resolution::Unresolved,
            Type::Function { .. } => {
                // TODO: for external functions, expose `selector`, `address`, etc
                Resolution::Unresolved
            }
            Type::Integer { .. } => Resolution::Unresolved,
            Type::Interface { .. } => {
                // TODO: expose the public interface methods
                Resolution::Unresolved
            }
            Type::Mapping { .. } => Resolution::Unresolved,
            Type::Rational => {
                // TODO: we should probably force cast the rational number to
                // the tighest integer type that can represent it
                Resolution::Unresolved
            }
            Type::String { .. } => match symbol {
                "length" => Resolution::BuiltIn(BuiltIn::Length),
                _ => Resolution::Unresolved,
            },
            Type::Struct { definition_id, .. } => {
                let scope_id = self.binder.scope_id_for_node_id(*definition_id).unwrap();
                self.binder.resolve_in_scope_as_namespace(scope_id, symbol)
            }
            Type::Tuple { .. } => Resolution::Unresolved,
            Type::UserDefinedValue { .. } => Resolution::Unresolved,
            Type::Void => Resolution::Unresolved,
        }
    }

    fn typing_of_expression(&self, node: &input_ir::Expression) -> Typing {
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
            input_ir::Expression::HexNumberExpression(_)
            | input_ir::Expression::DecimalNumberExpression(_) => {
                Typing::Resolved(self.types.rational())
            }
            input_ir::Expression::StringExpression(_) => Typing::Resolved(self.types.string()),
            input_ir::Expression::ElementaryType(_) => {
                // TODO: We need to support two things here:
                // - bytes/string for the `concat` member
                // - all types for casting
                Typing::Unresolved
            }
            input_ir::Expression::Identifier(identifier) => self.typing_of_identifier(identifier),
            input_ir::Expression::PayableKeyword => {
                // TODO: we need to support casting here
                Typing::Unresolved
            }
            input_ir::Expression::ThisKeyword => Typing::This,
            input_ir::Expression::SuperKeyword => Typing::Super,
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

    fn type_of_integer_binary_expression(
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

    fn types_of_tuple(&self, items: &[input_ir::TupleValue]) -> Option<Vec<TypeId>> {
        let mut result = Vec::new();
        for item in items {
            let expression = item.expression.as_ref()?;
            let type_id = self.typing_of_expression(expression).as_type_id()?;
            result.push(type_id);
        }
        Some(result)
    }

    fn typing_of_resolution(&self, resolution: &Resolution) -> Typing {
        match resolution {
            Resolution::Unresolved => Typing::Unresolved,
            Resolution::BuiltIn(built_in) => match built_in {
                BuiltIn::Balance => Typing::Resolved(self.types.uint256()),
                BuiltIn::Length => Typing::Resolved(self.types.uint256()),
                BuiltIn::Selector => Typing::Resolved(self.types.bytes4()),
                BuiltIn::TxGasPrice => Typing::Resolved(self.types.uint256()),
                BuiltIn::TxOrigin => Typing::Resolved(self.types.address()),
                BuiltIn::ArrayPop
                | BuiltIn::ArrayPush(_)
                | BuiltIn::ErrorType
                | BuiltIn::EventType
                | BuiltIn::Tx => Typing::BuiltIn(*built_in),
            },
            Resolution::Definition(definition_id) => self.binder.node_typing(*definition_id),
            Resolution::Ambiguous(definitions) => {
                // TODO: we need some sort of link between the definition_id and the resulting type
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
}

impl Visitor for Pass {
    fn enter_source_unit(&mut self, node: &input_ir::SourceUnit) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_source_unit(&mut self, node: &input_ir::SourceUnit) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_contract_definition(&mut self, node: &input_ir::ContractDefinition) {
        self.leave_scope_for_node_id(node.node_id);
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
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_function_definition(&mut self, node: &input_ir::FunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_modifier_definition(&mut self, node: &input_ir::ModifierDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_modifier_definition(&mut self, node: &input_ir::ModifierDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_constructor_definition(&mut self, node: &input_ir::ConstructorDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_constructor_definition(&mut self, node: &input_ir::ConstructorDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_fallback_function_definition(
        &mut self,
        node: &input_ir::FallbackFunctionDefinition,
    ) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_fallback_function_definition(&mut self, node: &input_ir::FallbackFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_receive_function_definition(
        &mut self,
        node: &input_ir::ReceiveFunctionDefinition,
    ) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_receive_function_definition(&mut self, node: &input_ir::ReceiveFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_unnamed_function_definition(
        &mut self,
        node: &input_ir::UnnamedFunctionDefinition,
    ) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_unnamed_function_definition(&mut self, node: &input_ir::UnnamedFunctionDefinition) {
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
            // TODO: we need to use a multi resolution method here, to be able
            // to select function overloads
            // FIXME: we need the `Resolution` to be able to represent the
            // ambiguity, which then gets transformed into
            // `Typing::Undetermined`
            let resolution = self.resolve_symbol_in_scope(scope_id, &identifier.unparse());
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
        true
    }

    fn leave_assignment_expression(&mut self, node: &input_ir::AssignmentExpression) {
        let type_id = self.typing_of_expression(&node.left_operand).as_type_id();
        // TODO(validation): check that the type of right_operand can be applied
        // to the left by means of the operator
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_conditional_expression(&mut self, node: &input_ir::ConditionalExpression) {
        let type_id = self
            .typing_of_expression(&node.true_expression)
            .as_type_id();
        // TODO(validation): both true_expression and false_expression should
        // have the same type
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
        let type_id = self.typing_of_expression(&node.left_operand).as_type_id();
        // TODO(validation): check that the left operand is an integer and the
        // right operand is an _unsigned_ integer
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
        let types = self.types_of_tuple(&node.items);
        let type_id = types.map(|types| {
            if types.len() == 1 {
                types.first().copied().unwrap()
            } else {
                self.types.register_type(Type::Tuple { types })
            }
        });
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_member_access_expression(&mut self, node: &input_ir::MemberAccessExpression) {
        // we need to resolve the identifier at this point that we already have
        // typing information of the operand expression
        let operand_typing = self.typing_of_expression(&node.operand);
        let resolution = self.resolve_symbol_in_typing(&operand_typing, &node.member.unparse());

        let typing = self.typing_of_resolution(&resolution);
        self.binder.set_node_typing(node.node_id, typing);

        let reference = Reference::new(Rc::clone(&node.member), resolution);
        self.binder.insert_reference(reference);
    }

    fn leave_index_access_expression(&mut self, node: &input_ir::IndexAccessExpression) {
        let typing =
            if let Some(operand_type_id) = self.typing_of_expression(&node.operand).as_type_id() {
                let operand_type = self.types.get_type_by_id(operand_type_id);
                match operand_type {
                    Some(Type::Array { element_type, .. }) => Typing::Resolved(*element_type),
                    Some(Type::Mapping { value_type_id, .. }) => Typing::Resolved(*value_type_id),
                    _ => {
                        // TODO(validation): the operand is not indexable
                        Typing::Unresolved
                    }
                }
            } else {
                Typing::Unresolved
            };
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_array_expression(&mut self, node: &input_ir::ArrayExpression) {
        let typing = if node.items.is_empty() {
            Typing::Unresolved
        } else {
            self.typing_of_expression(node.items.first().unwrap())
        };
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_function_call_expression(&mut self, node: &input_ir::FunctionCallExpression) {
        let operand_typing = self.typing_of_expression(&node.operand);
        let typing =
            match operand_typing {
                Typing::Unresolved | Typing::This | Typing::Super => Typing::Unresolved,
                Typing::Resolved(type_id) => {
                    let operand_type = self.types.get_type_by_id(type_id).unwrap();
                    match operand_type {
                        Type::Function { return_type, .. } => Typing::Resolved(*return_type),
                        // TODO: support other types that can be called
                        _ => {
                            // TODO(validation): the operand did not resolve to a function
                            Typing::Unresolved
                        }
                    }
                }
                Typing::Undetermined(_type_ids) => {
                    // TODO: resolve argument types and match best overload
                    // TODO: maybe update the typing of the operand?
                    Typing::Unresolved
                }
                Typing::MetaType(node_id) => {
                    if let Some(definition) = self.binder.find_definition_by_id(node_id) {
                        match definition {
                            Definition::Contract(_) => {
                                // TODO: we may want to use to follow up on new() expression
                                Typing::Unresolved
                            }
                            Definition::Struct(_) => {
                                // struct constructor
                                Typing::Resolved(self.types.register_type(Type::Struct {
                                    definition_id: node_id,
                                    location: DataLocation::Memory,
                                }))
                            }
                            _ => Typing::Unresolved,
                        }
                    } else {
                        Typing::Unresolved
                    }
                }
                Typing::BuiltIn(built_in) => {
                    match built_in {
                        BuiltIn::ArrayPush(type_id) => match &node.arguments {
                            input_ir::ArgumentsDeclaration::NamedArgumentsDeclaration(named)
                                if named.arguments.is_none()
                                    || named
                                        .arguments
                                        .as_ref()
                                        .is_some_and(|args| args.arguments.is_empty()) =>
                            {
                                Typing::Resolved(type_id)
                            }
                            input_ir::ArgumentsDeclaration::PositionalArgumentsDeclaration(
                                args,
                            ) if args.arguments.is_empty() => Typing::Resolved(type_id),
                            _ => Typing::Resolved(self.types.void()),
                        },
                        BuiltIn::ArrayPop => Typing::Resolved(self.types.void()),
                        _ => {
                            // none of these built-ins can be called
                            Typing::Unresolved
                        }
                    }
                }
            };
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_new_expression(&mut self, node: &input_ir::NewExpression) {
        // TODO: this should type to the constructor signature of the given type
        // name to be able to type a function call expression
        let typing = Typing::Unresolved;
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_call_options_expression(&mut self, node: &input_ir::CallOptionsExpression) {
        let typing = self.typing_of_expression(&node.operand);
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_type_expression(&mut self, node: &input_ir::TypeExpression) {
        // FIXME: this probably needs a special value to represent the
        // meta-type
        let typing = Typing::Unresolved;
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn enter_yul_path(&mut self, items: &input_ir::YulPath) -> bool {
        // TODO: on old Solidity versions, handle the `_offset` and `_slot` suffixes
        // resolve the first item in the path
        if let Some(identifier) = items.first() {
            let scope_id = self.current_scope_id();
            let resolution = self.resolve_symbol_in_scope(scope_id, &identifier.unparse());
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }

        // TODO: resolve the rest of the items
        // TODO: introduce special handling for the `.offset`, `.slot` and
        // `.selector` members

        false
    }
}
