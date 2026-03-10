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

pub type ImportDirective = Rc<ImportDirectiveStruct>;

#[derive(Debug, Serialize)]
pub struct ImportDirectiveStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub clause: ImportClause,
}

pub type PathImport = Rc<PathImportStruct>;

#[derive(Debug, Serialize)]
pub struct PathImportStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub path: StringLiteral,
    pub alias: Option<ImportAlias>,
}

pub type NamedImport = Rc<NamedImportStruct>;

#[derive(Debug, Serialize)]
pub struct NamedImportStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub alias: ImportAlias,
    pub path: StringLiteral,
}

pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

#[derive(Debug, Serialize)]
pub struct ImportDeconstructionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub symbols: ImportDeconstructionSymbols,
    pub path: StringLiteral,
}

pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

#[derive(Debug, Serialize)]
pub struct ImportDeconstructionSymbolStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub alias: Option<ImportAlias>,
}

pub type ImportAlias = Rc<ImportAliasStruct>;

#[derive(Debug, Serialize)]
pub struct ImportAliasStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
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
    pub alias: Option<UsingAlias>,
}

pub type UsingAlias = Rc<UsingAliasStruct>;

#[derive(Debug, Serialize)]
pub struct UsingAliasStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub operator: UsingOperator,
}

pub type ContractDefinition = Rc<ContractDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct ContractDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub abstract_keyword: bool,
    pub name: Rc<TerminalNode>,
    pub specifiers: ContractSpecifiers,
    pub members: ContractMembers,
}

pub type InheritanceSpecifier = Rc<InheritanceSpecifierStruct>;

#[derive(Debug, Serialize)]
pub struct InheritanceSpecifierStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub types: InheritanceTypes,
}

pub type InheritanceType = Rc<InheritanceTypeStruct>;

#[derive(Debug, Serialize)]
pub struct InheritanceTypeStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

pub type StorageLayoutSpecifier = Rc<StorageLayoutSpecifierStruct>;

#[derive(Debug, Serialize)]
pub struct StorageLayoutSpecifierStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub expression: Expression,
}

pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct InterfaceDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub inheritance: Option<InheritanceSpecifier>,
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
    pub value: Expression,
}

pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct StateVariableDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub attributes: StateVariableAttributes,
    pub name: Rc<TerminalNode>,
    pub value: Option<StateVariableDefinitionValue>,
}

pub type StateVariableDefinitionValue = Rc<StateVariableDefinitionValueStruct>;

#[derive(Debug, Serialize)]
pub struct StateVariableDefinitionValueStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub value: Expression,
}

pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct FunctionDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: FunctionName,
    pub parameters: ParametersDeclaration,
    pub attributes: FunctionAttributes,
    pub returns: Option<ReturnsDeclaration>,
    pub body: FunctionBody,
}

pub type ParametersDeclaration = Rc<ParametersDeclarationStruct>;

#[derive(Debug, Serialize)]
pub struct ParametersDeclarationStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub parameters: Parameters,
}

pub type Parameter = Rc<ParameterStruct>;

#[derive(Debug, Serialize)]
pub struct ParameterStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Option<Rc<TerminalNode>>,
}

pub type OverrideSpecifier = Rc<OverrideSpecifierStruct>;

#[derive(Debug, Serialize)]
pub struct OverrideSpecifierStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub overridden: Option<OverridePathsDeclaration>,
}

pub type OverridePathsDeclaration = Rc<OverridePathsDeclarationStruct>;

#[derive(Debug, Serialize)]
pub struct OverridePathsDeclarationStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub paths: OverridePaths,
}

pub type ReturnsDeclaration = Rc<ReturnsDeclarationStruct>;

#[derive(Debug, Serialize)]
pub struct ReturnsDeclarationStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub variables: ParametersDeclaration,
}

pub type ConstructorDefinition = Rc<ConstructorDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct ConstructorDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub parameters: ParametersDeclaration,
    pub attributes: ConstructorAttributes,
    pub body: Block,
}

pub type UnnamedFunctionDefinition = Rc<UnnamedFunctionDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct UnnamedFunctionDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub parameters: ParametersDeclaration,
    pub attributes: UnnamedFunctionAttributes,
    pub body: FunctionBody,
}

pub type FallbackFunctionDefinition = Rc<FallbackFunctionDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct FallbackFunctionDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub parameters: ParametersDeclaration,
    pub attributes: FallbackFunctionAttributes,
    pub returns: Option<ReturnsDeclaration>,
    pub body: FunctionBody,
}

pub type ReceiveFunctionDefinition = Rc<ReceiveFunctionDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct ReceiveFunctionDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub parameters: ParametersDeclaration,
    pub attributes: ReceiveFunctionAttributes,
    pub body: FunctionBody,
}

pub type ModifierDefinition = Rc<ModifierDefinitionStruct>;

#[derive(Debug, Serialize)]
pub struct ModifierDefinitionStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub name: Rc<TerminalNode>,
    pub parameters: Option<ParametersDeclaration>,
    pub attributes: ModifierAttributes,
    pub body: FunctionBody,
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
    pub parameters: EventParametersDeclaration,
    pub anonymous_keyword: bool,
}

pub type EventParametersDeclaration = Rc<EventParametersDeclarationStruct>;

#[derive(Debug, Serialize)]
pub struct EventParametersDeclarationStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub parameters: EventParameters,
}

pub type EventParameter = Rc<EventParameterStruct>;

#[derive(Debug, Serialize)]
pub struct EventParameterStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub indexed_keyword: bool,
    pub name: Option<Rc<TerminalNode>>,
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
    pub members: ErrorParametersDeclaration,
}

pub type ErrorParametersDeclaration = Rc<ErrorParametersDeclarationStruct>;

#[derive(Debug, Serialize)]
pub struct ErrorParametersDeclarationStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub parameters: ErrorParameters,
}

pub type ErrorParameter = Rc<ErrorParameterStruct>;

#[derive(Debug, Serialize)]
pub struct ErrorParameterStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub name: Option<Rc<TerminalNode>>,
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
    pub parameters: ParametersDeclaration,
    pub attributes: FunctionTypeAttributes,
    pub returns: Option<ReturnsDeclaration>,
}

pub type MappingType = Rc<MappingTypeStruct>;

#[derive(Debug, Serialize)]
pub struct MappingTypeStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub key_type: MappingKey,
    pub value_type: MappingValue,
}

pub type MappingKey = Rc<MappingKeyStruct>;

#[derive(Debug, Serialize)]
pub struct MappingKeyStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub key_type: MappingKeyType,
    pub name: Option<Rc<TerminalNode>>,
}

pub type MappingValue = Rc<MappingValueStruct>;

#[derive(Debug, Serialize)]
pub struct MappingValueStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub name: Option<Rc<TerminalNode>>,
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
    pub label: Option<StringLiteral>,
    pub flags: Option<AssemblyFlagsDeclaration>,
    pub body: YulBlock,
}

pub type AssemblyFlagsDeclaration = Rc<AssemblyFlagsDeclarationStruct>;

#[derive(Debug, Serialize)]
pub struct AssemblyFlagsDeclarationStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub flags: AssemblyFlags,
}

pub type TupleDeconstructionStatement = Rc<TupleDeconstructionStatementStruct>;

#[derive(Debug, Serialize)]
pub struct TupleDeconstructionStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub var_keyword: bool,
    pub elements: TupleDeconstructionElements,
    pub expression: Expression,
}

pub type TupleDeconstructionElement = Rc<TupleDeconstructionElementStruct>;

#[derive(Debug, Serialize)]
pub struct TupleDeconstructionElementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub member: Option<TupleMember>,
}

pub type TypedTupleMember = Rc<TypedTupleMemberStruct>;

#[derive(Debug, Serialize)]
pub struct TypedTupleMemberStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
}

pub type UntypedTupleMember = Rc<UntypedTupleMemberStruct>;

#[derive(Debug, Serialize)]
pub struct UntypedTupleMemberStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
}

pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

#[derive(Debug, Serialize)]
pub struct VariableDeclarationStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub variable_type: VariableDeclarationType,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
    pub value: Option<VariableDeclarationValue>,
}

pub type VariableDeclarationValue = Rc<VariableDeclarationValueStruct>;

#[derive(Debug, Serialize)]
pub struct VariableDeclarationValueStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub expression: Expression,
}

pub type IfStatement = Rc<IfStatementStruct>;

#[derive(Debug, Serialize)]
pub struct IfStatementStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub condition: Expression,
    pub body: Statement,
    pub else_branch: Option<ElseBranch>,
}

pub type ElseBranch = Rc<ElseBranchStruct>;

#[derive(Debug, Serialize)]
pub struct ElseBranchStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub body: Statement,
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
    pub returns: Option<ReturnsDeclaration>,
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
    pub parameters: ParametersDeclaration,
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
    pub end: Option<IndexAccessEnd>,
}

pub type IndexAccessEnd = Rc<IndexAccessEndStruct>;

#[derive(Debug, Serialize)]
pub struct IndexAccessEndStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub end: Option<Expression>,
}

pub type PositionalArgumentsDeclaration = Rc<PositionalArgumentsDeclarationStruct>;

#[derive(Debug, Serialize)]
pub struct PositionalArgumentsDeclarationStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub arguments: PositionalArguments,
}

pub type NamedArgumentsDeclaration = Rc<NamedArgumentsDeclarationStruct>;

#[derive(Debug, Serialize)]
pub struct NamedArgumentsDeclarationStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub arguments: Option<NamedArgumentGroup>,
}

pub type NamedArgumentGroup = Rc<NamedArgumentGroupStruct>;

#[derive(Debug, Serialize)]
pub struct NamedArgumentGroupStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub arguments: NamedArguments,
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
    pub parameters: YulParametersDeclaration,
    pub returns: Option<YulReturnsDeclaration>,
    pub body: YulBlock,
}

pub type YulParametersDeclaration = Rc<YulParametersDeclarationStruct>;

#[derive(Debug, Serialize)]
pub struct YulParametersDeclarationStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub parameters: YulParameters,
}

pub type YulReturnsDeclaration = Rc<YulReturnsDeclarationStruct>;

#[derive(Debug, Serialize)]
pub struct YulReturnsDeclarationStruct {
    #[serde(skip)]
    pub node_id: NodeId,
    pub variables: YulVariableNames,
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
    StringLiteral(StringLiteral),
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
    NamedImport(NamedImport),
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
pub enum ContractSpecifier {
    InheritanceSpecifier(InheritanceSpecifier),
    StorageLayoutSpecifier(StorageLayoutSpecifier),
}

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Serialize)]
pub enum StateVariableAttribute {
    OverrideSpecifier(OverrideSpecifier),
    ConstantKeyword,
    InternalKeyword,
    PrivateKeyword,
    PublicKeyword,
    ImmutableKeyword,
    TransientKeyword,
}

#[derive(Clone, Debug, Serialize)]
pub enum FunctionName {
    Identifier(Rc<TerminalNode>),
    FallbackKeyword,
    ReceiveKeyword,
}

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Serialize)]
pub enum FunctionBody {
    Block(Block),
    Semicolon,
}

#[derive(Clone, Debug, Serialize)]
pub enum ConstructorAttribute {
    ModifierInvocation(ModifierInvocation),
    InternalKeyword,
    OverrideKeyword,
    PayableKeyword,
    PublicKeyword,
    VirtualKeyword,
}

#[derive(Clone, Debug, Serialize)]
pub enum UnnamedFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    ExternalKeyword,
    InternalKeyword,
    PayableKeyword,
    PrivateKeyword,
    PublicKeyword,
}

#[derive(Clone, Debug, Serialize)]
pub enum FallbackFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ExternalKeyword,
    PayableKeyword,
    PureKeyword,
    ViewKeyword,
    VirtualKeyword,
}

#[derive(Clone, Debug, Serialize)]
pub enum ReceiveFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ExternalKeyword,
    PayableKeyword,
    VirtualKeyword,
}

#[derive(Clone, Debug, Serialize)]
pub enum ModifierAttribute {
    OverrideSpecifier(OverrideSpecifier),
    VirtualKeyword,
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

#[derive(Clone, Debug, Serialize)]
pub enum MappingKeyType {
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
pub enum TupleMember {
    TypedTupleMember(TypedTupleMember),
    UntypedTupleMember(UntypedTupleMember),
}

#[derive(Clone, Debug, Serialize)]
pub enum VariableDeclarationType {
    TypeName(TypeName),
    VarKeyword,
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
    PositionalArgumentsDeclaration(PositionalArgumentsDeclaration),
    NamedArgumentsDeclaration(NamedArgumentsDeclaration),
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
    StringLiteral(StringLiteral),
    StringLiterals(StringLiterals),
    HexStringLiteral(HexStringLiteral),
    HexStringLiterals(HexStringLiterals),
    UnicodeStringLiterals(UnicodeStringLiterals),
}

#[derive(Clone, Debug, Serialize)]
pub enum StringLiteral {
    SingleQuotedStringLiteral(Rc<TerminalNode>),
    DoubleQuotedStringLiteral(Rc<TerminalNode>),
}

#[derive(Clone, Debug, Serialize)]
pub enum HexStringLiteral {
    SingleQuotedHexStringLiteral(Rc<TerminalNode>),
    DoubleQuotedHexStringLiteral(Rc<TerminalNode>),
}

#[derive(Clone, Debug, Serialize)]
pub enum UnicodeStringLiteral {
    SingleQuotedUnicodeStringLiteral(Rc<TerminalNode>),
    DoubleQuotedUnicodeStringLiteral(Rc<TerminalNode>),
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
