// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(unused)]
#![allow(non_camel_case_types)]
use std::ops::Range;
use std::rc::Rc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::context::SemanticContext;

use super::types::Type;

//
// Sequences
//

pub type AbicoderPragma = Rc<AbicoderPragmaStruct>;

pub struct AbicoderPragmaStruct {
    pub(crate) ir_node: ir::AbicoderPragma,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_abicoder_pragma(
    ir_node: &ir::AbicoderPragma,
    semantic: &Rc<SemanticContext>,
) -> AbicoderPragma {
    Rc::new(AbicoderPragmaStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AdditiveExpression = Rc<AdditiveExpressionStruct>;

pub struct AdditiveExpressionStruct {
    pub(crate) ir_node: ir::AdditiveExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_additive_expression(
    ir_node: &ir::AdditiveExpression,
    semantic: &Rc<SemanticContext>,
) -> AdditiveExpression {
    Rc::new(AdditiveExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AddressType = Rc<AddressTypeStruct>;

pub struct AddressTypeStruct {
    pub(crate) ir_node: ir::AddressType,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_address_type(
    ir_node: &ir::AddressType,
    semantic: &Rc<SemanticContext>,
) -> AddressType {
    Rc::new(AddressTypeStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AddressTypeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn payable_keyword(&self) -> Option<PayableKeyword> {
        self.ir_node
            .payable_keyword
            .as_ref()
            .map(|ir_node| create_payable_keyword(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AndExpression = Rc<AndExpressionStruct>;

pub struct AndExpressionStruct {
    pub(crate) ir_node: ir::AndExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_and_expression(
    ir_node: &ir::AndExpression,
    semantic: &Rc<SemanticContext>,
) -> AndExpression {
    Rc::new(AndExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ArrayExpression = Rc<ArrayExpressionStruct>;

pub struct ArrayExpressionStruct {
    pub(crate) ir_node: ir::ArrayExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_array_expression(
    ir_node: &ir::ArrayExpression,
    semantic: &Rc<SemanticContext>,
) -> ArrayExpression {
    Rc::new(ArrayExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ArrayTypeName = Rc<ArrayTypeNameStruct>;

pub struct ArrayTypeNameStruct {
    pub(crate) ir_node: ir::ArrayTypeName,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_array_type_name(
    ir_node: &ir::ArrayTypeName,
    semantic: &Rc<SemanticContext>,
) -> ArrayTypeName {
    Rc::new(ArrayTypeNameStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AssemblyStatement = Rc<AssemblyStatementStruct>;

pub struct AssemblyStatementStruct {
    pub(crate) ir_node: ir::AssemblyStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_assembly_statement(
    ir_node: &ir::AssemblyStatement,
    semantic: &Rc<SemanticContext>,
) -> AssemblyStatement {
    Rc::new(AssemblyStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AssignmentExpression = Rc<AssignmentExpressionStruct>;

pub struct AssignmentExpressionStruct {
    pub(crate) ir_node: ir::AssignmentExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_assignment_expression(
    ir_node: &ir::AssignmentExpression,
    semantic: &Rc<SemanticContext>,
) -> AssignmentExpression {
    Rc::new(AssignmentExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BitwiseAndExpression = Rc<BitwiseAndExpressionStruct>;

pub struct BitwiseAndExpressionStruct {
    pub(crate) ir_node: ir::BitwiseAndExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_bitwise_and_expression(
    ir_node: &ir::BitwiseAndExpression,
    semantic: &Rc<SemanticContext>,
) -> BitwiseAndExpression {
    Rc::new(BitwiseAndExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BitwiseOrExpression = Rc<BitwiseOrExpressionStruct>;

pub struct BitwiseOrExpressionStruct {
    pub(crate) ir_node: ir::BitwiseOrExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_bitwise_or_expression(
    ir_node: &ir::BitwiseOrExpression,
    semantic: &Rc<SemanticContext>,
) -> BitwiseOrExpression {
    Rc::new(BitwiseOrExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BitwiseXorExpression = Rc<BitwiseXorExpressionStruct>;

pub struct BitwiseXorExpressionStruct {
    pub(crate) ir_node: ir::BitwiseXorExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_bitwise_xor_expression(
    ir_node: &ir::BitwiseXorExpression,
    semantic: &Rc<SemanticContext>,
) -> BitwiseXorExpression {
    Rc::new(BitwiseXorExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Block = Rc<BlockStruct>;

pub struct BlockStruct {
    pub(crate) ir_node: ir::Block,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_block(ir_node: &ir::Block, semantic: &Rc<SemanticContext>) -> Block {
    Rc::new(BlockStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BreakStatement = Rc<BreakStatementStruct>;

pub struct BreakStatementStruct {
    pub(crate) ir_node: ir::BreakStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_break_statement(
    ir_node: &ir::BreakStatement,
    semantic: &Rc<SemanticContext>,
) -> BreakStatement {
    Rc::new(BreakStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl BreakStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type CallOptionsExpression = Rc<CallOptionsExpressionStruct>;

pub struct CallOptionsExpressionStruct {
    pub(crate) ir_node: ir::CallOptionsExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_call_options_expression(
    ir_node: &ir::CallOptionsExpression,
    semantic: &Rc<SemanticContext>,
) -> CallOptionsExpression {
    Rc::new(CallOptionsExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type CatchClause = Rc<CatchClauseStruct>;

pub struct CatchClauseStruct {
    pub(crate) ir_node: ir::CatchClause,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_catch_clause(
    ir_node: &ir::CatchClause,
    semantic: &Rc<SemanticContext>,
) -> CatchClause {
    Rc::new(CatchClauseStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type CatchClauseError = Rc<CatchClauseErrorStruct>;

pub struct CatchClauseErrorStruct {
    pub(crate) ir_node: ir::CatchClauseError,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_catch_clause_error(
    ir_node: &ir::CatchClauseError,
    semantic: &Rc<SemanticContext>,
) -> CatchClauseError {
    Rc::new(CatchClauseErrorStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ConditionalExpression = Rc<ConditionalExpressionStruct>;

pub struct ConditionalExpressionStruct {
    pub(crate) ir_node: ir::ConditionalExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_conditional_expression(
    ir_node: &ir::ConditionalExpression,
    semantic: &Rc<SemanticContext>,
) -> ConditionalExpression {
    Rc::new(ConditionalExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ConstantDefinition = Rc<ConstantDefinitionStruct>;

pub struct ConstantDefinitionStruct {
    pub(crate) ir_node: ir::ConstantDefinition,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_constant_definition(
    ir_node: &ir::ConstantDefinition,
    semantic: &Rc<SemanticContext>,
) -> ConstantDefinition {
    Rc::new(ConstantDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ContinueStatement = Rc<ContinueStatementStruct>;

pub struct ContinueStatementStruct {
    pub(crate) ir_node: ir::ContinueStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_continue_statement(
    ir_node: &ir::ContinueStatement,
    semantic: &Rc<SemanticContext>,
) -> ContinueStatement {
    Rc::new(ContinueStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ContinueStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ContractDefinition = Rc<ContractDefinitionStruct>;

pub struct ContractDefinitionStruct {
    pub(crate) ir_node: ir::ContractDefinition,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_contract_definition(
    ir_node: &ir::ContractDefinition,
    semantic: &Rc<SemanticContext>,
) -> ContractDefinition {
    Rc::new(ContractDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ContractDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn abstract_keyword(&self) -> Option<AbstractKeyword> {
        self.ir_node
            .abstract_keyword
            .as_ref()
            .map(|ir_node| create_abstract_keyword(ir_node, &self.semantic))
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type DecimalNumberExpression = Rc<DecimalNumberExpressionStruct>;

pub struct DecimalNumberExpressionStruct {
    pub(crate) ir_node: ir::DecimalNumberExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_decimal_number_expression(
    ir_node: &ir::DecimalNumberExpression,
    semantic: &Rc<SemanticContext>,
) -> DecimalNumberExpression {
    Rc::new(DecimalNumberExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type DoWhileStatement = Rc<DoWhileStatementStruct>;

pub struct DoWhileStatementStruct {
    pub(crate) ir_node: ir::DoWhileStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_do_while_statement(
    ir_node: &ir::DoWhileStatement,
    semantic: &Rc<SemanticContext>,
) -> DoWhileStatement {
    Rc::new(DoWhileStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type EmitStatement = Rc<EmitStatementStruct>;

pub struct EmitStatementStruct {
    pub(crate) ir_node: ir::EmitStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_emit_statement(
    ir_node: &ir::EmitStatement,
    semantic: &Rc<SemanticContext>,
) -> EmitStatement {
    Rc::new(EmitStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type EnumDefinition = Rc<EnumDefinitionStruct>;

pub struct EnumDefinitionStruct {
    pub(crate) ir_node: ir::EnumDefinition,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_enum_definition(
    ir_node: &ir::EnumDefinition,
    semantic: &Rc<SemanticContext>,
) -> EnumDefinition {
    Rc::new(EnumDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type EqualityExpression = Rc<EqualityExpressionStruct>;

pub struct EqualityExpressionStruct {
    pub(crate) ir_node: ir::EqualityExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_equality_expression(
    ir_node: &ir::EqualityExpression,
    semantic: &Rc<SemanticContext>,
) -> EqualityExpression {
    Rc::new(EqualityExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ErrorDefinition = Rc<ErrorDefinitionStruct>;

pub struct ErrorDefinitionStruct {
    pub(crate) ir_node: ir::ErrorDefinition,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_error_definition(
    ir_node: &ir::ErrorDefinition,
    semantic: &Rc<SemanticContext>,
) -> ErrorDefinition {
    Rc::new(ErrorDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type EventDefinition = Rc<EventDefinitionStruct>;

pub struct EventDefinitionStruct {
    pub(crate) ir_node: ir::EventDefinition,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_event_definition(
    ir_node: &ir::EventDefinition,
    semantic: &Rc<SemanticContext>,
) -> EventDefinition {
    Rc::new(EventDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl EventDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn anonymous_keyword(&self) -> Option<AnonymousKeyword> {
        self.ir_node
            .anonymous_keyword
            .as_ref()
            .map(|ir_node| create_anonymous_keyword(ir_node, &self.semantic))
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ExperimentalPragma = Rc<ExperimentalPragmaStruct>;

pub struct ExperimentalPragmaStruct {
    pub(crate) ir_node: ir::ExperimentalPragma,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_experimental_pragma(
    ir_node: &ir::ExperimentalPragma,
    semantic: &Rc<SemanticContext>,
) -> ExperimentalPragma {
    Rc::new(ExperimentalPragmaStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ExponentiationExpression = Rc<ExponentiationExpressionStruct>;

pub struct ExponentiationExpressionStruct {
    pub(crate) ir_node: ir::ExponentiationExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_exponentiation_expression(
    ir_node: &ir::ExponentiationExpression,
    semantic: &Rc<SemanticContext>,
) -> ExponentiationExpression {
    Rc::new(ExponentiationExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ExpressionStatement = Rc<ExpressionStatementStruct>;

pub struct ExpressionStatementStruct {
    pub(crate) ir_node: ir::ExpressionStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_expression_statement(
    ir_node: &ir::ExpressionStatement,
    semantic: &Rc<SemanticContext>,
) -> ExpressionStatement {
    Rc::new(ExpressionStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ForStatement = Rc<ForStatementStruct>;

pub struct ForStatementStruct {
    pub(crate) ir_node: ir::ForStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_for_statement(
    ir_node: &ir::ForStatement,
    semantic: &Rc<SemanticContext>,
) -> ForStatement {
    Rc::new(ForStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionCallExpression = Rc<FunctionCallExpressionStruct>;

pub struct FunctionCallExpressionStruct {
    pub(crate) ir_node: ir::FunctionCallExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_function_call_expression(
    ir_node: &ir::FunctionCallExpression,
    semantic: &Rc<SemanticContext>,
) -> FunctionCallExpression {
    Rc::new(FunctionCallExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

pub struct FunctionDefinitionStruct {
    pub(crate) ir_node: ir::FunctionDefinition,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_function_definition(
    ir_node: &ir::FunctionDefinition,
    semantic: &Rc<SemanticContext>,
) -> FunctionDefinition {
    Rc::new(FunctionDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn visibility(&self) -> FunctionVisibility {
        create_function_visibility(&self.ir_node.visibility, &self.semantic)
    }

    pub fn mutability(&self) -> FunctionMutability {
        create_function_mutability(&self.ir_node.mutability, &self.semantic)
    }

    pub fn virtual_keyword(&self) -> Option<VirtualKeyword> {
        self.ir_node
            .virtual_keyword
            .as_ref()
            .map(|ir_node| create_virtual_keyword(ir_node, &self.semantic))
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionType = Rc<FunctionTypeStruct>;

pub struct FunctionTypeStruct {
    pub(crate) ir_node: ir::FunctionType,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_function_type(
    ir_node: &ir::FunctionType,
    semantic: &Rc<SemanticContext>,
) -> FunctionType {
    Rc::new(FunctionTypeStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl FunctionTypeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn visibility(&self) -> FunctionVisibility {
        create_function_visibility(&self.ir_node.visibility, &self.semantic)
    }

    pub fn mutability(&self) -> FunctionMutability {
        create_function_mutability(&self.ir_node.mutability, &self.semantic)
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type HexNumberExpression = Rc<HexNumberExpressionStruct>;

pub struct HexNumberExpressionStruct {
    pub(crate) ir_node: ir::HexNumberExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_hex_number_expression(
    ir_node: &ir::HexNumberExpression,
    semantic: &Rc<SemanticContext>,
) -> HexNumberExpression {
    Rc::new(HexNumberExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type IfStatement = Rc<IfStatementStruct>;

pub struct IfStatementStruct {
    pub(crate) ir_node: ir::IfStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_if_statement(
    ir_node: &ir::IfStatement,
    semantic: &Rc<SemanticContext>,
) -> IfStatement {
    Rc::new(IfStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

pub struct ImportDeconstructionStruct {
    pub(crate) ir_node: ir::ImportDeconstruction,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_import_deconstruction(
    ir_node: &ir::ImportDeconstruction,
    semantic: &Rc<SemanticContext>,
) -> ImportDeconstruction {
    Rc::new(ImportDeconstructionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

pub struct ImportDeconstructionSymbolStruct {
    pub(crate) ir_node: ir::ImportDeconstructionSymbol,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_import_deconstruction_symbol(
    ir_node: &ir::ImportDeconstructionSymbol,
    semantic: &Rc<SemanticContext>,
) -> ImportDeconstructionSymbol {
    Rc::new(ImportDeconstructionSymbolStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type IndexAccessExpression = Rc<IndexAccessExpressionStruct>;

pub struct IndexAccessExpressionStruct {
    pub(crate) ir_node: ir::IndexAccessExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_index_access_expression(
    ir_node: &ir::IndexAccessExpression,
    semantic: &Rc<SemanticContext>,
) -> IndexAccessExpression {
    Rc::new(IndexAccessExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type InequalityExpression = Rc<InequalityExpressionStruct>;

pub struct InequalityExpressionStruct {
    pub(crate) ir_node: ir::InequalityExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_inequality_expression(
    ir_node: &ir::InequalityExpression,
    semantic: &Rc<SemanticContext>,
) -> InequalityExpression {
    Rc::new(InequalityExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type InheritanceType = Rc<InheritanceTypeStruct>;

pub struct InheritanceTypeStruct {
    pub(crate) ir_node: ir::InheritanceType,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_inheritance_type(
    ir_node: &ir::InheritanceType,
    semantic: &Rc<SemanticContext>,
) -> InheritanceType {
    Rc::new(InheritanceTypeStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

pub struct InterfaceDefinitionStruct {
    pub(crate) ir_node: ir::InterfaceDefinition,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_interface_definition(
    ir_node: &ir::InterfaceDefinition,
    semantic: &Rc<SemanticContext>,
) -> InterfaceDefinition {
    Rc::new(InterfaceDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type LibraryDefinition = Rc<LibraryDefinitionStruct>;

pub struct LibraryDefinitionStruct {
    pub(crate) ir_node: ir::LibraryDefinition,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_library_definition(
    ir_node: &ir::LibraryDefinition,
    semantic: &Rc<SemanticContext>,
) -> LibraryDefinition {
    Rc::new(LibraryDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MappingType = Rc<MappingTypeStruct>;

pub struct MappingTypeStruct {
    pub(crate) ir_node: ir::MappingType,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_mapping_type(
    ir_node: &ir::MappingType,
    semantic: &Rc<SemanticContext>,
) -> MappingType {
    Rc::new(MappingTypeStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

pub struct MemberAccessExpressionStruct {
    pub(crate) ir_node: ir::MemberAccessExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_member_access_expression(
    ir_node: &ir::MemberAccessExpression,
    semantic: &Rc<SemanticContext>,
) -> MemberAccessExpression {
    Rc::new(MemberAccessExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ModifierInvocation = Rc<ModifierInvocationStruct>;

pub struct ModifierInvocationStruct {
    pub(crate) ir_node: ir::ModifierInvocation,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_modifier_invocation(
    ir_node: &ir::ModifierInvocation,
    semantic: &Rc<SemanticContext>,
) -> ModifierInvocation {
    Rc::new(ModifierInvocationStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MultiTypedDeclaration = Rc<MultiTypedDeclarationStruct>;

pub struct MultiTypedDeclarationStruct {
    pub(crate) ir_node: ir::MultiTypedDeclaration,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_multi_typed_declaration(
    ir_node: &ir::MultiTypedDeclaration,
    semantic: &Rc<SemanticContext>,
) -> MultiTypedDeclaration {
    Rc::new(MultiTypedDeclarationStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MultiTypedDeclarationElement = Rc<MultiTypedDeclarationElementStruct>;

pub struct MultiTypedDeclarationElementStruct {
    pub(crate) ir_node: ir::MultiTypedDeclarationElement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_multi_typed_declaration_element(
    ir_node: &ir::MultiTypedDeclarationElement,
    semantic: &Rc<SemanticContext>,
) -> MultiTypedDeclarationElement {
    Rc::new(MultiTypedDeclarationElementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MultiplicativeExpression = Rc<MultiplicativeExpressionStruct>;

pub struct MultiplicativeExpressionStruct {
    pub(crate) ir_node: ir::MultiplicativeExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_multiplicative_expression(
    ir_node: &ir::MultiplicativeExpression,
    semantic: &Rc<SemanticContext>,
) -> MultiplicativeExpression {
    Rc::new(MultiplicativeExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type NamedArgument = Rc<NamedArgumentStruct>;

pub struct NamedArgumentStruct {
    pub(crate) ir_node: ir::NamedArgument,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_named_argument(
    ir_node: &ir::NamedArgument,
    semantic: &Rc<SemanticContext>,
) -> NamedArgument {
    Rc::new(NamedArgumentStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type NewExpression = Rc<NewExpressionStruct>;

pub struct NewExpressionStruct {
    pub(crate) ir_node: ir::NewExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_new_expression(
    ir_node: &ir::NewExpression,
    semantic: &Rc<SemanticContext>,
) -> NewExpression {
    Rc::new(NewExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type OrExpression = Rc<OrExpressionStruct>;

pub struct OrExpressionStruct {
    pub(crate) ir_node: ir::OrExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_or_expression(
    ir_node: &ir::OrExpression,
    semantic: &Rc<SemanticContext>,
) -> OrExpression {
    Rc::new(OrExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Parameter = Rc<ParameterStruct>;

pub struct ParameterStruct {
    pub(crate) ir_node: ir::Parameter,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_parameter(ir_node: &ir::Parameter, semantic: &Rc<SemanticContext>) -> Parameter {
    Rc::new(ParameterStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn indexed(&self) -> Option<IndexedKeyword> {
        self.ir_node
            .indexed
            .as_ref()
            .map(|ir_node| create_indexed_keyword(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PathImport = Rc<PathImportStruct>;

pub struct PathImportStruct {
    pub(crate) ir_node: ir::PathImport,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_path_import(ir_node: &ir::PathImport, semantic: &Rc<SemanticContext>) -> PathImport {
    Rc::new(PathImportStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PostfixExpression = Rc<PostfixExpressionStruct>;

pub struct PostfixExpressionStruct {
    pub(crate) ir_node: ir::PostfixExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_postfix_expression(
    ir_node: &ir::PostfixExpression,
    semantic: &Rc<SemanticContext>,
) -> PostfixExpression {
    Rc::new(PostfixExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaDirective = Rc<PragmaDirectiveStruct>;

pub struct PragmaDirectiveStruct {
    pub(crate) ir_node: ir::PragmaDirective,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_pragma_directive(
    ir_node: &ir::PragmaDirective,
    semantic: &Rc<SemanticContext>,
) -> PragmaDirective {
    Rc::new(PragmaDirectiveStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PrefixExpression = Rc<PrefixExpressionStruct>;

pub struct PrefixExpressionStruct {
    pub(crate) ir_node: ir::PrefixExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_prefix_expression(
    ir_node: &ir::PrefixExpression,
    semantic: &Rc<SemanticContext>,
) -> PrefixExpression {
    Rc::new(PrefixExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ReturnStatement = Rc<ReturnStatementStruct>;

pub struct ReturnStatementStruct {
    pub(crate) ir_node: ir::ReturnStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_return_statement(
    ir_node: &ir::ReturnStatement,
    semantic: &Rc<SemanticContext>,
) -> ReturnStatement {
    Rc::new(ReturnStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type RevertStatement = Rc<RevertStatementStruct>;

pub struct RevertStatementStruct {
    pub(crate) ir_node: ir::RevertStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_revert_statement(
    ir_node: &ir::RevertStatement,
    semantic: &Rc<SemanticContext>,
) -> RevertStatement {
    Rc::new(RevertStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ShiftExpression = Rc<ShiftExpressionStruct>;

pub struct ShiftExpressionStruct {
    pub(crate) ir_node: ir::ShiftExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_shift_expression(
    ir_node: &ir::ShiftExpression,
    semantic: &Rc<SemanticContext>,
) -> ShiftExpression {
    Rc::new(ShiftExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type SingleTypedDeclaration = Rc<SingleTypedDeclarationStruct>;

pub struct SingleTypedDeclarationStruct {
    pub(crate) ir_node: ir::SingleTypedDeclaration,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_single_typed_declaration(
    ir_node: &ir::SingleTypedDeclaration,
    semantic: &Rc<SemanticContext>,
) -> SingleTypedDeclaration {
    Rc::new(SingleTypedDeclarationStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type SourceUnit = Rc<SourceUnitStruct>;

pub struct SourceUnitStruct {
    pub(crate) ir_node: ir::SourceUnit,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_source_unit(ir_node: &ir::SourceUnit, semantic: &Rc<SemanticContext>) -> SourceUnit {
    Rc::new(SourceUnitStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

pub struct StateVariableDefinitionStruct {
    pub(crate) ir_node: ir::StateVariableDefinition,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_state_variable_definition(
    ir_node: &ir::StateVariableDefinition,
    semantic: &Rc<SemanticContext>,
) -> StateVariableDefinition {
    Rc::new(StateVariableDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StructDefinition = Rc<StructDefinitionStruct>;

pub struct StructDefinitionStruct {
    pub(crate) ir_node: ir::StructDefinition,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_struct_definition(
    ir_node: &ir::StructDefinition,
    semantic: &Rc<SemanticContext>,
) -> StructDefinition {
    Rc::new(StructDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StructMember = Rc<StructMemberStruct>;

pub struct StructMemberStruct {
    pub(crate) ir_node: ir::StructMember,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_struct_member(
    ir_node: &ir::StructMember,
    semantic: &Rc<SemanticContext>,
) -> StructMember {
    Rc::new(StructMemberStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type TryStatement = Rc<TryStatementStruct>;

pub struct TryStatementStruct {
    pub(crate) ir_node: ir::TryStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_try_statement(
    ir_node: &ir::TryStatement,
    semantic: &Rc<SemanticContext>,
) -> TryStatement {
    Rc::new(TryStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type TupleExpression = Rc<TupleExpressionStruct>;

pub struct TupleExpressionStruct {
    pub(crate) ir_node: ir::TupleExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_tuple_expression(
    ir_node: &ir::TupleExpression,
    semantic: &Rc<SemanticContext>,
) -> TupleExpression {
    Rc::new(TupleExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type TupleValue = Rc<TupleValueStruct>;

pub struct TupleValueStruct {
    pub(crate) ir_node: ir::TupleValue,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_tuple_value(ir_node: &ir::TupleValue, semantic: &Rc<SemanticContext>) -> TupleValue {
    Rc::new(TupleValueStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type TypeExpression = Rc<TypeExpressionStruct>;

pub struct TypeExpressionStruct {
    pub(crate) ir_node: ir::TypeExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_type_expression(
    ir_node: &ir::TypeExpression,
    semantic: &Rc<SemanticContext>,
) -> TypeExpression {
    Rc::new(TypeExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UncheckedBlock = Rc<UncheckedBlockStruct>;

pub struct UncheckedBlockStruct {
    pub(crate) ir_node: ir::UncheckedBlock,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_unchecked_block(
    ir_node: &ir::UncheckedBlock,
    semantic: &Rc<SemanticContext>,
) -> UncheckedBlock {
    Rc::new(UncheckedBlockStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UserDefinedValueTypeDefinition = Rc<UserDefinedValueTypeDefinitionStruct>;

pub struct UserDefinedValueTypeDefinitionStruct {
    pub(crate) ir_node: ir::UserDefinedValueTypeDefinition,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_user_defined_value_type_definition(
    ir_node: &ir::UserDefinedValueTypeDefinition,
    semantic: &Rc<SemanticContext>,
) -> UserDefinedValueTypeDefinition {
    Rc::new(UserDefinedValueTypeDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UsingDeconstruction = Rc<UsingDeconstructionStruct>;

pub struct UsingDeconstructionStruct {
    pub(crate) ir_node: ir::UsingDeconstruction,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_using_deconstruction(
    ir_node: &ir::UsingDeconstruction,
    semantic: &Rc<SemanticContext>,
) -> UsingDeconstruction {
    Rc::new(UsingDeconstructionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UsingDeconstructionSymbol = Rc<UsingDeconstructionSymbolStruct>;

pub struct UsingDeconstructionSymbolStruct {
    pub(crate) ir_node: ir::UsingDeconstructionSymbol,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_using_deconstruction_symbol(
    ir_node: &ir::UsingDeconstructionSymbol,
    semantic: &Rc<SemanticContext>,
) -> UsingDeconstructionSymbol {
    Rc::new(UsingDeconstructionSymbolStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UsingDirective = Rc<UsingDirectiveStruct>;

pub struct UsingDirectiveStruct {
    pub(crate) ir_node: ir::UsingDirective,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_using_directive(
    ir_node: &ir::UsingDirective,
    semantic: &Rc<SemanticContext>,
) -> UsingDirective {
    Rc::new(UsingDirectiveStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn global_keyword(&self) -> Option<GlobalKeyword> {
        self.ir_node
            .global_keyword
            .as_ref()
            .map(|ir_node| create_global_keyword(ir_node, &self.semantic))
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VariableDeclaration = Rc<VariableDeclarationStruct>;

pub struct VariableDeclarationStruct {
    pub(crate) ir_node: ir::VariableDeclaration,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_variable_declaration(
    ir_node: &ir::VariableDeclaration,
    semantic: &Rc<SemanticContext>,
) -> VariableDeclaration {
    Rc::new(VariableDeclarationStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

pub struct VariableDeclarationStatementStruct {
    pub(crate) ir_node: ir::VariableDeclarationStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_variable_declaration_statement(
    ir_node: &ir::VariableDeclarationStatement,
    semantic: &Rc<SemanticContext>,
) -> VariableDeclarationStatement {
    Rc::new(VariableDeclarationStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VersionPragma = Rc<VersionPragmaStruct>;

pub struct VersionPragmaStruct {
    pub(crate) ir_node: ir::VersionPragma,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_version_pragma(
    ir_node: &ir::VersionPragma,
    semantic: &Rc<SemanticContext>,
) -> VersionPragma {
    Rc::new(VersionPragmaStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VersionRange = Rc<VersionRangeStruct>;

pub struct VersionRangeStruct {
    pub(crate) ir_node: ir::VersionRange,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_version_range(
    ir_node: &ir::VersionRange,
    semantic: &Rc<SemanticContext>,
) -> VersionRange {
    Rc::new(VersionRangeStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VersionTerm = Rc<VersionTermStruct>;

pub struct VersionTermStruct {
    pub(crate) ir_node: ir::VersionTerm,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_version_term(
    ir_node: &ir::VersionTerm,
    semantic: &Rc<SemanticContext>,
) -> VersionTerm {
    Rc::new(VersionTermStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type WhileStatement = Rc<WhileStatementStruct>;

pub struct WhileStatementStruct {
    pub(crate) ir_node: ir::WhileStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_while_statement(
    ir_node: &ir::WhileStatement,
    semantic: &Rc<SemanticContext>,
) -> WhileStatement {
    Rc::new(WhileStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulBlock = Rc<YulBlockStruct>;

pub struct YulBlockStruct {
    pub(crate) ir_node: ir::YulBlock,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_block(ir_node: &ir::YulBlock, semantic: &Rc<SemanticContext>) -> YulBlock {
    Rc::new(YulBlockStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulBreakStatement = Rc<YulBreakStatementStruct>;

pub struct YulBreakStatementStruct {
    pub(crate) ir_node: ir::YulBreakStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_break_statement(
    ir_node: &ir::YulBreakStatement,
    semantic: &Rc<SemanticContext>,
) -> YulBreakStatement {
    Rc::new(YulBreakStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulBreakStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulContinueStatement = Rc<YulContinueStatementStruct>;

pub struct YulContinueStatementStruct {
    pub(crate) ir_node: ir::YulContinueStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_continue_statement(
    ir_node: &ir::YulContinueStatement,
    semantic: &Rc<SemanticContext>,
) -> YulContinueStatement {
    Rc::new(YulContinueStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulContinueStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulDefaultCase = Rc<YulDefaultCaseStruct>;

pub struct YulDefaultCaseStruct {
    pub(crate) ir_node: ir::YulDefaultCase,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_default_case(
    ir_node: &ir::YulDefaultCase,
    semantic: &Rc<SemanticContext>,
) -> YulDefaultCase {
    Rc::new(YulDefaultCaseStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulForStatement = Rc<YulForStatementStruct>;

pub struct YulForStatementStruct {
    pub(crate) ir_node: ir::YulForStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_for_statement(
    ir_node: &ir::YulForStatement,
    semantic: &Rc<SemanticContext>,
) -> YulForStatement {
    Rc::new(YulForStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulFunctionCallExpression = Rc<YulFunctionCallExpressionStruct>;

pub struct YulFunctionCallExpressionStruct {
    pub(crate) ir_node: ir::YulFunctionCallExpression,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_function_call_expression(
    ir_node: &ir::YulFunctionCallExpression,
    semantic: &Rc<SemanticContext>,
) -> YulFunctionCallExpression {
    Rc::new(YulFunctionCallExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulFunctionDefinition = Rc<YulFunctionDefinitionStruct>;

pub struct YulFunctionDefinitionStruct {
    pub(crate) ir_node: ir::YulFunctionDefinition,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_function_definition(
    ir_node: &ir::YulFunctionDefinition,
    semantic: &Rc<SemanticContext>,
) -> YulFunctionDefinition {
    Rc::new(YulFunctionDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulIfStatement = Rc<YulIfStatementStruct>;

pub struct YulIfStatementStruct {
    pub(crate) ir_node: ir::YulIfStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_if_statement(
    ir_node: &ir::YulIfStatement,
    semantic: &Rc<SemanticContext>,
) -> YulIfStatement {
    Rc::new(YulIfStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulLeaveStatement = Rc<YulLeaveStatementStruct>;

pub struct YulLeaveStatementStruct {
    pub(crate) ir_node: ir::YulLeaveStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_leave_statement(
    ir_node: &ir::YulLeaveStatement,
    semantic: &Rc<SemanticContext>,
) -> YulLeaveStatement {
    Rc::new(YulLeaveStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulLeaveStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulSwitchStatement = Rc<YulSwitchStatementStruct>;

pub struct YulSwitchStatementStruct {
    pub(crate) ir_node: ir::YulSwitchStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_switch_statement(
    ir_node: &ir::YulSwitchStatement,
    semantic: &Rc<SemanticContext>,
) -> YulSwitchStatement {
    Rc::new(YulSwitchStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulValueCase = Rc<YulValueCaseStruct>;

pub struct YulValueCaseStruct {
    pub(crate) ir_node: ir::YulValueCase,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_value_case(
    ir_node: &ir::YulValueCase,
    semantic: &Rc<SemanticContext>,
) -> YulValueCase {
    Rc::new(YulValueCaseStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulVariableAssignmentStatement = Rc<YulVariableAssignmentStatementStruct>;

pub struct YulVariableAssignmentStatementStruct {
    pub(crate) ir_node: ir::YulVariableAssignmentStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_variable_assignment_statement(
    ir_node: &ir::YulVariableAssignmentStatement,
    semantic: &Rc<SemanticContext>,
) -> YulVariableAssignmentStatement {
    Rc::new(YulVariableAssignmentStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulVariableDeclarationStatement = Rc<YulVariableDeclarationStatementStruct>;

pub struct YulVariableDeclarationStatementStruct {
    pub(crate) ir_node: ir::YulVariableDeclarationStatement,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_variable_declaration_statement(
    ir_node: &ir::YulVariableDeclarationStatement,
    semantic: &Rc<SemanticContext>,
) -> YulVariableDeclarationStatement {
    Rc::new(YulVariableDeclarationStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type YulVariableDeclarationValue = Rc<YulVariableDeclarationValueStruct>;

pub struct YulVariableDeclarationValueStruct {
    pub(crate) ir_node: ir::YulVariableDeclarationValue,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub fn create_yul_variable_declaration_value(
    ir_node: &ir::YulVariableDeclarationValue,
    semantic: &Rc<SemanticContext>,
) -> YulVariableDeclarationValue {
    Rc::new(YulVariableDeclarationValueStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
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

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

//
// Choices
//

pub enum AbicoderVersion {
    AbicoderV1Keyword(AbicoderV1Keyword),
    AbicoderV2Keyword(AbicoderV2Keyword),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_abicoder_version(
    ir_node: &ir::AbicoderVersion,
    semantic: &Rc<SemanticContext>,
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

pub enum AdditiveExpressionOperator {
    Minus(Minus),
    Plus(Plus),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_additive_expression_operator(
    ir_node: &ir::AdditiveExpressionOperator,
    semantic: &Rc<SemanticContext>,
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

pub enum ArgumentsDeclaration {
    PositionalArguments(PositionalArguments),
    NamedArguments(NamedArguments),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_arguments_declaration(
    ir_node: &ir::ArgumentsDeclaration,
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
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

pub enum EqualityExpressionOperator {
    BangEqual(BangEqual),
    EqualEqual(EqualEqual),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_equality_expression_operator(
    ir_node: &ir::EqualityExpressionOperator,
    semantic: &Rc<SemanticContext>,
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

pub enum ExperimentalFeature {
    ABIEncoderV2Keyword(ABIEncoderV2Keyword),
    SMTCheckerKeyword(SMTCheckerKeyword),
    StringLiteral(StringLiteral),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_experimental_feature(
    ir_node: &ir::ExperimentalFeature,
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
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

pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    Semicolon(Semicolon),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_for_statement_condition(
    ir_node: &ir::ForStatementCondition,
    semantic: &Rc<SemanticContext>,
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

pub enum ForStatementInitialization {
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Semicolon(Semicolon),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_for_statement_initialization(
    ir_node: &ir::ForStatementInitialization,
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
) -> FunctionKind {
    match ir_node {
        ir::FunctionKind::Regular => FunctionKind::Regular,
        ir::FunctionKind::Constructor => FunctionKind::Constructor,
        ir::FunctionKind::Fallback => FunctionKind::Fallback,
        ir::FunctionKind::Receive => FunctionKind::Receive,
        ir::FunctionKind::Modifier => FunctionKind::Modifier,
    }
}

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
    semantic: &Rc<SemanticContext>,
) -> FunctionMutability {
    match ir_node {
        ir::FunctionMutability::Pure => FunctionMutability::Pure,
        ir::FunctionMutability::View => FunctionMutability::View,
        ir::FunctionMutability::NonPayable => FunctionMutability::NonPayable,
        ir::FunctionMutability::Payable => FunctionMutability::Payable,
    }
}

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
    semantic: &Rc<SemanticContext>,
) -> FunctionVisibility {
    match ir_node {
        ir::FunctionVisibility::Public => FunctionVisibility::Public,
        ir::FunctionVisibility::Private => FunctionVisibility::Private,
        ir::FunctionVisibility::Internal => FunctionVisibility::Internal,
        ir::FunctionVisibility::External => FunctionVisibility::External,
    }
}

pub enum ImportClause {
    PathImport(PathImport),
    ImportDeconstruction(ImportDeconstruction),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_import_clause(
    ir_node: &ir::ImportClause,
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
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

pub enum MultiplicativeExpressionOperator {
    Asterisk(Asterisk),
    Percent(Percent),
    Slash(Slash),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_multiplicative_expression_operator(
    ir_node: &ir::MultiplicativeExpressionOperator,
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
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

pub enum PostfixExpressionOperator {
    MinusMinus(MinusMinus),
    PlusPlus(PlusPlus),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_postfix_expression_operator(
    ir_node: &ir::PostfixExpressionOperator,
    semantic: &Rc<SemanticContext>,
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

pub enum Pragma {
    VersionPragma(VersionPragma),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_pragma(ir_node: &ir::Pragma, semantic: &Rc<SemanticContext>) -> Pragma {
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
    semantic: &Rc<SemanticContext>,
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

pub enum ShiftExpressionOperator {
    GreaterThanGreaterThan(GreaterThanGreaterThan),
    GreaterThanGreaterThanGreaterThan(GreaterThanGreaterThanGreaterThan),
    LessThanLessThan(LessThanLessThan),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_shift_expression_operator(
    ir_node: &ir::ShiftExpressionOperator,
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
) -> StateVariableMutability {
    match ir_node {
        ir::StateVariableMutability::Mutable => StateVariableMutability::Mutable,
        ir::StateVariableMutability::Constant => StateVariableMutability::Constant,
        ir::StateVariableMutability::Immutable => StateVariableMutability::Immutable,
        ir::StateVariableMutability::Transient => StateVariableMutability::Transient,
    }
}

pub enum StateVariableVisibility {
    Public,
    Private,
    Internal,
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_state_variable_visibility(
    ir_node: &ir::StateVariableVisibility,
    semantic: &Rc<SemanticContext>,
) -> StateVariableVisibility {
    match ir_node {
        ir::StateVariableVisibility::Public => StateVariableVisibility::Public,
        ir::StateVariableVisibility::Private => StateVariableVisibility::Private,
        ir::StateVariableVisibility::Internal => StateVariableVisibility::Internal,
    }
}

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
    semantic: &Rc<SemanticContext>,
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

pub enum StorageLocation {
    MemoryKeyword(MemoryKeyword),
    StorageKeyword(StorageKeyword),
    CallDataKeyword(CallDataKeyword),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_storage_location(
    ir_node: &ir::StorageLocation,
    semantic: &Rc<SemanticContext>,
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

pub enum StringExpression {
    StringLiterals(StringLiterals),
    HexStringLiterals(HexStringLiterals),
    UnicodeStringLiterals(UnicodeStringLiterals),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_string_expression(
    ir_node: &ir::StringExpression,
    semantic: &Rc<SemanticContext>,
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

pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    FunctionType(FunctionType),
    MappingType(MappingType),
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_type_name(ir_node: &ir::TypeName, semantic: &Rc<SemanticContext>) -> TypeName {
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

pub enum UsingClause {
    IdentifierPath(IdentifierPath),
    UsingDeconstruction(UsingDeconstruction),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_using_clause(
    ir_node: &ir::UsingClause,
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
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

pub enum UsingTarget {
    TypeName(TypeName),
    Asterisk(Asterisk),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_using_target(
    ir_node: &ir::UsingTarget,
    semantic: &Rc<SemanticContext>,
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

pub enum VariableDeclarationTarget {
    SingleTypedDeclaration(SingleTypedDeclaration),
    MultiTypedDeclaration(MultiTypedDeclaration),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_variable_declaration_target(
    ir_node: &ir::VariableDeclarationTarget,
    semantic: &Rc<SemanticContext>,
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

pub enum VersionExpression {
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_version_expression(
    ir_node: &ir::VersionExpression,
    semantic: &Rc<SemanticContext>,
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

pub enum VersionLiteral {
    SimpleVersionLiteral(SimpleVersionLiteral),
    StringLiteral(StringLiteral),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_version_literal(
    ir_node: &ir::VersionLiteral,
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
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

pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_yul_expression(
    ir_node: &ir::YulExpression,
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
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
    semantic: &Rc<SemanticContext>,
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

pub enum YulSwitchCase {
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_yul_switch_case(
    ir_node: &ir::YulSwitchCase,
    semantic: &Rc<SemanticContext>,
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

pub type ArrayValues = Rc<ArrayValuesStruct>;

pub(crate) fn create_array_values(
    nodes: &[ir::Expression],
    semantic: &Rc<SemanticContext>,
) -> ArrayValues {
    Rc::new(ArrayValuesStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct ArrayValuesStruct {
    pub(crate) ir_nodes: Vec<ir::Expression>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type CallOptions = Rc<CallOptionsStruct>;

pub(crate) fn create_call_options(
    nodes: &[ir::NamedArgument],
    semantic: &Rc<SemanticContext>,
) -> CallOptions {
    Rc::new(CallOptionsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct CallOptionsStruct {
    pub(crate) ir_nodes: Vec<ir::NamedArgument>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type CatchClauses = Rc<CatchClausesStruct>;

pub(crate) fn create_catch_clauses(
    nodes: &[ir::CatchClause],
    semantic: &Rc<SemanticContext>,
) -> CatchClauses {
    Rc::new(CatchClausesStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct CatchClausesStruct {
    pub(crate) ir_nodes: Vec<ir::CatchClause>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type ContractMembers = Rc<ContractMembersStruct>;

pub(crate) fn create_contract_members(
    nodes: &[ir::ContractMember],
    semantic: &Rc<SemanticContext>,
) -> ContractMembers {
    Rc::new(ContractMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct ContractMembersStruct {
    pub(crate) ir_nodes: Vec<ir::ContractMember>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type EnumMembers = Rc<EnumMembersStruct>;

pub(crate) fn create_enum_members(
    nodes: &[ir::Identifier],
    semantic: &Rc<SemanticContext>,
) -> EnumMembers {
    Rc::new(EnumMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct EnumMembersStruct {
    pub(crate) ir_nodes: Vec<ir::Identifier>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type HexStringLiterals = Rc<HexStringLiteralsStruct>;

pub(crate) fn create_hex_string_literals(
    nodes: &[ir::HexStringLiteral],
    semantic: &Rc<SemanticContext>,
) -> HexStringLiterals {
    Rc::new(HexStringLiteralsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct HexStringLiteralsStruct {
    pub(crate) ir_nodes: Vec<ir::HexStringLiteral>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type IdentifierPath = Rc<IdentifierPathStruct>;

pub(crate) fn create_identifier_path(
    nodes: &[ir::Identifier],
    semantic: &Rc<SemanticContext>,
) -> IdentifierPath {
    Rc::new(IdentifierPathStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct IdentifierPathStruct {
    pub(crate) ir_nodes: Vec<ir::Identifier>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type ImportDeconstructionSymbols = Rc<ImportDeconstructionSymbolsStruct>;

pub(crate) fn create_import_deconstruction_symbols(
    nodes: &[ir::ImportDeconstructionSymbol],
    semantic: &Rc<SemanticContext>,
) -> ImportDeconstructionSymbols {
    Rc::new(ImportDeconstructionSymbolsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct ImportDeconstructionSymbolsStruct {
    pub(crate) ir_nodes: Vec<ir::ImportDeconstructionSymbol>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type InheritanceTypes = Rc<InheritanceTypesStruct>;

pub(crate) fn create_inheritance_types(
    nodes: &[ir::InheritanceType],
    semantic: &Rc<SemanticContext>,
) -> InheritanceTypes {
    Rc::new(InheritanceTypesStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct InheritanceTypesStruct {
    pub(crate) ir_nodes: Vec<ir::InheritanceType>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type InterfaceMembers = Rc<InterfaceMembersStruct>;

pub(crate) fn create_interface_members(
    nodes: &[ir::ContractMember],
    semantic: &Rc<SemanticContext>,
) -> InterfaceMembers {
    Rc::new(InterfaceMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct InterfaceMembersStruct {
    pub(crate) ir_nodes: Vec<ir::ContractMember>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type LibraryMembers = Rc<LibraryMembersStruct>;

pub(crate) fn create_library_members(
    nodes: &[ir::ContractMember],
    semantic: &Rc<SemanticContext>,
) -> LibraryMembers {
    Rc::new(LibraryMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct LibraryMembersStruct {
    pub(crate) ir_nodes: Vec<ir::ContractMember>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type ModifierInvocations = Rc<ModifierInvocationsStruct>;

pub(crate) fn create_modifier_invocations(
    nodes: &[ir::ModifierInvocation],
    semantic: &Rc<SemanticContext>,
) -> ModifierInvocations {
    Rc::new(ModifierInvocationsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct ModifierInvocationsStruct {
    pub(crate) ir_nodes: Vec<ir::ModifierInvocation>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type MultiTypedDeclarationElements = Rc<MultiTypedDeclarationElementsStruct>;

pub(crate) fn create_multi_typed_declaration_elements(
    nodes: &[ir::MultiTypedDeclarationElement],
    semantic: &Rc<SemanticContext>,
) -> MultiTypedDeclarationElements {
    Rc::new(MultiTypedDeclarationElementsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct MultiTypedDeclarationElementsStruct {
    pub(crate) ir_nodes: Vec<ir::MultiTypedDeclarationElement>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type NamedArguments = Rc<NamedArgumentsStruct>;

pub(crate) fn create_named_arguments(
    nodes: &[ir::NamedArgument],
    semantic: &Rc<SemanticContext>,
) -> NamedArguments {
    Rc::new(NamedArgumentsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct NamedArgumentsStruct {
    pub(crate) ir_nodes: Vec<ir::NamedArgument>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type OverridePaths = Rc<OverridePathsStruct>;

pub(crate) fn create_override_paths(
    nodes: &[ir::IdentifierPath],
    semantic: &Rc<SemanticContext>,
) -> OverridePaths {
    Rc::new(OverridePathsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct OverridePathsStruct {
    pub(crate) ir_nodes: Vec<ir::IdentifierPath>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type Parameters = Rc<ParametersStruct>;

pub(crate) fn create_parameters(
    nodes: &[ir::Parameter],
    semantic: &Rc<SemanticContext>,
) -> Parameters {
    Rc::new(ParametersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct ParametersStruct {
    pub(crate) ir_nodes: Vec<ir::Parameter>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type PositionalArguments = Rc<PositionalArgumentsStruct>;

pub(crate) fn create_positional_arguments(
    nodes: &[ir::Expression],
    semantic: &Rc<SemanticContext>,
) -> PositionalArguments {
    Rc::new(PositionalArgumentsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct PositionalArgumentsStruct {
    pub(crate) ir_nodes: Vec<ir::Expression>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type SimpleVersionLiteral = Rc<SimpleVersionLiteralStruct>;

pub(crate) fn create_simple_version_literal(
    nodes: &[ir::VersionSpecifier],
    semantic: &Rc<SemanticContext>,
) -> SimpleVersionLiteral {
    Rc::new(SimpleVersionLiteralStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct SimpleVersionLiteralStruct {
    pub(crate) ir_nodes: Vec<ir::VersionSpecifier>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type SourceUnitMembers = Rc<SourceUnitMembersStruct>;

pub(crate) fn create_source_unit_members(
    nodes: &[ir::SourceUnitMember],
    semantic: &Rc<SemanticContext>,
) -> SourceUnitMembers {
    Rc::new(SourceUnitMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct SourceUnitMembersStruct {
    pub(crate) ir_nodes: Vec<ir::SourceUnitMember>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type Statements = Rc<StatementsStruct>;

pub(crate) fn create_statements(
    nodes: &[ir::Statement],
    semantic: &Rc<SemanticContext>,
) -> Statements {
    Rc::new(StatementsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct StatementsStruct {
    pub(crate) ir_nodes: Vec<ir::Statement>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type StringLiterals = Rc<StringLiteralsStruct>;

pub(crate) fn create_string_literals(
    nodes: &[ir::StringLiteral],
    semantic: &Rc<SemanticContext>,
) -> StringLiterals {
    Rc::new(StringLiteralsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct StringLiteralsStruct {
    pub(crate) ir_nodes: Vec<ir::StringLiteral>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type StructMembers = Rc<StructMembersStruct>;

pub(crate) fn create_struct_members(
    nodes: &[ir::StructMember],
    semantic: &Rc<SemanticContext>,
) -> StructMembers {
    Rc::new(StructMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct StructMembersStruct {
    pub(crate) ir_nodes: Vec<ir::StructMember>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type TupleValues = Rc<TupleValuesStruct>;

pub(crate) fn create_tuple_values(
    nodes: &[ir::TupleValue],
    semantic: &Rc<SemanticContext>,
) -> TupleValues {
    Rc::new(TupleValuesStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct TupleValuesStruct {
    pub(crate) ir_nodes: Vec<ir::TupleValue>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type UnicodeStringLiterals = Rc<UnicodeStringLiteralsStruct>;

pub(crate) fn create_unicode_string_literals(
    nodes: &[ir::UnicodeStringLiteral],
    semantic: &Rc<SemanticContext>,
) -> UnicodeStringLiterals {
    Rc::new(UnicodeStringLiteralsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct UnicodeStringLiteralsStruct {
    pub(crate) ir_nodes: Vec<ir::UnicodeStringLiteral>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type UsingDeconstructionSymbols = Rc<UsingDeconstructionSymbolsStruct>;

pub(crate) fn create_using_deconstruction_symbols(
    nodes: &[ir::UsingDeconstructionSymbol],
    semantic: &Rc<SemanticContext>,
) -> UsingDeconstructionSymbols {
    Rc::new(UsingDeconstructionSymbolsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct UsingDeconstructionSymbolsStruct {
    pub(crate) ir_nodes: Vec<ir::UsingDeconstructionSymbol>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type VersionExpressionSet = Rc<VersionExpressionSetStruct>;

pub(crate) fn create_version_expression_set(
    nodes: &[ir::VersionExpression],
    semantic: &Rc<SemanticContext>,
) -> VersionExpressionSet {
    Rc::new(VersionExpressionSetStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct VersionExpressionSetStruct {
    pub(crate) ir_nodes: Vec<ir::VersionExpression>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type VersionExpressionSets = Rc<VersionExpressionSetsStruct>;

pub(crate) fn create_version_expression_sets(
    nodes: &[ir::VersionExpressionSet],
    semantic: &Rc<SemanticContext>,
) -> VersionExpressionSets {
    Rc::new(VersionExpressionSetsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct VersionExpressionSetsStruct {
    pub(crate) ir_nodes: Vec<ir::VersionExpressionSet>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type YulArguments = Rc<YulArgumentsStruct>;

pub(crate) fn create_yul_arguments(
    nodes: &[ir::YulExpression],
    semantic: &Rc<SemanticContext>,
) -> YulArguments {
    Rc::new(YulArgumentsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulArgumentsStruct {
    pub(crate) ir_nodes: Vec<ir::YulExpression>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type YulFlags = Rc<YulFlagsStruct>;

pub(crate) fn create_yul_flags(
    nodes: &[ir::StringLiteral],
    semantic: &Rc<SemanticContext>,
) -> YulFlags {
    Rc::new(YulFlagsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulFlagsStruct {
    pub(crate) ir_nodes: Vec<ir::StringLiteral>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type YulParameters = Rc<YulParametersStruct>;

pub(crate) fn create_yul_parameters(
    nodes: &[ir::Identifier],
    semantic: &Rc<SemanticContext>,
) -> YulParameters {
    Rc::new(YulParametersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulParametersStruct {
    pub(crate) ir_nodes: Vec<ir::Identifier>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type YulPath = Rc<YulPathStruct>;

pub(crate) fn create_yul_path(nodes: &[ir::Identifier], semantic: &Rc<SemanticContext>) -> YulPath {
    Rc::new(YulPathStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulPathStruct {
    pub(crate) ir_nodes: Vec<ir::Identifier>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type YulPaths = Rc<YulPathsStruct>;

pub(crate) fn create_yul_paths(nodes: &[ir::YulPath], semantic: &Rc<SemanticContext>) -> YulPaths {
    Rc::new(YulPathsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulPathsStruct {
    pub(crate) ir_nodes: Vec<ir::YulPath>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type YulStatements = Rc<YulStatementsStruct>;

pub(crate) fn create_yul_statements(
    nodes: &[ir::YulStatement],
    semantic: &Rc<SemanticContext>,
) -> YulStatements {
    Rc::new(YulStatementsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulStatementsStruct {
    pub(crate) ir_nodes: Vec<ir::YulStatement>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type YulSwitchCases = Rc<YulSwitchCasesStruct>;

pub(crate) fn create_yul_switch_cases(
    nodes: &[ir::YulSwitchCase],
    semantic: &Rc<SemanticContext>,
) -> YulSwitchCases {
    Rc::new(YulSwitchCasesStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulSwitchCasesStruct {
    pub(crate) ir_nodes: Vec<ir::YulSwitchCase>,
    pub(crate) semantic: Rc<SemanticContext>,
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
pub type YulVariableNames = Rc<YulVariableNamesStruct>;

pub(crate) fn create_yul_variable_names(
    nodes: &[ir::Identifier],
    semantic: &Rc<SemanticContext>,
) -> YulVariableNames {
    Rc::new(YulVariableNamesStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulVariableNamesStruct {
    pub(crate) ir_nodes: Vec<ir::Identifier>,
    pub(crate) semantic: Rc<SemanticContext>,
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

pub type ABIEncoderV2Keyword = Rc<ABIEncoderV2KeywordStruct>;

pub struct ABIEncoderV2KeywordStruct {
    pub(crate) ir_node: ir::ABIEncoderV2Keyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_abi_encoder_v2_keyword(
    ir_node: &ir::ABIEncoderV2Keyword,
    semantic: &Rc<SemanticContext>,
) -> ABIEncoderV2Keyword {
    Rc::new(ABIEncoderV2KeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ABIEncoderV2KeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AbicoderV1Keyword = Rc<AbicoderV1KeywordStruct>;

pub struct AbicoderV1KeywordStruct {
    pub(crate) ir_node: ir::AbicoderV1Keyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_abicoder_v1_keyword(
    ir_node: &ir::AbicoderV1Keyword,
    semantic: &Rc<SemanticContext>,
) -> AbicoderV1Keyword {
    Rc::new(AbicoderV1KeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AbicoderV1KeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AbicoderV2Keyword = Rc<AbicoderV2KeywordStruct>;

pub struct AbicoderV2KeywordStruct {
    pub(crate) ir_node: ir::AbicoderV2Keyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_abicoder_v2_keyword(
    ir_node: &ir::AbicoderV2Keyword,
    semantic: &Rc<SemanticContext>,
) -> AbicoderV2Keyword {
    Rc::new(AbicoderV2KeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AbicoderV2KeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AbstractKeyword = Rc<AbstractKeywordStruct>;

pub struct AbstractKeywordStruct {
    pub(crate) ir_node: ir::AbstractKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_abstract_keyword(
    ir_node: &ir::AbstractKeyword,
    semantic: &Rc<SemanticContext>,
) -> AbstractKeyword {
    Rc::new(AbstractKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AbstractKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Ampersand = Rc<AmpersandStruct>;

pub struct AmpersandStruct {
    pub(crate) ir_node: ir::Ampersand,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_ampersand(
    ir_node: &ir::Ampersand,
    semantic: &Rc<SemanticContext>,
) -> Ampersand {
    Rc::new(AmpersandStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AmpersandStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AmpersandEqual = Rc<AmpersandEqualStruct>;

pub struct AmpersandEqualStruct {
    pub(crate) ir_node: ir::AmpersandEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_ampersand_equal(
    ir_node: &ir::AmpersandEqual,
    semantic: &Rc<SemanticContext>,
) -> AmpersandEqual {
    Rc::new(AmpersandEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AmpersandEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AnonymousKeyword = Rc<AnonymousKeywordStruct>;

pub struct AnonymousKeywordStruct {
    pub(crate) ir_node: ir::AnonymousKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_anonymous_keyword(
    ir_node: &ir::AnonymousKeyword,
    semantic: &Rc<SemanticContext>,
) -> AnonymousKeyword {
    Rc::new(AnonymousKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AnonymousKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Asterisk = Rc<AsteriskStruct>;

pub struct AsteriskStruct {
    pub(crate) ir_node: ir::Asterisk,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_asterisk(ir_node: &ir::Asterisk, semantic: &Rc<SemanticContext>) -> Asterisk {
    Rc::new(AsteriskStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AsteriskStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type AsteriskEqual = Rc<AsteriskEqualStruct>;

pub struct AsteriskEqualStruct {
    pub(crate) ir_node: ir::AsteriskEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_asterisk_equal(
    ir_node: &ir::AsteriskEqual,
    semantic: &Rc<SemanticContext>,
) -> AsteriskEqual {
    Rc::new(AsteriskEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AsteriskEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Bang = Rc<BangStruct>;

pub struct BangStruct {
    pub(crate) ir_node: ir::Bang,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_bang(ir_node: &ir::Bang, semantic: &Rc<SemanticContext>) -> Bang {
    Rc::new(BangStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl BangStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BangEqual = Rc<BangEqualStruct>;

pub struct BangEqualStruct {
    pub(crate) ir_node: ir::BangEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_bang_equal(
    ir_node: &ir::BangEqual,
    semantic: &Rc<SemanticContext>,
) -> BangEqual {
    Rc::new(BangEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl BangEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Bar = Rc<BarStruct>;

pub struct BarStruct {
    pub(crate) ir_node: ir::Bar,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_bar(ir_node: &ir::Bar, semantic: &Rc<SemanticContext>) -> Bar {
    Rc::new(BarStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl BarStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BarEqual = Rc<BarEqualStruct>;

pub struct BarEqualStruct {
    pub(crate) ir_node: ir::BarEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_bar_equal(ir_node: &ir::BarEqual, semantic: &Rc<SemanticContext>) -> BarEqual {
    Rc::new(BarEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl BarEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BoolKeyword = Rc<BoolKeywordStruct>;

pub struct BoolKeywordStruct {
    pub(crate) ir_node: ir::BoolKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_bool_keyword(
    ir_node: &ir::BoolKeyword,
    semantic: &Rc<SemanticContext>,
) -> BoolKeyword {
    Rc::new(BoolKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl BoolKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type BytesKeyword = Rc<BytesKeywordStruct>;

pub struct BytesKeywordStruct {
    pub(crate) ir_node: ir::BytesKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_bytes_keyword(
    ir_node: &ir::BytesKeyword,
    semantic: &Rc<SemanticContext>,
) -> BytesKeyword {
    Rc::new(BytesKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl BytesKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }
    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type CallDataKeyword = Rc<CallDataKeywordStruct>;

pub struct CallDataKeywordStruct {
    pub(crate) ir_node: ir::CallDataKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_call_data_keyword(
    ir_node: &ir::CallDataKeyword,
    semantic: &Rc<SemanticContext>,
) -> CallDataKeyword {
    Rc::new(CallDataKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl CallDataKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Caret = Rc<CaretStruct>;

pub struct CaretStruct {
    pub(crate) ir_node: ir::Caret,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_caret(ir_node: &ir::Caret, semantic: &Rc<SemanticContext>) -> Caret {
    Rc::new(CaretStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl CaretStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type CaretEqual = Rc<CaretEqualStruct>;

pub struct CaretEqualStruct {
    pub(crate) ir_node: ir::CaretEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_caret_equal(
    ir_node: &ir::CaretEqual,
    semantic: &Rc<SemanticContext>,
) -> CaretEqual {
    Rc::new(CaretEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl CaretEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type DaysKeyword = Rc<DaysKeywordStruct>;

pub struct DaysKeywordStruct {
    pub(crate) ir_node: ir::DaysKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_days_keyword(
    ir_node: &ir::DaysKeyword,
    semantic: &Rc<SemanticContext>,
) -> DaysKeyword {
    Rc::new(DaysKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl DaysKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type DecimalLiteral = Rc<DecimalLiteralStruct>;

pub struct DecimalLiteralStruct {
    pub(crate) ir_node: ir::DecimalLiteral,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_decimal_literal(
    ir_node: &ir::DecimalLiteral,
    semantic: &Rc<SemanticContext>,
) -> DecimalLiteral {
    Rc::new(DecimalLiteralStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl DecimalLiteralStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }
    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type DeleteKeyword = Rc<DeleteKeywordStruct>;

pub struct DeleteKeywordStruct {
    pub(crate) ir_node: ir::DeleteKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_delete_keyword(
    ir_node: &ir::DeleteKeyword,
    semantic: &Rc<SemanticContext>,
) -> DeleteKeyword {
    Rc::new(DeleteKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl DeleteKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Equal = Rc<EqualStruct>;

pub struct EqualStruct {
    pub(crate) ir_node: ir::Equal,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_equal(ir_node: &ir::Equal, semantic: &Rc<SemanticContext>) -> Equal {
    Rc::new(EqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl EqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type EqualEqual = Rc<EqualEqualStruct>;

pub struct EqualEqualStruct {
    pub(crate) ir_node: ir::EqualEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_equal_equal(
    ir_node: &ir::EqualEqual,
    semantic: &Rc<SemanticContext>,
) -> EqualEqual {
    Rc::new(EqualEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl EqualEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type EtherKeyword = Rc<EtherKeywordStruct>;

pub struct EtherKeywordStruct {
    pub(crate) ir_node: ir::EtherKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_ether_keyword(
    ir_node: &ir::EtherKeyword,
    semantic: &Rc<SemanticContext>,
) -> EtherKeyword {
    Rc::new(EtherKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl EtherKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type FalseKeyword = Rc<FalseKeywordStruct>;

pub struct FalseKeywordStruct {
    pub(crate) ir_node: ir::FalseKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_false_keyword(
    ir_node: &ir::FalseKeyword,
    semantic: &Rc<SemanticContext>,
) -> FalseKeyword {
    Rc::new(FalseKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl FalseKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type FixedKeyword = Rc<FixedKeywordStruct>;

pub struct FixedKeywordStruct {
    pub(crate) ir_node: ir::FixedKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_fixed_keyword(
    ir_node: &ir::FixedKeyword,
    semantic: &Rc<SemanticContext>,
) -> FixedKeyword {
    Rc::new(FixedKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl FixedKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }
    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GlobalKeyword = Rc<GlobalKeywordStruct>;

pub struct GlobalKeywordStruct {
    pub(crate) ir_node: ir::GlobalKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_global_keyword(
    ir_node: &ir::GlobalKeyword,
    semantic: &Rc<SemanticContext>,
) -> GlobalKeyword {
    Rc::new(GlobalKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl GlobalKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThan = Rc<GreaterThanStruct>;

pub struct GreaterThanStruct {
    pub(crate) ir_node: ir::GreaterThan,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_greater_than(
    ir_node: &ir::GreaterThan,
    semantic: &Rc<SemanticContext>,
) -> GreaterThan {
    Rc::new(GreaterThanStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl GreaterThanStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanEqual = Rc<GreaterThanEqualStruct>;

pub struct GreaterThanEqualStruct {
    pub(crate) ir_node: ir::GreaterThanEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_greater_than_equal(
    ir_node: &ir::GreaterThanEqual,
    semantic: &Rc<SemanticContext>,
) -> GreaterThanEqual {
    Rc::new(GreaterThanEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl GreaterThanEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanGreaterThan = Rc<GreaterThanGreaterThanStruct>;

pub struct GreaterThanGreaterThanStruct {
    pub(crate) ir_node: ir::GreaterThanGreaterThan,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_greater_than_greater_than(
    ir_node: &ir::GreaterThanGreaterThan,
    semantic: &Rc<SemanticContext>,
) -> GreaterThanGreaterThan {
    Rc::new(GreaterThanGreaterThanStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl GreaterThanGreaterThanStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanGreaterThanEqual = Rc<GreaterThanGreaterThanEqualStruct>;

pub struct GreaterThanGreaterThanEqualStruct {
    pub(crate) ir_node: ir::GreaterThanGreaterThanEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_greater_than_greater_than_equal(
    ir_node: &ir::GreaterThanGreaterThanEqual,
    semantic: &Rc<SemanticContext>,
) -> GreaterThanGreaterThanEqual {
    Rc::new(GreaterThanGreaterThanEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl GreaterThanGreaterThanEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanGreaterThanGreaterThan = Rc<GreaterThanGreaterThanGreaterThanStruct>;

pub struct GreaterThanGreaterThanGreaterThanStruct {
    pub(crate) ir_node: ir::GreaterThanGreaterThanGreaterThan,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_greater_than_greater_than_greater_than(
    ir_node: &ir::GreaterThanGreaterThanGreaterThan,
    semantic: &Rc<SemanticContext>,
) -> GreaterThanGreaterThanGreaterThan {
    Rc::new(GreaterThanGreaterThanGreaterThanStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl GreaterThanGreaterThanGreaterThanStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanGreaterThanGreaterThanEqual = Rc<GreaterThanGreaterThanGreaterThanEqualStruct>;

pub struct GreaterThanGreaterThanGreaterThanEqualStruct {
    pub(crate) ir_node: ir::GreaterThanGreaterThanGreaterThanEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_greater_than_greater_than_greater_than_equal(
    ir_node: &ir::GreaterThanGreaterThanGreaterThanEqual,
    semantic: &Rc<SemanticContext>,
) -> GreaterThanGreaterThanGreaterThanEqual {
    Rc::new(GreaterThanGreaterThanGreaterThanEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl GreaterThanGreaterThanGreaterThanEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type GweiKeyword = Rc<GweiKeywordStruct>;

pub struct GweiKeywordStruct {
    pub(crate) ir_node: ir::GweiKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_gwei_keyword(
    ir_node: &ir::GweiKeyword,
    semantic: &Rc<SemanticContext>,
) -> GweiKeyword {
    Rc::new(GweiKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl GweiKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type HexLiteral = Rc<HexLiteralStruct>;

pub struct HexLiteralStruct {
    pub(crate) ir_node: ir::HexLiteral,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_hex_literal(
    ir_node: &ir::HexLiteral,
    semantic: &Rc<SemanticContext>,
) -> HexLiteral {
    Rc::new(HexLiteralStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl HexLiteralStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }
    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type HexStringLiteral = Rc<HexStringLiteralStruct>;

pub struct HexStringLiteralStruct {
    pub(crate) ir_node: ir::HexStringLiteral,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_hex_string_literal(
    ir_node: &ir::HexStringLiteral,
    semantic: &Rc<SemanticContext>,
) -> HexStringLiteral {
    Rc::new(HexStringLiteralStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl HexStringLiteralStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }
    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type HoursKeyword = Rc<HoursKeywordStruct>;

pub struct HoursKeywordStruct {
    pub(crate) ir_node: ir::HoursKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_hours_keyword(
    ir_node: &ir::HoursKeyword,
    semantic: &Rc<SemanticContext>,
) -> HoursKeyword {
    Rc::new(HoursKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl HoursKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Identifier = Rc<IdentifierStruct>;

pub struct IdentifierStruct {
    pub(crate) ir_node: ir::Identifier,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_identifier(
    ir_node: &ir::Identifier,
    semantic: &Rc<SemanticContext>,
) -> Identifier {
    Rc::new(IdentifierStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl IdentifierStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }
    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type IndexedKeyword = Rc<IndexedKeywordStruct>;

pub struct IndexedKeywordStruct {
    pub(crate) ir_node: ir::IndexedKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_indexed_keyword(
    ir_node: &ir::IndexedKeyword,
    semantic: &Rc<SemanticContext>,
) -> IndexedKeyword {
    Rc::new(IndexedKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl IndexedKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type IntKeyword = Rc<IntKeywordStruct>;

pub struct IntKeywordStruct {
    pub(crate) ir_node: ir::IntKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_int_keyword(
    ir_node: &ir::IntKeyword,
    semantic: &Rc<SemanticContext>,
) -> IntKeyword {
    Rc::new(IntKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl IntKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }
    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type LessThan = Rc<LessThanStruct>;

pub struct LessThanStruct {
    pub(crate) ir_node: ir::LessThan,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_less_than(ir_node: &ir::LessThan, semantic: &Rc<SemanticContext>) -> LessThan {
    Rc::new(LessThanStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl LessThanStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type LessThanEqual = Rc<LessThanEqualStruct>;

pub struct LessThanEqualStruct {
    pub(crate) ir_node: ir::LessThanEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_less_than_equal(
    ir_node: &ir::LessThanEqual,
    semantic: &Rc<SemanticContext>,
) -> LessThanEqual {
    Rc::new(LessThanEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl LessThanEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type LessThanLessThan = Rc<LessThanLessThanStruct>;

pub struct LessThanLessThanStruct {
    pub(crate) ir_node: ir::LessThanLessThan,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_less_than_less_than(
    ir_node: &ir::LessThanLessThan,
    semantic: &Rc<SemanticContext>,
) -> LessThanLessThan {
    Rc::new(LessThanLessThanStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl LessThanLessThanStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type LessThanLessThanEqual = Rc<LessThanLessThanEqualStruct>;

pub struct LessThanLessThanEqualStruct {
    pub(crate) ir_node: ir::LessThanLessThanEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_less_than_less_than_equal(
    ir_node: &ir::LessThanLessThanEqual,
    semantic: &Rc<SemanticContext>,
) -> LessThanLessThanEqual {
    Rc::new(LessThanLessThanEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl LessThanLessThanEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MemoryKeyword = Rc<MemoryKeywordStruct>;

pub struct MemoryKeywordStruct {
    pub(crate) ir_node: ir::MemoryKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_memory_keyword(
    ir_node: &ir::MemoryKeyword,
    semantic: &Rc<SemanticContext>,
) -> MemoryKeyword {
    Rc::new(MemoryKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl MemoryKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Minus = Rc<MinusStruct>;

pub struct MinusStruct {
    pub(crate) ir_node: ir::Minus,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_minus(ir_node: &ir::Minus, semantic: &Rc<SemanticContext>) -> Minus {
    Rc::new(MinusStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl MinusStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MinusEqual = Rc<MinusEqualStruct>;

pub struct MinusEqualStruct {
    pub(crate) ir_node: ir::MinusEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_minus_equal(
    ir_node: &ir::MinusEqual,
    semantic: &Rc<SemanticContext>,
) -> MinusEqual {
    Rc::new(MinusEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl MinusEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MinusMinus = Rc<MinusMinusStruct>;

pub struct MinusMinusStruct {
    pub(crate) ir_node: ir::MinusMinus,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_minus_minus(
    ir_node: &ir::MinusMinus,
    semantic: &Rc<SemanticContext>,
) -> MinusMinus {
    Rc::new(MinusMinusStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl MinusMinusStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type MinutesKeyword = Rc<MinutesKeywordStruct>;

pub struct MinutesKeywordStruct {
    pub(crate) ir_node: ir::MinutesKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_minutes_keyword(
    ir_node: &ir::MinutesKeyword,
    semantic: &Rc<SemanticContext>,
) -> MinutesKeyword {
    Rc::new(MinutesKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl MinutesKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PayableKeyword = Rc<PayableKeywordStruct>;

pub struct PayableKeywordStruct {
    pub(crate) ir_node: ir::PayableKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_payable_keyword(
    ir_node: &ir::PayableKeyword,
    semantic: &Rc<SemanticContext>,
) -> PayableKeyword {
    Rc::new(PayableKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PayableKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Percent = Rc<PercentStruct>;

pub struct PercentStruct {
    pub(crate) ir_node: ir::Percent,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_percent(ir_node: &ir::Percent, semantic: &Rc<SemanticContext>) -> Percent {
    Rc::new(PercentStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PercentStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PercentEqual = Rc<PercentEqualStruct>;

pub struct PercentEqualStruct {
    pub(crate) ir_node: ir::PercentEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_percent_equal(
    ir_node: &ir::PercentEqual,
    semantic: &Rc<SemanticContext>,
) -> PercentEqual {
    Rc::new(PercentEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PercentEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Plus = Rc<PlusStruct>;

pub struct PlusStruct {
    pub(crate) ir_node: ir::Plus,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_plus(ir_node: &ir::Plus, semantic: &Rc<SemanticContext>) -> Plus {
    Rc::new(PlusStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PlusStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PlusEqual = Rc<PlusEqualStruct>;

pub struct PlusEqualStruct {
    pub(crate) ir_node: ir::PlusEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_plus_equal(
    ir_node: &ir::PlusEqual,
    semantic: &Rc<SemanticContext>,
) -> PlusEqual {
    Rc::new(PlusEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PlusEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PlusPlus = Rc<PlusPlusStruct>;

pub struct PlusPlusStruct {
    pub(crate) ir_node: ir::PlusPlus,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_plus_plus(ir_node: &ir::PlusPlus, semantic: &Rc<SemanticContext>) -> PlusPlus {
    Rc::new(PlusPlusStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PlusPlusStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaCaret = Rc<PragmaCaretStruct>;

pub struct PragmaCaretStruct {
    pub(crate) ir_node: ir::PragmaCaret,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_pragma_caret(
    ir_node: &ir::PragmaCaret,
    semantic: &Rc<SemanticContext>,
) -> PragmaCaret {
    Rc::new(PragmaCaretStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PragmaCaretStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaEqual = Rc<PragmaEqualStruct>;

pub struct PragmaEqualStruct {
    pub(crate) ir_node: ir::PragmaEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_pragma_equal(
    ir_node: &ir::PragmaEqual,
    semantic: &Rc<SemanticContext>,
) -> PragmaEqual {
    Rc::new(PragmaEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PragmaEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaGreaterThan = Rc<PragmaGreaterThanStruct>;

pub struct PragmaGreaterThanStruct {
    pub(crate) ir_node: ir::PragmaGreaterThan,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_pragma_greater_than(
    ir_node: &ir::PragmaGreaterThan,
    semantic: &Rc<SemanticContext>,
) -> PragmaGreaterThan {
    Rc::new(PragmaGreaterThanStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PragmaGreaterThanStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaGreaterThanEqual = Rc<PragmaGreaterThanEqualStruct>;

pub struct PragmaGreaterThanEqualStruct {
    pub(crate) ir_node: ir::PragmaGreaterThanEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_pragma_greater_than_equal(
    ir_node: &ir::PragmaGreaterThanEqual,
    semantic: &Rc<SemanticContext>,
) -> PragmaGreaterThanEqual {
    Rc::new(PragmaGreaterThanEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PragmaGreaterThanEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaLessThan = Rc<PragmaLessThanStruct>;

pub struct PragmaLessThanStruct {
    pub(crate) ir_node: ir::PragmaLessThan,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_pragma_less_than(
    ir_node: &ir::PragmaLessThan,
    semantic: &Rc<SemanticContext>,
) -> PragmaLessThan {
    Rc::new(PragmaLessThanStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PragmaLessThanStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaLessThanEqual = Rc<PragmaLessThanEqualStruct>;

pub struct PragmaLessThanEqualStruct {
    pub(crate) ir_node: ir::PragmaLessThanEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_pragma_less_than_equal(
    ir_node: &ir::PragmaLessThanEqual,
    semantic: &Rc<SemanticContext>,
) -> PragmaLessThanEqual {
    Rc::new(PragmaLessThanEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PragmaLessThanEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaTilde = Rc<PragmaTildeStruct>;

pub struct PragmaTildeStruct {
    pub(crate) ir_node: ir::PragmaTilde,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_pragma_tilde(
    ir_node: &ir::PragmaTilde,
    semantic: &Rc<SemanticContext>,
) -> PragmaTilde {
    Rc::new(PragmaTildeStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PragmaTildeStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type SMTCheckerKeyword = Rc<SMTCheckerKeywordStruct>;

pub struct SMTCheckerKeywordStruct {
    pub(crate) ir_node: ir::SMTCheckerKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_smt_checker_keyword(
    ir_node: &ir::SMTCheckerKeyword,
    semantic: &Rc<SemanticContext>,
) -> SMTCheckerKeyword {
    Rc::new(SMTCheckerKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl SMTCheckerKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type SecondsKeyword = Rc<SecondsKeywordStruct>;

pub struct SecondsKeywordStruct {
    pub(crate) ir_node: ir::SecondsKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_seconds_keyword(
    ir_node: &ir::SecondsKeyword,
    semantic: &Rc<SemanticContext>,
) -> SecondsKeyword {
    Rc::new(SecondsKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl SecondsKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Semicolon = Rc<SemicolonStruct>;

pub struct SemicolonStruct {
    pub(crate) ir_node: ir::Semicolon,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_semicolon(
    ir_node: &ir::Semicolon,
    semantic: &Rc<SemanticContext>,
) -> Semicolon {
    Rc::new(SemicolonStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl SemicolonStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Slash = Rc<SlashStruct>;

pub struct SlashStruct {
    pub(crate) ir_node: ir::Slash,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_slash(ir_node: &ir::Slash, semantic: &Rc<SemanticContext>) -> Slash {
    Rc::new(SlashStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl SlashStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type SlashEqual = Rc<SlashEqualStruct>;

pub struct SlashEqualStruct {
    pub(crate) ir_node: ir::SlashEqual,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_slash_equal(
    ir_node: &ir::SlashEqual,
    semantic: &Rc<SemanticContext>,
) -> SlashEqual {
    Rc::new(SlashEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl SlashEqualStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StorageKeyword = Rc<StorageKeywordStruct>;

pub struct StorageKeywordStruct {
    pub(crate) ir_node: ir::StorageKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_storage_keyword(
    ir_node: &ir::StorageKeyword,
    semantic: &Rc<SemanticContext>,
) -> StorageKeyword {
    Rc::new(StorageKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl StorageKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StringKeyword = Rc<StringKeywordStruct>;

pub struct StringKeywordStruct {
    pub(crate) ir_node: ir::StringKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_string_keyword(
    ir_node: &ir::StringKeyword,
    semantic: &Rc<SemanticContext>,
) -> StringKeyword {
    Rc::new(StringKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl StringKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type StringLiteral = Rc<StringLiteralStruct>;

pub struct StringLiteralStruct {
    pub(crate) ir_node: ir::StringLiteral,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_string_literal(
    ir_node: &ir::StringLiteral,
    semantic: &Rc<SemanticContext>,
) -> StringLiteral {
    Rc::new(StringLiteralStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl StringLiteralStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }
    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type SuperKeyword = Rc<SuperKeywordStruct>;

pub struct SuperKeywordStruct {
    pub(crate) ir_node: ir::SuperKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_super_keyword(
    ir_node: &ir::SuperKeyword,
    semantic: &Rc<SemanticContext>,
) -> SuperKeyword {
    Rc::new(SuperKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl SuperKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type ThisKeyword = Rc<ThisKeywordStruct>;

pub struct ThisKeywordStruct {
    pub(crate) ir_node: ir::ThisKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_this_keyword(
    ir_node: &ir::ThisKeyword,
    semantic: &Rc<SemanticContext>,
) -> ThisKeyword {
    Rc::new(ThisKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ThisKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type Tilde = Rc<TildeStruct>;

pub struct TildeStruct {
    pub(crate) ir_node: ir::Tilde,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_tilde(ir_node: &ir::Tilde, semantic: &Rc<SemanticContext>) -> Tilde {
    Rc::new(TildeStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl TildeStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type TrueKeyword = Rc<TrueKeywordStruct>;

pub struct TrueKeywordStruct {
    pub(crate) ir_node: ir::TrueKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_true_keyword(
    ir_node: &ir::TrueKeyword,
    semantic: &Rc<SemanticContext>,
) -> TrueKeyword {
    Rc::new(TrueKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl TrueKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UfixedKeyword = Rc<UfixedKeywordStruct>;

pub struct UfixedKeywordStruct {
    pub(crate) ir_node: ir::UfixedKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_ufixed_keyword(
    ir_node: &ir::UfixedKeyword,
    semantic: &Rc<SemanticContext>,
) -> UfixedKeyword {
    Rc::new(UfixedKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl UfixedKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }
    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UintKeyword = Rc<UintKeywordStruct>;

pub struct UintKeywordStruct {
    pub(crate) ir_node: ir::UintKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_uint_keyword(
    ir_node: &ir::UintKeyword,
    semantic: &Rc<SemanticContext>,
) -> UintKeyword {
    Rc::new(UintKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl UintKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }
    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type UnicodeStringLiteral = Rc<UnicodeStringLiteralStruct>;

pub struct UnicodeStringLiteralStruct {
    pub(crate) ir_node: ir::UnicodeStringLiteral,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_unicode_string_literal(
    ir_node: &ir::UnicodeStringLiteral,
    semantic: &Rc<SemanticContext>,
) -> UnicodeStringLiteral {
    Rc::new(UnicodeStringLiteralStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl UnicodeStringLiteralStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }
    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VersionSpecifier = Rc<VersionSpecifierStruct>;

pub struct VersionSpecifierStruct {
    pub(crate) ir_node: ir::VersionSpecifier,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_version_specifier(
    ir_node: &ir::VersionSpecifier,
    semantic: &Rc<SemanticContext>,
) -> VersionSpecifier {
    Rc::new(VersionSpecifierStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl VersionSpecifierStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }
    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type VirtualKeyword = Rc<VirtualKeywordStruct>;

pub struct VirtualKeywordStruct {
    pub(crate) ir_node: ir::VirtualKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_virtual_keyword(
    ir_node: &ir::VirtualKeyword,
    semantic: &Rc<SemanticContext>,
) -> VirtualKeyword {
    Rc::new(VirtualKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl VirtualKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type WeeksKeyword = Rc<WeeksKeywordStruct>;

pub struct WeeksKeywordStruct {
    pub(crate) ir_node: ir::WeeksKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_weeks_keyword(
    ir_node: &ir::WeeksKeyword,
    semantic: &Rc<SemanticContext>,
) -> WeeksKeyword {
    Rc::new(WeeksKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl WeeksKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}

pub type WeiKeyword = Rc<WeiKeywordStruct>;

pub struct WeiKeywordStruct {
    pub(crate) ir_node: ir::WeiKeyword,
    pub(crate) semantic: Rc<SemanticContext>,
}

pub(crate) fn create_wei_keyword(
    ir_node: &ir::WeiKeyword,
    semantic: &Rc<SemanticContext>,
) -> WeiKeyword {
    Rc::new(WeiKeywordStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl WeiKeywordStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type> {
        Type::try_create_for_node_id(self.ir_node.id(), &self.semantic)
    }

    pub fn get_file_id(&self) -> &str {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &Range<usize> {
        &self.ir_node.range
    }
}
