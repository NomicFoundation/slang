// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;
use std::vec::Vec;

use metaslang_cst::nodes::NodeId;

use crate::cst::SyntaxNode;

//
// Sequences:
//

pub type SourceUnit = Rc<SourceUnitStruct>;

#[derive(Debug, PartialEq)]
pub struct SourceUnitStruct {
    pub node: Rc<SyntaxNode>,
    pub members: SourceUnitMembers,
}

impl SourceUnitStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type PragmaDirective = Rc<PragmaDirectiveStruct>;

#[derive(Debug, PartialEq)]
pub struct PragmaDirectiveStruct {
    pub node: Rc<SyntaxNode>,
    pub pragma: Pragma,
}

impl PragmaDirectiveStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AbicoderPragma = Rc<AbicoderPragmaStruct>;

#[derive(Debug, PartialEq)]
pub struct AbicoderPragmaStruct {
    pub node: Rc<SyntaxNode>,
    pub version: AbicoderVersion,
}

impl AbicoderPragmaStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ExperimentalPragma = Rc<ExperimentalPragmaStruct>;

#[derive(Debug, PartialEq)]
pub struct ExperimentalPragmaStruct {
    pub node: Rc<SyntaxNode>,
    pub feature: ExperimentalFeature,
}

impl ExperimentalPragmaStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type VersionPragma = Rc<VersionPragmaStruct>;

#[derive(Debug, PartialEq)]
pub struct VersionPragmaStruct {
    pub node: Rc<SyntaxNode>,
    pub sets: VersionExpressionSets,
}

impl VersionPragmaStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type VersionRange = Rc<VersionRangeStruct>;

#[derive(Debug, PartialEq)]
pub struct VersionRangeStruct {
    pub node: Rc<SyntaxNode>,
    pub start: VersionLiteral,
    pub end: VersionLiteral,
}

impl VersionRangeStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type VersionTerm = Rc<VersionTermStruct>;

#[derive(Debug, PartialEq)]
pub struct VersionTermStruct {
    pub node: Rc<SyntaxNode>,
    pub operator: Option<VersionOperator>,
    pub literal: VersionLiteral,
}

impl VersionTermStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ImportDirective = Rc<ImportDirectiveStruct>;

#[derive(Debug, PartialEq)]
pub struct ImportDirectiveStruct {
    pub node: Rc<SyntaxNode>,
    pub clause: ImportClause,
}

impl ImportDirectiveStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type PathImport = Rc<PathImportStruct>;

#[derive(Debug, PartialEq)]
pub struct PathImportStruct {
    pub node: Rc<SyntaxNode>,
    pub alias: Option<Rc<SyntaxNode>>,
    pub path: Rc<SyntaxNode>,
}

impl PathImportStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type NamedImport = Rc<NamedImportStruct>;

#[derive(Debug, PartialEq)]
pub struct NamedImportStruct {
    pub node: Rc<SyntaxNode>,
    pub alias: Rc<SyntaxNode>,
    pub path: Rc<SyntaxNode>,
}

impl NamedImportStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

#[derive(Debug, PartialEq)]
pub struct ImportDeconstructionStruct {
    pub node: Rc<SyntaxNode>,
    pub symbols: ImportDeconstructionSymbols,
    pub path: Rc<SyntaxNode>,
}

impl ImportDeconstructionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

#[derive(Debug, PartialEq)]
pub struct ImportDeconstructionSymbolStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<SyntaxNode>,
    pub alias: Option<Rc<SyntaxNode>>,
}

impl ImportDeconstructionSymbolStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UsingDirective = Rc<UsingDirectiveStruct>;

#[derive(Debug, PartialEq)]
pub struct UsingDirectiveStruct {
    pub node: Rc<SyntaxNode>,
    pub clause: UsingClause,
    pub target: UsingTarget,
    pub global_keyword: bool,
}

impl UsingDirectiveStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UsingDeconstruction = Rc<UsingDeconstructionStruct>;

#[derive(Debug, PartialEq)]
pub struct UsingDeconstructionStruct {
    pub node: Rc<SyntaxNode>,
    pub symbols: UsingDeconstructionSymbols,
}

impl UsingDeconstructionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UsingDeconstructionSymbol = Rc<UsingDeconstructionSymbolStruct>;

#[derive(Debug, PartialEq)]
pub struct UsingDeconstructionSymbolStruct {
    pub node: Rc<SyntaxNode>,
    pub name: IdentifierPath,
    pub alias: Option<UsingOperator>,
}

impl UsingDeconstructionSymbolStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ContractDefinition = Rc<ContractDefinitionStruct>;

#[derive(Debug, PartialEq)]
pub struct ContractDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub abstract_keyword: bool,
    pub name: Rc<SyntaxNode>,
    pub members: ContractMembers,
    pub inheritance_types: InheritanceTypes,
    pub storage_layout: Option<Expression>,
}

impl ContractDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type InheritanceType = Rc<InheritanceTypeStruct>;

#[derive(Debug, PartialEq)]
pub struct InheritanceTypeStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

impl InheritanceTypeStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

#[derive(Debug, PartialEq)]
pub struct InterfaceDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<SyntaxNode>,
    pub inheritance: Option<InheritanceTypes>,
    pub members: InterfaceMembers,
}

impl InterfaceDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type LibraryDefinition = Rc<LibraryDefinitionStruct>;

#[derive(Debug, PartialEq)]
pub struct LibraryDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<SyntaxNode>,
    pub members: LibraryMembers,
}

impl LibraryDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type StructDefinition = Rc<StructDefinitionStruct>;

#[derive(Debug, PartialEq)]
pub struct StructDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<SyntaxNode>,
    pub members: StructMembers,
}

impl StructDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type StructMember = Rc<StructMemberStruct>;

#[derive(Debug, PartialEq)]
pub struct StructMemberStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub name: Rc<SyntaxNode>,
}

impl StructMemberStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type EnumDefinition = Rc<EnumDefinitionStruct>;

#[derive(Debug, PartialEq)]
pub struct EnumDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<SyntaxNode>,
    pub members: EnumMembers,
}

impl EnumDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ConstantDefinition = Rc<ConstantDefinitionStruct>;

#[derive(Debug, PartialEq)]
pub struct ConstantDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub name: Rc<SyntaxNode>,
    pub value: Expression,
}

impl ConstantDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

#[derive(Debug, PartialEq)]
pub struct StateVariableDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub name: Rc<SyntaxNode>,
    pub value: Option<Expression>,
    pub visibility: StateVariableVisibility,
    pub mutability: StateVariableMutability,
    pub override_specifier: Option<OverridePaths>,
}

impl StateVariableDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

#[derive(Debug, PartialEq)]
pub struct FunctionDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub parameters: Parameters,
    pub returns: Option<Parameters>,
    pub kind: FunctionKind,
    pub name: Option<Rc<SyntaxNode>>,
    pub body: Option<Block>,
    pub visibility: FunctionVisibility,
    pub mutability: FunctionMutability,
    pub virtual_keyword: bool,
    pub override_specifier: Option<OverridePaths>,
    pub modifier_invocations: ModifierInvocations,
}

impl FunctionDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type Parameter = Rc<ParameterStruct>;

#[derive(Debug, PartialEq)]
pub struct ParameterStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Option<Rc<SyntaxNode>>,
}

impl ParameterStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type OverrideSpecifier = Rc<OverrideSpecifierStruct>;

#[derive(Debug, PartialEq)]
pub struct OverrideSpecifierStruct {
    pub node: Rc<SyntaxNode>,
    pub overridden: Option<OverridePaths>,
}

impl OverrideSpecifierStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ModifierInvocation = Rc<ModifierInvocationStruct>;

#[derive(Debug, PartialEq)]
pub struct ModifierInvocationStruct {
    pub node: Rc<SyntaxNode>,
    pub name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

impl ModifierInvocationStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type EventDefinition = Rc<EventDefinitionStruct>;

#[derive(Debug, PartialEq)]
pub struct EventDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<SyntaxNode>,
    pub parameters: EventParameters,
    pub anonymous_keyword: bool,
}

impl EventDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type EventParameter = Rc<EventParameterStruct>;

#[derive(Debug, PartialEq)]
pub struct EventParameterStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub indexed_keyword: bool,
    pub name: Option<Rc<SyntaxNode>>,
}

impl EventParameterStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UserDefinedValueTypeDefinition = Rc<UserDefinedValueTypeDefinitionStruct>;

#[derive(Debug, PartialEq)]
pub struct UserDefinedValueTypeDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<SyntaxNode>,
    pub value_type: ElementaryType,
}

impl UserDefinedValueTypeDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ErrorDefinition = Rc<ErrorDefinitionStruct>;

#[derive(Debug, PartialEq)]
pub struct ErrorDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<SyntaxNode>,
    pub members: ErrorParameters,
}

impl ErrorDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ErrorParameter = Rc<ErrorParameterStruct>;

#[derive(Debug, PartialEq)]
pub struct ErrorParameterStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub name: Option<Rc<SyntaxNode>>,
}

impl ErrorParameterStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ArrayTypeName = Rc<ArrayTypeNameStruct>;

#[derive(Debug, PartialEq)]
pub struct ArrayTypeNameStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: TypeName,
    pub index: Option<Expression>,
}

impl ArrayTypeNameStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type FunctionType = Rc<FunctionTypeStruct>;

#[derive(Debug, PartialEq)]
pub struct FunctionTypeStruct {
    pub node: Rc<SyntaxNode>,
    pub parameters: Parameters,
    pub returns: Option<Parameters>,
    pub visibility: FunctionVisibility,
    pub mutability: FunctionMutability,
}

impl FunctionTypeStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type MappingType = Rc<MappingTypeStruct>;

#[derive(Debug, PartialEq)]
pub struct MappingTypeStruct {
    pub node: Rc<SyntaxNode>,
    pub key_type: MappingKey,
    pub value_type: MappingValue,
}

impl MappingTypeStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type MappingKey = Rc<MappingKeyStruct>;

#[derive(Debug, PartialEq)]
pub struct MappingKeyStruct {
    pub node: Rc<SyntaxNode>,
    pub key_type: MappingKeyType,
    pub name: Option<Rc<SyntaxNode>>,
}

impl MappingKeyStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type MappingValue = Rc<MappingValueStruct>;

#[derive(Debug, PartialEq)]
pub struct MappingValueStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub name: Option<Rc<SyntaxNode>>,
}

impl MappingValueStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AddressType = Rc<AddressTypeStruct>;

#[derive(Debug, PartialEq)]
pub struct AddressTypeStruct {
    pub node: Rc<SyntaxNode>,
    pub payable_keyword: bool,
}

impl AddressTypeStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type Block = Rc<BlockStruct>;

#[derive(Debug, PartialEq)]
pub struct BlockStruct {
    pub node: Rc<SyntaxNode>,
    pub statements: Statements,
}

impl BlockStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UncheckedBlock = Rc<UncheckedBlockStruct>;

#[derive(Debug, PartialEq)]
pub struct UncheckedBlockStruct {
    pub node: Rc<SyntaxNode>,
    pub block: Block,
}

impl UncheckedBlockStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ExpressionStatement = Rc<ExpressionStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct ExpressionStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub expression: Expression,
}

impl ExpressionStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AssemblyStatement = Rc<AssemblyStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct AssemblyStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub body: YulBlock,
    pub flags: AssemblyFlags,
    pub label: Option<Rc<SyntaxNode>>,
}

impl AssemblyStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TupleDeconstructionStatement = Rc<TupleDeconstructionStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct TupleDeconstructionStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub var_keyword: bool,
    pub elements: TupleDeconstructionElements,
    pub expression: Expression,
}

impl TupleDeconstructionStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TupleDeconstructionElement = Rc<TupleDeconstructionElementStruct>;

#[derive(Debug, PartialEq)]
pub struct TupleDeconstructionElementStruct {
    pub node: Rc<SyntaxNode>,
    pub member: Option<TupleMember>,
}

impl TupleDeconstructionElementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TypedTupleMember = Rc<TypedTupleMemberStruct>;

#[derive(Debug, PartialEq)]
pub struct TypedTupleMemberStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<SyntaxNode>,
}

impl TypedTupleMemberStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UntypedTupleMember = Rc<UntypedTupleMemberStruct>;

#[derive(Debug, PartialEq)]
pub struct UntypedTupleMemberStruct {
    pub node: Rc<SyntaxNode>,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<SyntaxNode>,
}

impl UntypedTupleMemberStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct VariableDeclarationStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub variable_type: VariableDeclarationType,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<SyntaxNode>,
    pub value: Option<Expression>,
}

impl VariableDeclarationStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type IfStatement = Rc<IfStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct IfStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub condition: Expression,
    pub body: Statement,
    pub else_branch: Option<Statement>,
}

impl IfStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ForStatement = Rc<ForStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct ForStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub initialization: ForStatementInitialization,
    pub condition: ForStatementCondition,
    pub iterator: Option<Expression>,
    pub body: Statement,
}

impl ForStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type WhileStatement = Rc<WhileStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct WhileStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub condition: Expression,
    pub body: Statement,
}

impl WhileStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type DoWhileStatement = Rc<DoWhileStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct DoWhileStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub body: Statement,
    pub condition: Expression,
}

impl DoWhileStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ContinueStatement = Rc<ContinueStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct ContinueStatementStruct {
    pub node: Rc<SyntaxNode>,
}

impl ContinueStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type BreakStatement = Rc<BreakStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct BreakStatementStruct {
    pub node: Rc<SyntaxNode>,
}

impl BreakStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ReturnStatement = Rc<ReturnStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct ReturnStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub expression: Option<Expression>,
}

impl ReturnStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type EmitStatement = Rc<EmitStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct EmitStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub event: IdentifierPath,
    pub arguments: ArgumentsDeclaration,
}

impl EmitStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TryStatement = Rc<TryStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct TryStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub expression: Expression,
    pub returns: Option<Parameters>,
    pub body: Block,
    pub catch_clauses: CatchClauses,
}

impl TryStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type CatchClause = Rc<CatchClauseStruct>;

#[derive(Debug, PartialEq)]
pub struct CatchClauseStruct {
    pub node: Rc<SyntaxNode>,
    pub error: Option<CatchClauseError>,
    pub body: Block,
}

impl CatchClauseStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type CatchClauseError = Rc<CatchClauseErrorStruct>;

#[derive(Debug, PartialEq)]
pub struct CatchClauseErrorStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Option<Rc<SyntaxNode>>,
    pub parameters: Parameters,
}

impl CatchClauseErrorStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type RevertStatement = Rc<RevertStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct RevertStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub error: Option<IdentifierPath>,
    pub arguments: ArgumentsDeclaration,
}

impl RevertStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ThrowStatement = Rc<ThrowStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct ThrowStatementStruct {
    pub node: Rc<SyntaxNode>,
}

impl ThrowStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AssignmentExpression = Rc<AssignmentExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct AssignmentExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<SyntaxNode>,
    pub right_operand: Expression,
}

impl AssignmentExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ConditionalExpression = Rc<ConditionalExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct ConditionalExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: Expression,
    pub true_expression: Expression,
    pub false_expression: Expression,
}

impl ConditionalExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type OrExpression = Rc<OrExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct OrExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl OrExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AndExpression = Rc<AndExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct AndExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl AndExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type EqualityExpression = Rc<EqualityExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct EqualityExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<SyntaxNode>,
    pub right_operand: Expression,
}

impl EqualityExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type InequalityExpression = Rc<InequalityExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct InequalityExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<SyntaxNode>,
    pub right_operand: Expression,
}

impl InequalityExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type BitwiseOrExpression = Rc<BitwiseOrExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct BitwiseOrExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl BitwiseOrExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type BitwiseXorExpression = Rc<BitwiseXorExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct BitwiseXorExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl BitwiseXorExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type BitwiseAndExpression = Rc<BitwiseAndExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct BitwiseAndExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl BitwiseAndExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ShiftExpression = Rc<ShiftExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct ShiftExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<SyntaxNode>,
    pub right_operand: Expression,
}

impl ShiftExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AdditiveExpression = Rc<AdditiveExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct AdditiveExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<SyntaxNode>,
    pub right_operand: Expression,
}

impl AdditiveExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type MultiplicativeExpression = Rc<MultiplicativeExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct MultiplicativeExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<SyntaxNode>,
    pub right_operand: Expression,
}

impl MultiplicativeExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ExponentiationExpression = Rc<ExponentiationExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct ExponentiationExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<SyntaxNode>,
    pub right_operand: Expression,
}

impl ExponentiationExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type PostfixExpression = Rc<PostfixExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct PostfixExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: Expression,
    pub operator: Rc<SyntaxNode>,
}

impl PostfixExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type PrefixExpression = Rc<PrefixExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct PrefixExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operator: Rc<SyntaxNode>,
    pub operand: Expression,
}

impl PrefixExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type FunctionCallExpression = Rc<FunctionCallExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct FunctionCallExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: Expression,
    pub arguments: ArgumentsDeclaration,
}

impl FunctionCallExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type CallOptionsExpression = Rc<CallOptionsExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct CallOptionsExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: Expression,
    pub options: CallOptions,
}

impl CallOptionsExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct MemberAccessExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: Expression,
    pub member: Rc<SyntaxNode>,
}

impl MemberAccessExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type IndexAccessExpression = Rc<IndexAccessExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct IndexAccessExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: Expression,
    pub start: Option<Expression>,
    pub end: Option<Expression>,
}

impl IndexAccessExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type NamedArgument = Rc<NamedArgumentStruct>;

#[derive(Debug, PartialEq)]
pub struct NamedArgumentStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<SyntaxNode>,
    pub value: Expression,
}

impl NamedArgumentStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TypeExpression = Rc<TypeExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct TypeExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
}

impl TypeExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type NewExpression = Rc<NewExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct NewExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
}

impl NewExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TupleExpression = Rc<TupleExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct TupleExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub items: TupleValues,
}

impl TupleExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TupleValue = Rc<TupleValueStruct>;

#[derive(Debug, PartialEq)]
pub struct TupleValueStruct {
    pub node: Rc<SyntaxNode>,
    pub expression: Option<Expression>,
}

impl TupleValueStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ArrayExpression = Rc<ArrayExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct ArrayExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub items: ArrayValues,
}

impl ArrayExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type HexNumberExpression = Rc<HexNumberExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct HexNumberExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub literal: Rc<SyntaxNode>,
    pub unit: Option<NumberUnit>,
}

impl HexNumberExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type DecimalNumberExpression = Rc<DecimalNumberExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct DecimalNumberExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub literal: Rc<SyntaxNode>,
    pub unit: Option<NumberUnit>,
}

impl DecimalNumberExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulBlock = Rc<YulBlockStruct>;

#[derive(Debug, PartialEq)]
pub struct YulBlockStruct {
    pub node: Rc<SyntaxNode>,
    pub statements: YulStatements,
}

impl YulBlockStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulFunctionDefinition = Rc<YulFunctionDefinitionStruct>;

#[derive(Debug, PartialEq)]
pub struct YulFunctionDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<SyntaxNode>,
    pub parameters: YulParameters,
    pub returns: Option<YulVariableNames>,
    pub body: YulBlock,
}

impl YulFunctionDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulVariableDeclarationStatement = Rc<YulVariableDeclarationStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct YulVariableDeclarationStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub variables: YulVariableNames,
    pub value: Option<YulVariableDeclarationValue>,
}

impl YulVariableDeclarationStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulVariableDeclarationValue = Rc<YulVariableDeclarationValueStruct>;

#[derive(Debug, PartialEq)]
pub struct YulVariableDeclarationValueStruct {
    pub node: Rc<SyntaxNode>,
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

impl YulVariableDeclarationValueStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulVariableAssignmentStatement = Rc<YulVariableAssignmentStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct YulVariableAssignmentStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub variables: YulPaths,
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

impl YulVariableAssignmentStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulColonAndEqual = Rc<YulColonAndEqualStruct>;

#[derive(Debug, PartialEq)]
pub struct YulColonAndEqualStruct {
    pub node: Rc<SyntaxNode>,
}

impl YulColonAndEqualStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulStackAssignmentStatement = Rc<YulStackAssignmentStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct YulStackAssignmentStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub assignment: YulStackAssignmentOperator,
    pub variable: Rc<SyntaxNode>,
}

impl YulStackAssignmentStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulEqualAndColon = Rc<YulEqualAndColonStruct>;

#[derive(Debug, PartialEq)]
pub struct YulEqualAndColonStruct {
    pub node: Rc<SyntaxNode>,
}

impl YulEqualAndColonStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulIfStatement = Rc<YulIfStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct YulIfStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub condition: YulExpression,
    pub body: YulBlock,
}

impl YulIfStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulForStatement = Rc<YulForStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct YulForStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub initialization: YulBlock,
    pub condition: YulExpression,
    pub iterator: YulBlock,
    pub body: YulBlock,
}

impl YulForStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulSwitchStatement = Rc<YulSwitchStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct YulSwitchStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub expression: YulExpression,
    pub cases: YulSwitchCases,
}

impl YulSwitchStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulDefaultCase = Rc<YulDefaultCaseStruct>;

#[derive(Debug, PartialEq)]
pub struct YulDefaultCaseStruct {
    pub node: Rc<SyntaxNode>,
    pub body: YulBlock,
}

impl YulDefaultCaseStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulValueCase = Rc<YulValueCaseStruct>;

#[derive(Debug, PartialEq)]
pub struct YulValueCaseStruct {
    pub node: Rc<SyntaxNode>,
    pub value: YulLiteral,
    pub body: YulBlock,
}

impl YulValueCaseStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulLeaveStatement = Rc<YulLeaveStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct YulLeaveStatementStruct {
    pub node: Rc<SyntaxNode>,
}

impl YulLeaveStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulBreakStatement = Rc<YulBreakStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct YulBreakStatementStruct {
    pub node: Rc<SyntaxNode>,
}

impl YulBreakStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulContinueStatement = Rc<YulContinueStatementStruct>;

#[derive(Debug, PartialEq)]
pub struct YulContinueStatementStruct {
    pub node: Rc<SyntaxNode>,
}

impl YulContinueStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulLabel = Rc<YulLabelStruct>;

#[derive(Debug, PartialEq)]
pub struct YulLabelStruct {
    pub node: Rc<SyntaxNode>,
    pub label: Rc<SyntaxNode>,
}

impl YulLabelStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulFunctionCallExpression = Rc<YulFunctionCallExpressionStruct>;

#[derive(Debug, PartialEq)]
pub struct YulFunctionCallExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: YulExpression,
    pub arguments: YulArguments,
}

impl YulFunctionCallExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

//
// Choices:
//

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Pragma {
    VersionPragma(VersionPragma),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
}

#[derive(Debug, PartialEq)]
pub enum AbicoderVersion {
    AbicoderV1Keyword,
    AbicoderV2Keyword,
}

#[derive(Debug, PartialEq)]
pub enum ExperimentalFeature {
    StringLiteral(Rc<SyntaxNode>),
    ABIEncoderV2Keyword,
    SMTCheckerKeyword,
}

#[derive(Debug, PartialEq)]
pub enum VersionExpression {
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
}

#[derive(Debug, PartialEq)]
pub enum VersionOperator {
    Caret,
    Tilde,
    Equal,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
}

#[derive(Debug, PartialEq)]
pub enum VersionLiteral {
    SimpleVersionLiteral(SimpleVersionLiteral),
    SingleQuotedVersionLiteral(Rc<SyntaxNode>),
    DoubleQuotedVersionLiteral(Rc<SyntaxNode>),
}

#[derive(Debug, PartialEq)]
pub enum ImportClause {
    PathImport(PathImport),
    NamedImport(NamedImport),
    ImportDeconstruction(ImportDeconstruction),
}

#[derive(Debug, PartialEq)]
pub enum UsingClause {
    IdentifierPath(IdentifierPath),
    UsingDeconstruction(UsingDeconstruction),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum UsingTarget {
    TypeName(TypeName),
    Asterisk,
}

#[derive(Debug, PartialEq)]
pub enum ContractMember {
    UsingDirective(UsingDirective),
    FunctionDefinition(FunctionDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    StateVariableDefinition(StateVariableDefinition),
}

#[derive(Debug, PartialEq)]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    FunctionType(FunctionType),
    MappingType(MappingType),
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug, PartialEq)]
pub enum MappingKeyType {
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug, PartialEq)]
pub enum ElementaryType {
    AddressType(AddressType),
    BytesKeyword(Rc<SyntaxNode>),
    IntKeyword(Rc<SyntaxNode>),
    UintKeyword(Rc<SyntaxNode>),
    FixedKeyword(Rc<SyntaxNode>),
    UfixedKeyword(Rc<SyntaxNode>),
    BoolKeyword,
    ByteKeyword,
    StringKeyword,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum TupleMember {
    TypedTupleMember(TypedTupleMember),
    UntypedTupleMember(UntypedTupleMember),
}

#[derive(Debug, PartialEq)]
pub enum VariableDeclarationType {
    TypeName(TypeName),
    VarKeyword,
}

#[derive(Debug, PartialEq)]
pub enum StorageLocation {
    MemoryKeyword,
    StorageKeyword,
    CallDataKeyword,
}

#[derive(Debug, PartialEq)]
pub enum ForStatementInitialization {
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[derive(Debug, PartialEq)]
pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[derive(Debug, PartialEq)]
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
    Identifier(Rc<SyntaxNode>),
    PayableKeyword,
    ThisKeyword,
    SuperKeyword,
    TrueKeyword,
    FalseKeyword,
}

#[derive(Debug, PartialEq)]
pub enum ArgumentsDeclaration {
    PositionalArguments(PositionalArguments),
    NamedArguments(NamedArguments),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum StringExpression {
    Strings(Strings),
    HexStrings(HexStrings),
    UnicodeStrings(UnicodeStrings),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum YulAssignmentOperator {
    YulColonAndEqual(YulColonAndEqual),
    ColonEqual,
}

#[derive(Debug, PartialEq)]
pub enum YulStackAssignmentOperator {
    YulEqualAndColon(YulEqualAndColon),
    EqualColon,
}

#[derive(Debug, PartialEq)]
pub enum YulSwitchCase {
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
}

#[derive(Debug, PartialEq)]
pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

#[derive(Debug, PartialEq)]
pub enum YulLiteral {
    YulDecimalLiteral(Rc<SyntaxNode>),
    YulHexLiteral(Rc<SyntaxNode>),
    StringLiteral(Rc<SyntaxNode>),
    HexStringLiteral(Rc<SyntaxNode>),
    YulTrueKeyword,
    YulFalseKeyword,
}

#[derive(Debug, PartialEq)]
pub enum FunctionKind {
    Regular,
    Constructor,
    Unnamed,
    Fallback,
    Receive,
    Modifier,
}

#[derive(Debug, PartialEq)]
pub enum FunctionVisibility {
    Public,
    Private,
    Internal,
    External,
}

#[derive(Debug, PartialEq)]
pub enum FunctionMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

#[derive(Debug, PartialEq)]
pub enum StateVariableVisibility {
    Public,
    Private,
    Internal,
}

#[derive(Debug, PartialEq)]
pub enum StateVariableMutability {
    Mutable,
    Constant,
    Immutable,
    Transient,
}

//
// Repeated & Separated
//

pub type SourceUnitMembers = Vec<SourceUnitMember>;

pub type VersionExpressionSets = Vec<VersionExpressionSet>;

pub type VersionExpressionSet = Vec<VersionExpression>;

pub type SimpleVersionLiteral = Vec<Rc<SyntaxNode>>;

pub type ImportDeconstructionSymbols = Vec<ImportDeconstructionSymbol>;

pub type UsingDeconstructionSymbols = Vec<UsingDeconstructionSymbol>;

pub type InheritanceTypes = Vec<InheritanceType>;

pub type ContractMembers = Vec<ContractMember>;

pub type InterfaceMembers = Vec<ContractMember>;

pub type LibraryMembers = Vec<ContractMember>;

pub type StructMembers = Vec<StructMember>;

pub type EnumMembers = Vec<Rc<SyntaxNode>>;

pub type Parameters = Vec<Parameter>;

pub type OverridePaths = Vec<IdentifierPath>;

pub type EventParameters = Vec<EventParameter>;

pub type ErrorParameters = Vec<ErrorParameter>;

pub type Statements = Vec<Statement>;

pub type TupleDeconstructionElements = Vec<TupleDeconstructionElement>;

pub type CatchClauses = Vec<CatchClause>;

pub type PositionalArguments = Vec<Expression>;

pub type NamedArguments = Vec<NamedArgument>;

pub type CallOptions = Vec<NamedArgument>;

pub type TupleValues = Vec<TupleValue>;

pub type ArrayValues = Vec<Expression>;

pub type IdentifierPath = Vec<Rc<SyntaxNode>>;

pub type YulStatements = Vec<YulStatement>;

pub type YulParameters = Vec<Rc<SyntaxNode>>;

pub type YulVariableNames = Vec<Rc<SyntaxNode>>;

pub type YulSwitchCases = Vec<YulSwitchCase>;

pub type YulArguments = Vec<YulExpression>;

pub type YulPaths = Vec<YulPath>;

pub type YulPath = Vec<Rc<SyntaxNode>>;

pub type ModifierInvocations = Vec<ModifierInvocation>;

pub type Strings = Vec<Rc<SyntaxNode>>;

pub type HexStrings = Vec<Rc<SyntaxNode>>;

pub type UnicodeStrings = Vec<Rc<SyntaxNode>>;

pub type AssemblyFlags = Vec<Rc<SyntaxNode>>;
