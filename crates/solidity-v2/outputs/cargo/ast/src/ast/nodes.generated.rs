// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(unused)]
#![allow(non_camel_case_types)]
use std::rc::Rc;

use slang_solidity_v2_semantic::compilation::unit::CompilationUnit;
use slang_solidity_v2_semantic::ir::{self, NodeId};

//
// Sequences
//

pub type AbicoderPragma = Rc<AbicoderPragmaStruct>;

pub struct AbicoderPragmaStruct {
    pub(crate) ir_node: ir::AbicoderPragma,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_abicoder_pragma(
    ir_node: &ir::AbicoderPragma,
    compilation: &Rc<CompilationUnit>,
) -> AbicoderPragma {
    Rc::new(AbicoderPragmaStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl AbicoderPragmaStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn version(&self) -> AbicoderVersion {
        create_abicoder_version(&self.ir_node.version, &self.compilation)
    }
}

pub type AdditiveExpression = Rc<AdditiveExpressionStruct>;

pub struct AdditiveExpressionStruct {
    pub(crate) ir_node: ir::AdditiveExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_additive_expression(
    ir_node: &ir::AdditiveExpression,
    compilation: &Rc<CompilationUnit>,
) -> AdditiveExpression {
    Rc::new(AdditiveExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl AdditiveExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.compilation)
    }

    pub fn expression_additive_expression_operator(
        &self,
    ) -> Expression_AdditiveExpression_Operator {
        create_expression_additive_expression_operator(
            &self.ir_node.expression_additive_expression_operator,
            &self.compilation,
        )
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.compilation)
    }
}

pub type AddressType = Rc<AddressTypeStruct>;

pub struct AddressTypeStruct {
    pub(crate) ir_node: ir::AddressType,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_address_type(
    ir_node: &ir::AddressType,
    compilation: &Rc<CompilationUnit>,
) -> AddressType {
    Rc::new(AddressTypeStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl AddressTypeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn payable_keyword(&self) -> bool {
        self.ir_node.payable_keyword
    }
}

pub type AndExpression = Rc<AndExpressionStruct>;

pub struct AndExpressionStruct {
    pub(crate) ir_node: ir::AndExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_and_expression(
    ir_node: &ir::AndExpression,
    compilation: &Rc<CompilationUnit>,
) -> AndExpression {
    Rc::new(AndExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl AndExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.compilation)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.compilation)
    }
}

pub type ArrayExpression = Rc<ArrayExpressionStruct>;

pub struct ArrayExpressionStruct {
    pub(crate) ir_node: ir::ArrayExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_array_expression(
    ir_node: &ir::ArrayExpression,
    compilation: &Rc<CompilationUnit>,
) -> ArrayExpression {
    Rc::new(ArrayExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ArrayExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn items(&self) -> ArrayValues {
        create_array_values(&self.ir_node.items, &self.compilation)
    }
}

pub type ArrayTypeName = Rc<ArrayTypeNameStruct>;

pub struct ArrayTypeNameStruct {
    pub(crate) ir_node: ir::ArrayTypeName,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_array_type_name(
    ir_node: &ir::ArrayTypeName,
    compilation: &Rc<CompilationUnit>,
) -> ArrayTypeName {
    Rc::new(ArrayTypeNameStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ArrayTypeNameStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> TypeName {
        create_type_name(&self.ir_node.operand, &self.compilation)
    }

    pub fn index(&self) -> Option<Expression> {
        self.ir_node
            .index
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.compilation))
    }
}

pub type AssemblyStatement = Rc<AssemblyStatementStruct>;

pub struct AssemblyStatementStruct {
    pub(crate) ir_node: ir::AssemblyStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_assembly_statement(
    ir_node: &ir::AssemblyStatement,
    compilation: &Rc<CompilationUnit>,
) -> AssemblyStatement {
    Rc::new(AssemblyStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl AssemblyStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn label(&self) -> Option<ir::StringLiteral> {
        self.ir_node.label.as_ref().map(Rc::clone)
    }

    pub fn flags(&self) -> Option<Vec<ir::StringLiteral>> {
        self.ir_node.flags.clone()
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.compilation)
    }
}

pub type AssignmentExpression = Rc<AssignmentExpressionStruct>;

pub struct AssignmentExpressionStruct {
    pub(crate) ir_node: ir::AssignmentExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_assignment_expression(
    ir_node: &ir::AssignmentExpression,
    compilation: &Rc<CompilationUnit>,
) -> AssignmentExpression {
    Rc::new(AssignmentExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl AssignmentExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.compilation)
    }

    pub fn expression_assignment_expression_operator(
        &self,
    ) -> Expression_AssignmentExpression_Operator {
        create_expression_assignment_expression_operator(
            &self.ir_node.expression_assignment_expression_operator,
            &self.compilation,
        )
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.compilation)
    }
}

pub type BitwiseAndExpression = Rc<BitwiseAndExpressionStruct>;

pub struct BitwiseAndExpressionStruct {
    pub(crate) ir_node: ir::BitwiseAndExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_bitwise_and_expression(
    ir_node: &ir::BitwiseAndExpression,
    compilation: &Rc<CompilationUnit>,
) -> BitwiseAndExpression {
    Rc::new(BitwiseAndExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl BitwiseAndExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.compilation)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.compilation)
    }
}

pub type BitwiseOrExpression = Rc<BitwiseOrExpressionStruct>;

pub struct BitwiseOrExpressionStruct {
    pub(crate) ir_node: ir::BitwiseOrExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_bitwise_or_expression(
    ir_node: &ir::BitwiseOrExpression,
    compilation: &Rc<CompilationUnit>,
) -> BitwiseOrExpression {
    Rc::new(BitwiseOrExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl BitwiseOrExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.compilation)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.compilation)
    }
}

pub type BitwiseXorExpression = Rc<BitwiseXorExpressionStruct>;

pub struct BitwiseXorExpressionStruct {
    pub(crate) ir_node: ir::BitwiseXorExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_bitwise_xor_expression(
    ir_node: &ir::BitwiseXorExpression,
    compilation: &Rc<CompilationUnit>,
) -> BitwiseXorExpression {
    Rc::new(BitwiseXorExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl BitwiseXorExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.compilation)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.compilation)
    }
}

pub type Block = Rc<BlockStruct>;

pub struct BlockStruct {
    pub(crate) ir_node: ir::Block,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_block(ir_node: &ir::Block, compilation: &Rc<CompilationUnit>) -> Block {
    Rc::new(BlockStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl BlockStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn statements(&self) -> Statements {
        create_statements(&self.ir_node.statements, &self.compilation)
    }
}

pub type BreakStatement = Rc<BreakStatementStruct>;

pub struct BreakStatementStruct {
    pub(crate) ir_node: ir::BreakStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_break_statement(
    ir_node: &ir::BreakStatement,
    compilation: &Rc<CompilationUnit>,
) -> BreakStatement {
    Rc::new(BreakStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl BreakStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }
}

pub type CallOptionsExpression = Rc<CallOptionsExpressionStruct>;

pub struct CallOptionsExpressionStruct {
    pub(crate) ir_node: ir::CallOptionsExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_call_options_expression(
    ir_node: &ir::CallOptionsExpression,
    compilation: &Rc<CompilationUnit>,
) -> CallOptionsExpression {
    Rc::new(CallOptionsExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl CallOptionsExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.compilation)
    }

    pub fn options(&self) -> CallOptions {
        create_call_options(&self.ir_node.options, &self.compilation)
    }
}

pub type CatchClause = Rc<CatchClauseStruct>;

pub struct CatchClauseStruct {
    pub(crate) ir_node: ir::CatchClause,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_catch_clause(
    ir_node: &ir::CatchClause,
    compilation: &Rc<CompilationUnit>,
) -> CatchClause {
    Rc::new(CatchClauseStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
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
            .map(|ir_node| create_catch_clause_error(ir_node, &self.compilation))
    }

    pub fn body(&self) -> Block {
        create_block(&self.ir_node.body, &self.compilation)
    }
}

pub type CatchClauseError = Rc<CatchClauseErrorStruct>;

pub struct CatchClauseErrorStruct {
    pub(crate) ir_node: ir::CatchClauseError,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_catch_clause_error(
    ir_node: &ir::CatchClauseError,
    compilation: &Rc<CompilationUnit>,
) -> CatchClauseError {
    Rc::new(CatchClauseErrorStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
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
            .map(|ir_node| create_identifier(ir_node, &self.compilation))
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.compilation)
    }
}

pub type ConditionalExpression = Rc<ConditionalExpressionStruct>;

pub struct ConditionalExpressionStruct {
    pub(crate) ir_node: ir::ConditionalExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_conditional_expression(
    ir_node: &ir::ConditionalExpression,
    compilation: &Rc<CompilationUnit>,
) -> ConditionalExpression {
    Rc::new(ConditionalExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ConditionalExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.compilation)
    }

    pub fn true_expression(&self) -> Expression {
        create_expression(&self.ir_node.true_expression, &self.compilation)
    }

    pub fn false_expression(&self) -> Expression {
        create_expression(&self.ir_node.false_expression, &self.compilation)
    }
}

pub type ConstantDefinition = Rc<ConstantDefinitionStruct>;

pub struct ConstantDefinitionStruct {
    pub(crate) ir_node: ir::ConstantDefinition,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_constant_definition(
    ir_node: &ir::ConstantDefinition,
    compilation: &Rc<CompilationUnit>,
) -> ConstantDefinition {
    Rc::new(ConstantDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ConstantDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.compilation)
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn visibility(&self) -> Option<StateVariableVisibility> {
        self.ir_node
            .visibility
            .as_ref()
            .map(|ir_node| create_state_variable_visibility(ir_node, &self.compilation))
    }

    pub fn value(&self) -> Option<Expression> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.compilation))
    }
}

pub type ContinueStatement = Rc<ContinueStatementStruct>;

pub struct ContinueStatementStruct {
    pub(crate) ir_node: ir::ContinueStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_continue_statement(
    ir_node: &ir::ContinueStatement,
    compilation: &Rc<CompilationUnit>,
) -> ContinueStatement {
    Rc::new(ContinueStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ContinueStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }
}

pub type ContractDefinition = Rc<ContractDefinitionStruct>;

pub struct ContractDefinitionStruct {
    pub(crate) ir_node: ir::ContractDefinition,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_contract_definition(
    ir_node: &ir::ContractDefinition,
    compilation: &Rc<CompilationUnit>,
) -> ContractDefinition {
    Rc::new(ContractDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ContractDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn abstract_keyword(&self) -> bool {
        self.ir_node.abstract_keyword
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn members(&self) -> ContractMembers {
        create_contract_members(&self.ir_node.members, &self.compilation)
    }

    pub fn inheritance_types(&self) -> InheritanceTypes {
        create_inheritance_types(&self.ir_node.inheritance_types, &self.compilation)
    }

    pub fn storage_layout(&self) -> Option<Expression> {
        self.ir_node
            .storage_layout
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.compilation))
    }
}

pub type DecimalNumberExpression = Rc<DecimalNumberExpressionStruct>;

pub struct DecimalNumberExpressionStruct {
    pub(crate) ir_node: ir::DecimalNumberExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_decimal_number_expression(
    ir_node: &ir::DecimalNumberExpression,
    compilation: &Rc<CompilationUnit>,
) -> DecimalNumberExpression {
    Rc::new(DecimalNumberExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl DecimalNumberExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn literal(&self) -> ir::DecimalLiteral {
        Rc::clone(&self.ir_node.literal)
    }

    pub fn unit(&self) -> Option<NumberUnit> {
        self.ir_node
            .unit
            .as_ref()
            .map(|ir_node| create_number_unit(ir_node, &self.compilation))
    }
}

pub type DoWhileStatement = Rc<DoWhileStatementStruct>;

pub struct DoWhileStatementStruct {
    pub(crate) ir_node: ir::DoWhileStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_do_while_statement(
    ir_node: &ir::DoWhileStatement,
    compilation: &Rc<CompilationUnit>,
) -> DoWhileStatement {
    Rc::new(DoWhileStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl DoWhileStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn body(&self) -> Statement {
        create_statement(&self.ir_node.body, &self.compilation)
    }

    pub fn condition(&self) -> Expression {
        create_expression(&self.ir_node.condition, &self.compilation)
    }
}

pub type EmitStatement = Rc<EmitStatementStruct>;

pub struct EmitStatementStruct {
    pub(crate) ir_node: ir::EmitStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_emit_statement(
    ir_node: &ir::EmitStatement,
    compilation: &Rc<CompilationUnit>,
) -> EmitStatement {
    Rc::new(EmitStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl EmitStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn event(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.event, &self.compilation)
    }

    pub fn arguments(&self) -> ArgumentsDeclaration {
        create_arguments_declaration(&self.ir_node.arguments, &self.compilation)
    }
}

pub type EnumDefinition = Rc<EnumDefinitionStruct>;

pub struct EnumDefinitionStruct {
    pub(crate) ir_node: ir::EnumDefinition,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_enum_definition(
    ir_node: &ir::EnumDefinition,
    compilation: &Rc<CompilationUnit>,
) -> EnumDefinition {
    Rc::new(EnumDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl EnumDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn members(&self) -> EnumMembers {
        create_enum_members(&self.ir_node.members, &self.compilation)
    }
}

pub type EqualityExpression = Rc<EqualityExpressionStruct>;

pub struct EqualityExpressionStruct {
    pub(crate) ir_node: ir::EqualityExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_equality_expression(
    ir_node: &ir::EqualityExpression,
    compilation: &Rc<CompilationUnit>,
) -> EqualityExpression {
    Rc::new(EqualityExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl EqualityExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.compilation)
    }

    pub fn expression_equality_expression_operator(
        &self,
    ) -> Expression_EqualityExpression_Operator {
        create_expression_equality_expression_operator(
            &self.ir_node.expression_equality_expression_operator,
            &self.compilation,
        )
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.compilation)
    }
}

pub type ErrorDefinition = Rc<ErrorDefinitionStruct>;

pub struct ErrorDefinitionStruct {
    pub(crate) ir_node: ir::ErrorDefinition,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_error_definition(
    ir_node: &ir::ErrorDefinition,
    compilation: &Rc<CompilationUnit>,
) -> ErrorDefinition {
    Rc::new(ErrorDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ErrorDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.compilation)
    }
}

pub type EventDefinition = Rc<EventDefinitionStruct>;

pub struct EventDefinitionStruct {
    pub(crate) ir_node: ir::EventDefinition,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_event_definition(
    ir_node: &ir::EventDefinition,
    compilation: &Rc<CompilationUnit>,
) -> EventDefinition {
    Rc::new(EventDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl EventDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn anonymous_keyword(&self) -> bool {
        self.ir_node.anonymous_keyword
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.compilation)
    }
}

pub type ExperimentalPragma = Rc<ExperimentalPragmaStruct>;

pub struct ExperimentalPragmaStruct {
    pub(crate) ir_node: ir::ExperimentalPragma,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_experimental_pragma(
    ir_node: &ir::ExperimentalPragma,
    compilation: &Rc<CompilationUnit>,
) -> ExperimentalPragma {
    Rc::new(ExperimentalPragmaStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ExperimentalPragmaStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn feature(&self) -> ExperimentalFeature {
        create_experimental_feature(&self.ir_node.feature, &self.compilation)
    }
}

pub type ExponentiationExpression = Rc<ExponentiationExpressionStruct>;

pub struct ExponentiationExpressionStruct {
    pub(crate) ir_node: ir::ExponentiationExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_exponentiation_expression(
    ir_node: &ir::ExponentiationExpression,
    compilation: &Rc<CompilationUnit>,
) -> ExponentiationExpression {
    Rc::new(ExponentiationExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ExponentiationExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.compilation)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.compilation)
    }
}

pub type ExpressionStatement = Rc<ExpressionStatementStruct>;

pub struct ExpressionStatementStruct {
    pub(crate) ir_node: ir::ExpressionStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_expression_statement(
    ir_node: &ir::ExpressionStatement,
    compilation: &Rc<CompilationUnit>,
) -> ExpressionStatement {
    Rc::new(ExpressionStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ExpressionStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> Expression {
        create_expression(&self.ir_node.expression, &self.compilation)
    }
}

pub type ForStatement = Rc<ForStatementStruct>;

pub struct ForStatementStruct {
    pub(crate) ir_node: ir::ForStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_for_statement(
    ir_node: &ir::ForStatement,
    compilation: &Rc<CompilationUnit>,
) -> ForStatement {
    Rc::new(ForStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ForStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn initialization(&self) -> ForStatementInitialization {
        create_for_statement_initialization(&self.ir_node.initialization, &self.compilation)
    }

    pub fn condition(&self) -> ForStatementCondition {
        create_for_statement_condition(&self.ir_node.condition, &self.compilation)
    }

    pub fn iterator(&self) -> Option<Expression> {
        self.ir_node
            .iterator
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.compilation))
    }

    pub fn body(&self) -> Statement {
        create_statement(&self.ir_node.body, &self.compilation)
    }
}

pub type FunctionCallExpression = Rc<FunctionCallExpressionStruct>;

pub struct FunctionCallExpressionStruct {
    pub(crate) ir_node: ir::FunctionCallExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_function_call_expression(
    ir_node: &ir::FunctionCallExpression,
    compilation: &Rc<CompilationUnit>,
) -> FunctionCallExpression {
    Rc::new(FunctionCallExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl FunctionCallExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.compilation)
    }

    pub fn arguments(&self) -> ArgumentsDeclaration {
        create_arguments_declaration(&self.ir_node.arguments, &self.compilation)
    }
}

pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

pub struct FunctionDefinitionStruct {
    pub(crate) ir_node: ir::FunctionDefinition,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_function_definition(
    ir_node: &ir::FunctionDefinition,
    compilation: &Rc<CompilationUnit>,
) -> FunctionDefinition {
    Rc::new(FunctionDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl FunctionDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.compilation)
    }

    pub fn returns(&self) -> Option<Parameters> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, &self.compilation))
    }

    pub fn kind(&self) -> FunctionKind {
        create_function_kind(&self.ir_node.kind, &self.compilation)
    }

    pub fn name(&self) -> Option<Identifier> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.compilation))
    }

    pub fn body(&self) -> Option<Block> {
        self.ir_node
            .body
            .as_ref()
            .map(|ir_node| create_block(ir_node, &self.compilation))
    }

    pub fn visibility(&self) -> FunctionVisibility {
        create_function_visibility(&self.ir_node.visibility, &self.compilation)
    }

    pub fn mutability(&self) -> FunctionMutability {
        create_function_mutability(&self.ir_node.mutability, &self.compilation)
    }

    pub fn virtual_keyword(&self) -> bool {
        self.ir_node.virtual_keyword
    }

    pub fn override_specifier(&self) -> Option<OverridePaths> {
        self.ir_node
            .override_specifier
            .as_ref()
            .map(|ir_node| create_override_paths(ir_node, &self.compilation))
    }

    pub fn modifier_invocations(&self) -> ModifierInvocations {
        create_modifier_invocations(&self.ir_node.modifier_invocations, &self.compilation)
    }
}

pub type FunctionType = Rc<FunctionTypeStruct>;

pub struct FunctionTypeStruct {
    pub(crate) ir_node: ir::FunctionType,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_function_type(
    ir_node: &ir::FunctionType,
    compilation: &Rc<CompilationUnit>,
) -> FunctionType {
    Rc::new(FunctionTypeStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl FunctionTypeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.compilation)
    }

    pub fn returns(&self) -> Option<Parameters> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, &self.compilation))
    }

    pub fn visibility(&self) -> FunctionVisibility {
        create_function_visibility(&self.ir_node.visibility, &self.compilation)
    }

    pub fn mutability(&self) -> FunctionMutability {
        create_function_mutability(&self.ir_node.mutability, &self.compilation)
    }
}

pub type HexNumberExpression = Rc<HexNumberExpressionStruct>;

pub struct HexNumberExpressionStruct {
    pub(crate) ir_node: ir::HexNumberExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_hex_number_expression(
    ir_node: &ir::HexNumberExpression,
    compilation: &Rc<CompilationUnit>,
) -> HexNumberExpression {
    Rc::new(HexNumberExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl HexNumberExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn literal(&self) -> ir::HexLiteral {
        Rc::clone(&self.ir_node.literal)
    }
}

pub type IfStatement = Rc<IfStatementStruct>;

pub struct IfStatementStruct {
    pub(crate) ir_node: ir::IfStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_if_statement(
    ir_node: &ir::IfStatement,
    compilation: &Rc<CompilationUnit>,
) -> IfStatement {
    Rc::new(IfStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl IfStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn condition(&self) -> Expression {
        create_expression(&self.ir_node.condition, &self.compilation)
    }

    pub fn body(&self) -> Statement {
        create_statement(&self.ir_node.body, &self.compilation)
    }

    pub fn else_branch(&self) -> Option<Statement> {
        self.ir_node
            .else_branch
            .as_ref()
            .map(|ir_node| create_statement(ir_node, &self.compilation))
    }
}

pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

pub struct ImportDeconstructionStruct {
    pub(crate) ir_node: ir::ImportDeconstruction,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_import_deconstruction(
    ir_node: &ir::ImportDeconstruction,
    compilation: &Rc<CompilationUnit>,
) -> ImportDeconstruction {
    Rc::new(ImportDeconstructionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ImportDeconstructionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn symbols(&self) -> ImportDeconstructionSymbols {
        create_import_deconstruction_symbols(&self.ir_node.symbols, &self.compilation)
    }

    pub fn path(&self) -> ir::StringLiteral {
        Rc::clone(&self.ir_node.path)
    }
}

pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

pub struct ImportDeconstructionSymbolStruct {
    pub(crate) ir_node: ir::ImportDeconstructionSymbol,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_import_deconstruction_symbol(
    ir_node: &ir::ImportDeconstructionSymbol,
    compilation: &Rc<CompilationUnit>,
) -> ImportDeconstructionSymbol {
    Rc::new(ImportDeconstructionSymbolStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ImportDeconstructionSymbolStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn alias(&self) -> Option<Identifier> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.compilation))
    }
}

pub type IndexAccessExpression = Rc<IndexAccessExpressionStruct>;

pub struct IndexAccessExpressionStruct {
    pub(crate) ir_node: ir::IndexAccessExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_index_access_expression(
    ir_node: &ir::IndexAccessExpression,
    compilation: &Rc<CompilationUnit>,
) -> IndexAccessExpression {
    Rc::new(IndexAccessExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl IndexAccessExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.compilation)
    }

    pub fn start(&self) -> Option<Expression> {
        self.ir_node
            .start
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.compilation))
    }

    pub fn end(&self) -> Option<Expression> {
        self.ir_node
            .end
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.compilation))
    }
}

pub type InequalityExpression = Rc<InequalityExpressionStruct>;

pub struct InequalityExpressionStruct {
    pub(crate) ir_node: ir::InequalityExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_inequality_expression(
    ir_node: &ir::InequalityExpression,
    compilation: &Rc<CompilationUnit>,
) -> InequalityExpression {
    Rc::new(InequalityExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl InequalityExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.compilation)
    }

    pub fn expression_inequality_expression_operator(
        &self,
    ) -> Expression_InequalityExpression_Operator {
        create_expression_inequality_expression_operator(
            &self.ir_node.expression_inequality_expression_operator,
            &self.compilation,
        )
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.compilation)
    }
}

pub type InheritanceType = Rc<InheritanceTypeStruct>;

pub struct InheritanceTypeStruct {
    pub(crate) ir_node: ir::InheritanceType,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_inheritance_type(
    ir_node: &ir::InheritanceType,
    compilation: &Rc<CompilationUnit>,
) -> InheritanceType {
    Rc::new(InheritanceTypeStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl InheritanceTypeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.type_name, &self.compilation)
    }

    pub fn arguments(&self) -> Option<ArgumentsDeclaration> {
        self.ir_node
            .arguments
            .as_ref()
            .map(|ir_node| create_arguments_declaration(ir_node, &self.compilation))
    }
}

pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

pub struct InterfaceDefinitionStruct {
    pub(crate) ir_node: ir::InterfaceDefinition,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_interface_definition(
    ir_node: &ir::InterfaceDefinition,
    compilation: &Rc<CompilationUnit>,
) -> InterfaceDefinition {
    Rc::new(InterfaceDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl InterfaceDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn inheritance(&self) -> Option<InheritanceTypes> {
        self.ir_node
            .inheritance
            .as_ref()
            .map(|ir_node| create_inheritance_types(ir_node, &self.compilation))
    }

    pub fn members(&self) -> InterfaceMembers {
        create_interface_members(&self.ir_node.members, &self.compilation)
    }
}

pub type LibraryDefinition = Rc<LibraryDefinitionStruct>;

pub struct LibraryDefinitionStruct {
    pub(crate) ir_node: ir::LibraryDefinition,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_library_definition(
    ir_node: &ir::LibraryDefinition,
    compilation: &Rc<CompilationUnit>,
) -> LibraryDefinition {
    Rc::new(LibraryDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl LibraryDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn members(&self) -> LibraryMembers {
        create_library_members(&self.ir_node.members, &self.compilation)
    }
}

pub type MappingType = Rc<MappingTypeStruct>;

pub struct MappingTypeStruct {
    pub(crate) ir_node: ir::MappingType,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_mapping_type(
    ir_node: &ir::MappingType,
    compilation: &Rc<CompilationUnit>,
) -> MappingType {
    Rc::new(MappingTypeStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl MappingTypeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn key_type(&self) -> Parameter {
        create_parameter(&self.ir_node.key_type, &self.compilation)
    }

    pub fn value_type(&self) -> Parameter {
        create_parameter(&self.ir_node.value_type, &self.compilation)
    }
}

pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

pub struct MemberAccessExpressionStruct {
    pub(crate) ir_node: ir::MemberAccessExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_member_access_expression(
    ir_node: &ir::MemberAccessExpression,
    compilation: &Rc<CompilationUnit>,
) -> MemberAccessExpression {
    Rc::new(MemberAccessExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl MemberAccessExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.compilation)
    }

    pub fn member(&self) -> Identifier {
        create_identifier(&self.ir_node.member, &self.compilation)
    }
}

pub type ModifierInvocation = Rc<ModifierInvocationStruct>;

pub struct ModifierInvocationStruct {
    pub(crate) ir_node: ir::ModifierInvocation,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_modifier_invocation(
    ir_node: &ir::ModifierInvocation,
    compilation: &Rc<CompilationUnit>,
) -> ModifierInvocation {
    Rc::new(ModifierInvocationStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ModifierInvocationStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.name, &self.compilation)
    }

    pub fn arguments(&self) -> Option<ArgumentsDeclaration> {
        self.ir_node
            .arguments
            .as_ref()
            .map(|ir_node| create_arguments_declaration(ir_node, &self.compilation))
    }
}

pub type MultiTypedDeclaration = Rc<MultiTypedDeclarationStruct>;

pub struct MultiTypedDeclarationStruct {
    pub(crate) ir_node: ir::MultiTypedDeclaration,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_multi_typed_declaration(
    ir_node: &ir::MultiTypedDeclaration,
    compilation: &Rc<CompilationUnit>,
) -> MultiTypedDeclaration {
    Rc::new(MultiTypedDeclarationStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl MultiTypedDeclarationStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn elements(&self) -> MultiTypedDeclarationElements {
        create_multi_typed_declaration_elements(&self.ir_node.elements, &self.compilation)
    }

    pub fn value(&self) -> Expression {
        create_expression(&self.ir_node.value, &self.compilation)
    }
}

pub type MultiTypedDeclarationElement = Rc<MultiTypedDeclarationElementStruct>;

pub struct MultiTypedDeclarationElementStruct {
    pub(crate) ir_node: ir::MultiTypedDeclarationElement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_multi_typed_declaration_element(
    ir_node: &ir::MultiTypedDeclarationElement,
    compilation: &Rc<CompilationUnit>,
) -> MultiTypedDeclarationElement {
    Rc::new(MultiTypedDeclarationElementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
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
            .map(|ir_node| create_variable_declaration(ir_node, &self.compilation))
    }
}

pub type MultiplicativeExpression = Rc<MultiplicativeExpressionStruct>;

pub struct MultiplicativeExpressionStruct {
    pub(crate) ir_node: ir::MultiplicativeExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_multiplicative_expression(
    ir_node: &ir::MultiplicativeExpression,
    compilation: &Rc<CompilationUnit>,
) -> MultiplicativeExpression {
    Rc::new(MultiplicativeExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl MultiplicativeExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.compilation)
    }

    pub fn expression_multiplicative_expression_operator(
        &self,
    ) -> Expression_MultiplicativeExpression_Operator {
        create_expression_multiplicative_expression_operator(
            &self.ir_node.expression_multiplicative_expression_operator,
            &self.compilation,
        )
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.compilation)
    }
}

pub type NamedArgument = Rc<NamedArgumentStruct>;

pub struct NamedArgumentStruct {
    pub(crate) ir_node: ir::NamedArgument,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_named_argument(
    ir_node: &ir::NamedArgument,
    compilation: &Rc<CompilationUnit>,
) -> NamedArgument {
    Rc::new(NamedArgumentStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl NamedArgumentStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn value(&self) -> Expression {
        create_expression(&self.ir_node.value, &self.compilation)
    }
}

pub type NewExpression = Rc<NewExpressionStruct>;

pub struct NewExpressionStruct {
    pub(crate) ir_node: ir::NewExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_new_expression(
    ir_node: &ir::NewExpression,
    compilation: &Rc<CompilationUnit>,
) -> NewExpression {
    Rc::new(NewExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl NewExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.compilation)
    }
}

pub type OrExpression = Rc<OrExpressionStruct>;

pub struct OrExpressionStruct {
    pub(crate) ir_node: ir::OrExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_or_expression(
    ir_node: &ir::OrExpression,
    compilation: &Rc<CompilationUnit>,
) -> OrExpression {
    Rc::new(OrExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl OrExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.compilation)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.compilation)
    }
}

pub type Parameter = Rc<ParameterStruct>;

pub struct ParameterStruct {
    pub(crate) ir_node: ir::Parameter,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_parameter(
    ir_node: &ir::Parameter,
    compilation: &Rc<CompilationUnit>,
) -> Parameter {
    Rc::new(ParameterStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ParameterStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.compilation)
    }

    pub fn storage_location(&self) -> Option<StorageLocation> {
        self.ir_node
            .storage_location
            .as_ref()
            .map(|ir_node| create_storage_location(ir_node, &self.compilation))
    }

    pub fn name(&self) -> Option<Identifier> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.compilation))
    }

    pub fn indexed(&self) -> bool {
        self.ir_node.indexed
    }
}

pub type PathImport = Rc<PathImportStruct>;

pub struct PathImportStruct {
    pub(crate) ir_node: ir::PathImport,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_path_import(
    ir_node: &ir::PathImport,
    compilation: &Rc<CompilationUnit>,
) -> PathImport {
    Rc::new(PathImportStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl PathImportStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn path(&self) -> ir::StringLiteral {
        Rc::clone(&self.ir_node.path)
    }

    pub fn alias(&self) -> Option<Identifier> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.compilation))
    }
}

pub type PostfixExpression = Rc<PostfixExpressionStruct>;

pub struct PostfixExpressionStruct {
    pub(crate) ir_node: ir::PostfixExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_postfix_expression(
    ir_node: &ir::PostfixExpression,
    compilation: &Rc<CompilationUnit>,
) -> PostfixExpression {
    Rc::new(PostfixExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl PostfixExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.compilation)
    }

    pub fn expression_postfix_expression_operator(&self) -> Expression_PostfixExpression_Operator {
        create_expression_postfix_expression_operator(
            &self.ir_node.expression_postfix_expression_operator,
            &self.compilation,
        )
    }
}

pub type PragmaDirective = Rc<PragmaDirectiveStruct>;

pub struct PragmaDirectiveStruct {
    pub(crate) ir_node: ir::PragmaDirective,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_pragma_directive(
    ir_node: &ir::PragmaDirective,
    compilation: &Rc<CompilationUnit>,
) -> PragmaDirective {
    Rc::new(PragmaDirectiveStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl PragmaDirectiveStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn pragma(&self) -> Pragma {
        create_pragma(&self.ir_node.pragma, &self.compilation)
    }
}

pub type PrefixExpression = Rc<PrefixExpressionStruct>;

pub struct PrefixExpressionStruct {
    pub(crate) ir_node: ir::PrefixExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_prefix_expression(
    ir_node: &ir::PrefixExpression,
    compilation: &Rc<CompilationUnit>,
) -> PrefixExpression {
    Rc::new(PrefixExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl PrefixExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression_prefix_expression_operator(&self) -> Expression_PrefixExpression_Operator {
        create_expression_prefix_expression_operator(
            &self.ir_node.expression_prefix_expression_operator,
            &self.compilation,
        )
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.compilation)
    }
}

pub type ReturnStatement = Rc<ReturnStatementStruct>;

pub struct ReturnStatementStruct {
    pub(crate) ir_node: ir::ReturnStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_return_statement(
    ir_node: &ir::ReturnStatement,
    compilation: &Rc<CompilationUnit>,
) -> ReturnStatement {
    Rc::new(ReturnStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
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
            .map(|ir_node| create_expression(ir_node, &self.compilation))
    }
}

pub type RevertStatement = Rc<RevertStatementStruct>;

pub struct RevertStatementStruct {
    pub(crate) ir_node: ir::RevertStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_revert_statement(
    ir_node: &ir::RevertStatement,
    compilation: &Rc<CompilationUnit>,
) -> RevertStatement {
    Rc::new(RevertStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl RevertStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn error(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.error, &self.compilation)
    }

    pub fn arguments(&self) -> ArgumentsDeclaration {
        create_arguments_declaration(&self.ir_node.arguments, &self.compilation)
    }
}

pub type ShiftExpression = Rc<ShiftExpressionStruct>;

pub struct ShiftExpressionStruct {
    pub(crate) ir_node: ir::ShiftExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_shift_expression(
    ir_node: &ir::ShiftExpression,
    compilation: &Rc<CompilationUnit>,
) -> ShiftExpression {
    Rc::new(ShiftExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl ShiftExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.compilation)
    }

    pub fn expression_shift_expression_operator(&self) -> Expression_ShiftExpression_Operator {
        create_expression_shift_expression_operator(
            &self.ir_node.expression_shift_expression_operator,
            &self.compilation,
        )
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.compilation)
    }
}

pub type SingleTypedDeclaration = Rc<SingleTypedDeclarationStruct>;

pub struct SingleTypedDeclarationStruct {
    pub(crate) ir_node: ir::SingleTypedDeclaration,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_single_typed_declaration(
    ir_node: &ir::SingleTypedDeclaration,
    compilation: &Rc<CompilationUnit>,
) -> SingleTypedDeclaration {
    Rc::new(SingleTypedDeclarationStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl SingleTypedDeclarationStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn declaration(&self) -> VariableDeclaration {
        create_variable_declaration(&self.ir_node.declaration, &self.compilation)
    }

    pub fn value(&self) -> Option<Expression> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.compilation))
    }
}

pub type SourceUnit = Rc<SourceUnitStruct>;

pub struct SourceUnitStruct {
    pub(crate) ir_node: ir::SourceUnit,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_source_unit(
    ir_node: &ir::SourceUnit,
    compilation: &Rc<CompilationUnit>,
) -> SourceUnit {
    Rc::new(SourceUnitStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl SourceUnitStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn members(&self) -> SourceUnitMembers {
        create_source_unit_members(&self.ir_node.members, &self.compilation)
    }
}

pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

pub struct StateVariableDefinitionStruct {
    pub(crate) ir_node: ir::StateVariableDefinition,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_state_variable_definition(
    ir_node: &ir::StateVariableDefinition,
    compilation: &Rc<CompilationUnit>,
) -> StateVariableDefinition {
    Rc::new(StateVariableDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl StateVariableDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.compilation)
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn value(&self) -> Option<Expression> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.compilation))
    }

    pub fn visibility(&self) -> StateVariableVisibility {
        create_state_variable_visibility(&self.ir_node.visibility, &self.compilation)
    }

    pub fn mutability(&self) -> StateVariableMutability {
        create_state_variable_mutability(&self.ir_node.mutability, &self.compilation)
    }

    pub fn override_specifier(&self) -> Option<OverridePaths> {
        self.ir_node
            .override_specifier
            .as_ref()
            .map(|ir_node| create_override_paths(ir_node, &self.compilation))
    }
}

pub type StructDefinition = Rc<StructDefinitionStruct>;

pub struct StructDefinitionStruct {
    pub(crate) ir_node: ir::StructDefinition,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_struct_definition(
    ir_node: &ir::StructDefinition,
    compilation: &Rc<CompilationUnit>,
) -> StructDefinition {
    Rc::new(StructDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl StructDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn members(&self) -> StructMembers {
        create_struct_members(&self.ir_node.members, &self.compilation)
    }
}

pub type StructMember = Rc<StructMemberStruct>;

pub struct StructMemberStruct {
    pub(crate) ir_node: ir::StructMember,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_struct_member(
    ir_node: &ir::StructMember,
    compilation: &Rc<CompilationUnit>,
) -> StructMember {
    Rc::new(StructMemberStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl StructMemberStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.compilation)
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }
}

pub type TryStatement = Rc<TryStatementStruct>;

pub struct TryStatementStruct {
    pub(crate) ir_node: ir::TryStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_try_statement(
    ir_node: &ir::TryStatement,
    compilation: &Rc<CompilationUnit>,
) -> TryStatement {
    Rc::new(TryStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl TryStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> Expression {
        create_expression(&self.ir_node.expression, &self.compilation)
    }

    pub fn returns(&self) -> Option<Parameters> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, &self.compilation))
    }

    pub fn body(&self) -> Block {
        create_block(&self.ir_node.body, &self.compilation)
    }

    pub fn catch_clauses(&self) -> CatchClauses {
        create_catch_clauses(&self.ir_node.catch_clauses, &self.compilation)
    }
}

pub type TupleExpression = Rc<TupleExpressionStruct>;

pub struct TupleExpressionStruct {
    pub(crate) ir_node: ir::TupleExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_tuple_expression(
    ir_node: &ir::TupleExpression,
    compilation: &Rc<CompilationUnit>,
) -> TupleExpression {
    Rc::new(TupleExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl TupleExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn items(&self) -> TupleValues {
        create_tuple_values(&self.ir_node.items, &self.compilation)
    }
}

pub type TupleValue = Rc<TupleValueStruct>;

pub struct TupleValueStruct {
    pub(crate) ir_node: ir::TupleValue,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_tuple_value(
    ir_node: &ir::TupleValue,
    compilation: &Rc<CompilationUnit>,
) -> TupleValue {
    Rc::new(TupleValueStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
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
            .map(|ir_node| create_expression(ir_node, &self.compilation))
    }
}

pub type TypeExpression = Rc<TypeExpressionStruct>;

pub struct TypeExpressionStruct {
    pub(crate) ir_node: ir::TypeExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_type_expression(
    ir_node: &ir::TypeExpression,
    compilation: &Rc<CompilationUnit>,
) -> TypeExpression {
    Rc::new(TypeExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl TypeExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.compilation)
    }
}

pub type UncheckedBlock = Rc<UncheckedBlockStruct>;

pub struct UncheckedBlockStruct {
    pub(crate) ir_node: ir::UncheckedBlock,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_unchecked_block(
    ir_node: &ir::UncheckedBlock,
    compilation: &Rc<CompilationUnit>,
) -> UncheckedBlock {
    Rc::new(UncheckedBlockStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl UncheckedBlockStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn block(&self) -> Block {
        create_block(&self.ir_node.block, &self.compilation)
    }
}

pub type UserDefinedValueTypeDefinition = Rc<UserDefinedValueTypeDefinitionStruct>;

pub struct UserDefinedValueTypeDefinitionStruct {
    pub(crate) ir_node: ir::UserDefinedValueTypeDefinition,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_user_defined_value_type_definition(
    ir_node: &ir::UserDefinedValueTypeDefinition,
    compilation: &Rc<CompilationUnit>,
) -> UserDefinedValueTypeDefinition {
    Rc::new(UserDefinedValueTypeDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl UserDefinedValueTypeDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn value_type(&self) -> ElementaryType {
        create_elementary_type(&self.ir_node.value_type, &self.compilation)
    }
}

pub type UsingDeconstruction = Rc<UsingDeconstructionStruct>;

pub struct UsingDeconstructionStruct {
    pub(crate) ir_node: ir::UsingDeconstruction,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_using_deconstruction(
    ir_node: &ir::UsingDeconstruction,
    compilation: &Rc<CompilationUnit>,
) -> UsingDeconstruction {
    Rc::new(UsingDeconstructionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl UsingDeconstructionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn symbols(&self) -> UsingDeconstructionSymbols {
        create_using_deconstruction_symbols(&self.ir_node.symbols, &self.compilation)
    }
}

pub type UsingDeconstructionSymbol = Rc<UsingDeconstructionSymbolStruct>;

pub struct UsingDeconstructionSymbolStruct {
    pub(crate) ir_node: ir::UsingDeconstructionSymbol,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_using_deconstruction_symbol(
    ir_node: &ir::UsingDeconstructionSymbol,
    compilation: &Rc<CompilationUnit>,
) -> UsingDeconstructionSymbol {
    Rc::new(UsingDeconstructionSymbolStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl UsingDeconstructionSymbolStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.name, &self.compilation)
    }

    pub fn alias(&self) -> Option<UsingOperator> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| create_using_operator(ir_node, &self.compilation))
    }
}

pub type UsingDirective = Rc<UsingDirectiveStruct>;

pub struct UsingDirectiveStruct {
    pub(crate) ir_node: ir::UsingDirective,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_using_directive(
    ir_node: &ir::UsingDirective,
    compilation: &Rc<CompilationUnit>,
) -> UsingDirective {
    Rc::new(UsingDirectiveStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl UsingDirectiveStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn clause(&self) -> UsingClause {
        create_using_clause(&self.ir_node.clause, &self.compilation)
    }

    pub fn target(&self) -> UsingTarget {
        create_using_target(&self.ir_node.target, &self.compilation)
    }

    pub fn global_keyword(&self) -> bool {
        self.ir_node.global_keyword
    }
}

pub type VariableDeclaration = Rc<VariableDeclarationStruct>;

pub struct VariableDeclarationStruct {
    pub(crate) ir_node: ir::VariableDeclaration,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_variable_declaration(
    ir_node: &ir::VariableDeclaration,
    compilation: &Rc<CompilationUnit>,
) -> VariableDeclaration {
    Rc::new(VariableDeclarationStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl VariableDeclarationStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.compilation)
    }

    pub fn storage_location(&self) -> Option<StorageLocation> {
        self.ir_node
            .storage_location
            .as_ref()
            .map(|ir_node| create_storage_location(ir_node, &self.compilation))
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }
}

pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

pub struct VariableDeclarationStatementStruct {
    pub(crate) ir_node: ir::VariableDeclarationStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_variable_declaration_statement(
    ir_node: &ir::VariableDeclarationStatement,
    compilation: &Rc<CompilationUnit>,
) -> VariableDeclarationStatement {
    Rc::new(VariableDeclarationStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl VariableDeclarationStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn target(&self) -> VariableDeclarationTarget {
        create_variable_declaration_target(&self.ir_node.target, &self.compilation)
    }
}

pub type VersionPragma = Rc<VersionPragmaStruct>;

pub struct VersionPragmaStruct {
    pub(crate) ir_node: ir::VersionPragma,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_version_pragma(
    ir_node: &ir::VersionPragma,
    compilation: &Rc<CompilationUnit>,
) -> VersionPragma {
    Rc::new(VersionPragmaStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl VersionPragmaStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn sets(&self) -> VersionExpressionSets {
        create_version_expression_sets(&self.ir_node.sets, &self.compilation)
    }
}

pub type VersionRange = Rc<VersionRangeStruct>;

pub struct VersionRangeStruct {
    pub(crate) ir_node: ir::VersionRange,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_version_range(
    ir_node: &ir::VersionRange,
    compilation: &Rc<CompilationUnit>,
) -> VersionRange {
    Rc::new(VersionRangeStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl VersionRangeStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn start(&self) -> VersionLiteral {
        create_version_literal(&self.ir_node.start, &self.compilation)
    }

    pub fn end(&self) -> VersionLiteral {
        create_version_literal(&self.ir_node.end, &self.compilation)
    }
}

pub type VersionTerm = Rc<VersionTermStruct>;

pub struct VersionTermStruct {
    pub(crate) ir_node: ir::VersionTerm,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_version_term(
    ir_node: &ir::VersionTerm,
    compilation: &Rc<CompilationUnit>,
) -> VersionTerm {
    Rc::new(VersionTermStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
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
            .map(|ir_node| create_version_operator(ir_node, &self.compilation))
    }

    pub fn literal(&self) -> VersionLiteral {
        create_version_literal(&self.ir_node.literal, &self.compilation)
    }
}

pub type WhileStatement = Rc<WhileStatementStruct>;

pub struct WhileStatementStruct {
    pub(crate) ir_node: ir::WhileStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_while_statement(
    ir_node: &ir::WhileStatement,
    compilation: &Rc<CompilationUnit>,
) -> WhileStatement {
    Rc::new(WhileStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl WhileStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn condition(&self) -> Expression {
        create_expression(&self.ir_node.condition, &self.compilation)
    }

    pub fn body(&self) -> Statement {
        create_statement(&self.ir_node.body, &self.compilation)
    }
}

pub type YulBlock = Rc<YulBlockStruct>;

pub struct YulBlockStruct {
    pub(crate) ir_node: ir::YulBlock,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_block(
    ir_node: &ir::YulBlock,
    compilation: &Rc<CompilationUnit>,
) -> YulBlock {
    Rc::new(YulBlockStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulBlockStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn statements(&self) -> YulStatements {
        create_yul_statements(&self.ir_node.statements, &self.compilation)
    }
}

pub type YulBreakStatement = Rc<YulBreakStatementStruct>;

pub struct YulBreakStatementStruct {
    pub(crate) ir_node: ir::YulBreakStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_break_statement(
    ir_node: &ir::YulBreakStatement,
    compilation: &Rc<CompilationUnit>,
) -> YulBreakStatement {
    Rc::new(YulBreakStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulBreakStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }
}

pub type YulContinueStatement = Rc<YulContinueStatementStruct>;

pub struct YulContinueStatementStruct {
    pub(crate) ir_node: ir::YulContinueStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_continue_statement(
    ir_node: &ir::YulContinueStatement,
    compilation: &Rc<CompilationUnit>,
) -> YulContinueStatement {
    Rc::new(YulContinueStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulContinueStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }
}

pub type YulDefaultCase = Rc<YulDefaultCaseStruct>;

pub struct YulDefaultCaseStruct {
    pub(crate) ir_node: ir::YulDefaultCase,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_default_case(
    ir_node: &ir::YulDefaultCase,
    compilation: &Rc<CompilationUnit>,
) -> YulDefaultCase {
    Rc::new(YulDefaultCaseStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulDefaultCaseStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.compilation)
    }
}

pub type YulForStatement = Rc<YulForStatementStruct>;

pub struct YulForStatementStruct {
    pub(crate) ir_node: ir::YulForStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_for_statement(
    ir_node: &ir::YulForStatement,
    compilation: &Rc<CompilationUnit>,
) -> YulForStatement {
    Rc::new(YulForStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulForStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn initialization(&self) -> YulBlock {
        create_yul_block(&self.ir_node.initialization, &self.compilation)
    }

    pub fn condition(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.condition, &self.compilation)
    }

    pub fn iterator(&self) -> YulBlock {
        create_yul_block(&self.ir_node.iterator, &self.compilation)
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.compilation)
    }
}

pub type YulFunctionCallExpression = Rc<YulFunctionCallExpressionStruct>;

pub struct YulFunctionCallExpressionStruct {
    pub(crate) ir_node: ir::YulFunctionCallExpression,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_function_call_expression(
    ir_node: &ir::YulFunctionCallExpression,
    compilation: &Rc<CompilationUnit>,
) -> YulFunctionCallExpression {
    Rc::new(YulFunctionCallExpressionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulFunctionCallExpressionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.operand, &self.compilation)
    }

    pub fn arguments(&self) -> YulArguments {
        create_yul_arguments(&self.ir_node.arguments, &self.compilation)
    }
}

pub type YulFunctionDefinition = Rc<YulFunctionDefinitionStruct>;

pub struct YulFunctionDefinitionStruct {
    pub(crate) ir_node: ir::YulFunctionDefinition,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_function_definition(
    ir_node: &ir::YulFunctionDefinition,
    compilation: &Rc<CompilationUnit>,
) -> YulFunctionDefinition {
    Rc::new(YulFunctionDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulFunctionDefinitionStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.compilation)
    }

    pub fn parameters(&self) -> YulParameters {
        create_yul_parameters(&self.ir_node.parameters, &self.compilation)
    }

    pub fn returns(&self) -> Option<YulVariableNames> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_yul_variable_names(ir_node, &self.compilation))
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.compilation)
    }
}

pub type YulIfStatement = Rc<YulIfStatementStruct>;

pub struct YulIfStatementStruct {
    pub(crate) ir_node: ir::YulIfStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_if_statement(
    ir_node: &ir::YulIfStatement,
    compilation: &Rc<CompilationUnit>,
) -> YulIfStatement {
    Rc::new(YulIfStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulIfStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn condition(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.condition, &self.compilation)
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.compilation)
    }
}

pub type YulLeaveStatement = Rc<YulLeaveStatementStruct>;

pub struct YulLeaveStatementStruct {
    pub(crate) ir_node: ir::YulLeaveStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_leave_statement(
    ir_node: &ir::YulLeaveStatement,
    compilation: &Rc<CompilationUnit>,
) -> YulLeaveStatement {
    Rc::new(YulLeaveStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulLeaveStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }
}

pub type YulSwitchStatement = Rc<YulSwitchStatementStruct>;

pub struct YulSwitchStatementStruct {
    pub(crate) ir_node: ir::YulSwitchStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_switch_statement(
    ir_node: &ir::YulSwitchStatement,
    compilation: &Rc<CompilationUnit>,
) -> YulSwitchStatement {
    Rc::new(YulSwitchStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulSwitchStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.expression, &self.compilation)
    }

    pub fn cases(&self) -> YulSwitchCases {
        create_yul_switch_cases(&self.ir_node.cases, &self.compilation)
    }
}

pub type YulValueCase = Rc<YulValueCaseStruct>;

pub struct YulValueCaseStruct {
    pub(crate) ir_node: ir::YulValueCase,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_value_case(
    ir_node: &ir::YulValueCase,
    compilation: &Rc<CompilationUnit>,
) -> YulValueCase {
    Rc::new(YulValueCaseStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulValueCaseStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn value(&self) -> YulLiteral {
        create_yul_literal(&self.ir_node.value, &self.compilation)
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.compilation)
    }
}

pub type YulVariableAssignmentStatement = Rc<YulVariableAssignmentStatementStruct>;

pub struct YulVariableAssignmentStatementStruct {
    pub(crate) ir_node: ir::YulVariableAssignmentStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_variable_assignment_statement(
    ir_node: &ir::YulVariableAssignmentStatement,
    compilation: &Rc<CompilationUnit>,
) -> YulVariableAssignmentStatement {
    Rc::new(YulVariableAssignmentStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulVariableAssignmentStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn variables(&self) -> YulPaths {
        create_yul_paths(&self.ir_node.variables, &self.compilation)
    }

    pub fn expression(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.expression, &self.compilation)
    }
}

pub type YulVariableDeclarationStatement = Rc<YulVariableDeclarationStatementStruct>;

pub struct YulVariableDeclarationStatementStruct {
    pub(crate) ir_node: ir::YulVariableDeclarationStatement,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_variable_declaration_statement(
    ir_node: &ir::YulVariableDeclarationStatement,
    compilation: &Rc<CompilationUnit>,
) -> YulVariableDeclarationStatement {
    Rc::new(YulVariableDeclarationStatementStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulVariableDeclarationStatementStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn variables(&self) -> YulVariableNames {
        create_yul_variable_names(&self.ir_node.variables, &self.compilation)
    }

    pub fn value(&self) -> Option<YulVariableDeclarationValue> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_yul_variable_declaration_value(ir_node, &self.compilation))
    }
}

pub type YulVariableDeclarationValue = Rc<YulVariableDeclarationValueStruct>;

pub struct YulVariableDeclarationValueStruct {
    pub(crate) ir_node: ir::YulVariableDeclarationValue,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_yul_variable_declaration_value(
    ir_node: &ir::YulVariableDeclarationValue,
    compilation: &Rc<CompilationUnit>,
) -> YulVariableDeclarationValue {
    Rc::new(YulVariableDeclarationValueStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl YulVariableDeclarationValueStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.expression, &self.compilation)
    }
}

//
// Choices
//

pub enum AbicoderVersion {
    AbicoderV1Keyword,
    AbicoderV2Keyword,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_abicoder_version(
    ir_node: &ir::AbicoderVersion,
    compilation: &Rc<CompilationUnit>,
) -> AbicoderVersion {
    match ir_node {
        ir::AbicoderVersion::AbicoderV1Keyword => AbicoderVersion::AbicoderV1Keyword,
        ir::AbicoderVersion::AbicoderV2Keyword => AbicoderVersion::AbicoderV2Keyword,
    }
}

pub enum ArgumentsDeclaration {
    PositionalArguments(PositionalArguments),
    NamedArguments(NamedArguments),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_arguments_declaration(
    ir_node: &ir::ArgumentsDeclaration,
    compilation: &Rc<CompilationUnit>,
) -> ArgumentsDeclaration {
    match ir_node {
        ir::ArgumentsDeclaration::PositionalArguments(nodes) => {
            ArgumentsDeclaration::PositionalArguments(create_positional_arguments(
                nodes,
                compilation,
            ))
        }

        ir::ArgumentsDeclaration::NamedArguments(nodes) => {
            ArgumentsDeclaration::NamedArguments(create_named_arguments(nodes, compilation))
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
pub(crate) fn create_contract_member(
    ir_node: &ir::ContractMember,
    compilation: &Rc<CompilationUnit>,
) -> ContractMember {
    match ir_node {
        ir::ContractMember::UsingDirective(variant) => {
            ContractMember::UsingDirective(create_using_directive(variant, compilation))
        }
        ir::ContractMember::FunctionDefinition(variant) => {
            ContractMember::FunctionDefinition(create_function_definition(variant, compilation))
        }
        ir::ContractMember::StructDefinition(variant) => {
            ContractMember::StructDefinition(create_struct_definition(variant, compilation))
        }
        ir::ContractMember::EnumDefinition(variant) => {
            ContractMember::EnumDefinition(create_enum_definition(variant, compilation))
        }
        ir::ContractMember::EventDefinition(variant) => {
            ContractMember::EventDefinition(create_event_definition(variant, compilation))
        }
        ir::ContractMember::ErrorDefinition(variant) => {
            ContractMember::ErrorDefinition(create_error_definition(variant, compilation))
        }
        ir::ContractMember::UserDefinedValueTypeDefinition(variant) => {
            ContractMember::UserDefinedValueTypeDefinition(
                create_user_defined_value_type_definition(variant, compilation),
            )
        }
        ir::ContractMember::StateVariableDefinition(variant) => {
            ContractMember::StateVariableDefinition(create_state_variable_definition(
                variant,
                compilation,
            ))
        }
        ir::ContractMember::ConstantDefinition(variant) => {
            ContractMember::ConstantDefinition(create_constant_definition(variant, compilation))
        }
    }
}

pub enum ElementaryType {
    BoolKeyword,
    StringKeyword,
    AddressType(AddressType),
    BytesKeyword(ir::BytesKeyword),
    IntKeyword(ir::IntKeyword),
    UintKeyword(ir::UintKeyword),
    FixedKeyword(ir::FixedKeyword),
    UfixedKeyword(ir::UfixedKeyword),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_elementary_type(
    ir_node: &ir::ElementaryType,
    compilation: &Rc<CompilationUnit>,
) -> ElementaryType {
    match ir_node {
        ir::ElementaryType::BoolKeyword => ElementaryType::BoolKeyword,
        ir::ElementaryType::StringKeyword => ElementaryType::StringKeyword,
        ir::ElementaryType::AddressType(variant) => {
            ElementaryType::AddressType(create_address_type(variant, compilation))
        }
        ir::ElementaryType::BytesKeyword(variant) => {
            ElementaryType::BytesKeyword(Rc::clone(variant))
        }
        ir::ElementaryType::IntKeyword(variant) => ElementaryType::IntKeyword(Rc::clone(variant)),
        ir::ElementaryType::UintKeyword(variant) => ElementaryType::UintKeyword(Rc::clone(variant)),
        ir::ElementaryType::FixedKeyword(variant) => {
            ElementaryType::FixedKeyword(Rc::clone(variant))
        }
        ir::ElementaryType::UfixedKeyword(variant) => {
            ElementaryType::UfixedKeyword(Rc::clone(variant))
        }
    }
}

pub enum ExperimentalFeature {
    ABIEncoderV2Keyword,
    SMTCheckerKeyword,
    StringLiteral(ir::StringLiteral),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_experimental_feature(
    ir_node: &ir::ExperimentalFeature,
    compilation: &Rc<CompilationUnit>,
) -> ExperimentalFeature {
    match ir_node {
        ir::ExperimentalFeature::ABIEncoderV2Keyword => ExperimentalFeature::ABIEncoderV2Keyword,
        ir::ExperimentalFeature::SMTCheckerKeyword => ExperimentalFeature::SMTCheckerKeyword,
        ir::ExperimentalFeature::StringLiteral(variant) => {
            ExperimentalFeature::StringLiteral(Rc::clone(variant))
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
    PayableKeyword,
    ThisKeyword,
    SuperKeyword,
    TrueKeyword,
    FalseKeyword,
    Identifier(Identifier),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_expression(
    ir_node: &ir::Expression,
    compilation: &Rc<CompilationUnit>,
) -> Expression {
    match ir_node {
        ir::Expression::AssignmentExpression(variant) => {
            Expression::AssignmentExpression(create_assignment_expression(variant, compilation))
        }
        ir::Expression::ConditionalExpression(variant) => {
            Expression::ConditionalExpression(create_conditional_expression(variant, compilation))
        }
        ir::Expression::OrExpression(variant) => {
            Expression::OrExpression(create_or_expression(variant, compilation))
        }
        ir::Expression::AndExpression(variant) => {
            Expression::AndExpression(create_and_expression(variant, compilation))
        }
        ir::Expression::EqualityExpression(variant) => {
            Expression::EqualityExpression(create_equality_expression(variant, compilation))
        }
        ir::Expression::InequalityExpression(variant) => {
            Expression::InequalityExpression(create_inequality_expression(variant, compilation))
        }
        ir::Expression::BitwiseOrExpression(variant) => {
            Expression::BitwiseOrExpression(create_bitwise_or_expression(variant, compilation))
        }
        ir::Expression::BitwiseXorExpression(variant) => {
            Expression::BitwiseXorExpression(create_bitwise_xor_expression(variant, compilation))
        }
        ir::Expression::BitwiseAndExpression(variant) => {
            Expression::BitwiseAndExpression(create_bitwise_and_expression(variant, compilation))
        }
        ir::Expression::ShiftExpression(variant) => {
            Expression::ShiftExpression(create_shift_expression(variant, compilation))
        }
        ir::Expression::AdditiveExpression(variant) => {
            Expression::AdditiveExpression(create_additive_expression(variant, compilation))
        }
        ir::Expression::MultiplicativeExpression(variant) => Expression::MultiplicativeExpression(
            create_multiplicative_expression(variant, compilation),
        ),
        ir::Expression::ExponentiationExpression(variant) => Expression::ExponentiationExpression(
            create_exponentiation_expression(variant, compilation),
        ),
        ir::Expression::PostfixExpression(variant) => {
            Expression::PostfixExpression(create_postfix_expression(variant, compilation))
        }
        ir::Expression::PrefixExpression(variant) => {
            Expression::PrefixExpression(create_prefix_expression(variant, compilation))
        }
        ir::Expression::FunctionCallExpression(variant) => Expression::FunctionCallExpression(
            create_function_call_expression(variant, compilation),
        ),
        ir::Expression::CallOptionsExpression(variant) => {
            Expression::CallOptionsExpression(create_call_options_expression(variant, compilation))
        }
        ir::Expression::MemberAccessExpression(variant) => Expression::MemberAccessExpression(
            create_member_access_expression(variant, compilation),
        ),
        ir::Expression::IndexAccessExpression(variant) => {
            Expression::IndexAccessExpression(create_index_access_expression(variant, compilation))
        }
        ir::Expression::NewExpression(variant) => {
            Expression::NewExpression(create_new_expression(variant, compilation))
        }
        ir::Expression::TupleExpression(variant) => {
            Expression::TupleExpression(create_tuple_expression(variant, compilation))
        }
        ir::Expression::TypeExpression(variant) => {
            Expression::TypeExpression(create_type_expression(variant, compilation))
        }
        ir::Expression::ArrayExpression(variant) => {
            Expression::ArrayExpression(create_array_expression(variant, compilation))
        }
        ir::Expression::HexNumberExpression(variant) => {
            Expression::HexNumberExpression(create_hex_number_expression(variant, compilation))
        }
        ir::Expression::DecimalNumberExpression(variant) => Expression::DecimalNumberExpression(
            create_decimal_number_expression(variant, compilation),
        ),
        ir::Expression::StringExpression(variant) => {
            Expression::StringExpression(create_string_expression(variant, compilation))
        }
        ir::Expression::ElementaryType(variant) => {
            Expression::ElementaryType(create_elementary_type(variant, compilation))
        }
        ir::Expression::PayableKeyword => Expression::PayableKeyword,
        ir::Expression::ThisKeyword => Expression::ThisKeyword,
        ir::Expression::SuperKeyword => Expression::SuperKeyword,
        ir::Expression::TrueKeyword => Expression::TrueKeyword,
        ir::Expression::FalseKeyword => Expression::FalseKeyword,
        ir::Expression::Identifier(variant) => {
            Expression::Identifier(create_identifier(variant, compilation))
        }
    }
}

pub enum Expression_AdditiveExpression_Operator {
    Minus,
    Plus,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_expression_additive_expression_operator(
    ir_node: &ir::Expression_AdditiveExpression_Operator,
    compilation: &Rc<CompilationUnit>,
) -> Expression_AdditiveExpression_Operator {
    match ir_node {
        ir::Expression_AdditiveExpression_Operator::Minus => {
            Expression_AdditiveExpression_Operator::Minus
        }
        ir::Expression_AdditiveExpression_Operator::Plus => {
            Expression_AdditiveExpression_Operator::Plus
        }
    }
}

pub enum Expression_AssignmentExpression_Operator {
    AmpersandEqual,
    AsteriskEqual,
    BarEqual,
    CaretEqual,
    Equal,
    GreaterThanGreaterThanEqual,
    GreaterThanGreaterThanGreaterThanEqual,
    LessThanLessThanEqual,
    MinusEqual,
    PercentEqual,
    PlusEqual,
    SlashEqual,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_expression_assignment_expression_operator(
    ir_node: &ir::Expression_AssignmentExpression_Operator,
    compilation: &Rc<CompilationUnit>,
) -> Expression_AssignmentExpression_Operator {
    match ir_node {
        ir::Expression_AssignmentExpression_Operator::AmpersandEqual => {
            Expression_AssignmentExpression_Operator::AmpersandEqual
        }
        ir::Expression_AssignmentExpression_Operator::AsteriskEqual => {
            Expression_AssignmentExpression_Operator::AsteriskEqual
        }
        ir::Expression_AssignmentExpression_Operator::BarEqual => {
            Expression_AssignmentExpression_Operator::BarEqual
        }
        ir::Expression_AssignmentExpression_Operator::CaretEqual => {
            Expression_AssignmentExpression_Operator::CaretEqual
        }
        ir::Expression_AssignmentExpression_Operator::Equal => {
            Expression_AssignmentExpression_Operator::Equal
        }
        ir::Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual => {
            Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual
        }
        ir::Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual => {
            Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual
        }
        ir::Expression_AssignmentExpression_Operator::LessThanLessThanEqual => {
            Expression_AssignmentExpression_Operator::LessThanLessThanEqual
        }
        ir::Expression_AssignmentExpression_Operator::MinusEqual => {
            Expression_AssignmentExpression_Operator::MinusEqual
        }
        ir::Expression_AssignmentExpression_Operator::PercentEqual => {
            Expression_AssignmentExpression_Operator::PercentEqual
        }
        ir::Expression_AssignmentExpression_Operator::PlusEqual => {
            Expression_AssignmentExpression_Operator::PlusEqual
        }
        ir::Expression_AssignmentExpression_Operator::SlashEqual => {
            Expression_AssignmentExpression_Operator::SlashEqual
        }
    }
}

pub enum Expression_EqualityExpression_Operator {
    BangEqual,
    EqualEqual,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_expression_equality_expression_operator(
    ir_node: &ir::Expression_EqualityExpression_Operator,
    compilation: &Rc<CompilationUnit>,
) -> Expression_EqualityExpression_Operator {
    match ir_node {
        ir::Expression_EqualityExpression_Operator::BangEqual => {
            Expression_EqualityExpression_Operator::BangEqual
        }
        ir::Expression_EqualityExpression_Operator::EqualEqual => {
            Expression_EqualityExpression_Operator::EqualEqual
        }
    }
}

pub enum Expression_InequalityExpression_Operator {
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_expression_inequality_expression_operator(
    ir_node: &ir::Expression_InequalityExpression_Operator,
    compilation: &Rc<CompilationUnit>,
) -> Expression_InequalityExpression_Operator {
    match ir_node {
        ir::Expression_InequalityExpression_Operator::GreaterThan => {
            Expression_InequalityExpression_Operator::GreaterThan
        }
        ir::Expression_InequalityExpression_Operator::GreaterThanEqual => {
            Expression_InequalityExpression_Operator::GreaterThanEqual
        }
        ir::Expression_InequalityExpression_Operator::LessThan => {
            Expression_InequalityExpression_Operator::LessThan
        }
        ir::Expression_InequalityExpression_Operator::LessThanEqual => {
            Expression_InequalityExpression_Operator::LessThanEqual
        }
    }
}

pub enum Expression_MultiplicativeExpression_Operator {
    Asterisk,
    Percent,
    Slash,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_expression_multiplicative_expression_operator(
    ir_node: &ir::Expression_MultiplicativeExpression_Operator,
    compilation: &Rc<CompilationUnit>,
) -> Expression_MultiplicativeExpression_Operator {
    match ir_node {
        ir::Expression_MultiplicativeExpression_Operator::Asterisk => {
            Expression_MultiplicativeExpression_Operator::Asterisk
        }
        ir::Expression_MultiplicativeExpression_Operator::Percent => {
            Expression_MultiplicativeExpression_Operator::Percent
        }
        ir::Expression_MultiplicativeExpression_Operator::Slash => {
            Expression_MultiplicativeExpression_Operator::Slash
        }
    }
}

pub enum Expression_PostfixExpression_Operator {
    MinusMinus,
    PlusPlus,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_expression_postfix_expression_operator(
    ir_node: &ir::Expression_PostfixExpression_Operator,
    compilation: &Rc<CompilationUnit>,
) -> Expression_PostfixExpression_Operator {
    match ir_node {
        ir::Expression_PostfixExpression_Operator::MinusMinus => {
            Expression_PostfixExpression_Operator::MinusMinus
        }
        ir::Expression_PostfixExpression_Operator::PlusPlus => {
            Expression_PostfixExpression_Operator::PlusPlus
        }
    }
}

pub enum Expression_PrefixExpression_Operator {
    Bang,
    DeleteKeyword,
    Minus,
    MinusMinus,
    PlusPlus,
    Tilde,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_expression_prefix_expression_operator(
    ir_node: &ir::Expression_PrefixExpression_Operator,
    compilation: &Rc<CompilationUnit>,
) -> Expression_PrefixExpression_Operator {
    match ir_node {
        ir::Expression_PrefixExpression_Operator::Bang => {
            Expression_PrefixExpression_Operator::Bang
        }
        ir::Expression_PrefixExpression_Operator::DeleteKeyword => {
            Expression_PrefixExpression_Operator::DeleteKeyword
        }
        ir::Expression_PrefixExpression_Operator::Minus => {
            Expression_PrefixExpression_Operator::Minus
        }
        ir::Expression_PrefixExpression_Operator::MinusMinus => {
            Expression_PrefixExpression_Operator::MinusMinus
        }
        ir::Expression_PrefixExpression_Operator::PlusPlus => {
            Expression_PrefixExpression_Operator::PlusPlus
        }
        ir::Expression_PrefixExpression_Operator::Tilde => {
            Expression_PrefixExpression_Operator::Tilde
        }
    }
}

pub enum Expression_ShiftExpression_Operator {
    GreaterThanGreaterThan,
    GreaterThanGreaterThanGreaterThan,
    LessThanLessThan,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_expression_shift_expression_operator(
    ir_node: &ir::Expression_ShiftExpression_Operator,
    compilation: &Rc<CompilationUnit>,
) -> Expression_ShiftExpression_Operator {
    match ir_node {
        ir::Expression_ShiftExpression_Operator::GreaterThanGreaterThan => {
            Expression_ShiftExpression_Operator::GreaterThanGreaterThan
        }
        ir::Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan => {
            Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan
        }
        ir::Expression_ShiftExpression_Operator::LessThanLessThan => {
            Expression_ShiftExpression_Operator::LessThanLessThan
        }
    }
}

pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_for_statement_condition(
    ir_node: &ir::ForStatementCondition,
    compilation: &Rc<CompilationUnit>,
) -> ForStatementCondition {
    match ir_node {
        ir::ForStatementCondition::ExpressionStatement(variant) => {
            ForStatementCondition::ExpressionStatement(create_expression_statement(
                variant,
                compilation,
            ))
        }
        ir::ForStatementCondition::Semicolon => ForStatementCondition::Semicolon,
    }
}

pub enum ForStatementInitialization {
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_for_statement_initialization(
    ir_node: &ir::ForStatementInitialization,
    compilation: &Rc<CompilationUnit>,
) -> ForStatementInitialization {
    match ir_node {
        ir::ForStatementInitialization::VariableDeclarationStatement(variant) => {
            ForStatementInitialization::VariableDeclarationStatement(
                create_variable_declaration_statement(variant, compilation),
            )
        }
        ir::ForStatementInitialization::ExpressionStatement(variant) => {
            ForStatementInitialization::ExpressionStatement(create_expression_statement(
                variant,
                compilation,
            ))
        }
        ir::ForStatementInitialization::Semicolon => ForStatementInitialization::Semicolon,
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
pub(crate) fn create_function_kind(
    ir_node: &ir::FunctionKind,
    compilation: &Rc<CompilationUnit>,
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
pub(crate) fn create_function_mutability(
    ir_node: &ir::FunctionMutability,
    compilation: &Rc<CompilationUnit>,
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
pub(crate) fn create_function_visibility(
    ir_node: &ir::FunctionVisibility,
    compilation: &Rc<CompilationUnit>,
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
pub(crate) fn create_import_clause(
    ir_node: &ir::ImportClause,
    compilation: &Rc<CompilationUnit>,
) -> ImportClause {
    match ir_node {
        ir::ImportClause::PathImport(variant) => {
            ImportClause::PathImport(create_path_import(variant, compilation))
        }
        ir::ImportClause::ImportDeconstruction(variant) => {
            ImportClause::ImportDeconstruction(create_import_deconstruction(variant, compilation))
        }
    }
}

pub enum NumberUnit {
    WeiKeyword,
    GweiKeyword,
    EtherKeyword,
    SecondsKeyword,
    MinutesKeyword,
    HoursKeyword,
    DaysKeyword,
    WeeksKeyword,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_number_unit(
    ir_node: &ir::NumberUnit,
    compilation: &Rc<CompilationUnit>,
) -> NumberUnit {
    match ir_node {
        ir::NumberUnit::WeiKeyword => NumberUnit::WeiKeyword,
        ir::NumberUnit::GweiKeyword => NumberUnit::GweiKeyword,
        ir::NumberUnit::EtherKeyword => NumberUnit::EtherKeyword,
        ir::NumberUnit::SecondsKeyword => NumberUnit::SecondsKeyword,
        ir::NumberUnit::MinutesKeyword => NumberUnit::MinutesKeyword,
        ir::NumberUnit::HoursKeyword => NumberUnit::HoursKeyword,
        ir::NumberUnit::DaysKeyword => NumberUnit::DaysKeyword,
        ir::NumberUnit::WeeksKeyword => NumberUnit::WeeksKeyword,
    }
}

pub enum Pragma {
    VersionPragma(VersionPragma),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_pragma(ir_node: &ir::Pragma, compilation: &Rc<CompilationUnit>) -> Pragma {
    match ir_node {
        ir::Pragma::VersionPragma(variant) => {
            Pragma::VersionPragma(create_version_pragma(variant, compilation))
        }
        ir::Pragma::AbicoderPragma(variant) => {
            Pragma::AbicoderPragma(create_abicoder_pragma(variant, compilation))
        }
        ir::Pragma::ExperimentalPragma(variant) => {
            Pragma::ExperimentalPragma(create_experimental_pragma(variant, compilation))
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
pub(crate) fn create_source_unit_member(
    ir_node: &ir::SourceUnitMember,
    compilation: &Rc<CompilationUnit>,
) -> SourceUnitMember {
    match ir_node {
        ir::SourceUnitMember::PragmaDirective(variant) => {
            SourceUnitMember::PragmaDirective(create_pragma_directive(variant, compilation))
        }
        ir::SourceUnitMember::ImportClause(variant) => {
            SourceUnitMember::ImportClause(create_import_clause(variant, compilation))
        }
        ir::SourceUnitMember::ContractDefinition(variant) => {
            SourceUnitMember::ContractDefinition(create_contract_definition(variant, compilation))
        }
        ir::SourceUnitMember::InterfaceDefinition(variant) => {
            SourceUnitMember::InterfaceDefinition(create_interface_definition(variant, compilation))
        }
        ir::SourceUnitMember::LibraryDefinition(variant) => {
            SourceUnitMember::LibraryDefinition(create_library_definition(variant, compilation))
        }
        ir::SourceUnitMember::StructDefinition(variant) => {
            SourceUnitMember::StructDefinition(create_struct_definition(variant, compilation))
        }
        ir::SourceUnitMember::EnumDefinition(variant) => {
            SourceUnitMember::EnumDefinition(create_enum_definition(variant, compilation))
        }
        ir::SourceUnitMember::FunctionDefinition(variant) => {
            SourceUnitMember::FunctionDefinition(create_function_definition(variant, compilation))
        }
        ir::SourceUnitMember::ErrorDefinition(variant) => {
            SourceUnitMember::ErrorDefinition(create_error_definition(variant, compilation))
        }
        ir::SourceUnitMember::UserDefinedValueTypeDefinition(variant) => {
            SourceUnitMember::UserDefinedValueTypeDefinition(
                create_user_defined_value_type_definition(variant, compilation),
            )
        }
        ir::SourceUnitMember::UsingDirective(variant) => {
            SourceUnitMember::UsingDirective(create_using_directive(variant, compilation))
        }
        ir::SourceUnitMember::EventDefinition(variant) => {
            SourceUnitMember::EventDefinition(create_event_definition(variant, compilation))
        }
        ir::SourceUnitMember::ConstantDefinition(variant) => {
            SourceUnitMember::ConstantDefinition(create_constant_definition(variant, compilation))
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
pub(crate) fn create_state_variable_mutability(
    ir_node: &ir::StateVariableMutability,
    compilation: &Rc<CompilationUnit>,
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
pub(crate) fn create_state_variable_visibility(
    ir_node: &ir::StateVariableVisibility,
    compilation: &Rc<CompilationUnit>,
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
pub(crate) fn create_statement(
    ir_node: &ir::Statement,
    compilation: &Rc<CompilationUnit>,
) -> Statement {
    match ir_node {
        ir::Statement::IfStatement(variant) => {
            Statement::IfStatement(create_if_statement(variant, compilation))
        }
        ir::Statement::ForStatement(variant) => {
            Statement::ForStatement(create_for_statement(variant, compilation))
        }
        ir::Statement::WhileStatement(variant) => {
            Statement::WhileStatement(create_while_statement(variant, compilation))
        }
        ir::Statement::DoWhileStatement(variant) => {
            Statement::DoWhileStatement(create_do_while_statement(variant, compilation))
        }
        ir::Statement::ContinueStatement(variant) => {
            Statement::ContinueStatement(create_continue_statement(variant, compilation))
        }
        ir::Statement::BreakStatement(variant) => {
            Statement::BreakStatement(create_break_statement(variant, compilation))
        }
        ir::Statement::ReturnStatement(variant) => {
            Statement::ReturnStatement(create_return_statement(variant, compilation))
        }
        ir::Statement::EmitStatement(variant) => {
            Statement::EmitStatement(create_emit_statement(variant, compilation))
        }
        ir::Statement::TryStatement(variant) => {
            Statement::TryStatement(create_try_statement(variant, compilation))
        }
        ir::Statement::RevertStatement(variant) => {
            Statement::RevertStatement(create_revert_statement(variant, compilation))
        }
        ir::Statement::AssemblyStatement(variant) => {
            Statement::AssemblyStatement(create_assembly_statement(variant, compilation))
        }
        ir::Statement::Block(variant) => Statement::Block(create_block(variant, compilation)),
        ir::Statement::UncheckedBlock(variant) => {
            Statement::UncheckedBlock(create_unchecked_block(variant, compilation))
        }
        ir::Statement::VariableDeclarationStatement(variant) => {
            Statement::VariableDeclarationStatement(create_variable_declaration_statement(
                variant,
                compilation,
            ))
        }
        ir::Statement::ExpressionStatement(variant) => {
            Statement::ExpressionStatement(create_expression_statement(variant, compilation))
        }
    }
}

pub enum StorageLocation {
    MemoryKeyword,
    StorageKeyword,
    CallDataKeyword,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_storage_location(
    ir_node: &ir::StorageLocation,
    compilation: &Rc<CompilationUnit>,
) -> StorageLocation {
    match ir_node {
        ir::StorageLocation::MemoryKeyword => StorageLocation::MemoryKeyword,
        ir::StorageLocation::StorageKeyword => StorageLocation::StorageKeyword,
        ir::StorageLocation::CallDataKeyword => StorageLocation::CallDataKeyword,
    }
}

pub enum StringExpression {
    StringLiterals(Vec<ir::StringLiteral>),
    HexStringLiterals(Vec<ir::HexStringLiteral>),
    UnicodeStringLiterals(Vec<ir::UnicodeStringLiteral>),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_string_expression(
    ir_node: &ir::StringExpression,
    compilation: &Rc<CompilationUnit>,
) -> StringExpression {
    match ir_node {
        ir::StringExpression::StringLiterals(nodes) => {
            StringExpression::StringLiterals(nodes.clone())
        }

        ir::StringExpression::HexStringLiterals(nodes) => {
            StringExpression::HexStringLiterals(nodes.clone())
        }

        ir::StringExpression::UnicodeStringLiterals(nodes) => {
            StringExpression::UnicodeStringLiterals(nodes.clone())
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
pub(crate) fn create_type_name(
    ir_node: &ir::TypeName,
    compilation: &Rc<CompilationUnit>,
) -> TypeName {
    match ir_node {
        ir::TypeName::ArrayTypeName(variant) => {
            TypeName::ArrayTypeName(create_array_type_name(variant, compilation))
        }
        ir::TypeName::FunctionType(variant) => {
            TypeName::FunctionType(create_function_type(variant, compilation))
        }
        ir::TypeName::MappingType(variant) => {
            TypeName::MappingType(create_mapping_type(variant, compilation))
        }
        ir::TypeName::ElementaryType(variant) => {
            TypeName::ElementaryType(create_elementary_type(variant, compilation))
        }
        ir::TypeName::IdentifierPath(nodes) => {
            TypeName::IdentifierPath(create_identifier_path(nodes, compilation))
        }
    }
}

pub enum UsingClause {
    IdentifierPath(IdentifierPath),
    UsingDeconstruction(UsingDeconstruction),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_using_clause(
    ir_node: &ir::UsingClause,
    compilation: &Rc<CompilationUnit>,
) -> UsingClause {
    match ir_node {
        ir::UsingClause::IdentifierPath(nodes) => {
            UsingClause::IdentifierPath(create_identifier_path(nodes, compilation))
        }

        ir::UsingClause::UsingDeconstruction(variant) => {
            UsingClause::UsingDeconstruction(create_using_deconstruction(variant, compilation))
        }
    }
}

pub enum UsingOperator {
    Ampersand,
    Asterisk,
    BangEqual,
    Bar,
    Caret,
    EqualEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Minus,
    Percent,
    Plus,
    Slash,
    Tilde,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_using_operator(
    ir_node: &ir::UsingOperator,
    compilation: &Rc<CompilationUnit>,
) -> UsingOperator {
    match ir_node {
        ir::UsingOperator::Ampersand => UsingOperator::Ampersand,
        ir::UsingOperator::Asterisk => UsingOperator::Asterisk,
        ir::UsingOperator::BangEqual => UsingOperator::BangEqual,
        ir::UsingOperator::Bar => UsingOperator::Bar,
        ir::UsingOperator::Caret => UsingOperator::Caret,
        ir::UsingOperator::EqualEqual => UsingOperator::EqualEqual,
        ir::UsingOperator::GreaterThan => UsingOperator::GreaterThan,
        ir::UsingOperator::GreaterThanEqual => UsingOperator::GreaterThanEqual,
        ir::UsingOperator::LessThan => UsingOperator::LessThan,
        ir::UsingOperator::LessThanEqual => UsingOperator::LessThanEqual,
        ir::UsingOperator::Minus => UsingOperator::Minus,
        ir::UsingOperator::Percent => UsingOperator::Percent,
        ir::UsingOperator::Plus => UsingOperator::Plus,
        ir::UsingOperator::Slash => UsingOperator::Slash,
        ir::UsingOperator::Tilde => UsingOperator::Tilde,
    }
}

pub enum UsingTarget {
    TypeName(TypeName),
    Asterisk,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_using_target(
    ir_node: &ir::UsingTarget,
    compilation: &Rc<CompilationUnit>,
) -> UsingTarget {
    match ir_node {
        ir::UsingTarget::TypeName(variant) => {
            UsingTarget::TypeName(create_type_name(variant, compilation))
        }
        ir::UsingTarget::Asterisk => UsingTarget::Asterisk,
    }
}

pub enum VariableDeclarationTarget {
    SingleTypedDeclaration(SingleTypedDeclaration),
    MultiTypedDeclaration(MultiTypedDeclaration),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_variable_declaration_target(
    ir_node: &ir::VariableDeclarationTarget,
    compilation: &Rc<CompilationUnit>,
) -> VariableDeclarationTarget {
    match ir_node {
        ir::VariableDeclarationTarget::SingleTypedDeclaration(variant) => {
            VariableDeclarationTarget::SingleTypedDeclaration(create_single_typed_declaration(
                variant,
                compilation,
            ))
        }
        ir::VariableDeclarationTarget::MultiTypedDeclaration(variant) => {
            VariableDeclarationTarget::MultiTypedDeclaration(create_multi_typed_declaration(
                variant,
                compilation,
            ))
        }
    }
}

pub enum VersionExpression {
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_version_expression(
    ir_node: &ir::VersionExpression,
    compilation: &Rc<CompilationUnit>,
) -> VersionExpression {
    match ir_node {
        ir::VersionExpression::VersionRange(variant) => {
            VersionExpression::VersionRange(create_version_range(variant, compilation))
        }
        ir::VersionExpression::VersionTerm(variant) => {
            VersionExpression::VersionTerm(create_version_term(variant, compilation))
        }
    }
}

pub enum VersionLiteral {
    SimpleVersionLiteral(Vec<ir::VersionSpecifier>),
    StringLiteral(ir::StringLiteral),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_version_literal(
    ir_node: &ir::VersionLiteral,
    compilation: &Rc<CompilationUnit>,
) -> VersionLiteral {
    match ir_node {
        ir::VersionLiteral::SimpleVersionLiteral(nodes) => {
            VersionLiteral::SimpleVersionLiteral(nodes.clone())
        }

        ir::VersionLiteral::StringLiteral(variant) => {
            VersionLiteral::StringLiteral(Rc::clone(variant))
        }
    }
}

pub enum VersionOperator {
    PragmaCaret,
    PragmaTilde,
    PragmaEqual,
    PragmaLessThan,
    PragmaGreaterThan,
    PragmaLessThanEqual,
    PragmaGreaterThanEqual,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_version_operator(
    ir_node: &ir::VersionOperator,
    compilation: &Rc<CompilationUnit>,
) -> VersionOperator {
    match ir_node {
        ir::VersionOperator::PragmaCaret => VersionOperator::PragmaCaret,
        ir::VersionOperator::PragmaTilde => VersionOperator::PragmaTilde,
        ir::VersionOperator::PragmaEqual => VersionOperator::PragmaEqual,
        ir::VersionOperator::PragmaLessThan => VersionOperator::PragmaLessThan,
        ir::VersionOperator::PragmaGreaterThan => VersionOperator::PragmaGreaterThan,
        ir::VersionOperator::PragmaLessThanEqual => VersionOperator::PragmaLessThanEqual,
        ir::VersionOperator::PragmaGreaterThanEqual => VersionOperator::PragmaGreaterThanEqual,
    }
}

pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_yul_expression(
    ir_node: &ir::YulExpression,
    compilation: &Rc<CompilationUnit>,
) -> YulExpression {
    match ir_node {
        ir::YulExpression::YulFunctionCallExpression(variant) => {
            YulExpression::YulFunctionCallExpression(create_yul_function_call_expression(
                variant,
                compilation,
            ))
        }
        ir::YulExpression::YulLiteral(variant) => {
            YulExpression::YulLiteral(create_yul_literal(variant, compilation))
        }
        ir::YulExpression::YulPath(nodes) => {
            YulExpression::YulPath(create_yul_path(nodes, compilation))
        }
    }
}

pub enum YulLiteral {
    TrueKeyword,
    FalseKeyword,
    DecimalLiteral(ir::DecimalLiteral),
    HexLiteral(ir::HexLiteral),
    HexStringLiteral(ir::HexStringLiteral),
    StringLiteral(ir::StringLiteral),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_yul_literal(
    ir_node: &ir::YulLiteral,
    compilation: &Rc<CompilationUnit>,
) -> YulLiteral {
    match ir_node {
        ir::YulLiteral::TrueKeyword => YulLiteral::TrueKeyword,
        ir::YulLiteral::FalseKeyword => YulLiteral::FalseKeyword,
        ir::YulLiteral::DecimalLiteral(variant) => YulLiteral::DecimalLiteral(Rc::clone(variant)),
        ir::YulLiteral::HexLiteral(variant) => YulLiteral::HexLiteral(Rc::clone(variant)),
        ir::YulLiteral::HexStringLiteral(variant) => {
            YulLiteral::HexStringLiteral(Rc::clone(variant))
        }
        ir::YulLiteral::StringLiteral(variant) => YulLiteral::StringLiteral(Rc::clone(variant)),
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
pub(crate) fn create_yul_statement(
    ir_node: &ir::YulStatement,
    compilation: &Rc<CompilationUnit>,
) -> YulStatement {
    match ir_node {
        ir::YulStatement::YulBlock(variant) => {
            YulStatement::YulBlock(create_yul_block(variant, compilation))
        }
        ir::YulStatement::YulFunctionDefinition(variant) => YulStatement::YulFunctionDefinition(
            create_yul_function_definition(variant, compilation),
        ),
        ir::YulStatement::YulIfStatement(variant) => {
            YulStatement::YulIfStatement(create_yul_if_statement(variant, compilation))
        }
        ir::YulStatement::YulForStatement(variant) => {
            YulStatement::YulForStatement(create_yul_for_statement(variant, compilation))
        }
        ir::YulStatement::YulSwitchStatement(variant) => {
            YulStatement::YulSwitchStatement(create_yul_switch_statement(variant, compilation))
        }
        ir::YulStatement::YulLeaveStatement(variant) => {
            YulStatement::YulLeaveStatement(create_yul_leave_statement(variant, compilation))
        }
        ir::YulStatement::YulBreakStatement(variant) => {
            YulStatement::YulBreakStatement(create_yul_break_statement(variant, compilation))
        }
        ir::YulStatement::YulContinueStatement(variant) => {
            YulStatement::YulContinueStatement(create_yul_continue_statement(variant, compilation))
        }
        ir::YulStatement::YulVariableAssignmentStatement(variant) => {
            YulStatement::YulVariableAssignmentStatement(create_yul_variable_assignment_statement(
                variant,
                compilation,
            ))
        }
        ir::YulStatement::YulVariableDeclarationStatement(variant) => {
            YulStatement::YulVariableDeclarationStatement(
                create_yul_variable_declaration_statement(variant, compilation),
            )
        }
        ir::YulStatement::YulExpression(variant) => {
            YulStatement::YulExpression(create_yul_expression(variant, compilation))
        }
    }
}

pub enum YulSwitchCase {
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_yul_switch_case(
    ir_node: &ir::YulSwitchCase,
    compilation: &Rc<CompilationUnit>,
) -> YulSwitchCase {
    match ir_node {
        ir::YulSwitchCase::YulDefaultCase(variant) => {
            YulSwitchCase::YulDefaultCase(create_yul_default_case(variant, compilation))
        }
        ir::YulSwitchCase::YulValueCase(variant) => {
            YulSwitchCase::YulValueCase(create_yul_value_case(variant, compilation))
        }
    }
}

//
// Repeated & Separated
//

pub type ArrayValues = Rc<ArrayValuesStruct>;

pub(crate) fn create_array_values(
    nodes: &[ir::Expression],
    compilation: &Rc<CompilationUnit>,
) -> ArrayValues {
    Rc::new(ArrayValuesStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct ArrayValuesStruct {
    pub(crate) ir_nodes: Vec<ir::Expression>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl ArrayValuesStruct {
    pub fn iter(&self) -> impl Iterator<Item = Expression> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_expression(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> CallOptions {
    Rc::new(CallOptionsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct CallOptionsStruct {
    pub(crate) ir_nodes: Vec<ir::NamedArgument>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl CallOptionsStruct {
    pub fn iter(&self) -> impl Iterator<Item = NamedArgument> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_named_argument(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> CatchClauses {
    Rc::new(CatchClausesStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct CatchClausesStruct {
    pub(crate) ir_nodes: Vec<ir::CatchClause>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl CatchClausesStruct {
    pub fn iter(&self) -> impl Iterator<Item = CatchClause> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_catch_clause(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> ContractMembers {
    Rc::new(ContractMembersStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct ContractMembersStruct {
    pub(crate) ir_nodes: Vec<ir::ContractMember>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl ContractMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = ContractMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_contract_member(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> EnumMembers {
    Rc::new(EnumMembersStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct EnumMembersStruct {
    pub(crate) ir_nodes: Vec<ir::Identifier>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl EnumMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> IdentifierPath {
    Rc::new(IdentifierPathStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct IdentifierPathStruct {
    pub(crate) ir_nodes: Vec<ir::Identifier>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl IdentifierPathStruct {
    pub fn iter(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> ImportDeconstructionSymbols {
    Rc::new(ImportDeconstructionSymbolsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct ImportDeconstructionSymbolsStruct {
    pub(crate) ir_nodes: Vec<ir::ImportDeconstructionSymbol>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl ImportDeconstructionSymbolsStruct {
    pub fn iter(&self) -> impl Iterator<Item = ImportDeconstructionSymbol> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_import_deconstruction_symbol(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> InheritanceTypes {
    Rc::new(InheritanceTypesStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct InheritanceTypesStruct {
    pub(crate) ir_nodes: Vec<ir::InheritanceType>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl InheritanceTypesStruct {
    pub fn iter(&self) -> impl Iterator<Item = InheritanceType> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_inheritance_type(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> InterfaceMembers {
    Rc::new(InterfaceMembersStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct InterfaceMembersStruct {
    pub(crate) ir_nodes: Vec<ir::ContractMember>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl InterfaceMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = ContractMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_contract_member(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> LibraryMembers {
    Rc::new(LibraryMembersStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct LibraryMembersStruct {
    pub(crate) ir_nodes: Vec<ir::ContractMember>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl LibraryMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = ContractMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_contract_member(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> ModifierInvocations {
    Rc::new(ModifierInvocationsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct ModifierInvocationsStruct {
    pub(crate) ir_nodes: Vec<ir::ModifierInvocation>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl ModifierInvocationsStruct {
    pub fn iter(&self) -> impl Iterator<Item = ModifierInvocation> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_modifier_invocation(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> MultiTypedDeclarationElements {
    Rc::new(MultiTypedDeclarationElementsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct MultiTypedDeclarationElementsStruct {
    pub(crate) ir_nodes: Vec<ir::MultiTypedDeclarationElement>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl MultiTypedDeclarationElementsStruct {
    pub fn iter(&self) -> impl Iterator<Item = MultiTypedDeclarationElement> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_multi_typed_declaration_element(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> NamedArguments {
    Rc::new(NamedArgumentsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct NamedArgumentsStruct {
    pub(crate) ir_nodes: Vec<ir::NamedArgument>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl NamedArgumentsStruct {
    pub fn iter(&self) -> impl Iterator<Item = NamedArgument> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_named_argument(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> OverridePaths {
    Rc::new(OverridePathsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct OverridePathsStruct {
    pub(crate) ir_nodes: Vec<ir::IdentifierPath>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl OverridePathsStruct {
    pub fn iter(&self) -> impl Iterator<Item = IdentifierPath> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier_path(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> Parameters {
    Rc::new(ParametersStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct ParametersStruct {
    pub(crate) ir_nodes: Vec<ir::Parameter>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl ParametersStruct {
    pub fn iter(&self) -> impl Iterator<Item = Parameter> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_parameter(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> PositionalArguments {
    Rc::new(PositionalArgumentsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct PositionalArgumentsStruct {
    pub(crate) ir_nodes: Vec<ir::Expression>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl PositionalArgumentsStruct {
    pub fn iter(&self) -> impl Iterator<Item = Expression> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_expression(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> SourceUnitMembers {
    Rc::new(SourceUnitMembersStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct SourceUnitMembersStruct {
    pub(crate) ir_nodes: Vec<ir::SourceUnitMember>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl SourceUnitMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = SourceUnitMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_source_unit_member(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> Statements {
    Rc::new(StatementsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct StatementsStruct {
    pub(crate) ir_nodes: Vec<ir::Statement>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl StatementsStruct {
    pub fn iter(&self) -> impl Iterator<Item = Statement> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_statement(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> StructMembers {
    Rc::new(StructMembersStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct StructMembersStruct {
    pub(crate) ir_nodes: Vec<ir::StructMember>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl StructMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = StructMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_struct_member(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> TupleValues {
    Rc::new(TupleValuesStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct TupleValuesStruct {
    pub(crate) ir_nodes: Vec<ir::TupleValue>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl TupleValuesStruct {
    pub fn iter(&self) -> impl Iterator<Item = TupleValue> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_tuple_value(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> UsingDeconstructionSymbols {
    Rc::new(UsingDeconstructionSymbolsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct UsingDeconstructionSymbolsStruct {
    pub(crate) ir_nodes: Vec<ir::UsingDeconstructionSymbol>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl UsingDeconstructionSymbolsStruct {
    pub fn iter(&self) -> impl Iterator<Item = UsingDeconstructionSymbol> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_using_deconstruction_symbol(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> VersionExpressionSet {
    Rc::new(VersionExpressionSetStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct VersionExpressionSetStruct {
    pub(crate) ir_nodes: Vec<ir::VersionExpression>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl VersionExpressionSetStruct {
    pub fn iter(&self) -> impl Iterator<Item = VersionExpression> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_version_expression(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> VersionExpressionSets {
    Rc::new(VersionExpressionSetsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct VersionExpressionSetsStruct {
    pub(crate) ir_nodes: Vec<ir::VersionExpressionSet>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl VersionExpressionSetsStruct {
    pub fn iter(&self) -> impl Iterator<Item = VersionExpressionSet> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_version_expression_set(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> YulArguments {
    Rc::new(YulArgumentsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct YulArgumentsStruct {
    pub(crate) ir_nodes: Vec<ir::YulExpression>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl YulArgumentsStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulExpression> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_expression(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> YulParameters {
    Rc::new(YulParametersStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct YulParametersStruct {
    pub(crate) ir_nodes: Vec<ir::Identifier>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl YulParametersStruct {
    pub fn iter(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier(ir_node, &self.compilation))
    }
    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}

pub type YulPath = Rc<YulPathStruct>;

pub(crate) fn create_yul_path(
    nodes: &[ir::Identifier],
    compilation: &Rc<CompilationUnit>,
) -> YulPath {
    Rc::new(YulPathStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct YulPathStruct {
    pub(crate) ir_nodes: Vec<ir::Identifier>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl YulPathStruct {
    pub fn iter(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier(ir_node, &self.compilation))
    }
    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}

pub type YulPaths = Rc<YulPathsStruct>;

pub(crate) fn create_yul_paths(
    nodes: &[ir::YulPath],
    compilation: &Rc<CompilationUnit>,
) -> YulPaths {
    Rc::new(YulPathsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct YulPathsStruct {
    pub(crate) ir_nodes: Vec<ir::YulPath>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl YulPathsStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulPath> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_path(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> YulStatements {
    Rc::new(YulStatementsStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct YulStatementsStruct {
    pub(crate) ir_nodes: Vec<ir::YulStatement>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl YulStatementsStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulStatement> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_statement(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> YulSwitchCases {
    Rc::new(YulSwitchCasesStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct YulSwitchCasesStruct {
    pub(crate) ir_nodes: Vec<ir::YulSwitchCase>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl YulSwitchCasesStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulSwitchCase> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_switch_case(ir_node, &self.compilation))
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
    compilation: &Rc<CompilationUnit>,
) -> YulVariableNames {
    Rc::new(YulVariableNamesStruct {
        ir_nodes: nodes.to_vec(),
        compilation: Rc::clone(compilation),
    })
}

pub struct YulVariableNamesStruct {
    pub(crate) ir_nodes: Vec<ir::Identifier>,
    pub(crate) compilation: Rc<CompilationUnit>,
}

impl YulVariableNamesStruct {
    pub fn iter(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier(ir_node, &self.compilation))
    }
    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}

//
// Identifiers
//

pub type Identifier = Rc<IdentifierStruct>;

pub struct IdentifierStruct {
    pub(crate) ir_node: ir::Identifier,
    pub(crate) compilation: Rc<CompilationUnit>,
}

pub(crate) fn create_identifier(
    ir_node: &ir::Identifier,
    compilation: &Rc<CompilationUnit>,
) -> Identifier {
    Rc::new(IdentifierStruct {
        ir_node: Rc::clone(ir_node),
        compilation: Rc::clone(compilation),
    })
}

impl IdentifierStruct {
    pub fn id(self: &Rc<Self>) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &str {
        self.ir_node.unparse()
    }
}
