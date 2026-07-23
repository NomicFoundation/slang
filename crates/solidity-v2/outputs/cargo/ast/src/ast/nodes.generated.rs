// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(unused)]
use std::ops::Range;
use std::sync::Arc;

use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::context::SemanticContext;

use super::types::Type;

//
// Sequences
//

pub type AbicoderPragma = AbicoderPragmaStruct;

#[derive(Clone)]
pub struct AbicoderPragmaStruct {
    pub(crate) ir_node: ir::AbicoderPragma,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_abicoder_pragma(
    ir_node: &ir::AbicoderPragma,
    semantic: &Arc<SemanticContext>,
) -> AbicoderPragma {
    AbicoderPragmaStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl AbicoderPragmaStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn version(&self) -> AbicoderVersion {
        create_abicoder_version(&self.ir_node.version, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AdditiveExpression = AdditiveExpressionStruct;

#[derive(Clone)]
pub struct AdditiveExpressionStruct {
    pub(crate) ir_node: ir::AdditiveExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_additive_expression(
    ir_node: &ir::AdditiveExpression,
    semantic: &Arc<SemanticContext>,
) -> AdditiveExpression {
    AdditiveExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl AdditiveExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> AdditiveExpressionOperator {
        create_additive_expression_operator(&self.ir_node.operator, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AddressType = AddressTypeStruct;

#[derive(Clone)]
pub struct AddressTypeStruct {
    pub(crate) ir_node: ir::AddressType,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_address_type(
    ir_node: &ir::AddressType,
    semantic: &Arc<SemanticContext>,
) -> AddressType {
    AddressTypeStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl AddressTypeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn is_payable(&self) -> bool {
        self.ir_node.is_payable
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AndExpression = AndExpressionStruct;

#[derive(Clone)]
pub struct AndExpressionStruct {
    pub(crate) ir_node: ir::AndExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_and_expression(
    ir_node: &ir::AndExpression,
    semantic: &Arc<SemanticContext>,
) -> AndExpression {
    AndExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl AndExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ArrayExpression = ArrayExpressionStruct;

#[derive(Clone)]
pub struct ArrayExpressionStruct {
    pub(crate) ir_node: ir::ArrayExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_array_expression(
    ir_node: &ir::ArrayExpression,
    semantic: &Arc<SemanticContext>,
) -> ArrayExpression {
    ArrayExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ArrayExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn items(&self) -> ArrayValues {
        create_array_values(&self.ir_node.items, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ArrayTypeName = ArrayTypeNameStruct;

#[derive(Clone)]
pub struct ArrayTypeNameStruct {
    pub(crate) ir_node: ir::ArrayTypeName,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_array_type_name(
    ir_node: &ir::ArrayTypeName,
    semantic: &Arc<SemanticContext>,
) -> ArrayTypeName {
    ArrayTypeNameStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ArrayTypeNameStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> TypeName {
        create_type_name(&self.ir_node.operand, &self.semantic)
    }

    pub fn index(&self) -> Option<Expression> {
        self.ir_node
            .index
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AssemblyStatement = AssemblyStatementStruct;

#[derive(Clone)]
pub struct AssemblyStatementStruct {
    pub(crate) ir_node: ir::AssemblyStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_assembly_statement(
    ir_node: &ir::AssemblyStatement,
    semantic: &Arc<SemanticContext>,
) -> AssemblyStatement {
    AssemblyStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl AssemblyStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn label(&self) -> Option<StringLiteral> {
        self.ir_node
            .label
            .as_ref()
            .map(|ir_node| create_string_literal(ir_node, &self.semantic))
    }

    pub fn flags(&self) -> Option<YulFlags> {
        self.ir_node
            .flags
            .as_ref()
            .map(|ir_node| create_yul_flags(ir_node, &self.semantic))
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AssignmentExpression = AssignmentExpressionStruct;

#[derive(Clone)]
pub struct AssignmentExpressionStruct {
    pub(crate) ir_node: ir::AssignmentExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_assignment_expression(
    ir_node: &ir::AssignmentExpression,
    semantic: &Arc<SemanticContext>,
) -> AssignmentExpression {
    AssignmentExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl AssignmentExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> AssignmentExpressionOperator {
        create_assignment_expression_operator(&self.ir_node.operator, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BitwiseAndExpression = BitwiseAndExpressionStruct;

#[derive(Clone)]
pub struct BitwiseAndExpressionStruct {
    pub(crate) ir_node: ir::BitwiseAndExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_bitwise_and_expression(
    ir_node: &ir::BitwiseAndExpression,
    semantic: &Arc<SemanticContext>,
) -> BitwiseAndExpression {
    BitwiseAndExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl BitwiseAndExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BitwiseOrExpression = BitwiseOrExpressionStruct;

#[derive(Clone)]
pub struct BitwiseOrExpressionStruct {
    pub(crate) ir_node: ir::BitwiseOrExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_bitwise_or_expression(
    ir_node: &ir::BitwiseOrExpression,
    semantic: &Arc<SemanticContext>,
) -> BitwiseOrExpression {
    BitwiseOrExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl BitwiseOrExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BitwiseXorExpression = BitwiseXorExpressionStruct;

#[derive(Clone)]
pub struct BitwiseXorExpressionStruct {
    pub(crate) ir_node: ir::BitwiseXorExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_bitwise_xor_expression(
    ir_node: &ir::BitwiseXorExpression,
    semantic: &Arc<SemanticContext>,
) -> BitwiseXorExpression {
    BitwiseXorExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl BitwiseXorExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Block = BlockStruct;

#[derive(Clone)]
pub struct BlockStruct {
    pub(crate) ir_node: ir::Block,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_block(ir_node: &ir::Block, semantic: &Arc<SemanticContext>) -> Block {
    BlockStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl BlockStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn statements(&self) -> Statements {
        create_statements(&self.ir_node.statements, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BreakStatement = BreakStatementStruct;

#[derive(Clone)]
pub struct BreakStatementStruct {
    pub(crate) ir_node: ir::BreakStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_break_statement(
    ir_node: &ir::BreakStatement,
    semantic: &Arc<SemanticContext>,
) -> BreakStatement {
    BreakStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl BreakStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type CallOptionsExpression = CallOptionsExpressionStruct;

#[derive(Clone)]
pub struct CallOptionsExpressionStruct {
    pub(crate) ir_node: ir::CallOptionsExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_call_options_expression(
    ir_node: &ir::CallOptionsExpression,
    semantic: &Arc<SemanticContext>,
) -> CallOptionsExpression {
    CallOptionsExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl CallOptionsExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn options(&self) -> CallOptions {
        create_call_options(&self.ir_node.options, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type CatchClause = CatchClauseStruct;

#[derive(Clone)]
pub struct CatchClauseStruct {
    pub(crate) ir_node: ir::CatchClause,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_catch_clause(
    ir_node: &ir::CatchClause,
    semantic: &Arc<SemanticContext>,
) -> CatchClause {
    CatchClauseStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl CatchClauseStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn error(&self) -> Option<CatchClauseError> {
        self.ir_node
            .error
            .as_ref()
            .map(|ir_node| create_catch_clause_error(ir_node, &self.semantic))
    }

    pub fn body(&self) -> Block {
        create_block(&self.ir_node.body, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type CatchClauseError = CatchClauseErrorStruct;

#[derive(Clone)]
pub struct CatchClauseErrorStruct {
    pub(crate) ir_node: ir::CatchClauseError,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_catch_clause_error(
    ir_node: &ir::CatchClauseError,
    semantic: &Arc<SemanticContext>,
) -> CatchClauseError {
    CatchClauseErrorStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl CatchClauseErrorStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Option<Identifier> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ConditionalExpression = ConditionalExpressionStruct;

#[derive(Clone)]
pub struct ConditionalExpressionStruct {
    pub(crate) ir_node: ir::ConditionalExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_conditional_expression(
    ir_node: &ir::ConditionalExpression,
    semantic: &Arc<SemanticContext>,
) -> ConditionalExpression {
    ConditionalExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ConditionalExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn true_expression(&self) -> Expression {
        create_expression(&self.ir_node.true_expression, &self.semantic)
    }

    pub fn false_expression(&self) -> Expression {
        create_expression(&self.ir_node.false_expression, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ConstantDefinition = ConstantDefinitionStruct;

#[derive(Clone)]
pub struct ConstantDefinitionStruct {
    pub(crate) ir_node: ir::ConstantDefinition,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_constant_definition(
    ir_node: &ir::ConstantDefinition,
    semantic: &Arc<SemanticContext>,
) -> ConstantDefinition {
    ConstantDefinitionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ConstantDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn visibility(&self) -> Option<StateVariableVisibility> {
        self.ir_node
            .visibility
            .as_ref()
            .map(|ir_node| create_state_variable_visibility(ir_node, &self.semantic))
    }

    pub fn value(&self) -> Option<Expression> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ContinueStatement = ContinueStatementStruct;

#[derive(Clone)]
pub struct ContinueStatementStruct {
    pub(crate) ir_node: ir::ContinueStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_continue_statement(
    ir_node: &ir::ContinueStatement,
    semantic: &Arc<SemanticContext>,
) -> ContinueStatement {
    ContinueStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ContinueStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ContractDefinition = ContractDefinitionStruct;

#[derive(Clone)]
pub struct ContractDefinitionStruct {
    pub(crate) ir_node: ir::ContractDefinition,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_contract_definition(
    ir_node: &ir::ContractDefinition,
    semantic: &Arc<SemanticContext>,
) -> ContractDefinition {
    ContractDefinitionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ContractDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn is_abstract(&self) -> bool {
        self.ir_node.is_abstract
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn inheritance_types(&self) -> InheritanceTypes {
        create_inheritance_types(&self.ir_node.inheritance_types, &self.semantic)
    }

    pub fn storage_layout(&self) -> Option<Expression> {
        self.ir_node
            .storage_layout
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn members(&self) -> ContractMembers {
        create_contract_members(&self.ir_node.members, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type DecimalNumberExpression = DecimalNumberExpressionStruct;

#[derive(Clone)]
pub struct DecimalNumberExpressionStruct {
    pub(crate) ir_node: ir::DecimalNumberExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_decimal_number_expression(
    ir_node: &ir::DecimalNumberExpression,
    semantic: &Arc<SemanticContext>,
) -> DecimalNumberExpression {
    DecimalNumberExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl DecimalNumberExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn literal(&self) -> DecimalLiteral {
        create_decimal_literal(&self.ir_node.literal, &self.semantic)
    }

    pub fn unit(&self) -> Option<NumberUnit> {
        self.ir_node
            .unit
            .as_ref()
            .map(|ir_node| create_number_unit(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type DoWhileStatement = DoWhileStatementStruct;

#[derive(Clone)]
pub struct DoWhileStatementStruct {
    pub(crate) ir_node: ir::DoWhileStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_do_while_statement(
    ir_node: &ir::DoWhileStatement,
    semantic: &Arc<SemanticContext>,
) -> DoWhileStatement {
    DoWhileStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl DoWhileStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn body(&self) -> Statement {
        create_statement(&self.ir_node.body, &self.semantic)
    }

    pub fn condition(&self) -> Expression {
        create_expression(&self.ir_node.condition, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type EmitStatement = EmitStatementStruct;

#[derive(Clone)]
pub struct EmitStatementStruct {
    pub(crate) ir_node: ir::EmitStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_emit_statement(
    ir_node: &ir::EmitStatement,
    semantic: &Arc<SemanticContext>,
) -> EmitStatement {
    EmitStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl EmitStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn event(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.event, &self.semantic)
    }

    pub fn arguments(&self) -> ArgumentsDeclaration {
        create_arguments_declaration(&self.ir_node.arguments, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type EnumDefinition = EnumDefinitionStruct;

#[derive(Clone)]
pub struct EnumDefinitionStruct {
    pub(crate) ir_node: ir::EnumDefinition,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_enum_definition(
    ir_node: &ir::EnumDefinition,
    semantic: &Arc<SemanticContext>,
) -> EnumDefinition {
    EnumDefinitionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl EnumDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn members(&self) -> EnumMembers {
        create_enum_members(&self.ir_node.members, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type EqualityExpression = EqualityExpressionStruct;

#[derive(Clone)]
pub struct EqualityExpressionStruct {
    pub(crate) ir_node: ir::EqualityExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_equality_expression(
    ir_node: &ir::EqualityExpression,
    semantic: &Arc<SemanticContext>,
) -> EqualityExpression {
    EqualityExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl EqualityExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> EqualityExpressionOperator {
        create_equality_expression_operator(&self.ir_node.operator, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ErrorDefinition = ErrorDefinitionStruct;

#[derive(Clone)]
pub struct ErrorDefinitionStruct {
    pub(crate) ir_node: ir::ErrorDefinition,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_error_definition(
    ir_node: &ir::ErrorDefinition,
    semantic: &Arc<SemanticContext>,
) -> ErrorDefinition {
    ErrorDefinitionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ErrorDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type EventDefinition = EventDefinitionStruct;

#[derive(Clone)]
pub struct EventDefinitionStruct {
    pub(crate) ir_node: ir::EventDefinition,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_event_definition(
    ir_node: &ir::EventDefinition,
    semantic: &Arc<SemanticContext>,
) -> EventDefinition {
    EventDefinitionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl EventDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn is_anonymous(&self) -> bool {
        self.ir_node.is_anonymous
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ExperimentalPragma = ExperimentalPragmaStruct;

#[derive(Clone)]
pub struct ExperimentalPragmaStruct {
    pub(crate) ir_node: ir::ExperimentalPragma,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_experimental_pragma(
    ir_node: &ir::ExperimentalPragma,
    semantic: &Arc<SemanticContext>,
) -> ExperimentalPragma {
    ExperimentalPragmaStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ExperimentalPragmaStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn feature(&self) -> ExperimentalFeature {
        create_experimental_feature(&self.ir_node.feature, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ExponentiationExpression = ExponentiationExpressionStruct;

#[derive(Clone)]
pub struct ExponentiationExpressionStruct {
    pub(crate) ir_node: ir::ExponentiationExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_exponentiation_expression(
    ir_node: &ir::ExponentiationExpression,
    semantic: &Arc<SemanticContext>,
) -> ExponentiationExpression {
    ExponentiationExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ExponentiationExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ExpressionStatement = ExpressionStatementStruct;

#[derive(Clone)]
pub struct ExpressionStatementStruct {
    pub(crate) ir_node: ir::ExpressionStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_expression_statement(
    ir_node: &ir::ExpressionStatement,
    semantic: &Arc<SemanticContext>,
) -> ExpressionStatement {
    ExpressionStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ExpressionStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> Expression {
        create_expression(&self.ir_node.expression, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ForStatement = ForStatementStruct;

#[derive(Clone)]
pub struct ForStatementStruct {
    pub(crate) ir_node: ir::ForStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_for_statement(
    ir_node: &ir::ForStatement,
    semantic: &Arc<SemanticContext>,
) -> ForStatement {
    ForStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ForStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn initialization(&self) -> ForStatementInitialization {
        create_for_statement_initialization(&self.ir_node.initialization, &self.semantic)
    }

    pub fn condition(&self) -> ForStatementCondition {
        create_for_statement_condition(&self.ir_node.condition, &self.semantic)
    }

    pub fn iterator(&self) -> Option<Expression> {
        self.ir_node
            .iterator
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn body(&self) -> Statement {
        create_statement(&self.ir_node.body, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionAttributes = FunctionAttributesStruct;

#[derive(Clone)]
pub struct FunctionAttributesStruct {
    pub(crate) ir_node: ir::FunctionAttributes,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_function_attributes(
    ir_node: &ir::FunctionAttributes,
    semantic: &Arc<SemanticContext>,
) -> FunctionAttributes {
    FunctionAttributesStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl FunctionAttributesStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn visibility(&self) -> FunctionVisibility {
        create_function_visibility(&self.ir_node.visibility, &self.semantic)
    }

    pub fn has_explicit_visibility(&self) -> bool {
        self.ir_node.has_explicit_visibility
    }

    pub fn mutability(&self) -> FunctionMutability {
        create_function_mutability(&self.ir_node.mutability, &self.semantic)
    }

    pub fn is_virtual(&self) -> bool {
        self.ir_node.is_virtual
    }

    pub fn override_specifier(&self) -> Option<OverridePaths> {
        self.ir_node
            .override_specifier
            .as_ref()
            .map(|ir_node| create_override_paths(ir_node, &self.semantic))
    }

    pub fn modifier_invocations(&self) -> ModifierInvocations {
        create_modifier_invocations(&self.ir_node.modifier_invocations, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionCallExpression = FunctionCallExpressionStruct;

#[derive(Clone)]
pub struct FunctionCallExpressionStruct {
    pub(crate) ir_node: ir::FunctionCallExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_function_call_expression(
    ir_node: &ir::FunctionCallExpression,
    semantic: &Arc<SemanticContext>,
) -> FunctionCallExpression {
    FunctionCallExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl FunctionCallExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn arguments(&self) -> ArgumentsDeclaration {
        create_arguments_declaration(&self.ir_node.arguments, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionDefinition = FunctionDefinitionStruct;

#[derive(Clone)]
pub struct FunctionDefinitionStruct {
    pub(crate) ir_node: ir::FunctionDefinition,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_function_definition(
    ir_node: &ir::FunctionDefinition,
    semantic: &Arc<SemanticContext>,
) -> FunctionDefinition {
    FunctionDefinitionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl FunctionDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn kind(&self) -> FunctionKind {
        create_function_kind(&self.ir_node.kind, &self.semantic)
    }

    pub fn name(&self) -> Option<Identifier> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn attributes(&self) -> FunctionAttributes {
        create_function_attributes(&self.ir_node.attributes, &self.semantic)
    }

    pub fn returns(&self) -> Option<Parameters> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, &self.semantic))
    }

    pub fn body(&self) -> Option<Block> {
        self.ir_node
            .body
            .as_ref()
            .map(|ir_node| create_block(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionType = FunctionTypeStruct;

#[derive(Clone)]
pub struct FunctionTypeStruct {
    pub(crate) ir_node: ir::FunctionType,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_function_type(
    ir_node: &ir::FunctionType,
    semantic: &Arc<SemanticContext>,
) -> FunctionType {
    FunctionTypeStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl FunctionTypeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn attributes(&self) -> FunctionTypeAttributes {
        create_function_type_attributes(&self.ir_node.attributes, &self.semantic)
    }

    pub fn returns(&self) -> Option<Parameters> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionTypeAttributes = FunctionTypeAttributesStruct;

#[derive(Clone)]
pub struct FunctionTypeAttributesStruct {
    pub(crate) ir_node: ir::FunctionTypeAttributes,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_function_type_attributes(
    ir_node: &ir::FunctionTypeAttributes,
    semantic: &Arc<SemanticContext>,
) -> FunctionTypeAttributes {
    FunctionTypeAttributesStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl FunctionTypeAttributesStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn visibility(&self) -> FunctionVisibility {
        create_function_visibility(&self.ir_node.visibility, &self.semantic)
    }

    pub fn mutability(&self) -> FunctionMutability {
        create_function_mutability(&self.ir_node.mutability, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type HexNumberExpression = HexNumberExpressionStruct;

#[derive(Clone)]
pub struct HexNumberExpressionStruct {
    pub(crate) ir_node: ir::HexNumberExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_hex_number_expression(
    ir_node: &ir::HexNumberExpression,
    semantic: &Arc<SemanticContext>,
) -> HexNumberExpression {
    HexNumberExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl HexNumberExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn literal(&self) -> HexLiteral {
        create_hex_literal(&self.ir_node.literal, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type IfStatement = IfStatementStruct;

#[derive(Clone)]
pub struct IfStatementStruct {
    pub(crate) ir_node: ir::IfStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_if_statement(
    ir_node: &ir::IfStatement,
    semantic: &Arc<SemanticContext>,
) -> IfStatement {
    IfStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl IfStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn condition(&self) -> Expression {
        create_expression(&self.ir_node.condition, &self.semantic)
    }

    pub fn body(&self) -> Statement {
        create_statement(&self.ir_node.body, &self.semantic)
    }

    pub fn else_branch(&self) -> Option<Statement> {
        self.ir_node
            .else_branch
            .as_ref()
            .map(|ir_node| create_statement(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ImportDeconstruction = ImportDeconstructionStruct;

#[derive(Clone)]
pub struct ImportDeconstructionStruct {
    pub(crate) ir_node: ir::ImportDeconstruction,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_import_deconstruction(
    ir_node: &ir::ImportDeconstruction,
    semantic: &Arc<SemanticContext>,
) -> ImportDeconstruction {
    ImportDeconstructionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ImportDeconstructionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn symbols(&self) -> ImportDeconstructionSymbols {
        create_import_deconstruction_symbols(&self.ir_node.symbols, &self.semantic)
    }

    pub fn path(&self) -> StringLiteral {
        create_string_literal(&self.ir_node.path, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ImportDeconstructionSymbol = ImportDeconstructionSymbolStruct;

#[derive(Clone)]
pub struct ImportDeconstructionSymbolStruct {
    pub(crate) ir_node: ir::ImportDeconstructionSymbol,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_import_deconstruction_symbol(
    ir_node: &ir::ImportDeconstructionSymbol,
    semantic: &Arc<SemanticContext>,
) -> ImportDeconstructionSymbol {
    ImportDeconstructionSymbolStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ImportDeconstructionSymbolStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn alias(&self) -> Option<Identifier> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type IndexAccessExpression = IndexAccessExpressionStruct;

#[derive(Clone)]
pub struct IndexAccessExpressionStruct {
    pub(crate) ir_node: ir::IndexAccessExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_index_access_expression(
    ir_node: &ir::IndexAccessExpression,
    semantic: &Arc<SemanticContext>,
) -> IndexAccessExpression {
    IndexAccessExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl IndexAccessExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn start(&self) -> Option<Expression> {
        self.ir_node
            .start
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn end(&self) -> Option<Expression> {
        self.ir_node
            .end
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn is_slice(&self) -> bool {
        self.ir_node.is_slice
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type InequalityExpression = InequalityExpressionStruct;

#[derive(Clone)]
pub struct InequalityExpressionStruct {
    pub(crate) ir_node: ir::InequalityExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_inequality_expression(
    ir_node: &ir::InequalityExpression,
    semantic: &Arc<SemanticContext>,
) -> InequalityExpression {
    InequalityExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl InequalityExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> InequalityExpressionOperator {
        create_inequality_expression_operator(&self.ir_node.operator, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type InheritanceType = InheritanceTypeStruct;

#[derive(Clone)]
pub struct InheritanceTypeStruct {
    pub(crate) ir_node: ir::InheritanceType,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_inheritance_type(
    ir_node: &ir::InheritanceType,
    semantic: &Arc<SemanticContext>,
) -> InheritanceType {
    InheritanceTypeStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl InheritanceTypeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.type_name, &self.semantic)
    }

    pub fn arguments(&self) -> Option<ArgumentsDeclaration> {
        self.ir_node
            .arguments
            .as_ref()
            .map(|ir_node| create_arguments_declaration(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type InterfaceDefinition = InterfaceDefinitionStruct;

#[derive(Clone)]
pub struct InterfaceDefinitionStruct {
    pub(crate) ir_node: ir::InterfaceDefinition,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_interface_definition(
    ir_node: &ir::InterfaceDefinition,
    semantic: &Arc<SemanticContext>,
) -> InterfaceDefinition {
    InterfaceDefinitionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl InterfaceDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn inheritance(&self) -> Option<InheritanceTypes> {
        self.ir_node
            .inheritance
            .as_ref()
            .map(|ir_node| create_inheritance_types(ir_node, &self.semantic))
    }

    pub fn members(&self) -> InterfaceMembers {
        create_interface_members(&self.ir_node.members, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type LibraryDefinition = LibraryDefinitionStruct;

#[derive(Clone)]
pub struct LibraryDefinitionStruct {
    pub(crate) ir_node: ir::LibraryDefinition,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_library_definition(
    ir_node: &ir::LibraryDefinition,
    semantic: &Arc<SemanticContext>,
) -> LibraryDefinition {
    LibraryDefinitionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl LibraryDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn members(&self) -> LibraryMembers {
        create_library_members(&self.ir_node.members, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MappingType = MappingTypeStruct;

#[derive(Clone)]
pub struct MappingTypeStruct {
    pub(crate) ir_node: ir::MappingType,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_mapping_type(
    ir_node: &ir::MappingType,
    semantic: &Arc<SemanticContext>,
) -> MappingType {
    MappingTypeStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl MappingTypeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn key_type(&self) -> Parameter {
        create_parameter(&self.ir_node.key_type, &self.semantic)
    }

    pub fn value_type(&self) -> Parameter {
        create_parameter(&self.ir_node.value_type, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MemberAccessExpression = MemberAccessExpressionStruct;

#[derive(Clone)]
pub struct MemberAccessExpressionStruct {
    pub(crate) ir_node: ir::MemberAccessExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_member_access_expression(
    ir_node: &ir::MemberAccessExpression,
    semantic: &Arc<SemanticContext>,
) -> MemberAccessExpression {
    MemberAccessExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl MemberAccessExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn member(&self) -> Identifier {
        create_identifier(&self.ir_node.member, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ModifierInvocation = ModifierInvocationStruct;

#[derive(Clone)]
pub struct ModifierInvocationStruct {
    pub(crate) ir_node: ir::ModifierInvocation,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_modifier_invocation(
    ir_node: &ir::ModifierInvocation,
    semantic: &Arc<SemanticContext>,
) -> ModifierInvocation {
    ModifierInvocationStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ModifierInvocationStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.name, &self.semantic)
    }

    pub fn arguments(&self) -> Option<ArgumentsDeclaration> {
        self.ir_node
            .arguments
            .as_ref()
            .map(|ir_node| create_arguments_declaration(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MultiTypedDeclaration = MultiTypedDeclarationStruct;

#[derive(Clone)]
pub struct MultiTypedDeclarationStruct {
    pub(crate) ir_node: ir::MultiTypedDeclaration,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_multi_typed_declaration(
    ir_node: &ir::MultiTypedDeclaration,
    semantic: &Arc<SemanticContext>,
) -> MultiTypedDeclaration {
    MultiTypedDeclarationStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl MultiTypedDeclarationStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn elements(&self) -> MultiTypedDeclarationElements {
        create_multi_typed_declaration_elements(&self.ir_node.elements, &self.semantic)
    }

    pub fn value(&self) -> Expression {
        create_expression(&self.ir_node.value, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MultiTypedDeclarationElement = MultiTypedDeclarationElementStruct;

#[derive(Clone)]
pub struct MultiTypedDeclarationElementStruct {
    pub(crate) ir_node: ir::MultiTypedDeclarationElement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_multi_typed_declaration_element(
    ir_node: &ir::MultiTypedDeclarationElement,
    semantic: &Arc<SemanticContext>,
) -> MultiTypedDeclarationElement {
    MultiTypedDeclarationElementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl MultiTypedDeclarationElementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn member(&self) -> Option<VariableDeclaration> {
        self.ir_node
            .member
            .as_ref()
            .map(|ir_node| create_variable_declaration(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MultiplicativeExpression = MultiplicativeExpressionStruct;

#[derive(Clone)]
pub struct MultiplicativeExpressionStruct {
    pub(crate) ir_node: ir::MultiplicativeExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_multiplicative_expression(
    ir_node: &ir::MultiplicativeExpression,
    semantic: &Arc<SemanticContext>,
) -> MultiplicativeExpression {
    MultiplicativeExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl MultiplicativeExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> MultiplicativeExpressionOperator {
        create_multiplicative_expression_operator(&self.ir_node.operator, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type NamedArgument = NamedArgumentStruct;

#[derive(Clone)]
pub struct NamedArgumentStruct {
    pub(crate) ir_node: ir::NamedArgument,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_named_argument(
    ir_node: &ir::NamedArgument,
    semantic: &Arc<SemanticContext>,
) -> NamedArgument {
    NamedArgumentStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl NamedArgumentStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn value(&self) -> Expression {
        create_expression(&self.ir_node.value, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type NewExpression = NewExpressionStruct;

#[derive(Clone)]
pub struct NewExpressionStruct {
    pub(crate) ir_node: ir::NewExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_new_expression(
    ir_node: &ir::NewExpression,
    semantic: &Arc<SemanticContext>,
) -> NewExpression {
    NewExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl NewExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type OrExpression = OrExpressionStruct;

#[derive(Clone)]
pub struct OrExpressionStruct {
    pub(crate) ir_node: ir::OrExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_or_expression(
    ir_node: &ir::OrExpression,
    semantic: &Arc<SemanticContext>,
) -> OrExpression {
    OrExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl OrExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Parameter = ParameterStruct;

#[derive(Clone)]
pub struct ParameterStruct {
    pub(crate) ir_node: ir::Parameter,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_parameter(ir_node: &ir::Parameter, semantic: &Arc<SemanticContext>) -> Parameter {
    ParameterStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ParameterStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn storage_location(&self) -> Option<StorageLocation> {
        self.ir_node
            .storage_location
            .as_ref()
            .map(|ir_node| create_storage_location(ir_node, &self.semantic))
    }

    pub fn name(&self) -> Option<Identifier> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn is_indexed(&self) -> bool {
        self.ir_node.is_indexed
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PathImport = PathImportStruct;

#[derive(Clone)]
pub struct PathImportStruct {
    pub(crate) ir_node: ir::PathImport,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_path_import(ir_node: &ir::PathImport, semantic: &Arc<SemanticContext>) -> PathImport {
    PathImportStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PathImportStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn path(&self) -> StringLiteral {
        create_string_literal(&self.ir_node.path, &self.semantic)
    }

    pub fn alias(&self) -> Option<Identifier> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PostfixExpression = PostfixExpressionStruct;

#[derive(Clone)]
pub struct PostfixExpressionStruct {
    pub(crate) ir_node: ir::PostfixExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_postfix_expression(
    ir_node: &ir::PostfixExpression,
    semantic: &Arc<SemanticContext>,
) -> PostfixExpression {
    PostfixExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PostfixExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn operator(&self) -> PostfixExpressionOperator {
        create_postfix_expression_operator(&self.ir_node.operator, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaDirective = PragmaDirectiveStruct;

#[derive(Clone)]
pub struct PragmaDirectiveStruct {
    pub(crate) ir_node: ir::PragmaDirective,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_pragma_directive(
    ir_node: &ir::PragmaDirective,
    semantic: &Arc<SemanticContext>,
) -> PragmaDirective {
    PragmaDirectiveStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PragmaDirectiveStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn pragma(&self) -> Pragma {
        create_pragma(&self.ir_node.pragma, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PrefixExpression = PrefixExpressionStruct;

#[derive(Clone)]
pub struct PrefixExpressionStruct {
    pub(crate) ir_node: ir::PrefixExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_prefix_expression(
    ir_node: &ir::PrefixExpression,
    semantic: &Arc<SemanticContext>,
) -> PrefixExpression {
    PrefixExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PrefixExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operator(&self) -> PrefixExpressionOperator {
        create_prefix_expression_operator(&self.ir_node.operator, &self.semantic)
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ReturnStatement = ReturnStatementStruct;

#[derive(Clone)]
pub struct ReturnStatementStruct {
    pub(crate) ir_node: ir::ReturnStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_return_statement(
    ir_node: &ir::ReturnStatement,
    semantic: &Arc<SemanticContext>,
) -> ReturnStatement {
    ReturnStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ReturnStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> Option<Expression> {
        self.ir_node
            .expression
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type RevertStatement = RevertStatementStruct;

#[derive(Clone)]
pub struct RevertStatementStruct {
    pub(crate) ir_node: ir::RevertStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_revert_statement(
    ir_node: &ir::RevertStatement,
    semantic: &Arc<SemanticContext>,
) -> RevertStatement {
    RevertStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl RevertStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn error(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.error, &self.semantic)
    }

    pub fn arguments(&self) -> ArgumentsDeclaration {
        create_arguments_declaration(&self.ir_node.arguments, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ShiftExpression = ShiftExpressionStruct;

#[derive(Clone)]
pub struct ShiftExpressionStruct {
    pub(crate) ir_node: ir::ShiftExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_shift_expression(
    ir_node: &ir::ShiftExpression,
    semantic: &Arc<SemanticContext>,
) -> ShiftExpression {
    ShiftExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ShiftExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> ShiftExpressionOperator {
        create_shift_expression_operator(&self.ir_node.operator, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type SingleTypedDeclaration = SingleTypedDeclarationStruct;

#[derive(Clone)]
pub struct SingleTypedDeclarationStruct {
    pub(crate) ir_node: ir::SingleTypedDeclaration,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_single_typed_declaration(
    ir_node: &ir::SingleTypedDeclaration,
    semantic: &Arc<SemanticContext>,
) -> SingleTypedDeclaration {
    SingleTypedDeclarationStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl SingleTypedDeclarationStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn declaration(&self) -> VariableDeclaration {
        create_variable_declaration(&self.ir_node.declaration, &self.semantic)
    }

    pub fn value(&self) -> Option<Expression> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type SourceUnit = SourceUnitStruct;

#[derive(Clone)]
pub struct SourceUnitStruct {
    pub(crate) ir_node: ir::SourceUnit,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_source_unit(ir_node: &ir::SourceUnit, semantic: &Arc<SemanticContext>) -> SourceUnit {
    SourceUnitStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl SourceUnitStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn members(&self) -> SourceUnitMembers {
        create_source_unit_members(&self.ir_node.members, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StateVariableAttributes = StateVariableAttributesStruct;

#[derive(Clone)]
pub struct StateVariableAttributesStruct {
    pub(crate) ir_node: ir::StateVariableAttributes,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_state_variable_attributes(
    ir_node: &ir::StateVariableAttributes,
    semantic: &Arc<SemanticContext>,
) -> StateVariableAttributes {
    StateVariableAttributesStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl StateVariableAttributesStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn visibility(&self) -> StateVariableVisibility {
        create_state_variable_visibility(&self.ir_node.visibility, &self.semantic)
    }

    pub fn mutability(&self) -> StateVariableMutability {
        create_state_variable_mutability(&self.ir_node.mutability, &self.semantic)
    }

    pub fn override_specifier(&self) -> Option<OverridePaths> {
        self.ir_node
            .override_specifier
            .as_ref()
            .map(|ir_node| create_override_paths(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StateVariableDefinition = StateVariableDefinitionStruct;

#[derive(Clone)]
pub struct StateVariableDefinitionStruct {
    pub(crate) ir_node: ir::StateVariableDefinition,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_state_variable_definition(
    ir_node: &ir::StateVariableDefinition,
    semantic: &Arc<SemanticContext>,
) -> StateVariableDefinition {
    StateVariableDefinitionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl StateVariableDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn value(&self) -> Option<Expression> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn attributes(&self) -> StateVariableAttributes {
        create_state_variable_attributes(&self.ir_node.attributes, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StructDefinition = StructDefinitionStruct;

#[derive(Clone)]
pub struct StructDefinitionStruct {
    pub(crate) ir_node: ir::StructDefinition,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_struct_definition(
    ir_node: &ir::StructDefinition,
    semantic: &Arc<SemanticContext>,
) -> StructDefinition {
    StructDefinitionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl StructDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn members(&self) -> StructMembers {
        create_struct_members(&self.ir_node.members, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StructMember = StructMemberStruct;

#[derive(Clone)]
pub struct StructMemberStruct {
    pub(crate) ir_node: ir::StructMember,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_struct_member(
    ir_node: &ir::StructMember,
    semantic: &Arc<SemanticContext>,
) -> StructMember {
    StructMemberStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl StructMemberStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type TryStatement = TryStatementStruct;

#[derive(Clone)]
pub struct TryStatementStruct {
    pub(crate) ir_node: ir::TryStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_try_statement(
    ir_node: &ir::TryStatement,
    semantic: &Arc<SemanticContext>,
) -> TryStatement {
    TryStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl TryStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> Expression {
        create_expression(&self.ir_node.expression, &self.semantic)
    }

    pub fn returns(&self) -> Option<Parameters> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, &self.semantic))
    }

    pub fn body(&self) -> Block {
        create_block(&self.ir_node.body, &self.semantic)
    }

    pub fn catch_clauses(&self) -> CatchClauses {
        create_catch_clauses(&self.ir_node.catch_clauses, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type TupleExpression = TupleExpressionStruct;

#[derive(Clone)]
pub struct TupleExpressionStruct {
    pub(crate) ir_node: ir::TupleExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_tuple_expression(
    ir_node: &ir::TupleExpression,
    semantic: &Arc<SemanticContext>,
) -> TupleExpression {
    TupleExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl TupleExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn items(&self) -> TupleValues {
        create_tuple_values(&self.ir_node.items, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type TupleValue = TupleValueStruct;

#[derive(Clone)]
pub struct TupleValueStruct {
    pub(crate) ir_node: ir::TupleValue,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_tuple_value(ir_node: &ir::TupleValue, semantic: &Arc<SemanticContext>) -> TupleValue {
    TupleValueStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl TupleValueStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> Option<Expression> {
        self.ir_node
            .expression
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type TypeExpression = TypeExpressionStruct;

#[derive(Clone)]
pub struct TypeExpressionStruct {
    pub(crate) ir_node: ir::TypeExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_type_expression(
    ir_node: &ir::TypeExpression,
    semantic: &Arc<SemanticContext>,
) -> TypeExpression {
    TypeExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl TypeExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UncheckedBlock = UncheckedBlockStruct;

#[derive(Clone)]
pub struct UncheckedBlockStruct {
    pub(crate) ir_node: ir::UncheckedBlock,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_unchecked_block(
    ir_node: &ir::UncheckedBlock,
    semantic: &Arc<SemanticContext>,
) -> UncheckedBlock {
    UncheckedBlockStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl UncheckedBlockStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn block(&self) -> Block {
        create_block(&self.ir_node.block, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UserDefinedValueTypeDefinition = UserDefinedValueTypeDefinitionStruct;

#[derive(Clone)]
pub struct UserDefinedValueTypeDefinitionStruct {
    pub(crate) ir_node: ir::UserDefinedValueTypeDefinition,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_user_defined_value_type_definition(
    ir_node: &ir::UserDefinedValueTypeDefinition,
    semantic: &Arc<SemanticContext>,
) -> UserDefinedValueTypeDefinition {
    UserDefinedValueTypeDefinitionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl UserDefinedValueTypeDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn value_type(&self) -> ElementaryType {
        create_elementary_type(&self.ir_node.value_type, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UsingDeconstruction = UsingDeconstructionStruct;

#[derive(Clone)]
pub struct UsingDeconstructionStruct {
    pub(crate) ir_node: ir::UsingDeconstruction,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_using_deconstruction(
    ir_node: &ir::UsingDeconstruction,
    semantic: &Arc<SemanticContext>,
) -> UsingDeconstruction {
    UsingDeconstructionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl UsingDeconstructionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn symbols(&self) -> UsingDeconstructionSymbols {
        create_using_deconstruction_symbols(&self.ir_node.symbols, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UsingDeconstructionSymbol = UsingDeconstructionSymbolStruct;

#[derive(Clone)]
pub struct UsingDeconstructionSymbolStruct {
    pub(crate) ir_node: ir::UsingDeconstructionSymbol,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_using_deconstruction_symbol(
    ir_node: &ir::UsingDeconstructionSymbol,
    semantic: &Arc<SemanticContext>,
) -> UsingDeconstructionSymbol {
    UsingDeconstructionSymbolStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl UsingDeconstructionSymbolStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.name, &self.semantic)
    }

    pub fn alias(&self) -> Option<UsingOperator> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| create_using_operator(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UsingDirective = UsingDirectiveStruct;

#[derive(Clone)]
pub struct UsingDirectiveStruct {
    pub(crate) ir_node: ir::UsingDirective,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_using_directive(
    ir_node: &ir::UsingDirective,
    semantic: &Arc<SemanticContext>,
) -> UsingDirective {
    UsingDirectiveStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl UsingDirectiveStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn clause(&self) -> UsingClause {
        create_using_clause(&self.ir_node.clause, &self.semantic)
    }

    pub fn target(&self) -> UsingTarget {
        create_using_target(&self.ir_node.target, &self.semantic)
    }

    pub fn is_global(&self) -> bool {
        self.ir_node.is_global
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VariableDeclaration = VariableDeclarationStruct;

#[derive(Clone)]
pub struct VariableDeclarationStruct {
    pub(crate) ir_node: ir::VariableDeclaration,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_variable_declaration(
    ir_node: &ir::VariableDeclaration,
    semantic: &Arc<SemanticContext>,
) -> VariableDeclaration {
    VariableDeclarationStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl VariableDeclarationStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn storage_location(&self) -> Option<StorageLocation> {
        self.ir_node
            .storage_location
            .as_ref()
            .map(|ir_node| create_storage_location(ir_node, &self.semantic))
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VariableDeclarationStatement = VariableDeclarationStatementStruct;

#[derive(Clone)]
pub struct VariableDeclarationStatementStruct {
    pub(crate) ir_node: ir::VariableDeclarationStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_variable_declaration_statement(
    ir_node: &ir::VariableDeclarationStatement,
    semantic: &Arc<SemanticContext>,
) -> VariableDeclarationStatement {
    VariableDeclarationStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl VariableDeclarationStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn target(&self) -> VariableDeclarationTarget {
        create_variable_declaration_target(&self.ir_node.target, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VersionPragma = VersionPragmaStruct;

#[derive(Clone)]
pub struct VersionPragmaStruct {
    pub(crate) ir_node: ir::VersionPragma,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_version_pragma(
    ir_node: &ir::VersionPragma,
    semantic: &Arc<SemanticContext>,
) -> VersionPragma {
    VersionPragmaStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl VersionPragmaStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn sets(&self) -> VersionExpressionSets {
        create_version_expression_sets(&self.ir_node.sets, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VersionRange = VersionRangeStruct;

#[derive(Clone)]
pub struct VersionRangeStruct {
    pub(crate) ir_node: ir::VersionRange,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_version_range(
    ir_node: &ir::VersionRange,
    semantic: &Arc<SemanticContext>,
) -> VersionRange {
    VersionRangeStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl VersionRangeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn start(&self) -> VersionLiteral {
        create_version_literal(&self.ir_node.start, &self.semantic)
    }

    pub fn end(&self) -> VersionLiteral {
        create_version_literal(&self.ir_node.end, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VersionTerm = VersionTermStruct;

#[derive(Clone)]
pub struct VersionTermStruct {
    pub(crate) ir_node: ir::VersionTerm,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_version_term(
    ir_node: &ir::VersionTerm,
    semantic: &Arc<SemanticContext>,
) -> VersionTerm {
    VersionTermStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl VersionTermStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operator(&self) -> Option<VersionOperator> {
        self.ir_node
            .operator
            .as_ref()
            .map(|ir_node| create_version_operator(ir_node, &self.semantic))
    }

    pub fn literal(&self) -> VersionLiteral {
        create_version_literal(&self.ir_node.literal, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type WhileStatement = WhileStatementStruct;

#[derive(Clone)]
pub struct WhileStatementStruct {
    pub(crate) ir_node: ir::WhileStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_while_statement(
    ir_node: &ir::WhileStatement,
    semantic: &Arc<SemanticContext>,
) -> WhileStatement {
    WhileStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl WhileStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn condition(&self) -> Expression {
        create_expression(&self.ir_node.condition, &self.semantic)
    }

    pub fn body(&self) -> Statement {
        create_statement(&self.ir_node.body, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulBlock = YulBlockStruct;

#[derive(Clone)]
pub struct YulBlockStruct {
    pub(crate) ir_node: ir::YulBlock,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_block(ir_node: &ir::YulBlock, semantic: &Arc<SemanticContext>) -> YulBlock {
    YulBlockStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulBlockStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn statements(&self) -> YulStatements {
        create_yul_statements(&self.ir_node.statements, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulBreakStatement = YulBreakStatementStruct;

#[derive(Clone)]
pub struct YulBreakStatementStruct {
    pub(crate) ir_node: ir::YulBreakStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_break_statement(
    ir_node: &ir::YulBreakStatement,
    semantic: &Arc<SemanticContext>,
) -> YulBreakStatement {
    YulBreakStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulBreakStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulContinueStatement = YulContinueStatementStruct;

#[derive(Clone)]
pub struct YulContinueStatementStruct {
    pub(crate) ir_node: ir::YulContinueStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_continue_statement(
    ir_node: &ir::YulContinueStatement,
    semantic: &Arc<SemanticContext>,
) -> YulContinueStatement {
    YulContinueStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulContinueStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulDefaultCase = YulDefaultCaseStruct;

#[derive(Clone)]
pub struct YulDefaultCaseStruct {
    pub(crate) ir_node: ir::YulDefaultCase,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_default_case(
    ir_node: &ir::YulDefaultCase,
    semantic: &Arc<SemanticContext>,
) -> YulDefaultCase {
    YulDefaultCaseStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulDefaultCaseStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulForStatement = YulForStatementStruct;

#[derive(Clone)]
pub struct YulForStatementStruct {
    pub(crate) ir_node: ir::YulForStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_for_statement(
    ir_node: &ir::YulForStatement,
    semantic: &Arc<SemanticContext>,
) -> YulForStatement {
    YulForStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulForStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn initialization(&self) -> YulBlock {
        create_yul_block(&self.ir_node.initialization, &self.semantic)
    }

    pub fn condition(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.condition, &self.semantic)
    }

    pub fn iterator(&self) -> YulBlock {
        create_yul_block(&self.ir_node.iterator, &self.semantic)
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulFunctionCallExpression = YulFunctionCallExpressionStruct;

#[derive(Clone)]
pub struct YulFunctionCallExpressionStruct {
    pub(crate) ir_node: ir::YulFunctionCallExpression,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_function_call_expression(
    ir_node: &ir::YulFunctionCallExpression,
    semantic: &Arc<SemanticContext>,
) -> YulFunctionCallExpression {
    YulFunctionCallExpressionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulFunctionCallExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn arguments(&self) -> YulArguments {
        create_yul_arguments(&self.ir_node.arguments, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulFunctionDefinition = YulFunctionDefinitionStruct;

#[derive(Clone)]
pub struct YulFunctionDefinitionStruct {
    pub(crate) ir_node: ir::YulFunctionDefinition,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_function_definition(
    ir_node: &ir::YulFunctionDefinition,
    semantic: &Arc<SemanticContext>,
) -> YulFunctionDefinition {
    YulFunctionDefinitionStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulFunctionDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn parameters(&self) -> YulParameters {
        create_yul_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn returns(&self) -> Option<YulVariableNames> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_yul_variable_names(ir_node, &self.semantic))
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulIfStatement = YulIfStatementStruct;

#[derive(Clone)]
pub struct YulIfStatementStruct {
    pub(crate) ir_node: ir::YulIfStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_if_statement(
    ir_node: &ir::YulIfStatement,
    semantic: &Arc<SemanticContext>,
) -> YulIfStatement {
    YulIfStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulIfStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn condition(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.condition, &self.semantic)
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulLeaveStatement = YulLeaveStatementStruct;

#[derive(Clone)]
pub struct YulLeaveStatementStruct {
    pub(crate) ir_node: ir::YulLeaveStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_leave_statement(
    ir_node: &ir::YulLeaveStatement,
    semantic: &Arc<SemanticContext>,
) -> YulLeaveStatement {
    YulLeaveStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulLeaveStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulSwitchStatement = YulSwitchStatementStruct;

#[derive(Clone)]
pub struct YulSwitchStatementStruct {
    pub(crate) ir_node: ir::YulSwitchStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_switch_statement(
    ir_node: &ir::YulSwitchStatement,
    semantic: &Arc<SemanticContext>,
) -> YulSwitchStatement {
    YulSwitchStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulSwitchStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.expression, &self.semantic)
    }

    pub fn cases(&self) -> YulSwitchCases {
        create_yul_switch_cases(&self.ir_node.cases, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulValueCase = YulValueCaseStruct;

#[derive(Clone)]
pub struct YulValueCaseStruct {
    pub(crate) ir_node: ir::YulValueCase,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_value_case(
    ir_node: &ir::YulValueCase,
    semantic: &Arc<SemanticContext>,
) -> YulValueCase {
    YulValueCaseStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulValueCaseStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn value(&self) -> YulLiteral {
        create_yul_literal(&self.ir_node.value, &self.semantic)
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulVariableAssignmentStatement = YulVariableAssignmentStatementStruct;

#[derive(Clone)]
pub struct YulVariableAssignmentStatementStruct {
    pub(crate) ir_node: ir::YulVariableAssignmentStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_variable_assignment_statement(
    ir_node: &ir::YulVariableAssignmentStatement,
    semantic: &Arc<SemanticContext>,
) -> YulVariableAssignmentStatement {
    YulVariableAssignmentStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulVariableAssignmentStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn variables(&self) -> YulPaths {
        create_yul_paths(&self.ir_node.variables, &self.semantic)
    }

    pub fn expression(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.expression, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulVariableDeclarationStatement = YulVariableDeclarationStatementStruct;

#[derive(Clone)]
pub struct YulVariableDeclarationStatementStruct {
    pub(crate) ir_node: ir::YulVariableDeclarationStatement,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_variable_declaration_statement(
    ir_node: &ir::YulVariableDeclarationStatement,
    semantic: &Arc<SemanticContext>,
) -> YulVariableDeclarationStatement {
    YulVariableDeclarationStatementStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulVariableDeclarationStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn variables(&self) -> YulVariableNames {
        create_yul_variable_names(&self.ir_node.variables, &self.semantic)
    }

    pub fn value(&self) -> Option<YulVariableDeclarationValue> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_yul_variable_declaration_value(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulVariableDeclarationValue = YulVariableDeclarationValueStruct;

#[derive(Clone)]
pub struct YulVariableDeclarationValueStruct {
    pub(crate) ir_node: ir::YulVariableDeclarationValue,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub fn create_yul_variable_declaration_value(
    ir_node: &ir::YulVariableDeclarationValue,
    semantic: &Arc<SemanticContext>,
) -> YulVariableDeclarationValue {
    YulVariableDeclarationValueStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl YulVariableDeclarationValueStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.expression, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

//
// Choices
//

#[derive(Clone)]
pub enum AbicoderVersion {
    AbicoderV1Keyword(AbicoderV1Keyword),
    AbicoderV2Keyword(AbicoderV2Keyword),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_abicoder_version(
    ir_node: &ir::AbicoderVersion,
    semantic: &Arc<SemanticContext>,
) -> AbicoderVersion {
    match ir_node {
        ir::AbicoderVersion::AbicoderV1Keyword(variant) => {
            AbicoderVersion::AbicoderV1Keyword(create_abicoder_v1_keyword(variant, semantic))
        }
        ir::AbicoderVersion::AbicoderV2Keyword(variant) => {
            AbicoderVersion::AbicoderV2Keyword(create_abicoder_v2_keyword(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum AdditiveExpressionOperator {
    Minus(Minus),
    Plus(Plus),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_additive_expression_operator(
    ir_node: &ir::AdditiveExpressionOperator,
    semantic: &Arc<SemanticContext>,
) -> AdditiveExpressionOperator {
    match ir_node {
        ir::AdditiveExpressionOperator::Minus(variant) => {
            AdditiveExpressionOperator::Minus(create_minus(variant, semantic))
        }
        ir::AdditiveExpressionOperator::Plus(variant) => {
            AdditiveExpressionOperator::Plus(create_plus(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum ArgumentsDeclaration {
    PositionalArguments(PositionalArguments),
    NamedArguments(NamedArguments),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_arguments_declaration(
    ir_node: &ir::ArgumentsDeclaration,
    semantic: &Arc<SemanticContext>,
) -> ArgumentsDeclaration {
    match ir_node {
        ir::ArgumentsDeclaration::PositionalArguments(nodes) => {
            ArgumentsDeclaration::PositionalArguments(create_positional_arguments(nodes, semantic))
        }
        ir::ArgumentsDeclaration::NamedArguments(nodes) => {
            ArgumentsDeclaration::NamedArguments(create_named_arguments(nodes, semantic))
        }
    }
}

#[derive(Clone)]
pub enum AssignmentExpressionOperator {
    AmpersandEqual(AmpersandEqual),
    AsteriskEqual(AsteriskEqual),
    BarEqual(BarEqual),
    CaretEqual(CaretEqual),
    Equal(Equal),
    GreaterThanGreaterThanEqual(GreaterThanGreaterThanEqual),
    GreaterThanGreaterThanGreaterThanEqual(GreaterThanGreaterThanGreaterThanEqual),
    LessThanLessThanEqual(LessThanLessThanEqual),
    MinusEqual(MinusEqual),
    PercentEqual(PercentEqual),
    PlusEqual(PlusEqual),
    SlashEqual(SlashEqual),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_assignment_expression_operator(
    ir_node: &ir::AssignmentExpressionOperator,
    semantic: &Arc<SemanticContext>,
) -> AssignmentExpressionOperator {
    match ir_node {
        ir::AssignmentExpressionOperator::AmpersandEqual(variant) => {
            AssignmentExpressionOperator::AmpersandEqual(create_ampersand_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::AsteriskEqual(variant) => {
            AssignmentExpressionOperator::AsteriskEqual(create_asterisk_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::BarEqual(variant) => {
            AssignmentExpressionOperator::BarEqual(create_bar_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::CaretEqual(variant) => {
            AssignmentExpressionOperator::CaretEqual(create_caret_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::Equal(variant) => {
            AssignmentExpressionOperator::Equal(create_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::GreaterThanGreaterThanEqual(variant) => {
            AssignmentExpressionOperator::GreaterThanGreaterThanEqual(
                create_greater_than_greater_than_equal(variant, semantic),
            )
        }
        ir::AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(variant) => {
            AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(
                create_greater_than_greater_than_greater_than_equal(variant, semantic),
            )
        }
        ir::AssignmentExpressionOperator::LessThanLessThanEqual(variant) => {
            AssignmentExpressionOperator::LessThanLessThanEqual(create_less_than_less_than_equal(
                variant, semantic,
            ))
        }
        ir::AssignmentExpressionOperator::MinusEqual(variant) => {
            AssignmentExpressionOperator::MinusEqual(create_minus_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::PercentEqual(variant) => {
            AssignmentExpressionOperator::PercentEqual(create_percent_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::PlusEqual(variant) => {
            AssignmentExpressionOperator::PlusEqual(create_plus_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::SlashEqual(variant) => {
            AssignmentExpressionOperator::SlashEqual(create_slash_equal(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum ContractMember {
    UsingDirective(UsingDirective),
    FunctionDefinition(FunctionDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    StateVariableDefinition(StateVariableDefinition),
    ConstantDefinition(ConstantDefinition),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_contract_member(
    ir_node: &ir::ContractMember,
    semantic: &Arc<SemanticContext>,
) -> ContractMember {
    match ir_node {
        ir::ContractMember::UsingDirective(variant) => {
            ContractMember::UsingDirective(create_using_directive(variant, semantic))
        }
        ir::ContractMember::FunctionDefinition(variant) => {
            ContractMember::FunctionDefinition(create_function_definition(variant, semantic))
        }
        ir::ContractMember::StructDefinition(variant) => {
            ContractMember::StructDefinition(create_struct_definition(variant, semantic))
        }
        ir::ContractMember::EnumDefinition(variant) => {
            ContractMember::EnumDefinition(create_enum_definition(variant, semantic))
        }
        ir::ContractMember::EventDefinition(variant) => {
            ContractMember::EventDefinition(create_event_definition(variant, semantic))
        }
        ir::ContractMember::ErrorDefinition(variant) => {
            ContractMember::ErrorDefinition(create_error_definition(variant, semantic))
        }
        ir::ContractMember::UserDefinedValueTypeDefinition(variant) => {
            ContractMember::UserDefinedValueTypeDefinition(
                create_user_defined_value_type_definition(variant, semantic),
            )
        }
        ir::ContractMember::StateVariableDefinition(variant) => {
            ContractMember::StateVariableDefinition(create_state_variable_definition(
                variant, semantic,
            ))
        }
        ir::ContractMember::ConstantDefinition(variant) => {
            ContractMember::ConstantDefinition(create_constant_definition(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum ElementaryType {
    BoolKeyword(BoolKeyword),
    StringKeyword(StringKeyword),
    AddressType(AddressType),
    BytesKeyword(BytesKeyword),
    IntKeyword(IntKeyword),
    UintKeyword(UintKeyword),
    FixedKeyword(FixedKeyword),
    UfixedKeyword(UfixedKeyword),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_elementary_type(
    ir_node: &ir::ElementaryType,
    semantic: &Arc<SemanticContext>,
) -> ElementaryType {
    match ir_node {
        ir::ElementaryType::BoolKeyword(variant) => {
            ElementaryType::BoolKeyword(create_bool_keyword(variant, semantic))
        }
        ir::ElementaryType::StringKeyword(variant) => {
            ElementaryType::StringKeyword(create_string_keyword(variant, semantic))
        }
        ir::ElementaryType::AddressType(variant) => {
            ElementaryType::AddressType(create_address_type(variant, semantic))
        }
        ir::ElementaryType::BytesKeyword(variant) => {
            ElementaryType::BytesKeyword(create_bytes_keyword(variant, semantic))
        }
        ir::ElementaryType::IntKeyword(variant) => {
            ElementaryType::IntKeyword(create_int_keyword(variant, semantic))
        }
        ir::ElementaryType::UintKeyword(variant) => {
            ElementaryType::UintKeyword(create_uint_keyword(variant, semantic))
        }
        ir::ElementaryType::FixedKeyword(variant) => {
            ElementaryType::FixedKeyword(create_fixed_keyword(variant, semantic))
        }
        ir::ElementaryType::UfixedKeyword(variant) => {
            ElementaryType::UfixedKeyword(create_ufixed_keyword(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum EqualityExpressionOperator {
    BangEqual(BangEqual),
    EqualEqual(EqualEqual),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_equality_expression_operator(
    ir_node: &ir::EqualityExpressionOperator,
    semantic: &Arc<SemanticContext>,
) -> EqualityExpressionOperator {
    match ir_node {
        ir::EqualityExpressionOperator::BangEqual(variant) => {
            EqualityExpressionOperator::BangEqual(create_bang_equal(variant, semantic))
        }
        ir::EqualityExpressionOperator::EqualEqual(variant) => {
            EqualityExpressionOperator::EqualEqual(create_equal_equal(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum ExperimentalFeature {
    ABIEncoderV2Keyword(ABIEncoderV2Keyword),
    SMTCheckerKeyword(SMTCheckerKeyword),
    StringLiteral(StringLiteral),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_experimental_feature(
    ir_node: &ir::ExperimentalFeature,
    semantic: &Arc<SemanticContext>,
) -> ExperimentalFeature {
    match ir_node {
        ir::ExperimentalFeature::ABIEncoderV2Keyword(variant) => {
            ExperimentalFeature::ABIEncoderV2Keyword(create_abi_encoder_v2_keyword(
                variant, semantic,
            ))
        }
        ir::ExperimentalFeature::SMTCheckerKeyword(variant) => {
            ExperimentalFeature::SMTCheckerKeyword(create_smt_checker_keyword(variant, semantic))
        }
        ir::ExperimentalFeature::StringLiteral(variant) => {
            ExperimentalFeature::StringLiteral(create_string_literal(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    AssignmentExpression(AssignmentExpression),
    ConditionalExpression(ConditionalExpression),
    OrExpression(OrExpression),
    AndExpression(AndExpression),
    EqualityExpression(EqualityExpression),
    InequalityExpression(InequalityExpression),
    BitwiseOrExpression(BitwiseOrExpression),
    BitwiseXorExpression(BitwiseXorExpression),
    BitwiseAndExpression(BitwiseAndExpression),
    ShiftExpression(ShiftExpression),
    AdditiveExpression(AdditiveExpression),
    MultiplicativeExpression(MultiplicativeExpression),
    ExponentiationExpression(ExponentiationExpression),
    PostfixExpression(PostfixExpression),
    PrefixExpression(PrefixExpression),
    FunctionCallExpression(FunctionCallExpression),
    CallOptionsExpression(CallOptionsExpression),
    MemberAccessExpression(MemberAccessExpression),
    IndexAccessExpression(IndexAccessExpression),
    NewExpression(NewExpression),
    TupleExpression(TupleExpression),
    TypeExpression(TypeExpression),
    ArrayExpression(ArrayExpression),
    HexNumberExpression(HexNumberExpression),
    DecimalNumberExpression(DecimalNumberExpression),
    StringExpression(StringExpression),
    ElementaryType(ElementaryType),
    PayableKeyword(PayableKeyword),
    ThisKeyword(ThisKeyword),
    SuperKeyword(SuperKeyword),
    TrueKeyword(TrueKeyword),
    FalseKeyword(FalseKeyword),
    Identifier(Identifier),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_expression(
    ir_node: &ir::Expression,
    semantic: &Arc<SemanticContext>,
) -> Expression {
    match ir_node {
        ir::Expression::AssignmentExpression(variant) => {
            Expression::AssignmentExpression(create_assignment_expression(variant, semantic))
        }
        ir::Expression::ConditionalExpression(variant) => {
            Expression::ConditionalExpression(create_conditional_expression(variant, semantic))
        }
        ir::Expression::OrExpression(variant) => {
            Expression::OrExpression(create_or_expression(variant, semantic))
        }
        ir::Expression::AndExpression(variant) => {
            Expression::AndExpression(create_and_expression(variant, semantic))
        }
        ir::Expression::EqualityExpression(variant) => {
            Expression::EqualityExpression(create_equality_expression(variant, semantic))
        }
        ir::Expression::InequalityExpression(variant) => {
            Expression::InequalityExpression(create_inequality_expression(variant, semantic))
        }
        ir::Expression::BitwiseOrExpression(variant) => {
            Expression::BitwiseOrExpression(create_bitwise_or_expression(variant, semantic))
        }
        ir::Expression::BitwiseXorExpression(variant) => {
            Expression::BitwiseXorExpression(create_bitwise_xor_expression(variant, semantic))
        }
        ir::Expression::BitwiseAndExpression(variant) => {
            Expression::BitwiseAndExpression(create_bitwise_and_expression(variant, semantic))
        }
        ir::Expression::ShiftExpression(variant) => {
            Expression::ShiftExpression(create_shift_expression(variant, semantic))
        }
        ir::Expression::AdditiveExpression(variant) => {
            Expression::AdditiveExpression(create_additive_expression(variant, semantic))
        }
        ir::Expression::MultiplicativeExpression(variant) => Expression::MultiplicativeExpression(
            create_multiplicative_expression(variant, semantic),
        ),
        ir::Expression::ExponentiationExpression(variant) => Expression::ExponentiationExpression(
            create_exponentiation_expression(variant, semantic),
        ),
        ir::Expression::PostfixExpression(variant) => {
            Expression::PostfixExpression(create_postfix_expression(variant, semantic))
        }
        ir::Expression::PrefixExpression(variant) => {
            Expression::PrefixExpression(create_prefix_expression(variant, semantic))
        }
        ir::Expression::FunctionCallExpression(variant) => {
            Expression::FunctionCallExpression(create_function_call_expression(variant, semantic))
        }
        ir::Expression::CallOptionsExpression(variant) => {
            Expression::CallOptionsExpression(create_call_options_expression(variant, semantic))
        }
        ir::Expression::MemberAccessExpression(variant) => {
            Expression::MemberAccessExpression(create_member_access_expression(variant, semantic))
        }
        ir::Expression::IndexAccessExpression(variant) => {
            Expression::IndexAccessExpression(create_index_access_expression(variant, semantic))
        }
        ir::Expression::NewExpression(variant) => {
            Expression::NewExpression(create_new_expression(variant, semantic))
        }
        ir::Expression::TupleExpression(variant) => {
            Expression::TupleExpression(create_tuple_expression(variant, semantic))
        }
        ir::Expression::TypeExpression(variant) => {
            Expression::TypeExpression(create_type_expression(variant, semantic))
        }
        ir::Expression::ArrayExpression(variant) => {
            Expression::ArrayExpression(create_array_expression(variant, semantic))
        }
        ir::Expression::HexNumberExpression(variant) => {
            Expression::HexNumberExpression(create_hex_number_expression(variant, semantic))
        }
        ir::Expression::DecimalNumberExpression(variant) => {
            Expression::DecimalNumberExpression(create_decimal_number_expression(variant, semantic))
        }
        ir::Expression::StringExpression(variant) => {
            Expression::StringExpression(create_string_expression(variant, semantic))
        }
        ir::Expression::ElementaryType(variant) => {
            Expression::ElementaryType(create_elementary_type(variant, semantic))
        }
        ir::Expression::PayableKeyword(variant) => {
            Expression::PayableKeyword(create_payable_keyword(variant, semantic))
        }
        ir::Expression::ThisKeyword(variant) => {
            Expression::ThisKeyword(create_this_keyword(variant, semantic))
        }
        ir::Expression::SuperKeyword(variant) => {
            Expression::SuperKeyword(create_super_keyword(variant, semantic))
        }
        ir::Expression::TrueKeyword(variant) => {
            Expression::TrueKeyword(create_true_keyword(variant, semantic))
        }
        ir::Expression::FalseKeyword(variant) => {
            Expression::FalseKeyword(create_false_keyword(variant, semantic))
        }
        ir::Expression::Identifier(variant) => {
            Expression::Identifier(create_identifier(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    Semicolon(Semicolon),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_for_statement_condition(
    ir_node: &ir::ForStatementCondition,
    semantic: &Arc<SemanticContext>,
) -> ForStatementCondition {
    match ir_node {
        ir::ForStatementCondition::ExpressionStatement(variant) => {
            ForStatementCondition::ExpressionStatement(create_expression_statement(
                variant, semantic,
            ))
        }
        ir::ForStatementCondition::Semicolon(variant) => {
            ForStatementCondition::Semicolon(create_semicolon(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum ForStatementInitialization {
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Semicolon(Semicolon),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_for_statement_initialization(
    ir_node: &ir::ForStatementInitialization,
    semantic: &Arc<SemanticContext>,
) -> ForStatementInitialization {
    match ir_node {
        ir::ForStatementInitialization::VariableDeclarationStatement(variant) => {
            ForStatementInitialization::VariableDeclarationStatement(
                create_variable_declaration_statement(variant, semantic),
            )
        }
        ir::ForStatementInitialization::ExpressionStatement(variant) => {
            ForStatementInitialization::ExpressionStatement(create_expression_statement(
                variant, semantic,
            ))
        }
        ir::ForStatementInitialization::Semicolon(variant) => {
            ForStatementInitialization::Semicolon(create_semicolon(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum FunctionKind {
    Regular,
    Constructor,
    Fallback,
    Receive,
    Modifier,
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_function_kind(
    ir_node: &ir::FunctionKind,
    semantic: &Arc<SemanticContext>,
) -> FunctionKind {
    match ir_node {
        ir::FunctionKind::Regular => FunctionKind::Regular,
        ir::FunctionKind::Constructor => FunctionKind::Constructor,
        ir::FunctionKind::Fallback => FunctionKind::Fallback,
        ir::FunctionKind::Receive => FunctionKind::Receive,
        ir::FunctionKind::Modifier => FunctionKind::Modifier,
    }
}

#[derive(Clone)]
pub enum FunctionMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_function_mutability(
    ir_node: &ir::FunctionMutability,
    semantic: &Arc<SemanticContext>,
) -> FunctionMutability {
    match ir_node {
        ir::FunctionMutability::Pure => FunctionMutability::Pure,
        ir::FunctionMutability::View => FunctionMutability::View,
        ir::FunctionMutability::NonPayable => FunctionMutability::NonPayable,
        ir::FunctionMutability::Payable => FunctionMutability::Payable,
    }
}

#[derive(Clone)]
pub enum FunctionVisibility {
    Public,
    Private,
    Internal,
    External,
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_function_visibility(
    ir_node: &ir::FunctionVisibility,
    semantic: &Arc<SemanticContext>,
) -> FunctionVisibility {
    match ir_node {
        ir::FunctionVisibility::Public => FunctionVisibility::Public,
        ir::FunctionVisibility::Private => FunctionVisibility::Private,
        ir::FunctionVisibility::Internal => FunctionVisibility::Internal,
        ir::FunctionVisibility::External => FunctionVisibility::External,
    }
}

#[derive(Clone)]
pub enum ImportClause {
    PathImport(PathImport),
    ImportDeconstruction(ImportDeconstruction),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_import_clause(
    ir_node: &ir::ImportClause,
    semantic: &Arc<SemanticContext>,
) -> ImportClause {
    match ir_node {
        ir::ImportClause::PathImport(variant) => {
            ImportClause::PathImport(create_path_import(variant, semantic))
        }
        ir::ImportClause::ImportDeconstruction(variant) => {
            ImportClause::ImportDeconstruction(create_import_deconstruction(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum InequalityExpressionOperator {
    GreaterThan(GreaterThan),
    GreaterThanEqual(GreaterThanEqual),
    LessThan(LessThan),
    LessThanEqual(LessThanEqual),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_inequality_expression_operator(
    ir_node: &ir::InequalityExpressionOperator,
    semantic: &Arc<SemanticContext>,
) -> InequalityExpressionOperator {
    match ir_node {
        ir::InequalityExpressionOperator::GreaterThan(variant) => {
            InequalityExpressionOperator::GreaterThan(create_greater_than(variant, semantic))
        }
        ir::InequalityExpressionOperator::GreaterThanEqual(variant) => {
            InequalityExpressionOperator::GreaterThanEqual(create_greater_than_equal(
                variant, semantic,
            ))
        }
        ir::InequalityExpressionOperator::LessThan(variant) => {
            InequalityExpressionOperator::LessThan(create_less_than(variant, semantic))
        }
        ir::InequalityExpressionOperator::LessThanEqual(variant) => {
            InequalityExpressionOperator::LessThanEqual(create_less_than_equal(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum MultiplicativeExpressionOperator {
    Asterisk(Asterisk),
    Percent(Percent),
    Slash(Slash),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_multiplicative_expression_operator(
    ir_node: &ir::MultiplicativeExpressionOperator,
    semantic: &Arc<SemanticContext>,
) -> MultiplicativeExpressionOperator {
    match ir_node {
        ir::MultiplicativeExpressionOperator::Asterisk(variant) => {
            MultiplicativeExpressionOperator::Asterisk(create_asterisk(variant, semantic))
        }
        ir::MultiplicativeExpressionOperator::Percent(variant) => {
            MultiplicativeExpressionOperator::Percent(create_percent(variant, semantic))
        }
        ir::MultiplicativeExpressionOperator::Slash(variant) => {
            MultiplicativeExpressionOperator::Slash(create_slash(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum NumberUnit {
    WeiKeyword(WeiKeyword),
    GweiKeyword(GweiKeyword),
    EtherKeyword(EtherKeyword),
    SecondsKeyword(SecondsKeyword),
    MinutesKeyword(MinutesKeyword),
    HoursKeyword(HoursKeyword),
    DaysKeyword(DaysKeyword),
    WeeksKeyword(WeeksKeyword),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_number_unit(
    ir_node: &ir::NumberUnit,
    semantic: &Arc<SemanticContext>,
) -> NumberUnit {
    match ir_node {
        ir::NumberUnit::WeiKeyword(variant) => {
            NumberUnit::WeiKeyword(create_wei_keyword(variant, semantic))
        }
        ir::NumberUnit::GweiKeyword(variant) => {
            NumberUnit::GweiKeyword(create_gwei_keyword(variant, semantic))
        }
        ir::NumberUnit::EtherKeyword(variant) => {
            NumberUnit::EtherKeyword(create_ether_keyword(variant, semantic))
        }
        ir::NumberUnit::SecondsKeyword(variant) => {
            NumberUnit::SecondsKeyword(create_seconds_keyword(variant, semantic))
        }
        ir::NumberUnit::MinutesKeyword(variant) => {
            NumberUnit::MinutesKeyword(create_minutes_keyword(variant, semantic))
        }
        ir::NumberUnit::HoursKeyword(variant) => {
            NumberUnit::HoursKeyword(create_hours_keyword(variant, semantic))
        }
        ir::NumberUnit::DaysKeyword(variant) => {
            NumberUnit::DaysKeyword(create_days_keyword(variant, semantic))
        }
        ir::NumberUnit::WeeksKeyword(variant) => {
            NumberUnit::WeeksKeyword(create_weeks_keyword(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum PostfixExpressionOperator {
    MinusMinus(MinusMinus),
    PlusPlus(PlusPlus),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_postfix_expression_operator(
    ir_node: &ir::PostfixExpressionOperator,
    semantic: &Arc<SemanticContext>,
) -> PostfixExpressionOperator {
    match ir_node {
        ir::PostfixExpressionOperator::MinusMinus(variant) => {
            PostfixExpressionOperator::MinusMinus(create_minus_minus(variant, semantic))
        }
        ir::PostfixExpressionOperator::PlusPlus(variant) => {
            PostfixExpressionOperator::PlusPlus(create_plus_plus(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum Pragma {
    VersionPragma(VersionPragma),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_pragma(ir_node: &ir::Pragma, semantic: &Arc<SemanticContext>) -> Pragma {
    match ir_node {
        ir::Pragma::VersionPragma(variant) => {
            Pragma::VersionPragma(create_version_pragma(variant, semantic))
        }
        ir::Pragma::AbicoderPragma(variant) => {
            Pragma::AbicoderPragma(create_abicoder_pragma(variant, semantic))
        }
        ir::Pragma::ExperimentalPragma(variant) => {
            Pragma::ExperimentalPragma(create_experimental_pragma(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum PrefixExpressionOperator {
    Bang(Bang),
    DeleteKeyword(DeleteKeyword),
    Minus(Minus),
    MinusMinus(MinusMinus),
    PlusPlus(PlusPlus),
    Tilde(Tilde),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_prefix_expression_operator(
    ir_node: &ir::PrefixExpressionOperator,
    semantic: &Arc<SemanticContext>,
) -> PrefixExpressionOperator {
    match ir_node {
        ir::PrefixExpressionOperator::Bang(variant) => {
            PrefixExpressionOperator::Bang(create_bang(variant, semantic))
        }
        ir::PrefixExpressionOperator::DeleteKeyword(variant) => {
            PrefixExpressionOperator::DeleteKeyword(create_delete_keyword(variant, semantic))
        }
        ir::PrefixExpressionOperator::Minus(variant) => {
            PrefixExpressionOperator::Minus(create_minus(variant, semantic))
        }
        ir::PrefixExpressionOperator::MinusMinus(variant) => {
            PrefixExpressionOperator::MinusMinus(create_minus_minus(variant, semantic))
        }
        ir::PrefixExpressionOperator::PlusPlus(variant) => {
            PrefixExpressionOperator::PlusPlus(create_plus_plus(variant, semantic))
        }
        ir::PrefixExpressionOperator::Tilde(variant) => {
            PrefixExpressionOperator::Tilde(create_tilde(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum ShiftExpressionOperator {
    GreaterThanGreaterThan(GreaterThanGreaterThan),
    GreaterThanGreaterThanGreaterThan(GreaterThanGreaterThanGreaterThan),
    LessThanLessThan(LessThanLessThan),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_shift_expression_operator(
    ir_node: &ir::ShiftExpressionOperator,
    semantic: &Arc<SemanticContext>,
) -> ShiftExpressionOperator {
    match ir_node {
        ir::ShiftExpressionOperator::GreaterThanGreaterThan(variant) => {
            ShiftExpressionOperator::GreaterThanGreaterThan(create_greater_than_greater_than(
                variant, semantic,
            ))
        }
        ir::ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(variant) => {
            ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(
                create_greater_than_greater_than_greater_than(variant, semantic),
            )
        }
        ir::ShiftExpressionOperator::LessThanLessThan(variant) => {
            ShiftExpressionOperator::LessThanLessThan(create_less_than_less_than(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum SourceUnitMember {
    PragmaDirective(PragmaDirective),
    ImportClause(ImportClause),
    ContractDefinition(ContractDefinition),
    InterfaceDefinition(InterfaceDefinition),
    LibraryDefinition(LibraryDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    FunctionDefinition(FunctionDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingDirective(UsingDirective),
    EventDefinition(EventDefinition),
    ConstantDefinition(ConstantDefinition),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_source_unit_member(
    ir_node: &ir::SourceUnitMember,
    semantic: &Arc<SemanticContext>,
) -> SourceUnitMember {
    match ir_node {
        ir::SourceUnitMember::PragmaDirective(variant) => {
            SourceUnitMember::PragmaDirective(create_pragma_directive(variant, semantic))
        }
        ir::SourceUnitMember::ImportClause(variant) => {
            SourceUnitMember::ImportClause(create_import_clause(variant, semantic))
        }
        ir::SourceUnitMember::ContractDefinition(variant) => {
            SourceUnitMember::ContractDefinition(create_contract_definition(variant, semantic))
        }
        ir::SourceUnitMember::InterfaceDefinition(variant) => {
            SourceUnitMember::InterfaceDefinition(create_interface_definition(variant, semantic))
        }
        ir::SourceUnitMember::LibraryDefinition(variant) => {
            SourceUnitMember::LibraryDefinition(create_library_definition(variant, semantic))
        }
        ir::SourceUnitMember::StructDefinition(variant) => {
            SourceUnitMember::StructDefinition(create_struct_definition(variant, semantic))
        }
        ir::SourceUnitMember::EnumDefinition(variant) => {
            SourceUnitMember::EnumDefinition(create_enum_definition(variant, semantic))
        }
        ir::SourceUnitMember::FunctionDefinition(variant) => {
            SourceUnitMember::FunctionDefinition(create_function_definition(variant, semantic))
        }
        ir::SourceUnitMember::ErrorDefinition(variant) => {
            SourceUnitMember::ErrorDefinition(create_error_definition(variant, semantic))
        }
        ir::SourceUnitMember::UserDefinedValueTypeDefinition(variant) => {
            SourceUnitMember::UserDefinedValueTypeDefinition(
                create_user_defined_value_type_definition(variant, semantic),
            )
        }
        ir::SourceUnitMember::UsingDirective(variant) => {
            SourceUnitMember::UsingDirective(create_using_directive(variant, semantic))
        }
        ir::SourceUnitMember::EventDefinition(variant) => {
            SourceUnitMember::EventDefinition(create_event_definition(variant, semantic))
        }
        ir::SourceUnitMember::ConstantDefinition(variant) => {
            SourceUnitMember::ConstantDefinition(create_constant_definition(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum StateVariableMutability {
    Mutable,
    Constant,
    Immutable,
    Transient,
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_state_variable_mutability(
    ir_node: &ir::StateVariableMutability,
    semantic: &Arc<SemanticContext>,
) -> StateVariableMutability {
    match ir_node {
        ir::StateVariableMutability::Mutable => StateVariableMutability::Mutable,
        ir::StateVariableMutability::Constant => StateVariableMutability::Constant,
        ir::StateVariableMutability::Immutable => StateVariableMutability::Immutable,
        ir::StateVariableMutability::Transient => StateVariableMutability::Transient,
    }
}

#[derive(Clone)]
pub enum StateVariableVisibility {
    Public,
    Private,
    Internal,
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_state_variable_visibility(
    ir_node: &ir::StateVariableVisibility,
    semantic: &Arc<SemanticContext>,
) -> StateVariableVisibility {
    match ir_node {
        ir::StateVariableVisibility::Public => StateVariableVisibility::Public,
        ir::StateVariableVisibility::Private => StateVariableVisibility::Private,
        ir::StateVariableVisibility::Internal => StateVariableVisibility::Internal,
    }
}

#[derive(Clone)]
pub enum Statement {
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ContinueStatement(ContinueStatement),
    BreakStatement(BreakStatement),
    ReturnStatement(ReturnStatement),
    EmitStatement(EmitStatement),
    TryStatement(TryStatement),
    RevertStatement(RevertStatement),
    AssemblyStatement(AssemblyStatement),
    Block(Block),
    UncheckedBlock(UncheckedBlock),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_statement(
    ir_node: &ir::Statement,
    semantic: &Arc<SemanticContext>,
) -> Statement {
    match ir_node {
        ir::Statement::IfStatement(variant) => {
            Statement::IfStatement(create_if_statement(variant, semantic))
        }
        ir::Statement::ForStatement(variant) => {
            Statement::ForStatement(create_for_statement(variant, semantic))
        }
        ir::Statement::WhileStatement(variant) => {
            Statement::WhileStatement(create_while_statement(variant, semantic))
        }
        ir::Statement::DoWhileStatement(variant) => {
            Statement::DoWhileStatement(create_do_while_statement(variant, semantic))
        }
        ir::Statement::ContinueStatement(variant) => {
            Statement::ContinueStatement(create_continue_statement(variant, semantic))
        }
        ir::Statement::BreakStatement(variant) => {
            Statement::BreakStatement(create_break_statement(variant, semantic))
        }
        ir::Statement::ReturnStatement(variant) => {
            Statement::ReturnStatement(create_return_statement(variant, semantic))
        }
        ir::Statement::EmitStatement(variant) => {
            Statement::EmitStatement(create_emit_statement(variant, semantic))
        }
        ir::Statement::TryStatement(variant) => {
            Statement::TryStatement(create_try_statement(variant, semantic))
        }
        ir::Statement::RevertStatement(variant) => {
            Statement::RevertStatement(create_revert_statement(variant, semantic))
        }
        ir::Statement::AssemblyStatement(variant) => {
            Statement::AssemblyStatement(create_assembly_statement(variant, semantic))
        }
        ir::Statement::Block(variant) => Statement::Block(create_block(variant, semantic)),
        ir::Statement::UncheckedBlock(variant) => {
            Statement::UncheckedBlock(create_unchecked_block(variant, semantic))
        }
        ir::Statement::VariableDeclarationStatement(variant) => {
            Statement::VariableDeclarationStatement(create_variable_declaration_statement(
                variant, semantic,
            ))
        }
        ir::Statement::ExpressionStatement(variant) => {
            Statement::ExpressionStatement(create_expression_statement(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum StorageLocation {
    MemoryKeyword(MemoryKeyword),
    StorageKeyword(StorageKeyword),
    CallDataKeyword(CallDataKeyword),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_storage_location(
    ir_node: &ir::StorageLocation,
    semantic: &Arc<SemanticContext>,
) -> StorageLocation {
    match ir_node {
        ir::StorageLocation::MemoryKeyword(variant) => {
            StorageLocation::MemoryKeyword(create_memory_keyword(variant, semantic))
        }
        ir::StorageLocation::StorageKeyword(variant) => {
            StorageLocation::StorageKeyword(create_storage_keyword(variant, semantic))
        }
        ir::StorageLocation::CallDataKeyword(variant) => {
            StorageLocation::CallDataKeyword(create_call_data_keyword(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum StringExpression {
    StringLiterals(StringLiterals),
    HexStringLiterals(HexStringLiterals),
    UnicodeStringLiterals(UnicodeStringLiterals),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_string_expression(
    ir_node: &ir::StringExpression,
    semantic: &Arc<SemanticContext>,
) -> StringExpression {
    match ir_node {
        ir::StringExpression::StringLiterals(nodes) => {
            StringExpression::StringLiterals(create_string_literals(nodes, semantic))
        }
        ir::StringExpression::HexStringLiterals(nodes) => {
            StringExpression::HexStringLiterals(create_hex_string_literals(nodes, semantic))
        }
        ir::StringExpression::UnicodeStringLiterals(nodes) => {
            StringExpression::UnicodeStringLiterals(create_unicode_string_literals(nodes, semantic))
        }
    }
}

#[derive(Clone)]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    FunctionType(FunctionType),
    MappingType(MappingType),
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_type_name(
    ir_node: &ir::TypeName,
    semantic: &Arc<SemanticContext>,
) -> TypeName {
    match ir_node {
        ir::TypeName::ArrayTypeName(variant) => {
            TypeName::ArrayTypeName(create_array_type_name(variant, semantic))
        }
        ir::TypeName::FunctionType(variant) => {
            TypeName::FunctionType(create_function_type(variant, semantic))
        }
        ir::TypeName::MappingType(variant) => {
            TypeName::MappingType(create_mapping_type(variant, semantic))
        }
        ir::TypeName::ElementaryType(variant) => {
            TypeName::ElementaryType(create_elementary_type(variant, semantic))
        }
        ir::TypeName::IdentifierPath(nodes) => {
            TypeName::IdentifierPath(create_identifier_path(nodes, semantic))
        }
    }
}

#[derive(Clone)]
pub enum UsingClause {
    IdentifierPath(IdentifierPath),
    UsingDeconstruction(UsingDeconstruction),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_using_clause(
    ir_node: &ir::UsingClause,
    semantic: &Arc<SemanticContext>,
) -> UsingClause {
    match ir_node {
        ir::UsingClause::IdentifierPath(nodes) => {
            UsingClause::IdentifierPath(create_identifier_path(nodes, semantic))
        }
        ir::UsingClause::UsingDeconstruction(variant) => {
            UsingClause::UsingDeconstruction(create_using_deconstruction(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum UsingOperator {
    Ampersand(Ampersand),
    Asterisk(Asterisk),
    BangEqual(BangEqual),
    Bar(Bar),
    Caret(Caret),
    EqualEqual(EqualEqual),
    GreaterThan(GreaterThan),
    GreaterThanEqual(GreaterThanEqual),
    LessThan(LessThan),
    LessThanEqual(LessThanEqual),
    Minus(Minus),
    Percent(Percent),
    Plus(Plus),
    Slash(Slash),
    Tilde(Tilde),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_using_operator(
    ir_node: &ir::UsingOperator,
    semantic: &Arc<SemanticContext>,
) -> UsingOperator {
    match ir_node {
        ir::UsingOperator::Ampersand(variant) => {
            UsingOperator::Ampersand(create_ampersand(variant, semantic))
        }
        ir::UsingOperator::Asterisk(variant) => {
            UsingOperator::Asterisk(create_asterisk(variant, semantic))
        }
        ir::UsingOperator::BangEqual(variant) => {
            UsingOperator::BangEqual(create_bang_equal(variant, semantic))
        }
        ir::UsingOperator::Bar(variant) => UsingOperator::Bar(create_bar(variant, semantic)),
        ir::UsingOperator::Caret(variant) => UsingOperator::Caret(create_caret(variant, semantic)),
        ir::UsingOperator::EqualEqual(variant) => {
            UsingOperator::EqualEqual(create_equal_equal(variant, semantic))
        }
        ir::UsingOperator::GreaterThan(variant) => {
            UsingOperator::GreaterThan(create_greater_than(variant, semantic))
        }
        ir::UsingOperator::GreaterThanEqual(variant) => {
            UsingOperator::GreaterThanEqual(create_greater_than_equal(variant, semantic))
        }
        ir::UsingOperator::LessThan(variant) => {
            UsingOperator::LessThan(create_less_than(variant, semantic))
        }
        ir::UsingOperator::LessThanEqual(variant) => {
            UsingOperator::LessThanEqual(create_less_than_equal(variant, semantic))
        }
        ir::UsingOperator::Minus(variant) => UsingOperator::Minus(create_minus(variant, semantic)),
        ir::UsingOperator::Percent(variant) => {
            UsingOperator::Percent(create_percent(variant, semantic))
        }
        ir::UsingOperator::Plus(variant) => UsingOperator::Plus(create_plus(variant, semantic)),
        ir::UsingOperator::Slash(variant) => UsingOperator::Slash(create_slash(variant, semantic)),
        ir::UsingOperator::Tilde(variant) => UsingOperator::Tilde(create_tilde(variant, semantic)),
    }
}

#[derive(Clone)]
pub enum UsingTarget {
    TypeName(TypeName),
    Asterisk(Asterisk),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_using_target(
    ir_node: &ir::UsingTarget,
    semantic: &Arc<SemanticContext>,
) -> UsingTarget {
    match ir_node {
        ir::UsingTarget::TypeName(variant) => {
            UsingTarget::TypeName(create_type_name(variant, semantic))
        }
        ir::UsingTarget::Asterisk(variant) => {
            UsingTarget::Asterisk(create_asterisk(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum VariableDeclarationTarget {
    SingleTypedDeclaration(SingleTypedDeclaration),
    MultiTypedDeclaration(MultiTypedDeclaration),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_variable_declaration_target(
    ir_node: &ir::VariableDeclarationTarget,
    semantic: &Arc<SemanticContext>,
) -> VariableDeclarationTarget {
    match ir_node {
        ir::VariableDeclarationTarget::SingleTypedDeclaration(variant) => {
            VariableDeclarationTarget::SingleTypedDeclaration(create_single_typed_declaration(
                variant, semantic,
            ))
        }
        ir::VariableDeclarationTarget::MultiTypedDeclaration(variant) => {
            VariableDeclarationTarget::MultiTypedDeclaration(create_multi_typed_declaration(
                variant, semantic,
            ))
        }
    }
}

#[derive(Clone)]
pub enum VersionExpression {
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_version_expression(
    ir_node: &ir::VersionExpression,
    semantic: &Arc<SemanticContext>,
) -> VersionExpression {
    match ir_node {
        ir::VersionExpression::VersionRange(variant) => {
            VersionExpression::VersionRange(create_version_range(variant, semantic))
        }
        ir::VersionExpression::VersionTerm(variant) => {
            VersionExpression::VersionTerm(create_version_term(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum VersionLiteral {
    SimpleVersionLiteral(SimpleVersionLiteral),
    StringLiteral(StringLiteral),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_version_literal(
    ir_node: &ir::VersionLiteral,
    semantic: &Arc<SemanticContext>,
) -> VersionLiteral {
    match ir_node {
        ir::VersionLiteral::SimpleVersionLiteral(nodes) => {
            VersionLiteral::SimpleVersionLiteral(create_simple_version_literal(nodes, semantic))
        }
        ir::VersionLiteral::StringLiteral(variant) => {
            VersionLiteral::StringLiteral(create_string_literal(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum VersionOperator {
    PragmaCaret(PragmaCaret),
    PragmaTilde(PragmaTilde),
    PragmaEqual(PragmaEqual),
    PragmaLessThan(PragmaLessThan),
    PragmaGreaterThan(PragmaGreaterThan),
    PragmaLessThanEqual(PragmaLessThanEqual),
    PragmaGreaterThanEqual(PragmaGreaterThanEqual),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_version_operator(
    ir_node: &ir::VersionOperator,
    semantic: &Arc<SemanticContext>,
) -> VersionOperator {
    match ir_node {
        ir::VersionOperator::PragmaCaret(variant) => {
            VersionOperator::PragmaCaret(create_pragma_caret(variant, semantic))
        }
        ir::VersionOperator::PragmaTilde(variant) => {
            VersionOperator::PragmaTilde(create_pragma_tilde(variant, semantic))
        }
        ir::VersionOperator::PragmaEqual(variant) => {
            VersionOperator::PragmaEqual(create_pragma_equal(variant, semantic))
        }
        ir::VersionOperator::PragmaLessThan(variant) => {
            VersionOperator::PragmaLessThan(create_pragma_less_than(variant, semantic))
        }
        ir::VersionOperator::PragmaGreaterThan(variant) => {
            VersionOperator::PragmaGreaterThan(create_pragma_greater_than(variant, semantic))
        }
        ir::VersionOperator::PragmaLessThanEqual(variant) => {
            VersionOperator::PragmaLessThanEqual(create_pragma_less_than_equal(variant, semantic))
        }
        ir::VersionOperator::PragmaGreaterThanEqual(variant) => {
            VersionOperator::PragmaGreaterThanEqual(create_pragma_greater_than_equal(
                variant, semantic,
            ))
        }
    }
}

#[derive(Clone)]
pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_yul_expression(
    ir_node: &ir::YulExpression,
    semantic: &Arc<SemanticContext>,
) -> YulExpression {
    match ir_node {
        ir::YulExpression::YulFunctionCallExpression(variant) => {
            YulExpression::YulFunctionCallExpression(create_yul_function_call_expression(
                variant, semantic,
            ))
        }
        ir::YulExpression::YulLiteral(variant) => {
            YulExpression::YulLiteral(create_yul_literal(variant, semantic))
        }
        ir::YulExpression::YulPath(nodes) => {
            YulExpression::YulPath(create_yul_path(nodes, semantic))
        }
    }
}

#[derive(Clone)]
pub enum YulLiteral {
    TrueKeyword(TrueKeyword),
    FalseKeyword(FalseKeyword),
    DecimalLiteral(DecimalLiteral),
    HexLiteral(HexLiteral),
    HexStringLiteral(HexStringLiteral),
    StringLiteral(StringLiteral),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_yul_literal(
    ir_node: &ir::YulLiteral,
    semantic: &Arc<SemanticContext>,
) -> YulLiteral {
    match ir_node {
        ir::YulLiteral::TrueKeyword(variant) => {
            YulLiteral::TrueKeyword(create_true_keyword(variant, semantic))
        }
        ir::YulLiteral::FalseKeyword(variant) => {
            YulLiteral::FalseKeyword(create_false_keyword(variant, semantic))
        }
        ir::YulLiteral::DecimalLiteral(variant) => {
            YulLiteral::DecimalLiteral(create_decimal_literal(variant, semantic))
        }
        ir::YulLiteral::HexLiteral(variant) => {
            YulLiteral::HexLiteral(create_hex_literal(variant, semantic))
        }
        ir::YulLiteral::HexStringLiteral(variant) => {
            YulLiteral::HexStringLiteral(create_hex_string_literal(variant, semantic))
        }
        ir::YulLiteral::StringLiteral(variant) => {
            YulLiteral::StringLiteral(create_string_literal(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum YulStatement {
    YulBlock(YulBlock),
    YulFunctionDefinition(YulFunctionDefinition),
    YulIfStatement(YulIfStatement),
    YulForStatement(YulForStatement),
    YulSwitchStatement(YulSwitchStatement),
    YulLeaveStatement(YulLeaveStatement),
    YulBreakStatement(YulBreakStatement),
    YulContinueStatement(YulContinueStatement),
    YulVariableAssignmentStatement(YulVariableAssignmentStatement),
    YulVariableDeclarationStatement(YulVariableDeclarationStatement),
    YulExpression(YulExpression),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_yul_statement(
    ir_node: &ir::YulStatement,
    semantic: &Arc<SemanticContext>,
) -> YulStatement {
    match ir_node {
        ir::YulStatement::YulBlock(variant) => {
            YulStatement::YulBlock(create_yul_block(variant, semantic))
        }
        ir::YulStatement::YulFunctionDefinition(variant) => {
            YulStatement::YulFunctionDefinition(create_yul_function_definition(variant, semantic))
        }
        ir::YulStatement::YulIfStatement(variant) => {
            YulStatement::YulIfStatement(create_yul_if_statement(variant, semantic))
        }
        ir::YulStatement::YulForStatement(variant) => {
            YulStatement::YulForStatement(create_yul_for_statement(variant, semantic))
        }
        ir::YulStatement::YulSwitchStatement(variant) => {
            YulStatement::YulSwitchStatement(create_yul_switch_statement(variant, semantic))
        }
        ir::YulStatement::YulLeaveStatement(variant) => {
            YulStatement::YulLeaveStatement(create_yul_leave_statement(variant, semantic))
        }
        ir::YulStatement::YulBreakStatement(variant) => {
            YulStatement::YulBreakStatement(create_yul_break_statement(variant, semantic))
        }
        ir::YulStatement::YulContinueStatement(variant) => {
            YulStatement::YulContinueStatement(create_yul_continue_statement(variant, semantic))
        }
        ir::YulStatement::YulVariableAssignmentStatement(variant) => {
            YulStatement::YulVariableAssignmentStatement(create_yul_variable_assignment_statement(
                variant, semantic,
            ))
        }
        ir::YulStatement::YulVariableDeclarationStatement(variant) => {
            YulStatement::YulVariableDeclarationStatement(
                create_yul_variable_declaration_statement(variant, semantic),
            )
        }
        ir::YulStatement::YulExpression(variant) => {
            YulStatement::YulExpression(create_yul_expression(variant, semantic))
        }
    }
}

#[derive(Clone)]
pub enum YulSwitchCase {
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_yul_switch_case(
    ir_node: &ir::YulSwitchCase,
    semantic: &Arc<SemanticContext>,
) -> YulSwitchCase {
    match ir_node {
        ir::YulSwitchCase::YulDefaultCase(variant) => {
            YulSwitchCase::YulDefaultCase(create_yul_default_case(variant, semantic))
        }
        ir::YulSwitchCase::YulValueCase(variant) => {
            YulSwitchCase::YulValueCase(create_yul_value_case(variant, semantic))
        }
    }
}

//
// Repeated & Separated
//

pub type ArrayValues = ArrayValuesStruct;

pub(crate) fn create_array_values(
    nodes: &ir::ArrayValues,
    semantic: &Arc<SemanticContext>,
) -> ArrayValues {
    ArrayValuesStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct ArrayValuesStruct {
    pub(crate) ir_nodes: ir::ArrayValues,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl ArrayValuesStruct {
    pub fn iter(&self) -> impl Iterator<Item = Expression> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type CallOptions = CallOptionsStruct;

pub(crate) fn create_call_options(
    nodes: &ir::CallOptions,
    semantic: &Arc<SemanticContext>,
) -> CallOptions {
    CallOptionsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct CallOptionsStruct {
    pub(crate) ir_nodes: ir::CallOptions,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl CallOptionsStruct {
    pub fn iter(&self) -> impl Iterator<Item = NamedArgument> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_named_argument(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type CatchClauses = CatchClausesStruct;

pub(crate) fn create_catch_clauses(
    nodes: &ir::CatchClauses,
    semantic: &Arc<SemanticContext>,
) -> CatchClauses {
    CatchClausesStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct CatchClausesStruct {
    pub(crate) ir_nodes: ir::CatchClauses,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl CatchClausesStruct {
    pub fn iter(&self) -> impl Iterator<Item = CatchClause> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_catch_clause(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type ContractMembers = ContractMembersStruct;

pub(crate) fn create_contract_members(
    nodes: &ir::ContractMembers,
    semantic: &Arc<SemanticContext>,
) -> ContractMembers {
    ContractMembersStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct ContractMembersStruct {
    pub(crate) ir_nodes: ir::ContractMembers,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl ContractMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = ContractMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_contract_member(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type EnumMembers = EnumMembersStruct;

pub(crate) fn create_enum_members(
    nodes: &ir::EnumMembers,
    semantic: &Arc<SemanticContext>,
) -> EnumMembers {
    EnumMembersStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct EnumMembersStruct {
    pub(crate) ir_nodes: ir::EnumMembers,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl EnumMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type HexStringLiterals = HexStringLiteralsStruct;

pub(crate) fn create_hex_string_literals(
    nodes: &ir::HexStringLiterals,
    semantic: &Arc<SemanticContext>,
) -> HexStringLiterals {
    HexStringLiteralsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct HexStringLiteralsStruct {
    pub(crate) ir_nodes: ir::HexStringLiterals,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl HexStringLiteralsStruct {
    pub fn iter(&self) -> impl Iterator<Item = HexStringLiteral> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_hex_string_literal(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type IdentifierPath = IdentifierPathStruct;

pub(crate) fn create_identifier_path(
    nodes: &ir::IdentifierPath,
    semantic: &Arc<SemanticContext>,
) -> IdentifierPath {
    IdentifierPathStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct IdentifierPathStruct {
    pub(crate) ir_nodes: ir::IdentifierPath,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl IdentifierPathStruct {
    pub fn iter(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type ImportDeconstructionSymbols = ImportDeconstructionSymbolsStruct;

pub(crate) fn create_import_deconstruction_symbols(
    nodes: &ir::ImportDeconstructionSymbols,
    semantic: &Arc<SemanticContext>,
) -> ImportDeconstructionSymbols {
    ImportDeconstructionSymbolsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct ImportDeconstructionSymbolsStruct {
    pub(crate) ir_nodes: ir::ImportDeconstructionSymbols,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl ImportDeconstructionSymbolsStruct {
    pub fn iter(&self) -> impl Iterator<Item = ImportDeconstructionSymbol> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_import_deconstruction_symbol(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type InheritanceTypes = InheritanceTypesStruct;

pub(crate) fn create_inheritance_types(
    nodes: &ir::InheritanceTypes,
    semantic: &Arc<SemanticContext>,
) -> InheritanceTypes {
    InheritanceTypesStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct InheritanceTypesStruct {
    pub(crate) ir_nodes: ir::InheritanceTypes,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl InheritanceTypesStruct {
    pub fn iter(&self) -> impl Iterator<Item = InheritanceType> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_inheritance_type(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type InterfaceMembers = InterfaceMembersStruct;

pub(crate) fn create_interface_members(
    nodes: &ir::InterfaceMembers,
    semantic: &Arc<SemanticContext>,
) -> InterfaceMembers {
    InterfaceMembersStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct InterfaceMembersStruct {
    pub(crate) ir_nodes: ir::InterfaceMembers,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl InterfaceMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = ContractMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_contract_member(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type LibraryMembers = LibraryMembersStruct;

pub(crate) fn create_library_members(
    nodes: &ir::LibraryMembers,
    semantic: &Arc<SemanticContext>,
) -> LibraryMembers {
    LibraryMembersStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct LibraryMembersStruct {
    pub(crate) ir_nodes: ir::LibraryMembers,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl LibraryMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = ContractMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_contract_member(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type ModifierInvocations = ModifierInvocationsStruct;

pub(crate) fn create_modifier_invocations(
    nodes: &ir::ModifierInvocations,
    semantic: &Arc<SemanticContext>,
) -> ModifierInvocations {
    ModifierInvocationsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct ModifierInvocationsStruct {
    pub(crate) ir_nodes: ir::ModifierInvocations,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl ModifierInvocationsStruct {
    pub fn iter(&self) -> impl Iterator<Item = ModifierInvocation> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_modifier_invocation(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type MultiTypedDeclarationElements = MultiTypedDeclarationElementsStruct;

pub(crate) fn create_multi_typed_declaration_elements(
    nodes: &ir::MultiTypedDeclarationElements,
    semantic: &Arc<SemanticContext>,
) -> MultiTypedDeclarationElements {
    MultiTypedDeclarationElementsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct MultiTypedDeclarationElementsStruct {
    pub(crate) ir_nodes: ir::MultiTypedDeclarationElements,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl MultiTypedDeclarationElementsStruct {
    pub fn iter(&self) -> impl Iterator<Item = MultiTypedDeclarationElement> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_multi_typed_declaration_element(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type NamedArguments = NamedArgumentsStruct;

pub(crate) fn create_named_arguments(
    nodes: &ir::NamedArguments,
    semantic: &Arc<SemanticContext>,
) -> NamedArguments {
    NamedArgumentsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct NamedArgumentsStruct {
    pub(crate) ir_nodes: ir::NamedArguments,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl NamedArgumentsStruct {
    pub fn iter(&self) -> impl Iterator<Item = NamedArgument> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_named_argument(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type OverridePaths = OverridePathsStruct;

pub(crate) fn create_override_paths(
    nodes: &ir::OverridePaths,
    semantic: &Arc<SemanticContext>,
) -> OverridePaths {
    OverridePathsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct OverridePathsStruct {
    pub(crate) ir_nodes: ir::OverridePaths,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl OverridePathsStruct {
    pub fn iter(&self) -> impl Iterator<Item = IdentifierPath> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier_path(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type Parameters = ParametersStruct;

pub(crate) fn create_parameters(
    nodes: &ir::Parameters,
    semantic: &Arc<SemanticContext>,
) -> Parameters {
    ParametersStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct ParametersStruct {
    pub(crate) ir_nodes: ir::Parameters,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl ParametersStruct {
    pub fn iter(&self) -> impl Iterator<Item = Parameter> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_parameter(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type PositionalArguments = PositionalArgumentsStruct;

pub(crate) fn create_positional_arguments(
    nodes: &ir::PositionalArguments,
    semantic: &Arc<SemanticContext>,
) -> PositionalArguments {
    PositionalArgumentsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct PositionalArgumentsStruct {
    pub(crate) ir_nodes: ir::PositionalArguments,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl PositionalArgumentsStruct {
    pub fn iter(&self) -> impl Iterator<Item = Expression> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type SimpleVersionLiteral = SimpleVersionLiteralStruct;

pub(crate) fn create_simple_version_literal(
    nodes: &ir::SimpleVersionLiteral,
    semantic: &Arc<SemanticContext>,
) -> SimpleVersionLiteral {
    SimpleVersionLiteralStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct SimpleVersionLiteralStruct {
    pub(crate) ir_nodes: ir::SimpleVersionLiteral,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl SimpleVersionLiteralStruct {
    pub fn iter(&self) -> impl Iterator<Item = VersionSpecifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_version_specifier(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type SourceUnitMembers = SourceUnitMembersStruct;

pub(crate) fn create_source_unit_members(
    nodes: &ir::SourceUnitMembers,
    semantic: &Arc<SemanticContext>,
) -> SourceUnitMembers {
    SourceUnitMembersStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct SourceUnitMembersStruct {
    pub(crate) ir_nodes: ir::SourceUnitMembers,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl SourceUnitMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = SourceUnitMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_source_unit_member(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type Statements = StatementsStruct;

pub(crate) fn create_statements(
    nodes: &ir::Statements,
    semantic: &Arc<SemanticContext>,
) -> Statements {
    StatementsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct StatementsStruct {
    pub(crate) ir_nodes: ir::Statements,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl StatementsStruct {
    pub fn iter(&self) -> impl Iterator<Item = Statement> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_statement(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type StringLiterals = StringLiteralsStruct;

pub(crate) fn create_string_literals(
    nodes: &ir::StringLiterals,
    semantic: &Arc<SemanticContext>,
) -> StringLiterals {
    StringLiteralsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct StringLiteralsStruct {
    pub(crate) ir_nodes: ir::StringLiterals,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl StringLiteralsStruct {
    pub fn iter(&self) -> impl Iterator<Item = StringLiteral> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_string_literal(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type StructMembers = StructMembersStruct;

pub(crate) fn create_struct_members(
    nodes: &ir::StructMembers,
    semantic: &Arc<SemanticContext>,
) -> StructMembers {
    StructMembersStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct StructMembersStruct {
    pub(crate) ir_nodes: ir::StructMembers,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl StructMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = StructMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_struct_member(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type TupleValues = TupleValuesStruct;

pub(crate) fn create_tuple_values(
    nodes: &ir::TupleValues,
    semantic: &Arc<SemanticContext>,
) -> TupleValues {
    TupleValuesStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct TupleValuesStruct {
    pub(crate) ir_nodes: ir::TupleValues,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl TupleValuesStruct {
    pub fn iter(&self) -> impl Iterator<Item = TupleValue> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_tuple_value(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type UnicodeStringLiterals = UnicodeStringLiteralsStruct;

pub(crate) fn create_unicode_string_literals(
    nodes: &ir::UnicodeStringLiterals,
    semantic: &Arc<SemanticContext>,
) -> UnicodeStringLiterals {
    UnicodeStringLiteralsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct UnicodeStringLiteralsStruct {
    pub(crate) ir_nodes: ir::UnicodeStringLiterals,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl UnicodeStringLiteralsStruct {
    pub fn iter(&self) -> impl Iterator<Item = UnicodeStringLiteral> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_unicode_string_literal(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type UsingDeconstructionSymbols = UsingDeconstructionSymbolsStruct;

pub(crate) fn create_using_deconstruction_symbols(
    nodes: &ir::UsingDeconstructionSymbols,
    semantic: &Arc<SemanticContext>,
) -> UsingDeconstructionSymbols {
    UsingDeconstructionSymbolsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct UsingDeconstructionSymbolsStruct {
    pub(crate) ir_nodes: ir::UsingDeconstructionSymbols,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl UsingDeconstructionSymbolsStruct {
    pub fn iter(&self) -> impl Iterator<Item = UsingDeconstructionSymbol> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_using_deconstruction_symbol(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type VersionExpressionSet = VersionExpressionSetStruct;

pub(crate) fn create_version_expression_set(
    nodes: &ir::VersionExpressionSet,
    semantic: &Arc<SemanticContext>,
) -> VersionExpressionSet {
    VersionExpressionSetStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct VersionExpressionSetStruct {
    pub(crate) ir_nodes: ir::VersionExpressionSet,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl VersionExpressionSetStruct {
    pub fn iter(&self) -> impl Iterator<Item = VersionExpression> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_version_expression(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type VersionExpressionSets = VersionExpressionSetsStruct;

pub(crate) fn create_version_expression_sets(
    nodes: &ir::VersionExpressionSets,
    semantic: &Arc<SemanticContext>,
) -> VersionExpressionSets {
    VersionExpressionSetsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct VersionExpressionSetsStruct {
    pub(crate) ir_nodes: ir::VersionExpressionSets,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl VersionExpressionSetsStruct {
    pub fn iter(&self) -> impl Iterator<Item = VersionExpressionSet> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_version_expression_set(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulArguments = YulArgumentsStruct;

pub(crate) fn create_yul_arguments(
    nodes: &ir::YulArguments,
    semantic: &Arc<SemanticContext>,
) -> YulArguments {
    YulArgumentsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct YulArgumentsStruct {
    pub(crate) ir_nodes: ir::YulArguments,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl YulArgumentsStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulExpression> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_expression(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulFlags = YulFlagsStruct;

pub(crate) fn create_yul_flags(nodes: &ir::YulFlags, semantic: &Arc<SemanticContext>) -> YulFlags {
    YulFlagsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct YulFlagsStruct {
    pub(crate) ir_nodes: ir::YulFlags,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl YulFlagsStruct {
    pub fn iter(&self) -> impl Iterator<Item = StringLiteral> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_string_literal(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulParameters = YulParametersStruct;

pub(crate) fn create_yul_parameters(
    nodes: &ir::YulParameters,
    semantic: &Arc<SemanticContext>,
) -> YulParameters {
    YulParametersStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct YulParametersStruct {
    pub(crate) ir_nodes: ir::YulParameters,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl YulParametersStruct {
    pub fn iter(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulPath = YulPathStruct;

pub(crate) fn create_yul_path(nodes: &ir::YulPath, semantic: &Arc<SemanticContext>) -> YulPath {
    YulPathStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct YulPathStruct {
    pub(crate) ir_nodes: ir::YulPath,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl YulPathStruct {
    pub fn iter(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulPaths = YulPathsStruct;

pub(crate) fn create_yul_paths(nodes: &ir::YulPaths, semantic: &Arc<SemanticContext>) -> YulPaths {
    YulPathsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct YulPathsStruct {
    pub(crate) ir_nodes: ir::YulPaths,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl YulPathsStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulPath> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_path(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulStatements = YulStatementsStruct;

pub(crate) fn create_yul_statements(
    nodes: &ir::YulStatements,
    semantic: &Arc<SemanticContext>,
) -> YulStatements {
    YulStatementsStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct YulStatementsStruct {
    pub(crate) ir_nodes: ir::YulStatements,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl YulStatementsStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulStatement> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_statement(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulSwitchCases = YulSwitchCasesStruct;

pub(crate) fn create_yul_switch_cases(
    nodes: &ir::YulSwitchCases,
    semantic: &Arc<SemanticContext>,
) -> YulSwitchCases {
    YulSwitchCasesStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct YulSwitchCasesStruct {
    pub(crate) ir_nodes: ir::YulSwitchCases,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl YulSwitchCasesStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulSwitchCase> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_switch_case(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulVariableNames = YulVariableNamesStruct;

pub(crate) fn create_yul_variable_names(
    nodes: &ir::YulVariableNames,
    semantic: &Arc<SemanticContext>,
) -> YulVariableNames {
    YulVariableNamesStruct {
        ir_nodes: Arc::clone(nodes),
        semantic: Arc::clone(semantic),
    }
}

#[derive(Clone)]
pub struct YulVariableNamesStruct {
    pub(crate) ir_nodes: ir::YulVariableNames,
    pub(crate) semantic: Arc<SemanticContext>,
}

impl YulVariableNamesStruct {
    pub fn iter(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}

//
// Terminals
//

pub type ABIEncoderV2Keyword = ABIEncoderV2KeywordStruct;

#[derive(Clone)]
pub struct ABIEncoderV2KeywordStruct {
    pub(crate) ir_node: ir::ABIEncoderV2Keyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_abi_encoder_v2_keyword(
    ir_node: &ir::ABIEncoderV2Keyword,
    semantic: &Arc<SemanticContext>,
) -> ABIEncoderV2Keyword {
    ABIEncoderV2KeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ABIEncoderV2KeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AbicoderV1Keyword = AbicoderV1KeywordStruct;

#[derive(Clone)]
pub struct AbicoderV1KeywordStruct {
    pub(crate) ir_node: ir::AbicoderV1Keyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_abicoder_v1_keyword(
    ir_node: &ir::AbicoderV1Keyword,
    semantic: &Arc<SemanticContext>,
) -> AbicoderV1Keyword {
    AbicoderV1KeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl AbicoderV1KeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AbicoderV2Keyword = AbicoderV2KeywordStruct;

#[derive(Clone)]
pub struct AbicoderV2KeywordStruct {
    pub(crate) ir_node: ir::AbicoderV2Keyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_abicoder_v2_keyword(
    ir_node: &ir::AbicoderV2Keyword,
    semantic: &Arc<SemanticContext>,
) -> AbicoderV2Keyword {
    AbicoderV2KeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl AbicoderV2KeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Ampersand = AmpersandStruct;

#[derive(Clone)]
pub struct AmpersandStruct {
    pub(crate) ir_node: ir::Ampersand,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_ampersand(
    ir_node: &ir::Ampersand,
    semantic: &Arc<SemanticContext>,
) -> Ampersand {
    AmpersandStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl AmpersandStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AmpersandEqual = AmpersandEqualStruct;

#[derive(Clone)]
pub struct AmpersandEqualStruct {
    pub(crate) ir_node: ir::AmpersandEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_ampersand_equal(
    ir_node: &ir::AmpersandEqual,
    semantic: &Arc<SemanticContext>,
) -> AmpersandEqual {
    AmpersandEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl AmpersandEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Asterisk = AsteriskStruct;

#[derive(Clone)]
pub struct AsteriskStruct {
    pub(crate) ir_node: ir::Asterisk,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_asterisk(ir_node: &ir::Asterisk, semantic: &Arc<SemanticContext>) -> Asterisk {
    AsteriskStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl AsteriskStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AsteriskEqual = AsteriskEqualStruct;

#[derive(Clone)]
pub struct AsteriskEqualStruct {
    pub(crate) ir_node: ir::AsteriskEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_asterisk_equal(
    ir_node: &ir::AsteriskEqual,
    semantic: &Arc<SemanticContext>,
) -> AsteriskEqual {
    AsteriskEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl AsteriskEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Bang = BangStruct;

#[derive(Clone)]
pub struct BangStruct {
    pub(crate) ir_node: ir::Bang,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_bang(ir_node: &ir::Bang, semantic: &Arc<SemanticContext>) -> Bang {
    BangStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl BangStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BangEqual = BangEqualStruct;

#[derive(Clone)]
pub struct BangEqualStruct {
    pub(crate) ir_node: ir::BangEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_bang_equal(
    ir_node: &ir::BangEqual,
    semantic: &Arc<SemanticContext>,
) -> BangEqual {
    BangEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl BangEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Bar = BarStruct;

#[derive(Clone)]
pub struct BarStruct {
    pub(crate) ir_node: ir::Bar,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_bar(ir_node: &ir::Bar, semantic: &Arc<SemanticContext>) -> Bar {
    BarStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl BarStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BarEqual = BarEqualStruct;

#[derive(Clone)]
pub struct BarEqualStruct {
    pub(crate) ir_node: ir::BarEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_bar_equal(
    ir_node: &ir::BarEqual,
    semantic: &Arc<SemanticContext>,
) -> BarEqual {
    BarEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl BarEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BoolKeyword = BoolKeywordStruct;

#[derive(Clone)]
pub struct BoolKeywordStruct {
    pub(crate) ir_node: ir::BoolKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_bool_keyword(
    ir_node: &ir::BoolKeyword,
    semantic: &Arc<SemanticContext>,
) -> BoolKeyword {
    BoolKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl BoolKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BytesKeyword = BytesKeywordStruct;

#[derive(Clone)]
pub struct BytesKeywordStruct {
    pub(crate) ir_node: ir::BytesKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_bytes_keyword(
    ir_node: &ir::BytesKeyword,
    semantic: &Arc<SemanticContext>,
) -> BytesKeyword {
    BytesKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl BytesKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type CallDataKeyword = CallDataKeywordStruct;

#[derive(Clone)]
pub struct CallDataKeywordStruct {
    pub(crate) ir_node: ir::CallDataKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_call_data_keyword(
    ir_node: &ir::CallDataKeyword,
    semantic: &Arc<SemanticContext>,
) -> CallDataKeyword {
    CallDataKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl CallDataKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Caret = CaretStruct;

#[derive(Clone)]
pub struct CaretStruct {
    pub(crate) ir_node: ir::Caret,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_caret(ir_node: &ir::Caret, semantic: &Arc<SemanticContext>) -> Caret {
    CaretStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl CaretStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type CaretEqual = CaretEqualStruct;

#[derive(Clone)]
pub struct CaretEqualStruct {
    pub(crate) ir_node: ir::CaretEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_caret_equal(
    ir_node: &ir::CaretEqual,
    semantic: &Arc<SemanticContext>,
) -> CaretEqual {
    CaretEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl CaretEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type DaysKeyword = DaysKeywordStruct;

#[derive(Clone)]
pub struct DaysKeywordStruct {
    pub(crate) ir_node: ir::DaysKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_days_keyword(
    ir_node: &ir::DaysKeyword,
    semantic: &Arc<SemanticContext>,
) -> DaysKeyword {
    DaysKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl DaysKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type DecimalLiteral = DecimalLiteralStruct;

#[derive(Clone)]
pub struct DecimalLiteralStruct {
    pub(crate) ir_node: ir::DecimalLiteral,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_decimal_literal(
    ir_node: &ir::DecimalLiteral,
    semantic: &Arc<SemanticContext>,
) -> DecimalLiteral {
    DecimalLiteralStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl DecimalLiteralStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type DeleteKeyword = DeleteKeywordStruct;

#[derive(Clone)]
pub struct DeleteKeywordStruct {
    pub(crate) ir_node: ir::DeleteKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_delete_keyword(
    ir_node: &ir::DeleteKeyword,
    semantic: &Arc<SemanticContext>,
) -> DeleteKeyword {
    DeleteKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl DeleteKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Equal = EqualStruct;

#[derive(Clone)]
pub struct EqualStruct {
    pub(crate) ir_node: ir::Equal,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_equal(ir_node: &ir::Equal, semantic: &Arc<SemanticContext>) -> Equal {
    EqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl EqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type EqualEqual = EqualEqualStruct;

#[derive(Clone)]
pub struct EqualEqualStruct {
    pub(crate) ir_node: ir::EqualEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_equal_equal(
    ir_node: &ir::EqualEqual,
    semantic: &Arc<SemanticContext>,
) -> EqualEqual {
    EqualEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl EqualEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type EtherKeyword = EtherKeywordStruct;

#[derive(Clone)]
pub struct EtherKeywordStruct {
    pub(crate) ir_node: ir::EtherKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_ether_keyword(
    ir_node: &ir::EtherKeyword,
    semantic: &Arc<SemanticContext>,
) -> EtherKeyword {
    EtherKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl EtherKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type FalseKeyword = FalseKeywordStruct;

#[derive(Clone)]
pub struct FalseKeywordStruct {
    pub(crate) ir_node: ir::FalseKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_false_keyword(
    ir_node: &ir::FalseKeyword,
    semantic: &Arc<SemanticContext>,
) -> FalseKeyword {
    FalseKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl FalseKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type FixedKeyword = FixedKeywordStruct;

#[derive(Clone)]
pub struct FixedKeywordStruct {
    pub(crate) ir_node: ir::FixedKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_fixed_keyword(
    ir_node: &ir::FixedKeyword,
    semantic: &Arc<SemanticContext>,
) -> FixedKeyword {
    FixedKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl FixedKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThan = GreaterThanStruct;

#[derive(Clone)]
pub struct GreaterThanStruct {
    pub(crate) ir_node: ir::GreaterThan,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_greater_than(
    ir_node: &ir::GreaterThan,
    semantic: &Arc<SemanticContext>,
) -> GreaterThan {
    GreaterThanStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl GreaterThanStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanEqual = GreaterThanEqualStruct;

#[derive(Clone)]
pub struct GreaterThanEqualStruct {
    pub(crate) ir_node: ir::GreaterThanEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_greater_than_equal(
    ir_node: &ir::GreaterThanEqual,
    semantic: &Arc<SemanticContext>,
) -> GreaterThanEqual {
    GreaterThanEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl GreaterThanEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanGreaterThan = GreaterThanGreaterThanStruct;

#[derive(Clone)]
pub struct GreaterThanGreaterThanStruct {
    pub(crate) ir_node: ir::GreaterThanGreaterThan,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_greater_than_greater_than(
    ir_node: &ir::GreaterThanGreaterThan,
    semantic: &Arc<SemanticContext>,
) -> GreaterThanGreaterThan {
    GreaterThanGreaterThanStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl GreaterThanGreaterThanStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanGreaterThanEqual = GreaterThanGreaterThanEqualStruct;

#[derive(Clone)]
pub struct GreaterThanGreaterThanEqualStruct {
    pub(crate) ir_node: ir::GreaterThanGreaterThanEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_greater_than_greater_than_equal(
    ir_node: &ir::GreaterThanGreaterThanEqual,
    semantic: &Arc<SemanticContext>,
) -> GreaterThanGreaterThanEqual {
    GreaterThanGreaterThanEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl GreaterThanGreaterThanEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanGreaterThanGreaterThan = GreaterThanGreaterThanGreaterThanStruct;

#[derive(Clone)]
pub struct GreaterThanGreaterThanGreaterThanStruct {
    pub(crate) ir_node: ir::GreaterThanGreaterThanGreaterThan,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_greater_than_greater_than_greater_than(
    ir_node: &ir::GreaterThanGreaterThanGreaterThan,
    semantic: &Arc<SemanticContext>,
) -> GreaterThanGreaterThanGreaterThan {
    GreaterThanGreaterThanGreaterThanStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl GreaterThanGreaterThanGreaterThanStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanGreaterThanGreaterThanEqual = GreaterThanGreaterThanGreaterThanEqualStruct;

#[derive(Clone)]
pub struct GreaterThanGreaterThanGreaterThanEqualStruct {
    pub(crate) ir_node: ir::GreaterThanGreaterThanGreaterThanEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_greater_than_greater_than_greater_than_equal(
    ir_node: &ir::GreaterThanGreaterThanGreaterThanEqual,
    semantic: &Arc<SemanticContext>,
) -> GreaterThanGreaterThanGreaterThanEqual {
    GreaterThanGreaterThanGreaterThanEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl GreaterThanGreaterThanGreaterThanEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GweiKeyword = GweiKeywordStruct;

#[derive(Clone)]
pub struct GweiKeywordStruct {
    pub(crate) ir_node: ir::GweiKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_gwei_keyword(
    ir_node: &ir::GweiKeyword,
    semantic: &Arc<SemanticContext>,
) -> GweiKeyword {
    GweiKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl GweiKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type HexLiteral = HexLiteralStruct;

#[derive(Clone)]
pub struct HexLiteralStruct {
    pub(crate) ir_node: ir::HexLiteral,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_hex_literal(
    ir_node: &ir::HexLiteral,
    semantic: &Arc<SemanticContext>,
) -> HexLiteral {
    HexLiteralStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl HexLiteralStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type HexStringLiteral = HexStringLiteralStruct;

#[derive(Clone)]
pub struct HexStringLiteralStruct {
    pub(crate) ir_node: ir::HexStringLiteral,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_hex_string_literal(
    ir_node: &ir::HexStringLiteral,
    semantic: &Arc<SemanticContext>,
) -> HexStringLiteral {
    HexStringLiteralStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl HexStringLiteralStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type HoursKeyword = HoursKeywordStruct;

#[derive(Clone)]
pub struct HoursKeywordStruct {
    pub(crate) ir_node: ir::HoursKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_hours_keyword(
    ir_node: &ir::HoursKeyword,
    semantic: &Arc<SemanticContext>,
) -> HoursKeyword {
    HoursKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl HoursKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Identifier = IdentifierStruct;

#[derive(Clone)]
pub struct IdentifierStruct {
    pub(crate) ir_node: ir::Identifier,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_identifier(
    ir_node: &ir::Identifier,
    semantic: &Arc<SemanticContext>,
) -> Identifier {
    IdentifierStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl IdentifierStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type IntKeyword = IntKeywordStruct;

#[derive(Clone)]
pub struct IntKeywordStruct {
    pub(crate) ir_node: ir::IntKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_int_keyword(
    ir_node: &ir::IntKeyword,
    semantic: &Arc<SemanticContext>,
) -> IntKeyword {
    IntKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl IntKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type LessThan = LessThanStruct;

#[derive(Clone)]
pub struct LessThanStruct {
    pub(crate) ir_node: ir::LessThan,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_less_than(
    ir_node: &ir::LessThan,
    semantic: &Arc<SemanticContext>,
) -> LessThan {
    LessThanStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl LessThanStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type LessThanEqual = LessThanEqualStruct;

#[derive(Clone)]
pub struct LessThanEqualStruct {
    pub(crate) ir_node: ir::LessThanEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_less_than_equal(
    ir_node: &ir::LessThanEqual,
    semantic: &Arc<SemanticContext>,
) -> LessThanEqual {
    LessThanEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl LessThanEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type LessThanLessThan = LessThanLessThanStruct;

#[derive(Clone)]
pub struct LessThanLessThanStruct {
    pub(crate) ir_node: ir::LessThanLessThan,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_less_than_less_than(
    ir_node: &ir::LessThanLessThan,
    semantic: &Arc<SemanticContext>,
) -> LessThanLessThan {
    LessThanLessThanStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl LessThanLessThanStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type LessThanLessThanEqual = LessThanLessThanEqualStruct;

#[derive(Clone)]
pub struct LessThanLessThanEqualStruct {
    pub(crate) ir_node: ir::LessThanLessThanEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_less_than_less_than_equal(
    ir_node: &ir::LessThanLessThanEqual,
    semantic: &Arc<SemanticContext>,
) -> LessThanLessThanEqual {
    LessThanLessThanEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl LessThanLessThanEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MemoryKeyword = MemoryKeywordStruct;

#[derive(Clone)]
pub struct MemoryKeywordStruct {
    pub(crate) ir_node: ir::MemoryKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_memory_keyword(
    ir_node: &ir::MemoryKeyword,
    semantic: &Arc<SemanticContext>,
) -> MemoryKeyword {
    MemoryKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl MemoryKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Minus = MinusStruct;

#[derive(Clone)]
pub struct MinusStruct {
    pub(crate) ir_node: ir::Minus,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_minus(ir_node: &ir::Minus, semantic: &Arc<SemanticContext>) -> Minus {
    MinusStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl MinusStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MinusEqual = MinusEqualStruct;

#[derive(Clone)]
pub struct MinusEqualStruct {
    pub(crate) ir_node: ir::MinusEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_minus_equal(
    ir_node: &ir::MinusEqual,
    semantic: &Arc<SemanticContext>,
) -> MinusEqual {
    MinusEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl MinusEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MinusMinus = MinusMinusStruct;

#[derive(Clone)]
pub struct MinusMinusStruct {
    pub(crate) ir_node: ir::MinusMinus,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_minus_minus(
    ir_node: &ir::MinusMinus,
    semantic: &Arc<SemanticContext>,
) -> MinusMinus {
    MinusMinusStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl MinusMinusStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MinutesKeyword = MinutesKeywordStruct;

#[derive(Clone)]
pub struct MinutesKeywordStruct {
    pub(crate) ir_node: ir::MinutesKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_minutes_keyword(
    ir_node: &ir::MinutesKeyword,
    semantic: &Arc<SemanticContext>,
) -> MinutesKeyword {
    MinutesKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl MinutesKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PayableKeyword = PayableKeywordStruct;

#[derive(Clone)]
pub struct PayableKeywordStruct {
    pub(crate) ir_node: ir::PayableKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_payable_keyword(
    ir_node: &ir::PayableKeyword,
    semantic: &Arc<SemanticContext>,
) -> PayableKeyword {
    PayableKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PayableKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Percent = PercentStruct;

#[derive(Clone)]
pub struct PercentStruct {
    pub(crate) ir_node: ir::Percent,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_percent(ir_node: &ir::Percent, semantic: &Arc<SemanticContext>) -> Percent {
    PercentStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PercentStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PercentEqual = PercentEqualStruct;

#[derive(Clone)]
pub struct PercentEqualStruct {
    pub(crate) ir_node: ir::PercentEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_percent_equal(
    ir_node: &ir::PercentEqual,
    semantic: &Arc<SemanticContext>,
) -> PercentEqual {
    PercentEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PercentEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Plus = PlusStruct;

#[derive(Clone)]
pub struct PlusStruct {
    pub(crate) ir_node: ir::Plus,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_plus(ir_node: &ir::Plus, semantic: &Arc<SemanticContext>) -> Plus {
    PlusStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PlusStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PlusEqual = PlusEqualStruct;

#[derive(Clone)]
pub struct PlusEqualStruct {
    pub(crate) ir_node: ir::PlusEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_plus_equal(
    ir_node: &ir::PlusEqual,
    semantic: &Arc<SemanticContext>,
) -> PlusEqual {
    PlusEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PlusEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PlusPlus = PlusPlusStruct;

#[derive(Clone)]
pub struct PlusPlusStruct {
    pub(crate) ir_node: ir::PlusPlus,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_plus_plus(
    ir_node: &ir::PlusPlus,
    semantic: &Arc<SemanticContext>,
) -> PlusPlus {
    PlusPlusStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PlusPlusStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaCaret = PragmaCaretStruct;

#[derive(Clone)]
pub struct PragmaCaretStruct {
    pub(crate) ir_node: ir::PragmaCaret,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_pragma_caret(
    ir_node: &ir::PragmaCaret,
    semantic: &Arc<SemanticContext>,
) -> PragmaCaret {
    PragmaCaretStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PragmaCaretStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaEqual = PragmaEqualStruct;

#[derive(Clone)]
pub struct PragmaEqualStruct {
    pub(crate) ir_node: ir::PragmaEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_pragma_equal(
    ir_node: &ir::PragmaEqual,
    semantic: &Arc<SemanticContext>,
) -> PragmaEqual {
    PragmaEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PragmaEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaGreaterThan = PragmaGreaterThanStruct;

#[derive(Clone)]
pub struct PragmaGreaterThanStruct {
    pub(crate) ir_node: ir::PragmaGreaterThan,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_pragma_greater_than(
    ir_node: &ir::PragmaGreaterThan,
    semantic: &Arc<SemanticContext>,
) -> PragmaGreaterThan {
    PragmaGreaterThanStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PragmaGreaterThanStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaGreaterThanEqual = PragmaGreaterThanEqualStruct;

#[derive(Clone)]
pub struct PragmaGreaterThanEqualStruct {
    pub(crate) ir_node: ir::PragmaGreaterThanEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_pragma_greater_than_equal(
    ir_node: &ir::PragmaGreaterThanEqual,
    semantic: &Arc<SemanticContext>,
) -> PragmaGreaterThanEqual {
    PragmaGreaterThanEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PragmaGreaterThanEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaLessThan = PragmaLessThanStruct;

#[derive(Clone)]
pub struct PragmaLessThanStruct {
    pub(crate) ir_node: ir::PragmaLessThan,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_pragma_less_than(
    ir_node: &ir::PragmaLessThan,
    semantic: &Arc<SemanticContext>,
) -> PragmaLessThan {
    PragmaLessThanStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PragmaLessThanStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaLessThanEqual = PragmaLessThanEqualStruct;

#[derive(Clone)]
pub struct PragmaLessThanEqualStruct {
    pub(crate) ir_node: ir::PragmaLessThanEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_pragma_less_than_equal(
    ir_node: &ir::PragmaLessThanEqual,
    semantic: &Arc<SemanticContext>,
) -> PragmaLessThanEqual {
    PragmaLessThanEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PragmaLessThanEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaTilde = PragmaTildeStruct;

#[derive(Clone)]
pub struct PragmaTildeStruct {
    pub(crate) ir_node: ir::PragmaTilde,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_pragma_tilde(
    ir_node: &ir::PragmaTilde,
    semantic: &Arc<SemanticContext>,
) -> PragmaTilde {
    PragmaTildeStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl PragmaTildeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type SMTCheckerKeyword = SMTCheckerKeywordStruct;

#[derive(Clone)]
pub struct SMTCheckerKeywordStruct {
    pub(crate) ir_node: ir::SMTCheckerKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_smt_checker_keyword(
    ir_node: &ir::SMTCheckerKeyword,
    semantic: &Arc<SemanticContext>,
) -> SMTCheckerKeyword {
    SMTCheckerKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl SMTCheckerKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type SecondsKeyword = SecondsKeywordStruct;

#[derive(Clone)]
pub struct SecondsKeywordStruct {
    pub(crate) ir_node: ir::SecondsKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_seconds_keyword(
    ir_node: &ir::SecondsKeyword,
    semantic: &Arc<SemanticContext>,
) -> SecondsKeyword {
    SecondsKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl SecondsKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Semicolon = SemicolonStruct;

#[derive(Clone)]
pub struct SemicolonStruct {
    pub(crate) ir_node: ir::Semicolon,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_semicolon(
    ir_node: &ir::Semicolon,
    semantic: &Arc<SemanticContext>,
) -> Semicolon {
    SemicolonStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl SemicolonStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Slash = SlashStruct;

#[derive(Clone)]
pub struct SlashStruct {
    pub(crate) ir_node: ir::Slash,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_slash(ir_node: &ir::Slash, semantic: &Arc<SemanticContext>) -> Slash {
    SlashStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl SlashStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type SlashEqual = SlashEqualStruct;

#[derive(Clone)]
pub struct SlashEqualStruct {
    pub(crate) ir_node: ir::SlashEqual,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_slash_equal(
    ir_node: &ir::SlashEqual,
    semantic: &Arc<SemanticContext>,
) -> SlashEqual {
    SlashEqualStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl SlashEqualStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StorageKeyword = StorageKeywordStruct;

#[derive(Clone)]
pub struct StorageKeywordStruct {
    pub(crate) ir_node: ir::StorageKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_storage_keyword(
    ir_node: &ir::StorageKeyword,
    semantic: &Arc<SemanticContext>,
) -> StorageKeyword {
    StorageKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl StorageKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StringKeyword = StringKeywordStruct;

#[derive(Clone)]
pub struct StringKeywordStruct {
    pub(crate) ir_node: ir::StringKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_string_keyword(
    ir_node: &ir::StringKeyword,
    semantic: &Arc<SemanticContext>,
) -> StringKeyword {
    StringKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl StringKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StringLiteral = StringLiteralStruct;

#[derive(Clone)]
pub struct StringLiteralStruct {
    pub(crate) ir_node: ir::StringLiteral,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_string_literal(
    ir_node: &ir::StringLiteral,
    semantic: &Arc<SemanticContext>,
) -> StringLiteral {
    StringLiteralStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl StringLiteralStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type SuperKeyword = SuperKeywordStruct;

#[derive(Clone)]
pub struct SuperKeywordStruct {
    pub(crate) ir_node: ir::SuperKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_super_keyword(
    ir_node: &ir::SuperKeyword,
    semantic: &Arc<SemanticContext>,
) -> SuperKeyword {
    SuperKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl SuperKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ThisKeyword = ThisKeywordStruct;

#[derive(Clone)]
pub struct ThisKeywordStruct {
    pub(crate) ir_node: ir::ThisKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_this_keyword(
    ir_node: &ir::ThisKeyword,
    semantic: &Arc<SemanticContext>,
) -> ThisKeyword {
    ThisKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl ThisKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Tilde = TildeStruct;

#[derive(Clone)]
pub struct TildeStruct {
    pub(crate) ir_node: ir::Tilde,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_tilde(ir_node: &ir::Tilde, semantic: &Arc<SemanticContext>) -> Tilde {
    TildeStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl TildeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type TrueKeyword = TrueKeywordStruct;

#[derive(Clone)]
pub struct TrueKeywordStruct {
    pub(crate) ir_node: ir::TrueKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_true_keyword(
    ir_node: &ir::TrueKeyword,
    semantic: &Arc<SemanticContext>,
) -> TrueKeyword {
    TrueKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl TrueKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UfixedKeyword = UfixedKeywordStruct;

#[derive(Clone)]
pub struct UfixedKeywordStruct {
    pub(crate) ir_node: ir::UfixedKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_ufixed_keyword(
    ir_node: &ir::UfixedKeyword,
    semantic: &Arc<SemanticContext>,
) -> UfixedKeyword {
    UfixedKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl UfixedKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UintKeyword = UintKeywordStruct;

#[derive(Clone)]
pub struct UintKeywordStruct {
    pub(crate) ir_node: ir::UintKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_uint_keyword(
    ir_node: &ir::UintKeyword,
    semantic: &Arc<SemanticContext>,
) -> UintKeyword {
    UintKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl UintKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UnicodeStringLiteral = UnicodeStringLiteralStruct;

#[derive(Clone)]
pub struct UnicodeStringLiteralStruct {
    pub(crate) ir_node: ir::UnicodeStringLiteral,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_unicode_string_literal(
    ir_node: &ir::UnicodeStringLiteral,
    semantic: &Arc<SemanticContext>,
) -> UnicodeStringLiteral {
    UnicodeStringLiteralStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl UnicodeStringLiteralStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VersionSpecifier = VersionSpecifierStruct;

#[derive(Clone)]
pub struct VersionSpecifierStruct {
    pub(crate) ir_node: ir::VersionSpecifier,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_version_specifier(
    ir_node: &ir::VersionSpecifier,
    semantic: &Arc<SemanticContext>,
) -> VersionSpecifier {
    VersionSpecifierStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl VersionSpecifierStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type WeeksKeyword = WeeksKeywordStruct;

#[derive(Clone)]
pub struct WeeksKeywordStruct {
    pub(crate) ir_node: ir::WeeksKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_weeks_keyword(
    ir_node: &ir::WeeksKeyword,
    semantic: &Arc<SemanticContext>,
) -> WeeksKeyword {
    WeeksKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl WeeksKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type WeiKeyword = WeiKeywordStruct;

#[derive(Clone)]
pub struct WeiKeywordStruct {
    pub(crate) ir_node: ir::WeiKeyword,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub(crate) fn create_wei_keyword(
    ir_node: &ir::WeiKeyword,
    semantic: &Arc<SemanticContext>,
) -> WeiKeyword {
    WeiKeywordStruct {
        ir_node: Arc::clone(ir_node),
        semantic: Arc::clone(semantic),
    }
}

impl WeiKeywordStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}
