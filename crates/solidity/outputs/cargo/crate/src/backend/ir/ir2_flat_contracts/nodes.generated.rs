// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;
use std::vec::Vec;

use metaslang_cst::nodes::NodeId;
use serde::Serialize;

use crate::cst::TerminalNode;

//
// Sequences:
//

pub type SourceUnit = Rc<SourceUnitStruct>;

#[derive(Debug, Serialize)]
pub struct SourceUnitStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub members: SourceUnitMembers,
}

pub type PragmaDirective = Rc<PragmaDirectiveStruct>;

#[derive(Debug, Serialize)]
pub struct PragmaDirectiveStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub pragma: Pragma,
}

pub type AbicoderPragma = Rc<AbicoderPragmaStruct>;

#[derive(Debug, Serialize)]
pub struct AbicoderPragmaStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub version: AbicoderVersion,
}

pub type ExperimentalPragma = Rc<ExperimentalPragmaStruct>;

#[derive(Debug, Serialize)]
pub struct ExperimentalPragmaStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub feature: ExperimentalFeature,
}

pub type VersionPragma = Rc<VersionPragmaStruct>;

#[derive(Debug, Serialize)]
pub struct VersionPragmaStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub sets: VersionExpressionSets,
}

pub type VersionRange = Rc<VersionRangeStruct>;

#[derive(Debug, Serialize)]
pub struct VersionRangeStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub start: VersionLiteral,
    pub end: VersionLiteral,
}

pub type VersionTerm = Rc<VersionTermStruct>;

#[derive(Debug, Serialize)]
pub struct VersionTermStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub operator: Option<VersionOperator>,
    pub literal: VersionLiteral,
}

pub type PathImport = Rc<PathImportStruct>;

#[derive(Debug, Serialize)]
pub struct PathImportStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub alias: Option<Rc<TerminalNode>>,
    pub path: Rc<TerminalNode>,
}

pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

#[derive(Debug, Serialize)]
pub struct ImportDeconstructionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub symbols: ImportDeconstructionSymbols,
    pub path: Rc<TerminalNode>,
}

pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

#[derive(Debug, Serialize)]
pub struct ImportDeconstructionSymbolStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub alias: Option<Rc<TerminalNode>>,
}

pub type UsingDirective = Rc<UsingDirectiveStruct>;

#[derive(Debug, Serialize)]
pub struct UsingDirectiveStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub clause: UsingClause,
    pub target: UsingTarget,
    pub global_keyword: bool,
}

pub type UsingDeconstruction = Rc<UsingDeconstructionStruct>;

#[derive(Debug, Serialize)]
pub struct UsingDeconstructionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub symbols: UsingDeconstructionSymbols,
}

pub type UsingDeconstructionSymbol = Rc<UsingDeconstructionSymbolStruct>;

#[derive(Debug, Serialize)]
pub struct UsingDeconstructionSymbolStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: IdentifierPath,
    pub alias: Option<UsingOperator>,
}

pub type ContractDefinition = Rc<ContractDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct ContractDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub abstract_keyword: bool,
    pub name: Rc<TerminalNode>,
    pub members: ContractMembers,
    pub inheritance_types: InheritanceTypes,
    pub storage_layout: Option<Expression>,
}

pub type InheritanceType = Rc<InheritanceTypeStruct>;

#[derive(Debug, Serialize)]
pub struct InheritanceTypeStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct InterfaceDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub inheritance: Option<InheritanceTypes>,
    pub members: InterfaceMembers,
}

pub type LibraryDefinition = Rc<LibraryDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct LibraryDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub members: LibraryMembers,
}

pub type StructDefinition = Rc<StructDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct StructDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub members: StructMembers,
}

pub type StructMember = Rc<StructMemberStruct>;

#[derive(Debug, Serialize)]
pub struct StructMemberStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub name: Rc<TerminalNode>,
}

pub type EnumDefinition = Rc<EnumDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct EnumDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub members: EnumMembers,
}

pub type ConstantDefinition = Rc<ConstantDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct ConstantDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub name: Rc<TerminalNode>,
    pub visibility: Option<StateVariableVisibility>,
    pub value: Option<Expression>,
}

pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct StateVariableDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub name: Rc<TerminalNode>,
    pub value: Option<Expression>,
    pub visibility: StateVariableVisibility,
    pub mutability: StateVariableMutability,
    pub override_specifier: Option<OverridePaths>,
}

pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct FunctionDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub parameters: Parameters,
    pub returns: Option<Parameters>,
    pub kind: FunctionKind,
    pub name: Option<Rc<TerminalNode>>,
    pub body: Option<Block>,
    pub visibility: FunctionVisibility,
    pub mutability: FunctionMutability,
    pub virtual_keyword: bool,
    pub override_specifier: Option<OverridePaths>,
    pub modifier_invocations: ModifierInvocations,
}

pub type Parameter = Rc<ParameterStruct>;

#[derive(Debug, Serialize)]
pub struct ParameterStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Option<Rc<TerminalNode>>,
    pub indexed: bool,
}

pub type OverrideSpecifier = Rc<OverrideSpecifierStruct>;

#[derive(Debug, Serialize)]
pub struct OverrideSpecifierStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub overridden: Option<OverridePaths>,
}

pub type ModifierInvocation = Rc<ModifierInvocationStruct>;

#[derive(Debug, Serialize)]
pub struct ModifierInvocationStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

pub type EventDefinition = Rc<EventDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct EventDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub anonymous_keyword: bool,
    pub parameters: Parameters,
}

pub type UserDefinedValueTypeDefinition = Rc<UserDefinedValueTypeDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct UserDefinedValueTypeDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub value_type: ElementaryType,
}

pub type ErrorDefinition = Rc<ErrorDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct ErrorDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub parameters: Parameters,
}

pub type ArrayTypeName = Rc<ArrayTypeNameStruct>;

#[derive(Debug, Serialize)]
pub struct ArrayTypeNameStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub operand: TypeName,
    pub index: Option<Expression>,
}

pub type FunctionType = Rc<FunctionTypeStruct>;

#[derive(Debug, Serialize)]
pub struct FunctionTypeStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub parameters: Parameters,
    pub returns: Option<Parameters>,
    pub visibility: FunctionVisibility,
    pub mutability: FunctionMutability,
}

pub type MappingType = Rc<MappingTypeStruct>;

#[derive(Debug, Serialize)]
pub struct MappingTypeStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub key_type: Parameter,
    pub value_type: Parameter,
}

pub type AddressType = Rc<AddressTypeStruct>;

#[derive(Debug, Serialize)]
pub struct AddressTypeStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub payable_keyword: bool,
}

pub type Block = Rc<BlockStruct>;

#[derive(Debug, Serialize)]
pub struct BlockStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub statements: Statements,
}

pub type UncheckedBlock = Rc<UncheckedBlockStruct>;

#[derive(Debug, Serialize)]
pub struct UncheckedBlockStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub block: Block,
}

pub type ExpressionStatement = Rc<ExpressionStatementStruct>;

#[derive(Debug, Serialize)]
pub struct ExpressionStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub expression: Expression,
}

pub type AssemblyStatement = Rc<AssemblyStatementStruct>;

#[derive(Debug, Serialize)]
pub struct AssemblyStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub body: YulBlock,
    pub flags: AssemblyFlags,
    pub label: Option<Rc<TerminalNode>>,
}

pub type TupleDeconstructionStatement = Rc<TupleDeconstructionStatementStruct>;

#[derive(Debug, Serialize)]
pub struct TupleDeconstructionStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub expression: Expression,
    pub members: TupleDeconstructionMembers,
}

pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

#[derive(Debug, Serialize)]
pub struct VariableDeclarationStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
    pub value: Option<Expression>,
    pub type_name: Option<TypeName>,
}

pub type IfStatement = Rc<IfStatementStruct>;

#[derive(Debug, Serialize)]
pub struct IfStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub condition: Expression,
    pub body: Statement,
    pub else_branch: Option<Statement>,
}

pub type ForStatement = Rc<ForStatementStruct>;

#[derive(Debug, Serialize)]
pub struct ForStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub initialization: ForStatementInitialization,
    pub condition: ForStatementCondition,
    pub iterator: Option<Expression>,
    pub body: Statement,
}

pub type WhileStatement = Rc<WhileStatementStruct>;

#[derive(Debug, Serialize)]
pub struct WhileStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub condition: Expression,
    pub body: Statement,
}

pub type DoWhileStatement = Rc<DoWhileStatementStruct>;

#[derive(Debug, Serialize)]
pub struct DoWhileStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub body: Statement,
    pub condition: Expression,
}

pub type ContinueStatement = Rc<ContinueStatementStruct>;

#[derive(Debug, Serialize)]
pub struct ContinueStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
}

pub type BreakStatement = Rc<BreakStatementStruct>;

#[derive(Debug, Serialize)]
pub struct BreakStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
}

pub type ReturnStatement = Rc<ReturnStatementStruct>;

#[derive(Debug, Serialize)]
pub struct ReturnStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub expression: Option<Expression>,
}

pub type EmitStatement = Rc<EmitStatementStruct>;

#[derive(Debug, Serialize)]
pub struct EmitStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub event: IdentifierPath,
    pub arguments: ArgumentsDeclaration,
}

pub type TryStatement = Rc<TryStatementStruct>;

#[derive(Debug, Serialize)]
pub struct TryStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub expression: Expression,
    pub returns: Option<Parameters>,
    pub body: Block,
    pub catch_clauses: CatchClauses,
}

pub type CatchClause = Rc<CatchClauseStruct>;

#[derive(Debug, Serialize)]
pub struct CatchClauseStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub error: Option<CatchClauseError>,
    pub body: Block,
}

pub type CatchClauseError = Rc<CatchClauseErrorStruct>;

#[derive(Debug, Serialize)]
pub struct CatchClauseErrorStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Option<Rc<TerminalNode>>,
    pub parameters: Parameters,
}

pub type RevertStatement = Rc<RevertStatementStruct>;

#[derive(Debug, Serialize)]
pub struct RevertStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub error: IdentifierPath,
    pub arguments: ArgumentsDeclaration,
}

pub type ThrowStatement = Rc<ThrowStatementStruct>;

#[derive(Debug, Serialize)]
pub struct ThrowStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
}

pub type AssignmentExpression = Rc<AssignmentExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct AssignmentExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type ConditionalExpression = Rc<ConditionalExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct ConditionalExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub operand: Expression,
    pub true_expression: Expression,
    pub false_expression: Expression,
}

pub type OrExpression = Rc<OrExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct OrExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

pub type AndExpression = Rc<AndExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct AndExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

pub type EqualityExpression = Rc<EqualityExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct EqualityExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type InequalityExpression = Rc<InequalityExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct InequalityExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type BitwiseOrExpression = Rc<BitwiseOrExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct BitwiseOrExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

pub type BitwiseXorExpression = Rc<BitwiseXorExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct BitwiseXorExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

pub type BitwiseAndExpression = Rc<BitwiseAndExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct BitwiseAndExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

pub type ShiftExpression = Rc<ShiftExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct ShiftExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type AdditiveExpression = Rc<AdditiveExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct AdditiveExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type MultiplicativeExpression = Rc<MultiplicativeExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct MultiplicativeExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type ExponentiationExpression = Rc<ExponentiationExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct ExponentiationExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type PostfixExpression = Rc<PostfixExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct PostfixExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub operand: Expression,
    pub operator: Rc<TerminalNode>,
}

pub type PrefixExpression = Rc<PrefixExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct PrefixExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub operator: Rc<TerminalNode>,
    pub operand: Expression,
}

pub type FunctionCallExpression = Rc<FunctionCallExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct FunctionCallExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub operand: Expression,
    pub arguments: ArgumentsDeclaration,
}

pub type CallOptionsExpression = Rc<CallOptionsExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct CallOptionsExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub operand: Expression,
    pub options: CallOptions,
}

pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct MemberAccessExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub operand: Expression,
    pub member: Rc<TerminalNode>,
}

pub type IndexAccessExpression = Rc<IndexAccessExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct IndexAccessExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub operand: Expression,
    pub start: Option<Expression>,
    pub end: Option<Expression>,
}

pub type NamedArgument = Rc<NamedArgumentStruct>;

#[derive(Debug, Serialize)]
pub struct NamedArgumentStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub value: Expression,
}

pub type TypeExpression = Rc<TypeExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct TypeExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: TypeName,
}

pub type NewExpression = Rc<NewExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct NewExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: TypeName,
}

pub type TupleExpression = Rc<TupleExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct TupleExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub items: TupleValues,
}

pub type TupleValue = Rc<TupleValueStruct>;

#[derive(Debug, Serialize)]
pub struct TupleValueStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub expression: Option<Expression>,
}

pub type ArrayExpression = Rc<ArrayExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct ArrayExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub items: ArrayValues,
}

pub type HexNumberExpression = Rc<HexNumberExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct HexNumberExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub literal: Rc<TerminalNode>,
    pub unit: Option<NumberUnit>,
}

pub type DecimalNumberExpression = Rc<DecimalNumberExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct DecimalNumberExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub literal: Rc<TerminalNode>,
    pub unit: Option<NumberUnit>,
}

pub type YulBlock = Rc<YulBlockStruct>;

#[derive(Debug, Serialize)]
pub struct YulBlockStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub statements: YulStatements,
}

pub type YulFunctionDefinition = Rc<YulFunctionDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct YulFunctionDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub parameters: YulParameters,
    pub returns: Option<YulVariableNames>,
    pub body: YulBlock,
}

pub type YulVariableDeclarationStatement = Rc<YulVariableDeclarationStatementStruct>;

#[derive(Debug, Serialize)]
pub struct YulVariableDeclarationStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub variables: YulVariableNames,
    pub value: Option<YulVariableDeclarationValue>,
}

pub type YulVariableDeclarationValue = Rc<YulVariableDeclarationValueStruct>;

#[derive(Debug, Serialize)]
pub struct YulVariableDeclarationValueStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

pub type YulVariableAssignmentStatement = Rc<YulVariableAssignmentStatementStruct>;

#[derive(Debug, Serialize)]
pub struct YulVariableAssignmentStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub variables: YulPaths,
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

pub type YulColonAndEqual = Rc<YulColonAndEqualStruct>;

#[derive(Debug, Serialize)]
pub struct YulColonAndEqualStruct {
    #[serde(skip)]
    pub node_id: NodeId,
}

pub type YulStackAssignmentStatement = Rc<YulStackAssignmentStatementStruct>;

#[derive(Debug, Serialize)]
pub struct YulStackAssignmentStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub assignment: YulStackAssignmentOperator,
    pub variable: Rc<TerminalNode>,
}

pub type YulEqualAndColon = Rc<YulEqualAndColonStruct>;

#[derive(Debug, Serialize)]
pub struct YulEqualAndColonStruct {
    #[serde(skip)]
    pub node_id: NodeId,
}

pub type YulIfStatement = Rc<YulIfStatementStruct>;

#[derive(Debug, Serialize)]
pub struct YulIfStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub condition: YulExpression,
    pub body: YulBlock,
}

pub type YulForStatement = Rc<YulForStatementStruct>;

#[derive(Debug, Serialize)]
pub struct YulForStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub initialization: YulBlock,
    pub condition: YulExpression,
    pub iterator: YulBlock,
    pub body: YulBlock,
}

pub type YulSwitchStatement = Rc<YulSwitchStatementStruct>;

#[derive(Debug, Serialize)]
pub struct YulSwitchStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub expression: YulExpression,
    pub cases: YulSwitchCases,
}

pub type YulDefaultCase = Rc<YulDefaultCaseStruct>;

#[derive(Debug, Serialize)]
pub struct YulDefaultCaseStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub body: YulBlock,
}

pub type YulValueCase = Rc<YulValueCaseStruct>;

#[derive(Debug, Serialize)]
pub struct YulValueCaseStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub value: YulLiteral,
    pub body: YulBlock,
}

pub type YulLeaveStatement = Rc<YulLeaveStatementStruct>;

#[derive(Debug, Serialize)]
pub struct YulLeaveStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
}

pub type YulBreakStatement = Rc<YulBreakStatementStruct>;

#[derive(Debug, Serialize)]
pub struct YulBreakStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
}

pub type YulContinueStatement = Rc<YulContinueStatementStruct>;

#[derive(Debug, Serialize)]
pub struct YulContinueStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
}

pub type YulLabel = Rc<YulLabelStruct>;

#[derive(Debug, Serialize)]
pub struct YulLabelStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub label: Rc<TerminalNode>,
}

pub type YulFunctionCallExpression = Rc<YulFunctionCallExpressionStruct>;

#[derive(Debug, Serialize)]
pub struct YulFunctionCallExpressionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub operand: YulExpression,
    pub arguments: YulArguments,
}

//
// Choices:
//

#[derive(Clone, Debug, Serialize)]
pub enum SourceUnitMember {
    PragmaDirective(PragmaDirective),
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
    ImportClause(ImportClause),
}

#[derive(Clone, Debug, Serialize)]
pub enum Pragma {
    VersionPragma(VersionPragma),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
}

#[derive(Clone, Debug, Serialize)]
pub enum AbicoderVersion {
    AbicoderV1Keyword,
    AbicoderV2Keyword,
}

#[derive(Clone, Debug, Serialize)]
pub enum ExperimentalFeature {
    StringLiteral(Rc<TerminalNode>),
    ABIEncoderV2Keyword,
    SMTCheckerKeyword,
}

#[derive(Clone, Debug, Serialize)]
pub enum VersionExpression {
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
}

#[derive(Clone, Debug, Serialize)]
pub enum VersionOperator {
    Caret,
    Tilde,
    Equal,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
}

#[derive(Clone, Debug, Serialize)]
pub enum VersionLiteral {
    SimpleVersionLiteral(SimpleVersionLiteral),
    SingleQuotedVersionLiteral(Rc<TerminalNode>),
    DoubleQuotedVersionLiteral(Rc<TerminalNode>),
}

#[derive(Clone, Debug, Serialize)]
pub enum ImportClause {
    PathImport(PathImport),
    ImportDeconstruction(ImportDeconstruction),
}

#[derive(Clone, Debug, Serialize)]
pub enum UsingClause {
    IdentifierPath(IdentifierPath),
    UsingDeconstruction(UsingDeconstruction),
}

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Serialize)]
pub enum UsingTarget {
    TypeName(TypeName),
    Asterisk,
}

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Serialize)]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    FunctionType(FunctionType),
    MappingType(MappingType),
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

#[derive(Clone, Debug, Serialize)]
pub enum ElementaryType {
    AddressType(AddressType),
    BytesKeyword(Rc<TerminalNode>),
    IntKeyword(Rc<TerminalNode>),
    UintKeyword(Rc<TerminalNode>),
    FixedKeyword(Rc<TerminalNode>),
    UfixedKeyword(Rc<TerminalNode>),
    BoolKeyword,
    ByteKeyword,
    StringKeyword,
}

#[derive(Clone, Debug, Serialize)]
pub enum Statement {
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ContinueStatement(ContinueStatement),
    BreakStatement(BreakStatement),
    ReturnStatement(ReturnStatement),
    ThrowStatement(ThrowStatement),
    EmitStatement(EmitStatement),
    TryStatement(TryStatement),
    RevertStatement(RevertStatement),
    AssemblyStatement(AssemblyStatement),
    Block(Block),
    UncheckedBlock(UncheckedBlock),
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
}

#[derive(Clone, Debug, Serialize)]
pub enum StorageLocation {
    MemoryKeyword,
    StorageKeyword,
    CallDataKeyword,
}

#[derive(Clone, Debug, Serialize)]
pub enum ForStatementInitialization {
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[derive(Clone, Debug, Serialize)]
pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[derive(Clone, Debug, Serialize)]
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
    Identifier(Rc<TerminalNode>),
    PayableKeyword,
    ThisKeyword,
    SuperKeyword,
    TrueKeyword,
    FalseKeyword,
}

#[derive(Clone, Debug, Serialize)]
pub enum ArgumentsDeclaration {
    PositionalArguments(PositionalArguments),
    NamedArguments(NamedArguments),
}

#[derive(Clone, Debug, Serialize)]
pub enum NumberUnit {
    WeiKeyword,
    GweiKeyword,
    SzaboKeyword,
    FinneyKeyword,
    EtherKeyword,
    SecondsKeyword,
    MinutesKeyword,
    HoursKeyword,
    DaysKeyword,
    WeeksKeyword,
    YearsKeyword,
}

#[derive(Clone, Debug, Serialize)]
pub enum StringExpression {
    Strings(Strings),
    HexStrings(HexStrings),
    UnicodeStrings(UnicodeStrings),
}

#[derive(Clone, Debug, Serialize)]
pub enum YulStatement {
    YulBlock(YulBlock),
    YulFunctionDefinition(YulFunctionDefinition),
    YulStackAssignmentStatement(YulStackAssignmentStatement),
    YulIfStatement(YulIfStatement),
    YulForStatement(YulForStatement),
    YulSwitchStatement(YulSwitchStatement),
    YulLeaveStatement(YulLeaveStatement),
    YulBreakStatement(YulBreakStatement),
    YulContinueStatement(YulContinueStatement),
    YulVariableAssignmentStatement(YulVariableAssignmentStatement),
    YulLabel(YulLabel),
    YulVariableDeclarationStatement(YulVariableDeclarationStatement),
    YulExpression(YulExpression),
}

#[derive(Clone, Debug, Serialize)]
pub enum YulAssignmentOperator {
    YulColonAndEqual(YulColonAndEqual),
    ColonEqual,
}

#[derive(Clone, Debug, Serialize)]
pub enum YulStackAssignmentOperator {
    YulEqualAndColon(YulEqualAndColon),
    EqualColon,
}

#[derive(Clone, Debug, Serialize)]
pub enum YulSwitchCase {
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
}

#[derive(Clone, Debug, Serialize)]
pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

#[derive(Clone, Debug, Serialize)]
pub enum YulLiteral {
    YulDecimalLiteral(Rc<TerminalNode>),
    YulHexLiteral(Rc<TerminalNode>),
    StringLiteral(Rc<TerminalNode>),
    HexStringLiteral(Rc<TerminalNode>),
    YulTrueKeyword,
    YulFalseKeyword,
}

#[derive(Clone, Debug, Serialize)]
pub enum FunctionKind {
    Regular,
    Constructor,
    Unnamed,
    Fallback,
    Receive,
    Modifier,
}

#[derive(Clone, Debug, Serialize)]
pub enum FunctionVisibility {
    Public,
    Private,
    Internal,
    External,
}

#[derive(Clone, Debug, Serialize)]
pub enum FunctionMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

#[derive(Clone, Debug, Serialize)]
pub enum StateVariableVisibility {
    Public,
    Private,
    Internal,
}

#[derive(Clone, Debug, Serialize)]
pub enum StateVariableMutability {
    Mutable,
    Constant,
    Immutable,
    Transient,
}

#[derive(Clone, Debug, Serialize)]
pub enum TupleDeconstructionMember {
    VariableDeclarationStatement(VariableDeclarationStatement),
    Identifier(Rc<TerminalNode>),
    None,
}

//
// Repeated & Separated
//

pub type SourceUnitMembers = Vec<SourceUnitMember>;

pub type VersionExpressionSets = Vec<VersionExpressionSet>;

pub type VersionExpressionSet = Vec<VersionExpression>;

pub type SimpleVersionLiteral = Vec<Rc<TerminalNode>>;

pub type ImportDeconstructionSymbols = Vec<ImportDeconstructionSymbol>;

pub type UsingDeconstructionSymbols = Vec<UsingDeconstructionSymbol>;

pub type InheritanceTypes = Vec<InheritanceType>;

pub type ContractMembers = Vec<ContractMember>;

pub type InterfaceMembers = Vec<ContractMember>;

pub type LibraryMembers = Vec<ContractMember>;

pub type StructMembers = Vec<StructMember>;

pub type EnumMembers = Vec<Rc<TerminalNode>>;

pub type Parameters = Vec<Parameter>;

pub type OverridePaths = Vec<IdentifierPath>;

pub type Statements = Vec<Statement>;

pub type CatchClauses = Vec<CatchClause>;

pub type PositionalArguments = Vec<Expression>;

pub type NamedArguments = Vec<NamedArgument>;

pub type CallOptions = Vec<NamedArgument>;

pub type TupleValues = Vec<TupleValue>;

pub type ArrayValues = Vec<Expression>;

pub type IdentifierPath = Vec<Rc<TerminalNode>>;

pub type YulStatements = Vec<YulStatement>;

pub type YulParameters = Vec<Rc<TerminalNode>>;

pub type YulVariableNames = Vec<Rc<TerminalNode>>;

pub type YulSwitchCases = Vec<YulSwitchCase>;

pub type YulArguments = Vec<YulExpression>;

pub type YulPaths = Vec<YulPath>;

pub type YulPath = Vec<Rc<TerminalNode>>;

pub type ModifierInvocations = Vec<ModifierInvocation>;

pub type Strings = Vec<Rc<TerminalNode>>;

pub type HexStrings = Vec<Rc<TerminalNode>>;

pub type UnicodeStrings = Vec<Rc<TerminalNode>>;

pub type AssemblyFlags = Vec<Rc<TerminalNode>>;

pub type TupleDeconstructionMembers = Vec<TupleDeconstructionMember>;
