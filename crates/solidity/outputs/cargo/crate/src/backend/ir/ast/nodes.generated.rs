// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(unused)]
use std::rc::Rc;

use paste::paste;

use super::input as input_ir;
use super::node_extensions::{
    create_identifier, create_yul_identifier, Identifier, IdentifierStruct, YulIdentifier,
    YulIdentifierStruct,
};
use crate::backend::{binder, SemanticAnalysis};
use crate::cst::{NodeId, TerminalKind, TerminalNode, TextIndex};

//
// Sequences:
//

pub type SourceUnit = Rc<SourceUnitStruct>;

pub struct SourceUnitStruct {
    pub(crate) ir_node: input_ir::SourceUnit,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_source_unit(
    ir_node: &input_ir::SourceUnit,
    semantic: &Rc<SemanticAnalysis>,
) -> SourceUnit {
    Rc::new(SourceUnitStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl SourceUnitStruct {
    pub fn members(&self) -> SourceUnitMembers {
        create_source_unit_members(&self.ir_node.members, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type PragmaDirective = Rc<PragmaDirectiveStruct>;

pub struct PragmaDirectiveStruct {
    pub(crate) ir_node: input_ir::PragmaDirective,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_pragma_directive(
    ir_node: &input_ir::PragmaDirective,
    semantic: &Rc<SemanticAnalysis>,
) -> PragmaDirective {
    Rc::new(PragmaDirectiveStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PragmaDirectiveStruct {
    pub fn pragma(&self) -> Pragma {
        create_pragma(&self.ir_node.pragma, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type AbicoderPragma = Rc<AbicoderPragmaStruct>;

pub struct AbicoderPragmaStruct {
    pub(crate) ir_node: input_ir::AbicoderPragma,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_abicoder_pragma(
    ir_node: &input_ir::AbicoderPragma,
    semantic: &Rc<SemanticAnalysis>,
) -> AbicoderPragma {
    Rc::new(AbicoderPragmaStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AbicoderPragmaStruct {
    pub fn version(&self) -> AbicoderVersion {
        create_abicoder_version(&self.ir_node.version, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ExperimentalPragma = Rc<ExperimentalPragmaStruct>;

pub struct ExperimentalPragmaStruct {
    pub(crate) ir_node: input_ir::ExperimentalPragma,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_experimental_pragma(
    ir_node: &input_ir::ExperimentalPragma,
    semantic: &Rc<SemanticAnalysis>,
) -> ExperimentalPragma {
    Rc::new(ExperimentalPragmaStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ExperimentalPragmaStruct {
    pub fn feature(&self) -> ExperimentalFeature {
        create_experimental_feature(&self.ir_node.feature, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type VersionPragma = Rc<VersionPragmaStruct>;

pub struct VersionPragmaStruct {
    pub(crate) ir_node: input_ir::VersionPragma,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_version_pragma(
    ir_node: &input_ir::VersionPragma,
    semantic: &Rc<SemanticAnalysis>,
) -> VersionPragma {
    Rc::new(VersionPragmaStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl VersionPragmaStruct {
    pub fn sets(&self) -> VersionExpressionSets {
        create_version_expression_sets(&self.ir_node.sets, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type VersionRange = Rc<VersionRangeStruct>;

pub struct VersionRangeStruct {
    pub(crate) ir_node: input_ir::VersionRange,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_version_range(
    ir_node: &input_ir::VersionRange,
    semantic: &Rc<SemanticAnalysis>,
) -> VersionRange {
    Rc::new(VersionRangeStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl VersionRangeStruct {
    pub fn start(&self) -> VersionLiteral {
        create_version_literal(&self.ir_node.start, &self.semantic)
    }

    pub fn end(&self) -> VersionLiteral {
        create_version_literal(&self.ir_node.end, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type VersionTerm = Rc<VersionTermStruct>;

pub struct VersionTermStruct {
    pub(crate) ir_node: input_ir::VersionTerm,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_version_term(
    ir_node: &input_ir::VersionTerm,
    semantic: &Rc<SemanticAnalysis>,
) -> VersionTerm {
    Rc::new(VersionTermStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl VersionTermStruct {
    pub fn operator(&self) -> Option<VersionOperator> {
        self.ir_node
            .operator
            .as_ref()
            .map(|ir_node| create_version_operator(ir_node, &self.semantic))
    }

    pub fn literal(&self) -> VersionLiteral {
        create_version_literal(&self.ir_node.literal, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type PathImport = Rc<PathImportStruct>;

pub struct PathImportStruct {
    pub(crate) ir_node: input_ir::PathImport,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_path_import(
    ir_node: &input_ir::PathImport,
    semantic: &Rc<SemanticAnalysis>,
) -> PathImport {
    Rc::new(PathImportStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PathImportStruct {
    pub fn alias(&self) -> Option<Identifier> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn path(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.path)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

pub struct ImportDeconstructionStruct {
    pub(crate) ir_node: input_ir::ImportDeconstruction,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_import_deconstruction(
    ir_node: &input_ir::ImportDeconstruction,
    semantic: &Rc<SemanticAnalysis>,
) -> ImportDeconstruction {
    Rc::new(ImportDeconstructionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ImportDeconstructionStruct {
    pub fn symbols(&self) -> ImportDeconstructionSymbols {
        create_import_deconstruction_symbols(&self.ir_node.symbols, &self.semantic)
    }

    pub fn path(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.path)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

pub struct ImportDeconstructionSymbolStruct {
    pub(crate) ir_node: input_ir::ImportDeconstructionSymbol,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_import_deconstruction_symbol(
    ir_node: &input_ir::ImportDeconstructionSymbol,
    semantic: &Rc<SemanticAnalysis>,
) -> ImportDeconstructionSymbol {
    Rc::new(ImportDeconstructionSymbolStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ImportDeconstructionSymbolStruct {
    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn alias(&self) -> Option<Identifier> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type UsingDirective = Rc<UsingDirectiveStruct>;

pub struct UsingDirectiveStruct {
    pub(crate) ir_node: input_ir::UsingDirective,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_using_directive(
    ir_node: &input_ir::UsingDirective,
    semantic: &Rc<SemanticAnalysis>,
) -> UsingDirective {
    Rc::new(UsingDirectiveStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl UsingDirectiveStruct {
    pub fn clause(&self) -> UsingClause {
        create_using_clause(&self.ir_node.clause, &self.semantic)
    }

    pub fn target(&self) -> UsingTarget {
        create_using_target(&self.ir_node.target, &self.semantic)
    }

    pub fn global_keyword(&self) -> bool {
        self.ir_node.global_keyword
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type UsingDeconstruction = Rc<UsingDeconstructionStruct>;

pub struct UsingDeconstructionStruct {
    pub(crate) ir_node: input_ir::UsingDeconstruction,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_using_deconstruction(
    ir_node: &input_ir::UsingDeconstruction,
    semantic: &Rc<SemanticAnalysis>,
) -> UsingDeconstruction {
    Rc::new(UsingDeconstructionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl UsingDeconstructionStruct {
    pub fn symbols(&self) -> UsingDeconstructionSymbols {
        create_using_deconstruction_symbols(&self.ir_node.symbols, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type UsingDeconstructionSymbol = Rc<UsingDeconstructionSymbolStruct>;

pub struct UsingDeconstructionSymbolStruct {
    pub(crate) ir_node: input_ir::UsingDeconstructionSymbol,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_using_deconstruction_symbol(
    ir_node: &input_ir::UsingDeconstructionSymbol,
    semantic: &Rc<SemanticAnalysis>,
) -> UsingDeconstructionSymbol {
    Rc::new(UsingDeconstructionSymbolStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl UsingDeconstructionSymbolStruct {
    pub fn name(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.name, &self.semantic)
    }

    pub fn alias(&self) -> Option<UsingOperator> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| create_using_operator(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ContractDefinition = Rc<ContractDefinitionStruct>;

pub struct ContractDefinitionStruct {
    pub(crate) ir_node: input_ir::ContractDefinition,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_contract_definition(
    ir_node: &input_ir::ContractDefinition,
    semantic: &Rc<SemanticAnalysis>,
) -> ContractDefinition {
    Rc::new(ContractDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ContractDefinitionStruct {
    pub fn abstract_keyword(&self) -> bool {
        self.ir_node.abstract_keyword
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn members(&self) -> ContractMembers {
        create_contract_members(&self.ir_node.members, &self.semantic)
    }

    pub fn inheritance_types(&self) -> InheritanceTypes {
        create_inheritance_types(&self.ir_node.inheritance_types, &self.semantic)
    }

    pub fn storage_layout(&self) -> Option<Expression> {
        self.ir_node
            .storage_layout
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type InheritanceType = Rc<InheritanceTypeStruct>;

pub struct InheritanceTypeStruct {
    pub(crate) ir_node: input_ir::InheritanceType,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_inheritance_type(
    ir_node: &input_ir::InheritanceType,
    semantic: &Rc<SemanticAnalysis>,
) -> InheritanceType {
    Rc::new(InheritanceTypeStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl InheritanceTypeStruct {
    pub fn type_name(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.type_name, &self.semantic)
    }

    pub fn arguments(&self) -> Option<ArgumentsDeclaration> {
        self.ir_node
            .arguments
            .as_ref()
            .map(|ir_node| create_arguments_declaration(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

pub struct InterfaceDefinitionStruct {
    pub(crate) ir_node: input_ir::InterfaceDefinition,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_interface_definition(
    ir_node: &input_ir::InterfaceDefinition,
    semantic: &Rc<SemanticAnalysis>,
) -> InterfaceDefinition {
    Rc::new(InterfaceDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl InterfaceDefinitionStruct {
    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn inheritance(&self) -> Option<InheritanceTypes> {
        self.ir_node
            .inheritance
            .as_ref()
            .map(|ir_node| create_inheritance_types(ir_node, &self.semantic))
    }

    pub fn members(&self) -> InterfaceMembers {
        create_interface_members(&self.ir_node.members, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type LibraryDefinition = Rc<LibraryDefinitionStruct>;

pub struct LibraryDefinitionStruct {
    pub(crate) ir_node: input_ir::LibraryDefinition,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_library_definition(
    ir_node: &input_ir::LibraryDefinition,
    semantic: &Rc<SemanticAnalysis>,
) -> LibraryDefinition {
    Rc::new(LibraryDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl LibraryDefinitionStruct {
    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn members(&self) -> LibraryMembers {
        create_library_members(&self.ir_node.members, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type StructDefinition = Rc<StructDefinitionStruct>;

pub struct StructDefinitionStruct {
    pub(crate) ir_node: input_ir::StructDefinition,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_struct_definition(
    ir_node: &input_ir::StructDefinition,
    semantic: &Rc<SemanticAnalysis>,
) -> StructDefinition {
    Rc::new(StructDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl StructDefinitionStruct {
    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn members(&self) -> StructMembers {
        create_struct_members(&self.ir_node.members, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type StructMember = Rc<StructMemberStruct>;

pub struct StructMemberStruct {
    pub(crate) ir_node: input_ir::StructMember,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_struct_member(
    ir_node: &input_ir::StructMember,
    semantic: &Rc<SemanticAnalysis>,
) -> StructMember {
    Rc::new(StructMemberStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl StructMemberStruct {
    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type EnumDefinition = Rc<EnumDefinitionStruct>;

pub struct EnumDefinitionStruct {
    pub(crate) ir_node: input_ir::EnumDefinition,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_enum_definition(
    ir_node: &input_ir::EnumDefinition,
    semantic: &Rc<SemanticAnalysis>,
) -> EnumDefinition {
    Rc::new(EnumDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl EnumDefinitionStruct {
    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn members(&self) -> EnumMembers {
        create_enum_members(&self.ir_node.members, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ConstantDefinition = Rc<ConstantDefinitionStruct>;

pub struct ConstantDefinitionStruct {
    pub(crate) ir_node: input_ir::ConstantDefinition,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_constant_definition(
    ir_node: &input_ir::ConstantDefinition,
    semantic: &Rc<SemanticAnalysis>,
) -> ConstantDefinition {
    Rc::new(ConstantDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ConstantDefinitionStruct {
    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn visibility(&self) -> Option<StateVariableVisibility> {
        self.ir_node
            .visibility
            .as_ref()
            .map(|ir_node| create_state_variable_visibility(ir_node, &self.semantic))
    }

    pub fn value(&self) -> Option<Expression> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

pub struct StateVariableDefinitionStruct {
    pub(crate) ir_node: input_ir::StateVariableDefinition,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_state_variable_definition(
    ir_node: &input_ir::StateVariableDefinition,
    semantic: &Rc<SemanticAnalysis>,
) -> StateVariableDefinition {
    Rc::new(StateVariableDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl StateVariableDefinitionStruct {
    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn value(&self) -> Option<Expression> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn visibility(&self) -> StateVariableVisibility {
        create_state_variable_visibility(&self.ir_node.visibility, &self.semantic)
    }

    pub fn mutability(&self) -> StateVariableMutability {
        create_state_variable_mutability(&self.ir_node.mutability, &self.semantic)
    }

    pub fn override_specifier(&self) -> Option<OverridePaths> {
        self.ir_node
            .override_specifier
            .as_ref()
            .map(|ir_node| create_override_paths(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

pub struct FunctionDefinitionStruct {
    pub(crate) ir_node: input_ir::FunctionDefinition,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_function_definition(
    ir_node: &input_ir::FunctionDefinition,
    semantic: &Rc<SemanticAnalysis>,
) -> FunctionDefinition {
    Rc::new(FunctionDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl FunctionDefinitionStruct {
    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn returns(&self) -> Option<Parameters> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, &self.semantic))
    }

    pub fn kind(&self) -> FunctionKind {
        create_function_kind(&self.ir_node.kind, &self.semantic)
    }

    pub fn name(&self) -> Option<Identifier> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn body(&self) -> Option<Block> {
        self.ir_node
            .body
            .as_ref()
            .map(|ir_node| create_block(ir_node, &self.semantic))
    }

    pub fn visibility(&self) -> FunctionVisibility {
        create_function_visibility(&self.ir_node.visibility, &self.semantic)
    }

    pub fn mutability(&self) -> FunctionMutability {
        create_function_mutability(&self.ir_node.mutability, &self.semantic)
    }

    pub fn virtual_keyword(&self) -> bool {
        self.ir_node.virtual_keyword
    }

    pub fn override_specifier(&self) -> Option<OverridePaths> {
        self.ir_node
            .override_specifier
            .as_ref()
            .map(|ir_node| create_override_paths(ir_node, &self.semantic))
    }

    pub fn modifier_invocations(&self) -> ModifierInvocations {
        create_modifier_invocations(&self.ir_node.modifier_invocations, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type Parameter = Rc<ParameterStruct>;

pub struct ParameterStruct {
    pub(crate) ir_node: input_ir::Parameter,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_parameter(
    ir_node: &input_ir::Parameter,
    semantic: &Rc<SemanticAnalysis>,
) -> Parameter {
    Rc::new(ParameterStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ParameterStruct {
    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn storage_location(&self) -> Option<StorageLocation> {
        self.ir_node
            .storage_location
            .as_ref()
            .map(|ir_node| create_storage_location(ir_node, &self.semantic))
    }

    pub fn name(&self) -> Option<Identifier> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn indexed(&self) -> bool {
        self.ir_node.indexed
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type OverrideSpecifier = Rc<OverrideSpecifierStruct>;

pub struct OverrideSpecifierStruct {
    pub(crate) ir_node: input_ir::OverrideSpecifier,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_override_specifier(
    ir_node: &input_ir::OverrideSpecifier,
    semantic: &Rc<SemanticAnalysis>,
) -> OverrideSpecifier {
    Rc::new(OverrideSpecifierStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl OverrideSpecifierStruct {
    pub fn overridden(&self) -> Option<OverridePaths> {
        self.ir_node
            .overridden
            .as_ref()
            .map(|ir_node| create_override_paths(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ModifierInvocation = Rc<ModifierInvocationStruct>;

pub struct ModifierInvocationStruct {
    pub(crate) ir_node: input_ir::ModifierInvocation,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_modifier_invocation(
    ir_node: &input_ir::ModifierInvocation,
    semantic: &Rc<SemanticAnalysis>,
) -> ModifierInvocation {
    Rc::new(ModifierInvocationStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ModifierInvocationStruct {
    pub fn name(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.name, &self.semantic)
    }

    pub fn arguments(&self) -> Option<ArgumentsDeclaration> {
        self.ir_node
            .arguments
            .as_ref()
            .map(|ir_node| create_arguments_declaration(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type EventDefinition = Rc<EventDefinitionStruct>;

pub struct EventDefinitionStruct {
    pub(crate) ir_node: input_ir::EventDefinition,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_event_definition(
    ir_node: &input_ir::EventDefinition,
    semantic: &Rc<SemanticAnalysis>,
) -> EventDefinition {
    Rc::new(EventDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl EventDefinitionStruct {
    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn anonymous_keyword(&self) -> bool {
        self.ir_node.anonymous_keyword
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type UserDefinedValueTypeDefinition = Rc<UserDefinedValueTypeDefinitionStruct>;

pub struct UserDefinedValueTypeDefinitionStruct {
    pub(crate) ir_node: input_ir::UserDefinedValueTypeDefinition,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_user_defined_value_type_definition(
    ir_node: &input_ir::UserDefinedValueTypeDefinition,
    semantic: &Rc<SemanticAnalysis>,
) -> UserDefinedValueTypeDefinition {
    Rc::new(UserDefinedValueTypeDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl UserDefinedValueTypeDefinitionStruct {
    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn value_type(&self) -> ElementaryType {
        create_elementary_type(&self.ir_node.value_type, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ErrorDefinition = Rc<ErrorDefinitionStruct>;

pub struct ErrorDefinitionStruct {
    pub(crate) ir_node: input_ir::ErrorDefinition,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_error_definition(
    ir_node: &input_ir::ErrorDefinition,
    semantic: &Rc<SemanticAnalysis>,
) -> ErrorDefinition {
    Rc::new(ErrorDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ErrorDefinitionStruct {
    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ArrayTypeName = Rc<ArrayTypeNameStruct>;

pub struct ArrayTypeNameStruct {
    pub(crate) ir_node: input_ir::ArrayTypeName,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_array_type_name(
    ir_node: &input_ir::ArrayTypeName,
    semantic: &Rc<SemanticAnalysis>,
) -> ArrayTypeName {
    Rc::new(ArrayTypeNameStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ArrayTypeNameStruct {
    pub fn operand(&self) -> TypeName {
        create_type_name(&self.ir_node.operand, &self.semantic)
    }

    pub fn index(&self) -> Option<Expression> {
        self.ir_node
            .index
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type FunctionType = Rc<FunctionTypeStruct>;

pub struct FunctionTypeStruct {
    pub(crate) ir_node: input_ir::FunctionType,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_function_type(
    ir_node: &input_ir::FunctionType,
    semantic: &Rc<SemanticAnalysis>,
) -> FunctionType {
    Rc::new(FunctionTypeStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl FunctionTypeStruct {
    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn returns(&self) -> Option<Parameters> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, &self.semantic))
    }

    pub fn visibility(&self) -> FunctionVisibility {
        create_function_visibility(&self.ir_node.visibility, &self.semantic)
    }

    pub fn mutability(&self) -> FunctionMutability {
        create_function_mutability(&self.ir_node.mutability, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type MappingType = Rc<MappingTypeStruct>;

pub struct MappingTypeStruct {
    pub(crate) ir_node: input_ir::MappingType,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_mapping_type(
    ir_node: &input_ir::MappingType,
    semantic: &Rc<SemanticAnalysis>,
) -> MappingType {
    Rc::new(MappingTypeStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl MappingTypeStruct {
    pub fn key_type(&self) -> Parameter {
        create_parameter(&self.ir_node.key_type, &self.semantic)
    }

    pub fn value_type(&self) -> Parameter {
        create_parameter(&self.ir_node.value_type, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type AddressType = Rc<AddressTypeStruct>;

pub struct AddressTypeStruct {
    pub(crate) ir_node: input_ir::AddressType,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_address_type(
    ir_node: &input_ir::AddressType,
    semantic: &Rc<SemanticAnalysis>,
) -> AddressType {
    Rc::new(AddressTypeStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AddressTypeStruct {
    pub fn payable_keyword(&self) -> bool {
        self.ir_node.payable_keyword
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type Block = Rc<BlockStruct>;

pub struct BlockStruct {
    pub(crate) ir_node: input_ir::Block,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_block(ir_node: &input_ir::Block, semantic: &Rc<SemanticAnalysis>) -> Block {
    Rc::new(BlockStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl BlockStruct {
    pub fn statements(&self) -> Statements {
        create_statements(&self.ir_node.statements, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type UncheckedBlock = Rc<UncheckedBlockStruct>;

pub struct UncheckedBlockStruct {
    pub(crate) ir_node: input_ir::UncheckedBlock,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_unchecked_block(
    ir_node: &input_ir::UncheckedBlock,
    semantic: &Rc<SemanticAnalysis>,
) -> UncheckedBlock {
    Rc::new(UncheckedBlockStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl UncheckedBlockStruct {
    pub fn block(&self) -> Block {
        create_block(&self.ir_node.block, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ExpressionStatement = Rc<ExpressionStatementStruct>;

pub struct ExpressionStatementStruct {
    pub(crate) ir_node: input_ir::ExpressionStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_expression_statement(
    ir_node: &input_ir::ExpressionStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> ExpressionStatement {
    Rc::new(ExpressionStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ExpressionStatementStruct {
    pub fn expression(&self) -> Expression {
        create_expression(&self.ir_node.expression, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type AssemblyStatement = Rc<AssemblyStatementStruct>;

pub struct AssemblyStatementStruct {
    pub(crate) ir_node: input_ir::AssemblyStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_assembly_statement(
    ir_node: &input_ir::AssemblyStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> AssemblyStatement {
    Rc::new(AssemblyStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AssemblyStatementStruct {
    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.semantic)
    }

    pub fn flags(&self) -> Vec<Rc<TerminalNode>> {
        self.ir_node.flags.clone()
    }

    pub fn label(&self) -> Option<Rc<TerminalNode>> {
        self.ir_node.label.as_ref().map(Rc::clone)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type TupleDeconstructionStatement = Rc<TupleDeconstructionStatementStruct>;

pub struct TupleDeconstructionStatementStruct {
    pub(crate) ir_node: input_ir::TupleDeconstructionStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_tuple_deconstruction_statement(
    ir_node: &input_ir::TupleDeconstructionStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> TupleDeconstructionStatement {
    Rc::new(TupleDeconstructionStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl TupleDeconstructionStatementStruct {
    pub fn expression(&self) -> Expression {
        create_expression(&self.ir_node.expression, &self.semantic)
    }

    pub fn members(&self) -> TupleDeconstructionMembers {
        create_tuple_deconstruction_members(&self.ir_node.members, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

pub struct VariableDeclarationStatementStruct {
    pub(crate) ir_node: input_ir::VariableDeclarationStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_variable_declaration_statement(
    ir_node: &input_ir::VariableDeclarationStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> VariableDeclarationStatement {
    Rc::new(VariableDeclarationStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl VariableDeclarationStatementStruct {
    pub fn storage_location(&self) -> Option<StorageLocation> {
        self.ir_node
            .storage_location
            .as_ref()
            .map(|ir_node| create_storage_location(ir_node, &self.semantic))
    }

    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn value(&self) -> Option<Expression> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn type_name(&self) -> Option<TypeName> {
        self.ir_node
            .type_name
            .as_ref()
            .map(|ir_node| create_type_name(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type IfStatement = Rc<IfStatementStruct>;

pub struct IfStatementStruct {
    pub(crate) ir_node: input_ir::IfStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_if_statement(
    ir_node: &input_ir::IfStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> IfStatement {
    Rc::new(IfStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl IfStatementStruct {
    pub fn condition(&self) -> Expression {
        create_expression(&self.ir_node.condition, &self.semantic)
    }

    pub fn body(&self) -> Statement {
        create_statement(&self.ir_node.body, &self.semantic)
    }

    pub fn else_branch(&self) -> Option<Statement> {
        self.ir_node
            .else_branch
            .as_ref()
            .map(|ir_node| create_statement(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ForStatement = Rc<ForStatementStruct>;

pub struct ForStatementStruct {
    pub(crate) ir_node: input_ir::ForStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_for_statement(
    ir_node: &input_ir::ForStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> ForStatement {
    Rc::new(ForStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ForStatementStruct {
    pub fn initialization(&self) -> ForStatementInitialization {
        create_for_statement_initialization(&self.ir_node.initialization, &self.semantic)
    }

    pub fn condition(&self) -> ForStatementCondition {
        create_for_statement_condition(&self.ir_node.condition, &self.semantic)
    }

    pub fn iterator(&self) -> Option<Expression> {
        self.ir_node
            .iterator
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn body(&self) -> Statement {
        create_statement(&self.ir_node.body, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type WhileStatement = Rc<WhileStatementStruct>;

pub struct WhileStatementStruct {
    pub(crate) ir_node: input_ir::WhileStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_while_statement(
    ir_node: &input_ir::WhileStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> WhileStatement {
    Rc::new(WhileStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl WhileStatementStruct {
    pub fn condition(&self) -> Expression {
        create_expression(&self.ir_node.condition, &self.semantic)
    }

    pub fn body(&self) -> Statement {
        create_statement(&self.ir_node.body, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type DoWhileStatement = Rc<DoWhileStatementStruct>;

pub struct DoWhileStatementStruct {
    pub(crate) ir_node: input_ir::DoWhileStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_do_while_statement(
    ir_node: &input_ir::DoWhileStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> DoWhileStatement {
    Rc::new(DoWhileStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl DoWhileStatementStruct {
    pub fn body(&self) -> Statement {
        create_statement(&self.ir_node.body, &self.semantic)
    }

    pub fn condition(&self) -> Expression {
        create_expression(&self.ir_node.condition, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ContinueStatement = Rc<ContinueStatementStruct>;

pub struct ContinueStatementStruct {
    pub(crate) ir_node: input_ir::ContinueStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_continue_statement(
    ir_node: &input_ir::ContinueStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> ContinueStatement {
    Rc::new(ContinueStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ContinueStatementStruct {
    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type BreakStatement = Rc<BreakStatementStruct>;

pub struct BreakStatementStruct {
    pub(crate) ir_node: input_ir::BreakStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_break_statement(
    ir_node: &input_ir::BreakStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> BreakStatement {
    Rc::new(BreakStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl BreakStatementStruct {
    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ReturnStatement = Rc<ReturnStatementStruct>;

pub struct ReturnStatementStruct {
    pub(crate) ir_node: input_ir::ReturnStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_return_statement(
    ir_node: &input_ir::ReturnStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> ReturnStatement {
    Rc::new(ReturnStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ReturnStatementStruct {
    pub fn expression(&self) -> Option<Expression> {
        self.ir_node
            .expression
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type EmitStatement = Rc<EmitStatementStruct>;

pub struct EmitStatementStruct {
    pub(crate) ir_node: input_ir::EmitStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_emit_statement(
    ir_node: &input_ir::EmitStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> EmitStatement {
    Rc::new(EmitStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl EmitStatementStruct {
    pub fn event(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.event, &self.semantic)
    }

    pub fn arguments(&self) -> ArgumentsDeclaration {
        create_arguments_declaration(&self.ir_node.arguments, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type TryStatement = Rc<TryStatementStruct>;

pub struct TryStatementStruct {
    pub(crate) ir_node: input_ir::TryStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_try_statement(
    ir_node: &input_ir::TryStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> TryStatement {
    Rc::new(TryStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl TryStatementStruct {
    pub fn expression(&self) -> Expression {
        create_expression(&self.ir_node.expression, &self.semantic)
    }

    pub fn returns(&self) -> Option<Parameters> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, &self.semantic))
    }

    pub fn body(&self) -> Block {
        create_block(&self.ir_node.body, &self.semantic)
    }

    pub fn catch_clauses(&self) -> CatchClauses {
        create_catch_clauses(&self.ir_node.catch_clauses, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type CatchClause = Rc<CatchClauseStruct>;

pub struct CatchClauseStruct {
    pub(crate) ir_node: input_ir::CatchClause,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_catch_clause(
    ir_node: &input_ir::CatchClause,
    semantic: &Rc<SemanticAnalysis>,
) -> CatchClause {
    Rc::new(CatchClauseStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl CatchClauseStruct {
    pub fn error(&self) -> Option<CatchClauseError> {
        self.ir_node
            .error
            .as_ref()
            .map(|ir_node| create_catch_clause_error(ir_node, &self.semantic))
    }

    pub fn body(&self) -> Block {
        create_block(&self.ir_node.body, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type CatchClauseError = Rc<CatchClauseErrorStruct>;

pub struct CatchClauseErrorStruct {
    pub(crate) ir_node: input_ir::CatchClauseError,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_catch_clause_error(
    ir_node: &input_ir::CatchClauseError,
    semantic: &Rc<SemanticAnalysis>,
) -> CatchClauseError {
    Rc::new(CatchClauseErrorStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl CatchClauseErrorStruct {
    pub fn name(&self) -> Option<Identifier> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
    }

    pub fn parameters(&self) -> Parameters {
        create_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type RevertStatement = Rc<RevertStatementStruct>;

pub struct RevertStatementStruct {
    pub(crate) ir_node: input_ir::RevertStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_revert_statement(
    ir_node: &input_ir::RevertStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> RevertStatement {
    Rc::new(RevertStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl RevertStatementStruct {
    pub fn error(&self) -> IdentifierPath {
        create_identifier_path(&self.ir_node.error, &self.semantic)
    }

    pub fn arguments(&self) -> ArgumentsDeclaration {
        create_arguments_declaration(&self.ir_node.arguments, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ThrowStatement = Rc<ThrowStatementStruct>;

pub struct ThrowStatementStruct {
    pub(crate) ir_node: input_ir::ThrowStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_throw_statement(
    ir_node: &input_ir::ThrowStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> ThrowStatement {
    Rc::new(ThrowStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ThrowStatementStruct {
    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type AssignmentExpression = Rc<AssignmentExpressionStruct>;

pub struct AssignmentExpressionStruct {
    pub(crate) ir_node: input_ir::AssignmentExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_assignment_expression(
    ir_node: &input_ir::AssignmentExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> AssignmentExpression {
    Rc::new(AssignmentExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AssignmentExpressionStruct {
    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ConditionalExpression = Rc<ConditionalExpressionStruct>;

pub struct ConditionalExpressionStruct {
    pub(crate) ir_node: input_ir::ConditionalExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_conditional_expression(
    ir_node: &input_ir::ConditionalExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> ConditionalExpression {
    Rc::new(ConditionalExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ConditionalExpressionStruct {
    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn true_expression(&self) -> Expression {
        create_expression(&self.ir_node.true_expression, &self.semantic)
    }

    pub fn false_expression(&self) -> Expression {
        create_expression(&self.ir_node.false_expression, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type OrExpression = Rc<OrExpressionStruct>;

pub struct OrExpressionStruct {
    pub(crate) ir_node: input_ir::OrExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_or_expression(
    ir_node: &input_ir::OrExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> OrExpression {
    Rc::new(OrExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl OrExpressionStruct {
    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type AndExpression = Rc<AndExpressionStruct>;

pub struct AndExpressionStruct {
    pub(crate) ir_node: input_ir::AndExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_and_expression(
    ir_node: &input_ir::AndExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> AndExpression {
    Rc::new(AndExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AndExpressionStruct {
    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type EqualityExpression = Rc<EqualityExpressionStruct>;

pub struct EqualityExpressionStruct {
    pub(crate) ir_node: input_ir::EqualityExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_equality_expression(
    ir_node: &input_ir::EqualityExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> EqualityExpression {
    Rc::new(EqualityExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl EqualityExpressionStruct {
    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type InequalityExpression = Rc<InequalityExpressionStruct>;

pub struct InequalityExpressionStruct {
    pub(crate) ir_node: input_ir::InequalityExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_inequality_expression(
    ir_node: &input_ir::InequalityExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> InequalityExpression {
    Rc::new(InequalityExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl InequalityExpressionStruct {
    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type BitwiseOrExpression = Rc<BitwiseOrExpressionStruct>;

pub struct BitwiseOrExpressionStruct {
    pub(crate) ir_node: input_ir::BitwiseOrExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_bitwise_or_expression(
    ir_node: &input_ir::BitwiseOrExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> BitwiseOrExpression {
    Rc::new(BitwiseOrExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl BitwiseOrExpressionStruct {
    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type BitwiseXorExpression = Rc<BitwiseXorExpressionStruct>;

pub struct BitwiseXorExpressionStruct {
    pub(crate) ir_node: input_ir::BitwiseXorExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_bitwise_xor_expression(
    ir_node: &input_ir::BitwiseXorExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> BitwiseXorExpression {
    Rc::new(BitwiseXorExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl BitwiseXorExpressionStruct {
    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type BitwiseAndExpression = Rc<BitwiseAndExpressionStruct>;

pub struct BitwiseAndExpressionStruct {
    pub(crate) ir_node: input_ir::BitwiseAndExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_bitwise_and_expression(
    ir_node: &input_ir::BitwiseAndExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> BitwiseAndExpression {
    Rc::new(BitwiseAndExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl BitwiseAndExpressionStruct {
    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ShiftExpression = Rc<ShiftExpressionStruct>;

pub struct ShiftExpressionStruct {
    pub(crate) ir_node: input_ir::ShiftExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_shift_expression(
    ir_node: &input_ir::ShiftExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> ShiftExpression {
    Rc::new(ShiftExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ShiftExpressionStruct {
    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type AdditiveExpression = Rc<AdditiveExpressionStruct>;

pub struct AdditiveExpressionStruct {
    pub(crate) ir_node: input_ir::AdditiveExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_additive_expression(
    ir_node: &input_ir::AdditiveExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> AdditiveExpression {
    Rc::new(AdditiveExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl AdditiveExpressionStruct {
    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type MultiplicativeExpression = Rc<MultiplicativeExpressionStruct>;

pub struct MultiplicativeExpressionStruct {
    pub(crate) ir_node: input_ir::MultiplicativeExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_multiplicative_expression(
    ir_node: &input_ir::MultiplicativeExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> MultiplicativeExpression {
    Rc::new(MultiplicativeExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl MultiplicativeExpressionStruct {
    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ExponentiationExpression = Rc<ExponentiationExpressionStruct>;

pub struct ExponentiationExpressionStruct {
    pub(crate) ir_node: input_ir::ExponentiationExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_exponentiation_expression(
    ir_node: &input_ir::ExponentiationExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> ExponentiationExpression {
    Rc::new(ExponentiationExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ExponentiationExpressionStruct {
    pub fn left_operand(&self) -> Expression {
        create_expression(&self.ir_node.left_operand, &self.semantic)
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn right_operand(&self) -> Expression {
        create_expression(&self.ir_node.right_operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type PostfixExpression = Rc<PostfixExpressionStruct>;

pub struct PostfixExpressionStruct {
    pub(crate) ir_node: input_ir::PostfixExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_postfix_expression(
    ir_node: &input_ir::PostfixExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> PostfixExpression {
    Rc::new(PostfixExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PostfixExpressionStruct {
    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type PrefixExpression = Rc<PrefixExpressionStruct>;

pub struct PrefixExpressionStruct {
    pub(crate) ir_node: input_ir::PrefixExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_prefix_expression(
    ir_node: &input_ir::PrefixExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> PrefixExpression {
    Rc::new(PrefixExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl PrefixExpressionStruct {
    pub fn operator(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.operator)
    }

    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type FunctionCallExpression = Rc<FunctionCallExpressionStruct>;

pub struct FunctionCallExpressionStruct {
    pub(crate) ir_node: input_ir::FunctionCallExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_function_call_expression(
    ir_node: &input_ir::FunctionCallExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> FunctionCallExpression {
    Rc::new(FunctionCallExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl FunctionCallExpressionStruct {
    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn arguments(&self) -> ArgumentsDeclaration {
        create_arguments_declaration(&self.ir_node.arguments, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type CallOptionsExpression = Rc<CallOptionsExpressionStruct>;

pub struct CallOptionsExpressionStruct {
    pub(crate) ir_node: input_ir::CallOptionsExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_call_options_expression(
    ir_node: &input_ir::CallOptionsExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> CallOptionsExpression {
    Rc::new(CallOptionsExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl CallOptionsExpressionStruct {
    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn options(&self) -> CallOptions {
        create_call_options(&self.ir_node.options, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

pub struct MemberAccessExpressionStruct {
    pub(crate) ir_node: input_ir::MemberAccessExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_member_access_expression(
    ir_node: &input_ir::MemberAccessExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> MemberAccessExpression {
    Rc::new(MemberAccessExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl MemberAccessExpressionStruct {
    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn member(&self) -> Identifier {
        create_identifier(&self.ir_node.member, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type IndexAccessExpression = Rc<IndexAccessExpressionStruct>;

pub struct IndexAccessExpressionStruct {
    pub(crate) ir_node: input_ir::IndexAccessExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_index_access_expression(
    ir_node: &input_ir::IndexAccessExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> IndexAccessExpression {
    Rc::new(IndexAccessExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl IndexAccessExpressionStruct {
    pub fn operand(&self) -> Expression {
        create_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn start(&self) -> Option<Expression> {
        self.ir_node
            .start
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn end(&self) -> Option<Expression> {
        self.ir_node
            .end
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type NamedArgument = Rc<NamedArgumentStruct>;

pub struct NamedArgumentStruct {
    pub(crate) ir_node: input_ir::NamedArgument,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_named_argument(
    ir_node: &input_ir::NamedArgument,
    semantic: &Rc<SemanticAnalysis>,
) -> NamedArgument {
    Rc::new(NamedArgumentStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl NamedArgumentStruct {
    pub fn name(&self) -> Identifier {
        create_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn value(&self) -> Expression {
        create_expression(&self.ir_node.value, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type TypeExpression = Rc<TypeExpressionStruct>;

pub struct TypeExpressionStruct {
    pub(crate) ir_node: input_ir::TypeExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_type_expression(
    ir_node: &input_ir::TypeExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> TypeExpression {
    Rc::new(TypeExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl TypeExpressionStruct {
    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type NewExpression = Rc<NewExpressionStruct>;

pub struct NewExpressionStruct {
    pub(crate) ir_node: input_ir::NewExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_new_expression(
    ir_node: &input_ir::NewExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> NewExpression {
    Rc::new(NewExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl NewExpressionStruct {
    pub fn type_name(&self) -> TypeName {
        create_type_name(&self.ir_node.type_name, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type TupleExpression = Rc<TupleExpressionStruct>;

pub struct TupleExpressionStruct {
    pub(crate) ir_node: input_ir::TupleExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_tuple_expression(
    ir_node: &input_ir::TupleExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> TupleExpression {
    Rc::new(TupleExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl TupleExpressionStruct {
    pub fn items(&self) -> TupleValues {
        create_tuple_values(&self.ir_node.items, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type TupleValue = Rc<TupleValueStruct>;

pub struct TupleValueStruct {
    pub(crate) ir_node: input_ir::TupleValue,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_tuple_value(
    ir_node: &input_ir::TupleValue,
    semantic: &Rc<SemanticAnalysis>,
) -> TupleValue {
    Rc::new(TupleValueStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl TupleValueStruct {
    pub fn expression(&self) -> Option<Expression> {
        self.ir_node
            .expression
            .as_ref()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type ArrayExpression = Rc<ArrayExpressionStruct>;

pub struct ArrayExpressionStruct {
    pub(crate) ir_node: input_ir::ArrayExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_array_expression(
    ir_node: &input_ir::ArrayExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> ArrayExpression {
    Rc::new(ArrayExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl ArrayExpressionStruct {
    pub fn items(&self) -> ArrayValues {
        create_array_values(&self.ir_node.items, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type HexNumberExpression = Rc<HexNumberExpressionStruct>;

pub struct HexNumberExpressionStruct {
    pub(crate) ir_node: input_ir::HexNumberExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_hex_number_expression(
    ir_node: &input_ir::HexNumberExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> HexNumberExpression {
    Rc::new(HexNumberExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl HexNumberExpressionStruct {
    pub fn literal(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.literal)
    }

    pub fn unit(&self) -> Option<NumberUnit> {
        self.ir_node
            .unit
            .as_ref()
            .map(|ir_node| create_number_unit(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type DecimalNumberExpression = Rc<DecimalNumberExpressionStruct>;

pub struct DecimalNumberExpressionStruct {
    pub(crate) ir_node: input_ir::DecimalNumberExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_decimal_number_expression(
    ir_node: &input_ir::DecimalNumberExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> DecimalNumberExpression {
    Rc::new(DecimalNumberExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl DecimalNumberExpressionStruct {
    pub fn literal(&self) -> Rc<TerminalNode> {
        Rc::clone(&self.ir_node.literal)
    }

    pub fn unit(&self) -> Option<NumberUnit> {
        self.ir_node
            .unit
            .as_ref()
            .map(|ir_node| create_number_unit(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulBlock = Rc<YulBlockStruct>;

pub struct YulBlockStruct {
    pub(crate) ir_node: input_ir::YulBlock,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_block(
    ir_node: &input_ir::YulBlock,
    semantic: &Rc<SemanticAnalysis>,
) -> YulBlock {
    Rc::new(YulBlockStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulBlockStruct {
    pub fn statements(&self) -> YulStatements {
        create_yul_statements(&self.ir_node.statements, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulFunctionDefinition = Rc<YulFunctionDefinitionStruct>;

pub struct YulFunctionDefinitionStruct {
    pub(crate) ir_node: input_ir::YulFunctionDefinition,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_function_definition(
    ir_node: &input_ir::YulFunctionDefinition,
    semantic: &Rc<SemanticAnalysis>,
) -> YulFunctionDefinition {
    Rc::new(YulFunctionDefinitionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulFunctionDefinitionStruct {
    pub fn name(&self) -> YulIdentifier {
        create_yul_identifier(&self.ir_node.name, &self.semantic)
    }

    pub fn parameters(&self) -> YulParameters {
        create_yul_parameters(&self.ir_node.parameters, &self.semantic)
    }

    pub fn returns(&self) -> Option<YulVariableNames> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_yul_variable_names(ir_node, &self.semantic))
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulVariableDeclarationStatement = Rc<YulVariableDeclarationStatementStruct>;

pub struct YulVariableDeclarationStatementStruct {
    pub(crate) ir_node: input_ir::YulVariableDeclarationStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_variable_declaration_statement(
    ir_node: &input_ir::YulVariableDeclarationStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> YulVariableDeclarationStatement {
    Rc::new(YulVariableDeclarationStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulVariableDeclarationStatementStruct {
    pub fn variables(&self) -> YulVariableNames {
        create_yul_variable_names(&self.ir_node.variables, &self.semantic)
    }

    pub fn value(&self) -> Option<YulVariableDeclarationValue> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_yul_variable_declaration_value(ir_node, &self.semantic))
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulVariableDeclarationValue = Rc<YulVariableDeclarationValueStruct>;

pub struct YulVariableDeclarationValueStruct {
    pub(crate) ir_node: input_ir::YulVariableDeclarationValue,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_variable_declaration_value(
    ir_node: &input_ir::YulVariableDeclarationValue,
    semantic: &Rc<SemanticAnalysis>,
) -> YulVariableDeclarationValue {
    Rc::new(YulVariableDeclarationValueStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulVariableDeclarationValueStruct {
    pub fn assignment(&self) -> YulAssignmentOperator {
        create_yul_assignment_operator(&self.ir_node.assignment, &self.semantic)
    }

    pub fn expression(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.expression, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulVariableAssignmentStatement = Rc<YulVariableAssignmentStatementStruct>;

pub struct YulVariableAssignmentStatementStruct {
    pub(crate) ir_node: input_ir::YulVariableAssignmentStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_variable_assignment_statement(
    ir_node: &input_ir::YulVariableAssignmentStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> YulVariableAssignmentStatement {
    Rc::new(YulVariableAssignmentStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulVariableAssignmentStatementStruct {
    pub fn variables(&self) -> YulPaths {
        create_yul_paths(&self.ir_node.variables, &self.semantic)
    }

    pub fn assignment(&self) -> YulAssignmentOperator {
        create_yul_assignment_operator(&self.ir_node.assignment, &self.semantic)
    }

    pub fn expression(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.expression, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulColonAndEqual = Rc<YulColonAndEqualStruct>;

pub struct YulColonAndEqualStruct {
    pub(crate) ir_node: input_ir::YulColonAndEqual,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_colon_and_equal(
    ir_node: &input_ir::YulColonAndEqual,
    semantic: &Rc<SemanticAnalysis>,
) -> YulColonAndEqual {
    Rc::new(YulColonAndEqualStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulColonAndEqualStruct {
    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulStackAssignmentStatement = Rc<YulStackAssignmentStatementStruct>;

pub struct YulStackAssignmentStatementStruct {
    pub(crate) ir_node: input_ir::YulStackAssignmentStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_stack_assignment_statement(
    ir_node: &input_ir::YulStackAssignmentStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> YulStackAssignmentStatement {
    Rc::new(YulStackAssignmentStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulStackAssignmentStatementStruct {
    pub fn assignment(&self) -> YulStackAssignmentOperator {
        create_yul_stack_assignment_operator(&self.ir_node.assignment, &self.semantic)
    }

    pub fn variable(&self) -> YulIdentifier {
        create_yul_identifier(&self.ir_node.variable, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulEqualAndColon = Rc<YulEqualAndColonStruct>;

pub struct YulEqualAndColonStruct {
    pub(crate) ir_node: input_ir::YulEqualAndColon,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_equal_and_colon(
    ir_node: &input_ir::YulEqualAndColon,
    semantic: &Rc<SemanticAnalysis>,
) -> YulEqualAndColon {
    Rc::new(YulEqualAndColonStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulEqualAndColonStruct {
    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulIfStatement = Rc<YulIfStatementStruct>;

pub struct YulIfStatementStruct {
    pub(crate) ir_node: input_ir::YulIfStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_if_statement(
    ir_node: &input_ir::YulIfStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> YulIfStatement {
    Rc::new(YulIfStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulIfStatementStruct {
    pub fn condition(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.condition, &self.semantic)
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulForStatement = Rc<YulForStatementStruct>;

pub struct YulForStatementStruct {
    pub(crate) ir_node: input_ir::YulForStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_for_statement(
    ir_node: &input_ir::YulForStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> YulForStatement {
    Rc::new(YulForStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulForStatementStruct {
    pub fn initialization(&self) -> YulBlock {
        create_yul_block(&self.ir_node.initialization, &self.semantic)
    }

    pub fn condition(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.condition, &self.semantic)
    }

    pub fn iterator(&self) -> YulBlock {
        create_yul_block(&self.ir_node.iterator, &self.semantic)
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulSwitchStatement = Rc<YulSwitchStatementStruct>;

pub struct YulSwitchStatementStruct {
    pub(crate) ir_node: input_ir::YulSwitchStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_switch_statement(
    ir_node: &input_ir::YulSwitchStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> YulSwitchStatement {
    Rc::new(YulSwitchStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulSwitchStatementStruct {
    pub fn expression(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.expression, &self.semantic)
    }

    pub fn cases(&self) -> YulSwitchCases {
        create_yul_switch_cases(&self.ir_node.cases, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulDefaultCase = Rc<YulDefaultCaseStruct>;

pub struct YulDefaultCaseStruct {
    pub(crate) ir_node: input_ir::YulDefaultCase,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_default_case(
    ir_node: &input_ir::YulDefaultCase,
    semantic: &Rc<SemanticAnalysis>,
) -> YulDefaultCase {
    Rc::new(YulDefaultCaseStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulDefaultCaseStruct {
    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulValueCase = Rc<YulValueCaseStruct>;

pub struct YulValueCaseStruct {
    pub(crate) ir_node: input_ir::YulValueCase,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_value_case(
    ir_node: &input_ir::YulValueCase,
    semantic: &Rc<SemanticAnalysis>,
) -> YulValueCase {
    Rc::new(YulValueCaseStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulValueCaseStruct {
    pub fn value(&self) -> YulLiteral {
        create_yul_literal(&self.ir_node.value, &self.semantic)
    }

    pub fn body(&self) -> YulBlock {
        create_yul_block(&self.ir_node.body, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulLeaveStatement = Rc<YulLeaveStatementStruct>;

pub struct YulLeaveStatementStruct {
    pub(crate) ir_node: input_ir::YulLeaveStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_leave_statement(
    ir_node: &input_ir::YulLeaveStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> YulLeaveStatement {
    Rc::new(YulLeaveStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulLeaveStatementStruct {
    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulBreakStatement = Rc<YulBreakStatementStruct>;

pub struct YulBreakStatementStruct {
    pub(crate) ir_node: input_ir::YulBreakStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_break_statement(
    ir_node: &input_ir::YulBreakStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> YulBreakStatement {
    Rc::new(YulBreakStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulBreakStatementStruct {
    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulContinueStatement = Rc<YulContinueStatementStruct>;

pub struct YulContinueStatementStruct {
    pub(crate) ir_node: input_ir::YulContinueStatement,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_continue_statement(
    ir_node: &input_ir::YulContinueStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> YulContinueStatement {
    Rc::new(YulContinueStatementStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulContinueStatementStruct {
    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulLabel = Rc<YulLabelStruct>;

pub struct YulLabelStruct {
    pub(crate) ir_node: input_ir::YulLabel,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_label(
    ir_node: &input_ir::YulLabel,
    semantic: &Rc<SemanticAnalysis>,
) -> YulLabel {
    Rc::new(YulLabelStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulLabelStruct {
    pub fn label(&self) -> YulIdentifier {
        create_yul_identifier(&self.ir_node.label, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

pub type YulFunctionCallExpression = Rc<YulFunctionCallExpressionStruct>;

pub struct YulFunctionCallExpressionStruct {
    pub(crate) ir_node: input_ir::YulFunctionCallExpression,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

pub(crate) fn create_yul_function_call_expression(
    ir_node: &input_ir::YulFunctionCallExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> YulFunctionCallExpression {
    Rc::new(YulFunctionCallExpressionStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

impl YulFunctionCallExpressionStruct {
    pub fn operand(&self) -> YulExpression {
        create_yul_expression(&self.ir_node.operand, &self.semantic)
    }

    pub fn arguments(&self) -> YulArguments {
        create_yul_arguments(&self.ir_node.arguments, &self.semantic)
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.node_id)
            .unwrap()
    }
}

//
// Choices:
//

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

#[allow(clippy::too_many_lines)]
pub(crate) fn create_source_unit_member(
    ir_node: &input_ir::SourceUnitMember,
    semantic: &Rc<SemanticAnalysis>,
) -> SourceUnitMember {
    match ir_node {
        input_ir::SourceUnitMember::PragmaDirective(variant) => {
            SourceUnitMember::PragmaDirective(create_pragma_directive(variant, semantic))
        }
        input_ir::SourceUnitMember::ContractDefinition(variant) => {
            SourceUnitMember::ContractDefinition(create_contract_definition(variant, semantic))
        }
        input_ir::SourceUnitMember::InterfaceDefinition(variant) => {
            SourceUnitMember::InterfaceDefinition(create_interface_definition(variant, semantic))
        }
        input_ir::SourceUnitMember::LibraryDefinition(variant) => {
            SourceUnitMember::LibraryDefinition(create_library_definition(variant, semantic))
        }
        input_ir::SourceUnitMember::StructDefinition(variant) => {
            SourceUnitMember::StructDefinition(create_struct_definition(variant, semantic))
        }
        input_ir::SourceUnitMember::EnumDefinition(variant) => {
            SourceUnitMember::EnumDefinition(create_enum_definition(variant, semantic))
        }
        input_ir::SourceUnitMember::FunctionDefinition(variant) => {
            SourceUnitMember::FunctionDefinition(create_function_definition(variant, semantic))
        }
        input_ir::SourceUnitMember::ErrorDefinition(variant) => {
            SourceUnitMember::ErrorDefinition(create_error_definition(variant, semantic))
        }
        input_ir::SourceUnitMember::UserDefinedValueTypeDefinition(variant) => {
            SourceUnitMember::UserDefinedValueTypeDefinition(
                create_user_defined_value_type_definition(variant, semantic),
            )
        }
        input_ir::SourceUnitMember::UsingDirective(variant) => {
            SourceUnitMember::UsingDirective(create_using_directive(variant, semantic))
        }
        input_ir::SourceUnitMember::EventDefinition(variant) => {
            SourceUnitMember::EventDefinition(create_event_definition(variant, semantic))
        }
        input_ir::SourceUnitMember::ConstantDefinition(variant) => {
            SourceUnitMember::ConstantDefinition(create_constant_definition(variant, semantic))
        }
        input_ir::SourceUnitMember::ImportClause(variant) => {
            SourceUnitMember::ImportClause(create_import_clause(variant, semantic))
        }
    }
}

pub enum Pragma {
    VersionPragma(VersionPragma),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_pragma(ir_node: &input_ir::Pragma, semantic: &Rc<SemanticAnalysis>) -> Pragma {
    match ir_node {
        input_ir::Pragma::VersionPragma(variant) => {
            Pragma::VersionPragma(create_version_pragma(variant, semantic))
        }
        input_ir::Pragma::AbicoderPragma(variant) => {
            Pragma::AbicoderPragma(create_abicoder_pragma(variant, semantic))
        }
        input_ir::Pragma::ExperimentalPragma(variant) => {
            Pragma::ExperimentalPragma(create_experimental_pragma(variant, semantic))
        }
    }
}

pub enum AbicoderVersion {
    AbicoderV1Keyword,
    AbicoderV2Keyword,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_abicoder_version(
    ir_node: &input_ir::AbicoderVersion,
    semantic: &Rc<SemanticAnalysis>,
) -> AbicoderVersion {
    match ir_node {
        input_ir::AbicoderVersion::AbicoderV1Keyword => AbicoderVersion::AbicoderV1Keyword,
        input_ir::AbicoderVersion::AbicoderV2Keyword => AbicoderVersion::AbicoderV2Keyword,
    }
}

pub enum ExperimentalFeature {
    ABIEncoderV2Keyword,
    SMTCheckerKeyword,
    StringLiteral(Rc<TerminalNode>),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_experimental_feature(
    ir_node: &input_ir::ExperimentalFeature,
    semantic: &Rc<SemanticAnalysis>,
) -> ExperimentalFeature {
    match ir_node {
        input_ir::ExperimentalFeature::ABIEncoderV2Keyword => {
            ExperimentalFeature::ABIEncoderV2Keyword
        }
        input_ir::ExperimentalFeature::SMTCheckerKeyword => ExperimentalFeature::SMTCheckerKeyword,
        input_ir::ExperimentalFeature::StringLiteral(node) => {
            ExperimentalFeature::StringLiteral(Rc::clone(node))
        }
    }
}

pub enum VersionExpression {
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_version_expression(
    ir_node: &input_ir::VersionExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> VersionExpression {
    match ir_node {
        input_ir::VersionExpression::VersionRange(variant) => {
            VersionExpression::VersionRange(create_version_range(variant, semantic))
        }
        input_ir::VersionExpression::VersionTerm(variant) => {
            VersionExpression::VersionTerm(create_version_term(variant, semantic))
        }
    }
}

pub enum VersionOperator {
    Caret,
    Tilde,
    Equal,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_version_operator(
    ir_node: &input_ir::VersionOperator,
    semantic: &Rc<SemanticAnalysis>,
) -> VersionOperator {
    match ir_node {
        input_ir::VersionOperator::Caret => VersionOperator::Caret,
        input_ir::VersionOperator::Tilde => VersionOperator::Tilde,
        input_ir::VersionOperator::Equal => VersionOperator::Equal,
        input_ir::VersionOperator::LessThan => VersionOperator::LessThan,
        input_ir::VersionOperator::GreaterThan => VersionOperator::GreaterThan,
        input_ir::VersionOperator::LessThanEqual => VersionOperator::LessThanEqual,
        input_ir::VersionOperator::GreaterThanEqual => VersionOperator::GreaterThanEqual,
    }
}

pub enum VersionLiteral {
    SimpleVersionLiteral(Vec<Rc<TerminalNode>>),
    SingleQuotedVersionLiteral(Rc<TerminalNode>),
    DoubleQuotedVersionLiteral(Rc<TerminalNode>),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_version_literal(
    ir_node: &input_ir::VersionLiteral,
    semantic: &Rc<SemanticAnalysis>,
) -> VersionLiteral {
    match ir_node {
        input_ir::VersionLiteral::SimpleVersionLiteral(nodes) => {
            VersionLiteral::SimpleVersionLiteral(nodes.clone())
        }

        input_ir::VersionLiteral::SingleQuotedVersionLiteral(node) => {
            VersionLiteral::SingleQuotedVersionLiteral(Rc::clone(node))
        }
        input_ir::VersionLiteral::DoubleQuotedVersionLiteral(node) => {
            VersionLiteral::DoubleQuotedVersionLiteral(Rc::clone(node))
        }
    }
}

pub enum ImportClause {
    PathImport(PathImport),
    ImportDeconstruction(ImportDeconstruction),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_import_clause(
    ir_node: &input_ir::ImportClause,
    semantic: &Rc<SemanticAnalysis>,
) -> ImportClause {
    match ir_node {
        input_ir::ImportClause::PathImport(variant) => {
            ImportClause::PathImport(create_path_import(variant, semantic))
        }
        input_ir::ImportClause::ImportDeconstruction(variant) => {
            ImportClause::ImportDeconstruction(create_import_deconstruction(variant, semantic))
        }
    }
}

pub enum UsingClause {
    IdentifierPath(IdentifierPath),
    UsingDeconstruction(UsingDeconstruction),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_using_clause(
    ir_node: &input_ir::UsingClause,
    semantic: &Rc<SemanticAnalysis>,
) -> UsingClause {
    match ir_node {
        input_ir::UsingClause::IdentifierPath(nodes) => {
            UsingClause::IdentifierPath(create_identifier_path(nodes, semantic))
        }

        input_ir::UsingClause::UsingDeconstruction(variant) => {
            UsingClause::UsingDeconstruction(create_using_deconstruction(variant, semantic))
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
    ir_node: &input_ir::UsingOperator,
    semantic: &Rc<SemanticAnalysis>,
) -> UsingOperator {
    match ir_node {
        input_ir::UsingOperator::Ampersand => UsingOperator::Ampersand,
        input_ir::UsingOperator::Asterisk => UsingOperator::Asterisk,
        input_ir::UsingOperator::BangEqual => UsingOperator::BangEqual,
        input_ir::UsingOperator::Bar => UsingOperator::Bar,
        input_ir::UsingOperator::Caret => UsingOperator::Caret,
        input_ir::UsingOperator::EqualEqual => UsingOperator::EqualEqual,
        input_ir::UsingOperator::GreaterThan => UsingOperator::GreaterThan,
        input_ir::UsingOperator::GreaterThanEqual => UsingOperator::GreaterThanEqual,
        input_ir::UsingOperator::LessThan => UsingOperator::LessThan,
        input_ir::UsingOperator::LessThanEqual => UsingOperator::LessThanEqual,
        input_ir::UsingOperator::Minus => UsingOperator::Minus,
        input_ir::UsingOperator::Percent => UsingOperator::Percent,
        input_ir::UsingOperator::Plus => UsingOperator::Plus,
        input_ir::UsingOperator::Slash => UsingOperator::Slash,
        input_ir::UsingOperator::Tilde => UsingOperator::Tilde,
    }
}

pub enum UsingTarget {
    TypeName(TypeName),
    Asterisk,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_using_target(
    ir_node: &input_ir::UsingTarget,
    semantic: &Rc<SemanticAnalysis>,
) -> UsingTarget {
    match ir_node {
        input_ir::UsingTarget::TypeName(variant) => {
            UsingTarget::TypeName(create_type_name(variant, semantic))
        }
        input_ir::UsingTarget::Asterisk => UsingTarget::Asterisk,
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
    ir_node: &input_ir::ContractMember,
    semantic: &Rc<SemanticAnalysis>,
) -> ContractMember {
    match ir_node {
        input_ir::ContractMember::UsingDirective(variant) => {
            ContractMember::UsingDirective(create_using_directive(variant, semantic))
        }
        input_ir::ContractMember::FunctionDefinition(variant) => {
            ContractMember::FunctionDefinition(create_function_definition(variant, semantic))
        }
        input_ir::ContractMember::StructDefinition(variant) => {
            ContractMember::StructDefinition(create_struct_definition(variant, semantic))
        }
        input_ir::ContractMember::EnumDefinition(variant) => {
            ContractMember::EnumDefinition(create_enum_definition(variant, semantic))
        }
        input_ir::ContractMember::EventDefinition(variant) => {
            ContractMember::EventDefinition(create_event_definition(variant, semantic))
        }
        input_ir::ContractMember::ErrorDefinition(variant) => {
            ContractMember::ErrorDefinition(create_error_definition(variant, semantic))
        }
        input_ir::ContractMember::UserDefinedValueTypeDefinition(variant) => {
            ContractMember::UserDefinedValueTypeDefinition(
                create_user_defined_value_type_definition(variant, semantic),
            )
        }
        input_ir::ContractMember::StateVariableDefinition(variant) => {
            ContractMember::StateVariableDefinition(create_state_variable_definition(
                variant, semantic,
            ))
        }
        input_ir::ContractMember::ConstantDefinition(variant) => {
            ContractMember::ConstantDefinition(create_constant_definition(variant, semantic))
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
    ir_node: &input_ir::TypeName,
    semantic: &Rc<SemanticAnalysis>,
) -> TypeName {
    match ir_node {
        input_ir::TypeName::ArrayTypeName(variant) => {
            TypeName::ArrayTypeName(create_array_type_name(variant, semantic))
        }
        input_ir::TypeName::FunctionType(variant) => {
            TypeName::FunctionType(create_function_type(variant, semantic))
        }
        input_ir::TypeName::MappingType(variant) => {
            TypeName::MappingType(create_mapping_type(variant, semantic))
        }
        input_ir::TypeName::ElementaryType(variant) => {
            TypeName::ElementaryType(create_elementary_type(variant, semantic))
        }
        input_ir::TypeName::IdentifierPath(nodes) => {
            TypeName::IdentifierPath(create_identifier_path(nodes, semantic))
        }
    }
}

pub enum ElementaryType {
    BoolKeyword,
    ByteKeyword,
    StringKeyword,
    AddressType(AddressType),
    BytesKeyword(Rc<TerminalNode>),
    IntKeyword(Rc<TerminalNode>),
    UintKeyword(Rc<TerminalNode>),
    FixedKeyword(Rc<TerminalNode>),
    UfixedKeyword(Rc<TerminalNode>),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_elementary_type(
    ir_node: &input_ir::ElementaryType,
    semantic: &Rc<SemanticAnalysis>,
) -> ElementaryType {
    match ir_node {
        input_ir::ElementaryType::BoolKeyword => ElementaryType::BoolKeyword,
        input_ir::ElementaryType::ByteKeyword => ElementaryType::ByteKeyword,
        input_ir::ElementaryType::StringKeyword => ElementaryType::StringKeyword,
        input_ir::ElementaryType::AddressType(variant) => {
            ElementaryType::AddressType(create_address_type(variant, semantic))
        }
        input_ir::ElementaryType::BytesKeyword(node) => {
            ElementaryType::BytesKeyword(Rc::clone(node))
        }
        input_ir::ElementaryType::IntKeyword(node) => ElementaryType::IntKeyword(Rc::clone(node)),
        input_ir::ElementaryType::UintKeyword(node) => ElementaryType::UintKeyword(Rc::clone(node)),
        input_ir::ElementaryType::FixedKeyword(node) => {
            ElementaryType::FixedKeyword(Rc::clone(node))
        }
        input_ir::ElementaryType::UfixedKeyword(node) => {
            ElementaryType::UfixedKeyword(Rc::clone(node))
        }
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

#[allow(clippy::too_many_lines)]
pub(crate) fn create_statement(
    ir_node: &input_ir::Statement,
    semantic: &Rc<SemanticAnalysis>,
) -> Statement {
    match ir_node {
        input_ir::Statement::IfStatement(variant) => {
            Statement::IfStatement(create_if_statement(variant, semantic))
        }
        input_ir::Statement::ForStatement(variant) => {
            Statement::ForStatement(create_for_statement(variant, semantic))
        }
        input_ir::Statement::WhileStatement(variant) => {
            Statement::WhileStatement(create_while_statement(variant, semantic))
        }
        input_ir::Statement::DoWhileStatement(variant) => {
            Statement::DoWhileStatement(create_do_while_statement(variant, semantic))
        }
        input_ir::Statement::ContinueStatement(variant) => {
            Statement::ContinueStatement(create_continue_statement(variant, semantic))
        }
        input_ir::Statement::BreakStatement(variant) => {
            Statement::BreakStatement(create_break_statement(variant, semantic))
        }
        input_ir::Statement::ReturnStatement(variant) => {
            Statement::ReturnStatement(create_return_statement(variant, semantic))
        }
        input_ir::Statement::ThrowStatement(variant) => {
            Statement::ThrowStatement(create_throw_statement(variant, semantic))
        }
        input_ir::Statement::EmitStatement(variant) => {
            Statement::EmitStatement(create_emit_statement(variant, semantic))
        }
        input_ir::Statement::TryStatement(variant) => {
            Statement::TryStatement(create_try_statement(variant, semantic))
        }
        input_ir::Statement::RevertStatement(variant) => {
            Statement::RevertStatement(create_revert_statement(variant, semantic))
        }
        input_ir::Statement::AssemblyStatement(variant) => {
            Statement::AssemblyStatement(create_assembly_statement(variant, semantic))
        }
        input_ir::Statement::Block(variant) => Statement::Block(create_block(variant, semantic)),
        input_ir::Statement::UncheckedBlock(variant) => {
            Statement::UncheckedBlock(create_unchecked_block(variant, semantic))
        }
        input_ir::Statement::TupleDeconstructionStatement(variant) => {
            Statement::TupleDeconstructionStatement(create_tuple_deconstruction_statement(
                variant, semantic,
            ))
        }
        input_ir::Statement::VariableDeclarationStatement(variant) => {
            Statement::VariableDeclarationStatement(create_variable_declaration_statement(
                variant, semantic,
            ))
        }
        input_ir::Statement::ExpressionStatement(variant) => {
            Statement::ExpressionStatement(create_expression_statement(variant, semantic))
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
    ir_node: &input_ir::StorageLocation,
    semantic: &Rc<SemanticAnalysis>,
) -> StorageLocation {
    match ir_node {
        input_ir::StorageLocation::MemoryKeyword => StorageLocation::MemoryKeyword,
        input_ir::StorageLocation::StorageKeyword => StorageLocation::StorageKeyword,
        input_ir::StorageLocation::CallDataKeyword => StorageLocation::CallDataKeyword,
    }
}

pub enum ForStatementInitialization {
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_for_statement_initialization(
    ir_node: &input_ir::ForStatementInitialization,
    semantic: &Rc<SemanticAnalysis>,
) -> ForStatementInitialization {
    match ir_node {
        input_ir::ForStatementInitialization::TupleDeconstructionStatement(variant) => {
            ForStatementInitialization::TupleDeconstructionStatement(
                create_tuple_deconstruction_statement(variant, semantic),
            )
        }
        input_ir::ForStatementInitialization::VariableDeclarationStatement(variant) => {
            ForStatementInitialization::VariableDeclarationStatement(
                create_variable_declaration_statement(variant, semantic),
            )
        }
        input_ir::ForStatementInitialization::ExpressionStatement(variant) => {
            ForStatementInitialization::ExpressionStatement(create_expression_statement(
                variant, semantic,
            ))
        }
        input_ir::ForStatementInitialization::Semicolon => ForStatementInitialization::Semicolon,
    }
}

pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_for_statement_condition(
    ir_node: &input_ir::ForStatementCondition,
    semantic: &Rc<SemanticAnalysis>,
) -> ForStatementCondition {
    match ir_node {
        input_ir::ForStatementCondition::ExpressionStatement(variant) => {
            ForStatementCondition::ExpressionStatement(create_expression_statement(
                variant, semantic,
            ))
        }
        input_ir::ForStatementCondition::Semicolon => ForStatementCondition::Semicolon,
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
    ir_node: &input_ir::Expression,
    semantic: &Rc<SemanticAnalysis>,
) -> Expression {
    match ir_node {
        input_ir::Expression::AssignmentExpression(variant) => {
            Expression::AssignmentExpression(create_assignment_expression(variant, semantic))
        }
        input_ir::Expression::ConditionalExpression(variant) => {
            Expression::ConditionalExpression(create_conditional_expression(variant, semantic))
        }
        input_ir::Expression::OrExpression(variant) => {
            Expression::OrExpression(create_or_expression(variant, semantic))
        }
        input_ir::Expression::AndExpression(variant) => {
            Expression::AndExpression(create_and_expression(variant, semantic))
        }
        input_ir::Expression::EqualityExpression(variant) => {
            Expression::EqualityExpression(create_equality_expression(variant, semantic))
        }
        input_ir::Expression::InequalityExpression(variant) => {
            Expression::InequalityExpression(create_inequality_expression(variant, semantic))
        }
        input_ir::Expression::BitwiseOrExpression(variant) => {
            Expression::BitwiseOrExpression(create_bitwise_or_expression(variant, semantic))
        }
        input_ir::Expression::BitwiseXorExpression(variant) => {
            Expression::BitwiseXorExpression(create_bitwise_xor_expression(variant, semantic))
        }
        input_ir::Expression::BitwiseAndExpression(variant) => {
            Expression::BitwiseAndExpression(create_bitwise_and_expression(variant, semantic))
        }
        input_ir::Expression::ShiftExpression(variant) => {
            Expression::ShiftExpression(create_shift_expression(variant, semantic))
        }
        input_ir::Expression::AdditiveExpression(variant) => {
            Expression::AdditiveExpression(create_additive_expression(variant, semantic))
        }
        input_ir::Expression::MultiplicativeExpression(variant) => {
            Expression::MultiplicativeExpression(create_multiplicative_expression(
                variant, semantic,
            ))
        }
        input_ir::Expression::ExponentiationExpression(variant) => {
            Expression::ExponentiationExpression(create_exponentiation_expression(
                variant, semantic,
            ))
        }
        input_ir::Expression::PostfixExpression(variant) => {
            Expression::PostfixExpression(create_postfix_expression(variant, semantic))
        }
        input_ir::Expression::PrefixExpression(variant) => {
            Expression::PrefixExpression(create_prefix_expression(variant, semantic))
        }
        input_ir::Expression::FunctionCallExpression(variant) => {
            Expression::FunctionCallExpression(create_function_call_expression(variant, semantic))
        }
        input_ir::Expression::CallOptionsExpression(variant) => {
            Expression::CallOptionsExpression(create_call_options_expression(variant, semantic))
        }
        input_ir::Expression::MemberAccessExpression(variant) => {
            Expression::MemberAccessExpression(create_member_access_expression(variant, semantic))
        }
        input_ir::Expression::IndexAccessExpression(variant) => {
            Expression::IndexAccessExpression(create_index_access_expression(variant, semantic))
        }
        input_ir::Expression::NewExpression(variant) => {
            Expression::NewExpression(create_new_expression(variant, semantic))
        }
        input_ir::Expression::TupleExpression(variant) => {
            Expression::TupleExpression(create_tuple_expression(variant, semantic))
        }
        input_ir::Expression::TypeExpression(variant) => {
            Expression::TypeExpression(create_type_expression(variant, semantic))
        }
        input_ir::Expression::ArrayExpression(variant) => {
            Expression::ArrayExpression(create_array_expression(variant, semantic))
        }
        input_ir::Expression::HexNumberExpression(variant) => {
            Expression::HexNumberExpression(create_hex_number_expression(variant, semantic))
        }
        input_ir::Expression::DecimalNumberExpression(variant) => {
            Expression::DecimalNumberExpression(create_decimal_number_expression(variant, semantic))
        }
        input_ir::Expression::StringExpression(variant) => {
            Expression::StringExpression(create_string_expression(variant, semantic))
        }
        input_ir::Expression::ElementaryType(variant) => {
            Expression::ElementaryType(create_elementary_type(variant, semantic))
        }
        input_ir::Expression::PayableKeyword => Expression::PayableKeyword,
        input_ir::Expression::ThisKeyword => Expression::ThisKeyword,
        input_ir::Expression::SuperKeyword => Expression::SuperKeyword,
        input_ir::Expression::TrueKeyword => Expression::TrueKeyword,
        input_ir::Expression::FalseKeyword => Expression::FalseKeyword,
        input_ir::Expression::Identifier(variant) => {
            Expression::Identifier(create_identifier(variant, semantic))
        }
    }
}

pub enum ArgumentsDeclaration {
    PositionalArguments(PositionalArguments),
    NamedArguments(NamedArguments),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_arguments_declaration(
    ir_node: &input_ir::ArgumentsDeclaration,
    semantic: &Rc<SemanticAnalysis>,
) -> ArgumentsDeclaration {
    match ir_node {
        input_ir::ArgumentsDeclaration::PositionalArguments(nodes) => {
            ArgumentsDeclaration::PositionalArguments(create_positional_arguments(nodes, semantic))
        }

        input_ir::ArgumentsDeclaration::NamedArguments(nodes) => {
            ArgumentsDeclaration::NamedArguments(create_named_arguments(nodes, semantic))
        }
    }
}

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

#[allow(clippy::too_many_lines)]
pub(crate) fn create_number_unit(
    ir_node: &input_ir::NumberUnit,
    semantic: &Rc<SemanticAnalysis>,
) -> NumberUnit {
    match ir_node {
        input_ir::NumberUnit::WeiKeyword => NumberUnit::WeiKeyword,
        input_ir::NumberUnit::GweiKeyword => NumberUnit::GweiKeyword,
        input_ir::NumberUnit::SzaboKeyword => NumberUnit::SzaboKeyword,
        input_ir::NumberUnit::FinneyKeyword => NumberUnit::FinneyKeyword,
        input_ir::NumberUnit::EtherKeyword => NumberUnit::EtherKeyword,
        input_ir::NumberUnit::SecondsKeyword => NumberUnit::SecondsKeyword,
        input_ir::NumberUnit::MinutesKeyword => NumberUnit::MinutesKeyword,
        input_ir::NumberUnit::HoursKeyword => NumberUnit::HoursKeyword,
        input_ir::NumberUnit::DaysKeyword => NumberUnit::DaysKeyword,
        input_ir::NumberUnit::WeeksKeyword => NumberUnit::WeeksKeyword,
        input_ir::NumberUnit::YearsKeyword => NumberUnit::YearsKeyword,
    }
}

pub enum StringExpression {
    Strings(Vec<Rc<TerminalNode>>),
    HexStrings(Vec<Rc<TerminalNode>>),
    UnicodeStrings(Vec<Rc<TerminalNode>>),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_string_expression(
    ir_node: &input_ir::StringExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> StringExpression {
    match ir_node {
        input_ir::StringExpression::Strings(nodes) => StringExpression::Strings(nodes.clone()),

        input_ir::StringExpression::HexStrings(nodes) => {
            StringExpression::HexStrings(nodes.clone())
        }

        input_ir::StringExpression::UnicodeStrings(nodes) => {
            StringExpression::UnicodeStrings(nodes.clone())
        }
    }
}

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

#[allow(clippy::too_many_lines)]
pub(crate) fn create_yul_statement(
    ir_node: &input_ir::YulStatement,
    semantic: &Rc<SemanticAnalysis>,
) -> YulStatement {
    match ir_node {
        input_ir::YulStatement::YulBlock(variant) => {
            YulStatement::YulBlock(create_yul_block(variant, semantic))
        }
        input_ir::YulStatement::YulFunctionDefinition(variant) => {
            YulStatement::YulFunctionDefinition(create_yul_function_definition(variant, semantic))
        }
        input_ir::YulStatement::YulStackAssignmentStatement(variant) => {
            YulStatement::YulStackAssignmentStatement(create_yul_stack_assignment_statement(
                variant, semantic,
            ))
        }
        input_ir::YulStatement::YulIfStatement(variant) => {
            YulStatement::YulIfStatement(create_yul_if_statement(variant, semantic))
        }
        input_ir::YulStatement::YulForStatement(variant) => {
            YulStatement::YulForStatement(create_yul_for_statement(variant, semantic))
        }
        input_ir::YulStatement::YulSwitchStatement(variant) => {
            YulStatement::YulSwitchStatement(create_yul_switch_statement(variant, semantic))
        }
        input_ir::YulStatement::YulLeaveStatement(variant) => {
            YulStatement::YulLeaveStatement(create_yul_leave_statement(variant, semantic))
        }
        input_ir::YulStatement::YulBreakStatement(variant) => {
            YulStatement::YulBreakStatement(create_yul_break_statement(variant, semantic))
        }
        input_ir::YulStatement::YulContinueStatement(variant) => {
            YulStatement::YulContinueStatement(create_yul_continue_statement(variant, semantic))
        }
        input_ir::YulStatement::YulVariableAssignmentStatement(variant) => {
            YulStatement::YulVariableAssignmentStatement(create_yul_variable_assignment_statement(
                variant, semantic,
            ))
        }
        input_ir::YulStatement::YulLabel(variant) => {
            YulStatement::YulLabel(create_yul_label(variant, semantic))
        }
        input_ir::YulStatement::YulVariableDeclarationStatement(variant) => {
            YulStatement::YulVariableDeclarationStatement(
                create_yul_variable_declaration_statement(variant, semantic),
            )
        }
        input_ir::YulStatement::YulExpression(variant) => {
            YulStatement::YulExpression(create_yul_expression(variant, semantic))
        }
    }
}

pub enum YulAssignmentOperator {
    ColonEqual,
    YulColonAndEqual(YulColonAndEqual),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_yul_assignment_operator(
    ir_node: &input_ir::YulAssignmentOperator,
    semantic: &Rc<SemanticAnalysis>,
) -> YulAssignmentOperator {
    match ir_node {
        input_ir::YulAssignmentOperator::ColonEqual => YulAssignmentOperator::ColonEqual,
        input_ir::YulAssignmentOperator::YulColonAndEqual(variant) => {
            YulAssignmentOperator::YulColonAndEqual(create_yul_colon_and_equal(variant, semantic))
        }
    }
}

pub enum YulStackAssignmentOperator {
    EqualColon,
    YulEqualAndColon(YulEqualAndColon),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_yul_stack_assignment_operator(
    ir_node: &input_ir::YulStackAssignmentOperator,
    semantic: &Rc<SemanticAnalysis>,
) -> YulStackAssignmentOperator {
    match ir_node {
        input_ir::YulStackAssignmentOperator::EqualColon => YulStackAssignmentOperator::EqualColon,
        input_ir::YulStackAssignmentOperator::YulEqualAndColon(variant) => {
            YulStackAssignmentOperator::YulEqualAndColon(create_yul_equal_and_colon(
                variant, semantic,
            ))
        }
    }
}

pub enum YulSwitchCase {
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_yul_switch_case(
    ir_node: &input_ir::YulSwitchCase,
    semantic: &Rc<SemanticAnalysis>,
) -> YulSwitchCase {
    match ir_node {
        input_ir::YulSwitchCase::YulDefaultCase(variant) => {
            YulSwitchCase::YulDefaultCase(create_yul_default_case(variant, semantic))
        }
        input_ir::YulSwitchCase::YulValueCase(variant) => {
            YulSwitchCase::YulValueCase(create_yul_value_case(variant, semantic))
        }
    }
}

pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_yul_expression(
    ir_node: &input_ir::YulExpression,
    semantic: &Rc<SemanticAnalysis>,
) -> YulExpression {
    match ir_node {
        input_ir::YulExpression::YulFunctionCallExpression(variant) => {
            YulExpression::YulFunctionCallExpression(create_yul_function_call_expression(
                variant, semantic,
            ))
        }
        input_ir::YulExpression::YulLiteral(variant) => {
            YulExpression::YulLiteral(create_yul_literal(variant, semantic))
        }
        input_ir::YulExpression::YulPath(nodes) => {
            YulExpression::YulPath(create_yul_path(nodes, semantic))
        }
    }
}

pub enum YulLiteral {
    YulTrueKeyword,
    YulFalseKeyword,
    YulDecimalLiteral(Rc<TerminalNode>),
    YulHexLiteral(Rc<TerminalNode>),
    StringLiteral(Rc<TerminalNode>),
    HexStringLiteral(Rc<TerminalNode>),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_yul_literal(
    ir_node: &input_ir::YulLiteral,
    semantic: &Rc<SemanticAnalysis>,
) -> YulLiteral {
    match ir_node {
        input_ir::YulLiteral::YulTrueKeyword => YulLiteral::YulTrueKeyword,
        input_ir::YulLiteral::YulFalseKeyword => YulLiteral::YulFalseKeyword,
        input_ir::YulLiteral::YulDecimalLiteral(node) => {
            YulLiteral::YulDecimalLiteral(Rc::clone(node))
        }
        input_ir::YulLiteral::YulHexLiteral(node) => YulLiteral::YulHexLiteral(Rc::clone(node)),
        input_ir::YulLiteral::StringLiteral(node) => YulLiteral::StringLiteral(Rc::clone(node)),
        input_ir::YulLiteral::HexStringLiteral(node) => {
            YulLiteral::HexStringLiteral(Rc::clone(node))
        }
    }
}

pub enum FunctionKind {
    Regular,
    Constructor,
    Unnamed,
    Fallback,
    Receive,
    Modifier,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_function_kind(
    ir_node: &input_ir::FunctionKind,
    semantic: &Rc<SemanticAnalysis>,
) -> FunctionKind {
    match ir_node {
        input_ir::FunctionKind::Regular => FunctionKind::Regular,
        input_ir::FunctionKind::Constructor => FunctionKind::Constructor,
        input_ir::FunctionKind::Unnamed => FunctionKind::Unnamed,
        input_ir::FunctionKind::Fallback => FunctionKind::Fallback,
        input_ir::FunctionKind::Receive => FunctionKind::Receive,
        input_ir::FunctionKind::Modifier => FunctionKind::Modifier,
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
    ir_node: &input_ir::FunctionVisibility,
    semantic: &Rc<SemanticAnalysis>,
) -> FunctionVisibility {
    match ir_node {
        input_ir::FunctionVisibility::Public => FunctionVisibility::Public,
        input_ir::FunctionVisibility::Private => FunctionVisibility::Private,
        input_ir::FunctionVisibility::Internal => FunctionVisibility::Internal,
        input_ir::FunctionVisibility::External => FunctionVisibility::External,
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
    ir_node: &input_ir::FunctionMutability,
    semantic: &Rc<SemanticAnalysis>,
) -> FunctionMutability {
    match ir_node {
        input_ir::FunctionMutability::Pure => FunctionMutability::Pure,
        input_ir::FunctionMutability::View => FunctionMutability::View,
        input_ir::FunctionMutability::NonPayable => FunctionMutability::NonPayable,
        input_ir::FunctionMutability::Payable => FunctionMutability::Payable,
    }
}

pub enum StateVariableVisibility {
    Public,
    Private,
    Internal,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_state_variable_visibility(
    ir_node: &input_ir::StateVariableVisibility,
    semantic: &Rc<SemanticAnalysis>,
) -> StateVariableVisibility {
    match ir_node {
        input_ir::StateVariableVisibility::Public => StateVariableVisibility::Public,
        input_ir::StateVariableVisibility::Private => StateVariableVisibility::Private,
        input_ir::StateVariableVisibility::Internal => StateVariableVisibility::Internal,
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
    ir_node: &input_ir::StateVariableMutability,
    semantic: &Rc<SemanticAnalysis>,
) -> StateVariableMutability {
    match ir_node {
        input_ir::StateVariableMutability::Mutable => StateVariableMutability::Mutable,
        input_ir::StateVariableMutability::Constant => StateVariableMutability::Constant,
        input_ir::StateVariableMutability::Immutable => StateVariableMutability::Immutable,
        input_ir::StateVariableMutability::Transient => StateVariableMutability::Transient,
    }
}

pub enum TupleDeconstructionMember {
    None,
    Identifier(Identifier),
    VariableDeclarationStatement(VariableDeclarationStatement),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_tuple_deconstruction_member(
    ir_node: &input_ir::TupleDeconstructionMember,
    semantic: &Rc<SemanticAnalysis>,
) -> TupleDeconstructionMember {
    match ir_node {
        input_ir::TupleDeconstructionMember::None => TupleDeconstructionMember::None,
        input_ir::TupleDeconstructionMember::Identifier(variant) => {
            TupleDeconstructionMember::Identifier(create_identifier(variant, semantic))
        }
        input_ir::TupleDeconstructionMember::VariableDeclarationStatement(variant) => {
            TupleDeconstructionMember::VariableDeclarationStatement(
                create_variable_declaration_statement(variant, semantic),
            )
        }
    }
}

//
// Repeated & Separated
//

pub type SourceUnitMembers = Rc<SourceUnitMembersStruct>;

pub(crate) fn create_source_unit_members(
    nodes: &[input_ir::SourceUnitMember],
    semantic: &Rc<SemanticAnalysis>,
) -> SourceUnitMembers {
    Rc::new(SourceUnitMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct SourceUnitMembersStruct {
    pub(crate) ir_nodes: Vec<input_ir::SourceUnitMember>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl SourceUnitMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = SourceUnitMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_source_unit_member(ir_node, &self.semantic))
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
    nodes: &[input_ir::VersionExpressionSet],
    semantic: &Rc<SemanticAnalysis>,
) -> VersionExpressionSets {
    Rc::new(VersionExpressionSetsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct VersionExpressionSetsStruct {
    pub(crate) ir_nodes: Vec<input_ir::VersionExpressionSet>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl VersionExpressionSetsStruct {
    pub fn iter(&self) -> impl Iterator<Item = VersionExpressionSet> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_version_expression_set(ir_node, &self.semantic))
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
    nodes: &[input_ir::VersionExpression],
    semantic: &Rc<SemanticAnalysis>,
) -> VersionExpressionSet {
    Rc::new(VersionExpressionSetStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct VersionExpressionSetStruct {
    pub(crate) ir_nodes: Vec<input_ir::VersionExpression>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl VersionExpressionSetStruct {
    pub fn iter(&self) -> impl Iterator<Item = VersionExpression> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_version_expression(ir_node, &self.semantic))
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
    nodes: &[input_ir::ImportDeconstructionSymbol],
    semantic: &Rc<SemanticAnalysis>,
) -> ImportDeconstructionSymbols {
    Rc::new(ImportDeconstructionSymbolsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct ImportDeconstructionSymbolsStruct {
    pub(crate) ir_nodes: Vec<input_ir::ImportDeconstructionSymbol>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl ImportDeconstructionSymbolsStruct {
    pub fn iter(&self) -> impl Iterator<Item = ImportDeconstructionSymbol> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_import_deconstruction_symbol(ir_node, &self.semantic))
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
    nodes: &[input_ir::UsingDeconstructionSymbol],
    semantic: &Rc<SemanticAnalysis>,
) -> UsingDeconstructionSymbols {
    Rc::new(UsingDeconstructionSymbolsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct UsingDeconstructionSymbolsStruct {
    pub(crate) ir_nodes: Vec<input_ir::UsingDeconstructionSymbol>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl UsingDeconstructionSymbolsStruct {
    pub fn iter(&self) -> impl Iterator<Item = UsingDeconstructionSymbol> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_using_deconstruction_symbol(ir_node, &self.semantic))
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
    nodes: &[input_ir::InheritanceType],
    semantic: &Rc<SemanticAnalysis>,
) -> InheritanceTypes {
    Rc::new(InheritanceTypesStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct InheritanceTypesStruct {
    pub(crate) ir_nodes: Vec<input_ir::InheritanceType>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl InheritanceTypesStruct {
    pub fn iter(&self) -> impl Iterator<Item = InheritanceType> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_inheritance_type(ir_node, &self.semantic))
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
    nodes: &[input_ir::ContractMember],
    semantic: &Rc<SemanticAnalysis>,
) -> ContractMembers {
    Rc::new(ContractMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct ContractMembersStruct {
    pub(crate) ir_nodes: Vec<input_ir::ContractMember>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl ContractMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = ContractMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_contract_member(ir_node, &self.semantic))
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
    nodes: &[input_ir::ContractMember],
    semantic: &Rc<SemanticAnalysis>,
) -> InterfaceMembers {
    Rc::new(InterfaceMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct InterfaceMembersStruct {
    pub(crate) ir_nodes: Vec<input_ir::ContractMember>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl InterfaceMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = ContractMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_contract_member(ir_node, &self.semantic))
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
    nodes: &[input_ir::ContractMember],
    semantic: &Rc<SemanticAnalysis>,
) -> LibraryMembers {
    Rc::new(LibraryMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct LibraryMembersStruct {
    pub(crate) ir_nodes: Vec<input_ir::ContractMember>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl LibraryMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = ContractMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_contract_member(ir_node, &self.semantic))
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
    nodes: &[input_ir::StructMember],
    semantic: &Rc<SemanticAnalysis>,
) -> StructMembers {
    Rc::new(StructMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct StructMembersStruct {
    pub(crate) ir_nodes: Vec<input_ir::StructMember>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl StructMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = StructMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_struct_member(ir_node, &self.semantic))
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
    nodes: &[Rc<TerminalNode>],
    semantic: &Rc<SemanticAnalysis>,
) -> EnumMembers {
    Rc::new(EnumMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct EnumMembersStruct {
    pub(crate) ir_nodes: Vec<Rc<TerminalNode>>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl EnumMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
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
    nodes: &[input_ir::Parameter],
    semantic: &Rc<SemanticAnalysis>,
) -> Parameters {
    Rc::new(ParametersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct ParametersStruct {
    pub(crate) ir_nodes: Vec<input_ir::Parameter>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl ParametersStruct {
    pub fn iter(&self) -> impl Iterator<Item = Parameter> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_parameter(ir_node, &self.semantic))
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
    nodes: &[input_ir::IdentifierPath],
    semantic: &Rc<SemanticAnalysis>,
) -> OverridePaths {
    Rc::new(OverridePathsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct OverridePathsStruct {
    pub(crate) ir_nodes: Vec<input_ir::IdentifierPath>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl OverridePathsStruct {
    pub fn iter(&self) -> impl Iterator<Item = IdentifierPath> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier_path(ir_node, &self.semantic))
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
    nodes: &[input_ir::Statement],
    semantic: &Rc<SemanticAnalysis>,
) -> Statements {
    Rc::new(StatementsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct StatementsStruct {
    pub(crate) ir_nodes: Vec<input_ir::Statement>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl StatementsStruct {
    pub fn iter(&self) -> impl Iterator<Item = Statement> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_statement(ir_node, &self.semantic))
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
    nodes: &[input_ir::CatchClause],
    semantic: &Rc<SemanticAnalysis>,
) -> CatchClauses {
    Rc::new(CatchClausesStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct CatchClausesStruct {
    pub(crate) ir_nodes: Vec<input_ir::CatchClause>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl CatchClausesStruct {
    pub fn iter(&self) -> impl Iterator<Item = CatchClause> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_catch_clause(ir_node, &self.semantic))
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
    nodes: &[input_ir::Expression],
    semantic: &Rc<SemanticAnalysis>,
) -> PositionalArguments {
    Rc::new(PositionalArgumentsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct PositionalArgumentsStruct {
    pub(crate) ir_nodes: Vec<input_ir::Expression>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl PositionalArgumentsStruct {
    pub fn iter(&self) -> impl Iterator<Item = Expression> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
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
    nodes: &[input_ir::NamedArgument],
    semantic: &Rc<SemanticAnalysis>,
) -> NamedArguments {
    Rc::new(NamedArgumentsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct NamedArgumentsStruct {
    pub(crate) ir_nodes: Vec<input_ir::NamedArgument>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl NamedArgumentsStruct {
    pub fn iter(&self) -> impl Iterator<Item = NamedArgument> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_named_argument(ir_node, &self.semantic))
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
    nodes: &[input_ir::NamedArgument],
    semantic: &Rc<SemanticAnalysis>,
) -> CallOptions {
    Rc::new(CallOptionsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct CallOptionsStruct {
    pub(crate) ir_nodes: Vec<input_ir::NamedArgument>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl CallOptionsStruct {
    pub fn iter(&self) -> impl Iterator<Item = NamedArgument> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_named_argument(ir_node, &self.semantic))
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
    nodes: &[input_ir::TupleValue],
    semantic: &Rc<SemanticAnalysis>,
) -> TupleValues {
    Rc::new(TupleValuesStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct TupleValuesStruct {
    pub(crate) ir_nodes: Vec<input_ir::TupleValue>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl TupleValuesStruct {
    pub fn iter(&self) -> impl Iterator<Item = TupleValue> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_tuple_value(ir_node, &self.semantic))
    }
    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}

pub type ArrayValues = Rc<ArrayValuesStruct>;

pub(crate) fn create_array_values(
    nodes: &[input_ir::Expression],
    semantic: &Rc<SemanticAnalysis>,
) -> ArrayValues {
    Rc::new(ArrayValuesStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct ArrayValuesStruct {
    pub(crate) ir_nodes: Vec<input_ir::Expression>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl ArrayValuesStruct {
    pub fn iter(&self) -> impl Iterator<Item = Expression> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_expression(ir_node, &self.semantic))
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
    nodes: &[Rc<TerminalNode>],
    semantic: &Rc<SemanticAnalysis>,
) -> IdentifierPath {
    Rc::new(IdentifierPathStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct IdentifierPathStruct {
    pub(crate) ir_nodes: Vec<Rc<TerminalNode>>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl IdentifierPathStruct {
    pub fn iter(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_identifier(ir_node, &self.semantic))
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
    nodes: &[input_ir::YulStatement],
    semantic: &Rc<SemanticAnalysis>,
) -> YulStatements {
    Rc::new(YulStatementsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulStatementsStruct {
    pub(crate) ir_nodes: Vec<input_ir::YulStatement>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl YulStatementsStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulStatement> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_statement(ir_node, &self.semantic))
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
    nodes: &[Rc<TerminalNode>],
    semantic: &Rc<SemanticAnalysis>,
) -> YulParameters {
    Rc::new(YulParametersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulParametersStruct {
    pub(crate) ir_nodes: Vec<Rc<TerminalNode>>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl YulParametersStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulIdentifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_identifier(ir_node, &self.semantic))
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
    nodes: &[Rc<TerminalNode>],
    semantic: &Rc<SemanticAnalysis>,
) -> YulVariableNames {
    Rc::new(YulVariableNamesStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulVariableNamesStruct {
    pub(crate) ir_nodes: Vec<Rc<TerminalNode>>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl YulVariableNamesStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulIdentifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_identifier(ir_node, &self.semantic))
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
    nodes: &[input_ir::YulSwitchCase],
    semantic: &Rc<SemanticAnalysis>,
) -> YulSwitchCases {
    Rc::new(YulSwitchCasesStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulSwitchCasesStruct {
    pub(crate) ir_nodes: Vec<input_ir::YulSwitchCase>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl YulSwitchCasesStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulSwitchCase> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_switch_case(ir_node, &self.semantic))
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
    nodes: &[input_ir::YulExpression],
    semantic: &Rc<SemanticAnalysis>,
) -> YulArguments {
    Rc::new(YulArgumentsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulArgumentsStruct {
    pub(crate) ir_nodes: Vec<input_ir::YulExpression>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl YulArgumentsStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulExpression> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_expression(ir_node, &self.semantic))
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
    nodes: &[input_ir::YulPath],
    semantic: &Rc<SemanticAnalysis>,
) -> YulPaths {
    Rc::new(YulPathsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulPathsStruct {
    pub(crate) ir_nodes: Vec<input_ir::YulPath>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl YulPathsStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulPath> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_path(ir_node, &self.semantic))
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
    nodes: &[Rc<TerminalNode>],
    semantic: &Rc<SemanticAnalysis>,
) -> YulPath {
    Rc::new(YulPathStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct YulPathStruct {
    pub(crate) ir_nodes: Vec<Rc<TerminalNode>>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl YulPathStruct {
    pub fn iter(&self) -> impl Iterator<Item = YulIdentifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_yul_identifier(ir_node, &self.semantic))
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
    nodes: &[input_ir::ModifierInvocation],
    semantic: &Rc<SemanticAnalysis>,
) -> ModifierInvocations {
    Rc::new(ModifierInvocationsStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct ModifierInvocationsStruct {
    pub(crate) ir_nodes: Vec<input_ir::ModifierInvocation>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl ModifierInvocationsStruct {
    pub fn iter(&self) -> impl Iterator<Item = ModifierInvocation> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_modifier_invocation(ir_node, &self.semantic))
    }
    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}

pub type TupleDeconstructionMembers = Rc<TupleDeconstructionMembersStruct>;

pub(crate) fn create_tuple_deconstruction_members(
    nodes: &[input_ir::TupleDeconstructionMember],
    semantic: &Rc<SemanticAnalysis>,
) -> TupleDeconstructionMembers {
    Rc::new(TupleDeconstructionMembersStruct {
        ir_nodes: nodes.to_vec(),
        semantic: Rc::clone(semantic),
    })
}

pub struct TupleDeconstructionMembersStruct {
    pub(crate) ir_nodes: Vec<input_ir::TupleDeconstructionMember>,
    pub(crate) semantic: Rc<SemanticAnalysis>,
}

impl TupleDeconstructionMembersStruct {
    pub fn iter(&self) -> impl Iterator<Item = TupleDeconstructionMember> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| create_tuple_deconstruction_member(ir_node, &self.semantic))
    }
    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
