use std::collections::HashMap;
use std::rc::Rc;

use semver::Version;

use super::p3_type_definitions::Output as Input;
use crate::backend::binder::{Binder, Reference, Resolution, ScopeId};
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::types::{Type, TypeId, TypeRegistry};
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

    fn resolve_symbol_in_scope(&self, scope_id: ScopeId, symbol: &str) -> Resolution {
        self.binder
            .resolve_single_in_scope_recursively(scope_id, symbol)
    }

    fn resolve_symbol_in_type(&self, type_id: TypeId, symbol: &str) -> Resolution {
        let type_ = self.types.get_type_by_id(type_id).unwrap();
        match type_ {
            Type::Address { .. } => todo!(),
            Type::Array { .. } => {
                todo!()
            }
            Type::Boolean => todo!(),
            Type::ByteArray { .. } => todo!(),
            Type::Bytes { .. } => todo!(),
            Type::Contract { .. } => todo!(),
            Type::Enum { .. } => todo!(),
            Type::Error { .. } => todo!(),
            Type::FixedPointNumber { .. } => todo!(),
            Type::Function { .. } => todo!(),
            Type::Integer { .. } => todo!(),
            Type::Interface { .. } => todo!(),
            Type::Mapping { .. } => todo!(),
            Type::Rational => todo!(),
            Type::String { .. } => todo!(),
            Type::Struct { definition_id, .. } => {
                let scope_id = self.binder.scope_id_for_node_id(*definition_id).unwrap();
                self.resolve_symbol_in_scope(scope_id, symbol)
            }
            Type::Tuple { .. } => todo!(),
            Type::UserDefinedValue { .. } => todo!(),
            Type::Void => Resolution::Unresolved,
        }
    }

    fn type_of_expression(&self, node: &input_ir::Expression) -> Option<TypeId> {
        match node {
            input_ir::Expression::AssignmentExpression(assignment_expression) => {
                self.binder.get_node_type(assignment_expression.node_id)
            }
            input_ir::Expression::ConditionalExpression(conditional_expression) => {
                self.binder.get_node_type(conditional_expression.node_id)
            }
            input_ir::Expression::OrExpression(_)
            | input_ir::Expression::AndExpression(_)
            | input_ir::Expression::EqualityExpression(_)
            | input_ir::Expression::InequalityExpression(_)
            | input_ir::Expression::TrueKeyword
            | input_ir::Expression::FalseKeyword => Some(self.types.boolean()),
            input_ir::Expression::BitwiseOrExpression(bitwise_or_expression) => {
                self.binder.get_node_type(bitwise_or_expression.node_id)
            }
            input_ir::Expression::BitwiseXorExpression(bitwise_xor_expression) => {
                self.binder.get_node_type(bitwise_xor_expression.node_id)
            }
            input_ir::Expression::BitwiseAndExpression(bitwise_and_expression) => {
                self.binder.get_node_type(bitwise_and_expression.node_id)
            }
            input_ir::Expression::ShiftExpression(shift_expression) => {
                self.binder.get_node_type(shift_expression.node_id)
            }
            input_ir::Expression::AdditiveExpression(additive_expression) => {
                self.binder.get_node_type(additive_expression.node_id)
            }
            input_ir::Expression::MultiplicativeExpression(multiplicative_expression) => {
                self.binder.get_node_type(multiplicative_expression.node_id)
            }
            input_ir::Expression::ExponentiationExpression(exponentiation_expression) => {
                self.binder.get_node_type(exponentiation_expression.node_id)
            }
            input_ir::Expression::PostfixExpression(postfix_expression) => {
                self.binder.get_node_type(postfix_expression.node_id)
            }
            input_ir::Expression::PrefixExpression(prefix_expression) => {
                self.binder.get_node_type(prefix_expression.node_id)
            }
            input_ir::Expression::FunctionCallExpression(_) => todo!(),
            input_ir::Expression::CallOptionsExpression(_) => todo!(),
            input_ir::Expression::MemberAccessExpression(member_access_expression) => {
                self.binder.get_node_type(member_access_expression.node_id)
            }
            input_ir::Expression::IndexAccessExpression(_) => todo!(),
            input_ir::Expression::NewExpression(_) => todo!(),
            input_ir::Expression::TupleExpression(tuple_expression) => {
                self.binder.get_node_type(tuple_expression.node_id)
            }
            input_ir::Expression::TypeExpression(_) => todo!(),
            input_ir::Expression::ArrayExpression(_) => todo!(),
            input_ir::Expression::HexNumberExpression(_)
            | input_ir::Expression::DecimalNumberExpression(_) => Some(self.types.rational()),
            input_ir::Expression::StringExpression(_) => Some(self.types.string()),
            input_ir::Expression::ElementaryType(_) => todo!(),
            input_ir::Expression::Identifier(identifier) => self.type_of_identifier(identifier),
            input_ir::Expression::PayableKeyword => todo!(),
            input_ir::Expression::ThisKeyword => todo!(),
            input_ir::Expression::SuperKeyword => todo!(),
        }
    }

    fn type_of_identifier(&self, identifier: &Rc<TerminalNode>) -> Option<TypeId> {
        // TODO: handle built-ins
        let definition_id = self
            .binder
            .find_reference_by_identifier_node_id(identifier.id())
            .unwrap()
            .resolution
            .as_definition_id()?;
        if !self.binder.node_has_type(definition_id) {
            return None;
        }
        self.binder.get_node_type(definition_id)
    }

    fn type_of_integer_binary_expression(
        &self,
        left_operand: &input_ir::Expression,
        right_operand: &input_ir::Expression,
    ) -> Option<TypeId> {
        let left_type_id = self.type_of_expression(left_operand);
        let right_type_id = self.type_of_expression(right_operand);
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
            result.push(self.type_of_expression(item.expression.as_ref()?)?);
        }
        Some(result)
    }

    fn type_of_resolution(&self, resolution: Resolution) -> Option<TypeId> {
        match resolution {
            Resolution::Unresolved => None,
            Resolution::BuiltIn(_) => todo!(),
            Resolution::Definition(definition_id) => {
                if self.binder.node_has_type(definition_id) {
                    self.binder.get_node_type(definition_id)
                } else {
                    None
                }
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
            let resolution = self.resolve_symbol_in_scope(scope_id, &identifier.unparse());
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
        true
    }

    fn leave_assignment_expression(&mut self, node: &input_ir::AssignmentExpression) {
        let type_id = self.type_of_expression(&node.left_operand);
        // TODO(validation): check that the type of right_operand can be applied
        // to the left by means of the operator
        self.binder.insert_node_type(node.node_id, type_id);
    }

    fn leave_conditional_expression(&mut self, node: &input_ir::ConditionalExpression) {
        let type_id = self.type_of_expression(&node.true_expression);
        // TODO(validation): both true_expression and false_expression should
        // have the same type
        self.binder.insert_node_type(node.node_id, type_id);
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
        self.binder.insert_node_type(node.node_id, type_id);
    }

    fn leave_bitwise_xor_expression(&mut self, node: &input_ir::BitwiseXorExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.insert_node_type(node.node_id, type_id);
    }

    fn leave_bitwise_and_expression(&mut self, node: &input_ir::BitwiseAndExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.insert_node_type(node.node_id, type_id);
    }

    fn leave_shift_expression(&mut self, node: &input_ir::ShiftExpression) {
        let type_id = self.type_of_expression(&node.left_operand);
        // TODO(validation): check that the left operand is an integer and the
        // right operand is an _unsigned_ integer
        self.binder.insert_node_type(node.node_id, type_id);
    }

    fn leave_additive_expression(&mut self, node: &input_ir::AdditiveExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.insert_node_type(node.node_id, type_id);
    }

    fn leave_multiplicative_expression(&mut self, node: &input_ir::MultiplicativeExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.insert_node_type(node.node_id, type_id);
    }

    fn leave_exponentiation_expression(&mut self, node: &input_ir::ExponentiationExpression) {
        let type_id = self.type_of_expression(&node.left_operand);
        // TODO(validation): check that the left operand is an integer and the
        // right operand is an _unsigned_ integer
        self.binder.insert_node_type(node.node_id, type_id);
    }

    fn leave_postfix_expression(&mut self, node: &input_ir::PostfixExpression) {
        // TODO(validation): check that the operand is an integer
        let type_id = self.type_of_expression(&node.operand);
        self.binder.insert_node_type(node.node_id, type_id);
    }

    fn leave_prefix_expression(&mut self, node: &input_ir::PrefixExpression) {
        let type_id = match node.operator.kind {
            TerminalKind::PlusPlus
            | TerminalKind::Plus
            | TerminalKind::MinusMinus
            | TerminalKind::Minus
            | TerminalKind::Tilde => {
                // TODO(validation): check that the operand is integer
                self.type_of_expression(&node.operand)
            }
            TerminalKind::Bang => {
                // TODO(validation): check that the operand is boolean
                Some(self.types.boolean())
            }
            TerminalKind::DeleteKeyword => Some(self.types.void()),
            _ => None,
        };
        self.binder.insert_node_type(node.node_id, type_id);
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
        self.binder.insert_node_type(node.node_id, type_id);
    }

    fn leave_member_access_expression(&mut self, node: &input_ir::MemberAccessExpression) {
        // we need to resolve the identifier at this point that we already have
        // typing information of the operand expression
        let parent_type_id = self.type_of_expression(&node.operand);
        let resolution = if let Some(parent_type_id) = parent_type_id {
            self.resolve_symbol_in_type(parent_type_id, &node.member.unparse())
        } else {
            Resolution::Unresolved
        };

        let reference = Reference::new(Rc::clone(&node.member), resolution);
        self.binder.insert_reference(reference);

        let type_id = self.type_of_resolution(resolution);
        self.binder.insert_node_type(node.node_id, type_id);
    }

    fn enter_yul_path(&mut self, items: &input_ir::YulPath) -> bool {
        // resolve the first item in the path
        if let Some(identifier) = items.first() {
            let scope_id = self.current_scope_id();
            let resolution = self.resolve_symbol_in_scope(scope_id, &identifier.unparse());
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }

        // TODO: resolve the rest of the items

        false
    }
}
