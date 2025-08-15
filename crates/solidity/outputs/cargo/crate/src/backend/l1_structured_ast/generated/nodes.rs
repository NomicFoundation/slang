// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;
use std::vec::Vec;

use metaslang_cst::nodes::NodeId;

use crate::cst::TerminalNode;

//
// Sequences:
//

pub type SourceUnit = Rc<SourceUnitStruct>;

#[derive(Debug)]
pub struct SourceUnitStruct {
    pub node_id: NodeId,
    pub members: SourceUnitMembers,
}

pub type PragmaDirective = Rc<PragmaDirectiveStruct>;

#[derive(Debug)]
pub struct PragmaDirectiveStruct {
    pub node_id: NodeId,
    pub pragma: Pragma,
}

pub type AbicoderPragma = Rc<AbicoderPragmaStruct>;

#[derive(Debug)]
pub struct AbicoderPragmaStruct {
    pub node_id: NodeId,
    pub version: AbicoderVersion,
}

pub type ExperimentalPragma = Rc<ExperimentalPragmaStruct>;

#[derive(Debug)]
pub struct ExperimentalPragmaStruct {
    pub node_id: NodeId,
    pub feature: ExperimentalFeature,
}

pub type VersionPragma = Rc<VersionPragmaStruct>;

#[derive(Debug)]
pub struct VersionPragmaStruct {
    pub node_id: NodeId,
    pub sets: VersionExpressionSets,
}

pub type VersionRange = Rc<VersionRangeStruct>;

#[derive(Debug)]
pub struct VersionRangeStruct {
    pub node_id: NodeId,
    pub start: VersionLiteral,
    pub end: VersionLiteral,
}

pub type VersionTerm = Rc<VersionTermStruct>;

#[derive(Debug)]
pub struct VersionTermStruct {
    pub node_id: NodeId,
    pub operator: Option<VersionOperator>,
    pub literal: VersionLiteral,
}

pub type ImportDirective = Rc<ImportDirectiveStruct>;

#[derive(Debug)]
pub struct ImportDirectiveStruct {
    pub node_id: NodeId,
    pub clause: ImportClause,
}

pub type PathImport = Rc<PathImportStruct>;

#[derive(Debug)]
pub struct PathImportStruct {
    pub node_id: NodeId,
    pub path: StringLiteral,
    pub alias: Option<ImportAlias>,
}

pub type NamedImport = Rc<NamedImportStruct>;

#[derive(Debug)]
pub struct NamedImportStruct {
    pub node_id: NodeId,
    pub alias: ImportAlias,
    pub path: StringLiteral,
}

pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

#[derive(Debug)]
pub struct ImportDeconstructionStruct {
    pub node_id: NodeId,
    pub symbols: ImportDeconstructionSymbols,
    pub path: StringLiteral,
}

pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

#[derive(Debug)]
pub struct ImportDeconstructionSymbolStruct {
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub alias: Option<ImportAlias>,
}

pub type ImportAlias = Rc<ImportAliasStruct>;

#[derive(Debug)]
pub struct ImportAliasStruct {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

pub type UsingDirective = Rc<UsingDirectiveStruct>;

#[derive(Debug)]
pub struct UsingDirectiveStruct {
    pub node_id: NodeId,
    pub clause: UsingClause,
    pub target: UsingTarget,
    pub global_keyword: Option<Rc<TerminalNode>>,
}

pub type UsingDeconstruction = Rc<UsingDeconstructionStruct>;

#[derive(Debug)]
pub struct UsingDeconstructionStruct {
    pub node_id: NodeId,
    pub symbols: UsingDeconstructionSymbols,
}

pub type UsingDeconstructionSymbol = Rc<UsingDeconstructionSymbolStruct>;

#[derive(Debug)]
pub struct UsingDeconstructionSymbolStruct {
    pub node_id: NodeId,
    pub name: IdentifierPath,
    pub alias: Option<UsingAlias>,
}

pub type UsingAlias = Rc<UsingAliasStruct>;

#[derive(Debug)]
pub struct UsingAliasStruct {
    pub node_id: NodeId,
    pub operator: UsingOperator,
}

pub type ContractDefinition = Rc<ContractDefinitionStruct>;

#[derive(Debug)]
pub struct ContractDefinitionStruct {
    pub node_id: NodeId,
    pub abstract_keyword: Option<Rc<TerminalNode>>,
    pub name: Rc<TerminalNode>,
    pub specifiers: ContractSpecifiers,
    pub members: ContractMembers,
}

pub type InheritanceSpecifier = Rc<InheritanceSpecifierStruct>;

#[derive(Debug)]
pub struct InheritanceSpecifierStruct {
    pub node_id: NodeId,
    pub types: InheritanceTypes,
}

pub type InheritanceType = Rc<InheritanceTypeStruct>;

#[derive(Debug)]
pub struct InheritanceTypeStruct {
    pub node_id: NodeId,
    pub type_name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

pub type StorageLayoutSpecifier = Rc<StorageLayoutSpecifierStruct>;

#[derive(Debug)]
pub struct StorageLayoutSpecifierStruct {
    pub node_id: NodeId,
    pub expression: Expression,
}

pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

#[derive(Debug)]
pub struct InterfaceDefinitionStruct {
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub inheritance: Option<InheritanceSpecifier>,
    pub members: InterfaceMembers,
}

pub type LibraryDefinition = Rc<LibraryDefinitionStruct>;

#[derive(Debug)]
pub struct LibraryDefinitionStruct {
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub members: LibraryMembers,
}

pub type StructDefinition = Rc<StructDefinitionStruct>;

#[derive(Debug)]
pub struct StructDefinitionStruct {
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub members: StructMembers,
}

pub type StructMember = Rc<StructMemberStruct>;

#[derive(Debug)]
pub struct StructMemberStruct {
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub name: Rc<TerminalNode>,
}

pub type EnumDefinition = Rc<EnumDefinitionStruct>;

#[derive(Debug)]
pub struct EnumDefinitionStruct {
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub members: EnumMembers,
}

pub type ConstantDefinition = Rc<ConstantDefinitionStruct>;

#[derive(Debug)]
pub struct ConstantDefinitionStruct {
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub name: Rc<TerminalNode>,
    pub value: Expression,
}

pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

#[derive(Debug)]
pub struct StateVariableDefinitionStruct {
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub attributes: StateVariableAttributes,
    pub name: Rc<TerminalNode>,
    pub value: Option<StateVariableDefinitionValue>,
}

pub type StateVariableDefinitionValue = Rc<StateVariableDefinitionValueStruct>;

#[derive(Debug)]
pub struct StateVariableDefinitionValueStruct {
    pub node_id: NodeId,
    pub value: Expression,
}

pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

#[derive(Debug)]
pub struct FunctionDefinitionStruct {
    pub node_id: NodeId,
    pub name: FunctionName,
    pub parameters: ParametersDeclaration,
    pub attributes: FunctionAttributes,
    pub returns: Option<ReturnsDeclaration>,
    pub body: FunctionBody,
}

pub type ParametersDeclaration = Rc<ParametersDeclarationStruct>;

#[derive(Debug)]
pub struct ParametersDeclarationStruct {
    pub node_id: NodeId,
    pub parameters: Parameters,
}

pub type Parameter = Rc<ParameterStruct>;

#[derive(Debug)]
pub struct ParameterStruct {
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Option<Rc<TerminalNode>>,
}

pub type OverrideSpecifier = Rc<OverrideSpecifierStruct>;

#[derive(Debug)]
pub struct OverrideSpecifierStruct {
    pub node_id: NodeId,
    pub overridden: Option<OverridePathsDeclaration>,
}

pub type OverridePathsDeclaration = Rc<OverridePathsDeclarationStruct>;

#[derive(Debug)]
pub struct OverridePathsDeclarationStruct {
    pub node_id: NodeId,
    pub paths: OverridePaths,
}

pub type ReturnsDeclaration = Rc<ReturnsDeclarationStruct>;

#[derive(Debug)]
pub struct ReturnsDeclarationStruct {
    pub node_id: NodeId,
    pub variables: ParametersDeclaration,
}

pub type ConstructorDefinition = Rc<ConstructorDefinitionStruct>;

#[derive(Debug)]
pub struct ConstructorDefinitionStruct {
    pub node_id: NodeId,
    pub parameters: ParametersDeclaration,
    pub attributes: ConstructorAttributes,
    pub body: Block,
}

pub type UnnamedFunctionDefinition = Rc<UnnamedFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct UnnamedFunctionDefinitionStruct {
    pub node_id: NodeId,
    pub parameters: ParametersDeclaration,
    pub attributes: UnnamedFunctionAttributes,
    pub body: FunctionBody,
}

pub type FallbackFunctionDefinition = Rc<FallbackFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct FallbackFunctionDefinitionStruct {
    pub node_id: NodeId,
    pub parameters: ParametersDeclaration,
    pub attributes: FallbackFunctionAttributes,
    pub returns: Option<ReturnsDeclaration>,
    pub body: FunctionBody,
}

pub type ReceiveFunctionDefinition = Rc<ReceiveFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct ReceiveFunctionDefinitionStruct {
    pub node_id: NodeId,
    pub parameters: ParametersDeclaration,
    pub attributes: ReceiveFunctionAttributes,
    pub body: FunctionBody,
}

pub type ModifierDefinition = Rc<ModifierDefinitionStruct>;

#[derive(Debug)]
pub struct ModifierDefinitionStruct {
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub parameters: Option<ParametersDeclaration>,
    pub attributes: ModifierAttributes,
    pub body: FunctionBody,
}

pub type ModifierInvocation = Rc<ModifierInvocationStruct>;

#[derive(Debug)]
pub struct ModifierInvocationStruct {
    pub node_id: NodeId,
    pub name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

pub type EventDefinition = Rc<EventDefinitionStruct>;

#[derive(Debug)]
pub struct EventDefinitionStruct {
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub parameters: EventParametersDeclaration,
    pub anonymous_keyword: Option<Rc<TerminalNode>>,
}

pub type EventParametersDeclaration = Rc<EventParametersDeclarationStruct>;

#[derive(Debug)]
pub struct EventParametersDeclarationStruct {
    pub node_id: NodeId,
    pub parameters: EventParameters,
}

pub type EventParameter = Rc<EventParameterStruct>;

#[derive(Debug)]
pub struct EventParameterStruct {
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub indexed_keyword: Option<Rc<TerminalNode>>,
    pub name: Option<Rc<TerminalNode>>,
}

pub type UserDefinedValueTypeDefinition = Rc<UserDefinedValueTypeDefinitionStruct>;

#[derive(Debug)]
pub struct UserDefinedValueTypeDefinitionStruct {
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub value_type: ElementaryType,
}

pub type ErrorDefinition = Rc<ErrorDefinitionStruct>;

#[derive(Debug)]
pub struct ErrorDefinitionStruct {
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub members: ErrorParametersDeclaration,
}

pub type ErrorParametersDeclaration = Rc<ErrorParametersDeclarationStruct>;

#[derive(Debug)]
pub struct ErrorParametersDeclarationStruct {
    pub node_id: NodeId,
    pub parameters: ErrorParameters,
}

pub type ErrorParameter = Rc<ErrorParameterStruct>;

#[derive(Debug)]
pub struct ErrorParameterStruct {
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub name: Option<Rc<TerminalNode>>,
}

pub type ArrayTypeName = Rc<ArrayTypeNameStruct>;

#[derive(Debug)]
pub struct ArrayTypeNameStruct {
    pub node_id: NodeId,
    pub operand: TypeName,
    pub index: Option<Expression>,
}

pub type FunctionType = Rc<FunctionTypeStruct>;

#[derive(Debug)]
pub struct FunctionTypeStruct {
    pub node_id: NodeId,
    pub parameters: ParametersDeclaration,
    pub attributes: FunctionTypeAttributes,
    pub returns: Option<ReturnsDeclaration>,
}

pub type MappingType = Rc<MappingTypeStruct>;

#[derive(Debug)]
pub struct MappingTypeStruct {
    pub node_id: NodeId,
    pub key_type: MappingKey,
    pub value_type: MappingValue,
}

pub type MappingKey = Rc<MappingKeyStruct>;

#[derive(Debug)]
pub struct MappingKeyStruct {
    pub node_id: NodeId,
    pub key_type: MappingKeyType,
    pub name: Option<Rc<TerminalNode>>,
}

pub type MappingValue = Rc<MappingValueStruct>;

#[derive(Debug)]
pub struct MappingValueStruct {
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub name: Option<Rc<TerminalNode>>,
}

pub type AddressType = Rc<AddressTypeStruct>;

#[derive(Debug)]
pub struct AddressTypeStruct {
    pub node_id: NodeId,
    pub payable_keyword: Option<Rc<TerminalNode>>,
}

pub type Block = Rc<BlockStruct>;

#[derive(Debug)]
pub struct BlockStruct {
    pub node_id: NodeId,
    pub statements: Statements,
}

pub type UncheckedBlock = Rc<UncheckedBlockStruct>;

#[derive(Debug)]
pub struct UncheckedBlockStruct {
    pub node_id: NodeId,
    pub block: Block,
}

pub type ExpressionStatement = Rc<ExpressionStatementStruct>;

#[derive(Debug)]
pub struct ExpressionStatementStruct {
    pub node_id: NodeId,
    pub expression: Expression,
}

pub type AssemblyStatement = Rc<AssemblyStatementStruct>;

#[derive(Debug)]
pub struct AssemblyStatementStruct {
    pub node_id: NodeId,
    pub label: Option<StringLiteral>,
    pub flags: Option<AssemblyFlagsDeclaration>,
    pub body: YulBlock,
}

pub type AssemblyFlagsDeclaration = Rc<AssemblyFlagsDeclarationStruct>;

#[derive(Debug)]
pub struct AssemblyFlagsDeclarationStruct {
    pub node_id: NodeId,
    pub flags: AssemblyFlags,
}

pub type TupleDeconstructionStatement = Rc<TupleDeconstructionStatementStruct>;

#[derive(Debug)]
pub struct TupleDeconstructionStatementStruct {
    pub node_id: NodeId,
    pub var_keyword: Option<Rc<TerminalNode>>,
    pub elements: TupleDeconstructionElements,
    pub expression: Expression,
}

pub type TupleDeconstructionElement = Rc<TupleDeconstructionElementStruct>;

#[derive(Debug)]
pub struct TupleDeconstructionElementStruct {
    pub node_id: NodeId,
    pub member: Option<TupleMember>,
}

pub type TypedTupleMember = Rc<TypedTupleMemberStruct>;

#[derive(Debug)]
pub struct TypedTupleMemberStruct {
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
}

pub type UntypedTupleMember = Rc<UntypedTupleMemberStruct>;

#[derive(Debug)]
pub struct UntypedTupleMemberStruct {
    pub node_id: NodeId,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
}

pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

#[derive(Debug)]
pub struct VariableDeclarationStatementStruct {
    pub node_id: NodeId,
    pub variable_type: VariableDeclarationType,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
    pub value: Option<VariableDeclarationValue>,
}

pub type VariableDeclarationValue = Rc<VariableDeclarationValueStruct>;

#[derive(Debug)]
pub struct VariableDeclarationValueStruct {
    pub node_id: NodeId,
    pub expression: Expression,
}

pub type IfStatement = Rc<IfStatementStruct>;

#[derive(Debug)]
pub struct IfStatementStruct {
    pub node_id: NodeId,
    pub condition: Expression,
    pub body: Statement,
    pub else_branch: Option<ElseBranch>,
}

pub type ElseBranch = Rc<ElseBranchStruct>;

#[derive(Debug)]
pub struct ElseBranchStruct {
    pub node_id: NodeId,
    pub body: Statement,
}

pub type ForStatement = Rc<ForStatementStruct>;

#[derive(Debug)]
pub struct ForStatementStruct {
    pub node_id: NodeId,
    pub initialization: ForStatementInitialization,
    pub condition: ForStatementCondition,
    pub iterator: Option<Expression>,
    pub body: Statement,
}

pub type WhileStatement = Rc<WhileStatementStruct>;

#[derive(Debug)]
pub struct WhileStatementStruct {
    pub node_id: NodeId,
    pub condition: Expression,
    pub body: Statement,
}

pub type DoWhileStatement = Rc<DoWhileStatementStruct>;

#[derive(Debug)]
pub struct DoWhileStatementStruct {
    pub node_id: NodeId,
    pub body: Statement,
    pub condition: Expression,
}

pub type ContinueStatement = Rc<ContinueStatementStruct>;

#[derive(Debug)]
pub struct ContinueStatementStruct {
    pub node_id: NodeId,
}

pub type BreakStatement = Rc<BreakStatementStruct>;

#[derive(Debug)]
pub struct BreakStatementStruct {
    pub node_id: NodeId,
}

pub type ReturnStatement = Rc<ReturnStatementStruct>;

#[derive(Debug)]
pub struct ReturnStatementStruct {
    pub node_id: NodeId,
    pub expression: Option<Expression>,
}

pub type EmitStatement = Rc<EmitStatementStruct>;

#[derive(Debug)]
pub struct EmitStatementStruct {
    pub node_id: NodeId,
    pub event: IdentifierPath,
    pub arguments: ArgumentsDeclaration,
}

pub type TryStatement = Rc<TryStatementStruct>;

#[derive(Debug)]
pub struct TryStatementStruct {
    pub node_id: NodeId,
    pub expression: Expression,
    pub returns: Option<ReturnsDeclaration>,
    pub body: Block,
    pub catch_clauses: CatchClauses,
}

pub type CatchClause = Rc<CatchClauseStruct>;

#[derive(Debug)]
pub struct CatchClauseStruct {
    pub node_id: NodeId,
    pub error: Option<CatchClauseError>,
    pub body: Block,
}

pub type CatchClauseError = Rc<CatchClauseErrorStruct>;

#[derive(Debug)]
pub struct CatchClauseErrorStruct {
    pub node_id: NodeId,
    pub name: Option<Rc<TerminalNode>>,
    pub parameters: ParametersDeclaration,
}

pub type RevertStatement = Rc<RevertStatementStruct>;

#[derive(Debug)]
pub struct RevertStatementStruct {
    pub node_id: NodeId,
    pub error: Option<IdentifierPath>,
    pub arguments: ArgumentsDeclaration,
}

pub type ThrowStatement = Rc<ThrowStatementStruct>;

#[derive(Debug)]
pub struct ThrowStatementStruct {
    pub node_id: NodeId,
}

pub type AssignmentExpression = Rc<AssignmentExpressionStruct>;

#[derive(Debug)]
pub struct AssignmentExpressionStruct {
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type ConditionalExpression = Rc<ConditionalExpressionStruct>;

#[derive(Debug)]
pub struct ConditionalExpressionStruct {
    pub node_id: NodeId,
    pub operand: Expression,
    pub true_expression: Expression,
    pub false_expression: Expression,
}

pub type OrExpression = Rc<OrExpressionStruct>;

#[derive(Debug)]
pub struct OrExpressionStruct {
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

pub type AndExpression = Rc<AndExpressionStruct>;

#[derive(Debug)]
pub struct AndExpressionStruct {
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

pub type EqualityExpression = Rc<EqualityExpressionStruct>;

#[derive(Debug)]
pub struct EqualityExpressionStruct {
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type InequalityExpression = Rc<InequalityExpressionStruct>;

#[derive(Debug)]
pub struct InequalityExpressionStruct {
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type BitwiseOrExpression = Rc<BitwiseOrExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseOrExpressionStruct {
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

pub type BitwiseXorExpression = Rc<BitwiseXorExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseXorExpressionStruct {
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

pub type BitwiseAndExpression = Rc<BitwiseAndExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseAndExpressionStruct {
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

pub type ShiftExpression = Rc<ShiftExpressionStruct>;

#[derive(Debug)]
pub struct ShiftExpressionStruct {
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type AdditiveExpression = Rc<AdditiveExpressionStruct>;

#[derive(Debug)]
pub struct AdditiveExpressionStruct {
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type MultiplicativeExpression = Rc<MultiplicativeExpressionStruct>;

#[derive(Debug)]
pub struct MultiplicativeExpressionStruct {
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type ExponentiationExpression = Rc<ExponentiationExpressionStruct>;

#[derive(Debug)]
pub struct ExponentiationExpressionStruct {
    pub node_id: NodeId,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

pub type PostfixExpression = Rc<PostfixExpressionStruct>;

#[derive(Debug)]
pub struct PostfixExpressionStruct {
    pub node_id: NodeId,
    pub operand: Expression,
    pub operator: Rc<TerminalNode>,
}

pub type PrefixExpression = Rc<PrefixExpressionStruct>;

#[derive(Debug)]
pub struct PrefixExpressionStruct {
    pub node_id: NodeId,
    pub operator: Rc<TerminalNode>,
    pub operand: Expression,
}

pub type FunctionCallExpression = Rc<FunctionCallExpressionStruct>;

#[derive(Debug)]
pub struct FunctionCallExpressionStruct {
    pub node_id: NodeId,
    pub operand: Expression,
    pub arguments: ArgumentsDeclaration,
}

pub type CallOptionsExpression = Rc<CallOptionsExpressionStruct>;

#[derive(Debug)]
pub struct CallOptionsExpressionStruct {
    pub node_id: NodeId,
    pub operand: Expression,
    pub options: CallOptions,
}

pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

#[derive(Debug)]
pub struct MemberAccessExpressionStruct {
    pub node_id: NodeId,
    pub operand: Expression,
    pub member: Rc<TerminalNode>,
}

pub type IndexAccessExpression = Rc<IndexAccessExpressionStruct>;

#[derive(Debug)]
pub struct IndexAccessExpressionStruct {
    pub node_id: NodeId,
    pub operand: Expression,
    pub start: Option<Expression>,
    pub end: Option<IndexAccessEnd>,
}

pub type IndexAccessEnd = Rc<IndexAccessEndStruct>;

#[derive(Debug)]
pub struct IndexAccessEndStruct {
    pub node_id: NodeId,
    pub end: Option<Expression>,
}

pub type PositionalArgumentsDeclaration = Rc<PositionalArgumentsDeclarationStruct>;

#[derive(Debug)]
pub struct PositionalArgumentsDeclarationStruct {
    pub node_id: NodeId,
    pub arguments: PositionalArguments,
}

pub type NamedArgumentsDeclaration = Rc<NamedArgumentsDeclarationStruct>;

#[derive(Debug)]
pub struct NamedArgumentsDeclarationStruct {
    pub node_id: NodeId,
    pub arguments: Option<NamedArgumentGroup>,
}

pub type NamedArgumentGroup = Rc<NamedArgumentGroupStruct>;

#[derive(Debug)]
pub struct NamedArgumentGroupStruct {
    pub node_id: NodeId,
    pub arguments: NamedArguments,
}

pub type NamedArgument = Rc<NamedArgumentStruct>;

#[derive(Debug)]
pub struct NamedArgumentStruct {
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub value: Expression,
}

pub type TypeExpression = Rc<TypeExpressionStruct>;

#[derive(Debug)]
pub struct TypeExpressionStruct {
    pub node_id: NodeId,
    pub type_name: TypeName,
}

pub type NewExpression = Rc<NewExpressionStruct>;

#[derive(Debug)]
pub struct NewExpressionStruct {
    pub node_id: NodeId,
    pub type_name: TypeName,
}

pub type TupleExpression = Rc<TupleExpressionStruct>;

#[derive(Debug)]
pub struct TupleExpressionStruct {
    pub node_id: NodeId,
    pub items: TupleValues,
}

pub type TupleValue = Rc<TupleValueStruct>;

#[derive(Debug)]
pub struct TupleValueStruct {
    pub node_id: NodeId,
    pub expression: Option<Expression>,
}

pub type ArrayExpression = Rc<ArrayExpressionStruct>;

#[derive(Debug)]
pub struct ArrayExpressionStruct {
    pub node_id: NodeId,
    pub items: ArrayValues,
}

pub type HexNumberExpression = Rc<HexNumberExpressionStruct>;

#[derive(Debug)]
pub struct HexNumberExpressionStruct {
    pub node_id: NodeId,
    pub literal: Rc<TerminalNode>,
    pub unit: Option<NumberUnit>,
}

pub type DecimalNumberExpression = Rc<DecimalNumberExpressionStruct>;

#[derive(Debug)]
pub struct DecimalNumberExpressionStruct {
    pub node_id: NodeId,
    pub literal: Rc<TerminalNode>,
    pub unit: Option<NumberUnit>,
}

pub type YulBlock = Rc<YulBlockStruct>;

#[derive(Debug)]
pub struct YulBlockStruct {
    pub node_id: NodeId,
    pub statements: YulStatements,
}

pub type YulFunctionDefinition = Rc<YulFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct YulFunctionDefinitionStruct {
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub parameters: YulParametersDeclaration,
    pub returns: Option<YulReturnsDeclaration>,
    pub body: YulBlock,
}

pub type YulParametersDeclaration = Rc<YulParametersDeclarationStruct>;

#[derive(Debug)]
pub struct YulParametersDeclarationStruct {
    pub node_id: NodeId,
    pub parameters: YulParameters,
}

pub type YulReturnsDeclaration = Rc<YulReturnsDeclarationStruct>;

#[derive(Debug)]
pub struct YulReturnsDeclarationStruct {
    pub node_id: NodeId,
    pub variables: YulVariableNames,
}

pub type YulVariableDeclarationStatement = Rc<YulVariableDeclarationStatementStruct>;

#[derive(Debug)]
pub struct YulVariableDeclarationStatementStruct {
    pub node_id: NodeId,
    pub variables: YulVariableNames,
    pub value: Option<YulVariableDeclarationValue>,
}

pub type YulVariableDeclarationValue = Rc<YulVariableDeclarationValueStruct>;

#[derive(Debug)]
pub struct YulVariableDeclarationValueStruct {
    pub node_id: NodeId,
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

pub type YulVariableAssignmentStatement = Rc<YulVariableAssignmentStatementStruct>;

#[derive(Debug)]
pub struct YulVariableAssignmentStatementStruct {
    pub node_id: NodeId,
    pub variables: YulPaths,
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

pub type YulColonAndEqual = Rc<YulColonAndEqualStruct>;

#[derive(Debug)]
pub struct YulColonAndEqualStruct {
    pub node_id: NodeId,
}

pub type YulStackAssignmentStatement = Rc<YulStackAssignmentStatementStruct>;

#[derive(Debug)]
pub struct YulStackAssignmentStatementStruct {
    pub node_id: NodeId,
    pub assignment: YulStackAssignmentOperator,
    pub variable: Rc<TerminalNode>,
}

pub type YulEqualAndColon = Rc<YulEqualAndColonStruct>;

#[derive(Debug)]
pub struct YulEqualAndColonStruct {
    pub node_id: NodeId,
}

pub type YulIfStatement = Rc<YulIfStatementStruct>;

#[derive(Debug)]
pub struct YulIfStatementStruct {
    pub node_id: NodeId,
    pub condition: YulExpression,
    pub body: YulBlock,
}

pub type YulForStatement = Rc<YulForStatementStruct>;

#[derive(Debug)]
pub struct YulForStatementStruct {
    pub node_id: NodeId,
    pub initialization: YulBlock,
    pub condition: YulExpression,
    pub iterator: YulBlock,
    pub body: YulBlock,
}

pub type YulSwitchStatement = Rc<YulSwitchStatementStruct>;

#[derive(Debug)]
pub struct YulSwitchStatementStruct {
    pub node_id: NodeId,
    pub expression: YulExpression,
    pub cases: YulSwitchCases,
}

pub type YulDefaultCase = Rc<YulDefaultCaseStruct>;

#[derive(Debug)]
pub struct YulDefaultCaseStruct {
    pub node_id: NodeId,
    pub body: YulBlock,
}

pub type YulValueCase = Rc<YulValueCaseStruct>;

#[derive(Debug)]
pub struct YulValueCaseStruct {
    pub node_id: NodeId,
    pub value: YulLiteral,
    pub body: YulBlock,
}

pub type YulLeaveStatement = Rc<YulLeaveStatementStruct>;

#[derive(Debug)]
pub struct YulLeaveStatementStruct {
    pub node_id: NodeId,
}

pub type YulBreakStatement = Rc<YulBreakStatementStruct>;

#[derive(Debug)]
pub struct YulBreakStatementStruct {
    pub node_id: NodeId,
}

pub type YulContinueStatement = Rc<YulContinueStatementStruct>;

#[derive(Debug)]
pub struct YulContinueStatementStruct {
    pub node_id: NodeId,
}

pub type YulLabel = Rc<YulLabelStruct>;

#[derive(Debug)]
pub struct YulLabelStruct {
    pub node_id: NodeId,
    pub label: Rc<TerminalNode>,
}

pub type YulFunctionCallExpression = Rc<YulFunctionCallExpressionStruct>;

#[derive(Debug)]
pub struct YulFunctionCallExpressionStruct {
    pub node_id: NodeId,
    pub operand: YulExpression,
    pub arguments: YulArguments,
}

//
// Choices:
//

#[derive(Debug)]
pub enum SourceUnitMember {
    PragmaDirective(PragmaDirective),
    ImportDirective(ImportDirective),
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

#[derive(Debug)]
pub enum Pragma {
    VersionPragma(VersionPragma),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
}

#[derive(Debug)]
pub enum AbicoderVersion {
    AbicoderV1Keyword,
    AbicoderV2Keyword,
}

#[derive(Debug)]
pub enum ExperimentalFeature {
    StringLiteral(StringLiteral),
    AbiencoderV2Keyword,
    SmtcheckerKeyword,
}

#[derive(Debug)]
pub enum VersionExpression {
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
}

#[derive(Debug)]
pub enum VersionOperator {
    Caret,
    Tilde,
    Equal,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
}

#[derive(Debug)]
pub enum VersionLiteral {
    SimpleVersionLiteral(SimpleVersionLiteral),
    SingleQuotedVersionLiteral(Rc<TerminalNode>),
    DoubleQuotedVersionLiteral(Rc<TerminalNode>),
}

#[derive(Debug)]
pub enum ImportClause {
    PathImport(PathImport),
    NamedImport(NamedImport),
    ImportDeconstruction(ImportDeconstruction),
}

#[derive(Debug)]
pub enum UsingClause {
    IdentifierPath(IdentifierPath),
    UsingDeconstruction(UsingDeconstruction),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum UsingTarget {
    TypeName(TypeName),
    Asterisk,
}

#[derive(Debug)]
pub enum ContractSpecifier {
    InheritanceSpecifier(InheritanceSpecifier),
    StorageLayoutSpecifier(StorageLayoutSpecifier),
}

#[derive(Debug)]
pub enum ContractMember {
    UsingDirective(UsingDirective),
    FunctionDefinition(FunctionDefinition),
    ConstructorDefinition(ConstructorDefinition),
    ReceiveFunctionDefinition(ReceiveFunctionDefinition),
    FallbackFunctionDefinition(FallbackFunctionDefinition),
    UnnamedFunctionDefinition(UnnamedFunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    StateVariableDefinition(StateVariableDefinition),
}

#[derive(Debug)]
pub enum StateVariableAttribute {
    OverrideSpecifier(OverrideSpecifier),
    ConstantKeyword,
    InternalKeyword,
    PrivateKeyword,
    PublicKeyword,
    ImmutableKeyword,
    TransientKeyword,
}

#[derive(Debug)]
pub enum FunctionName {
    Identifier(Rc<TerminalNode>),
    FallbackKeyword,
    ReceiveKeyword,
}

#[derive(Debug)]
pub enum FunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ConstantKeyword,
    ExternalKeyword,
    InternalKeyword,
    PayableKeyword,
    PrivateKeyword,
    PublicKeyword,
    PureKeyword,
    ViewKeyword,
    VirtualKeyword,
}

#[derive(Debug)]
pub enum FunctionBody {
    Block(Block),
    Semicolon,
}

#[derive(Debug)]
pub enum ConstructorAttribute {
    ModifierInvocation(ModifierInvocation),
    InternalKeyword,
    OverrideKeyword,
    PayableKeyword,
    PublicKeyword,
    VirtualKeyword,
}

#[derive(Debug)]
pub enum UnnamedFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    ConstantKeyword,
    ExternalKeyword,
    InternalKeyword,
    PayableKeyword,
    PrivateKeyword,
    PublicKeyword,
    PureKeyword,
    ViewKeyword,
}

#[derive(Debug)]
pub enum FallbackFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ExternalKeyword,
    PayableKeyword,
    PureKeyword,
    ViewKeyword,
    VirtualKeyword,
}

#[derive(Debug)]
pub enum ReceiveFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ExternalKeyword,
    PayableKeyword,
    VirtualKeyword,
}

#[derive(Debug)]
pub enum ModifierAttribute {
    OverrideSpecifier(OverrideSpecifier),
    VirtualKeyword,
}

#[derive(Debug)]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    FunctionType(FunctionType),
    MappingType(MappingType),
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug)]
pub enum FunctionTypeAttribute {
    InternalKeyword,
    ExternalKeyword,
    PrivateKeyword,
    PublicKeyword,
    ConstantKeyword,
    PureKeyword,
    ViewKeyword,
    PayableKeyword,
}

#[derive(Debug)]
pub enum MappingKeyType {
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum TupleMember {
    TypedTupleMember(TypedTupleMember),
    UntypedTupleMember(UntypedTupleMember),
}

#[derive(Debug)]
pub enum VariableDeclarationType {
    TypeName(TypeName),
    VarKeyword,
}

#[derive(Debug)]
pub enum StorageLocation {
    MemoryKeyword,
    StorageKeyword,
    CallDataKeyword,
}

#[derive(Debug)]
pub enum ForStatementInitialization {
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[derive(Debug)]
pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ArgumentsDeclaration {
    PositionalArgumentsDeclaration(PositionalArgumentsDeclaration),
    NamedArgumentsDeclaration(NamedArgumentsDeclaration),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum StringExpression {
    StringLiteral(StringLiteral),
    StringLiterals(StringLiterals),
    HexStringLiteral(HexStringLiteral),
    HexStringLiterals(HexStringLiterals),
    UnicodeStringLiterals(UnicodeStringLiterals),
}

#[derive(Debug)]
pub enum StringLiteral {
    SingleQuotedStringLiteral(Rc<TerminalNode>),
    DoubleQuotedStringLiteral(Rc<TerminalNode>),
}

#[derive(Debug)]
pub enum HexStringLiteral {
    SingleQuotedHexStringLiteral(Rc<TerminalNode>),
    DoubleQuotedHexStringLiteral(Rc<TerminalNode>),
}

#[derive(Debug)]
pub enum UnicodeStringLiteral {
    SingleQuotedUnicodeStringLiteral(Rc<TerminalNode>),
    DoubleQuotedUnicodeStringLiteral(Rc<TerminalNode>),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum YulAssignmentOperator {
    YulColonAndEqual(YulColonAndEqual),
    ColonEqual,
}

#[derive(Debug)]
pub enum YulStackAssignmentOperator {
    YulEqualAndColon(YulEqualAndColon),
    EqualColon,
}

#[derive(Debug)]
pub enum YulSwitchCase {
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
}

#[derive(Debug)]
pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

#[derive(Debug)]
pub enum YulLiteral {
    HexStringLiteral(HexStringLiteral),
    StringLiteral(StringLiteral),
    YulDecimalLiteral(Rc<TerminalNode>),
    YulHexLiteral(Rc<TerminalNode>),
    YulTrueKeyword,
    YulFalseKeyword,
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

pub type ContractSpecifiers = Vec<ContractSpecifier>;

pub type InheritanceTypes = Vec<InheritanceType>;

pub type ContractMembers = Vec<ContractMember>;

pub type InterfaceMembers = Vec<ContractMember>;

pub type LibraryMembers = Vec<ContractMember>;

pub type StructMembers = Vec<StructMember>;

pub type EnumMembers = Vec<Rc<TerminalNode>>;

pub type StateVariableAttributes = Vec<StateVariableAttribute>;

pub type Parameters = Vec<Parameter>;

pub type FunctionAttributes = Vec<FunctionAttribute>;

pub type OverridePaths = Vec<IdentifierPath>;

pub type ConstructorAttributes = Vec<ConstructorAttribute>;

pub type UnnamedFunctionAttributes = Vec<UnnamedFunctionAttribute>;

pub type FallbackFunctionAttributes = Vec<FallbackFunctionAttribute>;

pub type ReceiveFunctionAttributes = Vec<ReceiveFunctionAttribute>;

pub type ModifierAttributes = Vec<ModifierAttribute>;

pub type EventParameters = Vec<EventParameter>;

pub type ErrorParameters = Vec<ErrorParameter>;

pub type FunctionTypeAttributes = Vec<FunctionTypeAttribute>;

pub type Statements = Vec<Statement>;

pub type AssemblyFlags = Vec<StringLiteral>;

pub type TupleDeconstructionElements = Vec<TupleDeconstructionElement>;

pub type CatchClauses = Vec<CatchClause>;

pub type PositionalArguments = Vec<Expression>;

pub type NamedArguments = Vec<NamedArgument>;

pub type CallOptions = Vec<NamedArgument>;

pub type TupleValues = Vec<TupleValue>;

pub type ArrayValues = Vec<Expression>;

pub type StringLiterals = Vec<StringLiteral>;

pub type HexStringLiterals = Vec<HexStringLiteral>;

pub type UnicodeStringLiterals = Vec<UnicodeStringLiteral>;

pub type IdentifierPath = Vec<Rc<TerminalNode>>;

pub type YulStatements = Vec<YulStatement>;

pub type YulParameters = Vec<Rc<TerminalNode>>;

pub type YulVariableNames = Vec<Rc<TerminalNode>>;

pub type YulSwitchCases = Vec<YulSwitchCase>;

pub type YulArguments = Vec<YulExpression>;

pub type YulPaths = Vec<YulPath>;

pub type YulPath = Vec<Rc<TerminalNode>>;
