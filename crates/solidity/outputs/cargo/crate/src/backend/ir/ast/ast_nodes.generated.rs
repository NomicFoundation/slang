// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

//
// Sequences:
//

pub type SourceUnit = Rc<SourceUnitStruct>;

pub struct SourceUnitStruct {
    ir_node: input_ir::SourceUnit,
    semantic: Rc<SemanticAnalysis>,
}

impl SourceUnitStruct {
    pub(crate) fn create(ir_node: &input_ir::SourceUnit, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn members(&self) -> impl Iterator<Item = SourceUnitMember> + use<'_> {
        self.ir_node
            .members
            .iter()
            .map(|child| Rc::new(SourceUnitMemberStruct::create(child, &self.semantic)))
    }
}

pub type PragmaDirective = Rc<PragmaDirectiveStruct>;

pub struct PragmaDirectiveStruct {
    ir_node: input_ir::PragmaDirective,
    semantic: Rc<SemanticAnalysis>,
}

impl PragmaDirectiveStruct {
    pub(crate) fn create(
        ir_node: &input_ir::PragmaDirective,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn pragma(&self) -> Pragma {
        Rc::new(PragmaStruct::create(&self.ir_node.pragma, &self.semantic))
    }
}

pub type AbicoderPragma = Rc<AbicoderPragmaStruct>;

pub struct AbicoderPragmaStruct {
    ir_node: input_ir::AbicoderPragma,
    semantic: Rc<SemanticAnalysis>,
}

impl AbicoderPragmaStruct {
    pub(crate) fn create(
        ir_node: &input_ir::AbicoderPragma,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn version(&self) -> AbicoderVersion {
        Rc::new(AbicoderVersionStruct::create(
            &self.ir_node.version,
            &self.semantic,
        ))
    }
}

pub type ExperimentalPragma = Rc<ExperimentalPragmaStruct>;

pub struct ExperimentalPragmaStruct {
    ir_node: input_ir::ExperimentalPragma,
    semantic: Rc<SemanticAnalysis>,
}

impl ExperimentalPragmaStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ExperimentalPragma,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn feature(&self) -> ExperimentalFeature {
        Rc::new(ExperimentalFeatureStruct::create(
            &self.ir_node.feature,
            &self.semantic,
        ))
    }
}

pub type VersionPragma = Rc<VersionPragmaStruct>;

pub struct VersionPragmaStruct {
    ir_node: input_ir::VersionPragma,
    semantic: Rc<SemanticAnalysis>,
}

impl VersionPragmaStruct {
    pub(crate) fn create(
        ir_node: &input_ir::VersionPragma,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }
}

pub type VersionRange = Rc<VersionRangeStruct>;

pub struct VersionRangeStruct {
    ir_node: input_ir::VersionRange,
    semantic: Rc<SemanticAnalysis>,
}

impl VersionRangeStruct {
    pub(crate) fn create(
        ir_node: &input_ir::VersionRange,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn start(&self) -> VersionLiteral {
        Rc::new(VersionLiteralStruct::create(
            &self.ir_node.start,
            &self.semantic,
        ))
    }

    pub fn end(&self) -> VersionLiteral {
        Rc::new(VersionLiteralStruct::create(
            &self.ir_node.end,
            &self.semantic,
        ))
    }
}

pub type VersionTerm = Rc<VersionTermStruct>;

pub struct VersionTermStruct {
    ir_node: input_ir::VersionTerm,
    semantic: Rc<SemanticAnalysis>,
}

impl VersionTermStruct {
    pub(crate) fn create(ir_node: &input_ir::VersionTerm, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn operator(&self) -> Option<VersionOperator> {
        self.ir_node
            .operator
            .as_ref()
            .map(|ir_node| Rc::new(VersionOperatorStruct::create(ir_node, &self.semantic)))
    }

    pub fn literal(&self) -> VersionLiteral {
        Rc::new(VersionLiteralStruct::create(
            &self.ir_node.literal,
            &self.semantic,
        ))
    }
}

pub type PathImport = Rc<PathImportStruct>;

pub struct PathImportStruct {
    ir_node: input_ir::PathImport,
    semantic: Rc<SemanticAnalysis>,
}

impl PathImportStruct {
    pub(crate) fn create(ir_node: &input_ir::PathImport, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn alias(&self) -> Option<Identifier> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| Rc::new(IdentifierStruct::create(ir_node, &self.semantic)))
    }

    pub fn path(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.path)
    }
}

pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

pub struct ImportDeconstructionStruct {
    ir_node: input_ir::ImportDeconstruction,
    semantic: Rc<SemanticAnalysis>,
}

impl ImportDeconstructionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ImportDeconstruction,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn symbols(&self) -> impl Iterator<Item = ImportDeconstructionSymbol> + use<'_> {
        self.ir_node.symbols.iter().map(|child| {
            Rc::new(ImportDeconstructionSymbolStruct::create(
                child,
                &self.semantic,
            ))
        })
    }

    pub fn path(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.path)
    }
}

pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

pub struct ImportDeconstructionSymbolStruct {
    ir_node: input_ir::ImportDeconstructionSymbol,
    semantic: Rc<SemanticAnalysis>,
}

impl ImportDeconstructionSymbolStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ImportDeconstructionSymbol,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn alias(&self) -> Option<Identifier> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| Rc::new(IdentifierStruct::create(ir_node, &self.semantic)))
    }
}

pub type UsingDirective = Rc<UsingDirectiveStruct>;

pub struct UsingDirectiveStruct {
    ir_node: input_ir::UsingDirective,
    semantic: Rc<SemanticAnalysis>,
}

impl UsingDirectiveStruct {
    pub(crate) fn create(
        ir_node: &input_ir::UsingDirective,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn clause(&self) -> UsingClause {
        Rc::new(UsingClauseStruct::create(
            &self.ir_node.clause,
            &self.semantic,
        ))
    }

    pub fn target(&self) -> UsingTarget {
        Rc::new(UsingTargetStruct::create(
            &self.ir_node.target,
            &self.semantic,
        ))
    }

    pub fn global_keyword(&self) -> bool {
        self.ir_node.global_keyword
    }
}

pub type UsingDeconstruction = Rc<UsingDeconstructionStruct>;

pub struct UsingDeconstructionStruct {
    ir_node: input_ir::UsingDeconstruction,
    semantic: Rc<SemanticAnalysis>,
}

impl UsingDeconstructionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::UsingDeconstruction,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn symbols(&self) -> impl Iterator<Item = UsingDeconstructionSymbol> + use<'_> {
        self.ir_node.symbols.iter().map(|child| {
            Rc::new(UsingDeconstructionSymbolStruct::create(
                child,
                &self.semantic,
            ))
        })
    }
}

pub type UsingDeconstructionSymbol = Rc<UsingDeconstructionSymbolStruct>;

pub struct UsingDeconstructionSymbolStruct {
    ir_node: input_ir::UsingDeconstructionSymbol,
    semantic: Rc<SemanticAnalysis>,
}

impl UsingDeconstructionSymbolStruct {
    pub(crate) fn create(
        ir_node: &input_ir::UsingDeconstructionSymbol,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> IdentifierPath {
        Rc::new(IdentifierPathStruct::create(
            &self.ir_node.name,
            &self.semantic,
        ))
    }

    pub fn alias(&self) -> Option<UsingOperator> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| Rc::new(UsingOperatorStruct::create(ir_node, &self.semantic)))
    }
}

pub type ContractDefinition = Rc<ContractDefinitionStruct>;

pub struct ContractDefinitionStruct {
    ir_node: input_ir::ContractDefinition,
    semantic: Rc<SemanticAnalysis>,
}

impl ContractDefinitionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ContractDefinition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn abstract_keyword(&self) -> bool {
        self.ir_node.abstract_keyword
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn members(&self) -> impl Iterator<Item = ContractMember> + use<'_> {
        self.ir_node
            .members
            .iter()
            .map(|child| Rc::new(ContractMemberStruct::create(child, &self.semantic)))
    }

    pub fn inheritance_types(&self) -> impl Iterator<Item = InheritanceType> + use<'_> {
        self.ir_node
            .inheritance_types
            .iter()
            .map(|child| Rc::new(InheritanceTypeStruct::create(child, &self.semantic)))
    }

    pub fn storage_layout(&self) -> Option<Expression> {
        self.ir_node
            .storage_layout
            .as_ref()
            .map(|ir_node| Rc::new(ExpressionStruct::create(ir_node, &self.semantic)))
    }
}

pub type InheritanceType = Rc<InheritanceTypeStruct>;

pub struct InheritanceTypeStruct {
    ir_node: input_ir::InheritanceType,
    semantic: Rc<SemanticAnalysis>,
}

impl InheritanceTypeStruct {
    pub(crate) fn create(
        ir_node: &input_ir::InheritanceType,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn type_name(&self) -> IdentifierPath {
        Rc::new(IdentifierPathStruct::create(
            &self.ir_node.type_name,
            &self.semantic,
        ))
    }

    pub fn arguments(&self) -> Option<ArgumentsDeclaration> {
        self.ir_node
            .arguments
            .as_ref()
            .map(|ir_node| Rc::new(ArgumentsDeclarationStruct::create(ir_node, &self.semantic)))
    }
}

pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

pub struct InterfaceDefinitionStruct {
    ir_node: input_ir::InterfaceDefinition,
    semantic: Rc<SemanticAnalysis>,
}

impl InterfaceDefinitionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::InterfaceDefinition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn members(&self) -> impl Iterator<Item = ContractMember> + use<'_> {
        self.ir_node
            .members
            .iter()
            .map(|child| Rc::new(ContractMemberStruct::create(child, &self.semantic)))
    }
}

pub type LibraryDefinition = Rc<LibraryDefinitionStruct>;

pub struct LibraryDefinitionStruct {
    ir_node: input_ir::LibraryDefinition,
    semantic: Rc<SemanticAnalysis>,
}

impl LibraryDefinitionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::LibraryDefinition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn members(&self) -> impl Iterator<Item = ContractMember> + use<'_> {
        self.ir_node
            .members
            .iter()
            .map(|child| Rc::new(ContractMemberStruct::create(child, &self.semantic)))
    }
}

pub type StructDefinition = Rc<StructDefinitionStruct>;

pub struct StructDefinitionStruct {
    ir_node: input_ir::StructDefinition,
    semantic: Rc<SemanticAnalysis>,
}

impl StructDefinitionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::StructDefinition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn members(&self) -> impl Iterator<Item = StructMember> + use<'_> {
        self.ir_node
            .members
            .iter()
            .map(|child| Rc::new(StructMemberStruct::create(child, &self.semantic)))
    }
}

pub type StructMember = Rc<StructMemberStruct>;

pub struct StructMemberStruct {
    ir_node: input_ir::StructMember,
    semantic: Rc<SemanticAnalysis>,
}

impl StructMemberStruct {
    pub(crate) fn create(
        ir_node: &input_ir::StructMember,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn type_name(&self) -> TypeName {
        Rc::new(TypeNameStruct::create(
            &self.ir_node.type_name,
            &self.semantic,
        ))
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }
}

pub type EnumDefinition = Rc<EnumDefinitionStruct>;

pub struct EnumDefinitionStruct {
    ir_node: input_ir::EnumDefinition,
    semantic: Rc<SemanticAnalysis>,
}

impl EnumDefinitionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::EnumDefinition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn members(&self) -> impl Iterator<Item = Rc<TerminalNode>> + use<'_> {
        self.ir_node.members.iter().map(Rc::clone)
    }
}

pub type ConstantDefinition = Rc<ConstantDefinitionStruct>;

pub struct ConstantDefinitionStruct {
    ir_node: input_ir::ConstantDefinition,
    semantic: Rc<SemanticAnalysis>,
}

impl ConstantDefinitionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ConstantDefinition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn type_name(&self) -> TypeName {
        Rc::new(TypeNameStruct::create(
            &self.ir_node.type_name,
            &self.semantic,
        ))
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn visibility(&self) -> Option<StateVariableVisibility> {
        self.ir_node.visibility.as_ref().map(|ir_node| {
            Rc::new(StateVariableVisibilityStruct::create(
                ir_node,
                &self.semantic,
            ))
        })
    }

    pub fn value(&self) -> Option<Expression> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| Rc::new(ExpressionStruct::create(ir_node, &self.semantic)))
    }
}

pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

pub struct StateVariableDefinitionStruct {
    ir_node: input_ir::StateVariableDefinition,
    semantic: Rc<SemanticAnalysis>,
}

impl StateVariableDefinitionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::StateVariableDefinition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn type_name(&self) -> TypeName {
        Rc::new(TypeNameStruct::create(
            &self.ir_node.type_name,
            &self.semantic,
        ))
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn value(&self) -> Option<Expression> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| Rc::new(ExpressionStruct::create(ir_node, &self.semantic)))
    }

    pub fn visibility(&self) -> StateVariableVisibility {
        Rc::new(StateVariableVisibilityStruct::create(
            &self.ir_node.visibility,
            &self.semantic,
        ))
    }

    pub fn mutability(&self) -> StateVariableMutability {
        Rc::new(StateVariableMutabilityStruct::create(
            &self.ir_node.mutability,
            &self.semantic,
        ))
    }
}

pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

pub struct FunctionDefinitionStruct {
    ir_node: input_ir::FunctionDefinition,
    semantic: Rc<SemanticAnalysis>,
}

impl FunctionDefinitionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::FunctionDefinition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn parameters(&self) -> impl Iterator<Item = Parameter> + use<'_> {
        self.ir_node
            .parameters
            .iter()
            .map(|child| Rc::new(ParameterStruct::create(child, &self.semantic)))
    }

    pub fn kind(&self) -> FunctionKind {
        Rc::new(FunctionKindStruct::create(
            &self.ir_node.kind,
            &self.semantic,
        ))
    }

    pub fn name(&self) -> Option<Identifier> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| Rc::new(IdentifierStruct::create(ir_node, &self.semantic)))
    }

    pub fn body(&self) -> Option<Block> {
        self.ir_node
            .body
            .as_ref()
            .map(|ir_node| Rc::new(BlockStruct::create(ir_node, &self.semantic)))
    }

    pub fn visibility(&self) -> FunctionVisibility {
        Rc::new(FunctionVisibilityStruct::create(
            &self.ir_node.visibility,
            &self.semantic,
        ))
    }

    pub fn mutability(&self) -> FunctionMutability {
        Rc::new(FunctionMutabilityStruct::create(
            &self.ir_node.mutability,
            &self.semantic,
        ))
    }

    pub fn virtual_keyword(&self) -> bool {
        self.ir_node.virtual_keyword
    }

    pub fn modifier_invocations(&self) -> impl Iterator<Item = ModifierInvocation> + use<'_> {
        self.ir_node
            .modifier_invocations
            .iter()
            .map(|child| Rc::new(ModifierInvocationStruct::create(child, &self.semantic)))
    }
}

pub type Parameter = Rc<ParameterStruct>;

pub struct ParameterStruct {
    ir_node: input_ir::Parameter,
    semantic: Rc<SemanticAnalysis>,
}

impl ParameterStruct {
    pub(crate) fn create(ir_node: &input_ir::Parameter, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn type_name(&self) -> TypeName {
        Rc::new(TypeNameStruct::create(
            &self.ir_node.type_name,
            &self.semantic,
        ))
    }

    pub fn storage_location(&self) -> Option<StorageLocation> {
        self.ir_node
            .storage_location
            .as_ref()
            .map(|ir_node| Rc::new(StorageLocationStruct::create(ir_node, &self.semantic)))
    }

    pub fn name(&self) -> Option<Identifier> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| Rc::new(IdentifierStruct::create(ir_node, &self.semantic)))
    }

    pub fn indexed(&self) -> bool {
        self.ir_node.indexed
    }
}

pub type OverrideSpecifier = Rc<OverrideSpecifierStruct>;

pub struct OverrideSpecifierStruct {
    ir_node: input_ir::OverrideSpecifier,
    semantic: Rc<SemanticAnalysis>,
}

impl OverrideSpecifierStruct {
    pub(crate) fn create(
        ir_node: &input_ir::OverrideSpecifier,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }
}

pub type ModifierInvocation = Rc<ModifierInvocationStruct>;

pub struct ModifierInvocationStruct {
    ir_node: input_ir::ModifierInvocation,
    semantic: Rc<SemanticAnalysis>,
}

impl ModifierInvocationStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ModifierInvocation,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> IdentifierPath {
        Rc::new(IdentifierPathStruct::create(
            &self.ir_node.name,
            &self.semantic,
        ))
    }

    pub fn arguments(&self) -> Option<ArgumentsDeclaration> {
        self.ir_node
            .arguments
            .as_ref()
            .map(|ir_node| Rc::new(ArgumentsDeclarationStruct::create(ir_node, &self.semantic)))
    }
}

pub type EventDefinition = Rc<EventDefinitionStruct>;

pub struct EventDefinitionStruct {
    ir_node: input_ir::EventDefinition,
    semantic: Rc<SemanticAnalysis>,
}

impl EventDefinitionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::EventDefinition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn anonymous_keyword(&self) -> bool {
        self.ir_node.anonymous_keyword
    }

    pub fn parameters(&self) -> impl Iterator<Item = Parameter> + use<'_> {
        self.ir_node
            .parameters
            .iter()
            .map(|child| Rc::new(ParameterStruct::create(child, &self.semantic)))
    }
}

pub type UserDefinedValueTypeDefinition = Rc<UserDefinedValueTypeDefinitionStruct>;

pub struct UserDefinedValueTypeDefinitionStruct {
    ir_node: input_ir::UserDefinedValueTypeDefinition,
    semantic: Rc<SemanticAnalysis>,
}

impl UserDefinedValueTypeDefinitionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::UserDefinedValueTypeDefinition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn value_type(&self) -> ElementaryType {
        Rc::new(ElementaryTypeStruct::create(
            &self.ir_node.value_type,
            &self.semantic,
        ))
    }
}

pub type ErrorDefinition = Rc<ErrorDefinitionStruct>;

pub struct ErrorDefinitionStruct {
    ir_node: input_ir::ErrorDefinition,
    semantic: Rc<SemanticAnalysis>,
}

impl ErrorDefinitionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ErrorDefinition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn parameters(&self) -> impl Iterator<Item = Parameter> + use<'_> {
        self.ir_node
            .parameters
            .iter()
            .map(|child| Rc::new(ParameterStruct::create(child, &self.semantic)))
    }
}

pub type ArrayTypeName = Rc<ArrayTypeNameStruct>;

pub struct ArrayTypeNameStruct {
    ir_node: input_ir::ArrayTypeName,
    semantic: Rc<SemanticAnalysis>,
}

impl ArrayTypeNameStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ArrayTypeName,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn operand(&self) -> TypeName {
        Rc::new(TypeNameStruct::create(
            &self.ir_node.operand,
            &self.semantic,
        ))
    }

    pub fn index(&self) -> Option<Expression> {
        self.ir_node
            .index
            .as_ref()
            .map(|ir_node| Rc::new(ExpressionStruct::create(ir_node, &self.semantic)))
    }
}

pub type FunctionType = Rc<FunctionTypeStruct>;

pub struct FunctionTypeStruct {
    ir_node: input_ir::FunctionType,
    semantic: Rc<SemanticAnalysis>,
}

impl FunctionTypeStruct {
    pub(crate) fn create(
        ir_node: &input_ir::FunctionType,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn parameters(&self) -> impl Iterator<Item = Parameter> + use<'_> {
        self.ir_node
            .parameters
            .iter()
            .map(|child| Rc::new(ParameterStruct::create(child, &self.semantic)))
    }

    pub fn visibility(&self) -> FunctionVisibility {
        Rc::new(FunctionVisibilityStruct::create(
            &self.ir_node.visibility,
            &self.semantic,
        ))
    }

    pub fn mutability(&self) -> FunctionMutability {
        Rc::new(FunctionMutabilityStruct::create(
            &self.ir_node.mutability,
            &self.semantic,
        ))
    }
}

pub type MappingType = Rc<MappingTypeStruct>;

pub struct MappingTypeStruct {
    ir_node: input_ir::MappingType,
    semantic: Rc<SemanticAnalysis>,
}

impl MappingTypeStruct {
    pub(crate) fn create(ir_node: &input_ir::MappingType, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn key_type(&self) -> Parameter {
        Rc::new(ParameterStruct::create(
            &self.ir_node.key_type,
            &self.semantic,
        ))
    }

    pub fn value_type(&self) -> Parameter {
        Rc::new(ParameterStruct::create(
            &self.ir_node.value_type,
            &self.semantic,
        ))
    }
}

pub type AddressType = Rc<AddressTypeStruct>;

pub struct AddressTypeStruct {
    ir_node: input_ir::AddressType,
    semantic: Rc<SemanticAnalysis>,
}

impl AddressTypeStruct {
    pub(crate) fn create(ir_node: &input_ir::AddressType, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn payable_keyword(&self) -> bool {
        self.ir_node.payable_keyword
    }
}

pub type Block = Rc<BlockStruct>;

pub struct BlockStruct {
    ir_node: input_ir::Block,
    semantic: Rc<SemanticAnalysis>,
}

impl BlockStruct {
    pub(crate) fn create(ir_node: &input_ir::Block, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn statements(&self) -> impl Iterator<Item = Statement> + use<'_> {
        self.ir_node
            .statements
            .iter()
            .map(|child| Rc::new(StatementStruct::create(child, &self.semantic)))
    }
}

pub type UncheckedBlock = Rc<UncheckedBlockStruct>;

pub struct UncheckedBlockStruct {
    ir_node: input_ir::UncheckedBlock,
    semantic: Rc<SemanticAnalysis>,
}

impl UncheckedBlockStruct {
    pub(crate) fn create(
        ir_node: &input_ir::UncheckedBlock,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn block(&self) -> Block {
        Rc::new(BlockStruct::create(&self.ir_node.block, &self.semantic))
    }
}

pub type ExpressionStatement = Rc<ExpressionStatementStruct>;

pub struct ExpressionStatementStruct {
    ir_node: input_ir::ExpressionStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl ExpressionStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ExpressionStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn expression(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.expression,
            &self.semantic,
        ))
    }
}

pub type AssemblyStatement = Rc<AssemblyStatementStruct>;

pub struct AssemblyStatementStruct {
    ir_node: input_ir::AssemblyStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl AssemblyStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::AssemblyStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn body(&self) -> YulBlock {
        Rc::new(YulBlockStruct::create(&self.ir_node.body, &self.semantic))
    }

    pub fn flags(&self) -> impl Iterator<Item = Rc<TerminalNode>> + use<'_> {
        self.ir_node.flags.iter().map(Rc::clone)
    }

    pub fn label(&self) -> Option<Rc<TerminalNode>> {
        self.ir_node.label.as_ref().map(Rc::clone)
    }
}

pub type TupleDeconstructionStatement = Rc<TupleDeconstructionStatementStruct>;

pub struct TupleDeconstructionStatementStruct {
    ir_node: input_ir::TupleDeconstructionStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl TupleDeconstructionStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::TupleDeconstructionStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn expression(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.expression,
            &self.semantic,
        ))
    }

    pub fn members(&self) -> impl Iterator<Item = TupleDeconstructionMember> + use<'_> {
        self.ir_node.members.iter().map(|child| {
            Rc::new(TupleDeconstructionMemberStruct::create(
                child,
                &self.semantic,
            ))
        })
    }
}

pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

pub struct VariableDeclarationStatementStruct {
    ir_node: input_ir::VariableDeclarationStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl VariableDeclarationStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::VariableDeclarationStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn storage_location(&self) -> Option<StorageLocation> {
        self.ir_node
            .storage_location
            .as_ref()
            .map(|ir_node| Rc::new(StorageLocationStruct::create(ir_node, &self.semantic)))
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn value(&self) -> Option<Expression> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| Rc::new(ExpressionStruct::create(ir_node, &self.semantic)))
    }

    pub fn type_name(&self) -> Option<TypeName> {
        self.ir_node
            .type_name
            .as_ref()
            .map(|ir_node| Rc::new(TypeNameStruct::create(ir_node, &self.semantic)))
    }
}

pub type IfStatement = Rc<IfStatementStruct>;

pub struct IfStatementStruct {
    ir_node: input_ir::IfStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl IfStatementStruct {
    pub(crate) fn create(ir_node: &input_ir::IfStatement, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn condition(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.condition,
            &self.semantic,
        ))
    }

    pub fn body(&self) -> Statement {
        Rc::new(StatementStruct::create(&self.ir_node.body, &self.semantic))
    }

    pub fn else_branch(&self) -> Option<Statement> {
        self.ir_node
            .else_branch
            .as_ref()
            .map(|ir_node| Rc::new(StatementStruct::create(ir_node, &self.semantic)))
    }
}

pub type ForStatement = Rc<ForStatementStruct>;

pub struct ForStatementStruct {
    ir_node: input_ir::ForStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl ForStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ForStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn initialization(&self) -> ForStatementInitialization {
        Rc::new(ForStatementInitializationStruct::create(
            &self.ir_node.initialization,
            &self.semantic,
        ))
    }

    pub fn condition(&self) -> ForStatementCondition {
        Rc::new(ForStatementConditionStruct::create(
            &self.ir_node.condition,
            &self.semantic,
        ))
    }

    pub fn iterator(&self) -> Option<Expression> {
        self.ir_node
            .iterator
            .as_ref()
            .map(|ir_node| Rc::new(ExpressionStruct::create(ir_node, &self.semantic)))
    }

    pub fn body(&self) -> Statement {
        Rc::new(StatementStruct::create(&self.ir_node.body, &self.semantic))
    }
}

pub type WhileStatement = Rc<WhileStatementStruct>;

pub struct WhileStatementStruct {
    ir_node: input_ir::WhileStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl WhileStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::WhileStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn condition(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.condition,
            &self.semantic,
        ))
    }

    pub fn body(&self) -> Statement {
        Rc::new(StatementStruct::create(&self.ir_node.body, &self.semantic))
    }
}

pub type DoWhileStatement = Rc<DoWhileStatementStruct>;

pub struct DoWhileStatementStruct {
    ir_node: input_ir::DoWhileStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl DoWhileStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::DoWhileStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn body(&self) -> Statement {
        Rc::new(StatementStruct::create(&self.ir_node.body, &self.semantic))
    }

    pub fn condition(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.condition,
            &self.semantic,
        ))
    }
}

pub type ContinueStatement = Rc<ContinueStatementStruct>;

pub struct ContinueStatementStruct {
    ir_node: input_ir::ContinueStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl ContinueStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ContinueStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }
}

pub type BreakStatement = Rc<BreakStatementStruct>;

pub struct BreakStatementStruct {
    ir_node: input_ir::BreakStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl BreakStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::BreakStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }
}

pub type ReturnStatement = Rc<ReturnStatementStruct>;

pub struct ReturnStatementStruct {
    ir_node: input_ir::ReturnStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl ReturnStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ReturnStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn expression(&self) -> Option<Expression> {
        self.ir_node
            .expression
            .as_ref()
            .map(|ir_node| Rc::new(ExpressionStruct::create(ir_node, &self.semantic)))
    }
}

pub type EmitStatement = Rc<EmitStatementStruct>;

pub struct EmitStatementStruct {
    ir_node: input_ir::EmitStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl EmitStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::EmitStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn event(&self) -> IdentifierPath {
        Rc::new(IdentifierPathStruct::create(
            &self.ir_node.event,
            &self.semantic,
        ))
    }

    pub fn arguments(&self) -> ArgumentsDeclaration {
        Rc::new(ArgumentsDeclarationStruct::create(
            &self.ir_node.arguments,
            &self.semantic,
        ))
    }
}

pub type TryStatement = Rc<TryStatementStruct>;

pub struct TryStatementStruct {
    ir_node: input_ir::TryStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl TryStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::TryStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn expression(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.expression,
            &self.semantic,
        ))
    }

    pub fn body(&self) -> Block {
        Rc::new(BlockStruct::create(&self.ir_node.body, &self.semantic))
    }

    pub fn catch_clauses(&self) -> impl Iterator<Item = CatchClause> + use<'_> {
        self.ir_node
            .catch_clauses
            .iter()
            .map(|child| Rc::new(CatchClauseStruct::create(child, &self.semantic)))
    }
}

pub type CatchClause = Rc<CatchClauseStruct>;

pub struct CatchClauseStruct {
    ir_node: input_ir::CatchClause,
    semantic: Rc<SemanticAnalysis>,
}

impl CatchClauseStruct {
    pub(crate) fn create(ir_node: &input_ir::CatchClause, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn error(&self) -> Option<CatchClauseError> {
        self.ir_node
            .error
            .as_ref()
            .map(|ir_node| Rc::new(CatchClauseErrorStruct::create(ir_node, &self.semantic)))
    }

    pub fn body(&self) -> Block {
        Rc::new(BlockStruct::create(&self.ir_node.body, &self.semantic))
    }
}

pub type CatchClauseError = Rc<CatchClauseErrorStruct>;

pub struct CatchClauseErrorStruct {
    ir_node: input_ir::CatchClauseError,
    semantic: Rc<SemanticAnalysis>,
}

impl CatchClauseErrorStruct {
    pub(crate) fn create(
        ir_node: &input_ir::CatchClauseError,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> Option<Identifier> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| Rc::new(IdentifierStruct::create(ir_node, &self.semantic)))
    }

    pub fn parameters(&self) -> impl Iterator<Item = Parameter> + use<'_> {
        self.ir_node
            .parameters
            .iter()
            .map(|child| Rc::new(ParameterStruct::create(child, &self.semantic)))
    }
}

pub type RevertStatement = Rc<RevertStatementStruct>;

pub struct RevertStatementStruct {
    ir_node: input_ir::RevertStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl RevertStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::RevertStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn arguments(&self) -> ArgumentsDeclaration {
        Rc::new(ArgumentsDeclarationStruct::create(
            &self.ir_node.arguments,
            &self.semantic,
        ))
    }
}

pub type ThrowStatement = Rc<ThrowStatementStruct>;

pub struct ThrowStatementStruct {
    ir_node: input_ir::ThrowStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl ThrowStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ThrowStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }
}

pub type AssignmentExpression = Rc<AssignmentExpressionStruct>;

pub struct AssignmentExpressionStruct {
    ir_node: input_ir::AssignmentExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl AssignmentExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::AssignmentExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn left_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.left_operand,
            &self.semantic,
        ))
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.right_operand,
            &self.semantic,
        ))
    }
}

pub type ConditionalExpression = Rc<ConditionalExpressionStruct>;

pub struct ConditionalExpressionStruct {
    ir_node: input_ir::ConditionalExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl ConditionalExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ConditionalExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.operand,
            &self.semantic,
        ))
    }

    pub fn true_expression(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.true_expression,
            &self.semantic,
        ))
    }

    pub fn false_expression(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.false_expression,
            &self.semantic,
        ))
    }
}

pub type OrExpression = Rc<OrExpressionStruct>;

pub struct OrExpressionStruct {
    ir_node: input_ir::OrExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl OrExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::OrExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn left_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.left_operand,
            &self.semantic,
        ))
    }

    pub fn right_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.right_operand,
            &self.semantic,
        ))
    }
}

pub type AndExpression = Rc<AndExpressionStruct>;

pub struct AndExpressionStruct {
    ir_node: input_ir::AndExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl AndExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::AndExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn left_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.left_operand,
            &self.semantic,
        ))
    }

    pub fn right_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.right_operand,
            &self.semantic,
        ))
    }
}

pub type EqualityExpression = Rc<EqualityExpressionStruct>;

pub struct EqualityExpressionStruct {
    ir_node: input_ir::EqualityExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl EqualityExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::EqualityExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn left_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.left_operand,
            &self.semantic,
        ))
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.right_operand,
            &self.semantic,
        ))
    }
}

pub type InequalityExpression = Rc<InequalityExpressionStruct>;

pub struct InequalityExpressionStruct {
    ir_node: input_ir::InequalityExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl InequalityExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::InequalityExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn left_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.left_operand,
            &self.semantic,
        ))
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.right_operand,
            &self.semantic,
        ))
    }
}

pub type BitwiseOrExpression = Rc<BitwiseOrExpressionStruct>;

pub struct BitwiseOrExpressionStruct {
    ir_node: input_ir::BitwiseOrExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl BitwiseOrExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::BitwiseOrExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn left_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.left_operand,
            &self.semantic,
        ))
    }

    pub fn right_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.right_operand,
            &self.semantic,
        ))
    }
}

pub type BitwiseXorExpression = Rc<BitwiseXorExpressionStruct>;

pub struct BitwiseXorExpressionStruct {
    ir_node: input_ir::BitwiseXorExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl BitwiseXorExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::BitwiseXorExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn left_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.left_operand,
            &self.semantic,
        ))
    }

    pub fn right_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.right_operand,
            &self.semantic,
        ))
    }
}

pub type BitwiseAndExpression = Rc<BitwiseAndExpressionStruct>;

pub struct BitwiseAndExpressionStruct {
    ir_node: input_ir::BitwiseAndExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl BitwiseAndExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::BitwiseAndExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn left_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.left_operand,
            &self.semantic,
        ))
    }

    pub fn right_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.right_operand,
            &self.semantic,
        ))
    }
}

pub type ShiftExpression = Rc<ShiftExpressionStruct>;

pub struct ShiftExpressionStruct {
    ir_node: input_ir::ShiftExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl ShiftExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ShiftExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn left_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.left_operand,
            &self.semantic,
        ))
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.right_operand,
            &self.semantic,
        ))
    }
}

pub type AdditiveExpression = Rc<AdditiveExpressionStruct>;

pub struct AdditiveExpressionStruct {
    ir_node: input_ir::AdditiveExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl AdditiveExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::AdditiveExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn left_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.left_operand,
            &self.semantic,
        ))
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.right_operand,
            &self.semantic,
        ))
    }
}

pub type MultiplicativeExpression = Rc<MultiplicativeExpressionStruct>;

pub struct MultiplicativeExpressionStruct {
    ir_node: input_ir::MultiplicativeExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl MultiplicativeExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::MultiplicativeExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn left_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.left_operand,
            &self.semantic,
        ))
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.right_operand,
            &self.semantic,
        ))
    }
}

pub type ExponentiationExpression = Rc<ExponentiationExpressionStruct>;

pub struct ExponentiationExpressionStruct {
    ir_node: input_ir::ExponentiationExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl ExponentiationExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ExponentiationExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn left_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.left_operand,
            &self.semantic,
        ))
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.right_operand,
            &self.semantic,
        ))
    }
}

pub type PostfixExpression = Rc<PostfixExpressionStruct>;

pub struct PostfixExpressionStruct {
    ir_node: input_ir::PostfixExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl PostfixExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::PostfixExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.operand,
            &self.semantic,
        ))
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }
}

pub type PrefixExpression = Rc<PrefixExpressionStruct>;

pub struct PrefixExpressionStruct {
    ir_node: input_ir::PrefixExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl PrefixExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::PrefixExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.operand,
            &self.semantic,
        ))
    }
}

pub type FunctionCallExpression = Rc<FunctionCallExpressionStruct>;

pub struct FunctionCallExpressionStruct {
    ir_node: input_ir::FunctionCallExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl FunctionCallExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::FunctionCallExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.operand,
            &self.semantic,
        ))
    }

    pub fn arguments(&self) -> ArgumentsDeclaration {
        Rc::new(ArgumentsDeclarationStruct::create(
            &self.ir_node.arguments,
            &self.semantic,
        ))
    }
}

pub type CallOptionsExpression = Rc<CallOptionsExpressionStruct>;

pub struct CallOptionsExpressionStruct {
    ir_node: input_ir::CallOptionsExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl CallOptionsExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::CallOptionsExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.operand,
            &self.semantic,
        ))
    }

    pub fn options(&self) -> impl Iterator<Item = NamedArgument> + use<'_> {
        self.ir_node
            .options
            .iter()
            .map(|child| Rc::new(NamedArgumentStruct::create(child, &self.semantic)))
    }
}

pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

pub struct MemberAccessExpressionStruct {
    ir_node: input_ir::MemberAccessExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl MemberAccessExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::MemberAccessExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.operand,
            &self.semantic,
        ))
    }

    pub fn member(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(
            &self.ir_node.member,
            &self.semantic,
        ))
    }
}

pub type IndexAccessExpression = Rc<IndexAccessExpressionStruct>;

pub struct IndexAccessExpressionStruct {
    ir_node: input_ir::IndexAccessExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl IndexAccessExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::IndexAccessExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn operand(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.operand,
            &self.semantic,
        ))
    }

    pub fn start(&self) -> Option<Expression> {
        self.ir_node
            .start
            .as_ref()
            .map(|ir_node| Rc::new(ExpressionStruct::create(ir_node, &self.semantic)))
    }

    pub fn end(&self) -> Option<Expression> {
        self.ir_node
            .end
            .as_ref()
            .map(|ir_node| Rc::new(ExpressionStruct::create(ir_node, &self.semantic)))
    }
}

pub type NamedArgument = Rc<NamedArgumentStruct>;

pub struct NamedArgumentStruct {
    ir_node: input_ir::NamedArgument,
    semantic: Rc<SemanticAnalysis>,
}

impl NamedArgumentStruct {
    pub(crate) fn create(
        ir_node: &input_ir::NamedArgument,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> Identifier {
        Rc::new(IdentifierStruct::create(&self.ir_node.name, &self.semantic))
    }

    pub fn value(&self) -> Expression {
        Rc::new(ExpressionStruct::create(
            &self.ir_node.value,
            &self.semantic,
        ))
    }
}

pub type TypeExpression = Rc<TypeExpressionStruct>;

pub struct TypeExpressionStruct {
    ir_node: input_ir::TypeExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl TypeExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::TypeExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn type_name(&self) -> TypeName {
        Rc::new(TypeNameStruct::create(
            &self.ir_node.type_name,
            &self.semantic,
        ))
    }
}

pub type NewExpression = Rc<NewExpressionStruct>;

pub struct NewExpressionStruct {
    ir_node: input_ir::NewExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl NewExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::NewExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn type_name(&self) -> TypeName {
        Rc::new(TypeNameStruct::create(
            &self.ir_node.type_name,
            &self.semantic,
        ))
    }
}

pub type TupleExpression = Rc<TupleExpressionStruct>;

pub struct TupleExpressionStruct {
    ir_node: input_ir::TupleExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl TupleExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::TupleExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn items(&self) -> impl Iterator<Item = TupleValue> + use<'_> {
        self.ir_node
            .items
            .iter()
            .map(|child| Rc::new(TupleValueStruct::create(child, &self.semantic)))
    }
}

pub type TupleValue = Rc<TupleValueStruct>;

pub struct TupleValueStruct {
    ir_node: input_ir::TupleValue,
    semantic: Rc<SemanticAnalysis>,
}

impl TupleValueStruct {
    pub(crate) fn create(ir_node: &input_ir::TupleValue, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn expression(&self) -> Option<Expression> {
        self.ir_node
            .expression
            .as_ref()
            .map(|ir_node| Rc::new(ExpressionStruct::create(ir_node, &self.semantic)))
    }
}

pub type ArrayExpression = Rc<ArrayExpressionStruct>;

pub struct ArrayExpressionStruct {
    ir_node: input_ir::ArrayExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl ArrayExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ArrayExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn items(&self) -> impl Iterator<Item = Expression> + use<'_> {
        self.ir_node
            .items
            .iter()
            .map(|child| Rc::new(ExpressionStruct::create(child, &self.semantic)))
    }
}

pub type HexNumberExpression = Rc<HexNumberExpressionStruct>;

pub struct HexNumberExpressionStruct {
    ir_node: input_ir::HexNumberExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl HexNumberExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::HexNumberExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn literal(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.literal)
    }

    pub fn unit(&self) -> Option<NumberUnit> {
        self.ir_node
            .unit
            .as_ref()
            .map(|ir_node| Rc::new(NumberUnitStruct::create(ir_node, &self.semantic)))
    }
}

pub type DecimalNumberExpression = Rc<DecimalNumberExpressionStruct>;

pub struct DecimalNumberExpressionStruct {
    ir_node: input_ir::DecimalNumberExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl DecimalNumberExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::DecimalNumberExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn literal(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.literal)
    }

    pub fn unit(&self) -> Option<NumberUnit> {
        self.ir_node
            .unit
            .as_ref()
            .map(|ir_node| Rc::new(NumberUnitStruct::create(ir_node, &self.semantic)))
    }
}

pub type YulBlock = Rc<YulBlockStruct>;

pub struct YulBlockStruct {
    ir_node: input_ir::YulBlock,
    semantic: Rc<SemanticAnalysis>,
}

impl YulBlockStruct {
    pub(crate) fn create(ir_node: &input_ir::YulBlock, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn statements(&self) -> impl Iterator<Item = YulStatement> + use<'_> {
        self.ir_node
            .statements
            .iter()
            .map(|child| Rc::new(YulStatementStruct::create(child, &self.semantic)))
    }
}

pub type YulFunctionDefinition = Rc<YulFunctionDefinitionStruct>;

pub struct YulFunctionDefinitionStruct {
    ir_node: input_ir::YulFunctionDefinition,
    semantic: Rc<SemanticAnalysis>,
}

impl YulFunctionDefinitionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulFunctionDefinition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn name(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.name)
    }

    pub fn parameters(&self) -> impl Iterator<Item = Rc<TerminalNode>> + use<'_> {
        self.ir_node.parameters.iter().map(Rc::clone)
    }

    pub fn body(&self) -> YulBlock {
        Rc::new(YulBlockStruct::create(&self.ir_node.body, &self.semantic))
    }
}

pub type YulVariableDeclarationStatement = Rc<YulVariableDeclarationStatementStruct>;

pub struct YulVariableDeclarationStatementStruct {
    ir_node: input_ir::YulVariableDeclarationStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl YulVariableDeclarationStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulVariableDeclarationStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn variables(&self) -> impl Iterator<Item = Rc<TerminalNode>> + use<'_> {
        self.ir_node.variables.iter().map(Rc::clone)
    }

    pub fn value(&self) -> Option<YulVariableDeclarationValue> {
        self.ir_node.value.as_ref().map(|ir_node| {
            Rc::new(YulVariableDeclarationValueStruct::create(
                ir_node,
                &self.semantic,
            ))
        })
    }
}

pub type YulVariableDeclarationValue = Rc<YulVariableDeclarationValueStruct>;

pub struct YulVariableDeclarationValueStruct {
    ir_node: input_ir::YulVariableDeclarationValue,
    semantic: Rc<SemanticAnalysis>,
}

impl YulVariableDeclarationValueStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulVariableDeclarationValue,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn assignment(&self) -> YulAssignmentOperator {
        Rc::new(YulAssignmentOperatorStruct::create(
            &self.ir_node.assignment,
            &self.semantic,
        ))
    }

    pub fn expression(&self) -> YulExpression {
        Rc::new(YulExpressionStruct::create(
            &self.ir_node.expression,
            &self.semantic,
        ))
    }
}

pub type YulVariableAssignmentStatement = Rc<YulVariableAssignmentStatementStruct>;

pub struct YulVariableAssignmentStatementStruct {
    ir_node: input_ir::YulVariableAssignmentStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl YulVariableAssignmentStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulVariableAssignmentStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn assignment(&self) -> YulAssignmentOperator {
        Rc::new(YulAssignmentOperatorStruct::create(
            &self.ir_node.assignment,
            &self.semantic,
        ))
    }

    pub fn expression(&self) -> YulExpression {
        Rc::new(YulExpressionStruct::create(
            &self.ir_node.expression,
            &self.semantic,
        ))
    }
}

pub type YulColonAndEqual = Rc<YulColonAndEqualStruct>;

pub struct YulColonAndEqualStruct {
    ir_node: input_ir::YulColonAndEqual,
    semantic: Rc<SemanticAnalysis>,
}

impl YulColonAndEqualStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulColonAndEqual,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }
}

pub type YulStackAssignmentStatement = Rc<YulStackAssignmentStatementStruct>;

pub struct YulStackAssignmentStatementStruct {
    ir_node: input_ir::YulStackAssignmentStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl YulStackAssignmentStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulStackAssignmentStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn assignment(&self) -> YulStackAssignmentOperator {
        Rc::new(YulStackAssignmentOperatorStruct::create(
            &self.ir_node.assignment,
            &self.semantic,
        ))
    }

    pub fn variable(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.variable)
    }
}

pub type YulEqualAndColon = Rc<YulEqualAndColonStruct>;

pub struct YulEqualAndColonStruct {
    ir_node: input_ir::YulEqualAndColon,
    semantic: Rc<SemanticAnalysis>,
}

impl YulEqualAndColonStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulEqualAndColon,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }
}

pub type YulIfStatement = Rc<YulIfStatementStruct>;

pub struct YulIfStatementStruct {
    ir_node: input_ir::YulIfStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl YulIfStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulIfStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn condition(&self) -> YulExpression {
        Rc::new(YulExpressionStruct::create(
            &self.ir_node.condition,
            &self.semantic,
        ))
    }

    pub fn body(&self) -> YulBlock {
        Rc::new(YulBlockStruct::create(&self.ir_node.body, &self.semantic))
    }
}

pub type YulForStatement = Rc<YulForStatementStruct>;

pub struct YulForStatementStruct {
    ir_node: input_ir::YulForStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl YulForStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulForStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn initialization(&self) -> YulBlock {
        Rc::new(YulBlockStruct::create(
            &self.ir_node.initialization,
            &self.semantic,
        ))
    }

    pub fn condition(&self) -> YulExpression {
        Rc::new(YulExpressionStruct::create(
            &self.ir_node.condition,
            &self.semantic,
        ))
    }

    pub fn iterator(&self) -> YulBlock {
        Rc::new(YulBlockStruct::create(
            &self.ir_node.iterator,
            &self.semantic,
        ))
    }

    pub fn body(&self) -> YulBlock {
        Rc::new(YulBlockStruct::create(&self.ir_node.body, &self.semantic))
    }
}

pub type YulSwitchStatement = Rc<YulSwitchStatementStruct>;

pub struct YulSwitchStatementStruct {
    ir_node: input_ir::YulSwitchStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl YulSwitchStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulSwitchStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn expression(&self) -> YulExpression {
        Rc::new(YulExpressionStruct::create(
            &self.ir_node.expression,
            &self.semantic,
        ))
    }

    pub fn cases(&self) -> impl Iterator<Item = YulSwitchCase> + use<'_> {
        self.ir_node
            .cases
            .iter()
            .map(|child| Rc::new(YulSwitchCaseStruct::create(child, &self.semantic)))
    }
}

pub type YulDefaultCase = Rc<YulDefaultCaseStruct>;

pub struct YulDefaultCaseStruct {
    ir_node: input_ir::YulDefaultCase,
    semantic: Rc<SemanticAnalysis>,
}

impl YulDefaultCaseStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulDefaultCase,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn body(&self) -> YulBlock {
        Rc::new(YulBlockStruct::create(&self.ir_node.body, &self.semantic))
    }
}

pub type YulValueCase = Rc<YulValueCaseStruct>;

pub struct YulValueCaseStruct {
    ir_node: input_ir::YulValueCase,
    semantic: Rc<SemanticAnalysis>,
}

impl YulValueCaseStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulValueCase,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn value(&self) -> YulLiteral {
        Rc::new(YulLiteralStruct::create(
            &self.ir_node.value,
            &self.semantic,
        ))
    }

    pub fn body(&self) -> YulBlock {
        Rc::new(YulBlockStruct::create(&self.ir_node.body, &self.semantic))
    }
}

pub type YulLeaveStatement = Rc<YulLeaveStatementStruct>;

pub struct YulLeaveStatementStruct {
    ir_node: input_ir::YulLeaveStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl YulLeaveStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulLeaveStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }
}

pub type YulBreakStatement = Rc<YulBreakStatementStruct>;

pub struct YulBreakStatementStruct {
    ir_node: input_ir::YulBreakStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl YulBreakStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulBreakStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }
}

pub type YulContinueStatement = Rc<YulContinueStatementStruct>;

pub struct YulContinueStatementStruct {
    ir_node: input_ir::YulContinueStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl YulContinueStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulContinueStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }
}

pub type YulLabel = Rc<YulLabelStruct>;

pub struct YulLabelStruct {
    ir_node: input_ir::YulLabel,
    semantic: Rc<SemanticAnalysis>,
}

impl YulLabelStruct {
    pub(crate) fn create(ir_node: &input_ir::YulLabel, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn label(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.label)
    }
}

pub type YulFunctionCallExpression = Rc<YulFunctionCallExpressionStruct>;

pub struct YulFunctionCallExpressionStruct {
    ir_node: input_ir::YulFunctionCallExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl YulFunctionCallExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulFunctionCallExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn operand(&self) -> YulExpression {
        Rc::new(YulExpressionStruct::create(
            &self.ir_node.operand,
            &self.semantic,
        ))
    }

    pub fn arguments(&self) -> impl Iterator<Item = YulExpression> + use<'_> {
        self.ir_node
            .arguments
            .iter()
            .map(|child| Rc::new(YulExpressionStruct::create(child, &self.semantic)))
    }
}

//
// Choices:
//

pub type SourceUnitMember = Rc<SourceUnitMemberStruct>;

pub struct SourceUnitMemberStruct {
    ir_node: input_ir::SourceUnitMember,
    semantic: Rc<SemanticAnalysis>,
}

impl SourceUnitMemberStruct {
    pub(crate) fn create(
        ir_node: &input_ir::SourceUnitMember,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_pragma_directive(&self) -> bool {
        matches!(self.ir_node, input_ir::SourceUnitMember::PragmaDirective(_))
    }

    pub fn as_pragma_directive(&self) -> Option<PragmaDirective> {
        if let input_ir::SourceUnitMember::PragmaDirective(variant) = &self.ir_node {
            Some(Rc::new(PragmaDirectiveStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_contract_definition(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::SourceUnitMember::ContractDefinition(_)
        )
    }

    pub fn as_contract_definition(&self) -> Option<ContractDefinition> {
        if let input_ir::SourceUnitMember::ContractDefinition(variant) = &self.ir_node {
            Some(Rc::new(ContractDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_interface_definition(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::SourceUnitMember::InterfaceDefinition(_)
        )
    }

    pub fn as_interface_definition(&self) -> Option<InterfaceDefinition> {
        if let input_ir::SourceUnitMember::InterfaceDefinition(variant) = &self.ir_node {
            Some(Rc::new(InterfaceDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_library_definition(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::SourceUnitMember::LibraryDefinition(_)
        )
    }

    pub fn as_library_definition(&self) -> Option<LibraryDefinition> {
        if let input_ir::SourceUnitMember::LibraryDefinition(variant) = &self.ir_node {
            Some(Rc::new(LibraryDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_struct_definition(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::SourceUnitMember::StructDefinition(_)
        )
    }

    pub fn as_struct_definition(&self) -> Option<StructDefinition> {
        if let input_ir::SourceUnitMember::StructDefinition(variant) = &self.ir_node {
            Some(Rc::new(StructDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_enum_definition(&self) -> bool {
        matches!(self.ir_node, input_ir::SourceUnitMember::EnumDefinition(_))
    }

    pub fn as_enum_definition(&self) -> Option<EnumDefinition> {
        if let input_ir::SourceUnitMember::EnumDefinition(variant) = &self.ir_node {
            Some(Rc::new(EnumDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_function_definition(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::SourceUnitMember::FunctionDefinition(_)
        )
    }

    pub fn as_function_definition(&self) -> Option<FunctionDefinition> {
        if let input_ir::SourceUnitMember::FunctionDefinition(variant) = &self.ir_node {
            Some(Rc::new(FunctionDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_error_definition(&self) -> bool {
        matches!(self.ir_node, input_ir::SourceUnitMember::ErrorDefinition(_))
    }

    pub fn as_error_definition(&self) -> Option<ErrorDefinition> {
        if let input_ir::SourceUnitMember::ErrorDefinition(variant) = &self.ir_node {
            Some(Rc::new(ErrorDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_user_defined_value_type_definition(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::SourceUnitMember::UserDefinedValueTypeDefinition(_)
        )
    }

    pub fn as_user_defined_value_type_definition(&self) -> Option<UserDefinedValueTypeDefinition> {
        if let input_ir::SourceUnitMember::UserDefinedValueTypeDefinition(variant) = &self.ir_node {
            Some(Rc::new(UserDefinedValueTypeDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_using_directive(&self) -> bool {
        matches!(self.ir_node, input_ir::SourceUnitMember::UsingDirective(_))
    }

    pub fn as_using_directive(&self) -> Option<UsingDirective> {
        if let input_ir::SourceUnitMember::UsingDirective(variant) = &self.ir_node {
            Some(Rc::new(UsingDirectiveStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_event_definition(&self) -> bool {
        matches!(self.ir_node, input_ir::SourceUnitMember::EventDefinition(_))
    }

    pub fn as_event_definition(&self) -> Option<EventDefinition> {
        if let input_ir::SourceUnitMember::EventDefinition(variant) = &self.ir_node {
            Some(Rc::new(EventDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_constant_definition(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::SourceUnitMember::ConstantDefinition(_)
        )
    }

    pub fn as_constant_definition(&self) -> Option<ConstantDefinition> {
        if let input_ir::SourceUnitMember::ConstantDefinition(variant) = &self.ir_node {
            Some(Rc::new(ConstantDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_import_clause(&self) -> bool {
        matches!(self.ir_node, input_ir::SourceUnitMember::ImportClause(_))
    }

    pub fn as_import_clause(&self) -> Option<ImportClause> {
        if let input_ir::SourceUnitMember::ImportClause(variant) = &self.ir_node {
            Some(Rc::new(ImportClauseStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }
}

pub type Pragma = Rc<PragmaStruct>;

pub struct PragmaStruct {
    ir_node: input_ir::Pragma,
    semantic: Rc<SemanticAnalysis>,
}

impl PragmaStruct {
    pub(crate) fn create(ir_node: &input_ir::Pragma, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_version_pragma(&self) -> bool {
        matches!(self.ir_node, input_ir::Pragma::VersionPragma(_))
    }

    pub fn as_version_pragma(&self) -> Option<VersionPragma> {
        if let input_ir::Pragma::VersionPragma(variant) = &self.ir_node {
            Some(Rc::new(VersionPragmaStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_abicoder_pragma(&self) -> bool {
        matches!(self.ir_node, input_ir::Pragma::AbicoderPragma(_))
    }

    pub fn as_abicoder_pragma(&self) -> Option<AbicoderPragma> {
        if let input_ir::Pragma::AbicoderPragma(variant) = &self.ir_node {
            Some(Rc::new(AbicoderPragmaStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_experimental_pragma(&self) -> bool {
        matches!(self.ir_node, input_ir::Pragma::ExperimentalPragma(_))
    }

    pub fn as_experimental_pragma(&self) -> Option<ExperimentalPragma> {
        if let input_ir::Pragma::ExperimentalPragma(variant) = &self.ir_node {
            Some(Rc::new(ExperimentalPragmaStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }
}

pub type AbicoderVersion = Rc<AbicoderVersionStruct>;

pub struct AbicoderVersionStruct {
    ir_node: input_ir::AbicoderVersion,
    semantic: Rc<SemanticAnalysis>,
}

impl AbicoderVersionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::AbicoderVersion,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_abicoder_v1_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::AbicoderVersion::AbicoderV1Keyword)
    }

    pub fn is_abicoder_v2_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::AbicoderVersion::AbicoderV2Keyword)
    }
}

pub type ExperimentalFeature = Rc<ExperimentalFeatureStruct>;

pub struct ExperimentalFeatureStruct {
    ir_node: input_ir::ExperimentalFeature,
    semantic: Rc<SemanticAnalysis>,
}

impl ExperimentalFeatureStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ExperimentalFeature,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_abi_encoder_v2_keyword(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ExperimentalFeature::ABIEncoderV2Keyword
        )
    }

    pub fn is_smt_checker_keyword(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ExperimentalFeature::SMTCheckerKeyword
        )
    }

    pub fn is_string_literal(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ExperimentalFeature::StringLiteral(_)
        )
    }
}

pub type VersionExpression = Rc<VersionExpressionStruct>;

pub struct VersionExpressionStruct {
    ir_node: input_ir::VersionExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl VersionExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::VersionExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_version_range(&self) -> bool {
        matches!(self.ir_node, input_ir::VersionExpression::VersionRange(_))
    }

    pub fn as_version_range(&self) -> Option<VersionRange> {
        if let input_ir::VersionExpression::VersionRange(variant) = &self.ir_node {
            Some(Rc::new(VersionRangeStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_version_term(&self) -> bool {
        matches!(self.ir_node, input_ir::VersionExpression::VersionTerm(_))
    }

    pub fn as_version_term(&self) -> Option<VersionTerm> {
        if let input_ir::VersionExpression::VersionTerm(variant) = &self.ir_node {
            Some(Rc::new(VersionTermStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }
}

pub type VersionOperator = Rc<VersionOperatorStruct>;

pub struct VersionOperatorStruct {
    ir_node: input_ir::VersionOperator,
    semantic: Rc<SemanticAnalysis>,
}

impl VersionOperatorStruct {
    pub(crate) fn create(
        ir_node: &input_ir::VersionOperator,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_caret(&self) -> bool {
        matches!(self.ir_node, input_ir::VersionOperator::Caret)
    }

    pub fn is_tilde(&self) -> bool {
        matches!(self.ir_node, input_ir::VersionOperator::Tilde)
    }

    pub fn is_equal(&self) -> bool {
        matches!(self.ir_node, input_ir::VersionOperator::Equal)
    }

    pub fn is_less_than(&self) -> bool {
        matches!(self.ir_node, input_ir::VersionOperator::LessThan)
    }

    pub fn is_greater_than(&self) -> bool {
        matches!(self.ir_node, input_ir::VersionOperator::GreaterThan)
    }

    pub fn is_less_than_equal(&self) -> bool {
        matches!(self.ir_node, input_ir::VersionOperator::LessThanEqual)
    }

    pub fn is_greater_than_equal(&self) -> bool {
        matches!(self.ir_node, input_ir::VersionOperator::GreaterThanEqual)
    }
}

pub type VersionLiteral = Rc<VersionLiteralStruct>;

pub struct VersionLiteralStruct {
    ir_node: input_ir::VersionLiteral,
    semantic: Rc<SemanticAnalysis>,
}

impl VersionLiteralStruct {
    pub(crate) fn create(
        ir_node: &input_ir::VersionLiteral,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_simple_version_literal(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::VersionLiteral::SimpleVersionLiteral(_)
        )
    }

    pub fn as_simple_version_literal(
        &self,
    ) -> Option<impl Iterator<Item = Rc<TerminalNode>> + use<'_>> {
        if let input_ir::VersionLiteral::SimpleVersionLiteral(variant) = &self.ir_node {
            Some(variant.iter().map(Rc::clone))
        } else {
            None
        }
    }

    pub fn is_single_quoted_version_literal(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::VersionLiteral::SingleQuotedVersionLiteral(_)
        )
    }

    pub fn is_double_quoted_version_literal(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::VersionLiteral::DoubleQuotedVersionLiteral(_)
        )
    }
}

pub type ImportClause = Rc<ImportClauseStruct>;

pub struct ImportClauseStruct {
    ir_node: input_ir::ImportClause,
    semantic: Rc<SemanticAnalysis>,
}

impl ImportClauseStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ImportClause,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_path_import(&self) -> bool {
        matches!(self.ir_node, input_ir::ImportClause::PathImport(_))
    }

    pub fn as_path_import(&self) -> Option<PathImport> {
        if let input_ir::ImportClause::PathImport(variant) = &self.ir_node {
            Some(Rc::new(PathImportStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_import_deconstruction(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ImportClause::ImportDeconstruction(_)
        )
    }

    pub fn as_import_deconstruction(&self) -> Option<ImportDeconstruction> {
        if let input_ir::ImportClause::ImportDeconstruction(variant) = &self.ir_node {
            Some(Rc::new(ImportDeconstructionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }
}

pub type UsingClause = Rc<UsingClauseStruct>;

pub struct UsingClauseStruct {
    ir_node: input_ir::UsingClause,
    semantic: Rc<SemanticAnalysis>,
}

impl UsingClauseStruct {
    pub(crate) fn create(ir_node: &input_ir::UsingClause, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_identifier_path(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingClause::IdentifierPath(_))
    }

    pub fn as_identifier_path(&self) -> Option<IdentifierPath> {
        if let input_ir::UsingClause::IdentifierPath(variant) = &self.ir_node {
            Some(Rc::new(IdentifierPathStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_using_deconstruction(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingClause::UsingDeconstruction(_))
    }

    pub fn as_using_deconstruction(&self) -> Option<UsingDeconstruction> {
        if let input_ir::UsingClause::UsingDeconstruction(variant) = &self.ir_node {
            Some(Rc::new(UsingDeconstructionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }
}

pub type UsingOperator = Rc<UsingOperatorStruct>;

pub struct UsingOperatorStruct {
    ir_node: input_ir::UsingOperator,
    semantic: Rc<SemanticAnalysis>,
}

impl UsingOperatorStruct {
    pub(crate) fn create(
        ir_node: &input_ir::UsingOperator,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_ampersand(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::Ampersand)
    }

    pub fn is_asterisk(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::Asterisk)
    }

    pub fn is_bang_equal(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::BangEqual)
    }

    pub fn is_bar(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::Bar)
    }

    pub fn is_caret(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::Caret)
    }

    pub fn is_equal_equal(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::EqualEqual)
    }

    pub fn is_greater_than(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::GreaterThan)
    }

    pub fn is_greater_than_equal(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::GreaterThanEqual)
    }

    pub fn is_less_than(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::LessThan)
    }

    pub fn is_less_than_equal(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::LessThanEqual)
    }

    pub fn is_minus(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::Minus)
    }

    pub fn is_percent(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::Percent)
    }

    pub fn is_plus(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::Plus)
    }

    pub fn is_slash(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::Slash)
    }

    pub fn is_tilde(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingOperator::Tilde)
    }
}

pub type UsingTarget = Rc<UsingTargetStruct>;

pub struct UsingTargetStruct {
    ir_node: input_ir::UsingTarget,
    semantic: Rc<SemanticAnalysis>,
}

impl UsingTargetStruct {
    pub(crate) fn create(ir_node: &input_ir::UsingTarget, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_type_name(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingTarget::TypeName(_))
    }

    pub fn as_type_name(&self) -> Option<TypeName> {
        if let input_ir::UsingTarget::TypeName(variant) = &self.ir_node {
            Some(Rc::new(TypeNameStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_asterisk(&self) -> bool {
        matches!(self.ir_node, input_ir::UsingTarget::Asterisk)
    }
}

pub type ContractMember = Rc<ContractMemberStruct>;

pub struct ContractMemberStruct {
    ir_node: input_ir::ContractMember,
    semantic: Rc<SemanticAnalysis>,
}

impl ContractMemberStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ContractMember,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_using_directive(&self) -> bool {
        matches!(self.ir_node, input_ir::ContractMember::UsingDirective(_))
    }

    pub fn as_using_directive(&self) -> Option<UsingDirective> {
        if let input_ir::ContractMember::UsingDirective(variant) = &self.ir_node {
            Some(Rc::new(UsingDirectiveStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_function_definition(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ContractMember::FunctionDefinition(_)
        )
    }

    pub fn as_function_definition(&self) -> Option<FunctionDefinition> {
        if let input_ir::ContractMember::FunctionDefinition(variant) = &self.ir_node {
            Some(Rc::new(FunctionDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_struct_definition(&self) -> bool {
        matches!(self.ir_node, input_ir::ContractMember::StructDefinition(_))
    }

    pub fn as_struct_definition(&self) -> Option<StructDefinition> {
        if let input_ir::ContractMember::StructDefinition(variant) = &self.ir_node {
            Some(Rc::new(StructDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_enum_definition(&self) -> bool {
        matches!(self.ir_node, input_ir::ContractMember::EnumDefinition(_))
    }

    pub fn as_enum_definition(&self) -> Option<EnumDefinition> {
        if let input_ir::ContractMember::EnumDefinition(variant) = &self.ir_node {
            Some(Rc::new(EnumDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_event_definition(&self) -> bool {
        matches!(self.ir_node, input_ir::ContractMember::EventDefinition(_))
    }

    pub fn as_event_definition(&self) -> Option<EventDefinition> {
        if let input_ir::ContractMember::EventDefinition(variant) = &self.ir_node {
            Some(Rc::new(EventDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_error_definition(&self) -> bool {
        matches!(self.ir_node, input_ir::ContractMember::ErrorDefinition(_))
    }

    pub fn as_error_definition(&self) -> Option<ErrorDefinition> {
        if let input_ir::ContractMember::ErrorDefinition(variant) = &self.ir_node {
            Some(Rc::new(ErrorDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_user_defined_value_type_definition(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ContractMember::UserDefinedValueTypeDefinition(_)
        )
    }

    pub fn as_user_defined_value_type_definition(&self) -> Option<UserDefinedValueTypeDefinition> {
        if let input_ir::ContractMember::UserDefinedValueTypeDefinition(variant) = &self.ir_node {
            Some(Rc::new(UserDefinedValueTypeDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_state_variable_definition(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ContractMember::StateVariableDefinition(_)
        )
    }

    pub fn as_state_variable_definition(&self) -> Option<StateVariableDefinition> {
        if let input_ir::ContractMember::StateVariableDefinition(variant) = &self.ir_node {
            Some(Rc::new(StateVariableDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_constant_definition(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ContractMember::ConstantDefinition(_)
        )
    }

    pub fn as_constant_definition(&self) -> Option<ConstantDefinition> {
        if let input_ir::ContractMember::ConstantDefinition(variant) = &self.ir_node {
            Some(Rc::new(ConstantDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }
}

pub type TypeName = Rc<TypeNameStruct>;

pub struct TypeNameStruct {
    ir_node: input_ir::TypeName,
    semantic: Rc<SemanticAnalysis>,
}

impl TypeNameStruct {
    pub(crate) fn create(ir_node: &input_ir::TypeName, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_array_type_name(&self) -> bool {
        matches!(self.ir_node, input_ir::TypeName::ArrayTypeName(_))
    }

    pub fn as_array_type_name(&self) -> Option<ArrayTypeName> {
        if let input_ir::TypeName::ArrayTypeName(variant) = &self.ir_node {
            Some(Rc::new(ArrayTypeNameStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_function_type(&self) -> bool {
        matches!(self.ir_node, input_ir::TypeName::FunctionType(_))
    }

    pub fn as_function_type(&self) -> Option<FunctionType> {
        if let input_ir::TypeName::FunctionType(variant) = &self.ir_node {
            Some(Rc::new(FunctionTypeStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_mapping_type(&self) -> bool {
        matches!(self.ir_node, input_ir::TypeName::MappingType(_))
    }

    pub fn as_mapping_type(&self) -> Option<MappingType> {
        if let input_ir::TypeName::MappingType(variant) = &self.ir_node {
            Some(Rc::new(MappingTypeStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_elementary_type(&self) -> bool {
        matches!(self.ir_node, input_ir::TypeName::ElementaryType(_))
    }

    pub fn as_elementary_type(&self) -> Option<ElementaryType> {
        if let input_ir::TypeName::ElementaryType(variant) = &self.ir_node {
            Some(Rc::new(ElementaryTypeStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_identifier_path(&self) -> bool {
        matches!(self.ir_node, input_ir::TypeName::IdentifierPath(_))
    }

    pub fn as_identifier_path(&self) -> Option<IdentifierPath> {
        if let input_ir::TypeName::IdentifierPath(variant) = &self.ir_node {
            Some(Rc::new(IdentifierPathStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }
}

pub type ElementaryType = Rc<ElementaryTypeStruct>;

pub struct ElementaryTypeStruct {
    ir_node: input_ir::ElementaryType,
    semantic: Rc<SemanticAnalysis>,
}

impl ElementaryTypeStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ElementaryType,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_bool_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::ElementaryType::BoolKeyword)
    }

    pub fn is_byte_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::ElementaryType::ByteKeyword)
    }

    pub fn is_string_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::ElementaryType::StringKeyword)
    }

    pub fn is_address_type(&self) -> bool {
        matches!(self.ir_node, input_ir::ElementaryType::AddressType(_))
    }

    pub fn as_address_type(&self) -> Option<AddressType> {
        if let input_ir::ElementaryType::AddressType(variant) = &self.ir_node {
            Some(Rc::new(AddressTypeStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_bytes_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::ElementaryType::BytesKeyword(_))
    }

    pub fn is_int_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::ElementaryType::IntKeyword(_))
    }

    pub fn is_uint_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::ElementaryType::UintKeyword(_))
    }

    pub fn is_fixed_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::ElementaryType::FixedKeyword(_))
    }

    pub fn is_ufixed_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::ElementaryType::UfixedKeyword(_))
    }
}

pub type Statement = Rc<StatementStruct>;

pub struct StatementStruct {
    ir_node: input_ir::Statement,
    semantic: Rc<SemanticAnalysis>,
}

impl StatementStruct {
    pub(crate) fn create(ir_node: &input_ir::Statement, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_if_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::IfStatement(_))
    }

    pub fn as_if_statement(&self) -> Option<IfStatement> {
        if let input_ir::Statement::IfStatement(variant) = &self.ir_node {
            Some(Rc::new(IfStatementStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_for_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::ForStatement(_))
    }

    pub fn as_for_statement(&self) -> Option<ForStatement> {
        if let input_ir::Statement::ForStatement(variant) = &self.ir_node {
            Some(Rc::new(ForStatementStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_while_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::WhileStatement(_))
    }

    pub fn as_while_statement(&self) -> Option<WhileStatement> {
        if let input_ir::Statement::WhileStatement(variant) = &self.ir_node {
            Some(Rc::new(WhileStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_do_while_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::DoWhileStatement(_))
    }

    pub fn as_do_while_statement(&self) -> Option<DoWhileStatement> {
        if let input_ir::Statement::DoWhileStatement(variant) = &self.ir_node {
            Some(Rc::new(DoWhileStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_continue_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::ContinueStatement(_))
    }

    pub fn as_continue_statement(&self) -> Option<ContinueStatement> {
        if let input_ir::Statement::ContinueStatement(variant) = &self.ir_node {
            Some(Rc::new(ContinueStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_break_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::BreakStatement(_))
    }

    pub fn as_break_statement(&self) -> Option<BreakStatement> {
        if let input_ir::Statement::BreakStatement(variant) = &self.ir_node {
            Some(Rc::new(BreakStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_return_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::ReturnStatement(_))
    }

    pub fn as_return_statement(&self) -> Option<ReturnStatement> {
        if let input_ir::Statement::ReturnStatement(variant) = &self.ir_node {
            Some(Rc::new(ReturnStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_throw_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::ThrowStatement(_))
    }

    pub fn as_throw_statement(&self) -> Option<ThrowStatement> {
        if let input_ir::Statement::ThrowStatement(variant) = &self.ir_node {
            Some(Rc::new(ThrowStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_emit_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::EmitStatement(_))
    }

    pub fn as_emit_statement(&self) -> Option<EmitStatement> {
        if let input_ir::Statement::EmitStatement(variant) = &self.ir_node {
            Some(Rc::new(EmitStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_try_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::TryStatement(_))
    }

    pub fn as_try_statement(&self) -> Option<TryStatement> {
        if let input_ir::Statement::TryStatement(variant) = &self.ir_node {
            Some(Rc::new(TryStatementStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_revert_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::RevertStatement(_))
    }

    pub fn as_revert_statement(&self) -> Option<RevertStatement> {
        if let input_ir::Statement::RevertStatement(variant) = &self.ir_node {
            Some(Rc::new(RevertStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_assembly_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::AssemblyStatement(_))
    }

    pub fn as_assembly_statement(&self) -> Option<AssemblyStatement> {
        if let input_ir::Statement::AssemblyStatement(variant) = &self.ir_node {
            Some(Rc::new(AssemblyStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_block(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::Block(_))
    }

    pub fn as_block(&self) -> Option<Block> {
        if let input_ir::Statement::Block(variant) = &self.ir_node {
            Some(Rc::new(BlockStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_unchecked_block(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::UncheckedBlock(_))
    }

    pub fn as_unchecked_block(&self) -> Option<UncheckedBlock> {
        if let input_ir::Statement::UncheckedBlock(variant) = &self.ir_node {
            Some(Rc::new(UncheckedBlockStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_tuple_deconstruction_statement(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::Statement::TupleDeconstructionStatement(_)
        )
    }

    pub fn as_tuple_deconstruction_statement(&self) -> Option<TupleDeconstructionStatement> {
        if let input_ir::Statement::TupleDeconstructionStatement(variant) = &self.ir_node {
            Some(Rc::new(TupleDeconstructionStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_variable_declaration_statement(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::Statement::VariableDeclarationStatement(_)
        )
    }

    pub fn as_variable_declaration_statement(&self) -> Option<VariableDeclarationStatement> {
        if let input_ir::Statement::VariableDeclarationStatement(variant) = &self.ir_node {
            Some(Rc::new(VariableDeclarationStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_expression_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::Statement::ExpressionStatement(_))
    }

    pub fn as_expression_statement(&self) -> Option<ExpressionStatement> {
        if let input_ir::Statement::ExpressionStatement(variant) = &self.ir_node {
            Some(Rc::new(ExpressionStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }
}

pub type StorageLocation = Rc<StorageLocationStruct>;

pub struct StorageLocationStruct {
    ir_node: input_ir::StorageLocation,
    semantic: Rc<SemanticAnalysis>,
}

impl StorageLocationStruct {
    pub(crate) fn create(
        ir_node: &input_ir::StorageLocation,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_memory_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::StorageLocation::MemoryKeyword)
    }

    pub fn is_storage_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::StorageLocation::StorageKeyword)
    }

    pub fn is_call_data_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::StorageLocation::CallDataKeyword)
    }
}

pub type ForStatementInitialization = Rc<ForStatementInitializationStruct>;

pub struct ForStatementInitializationStruct {
    ir_node: input_ir::ForStatementInitialization,
    semantic: Rc<SemanticAnalysis>,
}

impl ForStatementInitializationStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ForStatementInitialization,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_tuple_deconstruction_statement(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ForStatementInitialization::TupleDeconstructionStatement(_)
        )
    }

    pub fn as_tuple_deconstruction_statement(&self) -> Option<TupleDeconstructionStatement> {
        if let input_ir::ForStatementInitialization::TupleDeconstructionStatement(variant) =
            &self.ir_node
        {
            Some(Rc::new(TupleDeconstructionStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_variable_declaration_statement(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ForStatementInitialization::VariableDeclarationStatement(_)
        )
    }

    pub fn as_variable_declaration_statement(&self) -> Option<VariableDeclarationStatement> {
        if let input_ir::ForStatementInitialization::VariableDeclarationStatement(variant) =
            &self.ir_node
        {
            Some(Rc::new(VariableDeclarationStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_expression_statement(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ForStatementInitialization::ExpressionStatement(_)
        )
    }

    pub fn as_expression_statement(&self) -> Option<ExpressionStatement> {
        if let input_ir::ForStatementInitialization::ExpressionStatement(variant) = &self.ir_node {
            Some(Rc::new(ExpressionStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_semicolon(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ForStatementInitialization::Semicolon
        )
    }
}

pub type ForStatementCondition = Rc<ForStatementConditionStruct>;

pub struct ForStatementConditionStruct {
    ir_node: input_ir::ForStatementCondition,
    semantic: Rc<SemanticAnalysis>,
}

impl ForStatementConditionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ForStatementCondition,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_expression_statement(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ForStatementCondition::ExpressionStatement(_)
        )
    }

    pub fn as_expression_statement(&self) -> Option<ExpressionStatement> {
        if let input_ir::ForStatementCondition::ExpressionStatement(variant) = &self.ir_node {
            Some(Rc::new(ExpressionStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_semicolon(&self) -> bool {
        matches!(self.ir_node, input_ir::ForStatementCondition::Semicolon)
    }
}

pub type Expression = Rc<ExpressionStruct>;

pub struct ExpressionStruct {
    ir_node: input_ir::Expression,
    semantic: Rc<SemanticAnalysis>,
}

impl ExpressionStruct {
    pub(crate) fn create(ir_node: &input_ir::Expression, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_assignment_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::AssignmentExpression(_))
    }

    pub fn as_assignment_expression(&self) -> Option<AssignmentExpression> {
        if let input_ir::Expression::AssignmentExpression(variant) = &self.ir_node {
            Some(Rc::new(AssignmentExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_conditional_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::ConditionalExpression(_))
    }

    pub fn as_conditional_expression(&self) -> Option<ConditionalExpression> {
        if let input_ir::Expression::ConditionalExpression(variant) = &self.ir_node {
            Some(Rc::new(ConditionalExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_or_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::OrExpression(_))
    }

    pub fn as_or_expression(&self) -> Option<OrExpression> {
        if let input_ir::Expression::OrExpression(variant) = &self.ir_node {
            Some(Rc::new(OrExpressionStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_and_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::AndExpression(_))
    }

    pub fn as_and_expression(&self) -> Option<AndExpression> {
        if let input_ir::Expression::AndExpression(variant) = &self.ir_node {
            Some(Rc::new(AndExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_equality_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::EqualityExpression(_))
    }

    pub fn as_equality_expression(&self) -> Option<EqualityExpression> {
        if let input_ir::Expression::EqualityExpression(variant) = &self.ir_node {
            Some(Rc::new(EqualityExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_inequality_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::InequalityExpression(_))
    }

    pub fn as_inequality_expression(&self) -> Option<InequalityExpression> {
        if let input_ir::Expression::InequalityExpression(variant) = &self.ir_node {
            Some(Rc::new(InequalityExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_bitwise_or_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::BitwiseOrExpression(_))
    }

    pub fn as_bitwise_or_expression(&self) -> Option<BitwiseOrExpression> {
        if let input_ir::Expression::BitwiseOrExpression(variant) = &self.ir_node {
            Some(Rc::new(BitwiseOrExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_bitwise_xor_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::BitwiseXorExpression(_))
    }

    pub fn as_bitwise_xor_expression(&self) -> Option<BitwiseXorExpression> {
        if let input_ir::Expression::BitwiseXorExpression(variant) = &self.ir_node {
            Some(Rc::new(BitwiseXorExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_bitwise_and_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::BitwiseAndExpression(_))
    }

    pub fn as_bitwise_and_expression(&self) -> Option<BitwiseAndExpression> {
        if let input_ir::Expression::BitwiseAndExpression(variant) = &self.ir_node {
            Some(Rc::new(BitwiseAndExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_shift_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::ShiftExpression(_))
    }

    pub fn as_shift_expression(&self) -> Option<ShiftExpression> {
        if let input_ir::Expression::ShiftExpression(variant) = &self.ir_node {
            Some(Rc::new(ShiftExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_additive_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::AdditiveExpression(_))
    }

    pub fn as_additive_expression(&self) -> Option<AdditiveExpression> {
        if let input_ir::Expression::AdditiveExpression(variant) = &self.ir_node {
            Some(Rc::new(AdditiveExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_multiplicative_expression(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::Expression::MultiplicativeExpression(_)
        )
    }

    pub fn as_multiplicative_expression(&self) -> Option<MultiplicativeExpression> {
        if let input_ir::Expression::MultiplicativeExpression(variant) = &self.ir_node {
            Some(Rc::new(MultiplicativeExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_exponentiation_expression(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::Expression::ExponentiationExpression(_)
        )
    }

    pub fn as_exponentiation_expression(&self) -> Option<ExponentiationExpression> {
        if let input_ir::Expression::ExponentiationExpression(variant) = &self.ir_node {
            Some(Rc::new(ExponentiationExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_postfix_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::PostfixExpression(_))
    }

    pub fn as_postfix_expression(&self) -> Option<PostfixExpression> {
        if let input_ir::Expression::PostfixExpression(variant) = &self.ir_node {
            Some(Rc::new(PostfixExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_prefix_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::PrefixExpression(_))
    }

    pub fn as_prefix_expression(&self) -> Option<PrefixExpression> {
        if let input_ir::Expression::PrefixExpression(variant) = &self.ir_node {
            Some(Rc::new(PrefixExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_function_call_expression(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::Expression::FunctionCallExpression(_)
        )
    }

    pub fn as_function_call_expression(&self) -> Option<FunctionCallExpression> {
        if let input_ir::Expression::FunctionCallExpression(variant) = &self.ir_node {
            Some(Rc::new(FunctionCallExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_call_options_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::CallOptionsExpression(_))
    }

    pub fn as_call_options_expression(&self) -> Option<CallOptionsExpression> {
        if let input_ir::Expression::CallOptionsExpression(variant) = &self.ir_node {
            Some(Rc::new(CallOptionsExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_member_access_expression(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::Expression::MemberAccessExpression(_)
        )
    }

    pub fn as_member_access_expression(&self) -> Option<MemberAccessExpression> {
        if let input_ir::Expression::MemberAccessExpression(variant) = &self.ir_node {
            Some(Rc::new(MemberAccessExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_index_access_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::IndexAccessExpression(_))
    }

    pub fn as_index_access_expression(&self) -> Option<IndexAccessExpression> {
        if let input_ir::Expression::IndexAccessExpression(variant) = &self.ir_node {
            Some(Rc::new(IndexAccessExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_new_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::NewExpression(_))
    }

    pub fn as_new_expression(&self) -> Option<NewExpression> {
        if let input_ir::Expression::NewExpression(variant) = &self.ir_node {
            Some(Rc::new(NewExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_tuple_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::TupleExpression(_))
    }

    pub fn as_tuple_expression(&self) -> Option<TupleExpression> {
        if let input_ir::Expression::TupleExpression(variant) = &self.ir_node {
            Some(Rc::new(TupleExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_type_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::TypeExpression(_))
    }

    pub fn as_type_expression(&self) -> Option<TypeExpression> {
        if let input_ir::Expression::TypeExpression(variant) = &self.ir_node {
            Some(Rc::new(TypeExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_array_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::ArrayExpression(_))
    }

    pub fn as_array_expression(&self) -> Option<ArrayExpression> {
        if let input_ir::Expression::ArrayExpression(variant) = &self.ir_node {
            Some(Rc::new(ArrayExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_hex_number_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::HexNumberExpression(_))
    }

    pub fn as_hex_number_expression(&self) -> Option<HexNumberExpression> {
        if let input_ir::Expression::HexNumberExpression(variant) = &self.ir_node {
            Some(Rc::new(HexNumberExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_decimal_number_expression(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::Expression::DecimalNumberExpression(_)
        )
    }

    pub fn as_decimal_number_expression(&self) -> Option<DecimalNumberExpression> {
        if let input_ir::Expression::DecimalNumberExpression(variant) = &self.ir_node {
            Some(Rc::new(DecimalNumberExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_string_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::StringExpression(_))
    }

    pub fn as_string_expression(&self) -> Option<StringExpression> {
        if let input_ir::Expression::StringExpression(variant) = &self.ir_node {
            Some(Rc::new(StringExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_elementary_type(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::ElementaryType(_))
    }

    pub fn as_elementary_type(&self) -> Option<ElementaryType> {
        if let input_ir::Expression::ElementaryType(variant) = &self.ir_node {
            Some(Rc::new(ElementaryTypeStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_payable_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::PayableKeyword)
    }

    pub fn is_this_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::ThisKeyword)
    }

    pub fn is_super_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::SuperKeyword)
    }

    pub fn is_true_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::TrueKeyword)
    }

    pub fn is_false_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::FalseKeyword)
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self.ir_node, input_ir::Expression::Identifier(_))
    }

    pub fn as_identifier(&self) -> Option<Identifier> {
        if let input_ir::Expression::Identifier(variant) = &self.ir_node {
            Some(Rc::new(IdentifierStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }
}

pub type ArgumentsDeclaration = Rc<ArgumentsDeclarationStruct>;

pub struct ArgumentsDeclarationStruct {
    ir_node: input_ir::ArgumentsDeclaration,
    semantic: Rc<SemanticAnalysis>,
}

impl ArgumentsDeclarationStruct {
    pub(crate) fn create(
        ir_node: &input_ir::ArgumentsDeclaration,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_positional_arguments(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ArgumentsDeclaration::PositionalArguments(_)
        )
    }

    pub fn as_positional_arguments(&self) -> Option<impl Iterator<Item = Expression> + use<'_>> {
        if let input_ir::ArgumentsDeclaration::PositionalArguments(variant) = &self.ir_node {
            Some(
                variant
                    .iter()
                    .map(|child| Rc::new(ExpressionStruct::create(child, &self.semantic))),
            )
        } else {
            None
        }
    }

    pub fn is_named_arguments(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::ArgumentsDeclaration::NamedArguments(_)
        )
    }

    pub fn as_named_arguments(&self) -> Option<impl Iterator<Item = NamedArgument> + use<'_>> {
        if let input_ir::ArgumentsDeclaration::NamedArguments(variant) = &self.ir_node {
            Some(
                variant
                    .iter()
                    .map(|child| Rc::new(NamedArgumentStruct::create(child, &self.semantic))),
            )
        } else {
            None
        }
    }
}

pub type NumberUnit = Rc<NumberUnitStruct>;

pub struct NumberUnitStruct {
    ir_node: input_ir::NumberUnit,
    semantic: Rc<SemanticAnalysis>,
}

impl NumberUnitStruct {
    pub(crate) fn create(ir_node: &input_ir::NumberUnit, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_wei_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::NumberUnit::WeiKeyword)
    }

    pub fn is_gwei_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::NumberUnit::GweiKeyword)
    }

    pub fn is_szabo_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::NumberUnit::SzaboKeyword)
    }

    pub fn is_finney_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::NumberUnit::FinneyKeyword)
    }

    pub fn is_ether_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::NumberUnit::EtherKeyword)
    }

    pub fn is_seconds_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::NumberUnit::SecondsKeyword)
    }

    pub fn is_minutes_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::NumberUnit::MinutesKeyword)
    }

    pub fn is_hours_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::NumberUnit::HoursKeyword)
    }

    pub fn is_days_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::NumberUnit::DaysKeyword)
    }

    pub fn is_weeks_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::NumberUnit::WeeksKeyword)
    }

    pub fn is_years_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::NumberUnit::YearsKeyword)
    }
}

pub type StringExpression = Rc<StringExpressionStruct>;

pub struct StringExpressionStruct {
    ir_node: input_ir::StringExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl StringExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::StringExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_strings(&self) -> bool {
        matches!(self.ir_node, input_ir::StringExpression::Strings(_))
    }

    pub fn as_strings(&self) -> Option<impl Iterator<Item = Rc<TerminalNode>> + use<'_>> {
        if let input_ir::StringExpression::Strings(variant) = &self.ir_node {
            Some(variant.iter().map(Rc::clone))
        } else {
            None
        }
    }

    pub fn is_hex_strings(&self) -> bool {
        matches!(self.ir_node, input_ir::StringExpression::HexStrings(_))
    }

    pub fn as_hex_strings(&self) -> Option<impl Iterator<Item = Rc<TerminalNode>> + use<'_>> {
        if let input_ir::StringExpression::HexStrings(variant) = &self.ir_node {
            Some(variant.iter().map(Rc::clone))
        } else {
            None
        }
    }

    pub fn is_unicode_strings(&self) -> bool {
        matches!(self.ir_node, input_ir::StringExpression::UnicodeStrings(_))
    }

    pub fn as_unicode_strings(&self) -> Option<impl Iterator<Item = Rc<TerminalNode>> + use<'_>> {
        if let input_ir::StringExpression::UnicodeStrings(variant) = &self.ir_node {
            Some(variant.iter().map(Rc::clone))
        } else {
            None
        }
    }
}

pub type YulStatement = Rc<YulStatementStruct>;

pub struct YulStatementStruct {
    ir_node: input_ir::YulStatement,
    semantic: Rc<SemanticAnalysis>,
}

impl YulStatementStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulStatement,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_yul_block(&self) -> bool {
        matches!(self.ir_node, input_ir::YulStatement::YulBlock(_))
    }

    pub fn as_yul_block(&self) -> Option<YulBlock> {
        if let input_ir::YulStatement::YulBlock(variant) = &self.ir_node {
            Some(Rc::new(YulBlockStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_yul_function_definition(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::YulStatement::YulFunctionDefinition(_)
        )
    }

    pub fn as_yul_function_definition(&self) -> Option<YulFunctionDefinition> {
        if let input_ir::YulStatement::YulFunctionDefinition(variant) = &self.ir_node {
            Some(Rc::new(YulFunctionDefinitionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_yul_stack_assignment_statement(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::YulStatement::YulStackAssignmentStatement(_)
        )
    }

    pub fn as_yul_stack_assignment_statement(&self) -> Option<YulStackAssignmentStatement> {
        if let input_ir::YulStatement::YulStackAssignmentStatement(variant) = &self.ir_node {
            Some(Rc::new(YulStackAssignmentStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_yul_if_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::YulStatement::YulIfStatement(_))
    }

    pub fn as_yul_if_statement(&self) -> Option<YulIfStatement> {
        if let input_ir::YulStatement::YulIfStatement(variant) = &self.ir_node {
            Some(Rc::new(YulIfStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_yul_for_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::YulStatement::YulForStatement(_))
    }

    pub fn as_yul_for_statement(&self) -> Option<YulForStatement> {
        if let input_ir::YulStatement::YulForStatement(variant) = &self.ir_node {
            Some(Rc::new(YulForStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_yul_switch_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::YulStatement::YulSwitchStatement(_))
    }

    pub fn as_yul_switch_statement(&self) -> Option<YulSwitchStatement> {
        if let input_ir::YulStatement::YulSwitchStatement(variant) = &self.ir_node {
            Some(Rc::new(YulSwitchStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_yul_leave_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::YulStatement::YulLeaveStatement(_))
    }

    pub fn as_yul_leave_statement(&self) -> Option<YulLeaveStatement> {
        if let input_ir::YulStatement::YulLeaveStatement(variant) = &self.ir_node {
            Some(Rc::new(YulLeaveStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_yul_break_statement(&self) -> bool {
        matches!(self.ir_node, input_ir::YulStatement::YulBreakStatement(_))
    }

    pub fn as_yul_break_statement(&self) -> Option<YulBreakStatement> {
        if let input_ir::YulStatement::YulBreakStatement(variant) = &self.ir_node {
            Some(Rc::new(YulBreakStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_yul_continue_statement(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::YulStatement::YulContinueStatement(_)
        )
    }

    pub fn as_yul_continue_statement(&self) -> Option<YulContinueStatement> {
        if let input_ir::YulStatement::YulContinueStatement(variant) = &self.ir_node {
            Some(Rc::new(YulContinueStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_yul_variable_assignment_statement(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::YulStatement::YulVariableAssignmentStatement(_)
        )
    }

    pub fn as_yul_variable_assignment_statement(&self) -> Option<YulVariableAssignmentStatement> {
        if let input_ir::YulStatement::YulVariableAssignmentStatement(variant) = &self.ir_node {
            Some(Rc::new(YulVariableAssignmentStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_yul_label(&self) -> bool {
        matches!(self.ir_node, input_ir::YulStatement::YulLabel(_))
    }

    pub fn as_yul_label(&self) -> Option<YulLabel> {
        if let input_ir::YulStatement::YulLabel(variant) = &self.ir_node {
            Some(Rc::new(YulLabelStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_yul_variable_declaration_statement(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::YulStatement::YulVariableDeclarationStatement(_)
        )
    }

    pub fn as_yul_variable_declaration_statement(&self) -> Option<YulVariableDeclarationStatement> {
        if let input_ir::YulStatement::YulVariableDeclarationStatement(variant) = &self.ir_node {
            Some(Rc::new(YulVariableDeclarationStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_yul_expression(&self) -> bool {
        matches!(self.ir_node, input_ir::YulStatement::YulExpression(_))
    }

    pub fn as_yul_expression(&self) -> Option<YulExpression> {
        if let input_ir::YulStatement::YulExpression(variant) = &self.ir_node {
            Some(Rc::new(YulExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }
}

pub type YulAssignmentOperator = Rc<YulAssignmentOperatorStruct>;

pub struct YulAssignmentOperatorStruct {
    ir_node: input_ir::YulAssignmentOperator,
    semantic: Rc<SemanticAnalysis>,
}

impl YulAssignmentOperatorStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulAssignmentOperator,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_colon_equal(&self) -> bool {
        matches!(self.ir_node, input_ir::YulAssignmentOperator::ColonEqual)
    }

    pub fn is_yul_colon_and_equal(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::YulAssignmentOperator::YulColonAndEqual(_)
        )
    }

    pub fn as_yul_colon_and_equal(&self) -> Option<YulColonAndEqual> {
        if let input_ir::YulAssignmentOperator::YulColonAndEqual(variant) = &self.ir_node {
            Some(Rc::new(YulColonAndEqualStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }
}

pub type YulStackAssignmentOperator = Rc<YulStackAssignmentOperatorStruct>;

pub struct YulStackAssignmentOperatorStruct {
    ir_node: input_ir::YulStackAssignmentOperator,
    semantic: Rc<SemanticAnalysis>,
}

impl YulStackAssignmentOperatorStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulStackAssignmentOperator,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_equal_colon(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::YulStackAssignmentOperator::EqualColon
        )
    }

    pub fn is_yul_equal_and_colon(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::YulStackAssignmentOperator::YulEqualAndColon(_)
        )
    }

    pub fn as_yul_equal_and_colon(&self) -> Option<YulEqualAndColon> {
        if let input_ir::YulStackAssignmentOperator::YulEqualAndColon(variant) = &self.ir_node {
            Some(Rc::new(YulEqualAndColonStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }
}

pub type YulSwitchCase = Rc<YulSwitchCaseStruct>;

pub struct YulSwitchCaseStruct {
    ir_node: input_ir::YulSwitchCase,
    semantic: Rc<SemanticAnalysis>,
}

impl YulSwitchCaseStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulSwitchCase,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_yul_default_case(&self) -> bool {
        matches!(self.ir_node, input_ir::YulSwitchCase::YulDefaultCase(_))
    }

    pub fn as_yul_default_case(&self) -> Option<YulDefaultCase> {
        if let input_ir::YulSwitchCase::YulDefaultCase(variant) = &self.ir_node {
            Some(Rc::new(YulDefaultCaseStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_yul_value_case(&self) -> bool {
        matches!(self.ir_node, input_ir::YulSwitchCase::YulValueCase(_))
    }

    pub fn as_yul_value_case(&self) -> Option<YulValueCase> {
        if let input_ir::YulSwitchCase::YulValueCase(variant) = &self.ir_node {
            Some(Rc::new(YulValueCaseStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }
}

pub type YulExpression = Rc<YulExpressionStruct>;

pub struct YulExpressionStruct {
    ir_node: input_ir::YulExpression,
    semantic: Rc<SemanticAnalysis>,
}

impl YulExpressionStruct {
    pub(crate) fn create(
        ir_node: &input_ir::YulExpression,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_yul_function_call_expression(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::YulExpression::YulFunctionCallExpression(_)
        )
    }

    pub fn as_yul_function_call_expression(&self) -> Option<YulFunctionCallExpression> {
        if let input_ir::YulExpression::YulFunctionCallExpression(variant) = &self.ir_node {
            Some(Rc::new(YulFunctionCallExpressionStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }

    pub fn is_yul_literal(&self) -> bool {
        matches!(self.ir_node, input_ir::YulExpression::YulLiteral(_))
    }

    pub fn as_yul_literal(&self) -> Option<YulLiteral> {
        if let input_ir::YulExpression::YulLiteral(variant) = &self.ir_node {
            Some(Rc::new(YulLiteralStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_yul_path(&self) -> bool {
        matches!(self.ir_node, input_ir::YulExpression::YulPath(_))
    }

    pub fn as_yul_path(&self) -> Option<impl Iterator<Item = Rc<TerminalNode>> + use<'_>> {
        if let input_ir::YulExpression::YulPath(variant) = &self.ir_node {
            Some(variant.iter().map(Rc::clone))
        } else {
            None
        }
    }
}

pub type YulLiteral = Rc<YulLiteralStruct>;

pub struct YulLiteralStruct {
    ir_node: input_ir::YulLiteral,
    semantic: Rc<SemanticAnalysis>,
}

impl YulLiteralStruct {
    pub(crate) fn create(ir_node: &input_ir::YulLiteral, semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_yul_true_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::YulLiteral::YulTrueKeyword)
    }

    pub fn is_yul_false_keyword(&self) -> bool {
        matches!(self.ir_node, input_ir::YulLiteral::YulFalseKeyword)
    }

    pub fn is_yul_decimal_literal(&self) -> bool {
        matches!(self.ir_node, input_ir::YulLiteral::YulDecimalLiteral(_))
    }

    pub fn is_yul_hex_literal(&self) -> bool {
        matches!(self.ir_node, input_ir::YulLiteral::YulHexLiteral(_))
    }

    pub fn is_string_literal(&self) -> bool {
        matches!(self.ir_node, input_ir::YulLiteral::StringLiteral(_))
    }

    pub fn is_hex_string_literal(&self) -> bool {
        matches!(self.ir_node, input_ir::YulLiteral::HexStringLiteral(_))
    }
}

pub type FunctionKind = Rc<FunctionKindStruct>;

pub struct FunctionKindStruct {
    ir_node: input_ir::FunctionKind,
    semantic: Rc<SemanticAnalysis>,
}

impl FunctionKindStruct {
    pub(crate) fn create(
        ir_node: &input_ir::FunctionKind,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_regular(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionKind::Regular)
    }

    pub fn is_constructor(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionKind::Constructor)
    }

    pub fn is_unnamed(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionKind::Unnamed)
    }

    pub fn is_fallback(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionKind::Fallback)
    }

    pub fn is_receive(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionKind::Receive)
    }

    pub fn is_modifier(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionKind::Modifier)
    }
}

pub type FunctionVisibility = Rc<FunctionVisibilityStruct>;

pub struct FunctionVisibilityStruct {
    ir_node: input_ir::FunctionVisibility,
    semantic: Rc<SemanticAnalysis>,
}

impl FunctionVisibilityStruct {
    pub(crate) fn create(
        ir_node: &input_ir::FunctionVisibility,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_public(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionVisibility::Public)
    }

    pub fn is_private(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionVisibility::Private)
    }

    pub fn is_internal(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionVisibility::Internal)
    }

    pub fn is_external(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionVisibility::External)
    }
}

pub type FunctionMutability = Rc<FunctionMutabilityStruct>;

pub struct FunctionMutabilityStruct {
    ir_node: input_ir::FunctionMutability,
    semantic: Rc<SemanticAnalysis>,
}

impl FunctionMutabilityStruct {
    pub(crate) fn create(
        ir_node: &input_ir::FunctionMutability,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_pure(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionMutability::Pure)
    }

    pub fn is_view(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionMutability::View)
    }

    pub fn is_non_payable(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionMutability::NonPayable)
    }

    pub fn is_payable(&self) -> bool {
        matches!(self.ir_node, input_ir::FunctionMutability::Payable)
    }
}

pub type StateVariableVisibility = Rc<StateVariableVisibilityStruct>;

pub struct StateVariableVisibilityStruct {
    ir_node: input_ir::StateVariableVisibility,
    semantic: Rc<SemanticAnalysis>,
}

impl StateVariableVisibilityStruct {
    pub(crate) fn create(
        ir_node: &input_ir::StateVariableVisibility,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_public(&self) -> bool {
        matches!(self.ir_node, input_ir::StateVariableVisibility::Public)
    }

    pub fn is_private(&self) -> bool {
        matches!(self.ir_node, input_ir::StateVariableVisibility::Private)
    }

    pub fn is_internal(&self) -> bool {
        matches!(self.ir_node, input_ir::StateVariableVisibility::Internal)
    }
}

pub type StateVariableMutability = Rc<StateVariableMutabilityStruct>;

pub struct StateVariableMutabilityStruct {
    ir_node: input_ir::StateVariableMutability,
    semantic: Rc<SemanticAnalysis>,
}

impl StateVariableMutabilityStruct {
    pub(crate) fn create(
        ir_node: &input_ir::StateVariableMutability,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_mutable(&self) -> bool {
        matches!(self.ir_node, input_ir::StateVariableMutability::Mutable)
    }

    pub fn is_constant(&self) -> bool {
        matches!(self.ir_node, input_ir::StateVariableMutability::Constant)
    }

    pub fn is_immutable(&self) -> bool {
        matches!(self.ir_node, input_ir::StateVariableMutability::Immutable)
    }

    pub fn is_transient(&self) -> bool {
        matches!(self.ir_node, input_ir::StateVariableMutability::Transient)
    }
}

pub type TupleDeconstructionMember = Rc<TupleDeconstructionMemberStruct>;

pub struct TupleDeconstructionMemberStruct {
    ir_node: input_ir::TupleDeconstructionMember,
    semantic: Rc<SemanticAnalysis>,
}

impl TupleDeconstructionMemberStruct {
    pub(crate) fn create(
        ir_node: &input_ir::TupleDeconstructionMember,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Self {
        Self {
            ir_node: ir_node.clone(),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self.ir_node, input_ir::TupleDeconstructionMember::None)
    }

    pub fn is_identifier(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::TupleDeconstructionMember::Identifier(_)
        )
    }

    pub fn as_identifier(&self) -> Option<Identifier> {
        if let input_ir::TupleDeconstructionMember::Identifier(variant) = &self.ir_node {
            Some(Rc::new(IdentifierStruct::create(variant, &self.semantic)))
        } else {
            None
        }
    }

    pub fn is_variable_declaration_statement(&self) -> bool {
        matches!(
            self.ir_node,
            input_ir::TupleDeconstructionMember::VariableDeclarationStatement(_)
        )
    }

    pub fn as_variable_declaration_statement(&self) -> Option<VariableDeclarationStatement> {
        if let input_ir::TupleDeconstructionMember::VariableDeclarationStatement(variant) =
            &self.ir_node
        {
            Some(Rc::new(VariableDeclarationStatementStruct::create(
                variant,
                &self.semantic,
            )))
        } else {
            None
        }
    }
}
