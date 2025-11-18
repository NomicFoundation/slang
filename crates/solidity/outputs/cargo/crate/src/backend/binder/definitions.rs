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
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct ContractDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
    pub bases: Option<Vec<NodeId>>,
    pub constructor_parameters_scope_id: Option<ScopeId>,
}

#[derive(Debug)]
pub struct EnumDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct EnumMemberDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct ErrorDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
    pub parameters_scope_id: ScopeId,
}

#[derive(Debug)]
pub struct EventDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
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
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
    pub parameters_scope_id: ScopeId,
    pub visibility: FunctionVisibility,
}

#[derive(Debug)]
pub struct ImportDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
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
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
    pub bases: Option<Vec<NodeId>>,
}

#[derive(Debug)]
pub struct LibraryDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct ModifierDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
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
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
    pub getter_type_id: Option<TypeId>,
    pub visibility: StateVariableVisibility,
}

#[derive(Debug)]
pub struct StructDefinition {
    pub(crate) ir_node: output_ir::StructDefinition,
}

#[derive(Debug)]
pub struct StructMemberDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct TypeParameterDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct UserDefinedValueTypeDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
    pub target_type_id: Option<TypeId>,
}

#[derive(Debug)]
pub struct VariableDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct YulLabelDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct YulFunctionDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct YulParameterDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct YulVariableDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

impl Definition {
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Constant(constant_definition) => constant_definition.node_id,
            Self::Contract(contract_definition) => contract_definition.node_id,
            Self::Enum(enum_definition) => enum_definition.node_id,
            Self::EnumMember(enum_member_definition) => enum_member_definition.node_id,
            Self::Error(error_definition) => error_definition.node_id,
            Self::Event(event_definition) => event_definition.node_id,
            Self::Function(function_definition) => function_definition.node_id,
            Self::Import(import_definition) => import_definition.node_id,
            Self::ImportedSymbol(imported_symbol_definition) => imported_symbol_definition.node_id,
            Self::Interface(interface_definition) => interface_definition.node_id,
            Self::Library(library_definition) => library_definition.node_id,
            Self::Modifier(modifier_definition) => modifier_definition.node_id,
            Self::Parameter(parameter_definition) => parameter_definition.node_id,
            Self::StateVariable(state_variable_definition) => state_variable_definition.node_id,
            Self::Struct(struct_definition) => struct_definition.ir_node.node_id,
            Self::StructMember(struct_member_definition) => struct_member_definition.node_id,
            Self::TypeParameter(parameter_definition) => parameter_definition.node_id,
            Self::UserDefinedValueType(udvt_definition) => udvt_definition.node_id,
            Self::Variable(variable_definition) => variable_definition.node_id,
            Self::YulFunction(function_definition) => function_definition.node_id,
            Self::YulLabel(label_definition) => label_definition.node_id,
            Self::YulParameter(parameter_definition) => parameter_definition.node_id,
            Self::YulVariable(variable_definition) => variable_definition.node_id,
        }
    }

    pub fn identifier(&self) -> &Rc<TerminalNode> {
        match self {
            Self::Constant(constant_definition) => &constant_definition.identifier,
            Self::Contract(contract_definition) => &contract_definition.identifier,
            Self::Enum(enum_definition) => &enum_definition.identifier,
            Self::EnumMember(enum_member_definition) => &enum_member_definition.identifier,
            Self::Error(error_definition) => &error_definition.identifier,
            Self::Event(event_definition) => &event_definition.identifier,
            Self::Function(function_definition) => &function_definition.identifier,
            Self::Import(import_definition) => &import_definition.identifier,
            Self::ImportedSymbol(symbol_definition) => &symbol_definition.identifier,
            Self::Interface(interface_definition) => &interface_definition.identifier,
            Self::Library(library_definition) => &library_definition.identifier,
            Self::Modifier(modifier_definition) => &modifier_definition.identifier,
            Self::Parameter(parameter_definition) => &parameter_definition.identifier,
            Self::StateVariable(state_variable_definition) => &state_variable_definition.identifier,
            Self::Struct(struct_definition) => &struct_definition.ir_node.name,
            Self::StructMember(struct_member_definition) => &struct_member_definition.identifier,
            Self::TypeParameter(parameter_definition) => &parameter_definition.identifier,
            Self::UserDefinedValueType(udvt_definition) => &udvt_definition.identifier,
            Self::Variable(variable_definition) => &variable_definition.identifier,
            Self::YulFunction(function_definition) => &function_definition.identifier,
            Self::YulLabel(label_definition) => &label_definition.identifier,
            Self::YulParameter(parameter_definition) => &parameter_definition.identifier,
            Self::YulVariable(variable_definition) => &variable_definition.identifier,
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

    pub(crate) fn new_constant(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Constant(ConstantDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_contract(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Contract(ContractDefinition {
            node_id,
            identifier: Rc::clone(identifier),
            bases: None,
            constructor_parameters_scope_id: None,
        })
    }

    pub(crate) fn new_enum(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Enum(EnumDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_enum_member(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::EnumMember(EnumMemberDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_error(
        node_id: NodeId,
        identifier: &Rc<TerminalNode>,
        parameters_scope_id: ScopeId,
    ) -> Self {
        Self::Error(ErrorDefinition {
            node_id,
            identifier: Rc::clone(identifier),
            parameters_scope_id,
        })
    }

    pub(crate) fn new_event(
        node_id: NodeId,
        identifier: &Rc<TerminalNode>,
        parameters_scope_id: ScopeId,
    ) -> Self {
        Self::Event(EventDefinition {
            node_id,
            identifier: Rc::clone(identifier),
            parameters_scope_id,
        })
    }

    pub(crate) fn new_function(
        node_id: NodeId,
        identifier: &Rc<TerminalNode>,
        parameters_scope_id: ScopeId,
        visibility: FunctionVisibility,
    ) -> Self {
        Self::Function(FunctionDefinition {
            node_id,
            identifier: Rc::clone(identifier),
            parameters_scope_id,
            visibility,
        })
    }

    pub(crate) fn new_import(
        node_id: NodeId,
        identifier: &Rc<TerminalNode>,
        resolved_file_id: Option<String>,
    ) -> Self {
        Self::Import(ImportDefinition {
            node_id,
            identifier: Rc::clone(identifier),
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

    pub(crate) fn new_interface(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Interface(InterfaceDefinition {
            node_id,
            identifier: Rc::clone(identifier),
            bases: None,
        })
    }

    pub(crate) fn new_library(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Library(LibraryDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_modifier(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Modifier(ModifierDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_parameter(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Parameter(ParameterDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_state_variable(
        node_id: NodeId,
        identifier: &Rc<TerminalNode>,
        visibility: StateVariableVisibility,
    ) -> Self {
        Self::StateVariable(StateVariableDefinition {
            node_id,
            identifier: Rc::clone(identifier),
            getter_type_id: None,
            visibility,
        })
    }

    pub(crate) fn new_struct(node: &output_ir::StructDefinition) -> Self {
        Self::Struct(StructDefinition {
            ir_node: Rc::clone(node),
        })
    }

    pub(crate) fn new_struct_member(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::StructMember(StructMemberDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_type_parameter(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::TypeParameter(TypeParameterDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_user_defined_value_type(
        node_id: NodeId,
        identifier: &Rc<TerminalNode>,
    ) -> Self {
        Self::UserDefinedValueType(UserDefinedValueTypeDefinition {
            node_id,
            identifier: Rc::clone(identifier),
            target_type_id: None,
        })
    }

    pub(crate) fn new_variable(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Variable(VariableDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_yul_function(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::YulFunction(YulFunctionDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_yul_label(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::YulLabel(YulLabelDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_yul_parameter(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::YulParameter(YulParameterDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_yul_variable(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::YulVariable(YulVariableDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }
}
