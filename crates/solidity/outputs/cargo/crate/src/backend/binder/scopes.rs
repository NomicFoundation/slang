use std::collections::{HashMap, HashSet};

use super::definitions::Definition;
use super::ScopeId;
use crate::backend::types::TypeId;
use crate::cst::{NodeId, TerminalKind};

//////////////////////////////////////////////////////////////////////////////
// Scopes - types

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

pub(crate) struct BlockScope {
    pub node_id: NodeId,
    pub parent_scope_id: ScopeId,
    pub definitions: HashMap<String, NodeId>,
}

pub(crate) struct ContractScope {
    pub node_id: NodeId,
    pub file_scope_id: ScopeId,
    pub definitions: HashMap<String, Vec<NodeId>>,
    pub using_directives: Vec<UsingDirective>,
}

pub(crate) struct EnumScope {
    pub node_id: NodeId,
    pub definitions: HashMap<String, NodeId>,
}

pub(crate) struct FileScope {
    pub node_id: NodeId,
    pub file_id: String,
    pub definitions: HashMap<String, Vec<NodeId>>,
    pub imported_files: HashSet<String>,
    pub using_directives: Vec<UsingDirective>,
}

pub(crate) struct FunctionScope {
    pub node_id: NodeId,
    pub parent_scope_id: ScopeId,
    pub parameters_scope_id: ScopeId,
    pub definitions: HashMap<String, NodeId>,
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

pub(crate) struct ParametersScope {
    pub node_id: NodeId,
    pub definitions: HashMap<String, NodeId>,
}

pub(crate) struct StructScope {
    pub node_id: NodeId,
    pub definitions: HashMap<String, NodeId>,
}

pub(crate) struct UsingScope {
    pub node_id: NodeId,
    pub symbols: HashMap<String, Vec<NodeId>>,
}

pub(crate) struct YulBlockScope {
    pub node_id: NodeId,
    pub parent_scope_id: ScopeId,
    pub definitions: HashMap<String, NodeId>,
}

pub(crate) struct YulFunctionScope {
    pub node_id: NodeId,
    pub parent_scope_id: ScopeId,
    pub definitions: HashMap<String, NodeId>,
}

//////////////////////////////////////////////////////////////////////////////
// Scopes - implementations

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

    pub(super) fn insert_definition(&mut self, definition: &Definition) {
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

    pub(super) fn lookup_symbol(&self, symbol: &str) -> Vec<NodeId> {
        self.definitions.get(symbol).cloned().unwrap_or(Vec::new())
    }
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

impl UsingScope {
    fn new(node_id: NodeId, symbols: HashMap<String, Vec<NodeId>>) -> Self {
        Self { node_id, symbols }
    }
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

impl YulFunctionScope {
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

//////////////////////////////////////////////////////////////////////////////
// Using directives

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
