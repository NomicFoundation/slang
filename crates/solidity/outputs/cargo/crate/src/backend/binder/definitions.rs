use std::rc::Rc;

use super::ScopeId;
use crate::backend::types::TypeId;
use crate::cst::{NodeId, SyntaxNode};

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
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
}

#[derive(Debug)]
pub struct ContractDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
    pub bases: Option<Vec<NodeId>>,
    pub constructor_parameters_scope_id: Option<ScopeId>,
}

#[derive(Debug)]
pub struct EnumDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
}

#[derive(Debug)]
pub struct EnumMemberDefinition {
    pub node: Rc<SyntaxNode>,
}

#[derive(Debug)]
pub struct ErrorDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
    pub parameters_scope_id: ScopeId,
}

#[derive(Debug)]
pub struct EventDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
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
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
    pub parameters_scope_id: ScopeId,
    pub visibility: FunctionVisibility,
}

#[derive(Debug)]
pub struct ImportDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
    pub resolved_file_id: Option<String>,
}

#[derive(Debug)]
pub struct ImportedSymbolDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
    pub symbol: String,
    pub resolved_file_id: Option<String>,
}

#[derive(Debug)]
pub struct InterfaceDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
    pub bases: Option<Vec<NodeId>>,
}

#[derive(Debug)]
pub struct LibraryDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
}

#[derive(Debug)]
pub struct ModifierDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
}

#[derive(Debug)]
pub struct ParameterDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum StateVariableVisibility {
    Internal,
    Private,
    Public,
}

#[derive(Debug)]
pub struct StateVariableDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
    pub getter_type_id: Option<TypeId>,
    pub visibility: StateVariableVisibility,
}

#[derive(Debug)]
pub struct StructDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
}

#[derive(Debug)]
pub struct StructMemberDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
}

#[derive(Debug)]
pub struct TypeParameterDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
}

#[derive(Debug)]
pub struct UserDefinedValueTypeDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
    pub target_type_id: Option<TypeId>,
}

#[derive(Debug)]
pub struct VariableDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
}

#[derive(Debug)]
pub struct YulLabelDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
}

#[derive(Debug)]
pub struct YulFunctionDefinition {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<SyntaxNode>,
}

#[derive(Debug)]
pub struct YulParameterDefinition {
    pub node: Rc<SyntaxNode>,
}

#[derive(Debug)]
pub struct YulVariableDefinition {
    pub node: Rc<SyntaxNode>,
}

impl Definition {
    pub fn node(&self) -> Rc<SyntaxNode> {
        match self {
            Self::Constant(constant_definition) => Rc::clone(&constant_definition.node),
            Self::Contract(contract_definition) => Rc::clone(&contract_definition.node),
            Self::Enum(enum_definition) => Rc::clone(&enum_definition.node),
            Self::EnumMember(enum_member_definition) => Rc::clone(&enum_member_definition.node),
            Self::Error(error_definition) => Rc::clone(&error_definition.node),
            Self::Event(event_definition) => Rc::clone(&event_definition.node),
            Self::Function(function_definition) => Rc::clone(&function_definition.node),
            Self::Import(import_definition) => Rc::clone(&import_definition.node),
            Self::ImportedSymbol(imported_symbol_definition) => {
                Rc::clone(&imported_symbol_definition.node)
            }
            Self::Interface(interface_definition) => Rc::clone(&interface_definition.node),
            Self::Library(library_definition) => Rc::clone(&library_definition.node),
            Self::Modifier(modifier_definition) => Rc::clone(&modifier_definition.node),
            Self::Parameter(parameter_definition) => Rc::clone(&parameter_definition.node),
            Self::StateVariable(state_variable_definition) => {
                Rc::clone(&state_variable_definition.node)
            }
            Self::Struct(struct_definition) => Rc::clone(&struct_definition.node),
            Self::StructMember(struct_member_definition) => {
                Rc::clone(&struct_member_definition.node)
            }
            Self::TypeParameter(parameter_definition) => Rc::clone(&parameter_definition.node),
            Self::UserDefinedValueType(udvt_definition) => Rc::clone(&udvt_definition.node),
            Self::Variable(variable_definition) => Rc::clone(&variable_definition.node),
            Self::YulFunction(function_definition) => Rc::clone(&function_definition.node),
            Self::YulLabel(label_definition) => Rc::clone(&label_definition.node),
            Self::YulParameter(parameter_definition) => Rc::clone(&parameter_definition.node),
            Self::YulVariable(variable_definition) => Rc::clone(&variable_definition.node),
        }
    }

    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Constant(constant_definition) => constant_definition.node.id(),
            Self::Contract(contract_definition) => contract_definition.node.id(),
            Self::Enum(enum_definition) => enum_definition.node.id(),
            Self::EnumMember(enum_member_definition) => enum_member_definition.node.id(),
            Self::Error(error_definition) => error_definition.node.id(),
            Self::Event(event_definition) => event_definition.node.id(),
            Self::Function(function_definition) => function_definition.node.id(),
            Self::Import(import_definition) => import_definition.node.id(),
            Self::ImportedSymbol(imported_symbol_definition) => {
                imported_symbol_definition.node.id()
            }
            Self::Interface(interface_definition) => interface_definition.node.id(),
            Self::Library(library_definition) => library_definition.node.id(),
            Self::Modifier(modifier_definition) => modifier_definition.node.id(),
            Self::Parameter(parameter_definition) => parameter_definition.node.id(),
            Self::StateVariable(state_variable_definition) => state_variable_definition.node.id(),
            Self::Struct(struct_definition) => struct_definition.node.id(),
            Self::StructMember(struct_member_definition) => struct_member_definition.node.id(),
            Self::TypeParameter(parameter_definition) => parameter_definition.node.id(),
            Self::UserDefinedValueType(udvt_definition) => udvt_definition.node.id(),
            Self::Variable(variable_definition) => variable_definition.node.id(),
            Self::YulFunction(function_definition) => function_definition.node.id(),
            Self::YulLabel(label_definition) => label_definition.node.id(),
            Self::YulParameter(parameter_definition) => parameter_definition.node.id(),
            Self::YulVariable(variable_definition) => variable_definition.node.id(),
        }
    }

    pub fn identifier(&self) -> &Rc<SyntaxNode> {
        match self {
            Self::Constant(constant_definition) => &constant_definition.identifier,
            Self::Contract(contract_definition) => &contract_definition.identifier,
            Self::Enum(enum_definition) => &enum_definition.identifier,
            Self::EnumMember(enum_member_definition) => &enum_member_definition.node,
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
            Self::Struct(struct_definition) => &struct_definition.identifier,
            Self::StructMember(struct_member_definition) => &struct_member_definition.identifier,
            Self::TypeParameter(parameter_definition) => &parameter_definition.identifier,
            Self::UserDefinedValueType(udvt_definition) => &udvt_definition.identifier,
            Self::Variable(variable_definition) => &variable_definition.identifier,
            Self::YulFunction(function_definition) => &function_definition.identifier,
            Self::YulLabel(label_definition) => &label_definition.identifier,
            Self::YulParameter(parameter_definition) => &parameter_definition.node,
            Self::YulVariable(variable_definition) => &variable_definition.node,
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

    pub(crate) fn new_constant(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::Constant(ConstantDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_contract(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::Contract(ContractDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
            bases: None,
            constructor_parameters_scope_id: None,
        })
    }

    pub(crate) fn new_enum(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::Enum(EnumDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_enum_member(node: &Rc<SyntaxNode>) -> Self {
        Self::EnumMember(EnumMemberDefinition {
            node: Rc::clone(node),
        })
    }

    pub(crate) fn new_error(
        node: &Rc<SyntaxNode>,
        identifier: &Rc<SyntaxNode>,
        parameters_scope_id: ScopeId,
    ) -> Self {
        Self::Error(ErrorDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
            parameters_scope_id,
        })
    }

    pub(crate) fn new_event(
        node: &Rc<SyntaxNode>,
        identifier: &Rc<SyntaxNode>,
        parameters_scope_id: ScopeId,
    ) -> Self {
        Self::Event(EventDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
            parameters_scope_id,
        })
    }

    pub(crate) fn new_function(
        node: &Rc<SyntaxNode>,
        identifier: &Rc<SyntaxNode>,
        parameters_scope_id: ScopeId,
        visibility: FunctionVisibility,
    ) -> Self {
        Self::Function(FunctionDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
            parameters_scope_id,
            visibility,
        })
    }

    pub(crate) fn new_import(
        node: &Rc<SyntaxNode>,
        identifier: &Rc<SyntaxNode>,
        resolved_file_id: Option<String>,
    ) -> Self {
        Self::Import(ImportDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
            resolved_file_id,
        })
    }

    pub(crate) fn new_imported_symbol(
        node: &Rc<SyntaxNode>,
        identifier: &Rc<SyntaxNode>,
        symbol: String,
        resolved_file_id: Option<String>,
    ) -> Self {
        Self::ImportedSymbol(ImportedSymbolDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
            symbol,
            resolved_file_id,
        })
    }

    pub(crate) fn new_interface(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::Interface(InterfaceDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
            bases: None,
        })
    }

    pub(crate) fn new_library(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::Library(LibraryDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_modifier(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::Modifier(ModifierDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_parameter(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::Parameter(ParameterDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_state_variable(
        node: &Rc<SyntaxNode>,
        identifier: &Rc<SyntaxNode>,
        visibility: StateVariableVisibility,
    ) -> Self {
        Self::StateVariable(StateVariableDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
            getter_type_id: None,
            visibility,
        })
    }

    pub(crate) fn new_struct(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::Struct(StructDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_struct_member(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::StructMember(StructMemberDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_type_parameter(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::TypeParameter(TypeParameterDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_user_defined_value_type(
        node: &Rc<SyntaxNode>,
        identifier: &Rc<SyntaxNode>,
    ) -> Self {
        Self::UserDefinedValueType(UserDefinedValueTypeDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
            target_type_id: None,
        })
    }

    pub(crate) fn new_variable(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::Variable(VariableDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_yul_function(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::YulFunction(YulFunctionDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_yul_label(node: &Rc<SyntaxNode>, identifier: &Rc<SyntaxNode>) -> Self {
        Self::YulLabel(YulLabelDefinition {
            node: Rc::clone(node),
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_yul_parameter(node: &Rc<SyntaxNode>) -> Self {
        Self::YulParameter(YulParameterDefinition {
            node: Rc::clone(node),
        })
    }

    pub(crate) fn new_yul_variable(node: &Rc<SyntaxNode>) -> Self {
        Self::YulVariable(YulVariableDefinition {
            node: Rc::clone(node),
        })
    }
}
