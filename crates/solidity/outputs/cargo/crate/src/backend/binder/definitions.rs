use std::rc::Rc;

use super::ScopeId;
use crate::backend::ir::ir2_flat_contracts as output_ir;
use crate::backend::types::TypeId;
use crate::cst::{NodeId, TerminalNode};

//////////////////////////////////////////////////////////////////////////////
// Definitions

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
    YulLabel(YulLabelDefinition),
    YulParameter(YulParameterDefinition),
    YulVariable(YulVariableDefinition),
}

#[derive(Debug)]
pub struct ConstantDefinition {
    pub(crate) ir_node: output_ir::ConstantDefinition,
}

#[derive(Debug)]
pub struct ContractDefinition {
    pub(crate) ir_node: output_ir::ContractDefinition,
    pub bases: Option<Vec<NodeId>>,
    pub constructor_parameters_scope_id: Option<ScopeId>,
}

impl ContractDefinition {
    pub(crate) fn name(&self) -> String {
        self.ir_node.name.unparse()
    }
}

#[derive(Debug)]
pub struct EnumDefinition {
    pub(crate) ir_node: output_ir::EnumDefinition,
}

#[derive(Debug)]
pub struct EnumMemberDefinition {
    pub(crate) ir_node: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct ErrorDefinition {
    pub(crate) ir_node: output_ir::ErrorDefinition,
    pub parameters_scope_id: ScopeId,
}

#[derive(Debug)]
pub struct EventDefinition {
    pub(crate) ir_node: output_ir::EventDefinition,
    pub parameters_scope_id: ScopeId,
}

#[derive(Debug, Eq, PartialEq)]
pub enum FunctionVisibility {
    External,
    Internal,
    Private,
    Public,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub(crate) ir_node: output_ir::FunctionDefinition,
    pub parameters_scope_id: ScopeId,
    pub visibility: FunctionVisibility,
}

#[derive(Debug)]
pub struct ImportDefinition {
    pub(crate) ir_node: output_ir::PathImport,
    pub resolved_file_id: Option<String>,
}

#[derive(Debug)]
pub struct ImportedSymbolDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
    pub symbol: String,
    pub resolved_file_id: Option<String>,
}

#[derive(Debug)]
pub struct InterfaceDefinition {
    pub(crate) ir_node: output_ir::InterfaceDefinition,
    pub bases: Option<Vec<NodeId>>,
}

#[derive(Debug)]
pub struct LibraryDefinition {
    pub(crate) ir_node: output_ir::LibraryDefinition,
}

#[derive(Debug)]
pub struct ModifierDefinition {
    pub(crate) ir_node: output_ir::FunctionDefinition,
}

#[derive(Debug)]
pub struct ParameterDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum StateVariableVisibility {
    Internal,
    Private,
    Public,
}

#[derive(Debug)]
pub struct StateVariableDefinition {
    pub(crate) ir_node: output_ir::StateVariableDefinition,
    pub getter_type_id: Option<TypeId>,
    pub visibility: StateVariableVisibility,
}

#[derive(Debug)]
pub struct StructDefinition {
    pub(crate) ir_node: output_ir::StructDefinition,
}

#[derive(Debug)]
pub struct StructMemberDefinition {
    pub(crate) ir_node: output_ir::StructMember,
}

#[derive(Debug)]
pub struct TypeParameterDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct UserDefinedValueTypeDefinition {
    pub(crate) ir_node: output_ir::UserDefinedValueTypeDefinition,
    pub target_type_id: Option<TypeId>,
}

#[derive(Debug)]
pub struct VariableDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct YulLabelDefinition {
    pub(crate) ir_node: output_ir::YulLabel,
}

#[derive(Debug)]
pub struct YulFunctionDefinition {
    pub(crate) ir_node: output_ir::YulFunctionDefinition,
}

#[derive(Debug)]
pub struct YulParameterDefinition {
    pub(crate) ir_node: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct YulVariableDefinition {
    pub(crate) ir_node: Rc<TerminalNode>,
}

impl Definition {
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Constant(constant_definition) => constant_definition.ir_node.node_id,
            Self::Contract(contract_definition) => contract_definition.ir_node.node_id,
            Self::Enum(enum_definition) => enum_definition.ir_node.node_id,
            Self::EnumMember(enum_member_definition) => enum_member_definition.ir_node.id(),
            Self::Error(error_definition) => error_definition.ir_node.node_id,
            Self::Event(event_definition) => event_definition.ir_node.node_id,
            Self::Function(function_definition) => function_definition.ir_node.node_id,
            Self::Import(import_definition) => import_definition.ir_node.node_id,
            Self::ImportedSymbol(imported_symbol_definition) => imported_symbol_definition.node_id,
            Self::Interface(interface_definition) => interface_definition.ir_node.node_id,
            Self::Library(library_definition) => library_definition.ir_node.node_id,
            Self::Modifier(modifier_definition) => modifier_definition.ir_node.node_id,
            Self::Parameter(parameter_definition) => parameter_definition.node_id,
            Self::StateVariable(state_variable_definition) => {
                state_variable_definition.ir_node.node_id
            }
            Self::Struct(struct_definition) => struct_definition.ir_node.node_id,
            Self::StructMember(struct_member_definition) => {
                struct_member_definition.ir_node.node_id
            }
            Self::TypeParameter(parameter_definition) => parameter_definition.node_id,
            Self::UserDefinedValueType(udvt_definition) => udvt_definition.ir_node.node_id,
            Self::Variable(variable_definition) => variable_definition.node_id,
            Self::YulFunction(function_definition) => function_definition.ir_node.node_id,
            Self::YulLabel(label_definition) => label_definition.ir_node.node_id,
            Self::YulParameter(parameter_definition) => parameter_definition.ir_node.id(),
            Self::YulVariable(variable_definition) => variable_definition.ir_node.id(),
        }
    }

    pub fn identifier(&self) -> &Rc<TerminalNode> {
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
            Self::ImportedSymbol(symbol_definition) => &symbol_definition.identifier,
            Self::Interface(interface_definition) => &interface_definition.ir_node.name,
            Self::Library(library_definition) => &library_definition.ir_node.name,
            Self::Modifier(modifier_definition) => {
                // Modifier definitions are only created for modifiers, which
                // always have a name
                modifier_definition.ir_node.name.as_ref().unwrap()
            }
            Self::Parameter(parameter_definition) => &parameter_definition.identifier,
            Self::StateVariable(state_variable_definition) => {
                &state_variable_definition.ir_node.name
            }
            Self::Struct(struct_definition) => &struct_definition.ir_node.name,
            Self::StructMember(struct_member_definition) => &struct_member_definition.ir_node.name,
            Self::TypeParameter(parameter_definition) => &parameter_definition.identifier,
            Self::UserDefinedValueType(udvt_definition) => &udvt_definition.ir_node.name,
            Self::Variable(variable_definition) => &variable_definition.identifier,
            Self::YulFunction(function_definition) => &function_definition.ir_node.name,
            Self::YulLabel(label_definition) => &label_definition.ir_node.label,
            Self::YulParameter(parameter_definition) => &parameter_definition.ir_node,
            Self::YulVariable(variable_definition) => &variable_definition.ir_node,
        }
    }

    pub(crate) fn is_private_or_internally_visible(&self) -> bool {
        if let Self::Function(function_definition) = self {
            function_definition.visibility != FunctionVisibility::External
        } else {
            true
        }
    }

    pub(crate) fn is_internally_visible(&self) -> bool {
        match self {
            Self::Function(function_definition) => {
                function_definition.visibility == FunctionVisibility::Internal
                    || function_definition.visibility == FunctionVisibility::Public
            }
            Self::StateVariable(variable_definition) => {
                variable_definition.visibility != StateVariableVisibility::Private
            }
            _ => true,
        }
    }

    pub(crate) fn is_externally_visible(&self) -> bool {
        match self {
            Self::Function(function_definition) => {
                function_definition.visibility == FunctionVisibility::Public
                    || function_definition.visibility == FunctionVisibility::External
            }
            Self::StateVariable(variable_definition) => {
                variable_definition.visibility == StateVariableVisibility::Public
            }
            _ => true,
        }
    }

    pub(crate) fn new_constant(ir_node: &output_ir::ConstantDefinition) -> Self {
        Self::Constant(ConstantDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_contract(ir_node: &output_ir::ContractDefinition) -> Self {
        Self::Contract(ContractDefinition {
            ir_node: Rc::clone(ir_node),
            bases: None,
            constructor_parameters_scope_id: None,
        })
    }

    pub(crate) fn new_enum(ir_node: &output_ir::EnumDefinition) -> Self {
        Self::Enum(EnumDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_enum_member(ir_node: &Rc<TerminalNode>) -> Self {
        Self::EnumMember(EnumMemberDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_error(
        ir_node: &output_ir::ErrorDefinition,
        parameters_scope_id: ScopeId,
    ) -> Self {
        Self::Error(ErrorDefinition {
            ir_node: Rc::clone(ir_node),
            parameters_scope_id,
        })
    }

    pub(crate) fn new_event(
        ir_node: &output_ir::EventDefinition,
        parameters_scope_id: ScopeId,
    ) -> Self {
        Self::Event(EventDefinition {
            ir_node: Rc::clone(ir_node),
            parameters_scope_id,
        })
    }

    pub(crate) fn new_function(
        ir_node: &output_ir::FunctionDefinition,
        parameters_scope_id: ScopeId,
        visibility: FunctionVisibility,
    ) -> Self {
        assert!(
            ir_node.name.is_some(),
            "Cannot create a definition for an unnamed function"
        );
        assert!(
            matches!(ir_node.kind, output_ir::FunctionKind::Regular),
            "Cannot create definition for special function or modifier"
        );
        Self::Function(FunctionDefinition {
            ir_node: Rc::clone(ir_node),
            parameters_scope_id,
            visibility,
        })
    }

    pub(crate) fn new_import(
        ir_node: &output_ir::PathImport,
        resolved_file_id: Option<String>,
    ) -> Self {
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
        node_id: NodeId,
        identifier: &Rc<TerminalNode>,
        symbol: String,
        resolved_file_id: Option<String>,
    ) -> Self {
        Self::ImportedSymbol(ImportedSymbolDefinition {
            node_id,
            identifier: Rc::clone(identifier),
            symbol,
            resolved_file_id,
        })
    }

    pub(crate) fn new_interface(ir_node: &output_ir::InterfaceDefinition) -> Self {
        Self::Interface(InterfaceDefinition {
            ir_node: Rc::clone(ir_node),
            bases: None,
        })
    }

    pub(crate) fn new_library(ir_node: &output_ir::LibraryDefinition) -> Self {
        Self::Library(LibraryDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_modifier(ir_node: &output_ir::FunctionDefinition) -> Self {
        assert!(
            ir_node.name.is_some(),
            "Modifier definition must have a name"
        );
        assert!(
            matches!(ir_node.kind, output_ir::FunctionKind::Modifier),
            "Cannot create definition from a non-modifier function definition"
        );
        Self::Modifier(ModifierDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_parameter(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Parameter(ParameterDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_state_variable(
        ir_node: &output_ir::StateVariableDefinition,
        visibility: StateVariableVisibility,
    ) -> Self {
        Self::StateVariable(StateVariableDefinition {
            ir_node: Rc::clone(ir_node),
            getter_type_id: None,
            visibility,
        })
    }

    pub(crate) fn new_struct(ir_node: &output_ir::StructDefinition) -> Self {
        Self::Struct(StructDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_struct_member(ir_node: &output_ir::StructMember) -> Self {
        Self::StructMember(StructMemberDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_type_parameter(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::TypeParameter(TypeParameterDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_user_defined_value_type(
        ir_node: &output_ir::UserDefinedValueTypeDefinition,
    ) -> Self {
        Self::UserDefinedValueType(UserDefinedValueTypeDefinition {
            ir_node: Rc::clone(ir_node),
            target_type_id: None,
        })
    }

    pub(crate) fn new_variable(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Variable(VariableDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_yul_function(ir_node: &output_ir::YulFunctionDefinition) -> Self {
        Self::YulFunction(YulFunctionDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_yul_label(ir_node: &output_ir::YulLabel) -> Self {
        Self::YulLabel(YulLabelDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_yul_parameter(ir_node: &Rc<TerminalNode>) -> Self {
        Self::YulParameter(YulParameterDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }

    pub(crate) fn new_yul_variable(ir_node: &Rc<TerminalNode>) -> Self {
        Self::YulVariable(YulVariableDefinition {
            ir_node: Rc::clone(ir_node),
        })
    }
}
