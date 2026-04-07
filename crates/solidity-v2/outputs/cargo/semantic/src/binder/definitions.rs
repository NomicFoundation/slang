use std::rc::Rc;

use slang_solidity_v2_ir::ir::{self, NodeId};

use super::ScopeId;
use crate::types::TypeId;

//////////////////////////////////////////////////////////////////////////////
// Definitions

// __SLANG_DEFINITION_TYPES__ keep in sync with AST type
#[derive(Debug)]
pub enum Definition {
    Constant(ConstantDefinition),
    Contract(ContractDefinition),
    Enum(EnumDefinition),
    EnumMember(EnumMemberDefinition),
    Error(ErrorDefinition),
    Event(EventDefinition),
    Function(FunctionDefinition),
    Import(ImportDefinition),
    ImportedSymbol(ImportedSymbolDefinition),
    Interface(InterfaceDefinition),
    Library(LibraryDefinition),
    Modifier(ModifierDefinition),
    Parameter(ParameterDefinition),
    StateVariable(StateVariableDefinition),
    Struct(StructDefinition),
    StructMember(StructMemberDefinition),
    TypeParameter(TypeParameterDefinition),
    UserDefinedValueType(UserDefinedValueTypeDefinition),
    Variable(VariableDefinition),
    YulFunction(YulFunctionDefinition),
    YulParameter(YulParameterDefinition),
    YulVariable(YulVariableDefinition),
}

#[derive(Debug)]
pub struct ConstantDefinition {
    pub(crate) ir_node: ir::ConstantDefinition,
    pub scope_id: ScopeId,
}

#[derive(Debug)]
pub struct ContractDefinition {
    pub(crate) ir_node: ir::ContractDefinition,
    pub bases: Option<Vec<NodeId>>,
    pub constructor_parameters_scope_id: Option<ScopeId>,
    pub base_slot: Option<usize>,
}

#[derive(Debug)]
pub struct EnumDefinition {
    pub(crate) ir_node: ir::EnumDefinition,
}

#[derive(Debug)]
pub struct EnumMemberDefinition {
    pub(crate) ir_node: ir::Identifier,
}

#[derive(Debug)]
pub struct ErrorDefinition {
    pub(crate) ir_node: ir::ErrorDefinition,
    pub parameters_scope_id: ScopeId,
}

#[derive(Debug)]
pub struct EventDefinition {
    pub(crate) ir_node: ir::EventDefinition,
    pub parameters_scope_id: ScopeId,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub(crate) ir_node: ir::FunctionDefinition,
    pub parameters_scope_id: ScopeId,
    pub visibility: ir::FunctionVisibility,
}

#[derive(Debug)]
pub struct ImportDefinition {
    pub(crate) ir_node: ir::PathImport,
    pub resolved_file_id: Option<String>,
}

#[derive(Debug)]
pub struct ImportedSymbolDefinition {
    pub(crate) ir_node: ir::ImportDeconstructionSymbol,
    pub symbol: String,
    pub resolved_file_id: Option<String>,
}

#[derive(Debug)]
pub struct InterfaceDefinition {
    pub(crate) ir_node: ir::InterfaceDefinition,
    pub bases: Option<Vec<NodeId>>,
}

#[derive(Debug)]
pub struct LibraryDefinition {
    pub(crate) ir_node: ir::LibraryDefinition,
}

#[derive(Debug)]
pub struct ModifierDefinition {
    pub(crate) ir_node: ir::FunctionDefinition,
}

#[derive(Debug)]
pub struct ParameterDefinition {
    pub(crate) ir_node: ir::Parameter,
}

#[derive(Debug)]
pub struct StateVariableDefinition {
    pub(crate) ir_node: ir::StateVariableDefinition,
    pub getter_type_id: Option<TypeId>,
    pub visibility: ir::StateVariableVisibility,
}

#[derive(Debug)]
pub struct StructDefinition {
    pub(crate) ir_node: ir::StructDefinition,
}

#[derive(Debug)]
pub struct StructMemberDefinition {
    pub(crate) ir_node: ir::StructMember,
}

#[derive(Debug)]
pub struct TypeParameterDefinition {
    pub(crate) ir_node: ir::Parameter,
}

#[derive(Debug)]
pub struct UserDefinedValueTypeDefinition {
    pub(crate) ir_node: ir::UserDefinedValueTypeDefinition,
    pub target_type_id: Option<TypeId>,
}

#[derive(Debug)]
pub struct VariableDefinition {
    pub(crate) ir_node: ir::VariableDeclaration,
}

#[derive(Debug)]
pub struct YulFunctionDefinition {
    pub(crate) ir_node: ir::YulFunctionDefinition,
}

#[derive(Debug)]
pub struct YulParameterDefinition {
    pub(crate) ir_node: ir::Identifier,
}

#[derive(Debug)]
pub struct YulVariableDefinition {
    pub(crate) ir_node: ir::Identifier,
}

impl Definition {
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Constant(constant_definition) => constant_definition.ir_node.id(),
            Self::Contract(contract_definition) => contract_definition.ir_node.id(),
            Self::Enum(enum_definition) => enum_definition.ir_node.id(),
            Self::EnumMember(enum_member_definition) => enum_member_definition.ir_node.id(),
            Self::Error(error_definition) => error_definition.ir_node.id(),
            Self::Event(event_definition) => event_definition.ir_node.id(),
            Self::Function(function_definition) => function_definition.ir_node.id(),
            Self::Import(import_definition) => import_definition.ir_node.id(),
            Self::ImportedSymbol(imported_symbol_definition) => {
                imported_symbol_definition.ir_node.id()
            }
            Self::Interface(interface_definition) => interface_definition.ir_node.id(),
            Self::Library(library_definition) => library_definition.ir_node.id(),
            Self::Modifier(modifier_definition) => modifier_definition.ir_node.id(),
            Self::Parameter(parameter_definition) => parameter_definition.ir_node.id(),
            Self::StateVariable(state_variable_definition) => {
                state_variable_definition.ir_node.id()
            }
            Self::Struct(struct_definition) => struct_definition.ir_node.id(),
            Self::StructMember(struct_member_definition) => struct_member_definition.ir_node.id(),
            Self::TypeParameter(parameter_definition) => parameter_definition.ir_node.id(),
            Self::UserDefinedValueType(udvt_definition) => udvt_definition.ir_node.id(),
            Self::Variable(variable_definition) => variable_definition.ir_node.id(),
            Self::YulFunction(function_definition) => function_definition.ir_node.id(),
            Self::YulParameter(parameter_definition) => parameter_definition.ir_node.id(),
            Self::YulVariable(variable_definition) => variable_definition.ir_node.id(),
        }
    }

    pub fn identifier(&self) -> &ir::Identifier {
        match self {
            Self::Constant(constant_definition) => &constant_definition.ir_node.name,
            Self::Contract(contract_definition) => &contract_definition.ir_node.name,
            Self::Enum(enum_definition) => &enum_definition.ir_node.name,
            Self::EnumMember(enum_member_definition) => &enum_member_definition.ir_node,
            Self::Error(error_definition) => &error_definition.ir_node.name,
            Self::Event(event_definition) => &event_definition.ir_node.name,
            Self::Function(function_definition) => {
                // Function definitions are only created for *named* functions
                function_definition.ir_node.name.as_ref().unwrap()
            }
            Self::Import(import_definition) => {
                // Definitions are created only for aliased imports
                import_definition.ir_node.alias.as_ref().unwrap()
            }
            Self::ImportedSymbol(symbol_definition) => {
                // This is the "local" identifier for the imported symbol, ie. the alias
                symbol_definition
                    .ir_node
                    .alias
                    .as_ref()
                    .unwrap_or(&symbol_definition.ir_node.name)
            }
            Self::Interface(interface_definition) => &interface_definition.ir_node.name,
            Self::Library(library_definition) => &library_definition.ir_node.name,
            Self::Modifier(modifier_definition) => {
                // Modifier definitions are only created for modifiers, which
                // always have a name
                modifier_definition.ir_node.name.as_ref().unwrap()
            }
            Self::Parameter(parameter_definition) => {
                // Definitions are created only for named parameters
                parameter_definition.ir_node.name.as_ref().unwrap()
            }
            Self::StateVariable(state_variable_definition) => {
                &state_variable_definition.ir_node.name
            }
            Self::Struct(struct_definition) => &struct_definition.ir_node.name,
            Self::StructMember(struct_member_definition) => &struct_member_definition.ir_node.name,
            Self::TypeParameter(parameter_definition) => {
                // Definitions are created only for named type parameters
                parameter_definition.ir_node.name.as_ref().unwrap()
            }
            Self::UserDefinedValueType(udvt_definition) => &udvt_definition.ir_node.name,
            Self::Variable(variable_definition) => &variable_definition.ir_node.name,
            Self::YulFunction(function_definition) => &function_definition.ir_node.name,
            Self::YulParameter(parameter_definition) => &parameter_definition.ir_node,
            Self::YulVariable(variable_definition) => &variable_definition.ir_node,
        }
    }

    pub(crate) fn is_private_or_internally_visible(&self) -> bool {
        if let Self::Function(function_definition) = self {
            function_definition.visibility != ir::FunctionVisibility::External
        } else {
            true
        }
    }

    pub(crate) fn is_internally_visible(&self) -> bool {
        match self {
            Self::Function(function_definition) => {
                function_definition.visibility == ir::FunctionVisibility::Internal
                    || function_definition.visibility == ir::FunctionVisibility::Public
            }
            Self::StateVariable(variable_definition) => {
                variable_definition.visibility != ir::StateVariableVisibility::Private
            }
            _ => true,
        }
    }

    pub(crate) fn is_externally_visible(&self) -> bool {
        match self {
            Self::Function(function_definition) => {
                function_definition.visibility == ir::FunctionVisibility::Public
                    || function_definition.visibility == ir::FunctionVisibility::External
            }
            Self::StateVariable(variable_definition) => {
                variable_definition.visibility == ir::StateVariableVisibility::Public
            }
            _ => true,
        }
    }

    pub(crate) fn new_constant(ir_node: &ir::ConstantDefinition, scope_id: ScopeId) -> Self {
        // for constants we store the scope_id where it's defined to use for
        // evaluation of compile-time constants (eg. fixed arrays size)
        Self::Constant(ConstantDefinition {
            ir_node: Rc::clone(ir_node),
            scope_id,
        })
    }

    pub(crate) fn new_contract(ir_node: &ir::ContractDefinition) -> Self {
        Self::Contract(ContractDefinition {
            ir_node: Rc::clone(ir_node),
            bases: None,
            constructor_parameters_scope_id: None,
            base_slot: None,
        })
    }

    pub(crate) fn new_enum(ir_node: &ir::EnumDefinition) -> Self {
        Self::Enum(EnumDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_enum_member(ir_node: &ir::Identifier) -> Self {
        Self::EnumMember(EnumMemberDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_error(ir_node: &ir::ErrorDefinition, parameters_scope_id: ScopeId) -> Self {
        Self::Error(ErrorDefinition {
            ir_node: Rc::clone(ir_node),
            parameters_scope_id,
        })
    }

    pub(crate) fn new_event(ir_node: &ir::EventDefinition, parameters_scope_id: ScopeId) -> Self {
        Self::Event(EventDefinition {
            ir_node: Rc::clone(ir_node),
            parameters_scope_id,
        })
    }

    pub(crate) fn new_function(
        ir_node: &ir::FunctionDefinition,
        parameters_scope_id: ScopeId,
        visibility: ir::FunctionVisibility,
    ) -> Self {
        assert!(
            ir_node.name.is_some(),
            "Cannot create a definition for an unnamed function"
        );
        assert!(
            matches!(ir_node.kind, ir::FunctionKind::Regular),
            "Cannot create definition for special function or modifier"
        );
        Self::Function(FunctionDefinition {
            ir_node: Rc::clone(ir_node),
            parameters_scope_id,
            visibility,
        })
    }

    pub(crate) fn new_import(ir_node: &ir::PathImport, resolved_file_id: Option<String>) -> Self {
        assert!(
            ir_node.alias.is_some(),
            "Definition can only be created for aliased imports"
        );

        Self::Import(ImportDefinition {
            ir_node: Rc::clone(ir_node),
            resolved_file_id,
        })
    }

    pub(crate) fn new_imported_symbol(
        ir_node: &ir::ImportDeconstructionSymbol,
        symbol: String,
        resolved_file_id: Option<String>,
    ) -> Self {
        Self::ImportedSymbol(ImportedSymbolDefinition {
            ir_node: Rc::clone(ir_node),
            symbol,
            resolved_file_id,
        })
    }

    pub(crate) fn new_interface(ir_node: &ir::InterfaceDefinition) -> Self {
        Self::Interface(InterfaceDefinition {
            ir_node: Rc::clone(ir_node),
            bases: None,
        })
    }

    pub(crate) fn new_library(ir_node: &ir::LibraryDefinition) -> Self {
        Self::Library(LibraryDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_modifier(ir_node: &ir::FunctionDefinition) -> Self {
        assert!(
            ir_node.name.is_some(),
            "Modifier definition must have a name"
        );
        assert!(
            matches!(ir_node.kind, ir::FunctionKind::Modifier),
            "Cannot create definition from a non-modifier function definition"
        );
        Self::Modifier(ModifierDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_parameter(ir_node: &ir::Parameter) -> Self {
        assert!(
            ir_node.name.is_some(),
            "Parameter definition must have a name"
        );
        Self::Parameter(ParameterDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_state_variable(
        ir_node: &ir::StateVariableDefinition,
        visibility: ir::StateVariableVisibility,
    ) -> Self {
        Self::StateVariable(StateVariableDefinition {
            ir_node: Rc::clone(ir_node),
            getter_type_id: None,
            visibility,
        })
    }

    pub(crate) fn new_struct(ir_node: &ir::StructDefinition) -> Self {
        Self::Struct(StructDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_struct_member(ir_node: &ir::StructMember) -> Self {
        Self::StructMember(StructMemberDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_type_parameter(ir_node: &ir::Parameter) -> Self {
        assert!(
            ir_node.name.is_some(),
            "Cannot create definition for nameless parameter"
        );
        Self::TypeParameter(TypeParameterDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_user_defined_value_type(
        ir_node: &ir::UserDefinedValueTypeDefinition,
    ) -> Self {
        Self::UserDefinedValueType(UserDefinedValueTypeDefinition {
            ir_node: Rc::clone(ir_node),
            target_type_id: None,
        })
    }

    pub(crate) fn new_variable(ir_node: &ir::VariableDeclaration) -> Self {
        Self::Variable(VariableDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_yul_function(ir_node: &ir::YulFunctionDefinition) -> Self {
        Self::YulFunction(YulFunctionDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_yul_parameter(ir_node: &ir::Identifier) -> Self {
        Self::YulParameter(YulParameterDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_yul_variable(ir_node: &ir::Identifier) -> Self {
        Self::YulVariable(YulVariableDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }
}
