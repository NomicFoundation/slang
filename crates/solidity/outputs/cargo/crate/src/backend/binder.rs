use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use metaslang_cst::nodes::NodeId;

use super::built_ins::BuiltIn;
use super::types::TypeId;
use crate::cst::{TerminalKind, TerminalNode};

//////////////////////////////////////////////////////////////////////////////
// Definitions

#[derive(Debug)]
pub struct ConstantDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct ContractDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
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

#[derive(Debug)]
pub struct FunctionDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
    pub parameters_scope_id: ScopeId,
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

#[derive(Debug)]
pub struct StateVariableDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct StructDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
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
            Self::Struct(struct_definition) => struct_definition.node_id,
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
            Self::Struct(struct_definition) => &struct_definition.identifier,
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
    ) -> Self {
        Self::Function(FunctionDefinition {
            node_id,
            identifier: Rc::clone(identifier),
            parameters_scope_id,
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

    pub(crate) fn new_state_variable(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::StateVariable(StateVariableDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub(crate) fn new_struct(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Struct(StructDefinition {
            node_id,
            identifier: Rc::clone(identifier),
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

//////////////////////////////////////////////////////////////////////////////
// References

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Resolution {
    Unresolved,
    Definition(NodeId),
    Ambiguous(Vec<NodeId>),
    BuiltIn(BuiltIn),
}

impl Resolution {
    pub fn as_definition_id(&self) -> Option<NodeId> {
        if let Resolution::Definition(definition_id) = self {
            Some(*definition_id)
        } else {
            None
        }
    }

    pub(crate) fn get_definition_ids(&self) -> Vec<NodeId> {
        match self {
            Resolution::Definition(id) => vec![*id],
            Resolution::Ambiguous(ids) => ids.clone(),
            _ => Vec::new(),
        }
    }

    #[must_use]
    pub fn non_ambiguous(self) -> Self {
        if matches!(self, Self::Ambiguous(_)) {
            Self::Unresolved
        } else {
            self
        }
    }

    fn or_else<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        if self == Self::Unresolved {
            f()
        } else {
            self
        }
    }
}

impl From<Option<NodeId>> for Resolution {
    fn from(value: Option<NodeId>) -> Self {
        if let Some(definition_id) = value {
            Self::Definition(definition_id)
        } else {
            Self::Unresolved
        }
    }
}

impl From<Option<&NodeId>> for Resolution {
    fn from(value: Option<&NodeId>) -> Self {
        if let Some(definition_id) = value {
            Self::Definition(*definition_id)
        } else {
            Self::Unresolved
        }
    }
}

impl From<Vec<NodeId>> for Resolution {
    fn from(mut value: Vec<NodeId>) -> Self {
        match value.len() {
            0 => Resolution::Unresolved,
            1 => Resolution::Definition(value.swap_remove(0)),
            _ => Resolution::Ambiguous(value),
        }
    }
}

impl From<Option<BuiltIn>> for Resolution {
    fn from(value: Option<BuiltIn>) -> Self {
        if let Some(built_in) = value {
            Self::BuiltIn(built_in)
        } else {
            Self::Unresolved
        }
    }
}

#[derive(Debug)]
pub struct Reference {
    pub identifier: Rc<TerminalNode>,
    pub resolution: Resolution,
}

impl Reference {
    fn node_id(&self) -> NodeId {
        self.identifier.id()
    }

    pub(crate) fn new(identifier: Rc<TerminalNode>, resolution: Resolution) -> Self {
        Self {
            identifier,
            resolution,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Scopes

#[allow(dead_code)]
pub(crate) enum UsingDirective {
    AllTypes {
        scope_id: ScopeId,
    },
    SingleType {
        scope_id: ScopeId,
        type_id: TypeId,
    },
    SingleTypeOperator {
        scope_id: ScopeId,
        operator_mapping: HashMap<TerminalKind, String>,
        type_id: TypeId,
    },
}

impl UsingDirective {
    pub(crate) fn new_all(scope_id: ScopeId) -> Self {
        Self::AllTypes { scope_id }
    }

    pub(crate) fn new_single_type(scope_id: ScopeId, type_id: TypeId) -> Self {
        Self::SingleType { scope_id, type_id }
    }

    pub(crate) fn new_single_type_with_operators(
        scope_id: ScopeId,
        type_id: TypeId,
        operator_mapping: HashMap<TerminalKind, String>,
    ) -> Self {
        Self::SingleTypeOperator {
            scope_id,
            operator_mapping,
            type_id,
        }
    }

    pub fn applies_to(&self, filter_type_id: TypeId) -> bool {
        match self {
            Self::AllTypes { .. } => true,
            Self::SingleType { type_id, .. } | Self::SingleTypeOperator { type_id, .. } => {
                *type_id == filter_type_id
            }
        }
    }

    pub fn get_scope_id(&self) -> ScopeId {
        match self {
            UsingDirective::AllTypes { scope_id }
            | UsingDirective::SingleType { scope_id, .. }
            | UsingDirective::SingleTypeOperator { scope_id, .. } => *scope_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(usize);

pub(crate) struct BlockScope {
    node_id: NodeId,
    parent_scope_id: ScopeId,
    definitions: HashMap<String, NodeId>,
}

impl BlockScope {
    fn new(node_id: NodeId, parent_scope_id: ScopeId) -> Self {
        Self {
            node_id,
            parent_scope_id,
            definitions: HashMap::new(),
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        let symbol = definition.identifier().unparse();
        let node_id = definition.node_id();
        self.definitions.insert(symbol, node_id);
    }
}

pub(crate) struct ContractScope {
    node_id: NodeId,
    file_scope_id: ScopeId,
    definitions: HashMap<String, Vec<NodeId>>,
    using_directives: Vec<UsingDirective>,
}

impl ContractScope {
    fn new(node_id: NodeId, file_scope_id: ScopeId) -> Self {
        Self {
            node_id,
            file_scope_id,
            definitions: HashMap::new(),
            using_directives: Vec::new(),
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        let symbol = definition.identifier().unparse();
        let node_id = definition.node_id();
        if let Some(definitions) = self.definitions.get_mut(&symbol) {
            definitions.push(node_id);
        } else {
            self.definitions.insert(symbol, vec![node_id]);
        }
    }
}

pub(crate) struct EnumScope {
    pub node_id: NodeId,
    pub definitions: HashMap<String, NodeId>,
}

impl EnumScope {
    fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
            definitions: HashMap::new(),
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        let symbol = definition.identifier().unparse();
        let node_id = definition.node_id();
        self.definitions.insert(symbol, node_id);
    }
}

pub(crate) struct FileScope {
    node_id: NodeId,
    file_id: String,
    definitions: HashMap<String, Vec<NodeId>>,
    imported_files: HashSet<String>,
    using_directives: Vec<UsingDirective>,
}

impl FileScope {
    fn new(node_id: NodeId, file_id: &str) -> Self {
        Self {
            node_id,
            file_id: file_id.to_string(),
            definitions: HashMap::new(),
            imported_files: HashSet::new(),
            using_directives: Vec::new(),
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        let symbol = definition.identifier().unparse();
        let node_id = definition.node_id();
        if let Some(definitions) = self.definitions.get_mut(&symbol) {
            definitions.push(node_id);
        } else {
            self.definitions.insert(symbol, vec![node_id]);
        }
    }

    pub(crate) fn add_imported_file(&mut self, file_id: String) {
        self.imported_files.insert(file_id);
    }

    fn lookup_symbol(&self, symbol: &str) -> Vec<NodeId> {
        self.definitions.get(symbol).cloned().unwrap_or(Vec::new())
    }
}

pub(crate) struct FunctionScope {
    pub node_id: NodeId,
    pub parent_scope_id: ScopeId,
    pub parameters_scope_id: ScopeId,
    pub definitions: HashMap<String, NodeId>,
}

impl FunctionScope {
    fn new(node_id: NodeId, parent_scope_id: ScopeId, parameters_scope_id: ScopeId) -> Self {
        Self {
            node_id,
            parent_scope_id,
            parameters_scope_id,
            definitions: HashMap::new(),
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        let symbol = definition.identifier().unparse();
        let node_id = definition.node_id();
        self.definitions.insert(symbol, node_id);
    }
}

// TODO: this is similar to a function scope, but it doesn't have a separate
// parameters scope; it also should bind the special `_` symbol to the built-in.
// There are other functions that don't need a separate parameters scope (eg.
// receive/fallback/unnamed), but they don't need to bind `_`. Should we
// refactor? Or remove this and make the parameters optional in FunctionScope?
// Probably the latter as we can control resolution in the relevant pass.
pub(crate) struct ModifierScope {
    pub node_id: NodeId,
    pub parent_scope_id: ScopeId,
    pub definitions: HashMap<String, NodeId>,
}

impl ModifierScope {
    fn new(node_id: NodeId, parent_scope_id: ScopeId) -> Self {
        Self {
            node_id,
            parent_scope_id,
            definitions: HashMap::new(),
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        let symbol = definition.identifier().unparse();
        let node_id = definition.node_id();
        self.definitions.insert(symbol, node_id);
    }
}

pub(crate) struct ParametersScope {
    pub node_id: NodeId,
    pub definitions: HashMap<String, NodeId>,
}

impl ParametersScope {
    fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
            definitions: HashMap::new(),
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        let symbol = definition.identifier().unparse();
        let node_id = definition.node_id();
        self.definitions.insert(symbol, node_id);
    }
}

pub(crate) struct StructScope {
    pub node_id: NodeId,
    pub definitions: HashMap<String, NodeId>,
}

impl StructScope {
    fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
            definitions: HashMap::new(),
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        let symbol = definition.identifier().unparse();
        let node_id = definition.node_id();
        self.definitions.insert(symbol, node_id);
    }
}

pub(crate) struct UsingScope {
    pub node_id: NodeId,
    pub symbols: HashMap<String, Vec<NodeId>>,
}

impl UsingScope {
    fn new(node_id: NodeId, symbols: HashMap<String, Vec<NodeId>>) -> Self {
        Self { node_id, symbols }
    }
}

pub(crate) struct YulBlockScope {
    pub node_id: NodeId,
    pub parent_scope_id: ScopeId,
    pub definitions: HashMap<String, NodeId>,
}

impl YulBlockScope {
    fn new(node_id: NodeId, parent_scope_id: ScopeId) -> Self {
        Self {
            node_id,
            parent_scope_id,
            definitions: HashMap::new(),
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        let symbol = definition.identifier().unparse();
        let node_id = definition.node_id();
        self.definitions.insert(symbol, node_id);
    }
}

pub(crate) struct YulFunctionScope {
    pub node_id: NodeId,
    pub enclosing_scope_id: ScopeId,
    pub definitions: HashMap<String, NodeId>,
}

impl YulFunctionScope {
    fn new(node_id: NodeId, enclosing_scope_id: ScopeId) -> Self {
        Self {
            node_id,
            enclosing_scope_id,
            definitions: HashMap::new(),
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        let symbol = definition.identifier().unparse();
        let node_id = definition.node_id();
        self.definitions.insert(symbol, node_id);
    }
}

pub(crate) enum Scope {
    Block(BlockScope),
    Contract(ContractScope),
    Enum(EnumScope),
    File(FileScope),
    Function(FunctionScope),
    Modifier(ModifierScope),
    Parameters(ParametersScope),
    Struct(StructScope),
    Using(UsingScope),
    YulBlock(YulBlockScope),
    YulFunction(YulFunctionScope),
}

impl Scope {
    pub(crate) fn node_id(&self) -> NodeId {
        match self {
            Self::Block(block_scope) => block_scope.node_id,
            Self::Contract(contract_scope) => contract_scope.node_id,
            Self::Enum(enum_scope) => enum_scope.node_id,
            Self::File(file_scope) => file_scope.node_id,
            Self::Function(function_scope) => function_scope.node_id,
            Self::Modifier(modifier_scope) => modifier_scope.node_id,
            Self::Parameters(parameters_scope) => parameters_scope.node_id,
            Self::Struct(struct_scope) => struct_scope.node_id,
            Self::Using(using_scope) => using_scope.node_id,
            Self::YulBlock(yul_block_scope) => yul_block_scope.node_id,
            Self::YulFunction(yul_function_scope) => yul_function_scope.node_id,
        }
    }

    fn insert_definition(&mut self, definition: &Definition) {
        match self {
            Self::Block(block_scope) => block_scope.insert_definition(definition),
            Self::Contract(contract_scope) => contract_scope.insert_definition(definition),
            Self::Enum(enum_scope) => enum_scope.insert_definition(definition),
            Self::File(file_scope) => file_scope.insert_definition(definition),
            Self::Function(function_scope) => function_scope.insert_definition(definition),
            Self::Modifier(modifier_scope) => modifier_scope.insert_definition(definition),
            Self::Parameters(parameters_scope) => parameters_scope.insert_definition(definition),
            Self::Struct(struct_scope) => struct_scope.insert_definition(definition),
            Self::Using(_) => unreachable!("cannot insert a definition into a using clause scope"),
            Self::YulBlock(yul_block_scope) => yul_block_scope.insert_definition(definition),
            Self::YulFunction(function_scope) => function_scope.insert_definition(definition),
        }
    }

    pub(crate) fn get_using_directives(&self) -> Vec<&UsingDirective> {
        match self {
            Self::Contract(contract_scope) => contract_scope.using_directives.iter().collect(),
            Self::File(file_scope) => file_scope.using_directives.iter().collect(),
            _ => Vec::new(),
        }
    }

    pub(crate) fn new_block(node_id: NodeId, parent_scope_id: ScopeId) -> Self {
        Self::Block(BlockScope::new(node_id, parent_scope_id))
    }

    pub(crate) fn new_contract(node_id: NodeId, file_scope_id: ScopeId) -> Self {
        Self::Contract(ContractScope::new(node_id, file_scope_id))
    }

    pub(crate) fn new_enum(node_id: NodeId) -> Self {
        Self::Enum(EnumScope::new(node_id))
    }

    pub(crate) fn new_file(node_id: NodeId, file_id: &str) -> Self {
        Self::File(FileScope::new(node_id, file_id))
    }

    pub(crate) fn new_function(
        node_id: NodeId,
        parent_scope_id: ScopeId,
        parameters_scope_id: ScopeId,
    ) -> Self {
        Self::Function(FunctionScope::new(
            node_id,
            parent_scope_id,
            parameters_scope_id,
        ))
    }

    pub(crate) fn new_modifier(node_id: NodeId, parent_scope_id: ScopeId) -> Self {
        Self::Modifier(ModifierScope::new(node_id, parent_scope_id))
    }

    pub(crate) fn new_parameters(node_id: NodeId) -> Self {
        Self::Parameters(ParametersScope::new(node_id))
    }

    pub(crate) fn new_struct(node_id: NodeId) -> Self {
        Self::Struct(StructScope::new(node_id))
    }

    pub(crate) fn new_using(node_id: NodeId, symbols: HashMap<String, Vec<NodeId>>) -> Self {
        Self::Using(UsingScope::new(node_id, symbols))
    }

    pub(crate) fn new_yul_block(node_id: NodeId, parent_scope_id: ScopeId) -> Self {
        Self::YulBlock(YulBlockScope::new(node_id, parent_scope_id))
    }

    pub(crate) fn new_yul_function(node_id: NodeId, enclosing_scope_id: ScopeId) -> Self {
        Self::YulFunction(YulFunctionScope::new(node_id, enclosing_scope_id))
    }
}

//////////////////////////////////////////////////////////////////////////////
// Binder

#[derive(Clone, Debug)]
pub enum Typing {
    Unresolved,
    Resolved(TypeId),
    Undetermined(Vec<TypeId>),
    MetaType(NodeId),
    BuiltIn(BuiltIn),
    This,
    Super,
}

impl Typing {
    pub(crate) fn as_type_id(&self) -> Option<TypeId> {
        match self {
            Self::Resolved(type_id) => Some(*type_id),
            _ => None,
        }
    }
}

pub struct Binder {
    scopes: Vec<Scope>,
    scopes_by_node_id: HashMap<NodeId, ScopeId>,
    scopes_by_file_id: HashMap<String, ScopeId>,
    global_using_directives: Vec<UsingDirective>,

    // definitions indexed by definiens node
    pub definitions: HashMap<NodeId, Definition>,
    // mapping from definition identifier to definiens
    definitions_by_identifier: HashMap<NodeId, NodeId>,
    // references indexed by identifier node
    pub references: HashMap<NodeId, Reference>,

    node_typing: HashMap<NodeId, Typing>,
}

impl Binder {
    pub(crate) fn new() -> Self {
        Self {
            scopes: Vec::new(),
            scopes_by_node_id: HashMap::new(),
            scopes_by_file_id: HashMap::new(),
            global_using_directives: Vec::new(),
            definitions: HashMap::new(),
            definitions_by_identifier: HashMap::new(),
            references: HashMap::new(),
            node_typing: HashMap::new(),
        }
    }

    pub(crate) fn get_scope_by_id(&self, scope_id: ScopeId) -> &Scope {
        self.scopes.get(scope_id.0).unwrap()
    }

    pub(crate) fn get_scope_mut(&mut self, scope_id: ScopeId) -> &mut Scope {
        self.scopes.get_mut(scope_id.0).unwrap()
    }

    pub(crate) fn scope_id_for_node_id(&self, node_id: NodeId) -> Option<ScopeId> {
        self.scopes_by_node_id.get(&node_id).copied()
    }

    pub(crate) fn scope_id_for_file_id(&self, file_id: &str) -> Option<ScopeId> {
        self.scopes_by_file_id.get(file_id).copied()
    }

    pub(crate) fn insert_scope(&mut self, scope: Scope) -> ScopeId {
        let node_id = scope.node_id();
        let scope_id = ScopeId(self.scopes.len());
        if self.scopes_by_node_id.contains_key(&node_id) {
            unreachable!("attempt to insert duplicate file scope for node {node_id:?}");
        }
        if let Scope::File(ref file_scope) = scope {
            let file_id = &file_scope.file_id;
            if self.scopes_by_file_id.contains_key(file_id) {
                unreachable!("attempt to insert duplicate file scope for {file_id}");
            }
            self.scopes_by_file_id.insert(file_id.clone(), scope_id);
        }
        self.scopes_by_node_id.insert(node_id, scope_id);
        self.scopes.push(scope);
        scope_id
    }

    pub(crate) fn insert_definition_no_scope(&mut self, definition: Definition) {
        let node_id = definition.node_id();
        if self.definitions.contains_key(&node_id) {
            unreachable!("attempt to insert duplicate definition on node {node_id:?}");
        }
        self.definitions_by_identifier
            .insert(definition.identifier().id(), node_id);
        self.definitions.insert(node_id, definition);
    }

    pub(crate) fn insert_definition_in_scope(&mut self, definition: Definition, scope_id: ScopeId) {
        let scope = self.get_scope_mut(scope_id);
        scope.insert_definition(&definition);
        self.insert_definition_no_scope(definition);
    }

    pub fn find_definition_by_id(&self, node_id: NodeId) -> Option<&Definition> {
        self.definitions.get(&node_id)
    }

    pub fn find_definition_by_identifier_node_id(&self, node_id: NodeId) -> Option<&Definition> {
        self.definitions_by_identifier
            .get(&node_id)
            .and_then(|definition_id| self.definitions.get(definition_id))
    }

    pub(crate) fn get_definition_mut(&mut self, node_id: NodeId) -> &mut Definition {
        self.definitions.get_mut(&node_id).unwrap()
    }

    pub(crate) fn insert_reference(&mut self, reference: Reference) {
        let node_id = reference.node_id();
        self.references.insert(node_id, reference);
    }

    pub fn find_reference_by_identifier_node_id(&self, node_id: NodeId) -> Option<&Reference> {
        self.references.get(&node_id)
    }

    pub(crate) fn insert_using_directive_in_scope(
        &mut self,
        directive: UsingDirective,
        scope_id: ScopeId,
    ) {
        let scope = self.get_scope_mut(scope_id);
        match scope {
            Scope::Contract(contract_scope) => contract_scope.using_directives.push(directive),
            Scope::File(file_scope) => file_scope.using_directives.push(directive),
            _ => unreachable!("cannot insert a using directive in scope {scope_id:?}"),
        }
    }

    pub(crate) fn insert_global_using_directive(&mut self, directive: UsingDirective) {
        self.global_using_directives.push(directive);
    }

    pub(crate) fn get_global_using_directives(&self) -> Vec<&UsingDirective> {
        self.global_using_directives.iter().collect()
    }

    pub fn node_typing(&self, node_id: NodeId) -> Typing {
        self.node_typing
            .get(&node_id)
            .cloned()
            .expect("expected node to have typing information")
    }

    pub(crate) fn mark_meta_type_node(&mut self, node_id: NodeId) {
        self.node_typing.insert(node_id, Typing::MetaType(node_id));
    }

    pub(crate) fn set_node_type(&mut self, node_id: NodeId, type_id: Option<TypeId>) {
        let typing = if let Some(type_id) = type_id {
            Typing::Resolved(type_id)
        } else {
            Typing::Unresolved
        };
        self.set_node_typing(node_id, typing);
    }

    pub(crate) fn set_node_typing(&mut self, node_id: NodeId, typing: Typing) {
        let previous_typing = self.node_typing.insert(node_id, typing);
        if previous_typing.is_some() {
            unreachable!("typing information for node {node_id:?} already set");
        }
    }

    // File scope resolution context

    fn get_file_scope(&self, file_id: &str) -> &FileScope {
        self.scopes_by_file_id
            .get(file_id)
            .and_then(|scope_id| self.scopes.get(scope_id.0))
            .and_then(|scope| match scope {
                Scope::File(file_scope) => Some(file_scope),
                _ => None,
            })
            .unwrap()
    }

    // Resolving a symbol in a file scope is special because of default imports.
    // We want to find *all* definitions with the given symbol reachable from
    // the file.
    fn resolve_in_file_scope(&self, file_id: &str, symbol: &str) -> Resolution {
        let mut found_definitions = Vec::new();
        let mut visited_files = HashSet::new();
        let mut files_to_search = VecDeque::new();
        files_to_search.push_back(file_id.to_owned());

        while let Some(file_id) = files_to_search.pop_front() {
            let file_scope = self.get_file_scope(&file_id);
            if !visited_files.insert(file_id) {
                continue;
            }

            found_definitions.extend(file_scope.lookup_symbol(symbol));
            files_to_search.extend(file_scope.imported_files.iter().cloned());
        }

        Resolution::from(found_definitions)
    }

    fn resolve_in_scope_internal(&self, scope_id: ScopeId, symbol: &str) -> Resolution {
        let scope = self.get_scope_by_id(scope_id);
        match scope {
            Scope::Block(block_scope) => block_scope.definitions.get(symbol).copied().map_or_else(
                || self.resolve_in_scope_internal(block_scope.parent_scope_id, symbol),
                Resolution::Definition,
            ),
            Scope::Contract(contract_scope) => {
                contract_scope.definitions.get(symbol).cloned().map_or_else(
                    || self.resolve_in_scope_internal(contract_scope.file_scope_id, symbol),
                    Resolution::from,
                )
            }
            Scope::Enum(enum_scope) => enum_scope.definitions.get(symbol).into(),
            Scope::File(file_scope) => self.resolve_in_file_scope(&file_scope.file_id, symbol),
            Scope::Function(function_scope) => function_scope
                .definitions
                .get(symbol)
                .copied()
                .map_or_else(
                    || self.resolve_in_scope_internal(function_scope.parameters_scope_id, symbol),
                    Resolution::Definition,
                )
                .or_else(|| self.resolve_in_scope_internal(function_scope.parent_scope_id, symbol)),
            Scope::Modifier(modifier_scope) => {
                if symbol == "_" {
                    Resolution::BuiltIn(BuiltIn::ModifierUnderscore)
                } else {
                    modifier_scope.definitions.get(symbol).copied().map_or_else(
                        || self.resolve_in_scope_internal(modifier_scope.parent_scope_id, symbol),
                        Resolution::Definition,
                    )
                }
            }
            Scope::Parameters(parameters_scope) => parameters_scope.definitions.get(symbol).into(),
            Scope::Struct(struct_scope) => struct_scope.definitions.get(symbol).into(),
            Scope::Using(using_scope) => using_scope
                .symbols
                .get(symbol)
                .cloned()
                .map_or(Resolution::Unresolved, Resolution::from),
            Scope::YulBlock(yul_block_scope) => yul_block_scope
                .definitions
                .get(symbol)
                .copied()
                .map_or_else(
                    || self.resolve_in_scope_internal(yul_block_scope.parent_scope_id, symbol),
                    Resolution::Definition,
                ),
            Scope::YulFunction(yul_function_scope) => yul_function_scope
                .definitions
                .get(symbol)
                .copied()
                .map_or_else(
                    || {
                        self.resolve_in_scope_internal(
                            yul_function_scope.enclosing_scope_id,
                            symbol,
                        )
                    },
                    Resolution::Definition,
                ),
        }
    }

    // This will attempt to lexically resolve `symbol` starting from the given
    // scope. This means that scopes can delegate to their "parent" scopes if
    // the symbol is not found there, and also that imported symbols are
    // followed recursively.
    pub(crate) fn resolve_in_scope(&self, scope_id: ScopeId, symbol: &str) -> Resolution {
        let mut found_ids = Vec::new();
        let mut working_set = Vec::new();
        let mut seen_ids = HashSet::new();

        let initial_resolution = self.resolve_in_scope_internal(scope_id, symbol);
        match initial_resolution {
            Resolution::Unresolved | Resolution::BuiltIn(_) => return initial_resolution,
            _ => {}
        }

        working_set.extend(initial_resolution.get_definition_ids().iter().rev());
        while let Some(definition_id) = working_set.pop() {
            if !seen_ids.insert(definition_id) {
                // we already processed this definition
                continue;
            }
            let Some(definition) = self.find_definition_by_id(definition_id) else {
                unreachable!("Definition {definition_id:?} does not exist");
            };
            if let Definition::ImportedSymbol(imported_symbol) = definition {
                // follow the imported symbol and expand the working set
                let Some(scope_id) = imported_symbol
                    .resolved_file_id
                    .as_ref()
                    .and_then(|file_id| self.scope_id_for_file_id(file_id))
                else {
                    continue;
                };
                working_set.extend(
                    self.resolve_in_scope_internal(scope_id, &imported_symbol.symbol)
                        .get_definition_ids()
                        .iter()
                        .rev(),
                );
            } else {
                found_ids.push(definition_id);
            }
        }

        Resolution::from(found_ids)
    }

    // Resolution in this function is strictly limited to the given scope, and
    // will only work in scope types that can work as a namespace only (eg.
    // block scopes and other lexical scopes don't)
    pub(crate) fn resolve_in_scope_as_namespace(
        &self,
        scope_id: ScopeId,
        symbol: &str,
    ) -> Resolution {
        let scope = self.get_scope_by_id(scope_id);
        match scope {
            Scope::Contract(contract_scope) => contract_scope
                .definitions
                .get(symbol)
                .cloned()
                .map_or(Resolution::Unresolved, Resolution::from),
            Scope::Enum(enum_scope) => enum_scope.definitions.get(symbol).into(),
            Scope::Struct(struct_scope) => struct_scope.definitions.get(symbol).into(),
            Scope::Using(using_scope) => using_scope
                .symbols
                .get(symbol)
                .cloned()
                .map_or(Resolution::Unresolved, Resolution::from),
            Scope::Parameters(parameters_scope) => parameters_scope.definitions.get(symbol).into(),

            Scope::Block(_)
            | Scope::File(_)
            | Scope::Function(_)
            | Scope::Modifier(_)
            | Scope::YulBlock(_)
            | Scope::YulFunction(_) => Resolution::Unresolved,
        }
    }

    pub(crate) fn get_parameters_scope_for_definition(
        &self,
        definition_id: NodeId,
    ) -> Option<ScopeId> {
        match self.find_definition_by_id(definition_id)? {
            Definition::Contract(contract_definition) => {
                contract_definition.constructor_parameters_scope_id
            }
            Definition::Error(error_definition) => Some(error_definition.parameters_scope_id),
            Definition::Event(event_definition) => Some(event_definition.parameters_scope_id),
            Definition::Function(function_definition) => {
                Some(function_definition.parameters_scope_id)
            }
            Definition::Struct(_) => self.scope_id_for_node_id(definition_id),
            _ => None,
        }
    }
}
