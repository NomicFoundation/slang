use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use metaslang_cst::nodes::NodeId;

use crate::cst::TerminalNode;

//////////////////////////////////////////////////////////////////////////////
// Definitions

#[derive(Debug)]
pub struct ContractDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct LibraryDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct InterfaceDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub struct ImportDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
    pub resolved_file_id: Option<String>,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

#[derive(Debug)]
pub enum Definition {
    Contract(ContractDefinition),
    Library(LibraryDefinition),
    Interface(InterfaceDefinition),
    Import(ImportDefinition),
    Function(FunctionDefinition),
}

impl Definition {
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Contract(contract_definition) => contract_definition.node_id,
            Self::Library(library_definition) => library_definition.node_id,
            Self::Interface(interface_definition) => interface_definition.node_id,
            Self::Import(import_definition) => import_definition.node_id,
            Self::Function(function_definition) => function_definition.node_id,
        }
    }

    pub fn identifier(&self) -> &Rc<TerminalNode> {
        match self {
            Self::Contract(contract_definition) => &contract_definition.identifier,
            Self::Library(library_definition) => &library_definition.identifier,
            Self::Interface(interface_definition) => &interface_definition.identifier,
            Self::Import(import_definition) => &import_definition.identifier,
            Self::Function(function_definition) => &function_definition.identifier,
        }
    }

    pub fn new_contract(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Contract(ContractDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub fn new_library(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Library(LibraryDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub fn new_interface(node_id: NodeId, identifier: &Rc<TerminalNode>) -> Self {
        Self::Interface(InterfaceDefinition {
            node_id,
            identifier: Rc::clone(identifier),
        })
    }

    pub fn new_import(
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

pub struct ContractScope {
    pub node_id: NodeId,
    pub parent_scope_id: NodeId,
    pub definitions: HashMap<String, Vec<NodeId>>,
}

impl ContractScope {
    fn new(node_id: NodeId, parent_scope_id: NodeId) -> Self {
        Self {
            node_id,
            parent_scope_id,
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

pub enum Scope {
    File(FileScope),
    Contract(ContractScope),
}

impl Scope {
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::File(file_scope) => file_scope.node_id,
            Self::Contract(contract_scope) => contract_scope.node_id,
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        match self {
            Self::File(file_scope) => file_scope.insert_definition(definition),
            Self::Contract(contract_scope) => contract_scope.insert_definition(definition),
        }
    }

    pub fn new_file(node_id: NodeId, file_id: &str) -> Self {
        Self::File(FileScope::new(node_id, file_id))
    }

    pub fn new_contract(node_id: NodeId, parent_scope_id: NodeId) -> Self {
        Self::Contract(ContractScope::new(node_id, parent_scope_id))
    }
}

//////////////////////////////////////////////////////////////////////////////
// Binder

pub struct Binder {
    scopes: HashMap<NodeId, Scope>,
    file_scopes: HashMap<String, NodeId>,
    pub definitions: HashMap<NodeId, Definition>,
    definitions_by_identifier: HashMap<NodeId, NodeId>,
    pub references: HashMap<NodeId, Reference>,
}

impl Binder {
    pub(crate) fn new() -> Self {
        Self {
            scopes: HashMap::new(),
            file_scopes: HashMap::new(),
            definitions: HashMap::new(),
            definitions_by_identifier: HashMap::new(),
            references: HashMap::new(),
        }
    }

    pub(crate) fn get_scope_mut(&mut self, node_id: NodeId) -> &mut Scope {
        self.scopes.get_mut(&node_id).unwrap()
    }

    pub(crate) fn get_file_scope_mut(&mut self, file_id: &str) -> &mut FileScope {
        self.file_scopes
            .get(file_id)
            .and_then(|node_id| self.scopes.get_mut(node_id))
            .map(|scope| match scope {
                Scope::File(file_scope) => file_scope,
                Scope::Contract(_) => unreachable!("scope in file_scopes is not a file scope"),
            })
            .unwrap()
    }

    pub(crate) fn insert_scope(&mut self, scope: Scope) {
        let node_id = scope.node_id();
        if self.scopes.contains_key(&node_id) {
            unreachable!("attempt to insert duplicate file scope for node {node_id:?}");
        }
        match scope {
            Scope::File(ref file_scope) => {
                let file_id = &file_scope.file_id;
                if self.file_scopes.contains_key(file_id) {
                    unreachable!("attempt to insert duplicate file scope for {file_id}");
                }
                self.file_scopes.insert(file_id.clone(), node_id);
            }
            Scope::Contract(_) => {}
        };
        self.scopes.insert(node_id, scope);
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
        self.file_scopes
            .get(file_id)
            .and_then(|node_id| self.scopes.get(node_id))
            .and_then(|scope| match scope {
                Scope::File(file_scope) => Some(file_scope),
                Scope::Contract(_) => None,
            })
            .unwrap()
    }

    pub(crate) fn resolve_single_in_file_scope(
        &self,
        file_id: &str,
        symbol: &str,
    ) -> Option<NodeId> {
        let mut visited_files = HashSet::new();
        let mut files_to_search = VecDeque::new();
        files_to_search.push_back(file_id.to_owned());

        while let Some(file_id) = files_to_search.pop_front() {
            let file_scope = self.get_file_scope(&file_id);
            if !visited_files.insert(file_id) {
                continue;
            }

            if let Some(definition) = file_scope.lookup_symbol(symbol).first() {
                return Some(*definition);
            }
            files_to_search.extend(file_scope.imported_files.iter().cloned());
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn resolve_multi_in_file_scope(&self, file_id: &str, symbol: &str) -> Vec<NodeId> {
        let mut result = Vec::new();
        let mut visited_files = HashSet::new();
        let mut files_to_search = VecDeque::new();
        files_to_search.push_back(file_id.to_owned());

        while let Some(file_id) = files_to_search.pop_front() {
            let file_scope = self.get_file_scope(&file_id);
            if !visited_files.insert(file_id) {
                continue;
            }

            let definitions = file_scope.lookup_symbol(symbol);
            result.extend(definitions.into_iter());
            files_to_search.extend(file_scope.imported_files.iter().cloned());
        }
        result
    }
}
