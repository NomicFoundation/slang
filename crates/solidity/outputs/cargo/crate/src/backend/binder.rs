use std::collections::HashMap;
use std::rc::Rc;

use metaslang_cst::nodes::NodeId;

use crate::cst::TerminalNode;

pub struct ContractDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

pub struct LibraryDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

pub struct InterfaceDefinition {
    pub node_id: NodeId,
    pub identifier: Rc<TerminalNode>,
}

pub enum Definition {
    Contract(ContractDefinition),
    Library(LibraryDefinition),
    Interface(InterfaceDefinition),
}

impl Definition {
    pub(crate) fn node_id(&self) -> NodeId {
        match self {
            Definition::Contract(contract_definition) => contract_definition.node_id,
            Definition::Library(library_definition) => library_definition.node_id,
            Definition::Interface(interface_definition) => interface_definition.node_id,
        }
    }

    pub(crate) fn identifier(&self) -> &Rc<TerminalNode> {
        match self {
            Definition::Contract(contract_definition) => &contract_definition.identifier,
            Definition::Library(library_definition) => &library_definition.identifier,
            Definition::Interface(interface_definition) => &interface_definition.identifier,
        }
    }
}

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
}

impl FileScope {
    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        let symbol = definition.identifier().unparse();
        let node_id = definition.node_id();
        self.definitions.insert(symbol, node_id);
    }

    pub(crate) fn resolve_symbol(&self, symbol: &str) -> Option<NodeId> {
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

    pub(crate) fn get_file_scope(&mut self, file_id: &str) -> &mut FileScope {
        self.file_scopes.get_mut(file_id).unwrap()
    }

    pub(crate) fn insert_file_scope(&mut self, file_id: &str) {
        if self.file_scopes.contains_key(file_id) {
            unreachable!("attempt to insert duplicate file scope for {file_id}");
        }
        let file_scope = FileScope {
            definitions: HashMap::new(),
        };
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
}
