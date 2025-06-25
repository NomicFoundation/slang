use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use metaslang_cst::nodes::NodeId;

use crate::cst::TerminalNode;

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
pub struct TypeParameterDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct UserDefinedValueTypeDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct VariableDefinition {
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
    TypeParameter(TypeParameterDefinition),
    UserDefinedValueType(UserDefinedValueTypeDefinition),
    Variable(VariableDefinition),
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
            Self::TypeParameter(parameter_definition) => parameter_definition.node_id,
            Self::UserDefinedValueType(udvt_definition) => udvt_definition.node_id,
            Self::Variable(variable_definition) => variable_definition.node_id,
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
            Self::TypeParameter(parameter_definition) => &parameter_definition.identifier,
            Self::UserDefinedValueType(udvt_definition) => &udvt_definition.identifier,
            Self::Variable(variable_definition) => &variable_definition.identifier,
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
        })
    }

    pub(crate) fn new_variable(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Variable(VariableDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }
}

//////////////////////////////////////////////////////////////////////////////
// References

#[derive(Debug)]
pub struct Reference {
    pub identifier: Rc<TerminalNode>,
    pub definition_id: Option<NodeId>,
}

impl Reference {
    fn node_id(&self) -> NodeId {
        self.identifier.id()
    }
}

//////////////////////////////////////////////////////////////////////////////
// Scopes

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScopeId(usize);

pub struct ContractScope {
    pub node_id: NodeId,
    pub file_scope_id: ScopeId,
    pub definitions: HashMap<String, Vec<NodeId>>,
}

impl ContractScope {
    fn new(node_id: NodeId, file_scope_id: ScopeId) -> Self {
        Self {
            node_id,
            file_scope_id,
            definitions: HashMap::new(),
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

pub struct EnumScope {
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

pub struct FileScope {
    pub node_id: NodeId,
    pub file_id: String,
    pub definitions: HashMap<String, Vec<NodeId>>,
    pub imported_files: HashSet<String>,
}

impl FileScope {
    fn new(node_id: NodeId, file_id: &str) -> Self {
        Self {
            node_id,
            file_id: file_id.to_string(),
            definitions: HashMap::new(),
            imported_files: HashSet::new(),
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

pub struct FunctionScope {
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

pub struct ModifierScope {
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

pub struct ParametersScope {
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

pub enum Scope {
    Contract(ContractScope),
    Enum(EnumScope),
    File(FileScope),
    Function(FunctionScope),
    Modifier(ModifierScope),
    Parameters(ParametersScope),
}

impl Scope {
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Contract(contract_scope) => contract_scope.node_id,
            Self::Enum(enum_scope) => enum_scope.node_id,
            Self::File(file_scope) => file_scope.node_id,
            Self::Function(function_scope) => function_scope.node_id,
            Self::Modifier(modifier_scope) => modifier_scope.node_id,
            Self::Parameters(parameters_scope) => parameters_scope.node_id,
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        match self {
            Self::Contract(contract_scope) => contract_scope.insert_definition(definition),
            Self::Enum(enum_scope) => enum_scope.insert_definition(definition),
            Self::File(file_scope) => file_scope.insert_definition(definition),
            Self::Function(function_scope) => function_scope.insert_definition(definition),
            Self::Modifier(modifier_scope) => modifier_scope.insert_definition(definition),
            Self::Parameters(parameters_scope) => parameters_scope.insert_definition(definition),
        }
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
}

//////////////////////////////////////////////////////////////////////////////
// Binder

pub struct Binder {
    scopes: Vec<Scope>,
    scopes_by_node_id: HashMap<NodeId, ScopeId>,
    scopes_by_file_id: HashMap<String, ScopeId>,

    // definitions indexed by definiens node
    pub definitions: HashMap<NodeId, Definition>,
    // mapping from definition identifier to definiens
    definitions_by_identifier: HashMap<NodeId, NodeId>,
    // references indexed by identifier node
    pub references: HashMap<NodeId, Reference>,
}

impl Binder {
    pub(crate) fn new() -> Self {
        Self {
            scopes: Vec::new(),
            scopes_by_node_id: HashMap::new(),
            scopes_by_file_id: HashMap::new(),
            definitions: HashMap::new(),
            definitions_by_identifier: HashMap::new(),
            references: HashMap::new(),
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

    pub(crate) fn insert_definition(&mut self, definition: Definition) {
        let node_id = definition.node_id();
        if self.definitions.contains_key(&node_id) {
            unreachable!("attempt to insert duplicate definition on node {node_id:?}");
        }
        self.definitions_by_identifier
            .insert(definition.identifier().id(), node_id);
        self.definitions.insert(node_id, definition);
    }

    pub fn find_definition_by_id(&self, node_id: NodeId) -> Option<&Definition> {
        self.definitions.get(&node_id)
    }

    pub fn find_definition_by_identifier_node_id(&self, node_id: NodeId) -> Option<&Definition> {
        self.definitions_by_identifier
            .get(&node_id)
            .and_then(|definition_id| self.definitions.get(definition_id))
    }

    pub(crate) fn insert_reference(&mut self, reference: Reference) {
        let node_id = reference.node_id();
        self.references.insert(node_id, reference);
    }

    pub fn find_reference_by_identifier_node_id(&self, node_id: NodeId) -> Option<&Reference> {
        self.references.get(&node_id)
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

    // resolving a symbol in a file scope is special because of default imports
    fn resolve_single_in_file_scope(&self, file_id: &str, symbol: &str) -> Option<NodeId> {
        let mut visited_files = HashSet::new();
        let mut files_to_search = VecDeque::new();
        files_to_search.push_back(file_id.to_owned());

        while let Some(file_id) = files_to_search.pop_front() {
            let file_scope = self.get_file_scope(&file_id);
            if !visited_files.insert(file_id) {
                continue;
            }

            // TODO: should we verify that there is a single definition for the symbol?
            if let Some(definition) = file_scope.lookup_symbol(symbol).first() {
                return Some(*definition);
            }
            files_to_search.extend(file_scope.imported_files.iter().cloned());
        }
        None
    }

    pub(crate) fn resolve_single_in_scope(
        &self,
        scope_id: ScopeId,
        symbol: &str,
    ) -> Option<NodeId> {
        let scope = self.get_scope_by_id(scope_id);
        match scope {
            Scope::Contract(contract_scope) => contract_scope
                .definitions
                .get(symbol)
                .and_then(|definitions| definitions.first().copied())
                .or_else(|| self.resolve_single_in_scope(contract_scope.file_scope_id, symbol)),
            Scope::Enum(enum_scope) => enum_scope.definitions.get(symbol).copied(),
            Scope::File(file_scope) => {
                self.resolve_single_in_file_scope(&file_scope.file_id, symbol)
            }
            Scope::Function(function_scope) => function_scope
                .definitions
                .get(symbol)
                .copied()
                .or_else(|| {
                    self.resolve_single_in_scope(function_scope.parameters_scope_id, symbol)
                })
                .or_else(|| self.resolve_single_in_scope(function_scope.parent_scope_id, symbol)),
            Scope::Modifier(modifier_scope) => {
                modifier_scope.definitions.get(symbol).copied().or_else(|| {
                    self.resolve_single_in_scope(modifier_scope.parent_scope_id, symbol)
                })
            }
            Scope::Parameters(parameters_scope) => {
                parameters_scope.definitions.get(symbol).copied()
            }
        }
    }
}
