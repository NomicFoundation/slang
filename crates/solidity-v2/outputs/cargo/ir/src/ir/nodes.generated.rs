// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(non_camel_case_types)]

use std::ops::Range;
use std::rc::Rc;
use std::vec::Vec;

pub(super) use slang_solidity_v2_common::nodes::NodeId;

//
// Sequences
//

pub type AbicoderPragma = Rc<AbicoderPragmaStruct>;

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

pub type AdditiveExpression = Rc<AdditiveExpressionStruct>;

#[derive(Debug)]
pub struct AdditiveExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub expression_additive_expression_operator: Expression_AdditiveExpression_Operator,
    pub right_operand: Expression,
}

impl AdditiveExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type AddressType = Rc<AddressTypeStruct>;

#[derive(Debug)]
pub struct AddressTypeStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub payable_keyword: bool,
}

impl AddressTypeStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type AndExpression = Rc<AndExpressionStruct>;

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

pub type ArrayExpression = Rc<ArrayExpressionStruct>;

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

pub type ArrayTypeName = Rc<ArrayTypeNameStruct>;

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

pub type AssemblyStatement = Rc<AssemblyStatementStruct>;

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

pub type AssignmentExpression = Rc<AssignmentExpressionStruct>;

#[derive(Debug)]
pub struct AssignmentExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub expression_assignment_expression_operator: Expression_AssignmentExpression_Operator,
    pub right_operand: Expression,
}

impl AssignmentExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type BitwiseAndExpression = Rc<BitwiseAndExpressionStruct>;

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

pub type BitwiseOrExpression = Rc<BitwiseOrExpressionStruct>;

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

pub type BitwiseXorExpression = Rc<BitwiseXorExpressionStruct>;

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

pub type Block = Rc<BlockStruct>;

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

pub type BreakStatement = Rc<BreakStatementStruct>;

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

pub type CallOptionsExpression = Rc<CallOptionsExpressionStruct>;

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

pub type CatchClause = Rc<CatchClauseStruct>;

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

pub type CatchClauseError = Rc<CatchClauseErrorStruct>;

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

pub type ConditionalExpression = Rc<ConditionalExpressionStruct>;

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

pub type ConstantDefinition = Rc<ConstantDefinitionStruct>;

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

pub type ContinueStatement = Rc<ContinueStatementStruct>;

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

pub type ContractDefinition = Rc<ContractDefinitionStruct>;

#[derive(Debug)]
pub struct ContractDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub abstract_keyword: bool,
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

pub type DecimalNumberExpression = Rc<DecimalNumberExpressionStruct>;

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

pub type DoWhileStatement = Rc<DoWhileStatementStruct>;

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

pub type EmitStatement = Rc<EmitStatementStruct>;

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

pub type EnumDefinition = Rc<EnumDefinitionStruct>;

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

pub type EqualityExpression = Rc<EqualityExpressionStruct>;

#[derive(Debug)]
pub struct EqualityExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub expression_equality_expression_operator: Expression_EqualityExpression_Operator,
    pub right_operand: Expression,
}

impl EqualityExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ErrorDefinition = Rc<ErrorDefinitionStruct>;

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

pub type EventDefinition = Rc<EventDefinitionStruct>;

#[derive(Debug)]
pub struct EventDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub name: Identifier,
    pub anonymous_keyword: bool,
    pub parameters: Parameters,
}

impl EventDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ExperimentalPragma = Rc<ExperimentalPragmaStruct>;

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

pub type ExponentiationExpression = Rc<ExponentiationExpressionStruct>;

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

pub type ExpressionStatement = Rc<ExpressionStatementStruct>;

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

pub type ForStatement = Rc<ForStatementStruct>;

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

pub type FunctionCallExpression = Rc<FunctionCallExpressionStruct>;

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

pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

#[derive(Debug)]
pub struct FunctionDefinitionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub kind: FunctionKind,
    pub name: Option<Identifier>,
    pub parameters: Parameters,
    pub visibility: FunctionVisibility,
    pub mutability: FunctionMutability,
    pub virtual_keyword: bool,
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

pub type FunctionType = Rc<FunctionTypeStruct>;

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

pub type HexNumberExpression = Rc<HexNumberExpressionStruct>;

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

pub type IfStatement = Rc<IfStatementStruct>;

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

pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

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

pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

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

pub type IndexAccessExpression = Rc<IndexAccessExpressionStruct>;

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

pub type InequalityExpression = Rc<InequalityExpressionStruct>;

#[derive(Debug)]
pub struct InequalityExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub expression_inequality_expression_operator: Expression_InequalityExpression_Operator,
    pub right_operand: Expression,
}

impl InequalityExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type InheritanceType = Rc<InheritanceTypeStruct>;

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

pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

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

pub type LibraryDefinition = Rc<LibraryDefinitionStruct>;

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

pub type MappingType = Rc<MappingTypeStruct>;

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

pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

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

pub type ModifierInvocation = Rc<ModifierInvocationStruct>;

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

pub type MultiTypedDeclaration = Rc<MultiTypedDeclarationStruct>;

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

pub type MultiTypedDeclarationElement = Rc<MultiTypedDeclarationElementStruct>;

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

pub type MultiplicativeExpression = Rc<MultiplicativeExpressionStruct>;

#[derive(Debug)]
pub struct MultiplicativeExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub expression_multiplicative_expression_operator: Expression_MultiplicativeExpression_Operator,
    pub right_operand: Expression,
}

impl MultiplicativeExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type NamedArgument = Rc<NamedArgumentStruct>;

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

pub type NewExpression = Rc<NewExpressionStruct>;

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

pub type OrExpression = Rc<OrExpressionStruct>;

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

pub type Parameter = Rc<ParameterStruct>;

#[derive(Debug)]
pub struct ParameterStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Option<Identifier>,
    pub indexed: bool,
}

impl ParameterStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type PathImport = Rc<PathImportStruct>;

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

pub type PostfixExpression = Rc<PostfixExpressionStruct>;

#[derive(Debug)]
pub struct PostfixExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub operand: Expression,
    pub expression_postfix_expression_operator: Expression_PostfixExpression_Operator,
}

impl PostfixExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type PragmaDirective = Rc<PragmaDirectiveStruct>;

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

pub type PrefixExpression = Rc<PrefixExpressionStruct>;

#[derive(Debug)]
pub struct PrefixExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub expression_prefix_expression_operator: Expression_PrefixExpression_Operator,
    pub operand: Expression,
}

impl PrefixExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type ReturnStatement = Rc<ReturnStatementStruct>;

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

pub type RevertStatement = Rc<RevertStatementStruct>;

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

pub type ShiftExpression = Rc<ShiftExpressionStruct>;

#[derive(Debug)]
pub struct ShiftExpressionStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub left_operand: Expression,
    pub expression_shift_expression_operator: Expression_ShiftExpression_Operator,
    pub right_operand: Expression,
}

impl ShiftExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type SingleTypedDeclaration = Rc<SingleTypedDeclarationStruct>;

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

pub type SourceUnit = Rc<SourceUnitStruct>;

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

pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

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

pub type StructDefinition = Rc<StructDefinitionStruct>;

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

pub type StructMember = Rc<StructMemberStruct>;

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

pub type TryStatement = Rc<TryStatementStruct>;

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

pub type TupleExpression = Rc<TupleExpressionStruct>;

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

pub type TupleValue = Rc<TupleValueStruct>;

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

pub type TypeExpression = Rc<TypeExpressionStruct>;

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

pub type UncheckedBlock = Rc<UncheckedBlockStruct>;

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

pub type UserDefinedValueTypeDefinition = Rc<UserDefinedValueTypeDefinitionStruct>;

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

pub type UsingDeconstruction = Rc<UsingDeconstructionStruct>;

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

pub type UsingDeconstructionSymbol = Rc<UsingDeconstructionSymbolStruct>;

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

pub type UsingDirective = Rc<UsingDirectiveStruct>;

#[derive(Debug)]
pub struct UsingDirectiveStruct {
    pub(crate) id: NodeId,
    pub range: Range<usize>,
    pub clause: UsingClause,
    pub target: UsingTarget,
    pub global_keyword: bool,
}

impl UsingDirectiveStruct {
    pub fn id(&self) -> NodeId {
        self.id
    }
}

pub type VariableDeclaration = Rc<VariableDeclarationStruct>;

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

pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

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

pub type VersionPragma = Rc<VersionPragmaStruct>;

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

pub type VersionRange = Rc<VersionRangeStruct>;

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

pub type VersionTerm = Rc<VersionTermStruct>;

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

pub type WhileStatement = Rc<WhileStatementStruct>;

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

pub type YulBlock = Rc<YulBlockStruct>;

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

pub type YulBreakStatement = Rc<YulBreakStatementStruct>;

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

pub type YulContinueStatement = Rc<YulContinueStatementStruct>;

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

pub type YulDefaultCase = Rc<YulDefaultCaseStruct>;

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

pub type YulForStatement = Rc<YulForStatementStruct>;

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

pub type YulFunctionCallExpression = Rc<YulFunctionCallExpressionStruct>;

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

pub type YulFunctionDefinition = Rc<YulFunctionDefinitionStruct>;

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

pub type YulIfStatement = Rc<YulIfStatementStruct>;

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

pub type YulLeaveStatement = Rc<YulLeaveStatementStruct>;

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

pub type YulSwitchStatement = Rc<YulSwitchStatementStruct>;

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

pub type YulValueCase = Rc<YulValueCaseStruct>;

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

pub type YulVariableAssignmentStatement = Rc<YulVariableAssignmentStatementStruct>;

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

pub type YulVariableDeclarationStatement = Rc<YulVariableDeclarationStatementStruct>;

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

pub type YulVariableDeclarationValue = Rc<YulVariableDeclarationValueStruct>;

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbicoderVersion {
    AbicoderV1Keyword,
    AbicoderV2Keyword,
}

#[derive(Clone, Debug)]
pub enum ArgumentsDeclaration {
    PositionalArguments(PositionalArguments),
    NamedArguments(NamedArguments),
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
    AddressType(AddressType),
    BytesKeyword(BytesKeyword),
    IntKeyword(IntKeyword),
    UintKeyword(UintKeyword),
    FixedKeyword(FixedKeyword),
    UfixedKeyword(UfixedKeyword),
    BoolKeyword,
    StringKeyword,
}

#[derive(Clone, Debug)]
pub enum ExperimentalFeature {
    StringLiteral(StringLiteral),
    ABIEncoderV2Keyword,
    SMTCheckerKeyword,
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
    Identifier(Identifier),
    PayableKeyword,
    ThisKeyword,
    SuperKeyword,
    TrueKeyword,
    FalseKeyword,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Expression_AdditiveExpression_Operator {
    Minus,
    Plus,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Expression_EqualityExpression_Operator {
    BangEqual,
    EqualEqual,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Expression_InequalityExpression_Operator {
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Expression_MultiplicativeExpression_Operator {
    Asterisk,
    Percent,
    Slash,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Expression_PostfixExpression_Operator {
    MinusMinus,
    PlusPlus,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Expression_PrefixExpression_Operator {
    Bang,
    DeleteKeyword,
    Minus,
    MinusMinus,
    PlusPlus,
    Tilde,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Expression_ShiftExpression_Operator {
    GreaterThanGreaterThan,
    GreaterThanGreaterThanGreaterThan,
    LessThanLessThan,
}

#[derive(Clone, Debug)]
pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[derive(Clone, Debug)]
pub enum ForStatementInitialization {
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Semicolon,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug)]
pub enum Pragma {
    VersionPragma(VersionPragma),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StorageLocation {
    MemoryKeyword,
    StorageKeyword,
    CallDataKeyword,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug)]
pub enum UsingTarget {
    TypeName(TypeName),
    Asterisk,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VersionOperator {
    PragmaCaret,
    PragmaTilde,
    PragmaEqual,
    PragmaLessThan,
    PragmaGreaterThan,
    PragmaLessThanEqual,
    PragmaGreaterThanEqual,
}

#[derive(Clone, Debug)]
pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

#[derive(Clone, Debug)]
pub enum YulLiteral {
    DecimalLiteral(DecimalLiteral),
    HexLiteral(HexLiteral),
    HexStringLiteral(HexStringLiteral),
    StringLiteral(StringLiteral),
    TrueKeyword,
    FalseKeyword,
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
// Non-unique Terminals
//

pub type BytesKeyword = Rc<BytesKeywordStruct>;

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

pub type DecimalLiteral = Rc<DecimalLiteralStruct>;

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

pub type FixedKeyword = Rc<FixedKeywordStruct>;

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

pub type HexLiteral = Rc<HexLiteralStruct>;

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

pub type HexStringLiteral = Rc<HexStringLiteralStruct>;

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

pub type Identifier = Rc<IdentifierStruct>;

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

pub type IntKeyword = Rc<IntKeywordStruct>;

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

pub type StringLiteral = Rc<StringLiteralStruct>;

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

pub type UfixedKeyword = Rc<UfixedKeywordStruct>;

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

pub type UintKeyword = Rc<UintKeywordStruct>;

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

pub type UnicodeStringLiteral = Rc<UnicodeStringLiteralStruct>;

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

pub type VersionSpecifier = Rc<VersionSpecifierStruct>;

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
