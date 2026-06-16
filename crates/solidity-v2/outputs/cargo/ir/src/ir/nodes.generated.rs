// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::ops::Range;
use std::sync::Arc;
use std::vec::Vec;

pub(super) use slang_solidity_v2_common::nodes::NodeId;

//
// Sequences
//

pub type AbicoderPragma = Arc<AbicoderPragmaStruct>;

#[derive(Debug)]
pub struct AbicoderPragmaStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub version: AbicoderVersion,
}

impl AbicoderPragmaStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type AdditiveExpression = Arc<AdditiveExpressionStruct>;

#[derive(Debug)]
pub struct AdditiveExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub operator: AdditiveExpressionOperator,
    pub right_operand: Expression,
}

impl AdditiveExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type AddressType = Arc<AddressTypeStruct>;

#[derive(Debug)]
pub struct AddressTypeStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub is_payable: bool,
}

impl AddressTypeStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type AndExpression = Arc<AndExpressionStruct>;

#[derive(Debug)]
pub struct AndExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl AndExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ArrayExpression = Arc<ArrayExpressionStruct>;

#[derive(Debug)]
pub struct ArrayExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub items: ArrayValues,
}

impl ArrayExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ArrayTypeName = Arc<ArrayTypeNameStruct>;

#[derive(Debug)]
pub struct ArrayTypeNameStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub operand: TypeName,
    pub index: Option<Expression>,
}

impl ArrayTypeNameStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type AssemblyStatement = Arc<AssemblyStatementStruct>;

#[derive(Debug)]
pub struct AssemblyStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub label: Option<StringLiteral>,
    pub flags: Option<YulFlags>,
    pub body: YulBlock,
}

impl AssemblyStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type AssignmentExpression = Arc<AssignmentExpressionStruct>;

#[derive(Debug)]
pub struct AssignmentExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub operator: AssignmentExpressionOperator,
    pub right_operand: Expression,
}

impl AssignmentExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type BitwiseAndExpression = Arc<BitwiseAndExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseAndExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl BitwiseAndExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type BitwiseOrExpression = Arc<BitwiseOrExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseOrExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl BitwiseOrExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type BitwiseXorExpression = Arc<BitwiseXorExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseXorExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl BitwiseXorExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type Block = Arc<BlockStruct>;

#[derive(Debug)]
pub struct BlockStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub statements: Statements,
}

impl BlockStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type BreakStatement = Arc<BreakStatementStruct>;

#[derive(Debug)]
pub struct BreakStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl BreakStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type CallOptionsExpression = Arc<CallOptionsExpressionStruct>;

#[derive(Debug)]
pub struct CallOptionsExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub operand: Expression,
    pub options: CallOptions,
}

impl CallOptionsExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type CatchClause = Arc<CatchClauseStruct>;

#[derive(Debug)]
pub struct CatchClauseStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub error: Option<CatchClauseError>,
    pub body: Block,
}

impl CatchClauseStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type CatchClauseError = Arc<CatchClauseErrorStruct>;

#[derive(Debug)]
pub struct CatchClauseErrorStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: Option<Identifier>,
    pub parameters: Parameters,
}

impl CatchClauseErrorStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ConditionalExpression = Arc<ConditionalExpressionStruct>;

#[derive(Debug)]
pub struct ConditionalExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub operand: Expression,
    pub true_expression: Expression,
    pub false_expression: Expression,
}

impl ConditionalExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ConstantDefinition = Arc<ConstantDefinitionStruct>;

#[derive(Debug)]
pub struct ConstantDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub type_name: TypeName,
    pub name: Identifier,
    pub visibility: Option<StateVariableVisibility>,
    pub value: Option<Expression>,
}

impl ConstantDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ContinueStatement = Arc<ContinueStatementStruct>;

#[derive(Debug)]
pub struct ContinueStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl ContinueStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ContractDefinition = Arc<ContractDefinitionStruct>;

#[derive(Debug)]
pub struct ContractDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub is_abstract: bool,
    pub name: Identifier,
    pub inheritance_types: InheritanceTypes,
    pub storage_layout: Option<Expression>,
    pub members: ContractMembers,
}

impl ContractDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type DecimalNumberExpression = Arc<DecimalNumberExpressionStruct>;

#[derive(Debug)]
pub struct DecimalNumberExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub literal: DecimalLiteral,
    pub unit: Option<NumberUnit>,
}

impl DecimalNumberExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type DoWhileStatement = Arc<DoWhileStatementStruct>;

#[derive(Debug)]
pub struct DoWhileStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub body: Statement,
    pub condition: Expression,
}

impl DoWhileStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type EmitStatement = Arc<EmitStatementStruct>;

#[derive(Debug)]
pub struct EmitStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub event: IdentifierPath,
    pub arguments: ArgumentsDeclaration,
}

impl EmitStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type EnumDefinition = Arc<EnumDefinitionStruct>;

#[derive(Debug)]
pub struct EnumDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: Identifier,
    pub members: EnumMembers,
}

impl EnumDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type EqualityExpression = Arc<EqualityExpressionStruct>;

#[derive(Debug)]
pub struct EqualityExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub operator: EqualityExpressionOperator,
    pub right_operand: Expression,
}

impl EqualityExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ErrorDefinition = Arc<ErrorDefinitionStruct>;

#[derive(Debug)]
pub struct ErrorDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: Identifier,
    pub parameters: Parameters,
}

impl ErrorDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type EventDefinition = Arc<EventDefinitionStruct>;

#[derive(Debug)]
pub struct EventDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: Identifier,
    pub is_anonymous: bool,
    pub parameters: Parameters,
}

impl EventDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ExperimentalPragma = Arc<ExperimentalPragmaStruct>;

#[derive(Debug)]
pub struct ExperimentalPragmaStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub feature: ExperimentalFeature,
}

impl ExperimentalPragmaStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ExponentiationExpression = Arc<ExponentiationExpressionStruct>;

#[derive(Debug)]
pub struct ExponentiationExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl ExponentiationExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ExpressionStatement = Arc<ExpressionStatementStruct>;

#[derive(Debug)]
pub struct ExpressionStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub expression: Expression,
}

impl ExpressionStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ForStatement = Arc<ForStatementStruct>;

#[derive(Debug)]
pub struct ForStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub initialization: ForStatementInitialization,
    pub condition: ForStatementCondition,
    pub iterator: Option<Expression>,
    pub body: Statement,
}

impl ForStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type FunctionCallExpression = Arc<FunctionCallExpressionStruct>;

#[derive(Debug)]
pub struct FunctionCallExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub operand: Expression,
    pub arguments: ArgumentsDeclaration,
}

impl FunctionCallExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type FunctionDefinition = Arc<FunctionDefinitionStruct>;

#[derive(Debug)]
pub struct FunctionDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub kind: FunctionKind,
    pub name: Option<Identifier>,
    pub parameters: Parameters,
    pub visibility: FunctionVisibility,
    pub mutability: FunctionMutability,
    pub is_virtual: bool,
    pub override_specifier: Option<OverridePaths>,
    pub modifier_invocations: ModifierInvocations,
    pub returns: Option<Parameters>,
    pub body: Option<Block>,
}

impl FunctionDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type FunctionType = Arc<FunctionTypeStruct>;

#[derive(Debug)]
pub struct FunctionTypeStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub parameters: Parameters,
    pub visibility: FunctionVisibility,
    pub mutability: FunctionMutability,
    pub returns: Option<Parameters>,
}

impl FunctionTypeStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type HexNumberExpression = Arc<HexNumberExpressionStruct>;

#[derive(Debug)]
pub struct HexNumberExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub literal: HexLiteral,
}

impl HexNumberExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type IfStatement = Arc<IfStatementStruct>;

#[derive(Debug)]
pub struct IfStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub condition: Expression,
    pub body: Statement,
    pub else_branch: Option<Statement>,
}

impl IfStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ImportDeconstruction = Arc<ImportDeconstructionStruct>;

#[derive(Debug)]
pub struct ImportDeconstructionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub symbols: ImportDeconstructionSymbols,
    pub path: StringLiteral,
}

impl ImportDeconstructionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ImportDeconstructionSymbol = Arc<ImportDeconstructionSymbolStruct>;

#[derive(Debug)]
pub struct ImportDeconstructionSymbolStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: Identifier,
    pub alias: Option<Identifier>,
}

impl ImportDeconstructionSymbolStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type IndexAccessExpression = Arc<IndexAccessExpressionStruct>;

#[derive(Debug)]
pub struct IndexAccessExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub operand: Expression,
    pub start: Option<Expression>,
    pub end: Option<Expression>,
}

impl IndexAccessExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type InequalityExpression = Arc<InequalityExpressionStruct>;

#[derive(Debug)]
pub struct InequalityExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub operator: InequalityExpressionOperator,
    pub right_operand: Expression,
}

impl InequalityExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type InheritanceType = Arc<InheritanceTypeStruct>;

#[derive(Debug)]
pub struct InheritanceTypeStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub type_name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

impl InheritanceTypeStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type InterfaceDefinition = Arc<InterfaceDefinitionStruct>;

#[derive(Debug)]
pub struct InterfaceDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: Identifier,
    pub inheritance: Option<InheritanceTypes>,
    pub members: InterfaceMembers,
}

impl InterfaceDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type LibraryDefinition = Arc<LibraryDefinitionStruct>;

#[derive(Debug)]
pub struct LibraryDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: Identifier,
    pub members: LibraryMembers,
}

impl LibraryDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type MappingType = Arc<MappingTypeStruct>;

#[derive(Debug)]
pub struct MappingTypeStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub key_type: Parameter,
    pub value_type: Parameter,
}

impl MappingTypeStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type MemberAccessExpression = Arc<MemberAccessExpressionStruct>;

#[derive(Debug)]
pub struct MemberAccessExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub operand: Expression,
    pub member: Identifier,
}

impl MemberAccessExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ModifierInvocation = Arc<ModifierInvocationStruct>;

#[derive(Debug)]
pub struct ModifierInvocationStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

impl ModifierInvocationStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type MultiTypedDeclaration = Arc<MultiTypedDeclarationStruct>;

#[derive(Debug)]
pub struct MultiTypedDeclarationStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub elements: MultiTypedDeclarationElements,
    pub value: Expression,
}

impl MultiTypedDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type MultiTypedDeclarationElement = Arc<MultiTypedDeclarationElementStruct>;

#[derive(Debug)]
pub struct MultiTypedDeclarationElementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub member: Option<VariableDeclaration>,
}

impl MultiTypedDeclarationElementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type MultiplicativeExpression = Arc<MultiplicativeExpressionStruct>;

#[derive(Debug)]
pub struct MultiplicativeExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub operator: MultiplicativeExpressionOperator,
    pub right_operand: Expression,
}

impl MultiplicativeExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type NamedArgument = Arc<NamedArgumentStruct>;

#[derive(Debug)]
pub struct NamedArgumentStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: Identifier,
    pub value: Expression,
}

impl NamedArgumentStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type NewExpression = Arc<NewExpressionStruct>;

#[derive(Debug)]
pub struct NewExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub type_name: TypeName,
}

impl NewExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type OrExpression = Arc<OrExpressionStruct>;

#[derive(Debug)]
pub struct OrExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl OrExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type Parameter = Arc<ParameterStruct>;

#[derive(Debug)]
pub struct ParameterStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Option<Identifier>,
    pub is_indexed: bool,
}

impl ParameterStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type PathImport = Arc<PathImportStruct>;

#[derive(Debug)]
pub struct PathImportStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub path: StringLiteral,
    pub alias: Option<Identifier>,
}

impl PathImportStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type PostfixExpression = Arc<PostfixExpressionStruct>;

#[derive(Debug)]
pub struct PostfixExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub operand: Expression,
    pub operator: PostfixExpressionOperator,
}

impl PostfixExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type PragmaDirective = Arc<PragmaDirectiveStruct>;

#[derive(Debug)]
pub struct PragmaDirectiveStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub pragma: Pragma,
}

impl PragmaDirectiveStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type PrefixExpression = Arc<PrefixExpressionStruct>;

#[derive(Debug)]
pub struct PrefixExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub operator: PrefixExpressionOperator,
    pub operand: Expression,
}

impl PrefixExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ReturnStatement = Arc<ReturnStatementStruct>;

#[derive(Debug)]
pub struct ReturnStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub expression: Option<Expression>,
}

impl ReturnStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type RevertStatement = Arc<RevertStatementStruct>;

#[derive(Debug)]
pub struct RevertStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub error: IdentifierPath,
    pub arguments: ArgumentsDeclaration,
}

impl RevertStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ShiftExpression = Arc<ShiftExpressionStruct>;

#[derive(Debug)]
pub struct ShiftExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub operator: ShiftExpressionOperator,
    pub right_operand: Expression,
}

impl ShiftExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type SingleTypedDeclaration = Arc<SingleTypedDeclarationStruct>;

#[derive(Debug)]
pub struct SingleTypedDeclarationStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub declaration: VariableDeclaration,
    pub value: Option<Expression>,
}

impl SingleTypedDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type SourceUnit = Arc<SourceUnitStruct>;

#[derive(Debug)]
pub struct SourceUnitStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub members: SourceUnitMembers,
}

impl SourceUnitStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type StateVariableDefinition = Arc<StateVariableDefinitionStruct>;

#[derive(Debug)]
pub struct StateVariableDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub type_name: TypeName,
    pub name: Identifier,
    pub value: Option<Expression>,
    pub visibility: StateVariableVisibility,
    pub mutability: StateVariableMutability,
    pub override_specifier: Option<OverridePaths>,
}

impl StateVariableDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type StructDefinition = Arc<StructDefinitionStruct>;

#[derive(Debug)]
pub struct StructDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: Identifier,
    pub members: StructMembers,
}

impl StructDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type StructMember = Arc<StructMemberStruct>;

#[derive(Debug)]
pub struct StructMemberStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub type_name: TypeName,
    pub name: Identifier,
}

impl StructMemberStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type TryStatement = Arc<TryStatementStruct>;

#[derive(Debug)]
pub struct TryStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub expression: Expression,
    pub returns: Option<Parameters>,
    pub body: Block,
    pub catch_clauses: CatchClauses,
}

impl TryStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type TupleExpression = Arc<TupleExpressionStruct>;

#[derive(Debug)]
pub struct TupleExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub items: TupleValues,
}

impl TupleExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type TupleValue = Arc<TupleValueStruct>;

#[derive(Debug)]
pub struct TupleValueStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub expression: Option<Expression>,
}

impl TupleValueStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type TypeExpression = Arc<TypeExpressionStruct>;

#[derive(Debug)]
pub struct TypeExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub type_name: TypeName,
}

impl TypeExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type UncheckedBlock = Arc<UncheckedBlockStruct>;

#[derive(Debug)]
pub struct UncheckedBlockStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub block: Block,
}

impl UncheckedBlockStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type UserDefinedValueTypeDefinition = Arc<UserDefinedValueTypeDefinitionStruct>;

#[derive(Debug)]
pub struct UserDefinedValueTypeDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: Identifier,
    pub value_type: ElementaryType,
}

impl UserDefinedValueTypeDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type UsingDeconstruction = Arc<UsingDeconstructionStruct>;

#[derive(Debug)]
pub struct UsingDeconstructionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub symbols: UsingDeconstructionSymbols,
}

impl UsingDeconstructionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type UsingDeconstructionSymbol = Arc<UsingDeconstructionSymbolStruct>;

#[derive(Debug)]
pub struct UsingDeconstructionSymbolStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: IdentifierPath,
    pub alias: Option<UsingOperator>,
}

impl UsingDeconstructionSymbolStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type UsingDirective = Arc<UsingDirectiveStruct>;

#[derive(Debug)]
pub struct UsingDirectiveStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub clause: UsingClause,
    pub target: UsingTarget,
    pub is_global: bool,
}

impl UsingDirectiveStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type VariableDeclaration = Arc<VariableDeclarationStruct>;

#[derive(Debug)]
pub struct VariableDeclarationStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Identifier,
}

impl VariableDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type VariableDeclarationStatement = Arc<VariableDeclarationStatementStruct>;

#[derive(Debug)]
pub struct VariableDeclarationStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub target: VariableDeclarationTarget,
}

impl VariableDeclarationStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type VersionPragma = Arc<VersionPragmaStruct>;

#[derive(Debug)]
pub struct VersionPragmaStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub sets: VersionExpressionSets,
}

impl VersionPragmaStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type VersionRange = Arc<VersionRangeStruct>;

#[derive(Debug)]
pub struct VersionRangeStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub start: VersionLiteral,
    pub end: VersionLiteral,
}

impl VersionRangeStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type VersionTerm = Arc<VersionTermStruct>;

#[derive(Debug)]
pub struct VersionTermStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub operator: Option<VersionOperator>,
    pub literal: VersionLiteral,
}

impl VersionTermStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type WhileStatement = Arc<WhileStatementStruct>;

#[derive(Debug)]
pub struct WhileStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub condition: Expression,
    pub body: Statement,
}

impl WhileStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulBlock = Arc<YulBlockStruct>;

#[derive(Debug)]
pub struct YulBlockStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub statements: YulStatements,
}

impl YulBlockStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulBreakStatement = Arc<YulBreakStatementStruct>;

#[derive(Debug)]
pub struct YulBreakStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl YulBreakStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulContinueStatement = Arc<YulContinueStatementStruct>;

#[derive(Debug)]
pub struct YulContinueStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl YulContinueStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulDefaultCase = Arc<YulDefaultCaseStruct>;

#[derive(Debug)]
pub struct YulDefaultCaseStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub body: YulBlock,
}

impl YulDefaultCaseStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulForStatement = Arc<YulForStatementStruct>;

#[derive(Debug)]
pub struct YulForStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub initialization: YulBlock,
    pub condition: YulExpression,
    pub iterator: YulBlock,
    pub body: YulBlock,
}

impl YulForStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulFunctionCallExpression = Arc<YulFunctionCallExpressionStruct>;

#[derive(Debug)]
pub struct YulFunctionCallExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub operand: YulExpression,
    pub arguments: YulArguments,
}

impl YulFunctionCallExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulFunctionDefinition = Arc<YulFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct YulFunctionDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: Identifier,
    pub parameters: YulParameters,
    pub returns: Option<YulVariableNames>,
    pub body: YulBlock,
}

impl YulFunctionDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulIfStatement = Arc<YulIfStatementStruct>;

#[derive(Debug)]
pub struct YulIfStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub condition: YulExpression,
    pub body: YulBlock,
}

impl YulIfStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulLeaveStatement = Arc<YulLeaveStatementStruct>;

#[derive(Debug)]
pub struct YulLeaveStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl YulLeaveStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulSwitchStatement = Arc<YulSwitchStatementStruct>;

#[derive(Debug)]
pub struct YulSwitchStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub expression: YulExpression,
    pub cases: YulSwitchCases,
}

impl YulSwitchStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulValueCase = Arc<YulValueCaseStruct>;

#[derive(Debug)]
pub struct YulValueCaseStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub value: YulLiteral,
    pub body: YulBlock,
}

impl YulValueCaseStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulVariableAssignmentStatement = Arc<YulVariableAssignmentStatementStruct>;

#[derive(Debug)]
pub struct YulVariableAssignmentStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub variables: YulPaths,
    pub expression: YulExpression,
}

impl YulVariableAssignmentStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulVariableDeclarationStatement = Arc<YulVariableDeclarationStatementStruct>;

#[derive(Debug)]
pub struct YulVariableDeclarationStatementStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub variables: YulVariableNames,
    pub value: Option<YulVariableDeclarationValue>,
}

impl YulVariableDeclarationStatementStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type YulVariableDeclarationValue = Arc<YulVariableDeclarationValueStruct>;

#[derive(Debug)]
pub struct YulVariableDeclarationValueStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub expression: YulExpression,
}

impl YulVariableDeclarationValueStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

//
// Choices
//

#[derive(Clone, Debug)]
pub enum AbicoderVersion {
    AbicoderV1Keyword(AbicoderV1Keyword),
    AbicoderV2Keyword(AbicoderV2Keyword),
}

impl AbicoderVersion {
    pub fn unparse(&self) -> &str {
        match self {
            AbicoderVersion::AbicoderV1Keyword(inner) => inner.unparse(),
            AbicoderVersion::AbicoderV2Keyword(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AdditiveExpressionOperator {
    Minus(Minus),
    Plus(Plus),
}

impl AdditiveExpressionOperator {
    pub fn unparse(&self) -> &str {
        match self {
            AdditiveExpressionOperator::Minus(inner) => inner.unparse(),
            AdditiveExpressionOperator::Plus(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ArgumentsDeclaration {
    PositionalArguments(PositionalArguments),
    NamedArguments(NamedArguments),
}

#[derive(Clone, Debug)]
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

impl AssignmentExpressionOperator {
    pub fn unparse(&self) -> &str {
        match self {
            AssignmentExpressionOperator::AmpersandEqual(inner) => inner.unparse(),
            AssignmentExpressionOperator::AsteriskEqual(inner) => inner.unparse(),
            AssignmentExpressionOperator::BarEqual(inner) => inner.unparse(),
            AssignmentExpressionOperator::CaretEqual(inner) => inner.unparse(),
            AssignmentExpressionOperator::Equal(inner) => inner.unparse(),
            AssignmentExpressionOperator::GreaterThanGreaterThanEqual(inner) => inner.unparse(),
            AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(inner) => {
                inner.unparse()
            }
            AssignmentExpressionOperator::LessThanLessThanEqual(inner) => inner.unparse(),
            AssignmentExpressionOperator::MinusEqual(inner) => inner.unparse(),
            AssignmentExpressionOperator::PercentEqual(inner) => inner.unparse(),
            AssignmentExpressionOperator::PlusEqual(inner) => inner.unparse(),
            AssignmentExpressionOperator::SlashEqual(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum EqualityExpressionOperator {
    BangEqual(BangEqual),
    EqualEqual(EqualEqual),
}

impl EqualityExpressionOperator {
    pub fn unparse(&self) -> &str {
        match self {
            EqualityExpressionOperator::BangEqual(inner) => inner.unparse(),
            EqualityExpressionOperator::EqualEqual(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ExperimentalFeature {
    ABIEncoderV2Keyword(ABIEncoderV2Keyword),
    SMTCheckerKeyword(SMTCheckerKeyword),
    StringLiteral(StringLiteral),
}

impl ExperimentalFeature {
    pub fn unparse(&self) -> &str {
        match self {
            ExperimentalFeature::ABIEncoderV2Keyword(inner) => inner.unparse(),
            ExperimentalFeature::SMTCheckerKeyword(inner) => inner.unparse(),
            ExperimentalFeature::StringLiteral(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    Semicolon(Semicolon),
}

#[derive(Clone, Debug)]
pub enum ForStatementInitialization {
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Semicolon(Semicolon),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FunctionKind {
    Regular,
    Constructor,
    Fallback,
    Receive,
    Modifier,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FunctionMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FunctionVisibility {
    Public,
    Private,
    Internal,
    External,
}

#[derive(Clone, Debug)]
pub enum ImportClause {
    PathImport(PathImport),
    ImportDeconstruction(ImportDeconstruction),
}

#[derive(Clone, Debug)]
pub enum InequalityExpressionOperator {
    GreaterThan(GreaterThan),
    GreaterThanEqual(GreaterThanEqual),
    LessThan(LessThan),
    LessThanEqual(LessThanEqual),
}

impl InequalityExpressionOperator {
    pub fn unparse(&self) -> &str {
        match self {
            InequalityExpressionOperator::GreaterThan(inner) => inner.unparse(),
            InequalityExpressionOperator::GreaterThanEqual(inner) => inner.unparse(),
            InequalityExpressionOperator::LessThan(inner) => inner.unparse(),
            InequalityExpressionOperator::LessThanEqual(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum MultiplicativeExpressionOperator {
    Asterisk(Asterisk),
    Percent(Percent),
    Slash(Slash),
}

impl MultiplicativeExpressionOperator {
    pub fn unparse(&self) -> &str {
        match self {
            MultiplicativeExpressionOperator::Asterisk(inner) => inner.unparse(),
            MultiplicativeExpressionOperator::Percent(inner) => inner.unparse(),
            MultiplicativeExpressionOperator::Slash(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
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

impl NumberUnit {
    pub fn unparse(&self) -> &str {
        match self {
            NumberUnit::WeiKeyword(inner) => inner.unparse(),
            NumberUnit::GweiKeyword(inner) => inner.unparse(),
            NumberUnit::EtherKeyword(inner) => inner.unparse(),
            NumberUnit::SecondsKeyword(inner) => inner.unparse(),
            NumberUnit::MinutesKeyword(inner) => inner.unparse(),
            NumberUnit::HoursKeyword(inner) => inner.unparse(),
            NumberUnit::DaysKeyword(inner) => inner.unparse(),
            NumberUnit::WeeksKeyword(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum PostfixExpressionOperator {
    MinusMinus(MinusMinus),
    PlusPlus(PlusPlus),
}

impl PostfixExpressionOperator {
    pub fn unparse(&self) -> &str {
        match self {
            PostfixExpressionOperator::MinusMinus(inner) => inner.unparse(),
            PostfixExpressionOperator::PlusPlus(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Pragma {
    VersionPragma(VersionPragma),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
}

#[derive(Clone, Debug)]
pub enum PrefixExpressionOperator {
    Bang(Bang),
    DeleteKeyword(DeleteKeyword),
    Minus(Minus),
    MinusMinus(MinusMinus),
    PlusPlus(PlusPlus),
    Tilde(Tilde),
}

impl PrefixExpressionOperator {
    pub fn unparse(&self) -> &str {
        match self {
            PrefixExpressionOperator::Bang(inner) => inner.unparse(),
            PrefixExpressionOperator::DeleteKeyword(inner) => inner.unparse(),
            PrefixExpressionOperator::Minus(inner) => inner.unparse(),
            PrefixExpressionOperator::MinusMinus(inner) => inner.unparse(),
            PrefixExpressionOperator::PlusPlus(inner) => inner.unparse(),
            PrefixExpressionOperator::Tilde(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ShiftExpressionOperator {
    GreaterThanGreaterThan(GreaterThanGreaterThan),
    GreaterThanGreaterThanGreaterThan(GreaterThanGreaterThanGreaterThan),
    LessThanLessThan(LessThanLessThan),
}

impl ShiftExpressionOperator {
    pub fn unparse(&self) -> &str {
        match self {
            ShiftExpressionOperator::GreaterThanGreaterThan(inner) => inner.unparse(),
            ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(inner) => inner.unparse(),
            ShiftExpressionOperator::LessThanLessThan(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StateVariableMutability {
    Mutable,
    Constant,
    Immutable,
    Transient,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StateVariableVisibility {
    Public,
    Private,
    Internal,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum StorageLocation {
    MemoryKeyword(MemoryKeyword),
    StorageKeyword(StorageKeyword),
    CallDataKeyword(CallDataKeyword),
}

impl StorageLocation {
    pub fn unparse(&self) -> &str {
        match self {
            StorageLocation::MemoryKeyword(inner) => inner.unparse(),
            StorageLocation::StorageKeyword(inner) => inner.unparse(),
            StorageLocation::CallDataKeyword(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum StringExpression {
    StringLiterals(StringLiterals),
    HexStringLiterals(HexStringLiterals),
    UnicodeStringLiterals(UnicodeStringLiterals),
}

#[derive(Clone, Debug)]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    FunctionType(FunctionType),
    MappingType(MappingType),
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

#[derive(Clone, Debug)]
pub enum UsingClause {
    IdentifierPath(IdentifierPath),
    UsingDeconstruction(UsingDeconstruction),
}

#[derive(Clone, Debug)]
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

impl UsingOperator {
    pub fn unparse(&self) -> &str {
        match self {
            UsingOperator::Ampersand(inner) => inner.unparse(),
            UsingOperator::Asterisk(inner) => inner.unparse(),
            UsingOperator::BangEqual(inner) => inner.unparse(),
            UsingOperator::Bar(inner) => inner.unparse(),
            UsingOperator::Caret(inner) => inner.unparse(),
            UsingOperator::EqualEqual(inner) => inner.unparse(),
            UsingOperator::GreaterThan(inner) => inner.unparse(),
            UsingOperator::GreaterThanEqual(inner) => inner.unparse(),
            UsingOperator::LessThan(inner) => inner.unparse(),
            UsingOperator::LessThanEqual(inner) => inner.unparse(),
            UsingOperator::Minus(inner) => inner.unparse(),
            UsingOperator::Percent(inner) => inner.unparse(),
            UsingOperator::Plus(inner) => inner.unparse(),
            UsingOperator::Slash(inner) => inner.unparse(),
            UsingOperator::Tilde(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum UsingTarget {
    TypeName(TypeName),
    Asterisk(Asterisk),
}

#[derive(Clone, Debug)]
pub enum VariableDeclarationTarget {
    SingleTypedDeclaration(SingleTypedDeclaration),
    MultiTypedDeclaration(MultiTypedDeclaration),
}

#[derive(Clone, Debug)]
pub enum VersionExpression {
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
}

#[derive(Clone, Debug)]
pub enum VersionLiteral {
    SimpleVersionLiteral(SimpleVersionLiteral),
    StringLiteral(StringLiteral),
}

#[derive(Clone, Debug)]
pub enum VersionOperator {
    PragmaCaret(PragmaCaret),
    PragmaTilde(PragmaTilde),
    PragmaEqual(PragmaEqual),
    PragmaLessThan(PragmaLessThan),
    PragmaGreaterThan(PragmaGreaterThan),
    PragmaLessThanEqual(PragmaLessThanEqual),
    PragmaGreaterThanEqual(PragmaGreaterThanEqual),
}

impl VersionOperator {
    pub fn unparse(&self) -> &str {
        match self {
            VersionOperator::PragmaCaret(inner) => inner.unparse(),
            VersionOperator::PragmaTilde(inner) => inner.unparse(),
            VersionOperator::PragmaEqual(inner) => inner.unparse(),
            VersionOperator::PragmaLessThan(inner) => inner.unparse(),
            VersionOperator::PragmaGreaterThan(inner) => inner.unparse(),
            VersionOperator::PragmaLessThanEqual(inner) => inner.unparse(),
            VersionOperator::PragmaGreaterThanEqual(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

#[derive(Clone, Debug)]
pub enum YulLiteral {
    TrueKeyword(TrueKeyword),
    FalseKeyword(FalseKeyword),
    DecimalLiteral(DecimalLiteral),
    HexLiteral(HexLiteral),
    HexStringLiteral(HexStringLiteral),
    StringLiteral(StringLiteral),
}

impl YulLiteral {
    pub fn unparse(&self) -> &str {
        match self {
            YulLiteral::TrueKeyword(inner) => inner.unparse(),
            YulLiteral::FalseKeyword(inner) => inner.unparse(),
            YulLiteral::DecimalLiteral(inner) => inner.unparse(),
            YulLiteral::HexLiteral(inner) => inner.unparse(),
            YulLiteral::HexStringLiteral(inner) => inner.unparse(),
            YulLiteral::StringLiteral(inner) => inner.unparse(),
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum YulSwitchCase {
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
}

//
// Repeated & Separated
//

pub type ArrayValues = Vec<Expression>;

pub type CallOptions = Vec<NamedArgument>;

pub type CatchClauses = Vec<CatchClause>;

pub type ContractMembers = Vec<ContractMember>;

pub type EnumMembers = Vec<Identifier>;

pub type HexStringLiterals = Vec<HexStringLiteral>;

pub type IdentifierPath = Vec<Identifier>;

pub type ImportDeconstructionSymbols = Vec<ImportDeconstructionSymbol>;

pub type InheritanceTypes = Vec<InheritanceType>;

pub type InterfaceMembers = Vec<ContractMember>;

pub type LibraryMembers = Vec<ContractMember>;

pub type ModifierInvocations = Vec<ModifierInvocation>;

pub type MultiTypedDeclarationElements = Vec<MultiTypedDeclarationElement>;

pub type NamedArguments = Vec<NamedArgument>;

pub type OverridePaths = Vec<IdentifierPath>;

pub type Parameters = Vec<Parameter>;

pub type PositionalArguments = Vec<Expression>;

pub type SimpleVersionLiteral = Vec<VersionSpecifier>;

pub type SourceUnitMembers = Vec<SourceUnitMember>;

pub type Statements = Vec<Statement>;

pub type StringLiterals = Vec<StringLiteral>;

pub type StructMembers = Vec<StructMember>;

pub type TupleValues = Vec<TupleValue>;

pub type UnicodeStringLiterals = Vec<UnicodeStringLiteral>;

pub type UsingDeconstructionSymbols = Vec<UsingDeconstructionSymbol>;

pub type VersionExpressionSet = Vec<VersionExpression>;

pub type VersionExpressionSets = Vec<VersionExpressionSet>;

pub type YulArguments = Vec<YulExpression>;

pub type YulFlags = Vec<StringLiteral>;

pub type YulParameters = Vec<Identifier>;

pub type YulPath = Vec<Identifier>;

pub type YulPaths = Vec<YulPath>;

pub type YulStatements = Vec<YulStatement>;

pub type YulSwitchCases = Vec<YulSwitchCase>;

pub type YulVariableNames = Vec<Identifier>;

//
// Terminals
//

pub type ABIEncoderV2Keyword = Arc<ABIEncoderV2KeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ABIEncoderV2KeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl ABIEncoderV2KeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "ABIEncoderV2"
    }
}

pub type AbicoderV1Keyword = Arc<AbicoderV1KeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AbicoderV1KeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl AbicoderV1KeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "v1"
    }
}

pub type AbicoderV2Keyword = Arc<AbicoderV2KeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AbicoderV2KeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl AbicoderV2KeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "v2"
    }
}

pub type Ampersand = Arc<AmpersandStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AmpersandStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl AmpersandStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "&"
    }
}

pub type AmpersandEqual = Arc<AmpersandEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AmpersandEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl AmpersandEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "&="
    }
}

pub type Asterisk = Arc<AsteriskStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsteriskStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl AsteriskStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "*"
    }
}

pub type AsteriskEqual = Arc<AsteriskEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsteriskEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl AsteriskEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "*="
    }
}

pub type Bang = Arc<BangStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BangStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl BangStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "!"
    }
}

pub type BangEqual = Arc<BangEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BangEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl BangEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "!="
    }
}

pub type Bar = Arc<BarStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BarStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl BarStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "|"
    }
}

pub type BarEqual = Arc<BarEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BarEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl BarEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "|="
    }
}

pub type BoolKeyword = Arc<BoolKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoolKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl BoolKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "bool"
    }
}

pub type BytesKeyword = Arc<BytesKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BytesKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub text: String,
}

impl BytesKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &str {
        &self.text
    }
}

pub type CallDataKeyword = Arc<CallDataKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallDataKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl CallDataKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "calldata"
    }
}

pub type Caret = Arc<CaretStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaretStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl CaretStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "^"
    }
}

pub type CaretEqual = Arc<CaretEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaretEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl CaretEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "^="
    }
}

pub type DaysKeyword = Arc<DaysKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DaysKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl DaysKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "days"
    }
}

pub type DecimalLiteral = Arc<DecimalLiteralStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecimalLiteralStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub text: String,
}

impl DecimalLiteralStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &str {
        &self.text
    }
}

pub type DeleteKeyword = Arc<DeleteKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeleteKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl DeleteKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "delete"
    }
}

pub type Equal = Arc<EqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl EqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "="
    }
}

pub type EqualEqual = Arc<EqualEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EqualEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl EqualEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "=="
    }
}

pub type EtherKeyword = Arc<EtherKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EtherKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl EtherKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "ether"
    }
}

pub type FalseKeyword = Arc<FalseKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FalseKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl FalseKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "false"
    }
}

pub type FixedKeyword = Arc<FixedKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub text: String,
}

impl FixedKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &str {
        &self.text
    }
}

pub type GreaterThan = Arc<GreaterThanStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GreaterThanStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl GreaterThanStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        ">"
    }
}

pub type GreaterThanEqual = Arc<GreaterThanEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GreaterThanEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl GreaterThanEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        ">="
    }
}

pub type GreaterThanGreaterThan = Arc<GreaterThanGreaterThanStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GreaterThanGreaterThanStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl GreaterThanGreaterThanStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        ">>"
    }
}

pub type GreaterThanGreaterThanEqual = Arc<GreaterThanGreaterThanEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GreaterThanGreaterThanEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl GreaterThanGreaterThanEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        ">>="
    }
}

pub type GreaterThanGreaterThanGreaterThan = Arc<GreaterThanGreaterThanGreaterThanStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GreaterThanGreaterThanGreaterThanStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl GreaterThanGreaterThanGreaterThanStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        ">>>"
    }
}

pub type GreaterThanGreaterThanGreaterThanEqual = Arc<GreaterThanGreaterThanGreaterThanEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GreaterThanGreaterThanGreaterThanEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl GreaterThanGreaterThanGreaterThanEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        ">>>="
    }
}

pub type GweiKeyword = Arc<GweiKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GweiKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl GweiKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "gwei"
    }
}

pub type HexLiteral = Arc<HexLiteralStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HexLiteralStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub text: String,
}

impl HexLiteralStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &str {
        &self.text
    }
}

pub type HexStringLiteral = Arc<HexStringLiteralStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HexStringLiteralStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub text: String,
}

impl HexStringLiteralStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &str {
        &self.text
    }
}

pub type HoursKeyword = Arc<HoursKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HoursKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl HoursKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "hours"
    }
}

pub type Identifier = Arc<IdentifierStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentifierStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub text: String,
}

impl IdentifierStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &str {
        &self.text
    }
}

pub type IntKeyword = Arc<IntKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub text: String,
}

impl IntKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &str {
        &self.text
    }
}

pub type LessThan = Arc<LessThanStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LessThanStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl LessThanStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "<"
    }
}

pub type LessThanEqual = Arc<LessThanEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LessThanEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl LessThanEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "<="
    }
}

pub type LessThanLessThan = Arc<LessThanLessThanStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LessThanLessThanStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl LessThanLessThanStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "<<"
    }
}

pub type LessThanLessThanEqual = Arc<LessThanLessThanEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LessThanLessThanEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl LessThanLessThanEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "<<="
    }
}

pub type MemoryKeyword = Arc<MemoryKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl MemoryKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "memory"
    }
}

pub type Minus = Arc<MinusStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinusStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl MinusStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "-"
    }
}

pub type MinusEqual = Arc<MinusEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinusEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl MinusEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "-="
    }
}

pub type MinusMinus = Arc<MinusMinusStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinusMinusStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl MinusMinusStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "--"
    }
}

pub type MinutesKeyword = Arc<MinutesKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinutesKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl MinutesKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "minutes"
    }
}

pub type PayableKeyword = Arc<PayableKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PayableKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PayableKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "payable"
    }
}

pub type Percent = Arc<PercentStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PercentStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PercentStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "%"
    }
}

pub type PercentEqual = Arc<PercentEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PercentEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PercentEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "%="
    }
}

pub type Plus = Arc<PlusStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlusStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PlusStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "+"
    }
}

pub type PlusEqual = Arc<PlusEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlusEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PlusEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "+="
    }
}

pub type PlusPlus = Arc<PlusPlusStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlusPlusStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PlusPlusStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "++"
    }
}

pub type PragmaCaret = Arc<PragmaCaretStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PragmaCaretStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PragmaCaretStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "^"
    }
}

pub type PragmaEqual = Arc<PragmaEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PragmaEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PragmaEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "="
    }
}

pub type PragmaGreaterThan = Arc<PragmaGreaterThanStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PragmaGreaterThanStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PragmaGreaterThanStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        ">"
    }
}

pub type PragmaGreaterThanEqual = Arc<PragmaGreaterThanEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PragmaGreaterThanEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PragmaGreaterThanEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        ">="
    }
}

pub type PragmaLessThan = Arc<PragmaLessThanStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PragmaLessThanStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PragmaLessThanStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "<"
    }
}

pub type PragmaLessThanEqual = Arc<PragmaLessThanEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PragmaLessThanEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PragmaLessThanEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "<="
    }
}

pub type PragmaTilde = Arc<PragmaTildeStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PragmaTildeStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl PragmaTildeStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "~"
    }
}

pub type SMTCheckerKeyword = Arc<SMTCheckerKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SMTCheckerKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl SMTCheckerKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "SMTChecker"
    }
}

pub type SecondsKeyword = Arc<SecondsKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecondsKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl SecondsKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "seconds"
    }
}

pub type Semicolon = Arc<SemicolonStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemicolonStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl SemicolonStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        ";"
    }
}

pub type Slash = Arc<SlashStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SlashStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl SlashStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "/"
    }
}

pub type SlashEqual = Arc<SlashEqualStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SlashEqualStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl SlashEqualStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "/="
    }
}

pub type StorageKeyword = Arc<StorageKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl StorageKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "storage"
    }
}

pub type StringKeyword = Arc<StringKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl StringKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "string"
    }
}

pub type StringLiteral = Arc<StringLiteralStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringLiteralStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub text: String,
}

impl StringLiteralStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &str {
        &self.text
    }
}

pub type SuperKeyword = Arc<SuperKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuperKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl SuperKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "super"
    }
}

pub type ThisKeyword = Arc<ThisKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThisKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl ThisKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "this"
    }
}

pub type Tilde = Arc<TildeStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TildeStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl TildeStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "~"
    }
}

pub type TrueKeyword = Arc<TrueKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrueKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl TrueKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "true"
    }
}

pub type UfixedKeyword = Arc<UfixedKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UfixedKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub text: String,
}

impl UfixedKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &str {
        &self.text
    }
}

pub type UintKeyword = Arc<UintKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UintKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub text: String,
}

impl UintKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &str {
        &self.text
    }
}

pub type UnicodeStringLiteral = Arc<UnicodeStringLiteralStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnicodeStringLiteralStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub text: String,
}

impl UnicodeStringLiteralStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &str {
        &self.text
    }
}

pub type VersionSpecifier = Arc<VersionSpecifierStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VersionSpecifierStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub text: String,
}

impl VersionSpecifierStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &str {
        &self.text
    }
}

pub type WeeksKeyword = Arc<WeeksKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeeksKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl WeeksKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "weeks"
    }
}

pub type WeiKeyword = Arc<WeiKeywordStruct>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeiKeywordStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
}

impl WeiKeywordStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn unparse(&self) -> &'static str {
        "wei"
    }
}
