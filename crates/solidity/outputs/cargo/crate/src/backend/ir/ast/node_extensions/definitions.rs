use std::rc::Rc;

use paste::paste;

use super::super::nodes::{
    ConstantDefinition, ConstantDefinitionStruct, ContractDefinition, ContractDefinitionStruct,
    EnumDefinition, EnumDefinitionStruct, ErrorDefinition, ErrorDefinitionStruct, EventDefinition,
    EventDefinitionStruct, FunctionDefinition, FunctionDefinitionStruct,
    ImportDeconstructionSymbol, ImportDeconstructionSymbolStruct, InterfaceDefinition,
    InterfaceDefinitionStruct, LibraryDefinition, LibraryDefinitionStruct, Parameter,
    ParameterStruct, PathImport, PathImportStruct, StateVariableDefinition,
    StateVariableDefinitionStruct, StructDefinition, StructDefinitionStruct, StructMember,
    StructMemberStruct, UserDefinedValueTypeDefinition, UserDefinedValueTypeDefinitionStruct,
    VariableDeclarationStatement, VariableDeclarationStatementStruct, YulFunctionDefinition,
    YulFunctionDefinitionStruct, YulLabel, YulLabelStruct,
};
use super::{create_identifier, create_yul_identifier, Identifier, Reference, YulIdentifier};
use crate::backend::ir::ast::{
    create_constant_definition, create_contract_definition, create_enum_definition,
    create_error_definition, create_event_definition, create_function_definition,
    create_import_deconstruction_symbol, create_interface_definition, create_library_definition,
    create_parameter, create_path_import, create_state_variable_definition,
    create_struct_definition, create_struct_member, create_user_defined_value_type_definition,
    create_variable_declaration_statement, create_yul_function_definition, create_yul_label,
};
use crate::backend::{binder, SemanticAnalysis};
use crate::cst::NodeId;

// __SLANG_DEFINITION_TYPES__ keep in sync with binder
pub enum Definition {
    Constant(ConstantDefinition),
    Contract(ContractDefinition),
    Enum(EnumDefinition),
    EnumMember(Identifier),
    Error(ErrorDefinition),
    Event(EventDefinition),
    Function(FunctionDefinition),
    Import(PathImport),
    ImportedSymbol(ImportDeconstructionSymbol),
    Interface(InterfaceDefinition),
    Library(LibraryDefinition),
    Modifier(FunctionDefinition),
    Parameter(Parameter),
    StateVariable(StateVariableDefinition),
    Struct(StructDefinition),
    StructMember(StructMember),
    TypeParameter(Parameter),
    UserDefinedValueType(UserDefinedValueTypeDefinition),
    Variable(VariableDeclarationStatement),
    YulFunction(YulFunctionDefinition),
    YulLabel(YulLabel),
    YulParameter(YulIdentifier),
    YulVariable(YulIdentifier),
}

impl Definition {
    pub(crate) fn create(definition_id: NodeId, semantic: &Rc<SemanticAnalysis>) -> Self {
        let definition = semantic
            .binder()
            .find_definition_by_id(definition_id)
            .expect("definition_id references a definition node");

        match definition {
            binder::Definition::Constant(constant_definition) => Self::Constant(
                create_constant_definition(&constant_definition.ir_node, semantic),
            ),
            binder::Definition::Contract(contract_definition) => Self::Contract(
                create_contract_definition(&contract_definition.ir_node, semantic),
            ),
            binder::Definition::Enum(enum_definition) => {
                Self::Enum(create_enum_definition(&enum_definition.ir_node, semantic))
            }
            binder::Definition::EnumMember(enum_member_definition) => {
                Self::EnumMember(create_identifier(&enum_member_definition.ir_node, semantic))
            }
            binder::Definition::Error(error_definition) => {
                Self::Error(create_error_definition(&error_definition.ir_node, semantic))
            }
            binder::Definition::Event(event_definition) => {
                Self::Event(create_event_definition(&event_definition.ir_node, semantic))
            }
            binder::Definition::Function(function_definition) => Self::Function(
                create_function_definition(&function_definition.ir_node, semantic),
            ),
            binder::Definition::Import(import_definition) => {
                Self::Import(create_path_import(&import_definition.ir_node, semantic))
            }
            binder::Definition::ImportedSymbol(imported_symbol_definition) => Self::ImportedSymbol(
                create_import_deconstruction_symbol(&imported_symbol_definition.ir_node, semantic),
            ),
            binder::Definition::Interface(interface_definition) => Self::Interface(
                create_interface_definition(&interface_definition.ir_node, semantic),
            ),
            binder::Definition::Library(library_definition) => Self::Library(
                create_library_definition(&library_definition.ir_node, semantic),
            ),
            binder::Definition::Modifier(modifier_definition) => Self::Modifier(
                create_function_definition(&modifier_definition.ir_node, semantic),
            ),
            binder::Definition::Parameter(parameter_definition) => {
                Self::Parameter(create_parameter(&parameter_definition.ir_node, semantic))
            }
            binder::Definition::StateVariable(state_variable_definition) => Self::StateVariable(
                create_state_variable_definition(&state_variable_definition.ir_node, semantic),
            ),
            binder::Definition::Struct(struct_definition) => Self::Struct(
                create_struct_definition(&struct_definition.ir_node, semantic),
            ),
            binder::Definition::StructMember(struct_member_definition) => Self::StructMember(
                create_struct_member(&struct_member_definition.ir_node, semantic),
            ),
            binder::Definition::TypeParameter(type_parameter_definition) => Self::TypeParameter(
                create_parameter(&type_parameter_definition.ir_node, semantic),
            ),
            binder::Definition::UserDefinedValueType(user_defined_value_type_definition) => {
                Self::UserDefinedValueType(create_user_defined_value_type_definition(
                    &user_defined_value_type_definition.ir_node,
                    semantic,
                ))
            }
            binder::Definition::Variable(variable_definition) => Self::Variable(
                create_variable_declaration_statement(&variable_definition.ir_node, semantic),
            ),
            binder::Definition::YulFunction(yul_function_definition) => Self::YulFunction(
                create_yul_function_definition(&yul_function_definition.ir_node, semantic),
            ),
            binder::Definition::YulLabel(yul_label_definition) => {
                Self::YulLabel(create_yul_label(&yul_label_definition.ir_node, semantic))
            }
            binder::Definition::YulParameter(yul_parameter_definition) => Self::YulParameter(
                create_yul_identifier(&yul_parameter_definition.ir_node, semantic),
            ),
            binder::Definition::YulVariable(yul_variable_definition) => Self::YulVariable(
                create_yul_identifier(&yul_variable_definition.ir_node, semantic),
            ),
        }
    }

    pub fn references(&self) -> Vec<Reference> {
        match self {
            Definition::Constant(constant_definition) => constant_definition.references(),
            Definition::Contract(contract_definition) => contract_definition.references(),
            Definition::Enum(enum_definition) => enum_definition.references(),
            Definition::EnumMember(identifier) => identifier.references(),
            Definition::Error(error_definition) => error_definition.references(),
            Definition::Event(event_definition) => event_definition.references(),
            Definition::Function(function_definition) => function_definition.references(),
            Definition::Import(path_import) => path_import.references(),
            Definition::ImportedSymbol(import_deconstruction_symbol) => {
                import_deconstruction_symbol.references()
            }
            Definition::Interface(interface_definition) => interface_definition.references(),
            Definition::Library(library_definition) => library_definition.references(),
            Definition::Modifier(function_definition) => function_definition.references(),
            Definition::Parameter(parameter) => parameter.references(),
            Definition::StateVariable(state_variable_definition) => {
                state_variable_definition.references()
            }
            Definition::Struct(struct_definition) => struct_definition.references(),
            Definition::StructMember(struct_member) => struct_member.references(),
            Definition::TypeParameter(parameter) => parameter.references(),
            Definition::UserDefinedValueType(user_defined_value_type_definition) => {
                user_defined_value_type_definition.references()
            }
            Definition::Variable(variable_declaration_statement) => {
                variable_declaration_statement.references()
            }
            Definition::YulFunction(yul_function_definition) => {
                yul_function_definition.references()
            }
            Definition::YulLabel(yul_label) => yul_label.references(),
            Definition::YulParameter(identifier) => identifier.references(),
            Definition::YulVariable(identifier) => identifier.references(),
        }
    }
}

macro_rules! define_references_method {
    ($type:ident) => {
        paste! {
            impl [<$type Struct>] {
                pub fn references(&self) -> Vec<Reference> {
                    self.semantic.references_binding_to(self.ir_node.node_id)
                }
            }
        }
    };
}

define_references_method!(ConstantDefinition);
define_references_method!(ContractDefinition);
define_references_method!(EnumDefinition);
define_references_method!(ErrorDefinition);
define_references_method!(EventDefinition);
define_references_method!(FunctionDefinition);
define_references_method!(ImportDeconstructionSymbol);
define_references_method!(InterfaceDefinition);
define_references_method!(LibraryDefinition);
define_references_method!(Parameter);
define_references_method!(PathImport);
define_references_method!(StateVariableDefinition);
define_references_method!(StructDefinition);
define_references_method!(StructMember);
define_references_method!(UserDefinedValueTypeDefinition);
define_references_method!(VariableDeclarationStatement);
define_references_method!(YulLabel);
define_references_method!(YulFunctionDefinition);
