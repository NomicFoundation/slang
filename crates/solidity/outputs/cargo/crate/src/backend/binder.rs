use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use metaslang_cst::nodes::NodeId;

use crate::cst::TerminalNode;

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
pub enum Definition {
    Contract(ContractDefinition),
    Library(LibraryDefinition),
    Interface(InterfaceDefinition),
    Import(ImportDefinition),
}

impl Definition {
    pub(crate) fn node_id(&self) -> NodeId {
        match self {
            Definition::Contract(contract_definition) => contract_definition.node_id,
            Definition::Library(library_definition) => library_definition.node_id,
            Definition::Interface(interface_definition) => interface_definition.node_id,
            Definition::Import(import_definition) => import_definition.node_id,
        }
    }

    pub(crate) fn identifier(&self) -> &Rc<TerminalNode> {
        match self {
            Definition::Contract(contract_definition) => &contract_definition.identifier,
            Definition::Library(library_definition) => &library_definition.identifier,
            Definition::Interface(interface_definition) => &interface_definition.identifier,
            Definition::Import(import_definition) => &import_definition.identifier,
        }
    }
}

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

pub struct FileScope {
    pub definitions: HashMap<String, NodeId>,
    pub imported_files: HashSet<String>,
}

impl FileScope {
    fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            imported_files: HashSet::new(),
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        let symbol = definition.identifier().unparse();
        let node_id = definition.node_id();
        self.definitions.insert(symbol, node_id);
    }

    pub(crate) fn add_imported_file(&mut self, file_id: String) {
        self.imported_files.insert(file_id);
    }

    pub fn resolve_symbol(&self, symbol: &str) -> Option<NodeId> {
        self.definitions.get(symbol).copied()
    }
}

pub struct Binder {
    pub file_scopes: HashMap<String, FileScope>,
    pub definitions: HashMap<NodeId, Definition>,
    pub references: HashMap<NodeId, Reference>,
}

impl Binder {
    pub(crate) fn new() -> Self {
        Self {
            file_scopes: HashMap::new(),
            definitions: HashMap::new(),
            references: HashMap::new(),
        }
    }

    pub(crate) fn get_file_scope(&self, file_id: &str) -> &FileScope {
        self.file_scopes.get(file_id).unwrap()
    }

    pub(crate) fn get_file_scope_mut(&mut self, file_id: &str) -> &mut FileScope {
        self.file_scopes.get_mut(file_id).unwrap()
    }

    pub(crate) fn insert_file_scope(&mut self, file_id: &str) {
        if self.file_scopes.contains_key(file_id) {
            unreachable!("attempt to insert duplicate file scope for {file_id}");
        }
        let file_scope = FileScope::new();
        self.file_scopes.insert(file_id.to_string(), file_scope);
    }

    pub(crate) fn insert_definition(&mut self, definition: Definition) {
        let node_id = definition.node_id();
        if self.definitions.contains_key(&node_id) {
            unreachable!("attempt to insert duplicate definition on node {node_id:?}");
        }
        self.definitions.insert(node_id, definition);
    }

    pub(crate) fn insert_reference(&mut self, reference: Reference) {
        let node_id = reference.node_id();
        self.references.insert(node_id, reference);
    }

    pub(crate) fn resolve_in_file_scope(&self, file_id: &str, symbol: &str) -> Option<NodeId> {
        let mut visited_files = HashSet::new();
        let mut files_to_search = VecDeque::new();
        files_to_search.push_back(file_id.to_owned());

        while let Some(file_id) = files_to_search.pop_front() {
            let file_scope = self.get_file_scope(&file_id);
            if !visited_files.insert(file_id) {
                continue;
            }

            if let Some(definition) = file_scope.resolve_symbol(symbol) {
                return Some(definition);
            }
            files_to_search.extend(file_scope.imported_files.iter().cloned());
        }
        None
    }
}
