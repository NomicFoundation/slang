// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::HashMap;
use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;
use crate::cst::{NodeId, SyntaxNode};

pub enum NodeType {
    SourceUnit(SourceUnit),
    PragmaDirective(PragmaDirective),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
    VersionPragma(VersionPragma),
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
    ImportDirective(ImportDirective),
    PathImport(PathImport),
    NamedImport(NamedImport),
    ImportDeconstruction(ImportDeconstruction),
    ImportDeconstructionSymbol(ImportDeconstructionSymbol),
    UsingDirective(UsingDirective),
    UsingDeconstruction(UsingDeconstruction),
    UsingDeconstructionSymbol(UsingDeconstructionSymbol),
    ContractDefinition(ContractDefinition),
    InheritanceType(InheritanceType),
    InterfaceDefinition(InterfaceDefinition),
    LibraryDefinition(LibraryDefinition),
    StructDefinition(StructDefinition),
    StructMember(StructMember),
    EnumDefinition(EnumDefinition),
    ConstantDefinition(ConstantDefinition),
    StateVariableDefinition(StateVariableDefinition),
    FunctionDefinition(FunctionDefinition),
    Parameter(Parameter),
    OverrideSpecifier(OverrideSpecifier),
    ModifierInvocation(ModifierInvocation),
    EventDefinition(EventDefinition),
    EventParameter(EventParameter),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    ErrorDefinition(ErrorDefinition),
    ErrorParameter(ErrorParameter),
    ArrayTypeName(ArrayTypeName),
    FunctionType(FunctionType),
    MappingType(MappingType),
    MappingKey(MappingKey),
    MappingValue(MappingValue),
    AddressType(AddressType),
    Block(Block),
    UncheckedBlock(UncheckedBlock),
    ExpressionStatement(ExpressionStatement),
    AssemblyStatement(AssemblyStatement),
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    TupleDeconstructionElement(TupleDeconstructionElement),
    TypedTupleMember(TypedTupleMember),
    UntypedTupleMember(UntypedTupleMember),
    VariableDeclarationStatement(VariableDeclarationStatement),
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ContinueStatement(ContinueStatement),
    BreakStatement(BreakStatement),
    ReturnStatement(ReturnStatement),
    EmitStatement(EmitStatement),
    TryStatement(TryStatement),
    CatchClause(CatchClause),
    CatchClauseError(CatchClauseError),
    RevertStatement(RevertStatement),
    ThrowStatement(ThrowStatement),
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
    NamedArgument(NamedArgument),
    TypeExpression(TypeExpression),
    NewExpression(NewExpression),
    TupleExpression(TupleExpression),
    TupleValue(TupleValue),
    ArrayExpression(ArrayExpression),
    HexNumberExpression(HexNumberExpression),
    DecimalNumberExpression(DecimalNumberExpression),
    YulBlock(YulBlock),
    YulFunctionDefinition(YulFunctionDefinition),
    YulVariableDeclarationStatement(YulVariableDeclarationStatement),
    YulVariableDeclarationValue(YulVariableDeclarationValue),
    YulVariableAssignmentStatement(YulVariableAssignmentStatement),
    YulColonAndEqual(YulColonAndEqual),
    YulStackAssignmentStatement(YulStackAssignmentStatement),
    YulEqualAndColon(YulEqualAndColon),
    YulIfStatement(YulIfStatement),
    YulForStatement(YulForStatement),
    YulSwitchStatement(YulSwitchStatement),
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
    YulLeaveStatement(YulLeaveStatement),
    YulBreakStatement(YulBreakStatement),
    YulContinueStatement(YulContinueStatement),
    YulLabel(YulLabel),
    YulFunctionCallExpression(YulFunctionCallExpression),
}

pub type TreeIndex = HashMap<NodeId, NodeType>;

pub trait IntoIr<T> {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<T>;
}

impl IntoIr<SourceUnit> for SourceUnit {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::SourceUnit(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<PragmaDirective> for PragmaDirective {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::PragmaDirective(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<AbicoderPragma> for AbicoderPragma {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::AbicoderPragma(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ExperimentalPragma> for ExperimentalPragma {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ExperimentalPragma(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<VersionPragma> for VersionPragma {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::VersionPragma(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<VersionRange> for VersionRange {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::VersionRange(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<VersionTerm> for VersionTerm {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::VersionTerm(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ImportDirective> for ImportDirective {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ImportDirective(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<PathImport> for PathImport {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::PathImport(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<NamedImport> for NamedImport {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::NamedImport(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ImportDeconstruction> for ImportDeconstruction {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ImportDeconstruction(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ImportDeconstructionSymbol> for ImportDeconstructionSymbol {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ImportDeconstructionSymbol(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<UsingDirective> for UsingDirective {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::UsingDirective(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<UsingDeconstruction> for UsingDeconstruction {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::UsingDeconstruction(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<UsingDeconstructionSymbol> for UsingDeconstructionSymbol {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::UsingDeconstructionSymbol(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ContractDefinition> for ContractDefinition {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ContractDefinition(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<InheritanceType> for InheritanceType {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::InheritanceType(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<InterfaceDefinition> for InterfaceDefinition {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::InterfaceDefinition(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<LibraryDefinition> for LibraryDefinition {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::LibraryDefinition(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<StructDefinition> for StructDefinition {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::StructDefinition(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<StructMember> for StructMember {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::StructMember(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<EnumDefinition> for EnumDefinition {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::EnumDefinition(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ConstantDefinition> for ConstantDefinition {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ConstantDefinition(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<StateVariableDefinition> for StateVariableDefinition {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::StateVariableDefinition(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<FunctionDefinition> for FunctionDefinition {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::FunctionDefinition(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<Parameter> for Parameter {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::Parameter(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<OverrideSpecifier> for OverrideSpecifier {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::OverrideSpecifier(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ModifierInvocation> for ModifierInvocation {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ModifierInvocation(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<EventDefinition> for EventDefinition {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::EventDefinition(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<EventParameter> for EventParameter {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::EventParameter(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<UserDefinedValueTypeDefinition> for UserDefinedValueTypeDefinition {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::UserDefinedValueTypeDefinition(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ErrorDefinition> for ErrorDefinition {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ErrorDefinition(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ErrorParameter> for ErrorParameter {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ErrorParameter(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ArrayTypeName> for ArrayTypeName {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ArrayTypeName(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<FunctionType> for FunctionType {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::FunctionType(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<MappingType> for MappingType {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::MappingType(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<MappingKey> for MappingKey {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::MappingKey(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<MappingValue> for MappingValue {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::MappingValue(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<AddressType> for AddressType {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::AddressType(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<Block> for Block {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::Block(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<UncheckedBlock> for UncheckedBlock {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::UncheckedBlock(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ExpressionStatement> for ExpressionStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ExpressionStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<AssemblyStatement> for AssemblyStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::AssemblyStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<TupleDeconstructionStatement> for TupleDeconstructionStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::TupleDeconstructionStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<TupleDeconstructionElement> for TupleDeconstructionElement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::TupleDeconstructionElement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<TypedTupleMember> for TypedTupleMember {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::TypedTupleMember(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<UntypedTupleMember> for UntypedTupleMember {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::UntypedTupleMember(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<VariableDeclarationStatement> for VariableDeclarationStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::VariableDeclarationStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<IfStatement> for IfStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::IfStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ForStatement> for ForStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ForStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<WhileStatement> for WhileStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::WhileStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<DoWhileStatement> for DoWhileStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::DoWhileStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ContinueStatement> for ContinueStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ContinueStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<BreakStatement> for BreakStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::BreakStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ReturnStatement> for ReturnStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ReturnStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<EmitStatement> for EmitStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::EmitStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<TryStatement> for TryStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::TryStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<CatchClause> for CatchClause {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::CatchClause(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<CatchClauseError> for CatchClauseError {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::CatchClauseError(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<RevertStatement> for RevertStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::RevertStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ThrowStatement> for ThrowStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ThrowStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<AssignmentExpression> for AssignmentExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::AssignmentExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ConditionalExpression> for ConditionalExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ConditionalExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<OrExpression> for OrExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::OrExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<AndExpression> for AndExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::AndExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<EqualityExpression> for EqualityExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::EqualityExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<InequalityExpression> for InequalityExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::InequalityExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<BitwiseOrExpression> for BitwiseOrExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::BitwiseOrExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<BitwiseXorExpression> for BitwiseXorExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::BitwiseXorExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<BitwiseAndExpression> for BitwiseAndExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::BitwiseAndExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ShiftExpression> for ShiftExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ShiftExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<AdditiveExpression> for AdditiveExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::AdditiveExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<MultiplicativeExpression> for MultiplicativeExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::MultiplicativeExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ExponentiationExpression> for ExponentiationExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ExponentiationExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<PostfixExpression> for PostfixExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::PostfixExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<PrefixExpression> for PrefixExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::PrefixExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<FunctionCallExpression> for FunctionCallExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::FunctionCallExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<CallOptionsExpression> for CallOptionsExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::CallOptionsExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<MemberAccessExpression> for MemberAccessExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::MemberAccessExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<IndexAccessExpression> for IndexAccessExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::IndexAccessExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<NamedArgument> for NamedArgument {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::NamedArgument(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<TypeExpression> for TypeExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::TypeExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<NewExpression> for NewExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::NewExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<TupleExpression> for TupleExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::TupleExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<TupleValue> for TupleValue {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::TupleValue(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<ArrayExpression> for ArrayExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::ArrayExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<HexNumberExpression> for HexNumberExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::HexNumberExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<DecimalNumberExpression> for DecimalNumberExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::DecimalNumberExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulBlock> for YulBlock {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulBlock(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulFunctionDefinition> for YulFunctionDefinition {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulFunctionDefinition(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulVariableDeclarationStatement> for YulVariableDeclarationStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulVariableDeclarationStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulVariableDeclarationValue> for YulVariableDeclarationValue {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulVariableDeclarationValue(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulVariableAssignmentStatement> for YulVariableAssignmentStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulVariableAssignmentStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulColonAndEqual> for YulColonAndEqual {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulColonAndEqual(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulStackAssignmentStatement> for YulStackAssignmentStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulStackAssignmentStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulEqualAndColon> for YulEqualAndColon {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulEqualAndColon(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulIfStatement> for YulIfStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulIfStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulForStatement> for YulForStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulForStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulSwitchStatement> for YulSwitchStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulSwitchStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulDefaultCase> for YulDefaultCase {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulDefaultCase(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulValueCase> for YulValueCase {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulValueCase(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulLeaveStatement> for YulLeaveStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulLeaveStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulBreakStatement> for YulBreakStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulBreakStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulContinueStatement> for YulContinueStatement {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulContinueStatement(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulLabel> for YulLabel {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulLabel(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

impl IntoIr<YulFunctionCallExpression> for YulFunctionCallExpression {
    fn into_ir(node: &Rc<SyntaxNode>, index: &TreeIndex) -> Option<Self> {
        index.get(&node.id()).and_then(|ir_node| {
            if let NodeType::YulFunctionCallExpression(ir_node) = ir_node {
                Some(Rc::clone(ir_node))
            } else {
                None
            }
        })
    }
}

//
// Sequences:
//

pub fn register_source_unit(node: &SourceUnit, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::SourceUnit(Rc::clone(node)));
    register_source_unit_members(&node.members, tree);
}

pub fn register_pragma_directive(node: &PragmaDirective, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::PragmaDirective(Rc::clone(node)));
    register_pragma(&node.pragma, tree);
}

pub fn register_abicoder_pragma(node: &AbicoderPragma, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::AbicoderPragma(Rc::clone(node)));
    register_abicoder_version(&node.version, tree);
}

pub fn register_experimental_pragma(node: &ExperimentalPragma, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ExperimentalPragma(Rc::clone(node)));
    register_experimental_feature(&node.feature, tree);
}

pub fn register_version_pragma(node: &VersionPragma, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::VersionPragma(Rc::clone(node)));
    register_version_expression_sets(&node.sets, tree);
}

pub fn register_version_range(node: &VersionRange, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::VersionRange(Rc::clone(node)));
    register_version_literal(&node.start, tree);
    register_version_literal(&node.end, tree);
}

pub fn register_version_term(node: &VersionTerm, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::VersionTerm(Rc::clone(node)));
    if let Some(ref operator) = node.operator {
        register_version_operator(operator, tree);
    }
    register_version_literal(&node.literal, tree);
}

pub fn register_import_directive(node: &ImportDirective, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ImportDirective(Rc::clone(node)));
    register_import_clause(&node.clause, tree);
}

pub fn register_path_import(node: &PathImport, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::PathImport(Rc::clone(node)));
}

pub fn register_named_import(node: &NamedImport, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::NamedImport(Rc::clone(node)));
}

pub fn register_import_deconstruction(node: &ImportDeconstruction, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ImportDeconstruction(Rc::clone(node)));
    register_import_deconstruction_symbols(&node.symbols, tree);
}

pub fn register_import_deconstruction_symbol(
    node: &ImportDeconstructionSymbol,
    tree: &mut TreeIndex,
) {
    tree.insert(
        node.id(),
        NodeType::ImportDeconstructionSymbol(Rc::clone(node)),
    );
}

pub fn register_using_directive(node: &UsingDirective, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::UsingDirective(Rc::clone(node)));
    register_using_clause(&node.clause, tree);
    register_using_target(&node.target, tree);
}

pub fn register_using_deconstruction(node: &UsingDeconstruction, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::UsingDeconstruction(Rc::clone(node)));
    register_using_deconstruction_symbols(&node.symbols, tree);
}

pub fn register_using_deconstruction_symbol(
    node: &UsingDeconstructionSymbol,
    tree: &mut TreeIndex,
) {
    tree.insert(
        node.id(),
        NodeType::UsingDeconstructionSymbol(Rc::clone(node)),
    );
    register_identifier_path(&node.name, tree);
    if let Some(ref alias) = node.alias {
        register_using_operator(alias, tree);
    }
}

pub fn register_contract_definition(node: &ContractDefinition, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ContractDefinition(Rc::clone(node)));
    register_contract_members(&node.members, tree);
    register_inheritance_types(&node.inheritance_types, tree);
    if let Some(ref storage_layout) = node.storage_layout {
        register_expression(storage_layout, tree);
    }
}

pub fn register_inheritance_type(node: &InheritanceType, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::InheritanceType(Rc::clone(node)));
    register_identifier_path(&node.type_name, tree);
    if let Some(ref arguments) = node.arguments {
        register_arguments_declaration(arguments, tree);
    }
}

pub fn register_interface_definition(node: &InterfaceDefinition, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::InterfaceDefinition(Rc::clone(node)));
    if let Some(ref inheritance) = node.inheritance {
        register_inheritance_types(inheritance, tree);
    }
    register_interface_members(&node.members, tree);
}

pub fn register_library_definition(node: &LibraryDefinition, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::LibraryDefinition(Rc::clone(node)));
    register_library_members(&node.members, tree);
}

pub fn register_struct_definition(node: &StructDefinition, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::StructDefinition(Rc::clone(node)));
    register_struct_members(&node.members, tree);
}

pub fn register_struct_member(node: &StructMember, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::StructMember(Rc::clone(node)));
    register_type_name(&node.type_name, tree);
}

pub fn register_enum_definition(node: &EnumDefinition, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::EnumDefinition(Rc::clone(node)));
    register_enum_members(&node.members, tree);
}

pub fn register_constant_definition(node: &ConstantDefinition, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ConstantDefinition(Rc::clone(node)));
    register_type_name(&node.type_name, tree);
    register_expression(&node.value, tree);
}

pub fn register_state_variable_definition(node: &StateVariableDefinition, tree: &mut TreeIndex) {
    tree.insert(
        node.id(),
        NodeType::StateVariableDefinition(Rc::clone(node)),
    );
    register_type_name(&node.type_name, tree);
    if let Some(ref value) = node.value {
        register_expression(value, tree);
    }
    register_state_variable_visibility(&node.visibility, tree);
    register_state_variable_mutability(&node.mutability, tree);
    if let Some(ref override_specifier) = node.override_specifier {
        register_override_paths(override_specifier, tree);
    }
}

pub fn register_function_definition(node: &FunctionDefinition, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::FunctionDefinition(Rc::clone(node)));
    register_parameters(&node.parameters, tree);
    if let Some(ref returns) = node.returns {
        register_parameters(returns, tree);
    }
    register_function_kind(&node.kind, tree);
    if let Some(ref body) = node.body {
        register_block(body, tree);
    }
    register_function_visibility(&node.visibility, tree);
    register_function_mutability(&node.mutability, tree);
    if let Some(ref override_specifier) = node.override_specifier {
        register_override_paths(override_specifier, tree);
    }
    register_modifier_invocations(&node.modifier_invocations, tree);
}

pub fn register_parameter(node: &Parameter, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::Parameter(Rc::clone(node)));
    register_type_name(&node.type_name, tree);
    if let Some(ref storage_location) = node.storage_location {
        register_storage_location(storage_location, tree);
    }
}

pub fn register_override_specifier(node: &OverrideSpecifier, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::OverrideSpecifier(Rc::clone(node)));
    if let Some(ref overridden) = node.overridden {
        register_override_paths(overridden, tree);
    }
}

pub fn register_modifier_invocation(node: &ModifierInvocation, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ModifierInvocation(Rc::clone(node)));
    register_identifier_path(&node.name, tree);
    if let Some(ref arguments) = node.arguments {
        register_arguments_declaration(arguments, tree);
    }
}

pub fn register_event_definition(node: &EventDefinition, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::EventDefinition(Rc::clone(node)));
    register_event_parameters(&node.parameters, tree);
}

pub fn register_event_parameter(node: &EventParameter, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::EventParameter(Rc::clone(node)));
    register_type_name(&node.type_name, tree);
}

pub fn register_user_defined_value_type_definition(
    node: &UserDefinedValueTypeDefinition,
    tree: &mut TreeIndex,
) {
    tree.insert(
        node.id(),
        NodeType::UserDefinedValueTypeDefinition(Rc::clone(node)),
    );
    register_elementary_type(&node.value_type, tree);
}

pub fn register_error_definition(node: &ErrorDefinition, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ErrorDefinition(Rc::clone(node)));
    register_error_parameters(&node.members, tree);
}

pub fn register_error_parameter(node: &ErrorParameter, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ErrorParameter(Rc::clone(node)));
    register_type_name(&node.type_name, tree);
}

pub fn register_array_type_name(node: &ArrayTypeName, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ArrayTypeName(Rc::clone(node)));
    register_type_name(&node.operand, tree);
    if let Some(ref index) = node.index {
        register_expression(index, tree);
    }
}

pub fn register_function_type(node: &FunctionType, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::FunctionType(Rc::clone(node)));
    register_parameters(&node.parameters, tree);
    if let Some(ref returns) = node.returns {
        register_parameters(returns, tree);
    }
    register_function_visibility(&node.visibility, tree);
    register_function_mutability(&node.mutability, tree);
}

pub fn register_mapping_type(node: &MappingType, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::MappingType(Rc::clone(node)));
    register_mapping_key(&node.key_type, tree);
    register_mapping_value(&node.value_type, tree);
}

pub fn register_mapping_key(node: &MappingKey, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::MappingKey(Rc::clone(node)));
    register_mapping_key_type(&node.key_type, tree);
}

pub fn register_mapping_value(node: &MappingValue, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::MappingValue(Rc::clone(node)));
    register_type_name(&node.type_name, tree);
}

pub fn register_address_type(node: &AddressType, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::AddressType(Rc::clone(node)));
}

pub fn register_block(node: &Block, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::Block(Rc::clone(node)));
    register_statements(&node.statements, tree);
}

pub fn register_unchecked_block(node: &UncheckedBlock, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::UncheckedBlock(Rc::clone(node)));
    register_block(&node.block, tree);
}

pub fn register_expression_statement(node: &ExpressionStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ExpressionStatement(Rc::clone(node)));
    register_expression(&node.expression, tree);
}

pub fn register_assembly_statement(node: &AssemblyStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::AssemblyStatement(Rc::clone(node)));
    register_yul_block(&node.body, tree);
    register_assembly_flags(&node.flags, tree);
}

pub fn register_tuple_deconstruction_statement(
    node: &TupleDeconstructionStatement,
    tree: &mut TreeIndex,
) {
    tree.insert(
        node.id(),
        NodeType::TupleDeconstructionStatement(Rc::clone(node)),
    );
    register_tuple_deconstruction_elements(&node.elements, tree);
    register_expression(&node.expression, tree);
}

pub fn register_tuple_deconstruction_element(
    node: &TupleDeconstructionElement,
    tree: &mut TreeIndex,
) {
    tree.insert(
        node.id(),
        NodeType::TupleDeconstructionElement(Rc::clone(node)),
    );
    if let Some(ref member) = node.member {
        register_tuple_member(member, tree);
    }
}

pub fn register_typed_tuple_member(node: &TypedTupleMember, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::TypedTupleMember(Rc::clone(node)));
    register_type_name(&node.type_name, tree);
    if let Some(ref storage_location) = node.storage_location {
        register_storage_location(storage_location, tree);
    }
}

pub fn register_untyped_tuple_member(node: &UntypedTupleMember, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::UntypedTupleMember(Rc::clone(node)));
    if let Some(ref storage_location) = node.storage_location {
        register_storage_location(storage_location, tree);
    }
}

pub fn register_variable_declaration_statement(
    node: &VariableDeclarationStatement,
    tree: &mut TreeIndex,
) {
    tree.insert(
        node.id(),
        NodeType::VariableDeclarationStatement(Rc::clone(node)),
    );
    register_variable_declaration_type(&node.variable_type, tree);
    if let Some(ref storage_location) = node.storage_location {
        register_storage_location(storage_location, tree);
    }
    if let Some(ref value) = node.value {
        register_expression(value, tree);
    }
}

pub fn register_if_statement(node: &IfStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::IfStatement(Rc::clone(node)));
    register_expression(&node.condition, tree);
    register_statement(&node.body, tree);
    if let Some(ref else_branch) = node.else_branch {
        register_statement(else_branch, tree);
    }
}

pub fn register_for_statement(node: &ForStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ForStatement(Rc::clone(node)));
    register_for_statement_initialization(&node.initialization, tree);
    register_for_statement_condition(&node.condition, tree);
    if let Some(ref iterator) = node.iterator {
        register_expression(iterator, tree);
    }
    register_statement(&node.body, tree);
}

pub fn register_while_statement(node: &WhileStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::WhileStatement(Rc::clone(node)));
    register_expression(&node.condition, tree);
    register_statement(&node.body, tree);
}

pub fn register_do_while_statement(node: &DoWhileStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::DoWhileStatement(Rc::clone(node)));
    register_statement(&node.body, tree);
    register_expression(&node.condition, tree);
}

pub fn register_continue_statement(node: &ContinueStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ContinueStatement(Rc::clone(node)));
}

pub fn register_break_statement(node: &BreakStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::BreakStatement(Rc::clone(node)));
}

pub fn register_return_statement(node: &ReturnStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ReturnStatement(Rc::clone(node)));
    if let Some(ref expression) = node.expression {
        register_expression(expression, tree);
    }
}

pub fn register_emit_statement(node: &EmitStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::EmitStatement(Rc::clone(node)));
    register_identifier_path(&node.event, tree);
    register_arguments_declaration(&node.arguments, tree);
}

pub fn register_try_statement(node: &TryStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::TryStatement(Rc::clone(node)));
    register_expression(&node.expression, tree);
    if let Some(ref returns) = node.returns {
        register_parameters(returns, tree);
    }
    register_block(&node.body, tree);
    register_catch_clauses(&node.catch_clauses, tree);
}

pub fn register_catch_clause(node: &CatchClause, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::CatchClause(Rc::clone(node)));
    if let Some(ref error) = node.error {
        register_catch_clause_error(error, tree);
    }
    register_block(&node.body, tree);
}

pub fn register_catch_clause_error(node: &CatchClauseError, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::CatchClauseError(Rc::clone(node)));
    register_parameters(&node.parameters, tree);
}

pub fn register_revert_statement(node: &RevertStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::RevertStatement(Rc::clone(node)));
    if let Some(ref error) = node.error {
        register_identifier_path(error, tree);
    }
    register_arguments_declaration(&node.arguments, tree);
}

pub fn register_throw_statement(node: &ThrowStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ThrowStatement(Rc::clone(node)));
}

pub fn register_assignment_expression(node: &AssignmentExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::AssignmentExpression(Rc::clone(node)));
    register_expression(&node.left_operand, tree);
    register_expression(&node.right_operand, tree);
}

pub fn register_conditional_expression(node: &ConditionalExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ConditionalExpression(Rc::clone(node)));
    register_expression(&node.operand, tree);
    register_expression(&node.true_expression, tree);
    register_expression(&node.false_expression, tree);
}

pub fn register_or_expression(node: &OrExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::OrExpression(Rc::clone(node)));
    register_expression(&node.left_operand, tree);
    register_expression(&node.right_operand, tree);
}

pub fn register_and_expression(node: &AndExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::AndExpression(Rc::clone(node)));
    register_expression(&node.left_operand, tree);
    register_expression(&node.right_operand, tree);
}

pub fn register_equality_expression(node: &EqualityExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::EqualityExpression(Rc::clone(node)));
    register_expression(&node.left_operand, tree);
    register_expression(&node.right_operand, tree);
}

pub fn register_inequality_expression(node: &InequalityExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::InequalityExpression(Rc::clone(node)));
    register_expression(&node.left_operand, tree);
    register_expression(&node.right_operand, tree);
}

pub fn register_bitwise_or_expression(node: &BitwiseOrExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::BitwiseOrExpression(Rc::clone(node)));
    register_expression(&node.left_operand, tree);
    register_expression(&node.right_operand, tree);
}

pub fn register_bitwise_xor_expression(node: &BitwiseXorExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::BitwiseXorExpression(Rc::clone(node)));
    register_expression(&node.left_operand, tree);
    register_expression(&node.right_operand, tree);
}

pub fn register_bitwise_and_expression(node: &BitwiseAndExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::BitwiseAndExpression(Rc::clone(node)));
    register_expression(&node.left_operand, tree);
    register_expression(&node.right_operand, tree);
}

pub fn register_shift_expression(node: &ShiftExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ShiftExpression(Rc::clone(node)));
    register_expression(&node.left_operand, tree);
    register_expression(&node.right_operand, tree);
}

pub fn register_additive_expression(node: &AdditiveExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::AdditiveExpression(Rc::clone(node)));
    register_expression(&node.left_operand, tree);
    register_expression(&node.right_operand, tree);
}

pub fn register_multiplicative_expression(node: &MultiplicativeExpression, tree: &mut TreeIndex) {
    tree.insert(
        node.id(),
        NodeType::MultiplicativeExpression(Rc::clone(node)),
    );
    register_expression(&node.left_operand, tree);
    register_expression(&node.right_operand, tree);
}

pub fn register_exponentiation_expression(node: &ExponentiationExpression, tree: &mut TreeIndex) {
    tree.insert(
        node.id(),
        NodeType::ExponentiationExpression(Rc::clone(node)),
    );
    register_expression(&node.left_operand, tree);
    register_expression(&node.right_operand, tree);
}

pub fn register_postfix_expression(node: &PostfixExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::PostfixExpression(Rc::clone(node)));
    register_expression(&node.operand, tree);
}

pub fn register_prefix_expression(node: &PrefixExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::PrefixExpression(Rc::clone(node)));
    register_expression(&node.operand, tree);
}

pub fn register_function_call_expression(node: &FunctionCallExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::FunctionCallExpression(Rc::clone(node)));
    register_expression(&node.operand, tree);
    register_arguments_declaration(&node.arguments, tree);
}

pub fn register_call_options_expression(node: &CallOptionsExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::CallOptionsExpression(Rc::clone(node)));
    register_expression(&node.operand, tree);
    register_call_options(&node.options, tree);
}

pub fn register_member_access_expression(node: &MemberAccessExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::MemberAccessExpression(Rc::clone(node)));
    register_expression(&node.operand, tree);
}

pub fn register_index_access_expression(node: &IndexAccessExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::IndexAccessExpression(Rc::clone(node)));
    register_expression(&node.operand, tree);
    if let Some(ref start) = node.start {
        register_expression(start, tree);
    }
    if let Some(ref end) = node.end {
        register_expression(end, tree);
    }
}

pub fn register_named_argument(node: &NamedArgument, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::NamedArgument(Rc::clone(node)));
    register_expression(&node.value, tree);
}

pub fn register_type_expression(node: &TypeExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::TypeExpression(Rc::clone(node)));
    register_type_name(&node.type_name, tree);
}

pub fn register_new_expression(node: &NewExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::NewExpression(Rc::clone(node)));
    register_type_name(&node.type_name, tree);
}

pub fn register_tuple_expression(node: &TupleExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::TupleExpression(Rc::clone(node)));
    register_tuple_values(&node.items, tree);
}

pub fn register_tuple_value(node: &TupleValue, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::TupleValue(Rc::clone(node)));
    if let Some(ref expression) = node.expression {
        register_expression(expression, tree);
    }
}

pub fn register_array_expression(node: &ArrayExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::ArrayExpression(Rc::clone(node)));
    register_array_values(&node.items, tree);
}

pub fn register_hex_number_expression(node: &HexNumberExpression, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::HexNumberExpression(Rc::clone(node)));
    if let Some(ref unit) = node.unit {
        register_number_unit(unit, tree);
    }
}

pub fn register_decimal_number_expression(node: &DecimalNumberExpression, tree: &mut TreeIndex) {
    tree.insert(
        node.id(),
        NodeType::DecimalNumberExpression(Rc::clone(node)),
    );
    if let Some(ref unit) = node.unit {
        register_number_unit(unit, tree);
    }
}

pub fn register_yul_block(node: &YulBlock, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulBlock(Rc::clone(node)));
    register_yul_statements(&node.statements, tree);
}

pub fn register_yul_function_definition(node: &YulFunctionDefinition, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulFunctionDefinition(Rc::clone(node)));
    register_yul_parameters(&node.parameters, tree);
    if let Some(ref returns) = node.returns {
        register_yul_variable_names(returns, tree);
    }
    register_yul_block(&node.body, tree);
}

pub fn register_yul_variable_declaration_statement(
    node: &YulVariableDeclarationStatement,
    tree: &mut TreeIndex,
) {
    tree.insert(
        node.id(),
        NodeType::YulVariableDeclarationStatement(Rc::clone(node)),
    );
    register_yul_variable_names(&node.variables, tree);
    if let Some(ref value) = node.value {
        register_yul_variable_declaration_value(value, tree);
    }
}

pub fn register_yul_variable_declaration_value(
    node: &YulVariableDeclarationValue,
    tree: &mut TreeIndex,
) {
    tree.insert(
        node.id(),
        NodeType::YulVariableDeclarationValue(Rc::clone(node)),
    );
    register_yul_assignment_operator(&node.assignment, tree);
    register_yul_expression(&node.expression, tree);
}

pub fn register_yul_variable_assignment_statement(
    node: &YulVariableAssignmentStatement,
    tree: &mut TreeIndex,
) {
    tree.insert(
        node.id(),
        NodeType::YulVariableAssignmentStatement(Rc::clone(node)),
    );
    register_yul_paths(&node.variables, tree);
    register_yul_assignment_operator(&node.assignment, tree);
    register_yul_expression(&node.expression, tree);
}

pub fn register_yul_colon_and_equal(node: &YulColonAndEqual, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulColonAndEqual(Rc::clone(node)));
}

pub fn register_yul_stack_assignment_statement(
    node: &YulStackAssignmentStatement,
    tree: &mut TreeIndex,
) {
    tree.insert(
        node.id(),
        NodeType::YulStackAssignmentStatement(Rc::clone(node)),
    );
    register_yul_stack_assignment_operator(&node.assignment, tree);
}

pub fn register_yul_equal_and_colon(node: &YulEqualAndColon, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulEqualAndColon(Rc::clone(node)));
}

pub fn register_yul_if_statement(node: &YulIfStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulIfStatement(Rc::clone(node)));
    register_yul_expression(&node.condition, tree);
    register_yul_block(&node.body, tree);
}

pub fn register_yul_for_statement(node: &YulForStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulForStatement(Rc::clone(node)));
    register_yul_block(&node.initialization, tree);
    register_yul_expression(&node.condition, tree);
    register_yul_block(&node.iterator, tree);
    register_yul_block(&node.body, tree);
}

pub fn register_yul_switch_statement(node: &YulSwitchStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulSwitchStatement(Rc::clone(node)));
    register_yul_expression(&node.expression, tree);
    register_yul_switch_cases(&node.cases, tree);
}

pub fn register_yul_default_case(node: &YulDefaultCase, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulDefaultCase(Rc::clone(node)));
    register_yul_block(&node.body, tree);
}

pub fn register_yul_value_case(node: &YulValueCase, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulValueCase(Rc::clone(node)));
    register_yul_literal(&node.value, tree);
    register_yul_block(&node.body, tree);
}

pub fn register_yul_leave_statement(node: &YulLeaveStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulLeaveStatement(Rc::clone(node)));
}

pub fn register_yul_break_statement(node: &YulBreakStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulBreakStatement(Rc::clone(node)));
}

pub fn register_yul_continue_statement(node: &YulContinueStatement, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulContinueStatement(Rc::clone(node)));
}

pub fn register_yul_label(node: &YulLabel, tree: &mut TreeIndex) {
    tree.insert(node.id(), NodeType::YulLabel(Rc::clone(node)));
}

pub fn register_yul_function_call_expression(
    node: &YulFunctionCallExpression,
    tree: &mut TreeIndex,
) {
    tree.insert(
        node.id(),
        NodeType::YulFunctionCallExpression(Rc::clone(node)),
    );
    register_yul_expression(&node.operand, tree);
    register_yul_arguments(&node.arguments, tree);
}

//
// Choices:
//

pub fn register_source_unit_member(node: &SourceUnitMember, tree: &mut TreeIndex) {
    match node {
        SourceUnitMember::PragmaDirective(ref pragma_directive) => {
            register_pragma_directive(pragma_directive, tree);
        }
        SourceUnitMember::ImportDirective(ref import_directive) => {
            register_import_directive(import_directive, tree);
        }
        SourceUnitMember::ContractDefinition(ref contract_definition) => {
            register_contract_definition(contract_definition, tree);
        }
        SourceUnitMember::InterfaceDefinition(ref interface_definition) => {
            register_interface_definition(interface_definition, tree);
        }
        SourceUnitMember::LibraryDefinition(ref library_definition) => {
            register_library_definition(library_definition, tree);
        }
        SourceUnitMember::StructDefinition(ref struct_definition) => {
            register_struct_definition(struct_definition, tree);
        }
        SourceUnitMember::EnumDefinition(ref enum_definition) => {
            register_enum_definition(enum_definition, tree);
        }
        SourceUnitMember::FunctionDefinition(ref function_definition) => {
            register_function_definition(function_definition, tree);
        }
        SourceUnitMember::ErrorDefinition(ref error_definition) => {
            register_error_definition(error_definition, tree);
        }
        SourceUnitMember::UserDefinedValueTypeDefinition(
            ref user_defined_value_type_definition,
        ) => {
            register_user_defined_value_type_definition(user_defined_value_type_definition, tree);
        }
        SourceUnitMember::UsingDirective(ref using_directive) => {
            register_using_directive(using_directive, tree);
        }
        SourceUnitMember::EventDefinition(ref event_definition) => {
            register_event_definition(event_definition, tree);
        }
        SourceUnitMember::ConstantDefinition(ref constant_definition) => {
            register_constant_definition(constant_definition, tree);
        }
    }
}

pub fn register_pragma(node: &Pragma, tree: &mut TreeIndex) {
    match node {
        Pragma::VersionPragma(ref version_pragma) => {
            register_version_pragma(version_pragma, tree);
        }
        Pragma::AbicoderPragma(ref abicoder_pragma) => {
            register_abicoder_pragma(abicoder_pragma, tree);
        }
        Pragma::ExperimentalPragma(ref experimental_pragma) => {
            register_experimental_pragma(experimental_pragma, tree);
        }
    }
}

pub fn register_abicoder_version(_node: &AbicoderVersion, _tree: &mut TreeIndex) {}

pub fn register_experimental_feature(_node: &ExperimentalFeature, _tree: &mut TreeIndex) {}

pub fn register_version_expression(node: &VersionExpression, tree: &mut TreeIndex) {
    match node {
        VersionExpression::VersionRange(ref version_range) => {
            register_version_range(version_range, tree);
        }
        VersionExpression::VersionTerm(ref version_term) => {
            register_version_term(version_term, tree);
        }
    }
}

pub fn register_version_operator(_node: &VersionOperator, _tree: &mut TreeIndex) {}

pub fn register_version_literal(node: &VersionLiteral, tree: &mut TreeIndex) {
    match node {
        VersionLiteral::SimpleVersionLiteral(ref simple_version_literal) => {
            register_simple_version_literal(simple_version_literal, tree);
        }
        VersionLiteral::SingleQuotedVersionLiteral(_)
        | VersionLiteral::DoubleQuotedVersionLiteral(_) => {}
    }
}

pub fn register_import_clause(node: &ImportClause, tree: &mut TreeIndex) {
    match node {
        ImportClause::PathImport(ref path_import) => {
            register_path_import(path_import, tree);
        }
        ImportClause::NamedImport(ref named_import) => {
            register_named_import(named_import, tree);
        }
        ImportClause::ImportDeconstruction(ref import_deconstruction) => {
            register_import_deconstruction(import_deconstruction, tree);
        }
    }
}

pub fn register_using_clause(node: &UsingClause, tree: &mut TreeIndex) {
    match node {
        UsingClause::IdentifierPath(ref identifier_path) => {
            register_identifier_path(identifier_path, tree);
        }
        UsingClause::UsingDeconstruction(ref using_deconstruction) => {
            register_using_deconstruction(using_deconstruction, tree);
        }
    }
}

pub fn register_using_operator(_node: &UsingOperator, _tree: &mut TreeIndex) {}

pub fn register_using_target(node: &UsingTarget, tree: &mut TreeIndex) {
    match node {
        UsingTarget::TypeName(ref type_name) => {
            register_type_name(type_name, tree);
        }
        UsingTarget::Asterisk => {}
    }
}

pub fn register_contract_member(node: &ContractMember, tree: &mut TreeIndex) {
    match node {
        ContractMember::UsingDirective(ref using_directive) => {
            register_using_directive(using_directive, tree);
        }
        ContractMember::FunctionDefinition(ref function_definition) => {
            register_function_definition(function_definition, tree);
        }
        ContractMember::StructDefinition(ref struct_definition) => {
            register_struct_definition(struct_definition, tree);
        }
        ContractMember::EnumDefinition(ref enum_definition) => {
            register_enum_definition(enum_definition, tree);
        }
        ContractMember::EventDefinition(ref event_definition) => {
            register_event_definition(event_definition, tree);
        }
        ContractMember::ErrorDefinition(ref error_definition) => {
            register_error_definition(error_definition, tree);
        }
        ContractMember::UserDefinedValueTypeDefinition(ref user_defined_value_type_definition) => {
            register_user_defined_value_type_definition(user_defined_value_type_definition, tree);
        }
        ContractMember::StateVariableDefinition(ref state_variable_definition) => {
            register_state_variable_definition(state_variable_definition, tree);
        }
    }
}

pub fn register_type_name(node: &TypeName, tree: &mut TreeIndex) {
    match node {
        TypeName::ArrayTypeName(ref array_type_name) => {
            register_array_type_name(array_type_name, tree);
        }
        TypeName::FunctionType(ref function_type) => {
            register_function_type(function_type, tree);
        }
        TypeName::MappingType(ref mapping_type) => {
            register_mapping_type(mapping_type, tree);
        }
        TypeName::ElementaryType(ref elementary_type) => {
            register_elementary_type(elementary_type, tree);
        }
        TypeName::IdentifierPath(ref identifier_path) => {
            register_identifier_path(identifier_path, tree);
        }
    }
}

pub fn register_mapping_key_type(node: &MappingKeyType, tree: &mut TreeIndex) {
    match node {
        MappingKeyType::ElementaryType(ref elementary_type) => {
            register_elementary_type(elementary_type, tree);
        }
        MappingKeyType::IdentifierPath(ref identifier_path) => {
            register_identifier_path(identifier_path, tree);
        }
    }
}

pub fn register_elementary_type(node: &ElementaryType, tree: &mut TreeIndex) {
    match node {
        ElementaryType::AddressType(ref address_type) => {
            register_address_type(address_type, tree);
        }
        ElementaryType::BytesKeyword(_)
        | ElementaryType::IntKeyword(_)
        | ElementaryType::UintKeyword(_)
        | ElementaryType::FixedKeyword(_)
        | ElementaryType::UfixedKeyword(_) => {}
        ElementaryType::BoolKeyword
        | ElementaryType::ByteKeyword
        | ElementaryType::StringKeyword => {}
    }
}

pub fn register_statement(node: &Statement, tree: &mut TreeIndex) {
    match node {
        Statement::IfStatement(ref if_statement) => {
            register_if_statement(if_statement, tree);
        }
        Statement::ForStatement(ref for_statement) => {
            register_for_statement(for_statement, tree);
        }
        Statement::WhileStatement(ref while_statement) => {
            register_while_statement(while_statement, tree);
        }
        Statement::DoWhileStatement(ref do_while_statement) => {
            register_do_while_statement(do_while_statement, tree);
        }
        Statement::ContinueStatement(ref continue_statement) => {
            register_continue_statement(continue_statement, tree);
        }
        Statement::BreakStatement(ref break_statement) => {
            register_break_statement(break_statement, tree);
        }
        Statement::ReturnStatement(ref return_statement) => {
            register_return_statement(return_statement, tree);
        }
        Statement::ThrowStatement(ref throw_statement) => {
            register_throw_statement(throw_statement, tree);
        }
        Statement::EmitStatement(ref emit_statement) => {
            register_emit_statement(emit_statement, tree);
        }
        Statement::TryStatement(ref try_statement) => {
            register_try_statement(try_statement, tree);
        }
        Statement::RevertStatement(ref revert_statement) => {
            register_revert_statement(revert_statement, tree);
        }
        Statement::AssemblyStatement(ref assembly_statement) => {
            register_assembly_statement(assembly_statement, tree);
        }
        Statement::Block(ref block) => {
            register_block(block, tree);
        }
        Statement::UncheckedBlock(ref unchecked_block) => {
            register_unchecked_block(unchecked_block, tree);
        }
        Statement::TupleDeconstructionStatement(ref tuple_deconstruction_statement) => {
            register_tuple_deconstruction_statement(tuple_deconstruction_statement, tree);
        }
        Statement::VariableDeclarationStatement(ref variable_declaration_statement) => {
            register_variable_declaration_statement(variable_declaration_statement, tree);
        }
        Statement::ExpressionStatement(ref expression_statement) => {
            register_expression_statement(expression_statement, tree);
        }
    }
}

pub fn register_tuple_member(node: &TupleMember, tree: &mut TreeIndex) {
    match node {
        TupleMember::TypedTupleMember(ref typed_tuple_member) => {
            register_typed_tuple_member(typed_tuple_member, tree);
        }
        TupleMember::UntypedTupleMember(ref untyped_tuple_member) => {
            register_untyped_tuple_member(untyped_tuple_member, tree);
        }
    }
}

pub fn register_variable_declaration_type(node: &VariableDeclarationType, tree: &mut TreeIndex) {
    match node {
        VariableDeclarationType::TypeName(ref type_name) => {
            register_type_name(type_name, tree);
        }
        VariableDeclarationType::VarKeyword => {}
    }
}

pub fn register_storage_location(_node: &StorageLocation, _tree: &mut TreeIndex) {}

pub fn register_for_statement_initialization(
    node: &ForStatementInitialization,
    tree: &mut TreeIndex,
) {
    match node {
        ForStatementInitialization::TupleDeconstructionStatement(
            ref tuple_deconstruction_statement,
        ) => {
            register_tuple_deconstruction_statement(tuple_deconstruction_statement, tree);
        }
        ForStatementInitialization::VariableDeclarationStatement(
            ref variable_declaration_statement,
        ) => {
            register_variable_declaration_statement(variable_declaration_statement, tree);
        }
        ForStatementInitialization::ExpressionStatement(ref expression_statement) => {
            register_expression_statement(expression_statement, tree);
        }
        ForStatementInitialization::Semicolon => {}
    }
}

pub fn register_for_statement_condition(node: &ForStatementCondition, tree: &mut TreeIndex) {
    match node {
        ForStatementCondition::ExpressionStatement(ref expression_statement) => {
            register_expression_statement(expression_statement, tree);
        }
        ForStatementCondition::Semicolon => {}
    }
}

pub fn register_expression(node: &Expression, tree: &mut TreeIndex) {
    match node {
        Expression::AssignmentExpression(ref assignment_expression) => {
            register_assignment_expression(assignment_expression, tree);
        }
        Expression::ConditionalExpression(ref conditional_expression) => {
            register_conditional_expression(conditional_expression, tree);
        }
        Expression::OrExpression(ref or_expression) => {
            register_or_expression(or_expression, tree);
        }
        Expression::AndExpression(ref and_expression) => {
            register_and_expression(and_expression, tree);
        }
        Expression::EqualityExpression(ref equality_expression) => {
            register_equality_expression(equality_expression, tree);
        }
        Expression::InequalityExpression(ref inequality_expression) => {
            register_inequality_expression(inequality_expression, tree);
        }
        Expression::BitwiseOrExpression(ref bitwise_or_expression) => {
            register_bitwise_or_expression(bitwise_or_expression, tree);
        }
        Expression::BitwiseXorExpression(ref bitwise_xor_expression) => {
            register_bitwise_xor_expression(bitwise_xor_expression, tree);
        }
        Expression::BitwiseAndExpression(ref bitwise_and_expression) => {
            register_bitwise_and_expression(bitwise_and_expression, tree);
        }
        Expression::ShiftExpression(ref shift_expression) => {
            register_shift_expression(shift_expression, tree);
        }
        Expression::AdditiveExpression(ref additive_expression) => {
            register_additive_expression(additive_expression, tree);
        }
        Expression::MultiplicativeExpression(ref multiplicative_expression) => {
            register_multiplicative_expression(multiplicative_expression, tree);
        }
        Expression::ExponentiationExpression(ref exponentiation_expression) => {
            register_exponentiation_expression(exponentiation_expression, tree);
        }
        Expression::PostfixExpression(ref postfix_expression) => {
            register_postfix_expression(postfix_expression, tree);
        }
        Expression::PrefixExpression(ref prefix_expression) => {
            register_prefix_expression(prefix_expression, tree);
        }
        Expression::FunctionCallExpression(ref function_call_expression) => {
            register_function_call_expression(function_call_expression, tree);
        }
        Expression::CallOptionsExpression(ref call_options_expression) => {
            register_call_options_expression(call_options_expression, tree);
        }
        Expression::MemberAccessExpression(ref member_access_expression) => {
            register_member_access_expression(member_access_expression, tree);
        }
        Expression::IndexAccessExpression(ref index_access_expression) => {
            register_index_access_expression(index_access_expression, tree);
        }
        Expression::NewExpression(ref new_expression) => {
            register_new_expression(new_expression, tree);
        }
        Expression::TupleExpression(ref tuple_expression) => {
            register_tuple_expression(tuple_expression, tree);
        }
        Expression::TypeExpression(ref type_expression) => {
            register_type_expression(type_expression, tree);
        }
        Expression::ArrayExpression(ref array_expression) => {
            register_array_expression(array_expression, tree);
        }
        Expression::HexNumberExpression(ref hex_number_expression) => {
            register_hex_number_expression(hex_number_expression, tree);
        }
        Expression::DecimalNumberExpression(ref decimal_number_expression) => {
            register_decimal_number_expression(decimal_number_expression, tree);
        }
        Expression::StringExpression(ref string_expression) => {
            register_string_expression(string_expression, tree);
        }
        Expression::ElementaryType(ref elementary_type) => {
            register_elementary_type(elementary_type, tree);
        }
        Expression::Identifier(_) => {}
        Expression::PayableKeyword
        | Expression::ThisKeyword
        | Expression::SuperKeyword
        | Expression::TrueKeyword
        | Expression::FalseKeyword => {}
    }
}

pub fn register_arguments_declaration(node: &ArgumentsDeclaration, tree: &mut TreeIndex) {
    match node {
        ArgumentsDeclaration::PositionalArguments(ref positional_arguments) => {
            register_positional_arguments(positional_arguments, tree);
        }
        ArgumentsDeclaration::NamedArguments(ref named_arguments) => {
            register_named_arguments(named_arguments, tree);
        }
    }
}

pub fn register_number_unit(_node: &NumberUnit, _tree: &mut TreeIndex) {}

pub fn register_string_expression(node: &StringExpression, tree: &mut TreeIndex) {
    match node {
        StringExpression::Strings(ref strings) => {
            register_strings(strings, tree);
        }
        StringExpression::HexStrings(ref hex_strings) => {
            register_hex_strings(hex_strings, tree);
        }
        StringExpression::UnicodeStrings(ref unicode_strings) => {
            register_unicode_strings(unicode_strings, tree);
        }
    }
}

pub fn register_yul_statement(node: &YulStatement, tree: &mut TreeIndex) {
    match node {
        YulStatement::YulBlock(ref yul_block) => {
            register_yul_block(yul_block, tree);
        }
        YulStatement::YulFunctionDefinition(ref yul_function_definition) => {
            register_yul_function_definition(yul_function_definition, tree);
        }
        YulStatement::YulStackAssignmentStatement(ref yul_stack_assignment_statement) => {
            register_yul_stack_assignment_statement(yul_stack_assignment_statement, tree);
        }
        YulStatement::YulIfStatement(ref yul_if_statement) => {
            register_yul_if_statement(yul_if_statement, tree);
        }
        YulStatement::YulForStatement(ref yul_for_statement) => {
            register_yul_for_statement(yul_for_statement, tree);
        }
        YulStatement::YulSwitchStatement(ref yul_switch_statement) => {
            register_yul_switch_statement(yul_switch_statement, tree);
        }
        YulStatement::YulLeaveStatement(ref yul_leave_statement) => {
            register_yul_leave_statement(yul_leave_statement, tree);
        }
        YulStatement::YulBreakStatement(ref yul_break_statement) => {
            register_yul_break_statement(yul_break_statement, tree);
        }
        YulStatement::YulContinueStatement(ref yul_continue_statement) => {
            register_yul_continue_statement(yul_continue_statement, tree);
        }
        YulStatement::YulVariableAssignmentStatement(ref yul_variable_assignment_statement) => {
            register_yul_variable_assignment_statement(yul_variable_assignment_statement, tree);
        }
        YulStatement::YulLabel(ref yul_label) => {
            register_yul_label(yul_label, tree);
        }
        YulStatement::YulVariableDeclarationStatement(ref yul_variable_declaration_statement) => {
            register_yul_variable_declaration_statement(yul_variable_declaration_statement, tree);
        }
        YulStatement::YulExpression(ref yul_expression) => {
            register_yul_expression(yul_expression, tree);
        }
    }
}

pub fn register_yul_assignment_operator(node: &YulAssignmentOperator, tree: &mut TreeIndex) {
    match node {
        YulAssignmentOperator::YulColonAndEqual(ref yul_colon_and_equal) => {
            register_yul_colon_and_equal(yul_colon_and_equal, tree);
        }
        YulAssignmentOperator::ColonEqual => {}
    }
}

pub fn register_yul_stack_assignment_operator(
    node: &YulStackAssignmentOperator,
    tree: &mut TreeIndex,
) {
    match node {
        YulStackAssignmentOperator::YulEqualAndColon(ref yul_equal_and_colon) => {
            register_yul_equal_and_colon(yul_equal_and_colon, tree);
        }
        YulStackAssignmentOperator::EqualColon => {}
    }
}

pub fn register_yul_switch_case(node: &YulSwitchCase, tree: &mut TreeIndex) {
    match node {
        YulSwitchCase::YulDefaultCase(ref yul_default_case) => {
            register_yul_default_case(yul_default_case, tree);
        }
        YulSwitchCase::YulValueCase(ref yul_value_case) => {
            register_yul_value_case(yul_value_case, tree);
        }
    }
}

pub fn register_yul_expression(node: &YulExpression, tree: &mut TreeIndex) {
    match node {
        YulExpression::YulFunctionCallExpression(ref yul_function_call_expression) => {
            register_yul_function_call_expression(yul_function_call_expression, tree);
        }
        YulExpression::YulLiteral(ref yul_literal) => {
            register_yul_literal(yul_literal, tree);
        }
        YulExpression::YulPath(ref yul_path) => {
            register_yul_path(yul_path, tree);
        }
    }
}

pub fn register_yul_literal(_node: &YulLiteral, _tree: &mut TreeIndex) {}

pub fn register_function_kind(_node: &FunctionKind, _tree: &mut TreeIndex) {}

pub fn register_function_visibility(_node: &FunctionVisibility, _tree: &mut TreeIndex) {}

pub fn register_function_mutability(_node: &FunctionMutability, _tree: &mut TreeIndex) {}

pub fn register_state_variable_visibility(_node: &StateVariableVisibility, _tree: &mut TreeIndex) {}

pub fn register_state_variable_mutability(_node: &StateVariableMutability, _tree: &mut TreeIndex) {}

//
// Repeated & Separated
//

#[inline]
fn register_source_unit_members(items: &[SourceUnitMember], tree: &mut TreeIndex) {
    for item in items {
        register_source_unit_member(item, tree);
    }
}

#[inline]
fn register_version_expression_sets(items: &[VersionExpressionSet], tree: &mut TreeIndex) {
    for item in items {
        register_version_expression_set(item, tree);
    }
}

#[inline]
fn register_version_expression_set(items: &[VersionExpression], tree: &mut TreeIndex) {
    for item in items {
        register_version_expression(item, tree);
    }
}

#[inline]
fn register_simple_version_literal(_items: &[Rc<SyntaxNode>], _tree: &mut TreeIndex) {}

#[inline]
fn register_import_deconstruction_symbols(
    items: &[ImportDeconstructionSymbol],
    tree: &mut TreeIndex,
) {
    for item in items {
        register_import_deconstruction_symbol(item, tree);
    }
}

#[inline]
fn register_using_deconstruction_symbols(
    items: &[UsingDeconstructionSymbol],
    tree: &mut TreeIndex,
) {
    for item in items {
        register_using_deconstruction_symbol(item, tree);
    }
}

#[inline]
fn register_inheritance_types(items: &[InheritanceType], tree: &mut TreeIndex) {
    for item in items {
        register_inheritance_type(item, tree);
    }
}

#[inline]
fn register_contract_members(items: &[ContractMember], tree: &mut TreeIndex) {
    for item in items {
        register_contract_member(item, tree);
    }
}

#[inline]
fn register_interface_members(items: &[ContractMember], tree: &mut TreeIndex) {
    for item in items {
        register_contract_member(item, tree);
    }
}

#[inline]
fn register_library_members(items: &[ContractMember], tree: &mut TreeIndex) {
    for item in items {
        register_contract_member(item, tree);
    }
}

#[inline]
fn register_struct_members(items: &[StructMember], tree: &mut TreeIndex) {
    for item in items {
        register_struct_member(item, tree);
    }
}

#[inline]
fn register_enum_members(_items: &[Rc<SyntaxNode>], _tree: &mut TreeIndex) {}

#[inline]
fn register_parameters(items: &[Parameter], tree: &mut TreeIndex) {
    for item in items {
        register_parameter(item, tree);
    }
}

#[inline]
fn register_override_paths(items: &[IdentifierPath], tree: &mut TreeIndex) {
    for item in items {
        register_identifier_path(item, tree);
    }
}

#[inline]
fn register_event_parameters(items: &[EventParameter], tree: &mut TreeIndex) {
    for item in items {
        register_event_parameter(item, tree);
    }
}

#[inline]
fn register_error_parameters(items: &[ErrorParameter], tree: &mut TreeIndex) {
    for item in items {
        register_error_parameter(item, tree);
    }
}

#[inline]
fn register_statements(items: &[Statement], tree: &mut TreeIndex) {
    for item in items {
        register_statement(item, tree);
    }
}

#[inline]
fn register_tuple_deconstruction_elements(
    items: &[TupleDeconstructionElement],
    tree: &mut TreeIndex,
) {
    for item in items {
        register_tuple_deconstruction_element(item, tree);
    }
}

#[inline]
fn register_catch_clauses(items: &[CatchClause], tree: &mut TreeIndex) {
    for item in items {
        register_catch_clause(item, tree);
    }
}

#[inline]
fn register_positional_arguments(items: &[Expression], tree: &mut TreeIndex) {
    for item in items {
        register_expression(item, tree);
    }
}

#[inline]
fn register_named_arguments(items: &[NamedArgument], tree: &mut TreeIndex) {
    for item in items {
        register_named_argument(item, tree);
    }
}

#[inline]
fn register_call_options(items: &[NamedArgument], tree: &mut TreeIndex) {
    for item in items {
        register_named_argument(item, tree);
    }
}

#[inline]
fn register_tuple_values(items: &[TupleValue], tree: &mut TreeIndex) {
    for item in items {
        register_tuple_value(item, tree);
    }
}

#[inline]
fn register_array_values(items: &[Expression], tree: &mut TreeIndex) {
    for item in items {
        register_expression(item, tree);
    }
}

#[inline]
fn register_identifier_path(_items: &[Rc<SyntaxNode>], _tree: &mut TreeIndex) {}

#[inline]
fn register_yul_statements(items: &[YulStatement], tree: &mut TreeIndex) {
    for item in items {
        register_yul_statement(item, tree);
    }
}

#[inline]
fn register_yul_parameters(_items: &[Rc<SyntaxNode>], _tree: &mut TreeIndex) {}

#[inline]
fn register_yul_variable_names(_items: &[Rc<SyntaxNode>], _tree: &mut TreeIndex) {}

#[inline]
fn register_yul_switch_cases(items: &[YulSwitchCase], tree: &mut TreeIndex) {
    for item in items {
        register_yul_switch_case(item, tree);
    }
}

#[inline]
fn register_yul_arguments(items: &[YulExpression], tree: &mut TreeIndex) {
    for item in items {
        register_yul_expression(item, tree);
    }
}

#[inline]
fn register_yul_paths(items: &[YulPath], tree: &mut TreeIndex) {
    for item in items {
        register_yul_path(item, tree);
    }
}

#[inline]
fn register_yul_path(_items: &[Rc<SyntaxNode>], _tree: &mut TreeIndex) {}

#[inline]
fn register_modifier_invocations(items: &[ModifierInvocation], tree: &mut TreeIndex) {
    for item in items {
        register_modifier_invocation(item, tree);
    }
}

#[inline]
fn register_strings(_items: &[Rc<SyntaxNode>], _tree: &mut TreeIndex) {}

#[inline]
fn register_hex_strings(_items: &[Rc<SyntaxNode>], _tree: &mut TreeIndex) {}

#[inline]
fn register_unicode_strings(_items: &[Rc<SyntaxNode>], _tree: &mut TreeIndex) {}

#[inline]
fn register_assembly_flags(_items: &[Rc<SyntaxNode>], _tree: &mut TreeIndex) {}
