use std::collections::HashMap;
use std::rc::Rc;

use metaslang_bindings::BuiltInLocation;
use metaslang_cst::nodes::NodeId;
use num_traits::FromPrimitive;

use super::p2_collect_types::Output as Input;
use crate::backend::built_ins::built_in_type;
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir, Expression, SourceUnit};
use crate::backend::types::{Type, TypeId, TypeRegistry};
use crate::bindings::{BindingGraph, BindingLocation, Definition, UserFileLocation};
use crate::cst::TerminalNode;
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
            Expression::AssignmentExpression(_) => todo!(),
            Expression::ConditionalExpression(_) => todo!(),
            Expression::OrExpression(_) => todo!(),
            Expression::AndExpression(_) => todo!(),
            Expression::EqualityExpression(_) => todo!(),
            Expression::InequalityExpression(_) => todo!(),
            Expression::BitwiseOrExpression(_) => todo!(),
            Expression::BitwiseXorExpression(_) => todo!(),
            Expression::BitwiseAndExpression(_) => todo!(),
            Expression::ShiftExpression(_) => todo!(),
            Expression::AdditiveExpression(_) => todo!(),
            Expression::MultiplicativeExpression(_) => todo!(),
            Expression::ExponentiationExpression(_) => todo!(),
            Expression::PostfixExpression(_) => todo!(),
            Expression::PrefixExpression(_) => todo!(),
            Expression::FunctionCallExpression(_) => todo!(),
            Expression::CallOptionsExpression(_) => todo!(),
            Expression::MemberAccessExpression(_) => todo!(),
            Expression::IndexAccessExpression(index_access_expression) => self
                .annotations
                .get(&index_access_expression.node_id)
                .copied()
                .unwrap_or_else(|| {
                    unimplemented!("Expected index access expression to already be typed")
                }),
            Expression::NewExpression(_) => todo!(),
            Expression::TupleExpression(_) => todo!(),
            Expression::TypeExpression(_) => todo!(),
            Expression::ArrayExpression(_) => todo!(),
            Expression::HexNumberExpression(_) => todo!(),
            Expression::DecimalNumberExpression(_) => todo!(),
            Expression::StringExpression(_) => todo!(),
            Expression::ElementaryType(_) => todo!(),
            Expression::Identifier(identifier) => self.type_of_identifier(identifier),
            Expression::PayableKeyword => todo!(),
            Expression::ThisKeyword => todo!(),
            Expression::SuperKeyword => todo!(),
            Expression::TrueKeyword | Expression::FalseKeyword => self.types.bool(),
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
            BindingLocation::BuiltIn(BuiltInLocation { .. }) => {
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
}

impl Visitor for Pass {
    fn leave_assignment_expression(&mut self, node: &input_ir::AssignmentExpression) {
        let assignment_type = self.type_of_expression(&node.left_operand);
        self.annotations.insert(node.node_id, assignment_type);
    }

    fn leave_conditional_expression(&mut self, _node: &input_ir::ConditionalExpression) {
        todo!()
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

    fn leave_bitwise_or_expression(&mut self, _node: &input_ir::BitwiseOrExpression) {
        todo!()
    }

    fn leave_bitwise_xor_expression(&mut self, _node: &input_ir::BitwiseXorExpression) {
        todo!()
    }

    fn leave_bitwise_and_expression(&mut self, _node: &input_ir::BitwiseAndExpression) {
        todo!()
    }

    fn leave_shift_expression(&mut self, _node: &input_ir::ShiftExpression) {
        todo!()
    }

    fn leave_additive_expression(&mut self, _node: &input_ir::AdditiveExpression) {
        todo!()
    }

    fn leave_multiplicative_expression(&mut self, _node: &input_ir::MultiplicativeExpression) {
        todo!()
    }

    fn leave_exponentiation_expression(&mut self, _node: &input_ir::ExponentiationExpression) {
        todo!()
    }

    fn leave_postfix_expression(&mut self, _node: &input_ir::PostfixExpression) {
        todo!()
    }

    fn leave_prefix_expression(&mut self, _node: &input_ir::PrefixExpression) {
        todo!()
    }

    fn leave_function_call_expression(&mut self, node: &input_ir::FunctionCallExpression) {
        let function_type_id = self.type_of_expression(&node.operand);
        let function_type = self.types.get_type_by_id(function_type_id).unwrap();
        match function_type {
            Type::Function { return_type, .. } => {
                self.annotations.insert(node.node_id, *return_type);
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
