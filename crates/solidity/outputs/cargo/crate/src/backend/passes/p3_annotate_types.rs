use std::collections::HashMap;
use std::rc::Rc;

use metaslang_bindings::BuiltInLocation;
use metaslang_cst::nodes::NodeId;
use num_traits::FromPrimitive;

use super::p2_collect_types::Output as Input;
use crate::backend::built_ins::built_in_type;
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{
    self as input_ir, ArgumentsDeclaration, Expression, SourceUnit,
};
use crate::backend::types::{Type, TypeId, TypeRegistry};
use crate::bindings::{BindingGraph, BindingLocation, Definition, UserFileLocation};
use crate::cst::{TerminalKind, TerminalNode};
use crate::extensions::built_ins::BuiltInTag;

pub struct Output {
    pub files: HashMap<String, SourceUnit>,
    pub binding_graph: Rc<BindingGraph>,
    pub types: TypeRegistry,
    pub annotations: HashMap<NodeId, TypeId>,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let definition_types = input.definition_types;
    let mut pass = Pass::new(
        Rc::clone(&input.binding_graph),
        input.types,
        definition_types,
    );
    for source_unit in files.values() {
        input_ir::visitor::accept_source_unit(source_unit, &mut pass);
    }

    let annotations = pass.annotations;
    let types = pass.types;

    Output {
        files,
        binding_graph: Rc::clone(&input.binding_graph),
        types,
        annotations,
    }
}

struct Pass {
    binding_graph: Rc<BindingGraph>,
    types: TypeRegistry,
    pub annotations: HashMap<NodeId, TypeId>,
}

impl Pass {
    pub fn new(
        binding_graph: Rc<BindingGraph>,
        types: TypeRegistry,
        definition_types: HashMap<NodeId, TypeId>,
    ) -> Self {
        Self {
            binding_graph,
            types,
            annotations: definition_types,
        }
    }

    fn type_of_expression(&mut self, expression: &Expression) -> TypeId {
        match expression {
            Expression::StringExpression(_) => self.types.string(),
            Expression::ElementaryType(_) => todo!(),
            Expression::Identifier(identifier) => self.type_of_identifier(identifier),
            Expression::PayableKeyword => todo!(),
            Expression::ThisKeyword => todo!(),
            Expression::SuperKeyword => todo!(),
            Expression::TrueKeyword | Expression::FalseKeyword => self.types.bool(),
            _ => {
                let node_id = match expression {
                    Expression::AssignmentExpression(assignment_expression) => {
                        assignment_expression.node_id
                    }
                    Expression::ConditionalExpression(conditional_expression) => {
                        conditional_expression.node_id
                    }
                    Expression::OrExpression(or_expression) => or_expression.node_id,
                    Expression::AndExpression(and_expression) => and_expression.node_id,
                    Expression::EqualityExpression(equality_expression) => {
                        equality_expression.node_id
                    }
                    Expression::InequalityExpression(inequality_expression) => {
                        inequality_expression.node_id
                    }
                    Expression::BitwiseOrExpression(bitwise_or_expression) => {
                        bitwise_or_expression.node_id
                    }
                    Expression::BitwiseXorExpression(bitwise_xor_expression) => {
                        bitwise_xor_expression.node_id
                    }
                    Expression::BitwiseAndExpression(bitwise_and_expression) => {
                        bitwise_and_expression.node_id
                    }
                    Expression::ShiftExpression(shift_expression) => shift_expression.node_id,
                    Expression::AdditiveExpression(additive_expression) => {
                        additive_expression.node_id
                    }
                    Expression::MultiplicativeExpression(multiplicative_expression) => {
                        multiplicative_expression.node_id
                    }
                    Expression::ExponentiationExpression(exponetiation_expression) => {
                        exponetiation_expression.node_id
                    }
                    Expression::PostfixExpression(postfix_expression) => postfix_expression.node_id,
                    Expression::PrefixExpression(prefix_expression) => prefix_expression.node_id,
                    Expression::FunctionCallExpression(function_call_expression) => {
                        function_call_expression.node_id
                    }
                    Expression::CallOptionsExpression(call_options_expression) => {
                        call_options_expression.node_id
                    }
                    Expression::MemberAccessExpression(member_access_expression) => {
                        member_access_expression.node_id
                    }
                    Expression::IndexAccessExpression(index_access_expression) => {
                        index_access_expression.node_id
                    }
                    Expression::NewExpression(new_expression) => new_expression.node_id,
                    Expression::TupleExpression(tuple_expression) => tuple_expression.node_id,
                    Expression::TypeExpression(type_expression) => type_expression.node_id,
                    Expression::ArrayExpression(array_expression) => array_expression.node_id,
                    Expression::HexNumberExpression(hex_number_expression) => {
                        hex_number_expression.node_id
                    }
                    Expression::DecimalNumberExpression(decimal_number_expression) => {
                        decimal_number_expression.node_id
                    }
                    _ => unreachable!(),
                };
                self.annotations.get(&node_id).copied().unwrap_or_else(|| {
                    unimplemented!("Expected index access expression to already be typed")
                })
            }
        }
    }

    fn type_of_identifier(&mut self, identifier: &Rc<TerminalNode>) -> TypeId {
        let Some(reference) = self.binding_graph.reference_by_node_id(identifier.id()) else {
            // This indicates an error in the binding rules
            unreachable!("Missing reference for identifier {identifier:?} in expression");
        };
        let definitions = reference.definitions();
        match definitions.len() {
            0 => {
                unimplemented!("Reference to {identifier:?} cannot be resolved");
            }
            1 => self.type_of_definition(&definitions[0]),
            _ => {
                let choices = definitions
                    .iter()
                    .map(|definition| self.type_of_definition(definition))
                    .collect();
                self.types.register_type(Type::Undecided { choices })
            }
        }
    }

    fn type_of_definition(&mut self, definition: &Definition) -> TypeId {
        match definition.definiens_location() {
            BindingLocation::UserFile(UserFileLocation { .. }) => self
                .annotations
                .get(&definition.id())
                .copied()
                .unwrap_or_else(|| {
                    let identifier = definition.get_cursor().node().unparse();
                    unimplemented!("Type for definition of {identifier:?} is not registered")
                }),
            BindingLocation::BuiltIn(BuiltInLocation {}) => {
                let tag = definition
                    .get_built_in_tag()
                    .expect("Missing built-in tag for definition {definition:?}");
                let tag = BuiltInTag::from_u32(tag).expect("Invalid built-in tag {tag}");
                built_in_type(tag, &mut self.types)
            }
        }
    }

    fn annotate_logical_expression(
        &mut self,
        node_id: NodeId,
        left_operand: &Expression,
        right_operand: &Expression,
    ) {
        if self.type_of_expression(left_operand) == self.types.bool()
            && self.type_of_expression(right_operand) == self.types.bool()
        {
            self.annotations.insert(node_id, self.types.bool());
        } else {
            unimplemented!("Type error: logical expression operands are not boolean");
        }
    }

    fn annotate_bitwise_expression(
        &mut self,
        node_id: NodeId,
        left_operand: &Expression,
        right_operand: &Expression,
    ) {
        let left_type = self.type_of_expression(left_operand);
        let right_type = self.type_of_expression(right_operand);
        if self.types.implicitly_convertible_to(right_type, left_type) {
            self.annotations.insert(node_id, left_type);
        } else if self.types.implicitly_convertible_to(left_type, right_type) {
            self.annotations.insert(node_id, right_type);
        } else {
            unimplemented!("operands type mismatch in bitwise expression");
        }
    }

    fn annotate_arithmetic_expression(
        &mut self,
        node_id: NodeId,
        left_operand: &Expression,
        right_operand: &Expression,
    ) {
        let left_type = self.type_of_expression(left_operand);
        let right_type = self.type_of_expression(right_operand);
        if self.types.implicitly_convertible_to(right_type, left_type) {
            self.annotations.insert(node_id, left_type);
        } else if self.types.implicitly_convertible_to(left_type, right_type) {
            self.annotations.insert(node_id, right_type);
        } else {
            unimplemented!("operands type mismatch in arithmetic expression");
        }
    }

    fn resolve_overload(&self, choices: &[TypeId], argument_types: &[TypeId]) -> TypeId {
        for type_id in choices {
            let Type::Function {
                parameter_types, ..
            } = self.types.get_type_by_id(*type_id).unwrap()
            else {
                unimplemented!("Type error: overload choice is not a function");
            };
            if parameter_types
                .iter()
                .zip(argument_types)
                .all(|(parameter, argument)| parameter == argument)
            {
                return *type_id;
            }
        }

        unimplemented!("Type error: failed to resolve overloaded function");
    }
}

impl Visitor for Pass {
    fn leave_assignment_expression(&mut self, node: &input_ir::AssignmentExpression) {
        let assignment_type = self.type_of_expression(&node.left_operand);
        self.annotations.insert(node.node_id, assignment_type);
    }

    fn leave_conditional_expression(&mut self, node: &input_ir::ConditionalExpression) {
        let true_type = self.type_of_expression(&node.true_expression);
        let false_type = self.type_of_expression(&node.false_expression);
        if true_type != false_type {
            unimplemented!("type mismatch between true and false expressions in conditional");
        }
        self.annotations.insert(node.node_id, true_type);
    }

    fn leave_or_expression(&mut self, node: &input_ir::OrExpression) {
        self.annotate_logical_expression(node.node_id, &node.left_operand, &node.right_operand);
    }

    fn leave_and_expression(&mut self, node: &input_ir::AndExpression) {
        self.annotate_logical_expression(node.node_id, &node.left_operand, &node.right_operand);
    }

    fn leave_equality_expression(&mut self, node: &input_ir::EqualityExpression) {
        self.annotations.insert(node.node_id, self.types.bool());
    }

    fn leave_inequality_expression(&mut self, node: &input_ir::InequalityExpression) {
        self.annotations.insert(node.node_id, self.types.bool());
    }

    fn leave_bitwise_or_expression(&mut self, node: &input_ir::BitwiseOrExpression) {
        self.annotate_bitwise_expression(node.node_id, &node.left_operand, &node.right_operand);
    }

    fn leave_bitwise_xor_expression(&mut self, node: &input_ir::BitwiseXorExpression) {
        self.annotate_bitwise_expression(node.node_id, &node.left_operand, &node.right_operand);
    }

    fn leave_bitwise_and_expression(&mut self, node: &input_ir::BitwiseAndExpression) {
        self.annotate_bitwise_expression(node.node_id, &node.left_operand, &node.right_operand);
    }

    fn leave_shift_expression(&mut self, node: &input_ir::ShiftExpression) {
        let left_type_id = self.type_of_expression(&node.left_operand);
        self.annotations.insert(node.node_id, left_type_id);
    }

    fn leave_additive_expression(&mut self, node: &input_ir::AdditiveExpression) {
        self.annotate_arithmetic_expression(node.node_id, &node.left_operand, &node.right_operand);
    }

    fn leave_multiplicative_expression(&mut self, node: &input_ir::MultiplicativeExpression) {
        self.annotate_arithmetic_expression(node.node_id, &node.left_operand, &node.right_operand);
    }

    fn leave_exponentiation_expression(&mut self, node: &input_ir::ExponentiationExpression) {
        let left_type_id = self.type_of_expression(&node.left_operand);
        self.annotations.insert(node.node_id, left_type_id);
    }

    fn leave_postfix_expression(&mut self, node: &input_ir::PostfixExpression) {
        let operand_type_id = self.type_of_expression(&node.operand);
        self.annotations.insert(node.node_id, operand_type_id);
    }

    fn leave_prefix_expression(&mut self, node: &input_ir::PrefixExpression) {
        match node.operator.kind {
            TerminalKind::PlusPlus | TerminalKind::MinusMinus | TerminalKind::Tilde => {
                // TODO: should validate that the type is an integer
                let operand_type_id = self.type_of_expression(&node.operand);
                self.annotations.insert(node.node_id, operand_type_id);
            }
            TerminalKind::Bang => {
                let operand_type_id = self.type_of_expression(&node.operand);
                if operand_type_id != self.types.bool() {
                    unimplemented!("logical negation only operates on booleans");
                }
                self.annotations.insert(node.node_id, operand_type_id);
            }
            TerminalKind::DeleteKeyword => {
                self.annotations.insert(node.node_id, self.types.void());
            }
            _ => unreachable!("invalid operator in prefix expression"),
        }
    }

    fn leave_function_call_expression(&mut self, node: &input_ir::FunctionCallExpression) {
        let function_type_id = self.type_of_expression(&node.operand);
        let function_type = self.types.get_type_by_id(function_type_id).unwrap();
        match function_type {
            Type::Function { return_type, .. } => {
                self.annotations.insert(node.node_id, *return_type);
            }
            Type::Undecided { choices } => {
                // clone here to release the borrow on self
                let choices = choices.clone();
                match &node.arguments {
                    ArgumentsDeclaration::PositionalArgumentsDeclaration(positional_arguments) => {
                        let argument_types = positional_arguments
                            .arguments
                            .iter()
                            .map(|argument| self.type_of_expression(argument))
                            .collect::<Vec<_>>();
                        let overload_type_id = self.resolve_overload(&choices, &argument_types);
                        let Type::Function { return_type, .. } =
                            self.types.get_type_by_id(overload_type_id).unwrap()
                        else {
                            unimplemented!("Type error: resolved overload is not a function");
                        };
                        self.annotations.insert(node.node_id, *return_type);
                    }
                    ArgumentsDeclaration::NamedArgumentsDeclaration(_) => {
                        unimplemented!(
                            "Type error: overload resolution not supported with named arguments"
                        );
                    }
                }
            }
            _ => {
                unimplemented!("Type error: function call operand is not a function");
            }
        }
    }

    fn leave_call_options_expression(&mut self, _node: &input_ir::CallOptionsExpression) {
        todo!()
    }

    fn leave_member_access_expression(&mut self, node: &input_ir::MemberAccessExpression) {
        let member_type = self.type_of_identifier(&node.member);
        self.annotations.insert(node.node_id, member_type);
    }

    fn leave_index_access_expression(&mut self, node: &input_ir::IndexAccessExpression) {
        let operand_type_id = self.type_of_expression(&node.operand);
        let operand_type = &self.types.get_type_by_id(operand_type_id).unwrap();
        match operand_type {
            Type::Array { element_type, .. } => {
                self.annotations.insert(node.node_id, *element_type);
            }
            Type::Mapping { value_type_id, .. } => {
                self.annotations.insert(node.node_id, *value_type_id);
            }
            _ => {
                unimplemented!("Type error: Index access to {operand_type:?} not supported");
            }
        }
    }

    fn leave_new_expression(&mut self, _node: &input_ir::NewExpression) {
        todo!()
    }

    fn leave_tuple_expression(&mut self, _node: &input_ir::TupleExpression) {
        todo!()
    }

    fn leave_type_expression(&mut self, _node: &input_ir::TypeExpression) {
        todo!()
    }

    fn leave_array_expression(&mut self, _node: &input_ir::ArrayExpression) {
        todo!()
    }

    fn leave_hex_number_expression(&mut self, _node: &input_ir::HexNumberExpression) {
        todo!()
    }

    fn leave_decimal_number_expression(&mut self, node: &input_ir::DecimalNumberExpression) {
        self.annotations.insert(node.node_id, self.types.rational());
    }
}
