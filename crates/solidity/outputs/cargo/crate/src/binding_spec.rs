use crate::binding;
use crate::binding::ScopeGraphAction::*;
use crate::cst::NameLinkType::*;
use crate::kinds::RuleKind::*;
use crate::*;

pub fn make_binding_specification() -> binding::BindingSpecification {
    let mut bs = binding::BindingSpecification::default();
    bs.insert(
        ContractDefinition,
        [
            AddDefinitionAndPushAssociatedScope(token! { Identifier }),
            VisitAllChildren,
            PopScope,
        ],
    );
    bs.insert(
        InterfaceDefinition,
        [
            AddDefinitionAndPushAssociatedScope(token! { Identifier }),
            VisitAllChildren,
            PopScope,
        ],
    );
    bs.insert(
        LibraryDefinition,
        [
            AddDefinitionAndPushAssociatedScope(token! { Identifier }),
            VisitAllChildren,
            PopScope,
        ],
    );

    bs.insert(
        rule! { InheritanceType IdentifierPath },
        [AddNameLink(Inheritance, token! { last Identifier })],
    );

    bs.insert(Block, [PushScope, VisitAllChildren, PopScope]);

    bs.insert(
        FunctionDefinition,
        [
            Visit(FunctionAttributes),
            PushScope,
            Visit(ParametersDeclaration), // TODO: Which of these should go first?
            Visit(ReturnsDeclaration),
            Visit(Block),
            PopScope,
        ],
    );

    bs.insert(
        FallbackFunctionDefinition,
        [
            Visit(FallbackFunctionAttributes),
            PushScope,
            Visit(ParametersDeclaration), // TODO: Which of these should go first?
            Visit(ReturnsDeclaration),
            Visit(Block),
            PopScope,
        ],
    );

    bs.insert(
        ReceiveFunctionDefinition,
        [
            Visit(ReceiveFunctionAttributes),
            PushScope,
            Visit(ParametersDeclaration), // TODO: Which of these should go first?
            Visit(ReturnsDeclaration),
            Visit(Block),
            PopScope,
        ],
    );

    bs.insert(
        UnnamedFunctionDefinition,
        [
            Visit(ReceiveFunctionAttributes),
            PushScope,
            Visit(ParametersDeclaration), // TODO: Which of these should go first?
            Visit(ReturnsDeclaration),
            Visit(Block),
            PopScope,
        ],
    );

    bs.insert(
        ConstructorDefinition,
        [
            Visit(ConstructorAttributes),
            PushScope,
            Visit(ParametersDeclaration),
            Visit(Block),
            PopScope,
        ],
    );

    bs.insert(
        EnumDefinition,
        [
            AddDefinitionAndPushAssociatedScope(token! { Identifier }),
            AddDefinition(token! { EnumMembers Identifier }),
            PopScope,
        ],
    );

    bs.insert(
        StructDefinition,
        [
            AddDefinitionAndPushAssociatedScope(token! { Identifier }),
            VisitAllChildren,
            PopScope,
        ],
    );

    bs.insert(
        StructMember,
        [AddDefinition(token! { Identifier }), Visit(TypeName)],
    );

    bs.insert(
        ErrorDefinition,
        [
            AddDefinitionAndPushAssociatedScope(token! { Identifier }),
            VisitAllChildren,
            PopScope,
        ],
    );

    bs.insert(
        ErrorParameter,
        [AddDefinition(token! { Identifier }), Visit(TypeName)],
    );

    bs.insert(
        EventDefinition,
        [
            AddDefinitionAndPushAssociatedScope(token! { Identifier }),
            VisitAllChildren,
            PopScope,
        ],
    );

    bs.insert(
        EventParameter,
        [AddDefinition(token! { Identifier }), Visit(TypeName)],
    );

    bs.insert(
        Parameter,
        [AddDefinition(token! { Identifier }), Visit(TypeName)],
    );

    // This is a hack
    // TODO: Change the simple reference in Expression to an explicit rule
    bs.insert(
        Expression,
        [AddReference(token! { Identifier}), VisitAllChildren],
    );

    // TODO: CatchClause
    // TODO: DeconstructionImport
    // TODO: DeconstructionImport
    // TODO: TypeName
    // TODO: ModifierInvocation
    // TODO: Import Directive
    // TODO: IdentifierPath
    // TODO: MemberAccessOperator
    // TODO: NamedImport
    // TODO: PathImport
    // TODO: EmitStatement
    // TODO: RevertStatement
    // TODO: ConstantDefinition
    // TODO: StateVariableDefinition
    // TODO: TryStatement
    // TODO: TupleDeconstructionStatement
    // TODO: UserDefinedValueTypeDefinition
    // TODO: UsingDirective
    // TODO: VariableDeclaration

    // TODO: YUL

    bs
}
