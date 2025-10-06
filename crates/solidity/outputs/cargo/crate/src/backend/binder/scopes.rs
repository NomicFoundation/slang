use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::definitions::Definition;
use super::ScopeId;
use crate::backend::types::TypeId;
use crate::cst::{NodeId, TerminalKind, TerminalNode};

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
    pub(crate) node_id: NodeId,
    pub(crate) parent_scope_id: ScopeId,
    pub(crate) definitions: HashMap<String, NodeId>,
}

pub(crate) struct ContractScope {
    pub(crate) node_id: NodeId,
    pub(crate) file_scope_id: ScopeId,
    pub(crate) definitions: HashMap<String, Vec<NodeId>>,
    pub(crate) using_directives: Vec<UsingDirective>,
}

pub(crate) struct EnumScope {
    pub(crate) node_id: NodeId,
    pub(crate) definitions: HashMap<String, NodeId>,
}

pub(crate) struct FileScope {
    pub(crate) node_id: NodeId,
    pub(crate) file_id: String,
    pub(crate) definitions: HashMap<String, Vec<NodeId>>,
    pub(crate) imported_files: HashSet<String>,
    pub(crate) using_directives: Vec<UsingDirective>,
}

pub(crate) struct FunctionScope {
    pub(crate) node_id: NodeId,
    pub(crate) parent_scope_id: ScopeId,
    pub(crate) parameters_scope_id: ScopeId,
    pub(crate) definitions: HashMap<String, NodeId>,
}

// TODO: this is similar to a function scope, but it doesn't have a separate
// parameters scope; it also should bind the special `_` symbol to the built-in.
// There are other functions that don't need a separate parameters scope (eg.
// receive/fallback/unnamed), but they don't need to bind `_`. Should we
// refactor? Or remove this and make the parameters optional in FunctionScope?
// Probably the latter as we can control resolution in the relevant pass.
pub(crate) struct ModifierScope {
    pub(crate) node_id: NodeId,
    pub(crate) parent_scope_id: ScopeId,
    pub(crate) definitions: HashMap<String, NodeId>,
}

/// This is stored in a vector in the `ParametersScope` below preserving order
/// for positional arguments and used by the binder, both to resolve named
/// arguments, and disambiguate overloads of functions and events.
pub(crate) struct ParameterDefinition {
    pub(crate) name: Option<String>,
    pub(crate) node_id: NodeId,
    /// The type of the parameter, or `None` if it's not yet computed or cannot
    /// be computed. For some parameter containers (eg. errors) it's never
    /// computed.
    pub(crate) type_id: Option<TypeId>,
}

pub(crate) struct ParametersScope {
    pub(crate) node_id: NodeId,
    pub(crate) parameters: Vec<ParameterDefinition>,
}

pub(crate) struct StructScope {
    pub(crate) node_id: NodeId,
    pub(crate) definitions: HashMap<String, NodeId>,
}

pub(crate) struct UsingScope {
    pub(crate) node_id: NodeId,
    pub(crate) symbols: HashMap<String, Vec<NodeId>>,
}

pub(crate) struct YulBlockScope {
    pub(crate) node_id: NodeId,
    pub(crate) parent_scope_id: ScopeId,
    pub(crate) definitions: HashMap<String, NodeId>,
}

pub(crate) struct YulFunctionScope {
    pub(crate) node_id: NodeId,
    pub(crate) parent_scope_id: ScopeId,
    pub(crate) definitions: HashMap<String, NodeId>,
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

    pub(crate) fn insert_definition(&mut self, definition: &Definition) {
        match self {
            Self::Block(block_scope) => block_scope.insert_definition(definition),
            Self::Contract(contract_scope) => contract_scope.insert_definition(definition),
            Self::Enum(enum_scope) => enum_scope.insert_definition(definition),
            Self::File(file_scope) => file_scope.insert_definition(definition),
            Self::Function(function_scope) => function_scope.insert_definition(definition),
            Self::Modifier(modifier_scope) => modifier_scope.insert_definition(definition),
            Self::Parameters(_) => {
                unreachable!("cannot insert a definition into parameters scope directly")
            }
            Self::Struct(struct_scope) => struct_scope.insert_definition(definition),
            Self::Using(_) => unreachable!("cannot insert a definition into a using clause scope"),
            Self::YulBlock(yul_block_scope) => yul_block_scope.insert_definition(definition),
            Self::YulFunction(function_scope) => function_scope.insert_definition(definition),
        }
    }

    pub(crate) fn get_using_directives(&self) -> impl Iterator<Item = &UsingDirective> {
        match self {
            Self::Contract(contract_scope) => {
                EitherIter::Left(contract_scope.using_directives.iter())
            }
            Self::File(file_scope) => EitherIter::Right(file_scope.using_directives.iter()),
            _ => EitherIter::Empty,
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

    pub(super) fn lookup_symbol<'a>(&'a self, symbol: &str) -> impl Iterator<Item = NodeId> + 'a {
        match self.definitions.get(symbol) {
            Some(defs) => OptionIter::Some(defs.iter().copied()),
            None => OptionIter::Empty,
        }
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
    pub(crate) fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
            parameters: Vec::new(),
        }
    }

    pub(crate) fn add_parameter(&mut self, identifier: Option<&Rc<TerminalNode>>, node_id: NodeId) {
        self.parameters.push(ParameterDefinition {
            name: identifier.map(|name| name.unparse()),
            node_id,
            type_id: None,
        });
    }

    pub(crate) fn set_parameter_types(&mut self, type_ids: &[Option<TypeId>]) {
        if type_ids.len() != self.parameters.len() {
            unreachable!("parameter count mismatch while setting types");
        }
        for (index, parameter) in self.parameters.iter_mut().enumerate() {
            parameter.type_id = type_ids[index];
        }
    }

    pub(crate) fn lookup_definition(&self, symbol: &str) -> Option<NodeId> {
        self.parameters
            .iter()
            .find(|parameter| parameter.name.as_ref().is_some_and(|name| name == symbol))
            .map(|parameter| parameter.node_id)
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

    pub(crate) fn applies_to(&self, filter_type_id: TypeId) -> bool {
        match self {
            Self::AllTypes { .. } => true,
            Self::SingleType { type_id, .. } | Self::SingleTypeOperator { type_id, .. } => {
                *type_id == filter_type_id
            }
        }
    }

    pub(crate) fn get_scope_id(&self) -> ScopeId {
        match self {
            UsingDirective::AllTypes { scope_id }
            | UsingDirective::SingleType { scope_id, .. }
            | UsingDirective::SingleTypeOperator { scope_id, .. } => *scope_id,
        }
    }
}

pub(crate) enum OptionIter<T: Iterator> {
    Some(T),
    Empty,
}

impl<T> Iterator for OptionIter<T>
where
    T: Iterator,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OptionIter::Some(iter) => iter.next(),
            OptionIter::Empty => None,
        }
    }
}

pub(crate) enum EitherIter<L: Iterator, R: Iterator> {
    Left(L),
    Right(R),
    Empty,
}

impl<L, R> Iterator for EitherIter<L, R>
where
    L: Iterator,
    R: Iterator<Item = L::Item>,
{
    type Item = L::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EitherIter::Left(iter) => iter.next(),
            EitherIter::Right(iter) => iter.next(),
            EitherIter::Empty => None,
        }
    }
}
