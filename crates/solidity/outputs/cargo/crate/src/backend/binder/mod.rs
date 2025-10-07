use std::collections::{HashMap, HashSet, VecDeque};

use super::built_ins::BuiltIn;
use super::types::{Type, TypeId};
use crate::cst::NodeId;

mod definitions;
mod references;
mod scopes;

pub use definitions::Definition;
pub(crate) use definitions::{
    ContractDefinition, FunctionVisibility, ImportDefinition, InterfaceDefinition,
    LibraryDefinition, StateVariableVisibility,
};
pub use references::{Reference, Resolution};
use scopes::ContractScope;
pub(crate) use scopes::{
    EitherIter, FileScope, ParameterDefinition, ParametersScope, Scope, UsingDirective,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(usize);

//////////////////////////////////////////////////////////////////////////////
// Binder

// TODO: do we want to move tuples (which are not a real Solidity type) as a
// typing variant? That has the additional benefit of not requiring us to
// register every tuple combination.

/// `Typing` represents typing information related to an AST node. If the node
/// happens to be a reference identifier, there is a direct relationship between
/// its `Resolution` and the `Typing`. Otherwise the typing is computed from the
/// node kind and its contents.

#[derive(Clone, Debug)]
pub enum Typing {
    /// The type of the node cannot be resolved at this time (this normally
    /// propagates upwards).
    Unresolved,
    /// Resolved with a corresponding `TypeId` valid in the `TypeRegistry`.
    Resolved(TypeId),
    /// There are multiple possible types that result from the typing of an
    /// `Ambiguous` resolution. Function call overload resolution will select
    /// the appropriate type by matching the types of the arguments and work
    /// backwards to the reference (if any) and fixup the selected definition.
    Undetermined(Vec<TypeId>),
    /// A node which refers to a user defined type by name, eg. the identifier
    /// of a contract. This is not a type that can be translated to the EVM, and
    /// as such is a way to suspend the typing until more information on how
    /// it's used is gathered. Eg. to create a new contract via the new
    /// operator, or construct a struct by using the struct's name in a function
    /// call.
    UserMetaType(NodeId),
    /// Similar to `UserMetaType` above, but refers to the meta type of an
    /// elementary type. A typical use case is explicit casting, which parses as
    /// a function call.
    MetaType(Type),
    /// Used to type the `BuiltIn` resolution when the result is not yet an
    /// actual type representable in the EVM. Eg. the built-in function `addmod`
    /// resolves to `Resolution::BuiltIn(Addmod)` and will type to
    /// `Typing::BuiltIn(Addmod)` which in turn will type to the `uint256` type
    /// when evaluated in the context of a function call.
    BuiltIn(BuiltIn),
    /// A special typing that's used as the result of applying the `new`
    /// operator, and will type to a contract reference when evaluated as a
    /// function call.
    NewExpression(TypeId),
    /// Typing of the `this` keyword. Resolving a member of `this` requires
    /// special lookup rules and it can also type to an address-type equivalent
    /// if used as a value.
    This,
    /// Typing of the `super` keyword. Resolving members requires special lookup
    /// rules.
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

/// The `Binder` contains binding information for all definitions and references
/// in the `CompilationUnit`, typing information for relevant AST nodes, as well
/// as other semantic information (such as scopes and linearisations).
pub struct Binder {
    /// Index of `Scope` objects. The `ScopeId` is an opaque index into this vector
    scopes: Vec<Scope>,
    /// Scopes (set of definitions contained and relationship to other scopes), indexed by `NodeId`
    scopes_by_node_id: HashMap<NodeId, ScopeId>,
    /// Index of root `FileScope`s for all files in the `CompilationUnit`
    scopes_by_file_id: HashMap<String, ScopeId>,
    /// `UsingDirective` objects registered globally (contract level directives
    /// are stored in the corresponding `ContractScope`)
    global_using_directives: Vec<UsingDirective>,
    /// `Definition` objects (identifier + definiens node), indexed by definiens `NodeId`
    definitions: HashMap<NodeId, Definition>,
    /// Definitions indexed by the `NodeId` of the identifier that names them
    definitions_by_identifier: HashMap<NodeId, NodeId>,
    /// `Reference` objects (identifier + `Resolution`), indexed by identifier `NodeId`
    references: HashMap<NodeId, Reference>,
    /// `Typing` information for each relevant `NodeId` of the AST
    node_typing: HashMap<NodeId, Typing>,
    /// Linearisations, as a vector of definitions, indexed by the
    /// contract/interface definition's `NodeId`
    linearisations: HashMap<NodeId, Vec<NodeId>>,
}

/// This controls visibility filtering and how to use the linearisation when
/// performing a contract scope resolution.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum ResolveOptions {
    /// Use all the linearised bases in order for an internal lookup.
    Internal,
    /// Use all the linearised bases for an external lookup.
    External,
    /// Use all bases and only excludes private members in bases types (used for
    /// general qualified member access).
    Qualified,
    /// Starts at `node_id` and proceeds in order for an external lookup.
    This(NodeId),
    /// Starts at the contract _following_ `node_id` (ie. its parent) for an
    /// internal lookup.
    Super(NodeId),
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
            linearisations: HashMap::new(),
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
        let scope_id = ScopeId(self.scopes.len());

        if let Scope::File(file_scope) = &scope {
            let file_id = &file_scope.file_id;
            if self.scopes_by_file_id.contains_key(file_id) {
                unreachable!("attempt to insert duplicate file scope for {file_id}");
            }
            self.scopes_by_file_id.insert(file_id.clone(), scope_id);
        }

        if let Scope::Parameters(_) = &scope {
            // parameters scope don't have an associated node ID
        } else {
            let node_id = scope.node_id();
            if self.scopes_by_node_id.contains_key(&node_id) {
                unreachable!("attempt to insert duplicate file scope for node {node_id:?}");
            }
            self.scopes_by_node_id.insert(node_id, scope_id);
        }

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

    pub(crate) fn insert_linearised_bases(
        &mut self,
        node_id: NodeId,
        linearised_bases: Vec<NodeId>,
    ) {
        if self
            .linearisations
            .insert(node_id, linearised_bases)
            .is_some()
        {
            unreachable!("Trying to linearised twice {node_id:?}");
        }
    }

    #[cfg(feature = "__private_testing_utils")]
    pub(crate) fn get_linearised_bases(&self, node_id: NodeId) -> Option<&Vec<NodeId>> {
        self.linearisations.get(&node_id)
    }

    #[cfg(feature = "__private_testing_utils")]
    pub fn linearisations(&self) -> &HashMap<NodeId, Vec<NodeId>> {
        &self.linearisations
    }

    pub fn definitions(&self) -> &HashMap<NodeId, Definition> {
        &self.definitions
    }

    pub(crate) fn insert_reference(&mut self, reference: Reference) {
        let node_id = reference.node_id();
        self.references.insert(node_id, reference);
    }

    pub(crate) fn fixup_reference(&mut self, node_id: NodeId, resolution: Resolution) {
        let reference = self.references.get_mut(&node_id).unwrap();
        reference.resolution = resolution;
    }

    pub fn find_reference_by_identifier_node_id(&self, node_id: NodeId) -> Option<&Reference> {
        self.references.get(&node_id)
    }

    pub fn references(&self) -> &HashMap<NodeId, Reference> {
        &self.references
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

    pub(crate) fn get_global_using_directives(&self) -> impl Iterator<Item = &UsingDirective> {
        self.global_using_directives.iter()
    }

    pub(crate) fn get_using_directives_in_scope(
        &self,
        scope_id: ScopeId,
    ) -> impl Iterator<Item = &UsingDirective> {
        self.get_scope_by_id(scope_id).get_using_directives()
    }

    pub(crate) fn get_using_directives_in_scope_including_inherited(
        &self,
        scope_id: ScopeId,
    ) -> impl Iterator<Item = &UsingDirective> {
        let scope = self.get_scope_by_id(scope_id);
        if let Scope::Contract(contract_scope) = scope {
            if let Some(linearisations) = self.linearisations.get(&contract_scope.node_id) {
                return EitherIter::Left(
                    linearisations
                        .iter()
                        .filter_map(|node_id| self.scope_id_for_node_id(*node_id))
                        .flat_map(|scope_id| self.get_scope_by_id(scope_id).get_using_directives()),
                );
            }
        }
        EitherIter::Right(scope.get_using_directives())
    }

    pub fn node_typing(&self, node_id: NodeId) -> Typing {
        self.node_typing
            .get(&node_id)
            .cloned()
            .expect("expected node to have typing information")
    }

    pub(crate) fn mark_user_meta_type_node(&mut self, node_id: NodeId) {
        self.node_typing
            .insert(node_id, Typing::UserMetaType(node_id));
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

    pub(crate) fn fixup_node_typing(&mut self, node_id: NodeId, typing: Typing) {
        assert!(
            self.node_typing.contains_key(&node_id),
            "typing information for node {node_id:?} not set"
        );
        self.node_typing.insert(node_id, typing);
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

    fn resolve_in_contract_scope_internal(
        &self,
        contract_scope: &ContractScope,
        symbol: &str,
        options: ResolveOptions,
    ) -> Resolution {
        // Search for the symbol in the linearised bases *in-order*.
        if let Some(linearisations) = self.linearisations.get(&contract_scope.node_id) {
            let linearisations = match options {
                ResolveOptions::Internal | ResolveOptions::External | ResolveOptions::Qualified => {
                    linearisations.as_slice()
                }
                ResolveOptions::This(node_id) => {
                    if let Some(index) = linearisations.iter().position(|id| *id == node_id) {
                        &linearisations[index..]
                    } else {
                        return Resolution::Unresolved;
                    }
                }
                ResolveOptions::Super(node_id) => {
                    if let Some(index) = linearisations.iter().position(|id| *id == node_id) {
                        &linearisations[index + 1..]
                    } else {
                        return Resolution::Unresolved;
                    }
                }
            };
            let mut results = Vec::new();
            for (index, node_id) in linearisations.iter().enumerate() {
                let Some(base_scope_id) = self.scope_id_for_node_id(*node_id) else {
                    continue;
                };
                let Scope::Contract(base_contract_scope) = self.get_scope_by_id(base_scope_id)
                else {
                    unreachable!("expected the scope of a base contract to be a ContractScope");
                };
                let Some(definitions) = base_contract_scope.definitions.get(symbol) else {
                    continue;
                };
                for definition_id in definitions {
                    let definition = &self.definitions[definition_id];
                    let visible = match options {
                        ResolveOptions::Internal => {
                            if index == 0 {
                                definition.is_private_or_internally_visible()
                            } else {
                                definition.is_internally_visible()
                            }
                        }
                        ResolveOptions::Qualified => {
                            index == 0
                                || definition.is_internally_visible()
                                || definition.is_externally_visible()
                        }
                        ResolveOptions::This(_) | ResolveOptions::External => {
                            definition.is_externally_visible()
                        }
                        ResolveOptions::Super(_) => definition.is_internally_visible(),
                    };
                    if visible {
                        results.push(*definition_id);
                    }
                }
            }
            Resolution::from(results)
        } else if let Some(definitions) = contract_scope.definitions.get(symbol) {
            // This case shouldn't happen for valid Solidity, as all
            // contracts should have a proper linearisation
            Resolution::from(definitions.clone())
        } else {
            Resolution::Unresolved
        }
    }

    fn resolve_in_scope_internal(&self, scope_id: ScopeId, symbol: &str) -> Resolution {
        let scope = self.get_scope_by_id(scope_id);
        match scope {
            Scope::Block(block_scope) => block_scope.definitions.get(symbol).copied().map_or_else(
                || self.resolve_in_scope_internal(block_scope.parent_scope_id, symbol),
                Resolution::Definition,
            ),
            Scope::Contract(contract_scope) => {
                self.resolve_in_contract_scope_internal(
                    contract_scope,
                    symbol,
                    ResolveOptions::Internal,
                )
                .or_else(|| {
                    // Otherwise, delegate to the containing file scope.
                    self.resolve_in_scope_internal(contract_scope.file_scope_id, symbol)
                })
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
            Scope::Parameters(parameters_scope) => {
                parameters_scope.lookup_definition(symbol).into()
            }
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
            Scope::YulFunction(yul_function_scope) => {
                // TODO(validation): when delegating to the parent scope, we
                // cannot resolve to a Yul variable defined in a parent block
                yul_function_scope
                    .definitions
                    .get(symbol)
                    .copied()
                    .map_or_else(
                        || {
                            self.resolve_in_scope_internal(
                                yul_function_scope.parent_scope_id,
                                symbol,
                            )
                        },
                        Resolution::Definition,
                    )
            }
        }
    }

    // This will attempt to lexically resolve `symbol` starting from the given
    // scope. This means that scopes can delegate to their "parent" scopes if
    // the symbol is not found there, and also that imported symbols are
    // followed recursively. This function handles the latter (following
    // imported symbols recursively), while `resolve_in_scope_internal`
    // delegates to parent or otherwise linked scopes.
    pub(crate) fn resolve_in_scope(&self, scope_id: ScopeId, symbol: &str) -> Resolution {
        // TODO: since this function uses the results from other resolution
        // functions, we making more allocations than necessary; it may be worth
        // it to try and avoid them by returning iterators from the delegated
        // resolution functions
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
            Scope::Contract(contract_scope) => self.resolve_in_contract_scope_internal(
                contract_scope,
                symbol,
                ResolveOptions::Qualified,
            ),
            Scope::Enum(enum_scope) => enum_scope.definitions.get(symbol).into(),
            Scope::Struct(struct_scope) => struct_scope.definitions.get(symbol).into(),
            Scope::Using(using_scope) => using_scope
                .symbols
                .get(symbol)
                .cloned()
                .map_or(Resolution::Unresolved, Resolution::from),
            Scope::Parameters(parameters_scope) => {
                parameters_scope.lookup_definition(symbol).into()
            }

            Scope::Block(_)
            | Scope::File(_)
            | Scope::Function(_)
            | Scope::Modifier(_)
            | Scope::YulBlock(_)
            | Scope::YulFunction(_) => Resolution::Unresolved,
        }
    }

    pub(crate) fn resolve_in_contract_scope(
        &self,
        scope_id: ScopeId,
        symbol: &str,
        options: ResolveOptions,
    ) -> Resolution {
        let scope = self.get_scope_by_id(scope_id);
        if let Scope::Contract(contract_scope) = scope {
            self.resolve_in_contract_scope_internal(contract_scope, symbol, options)
        } else {
            Resolution::Unresolved
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
