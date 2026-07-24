use std::collections::VecDeque;

use slang_solidity_v2_common::collections::{Map, OrderedSet, Set, SortedMap};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::references::{CallableReference, UnitReferences};
use crate::binder::{Binder, Definition};
use crate::context::{ContractData, ContractReference};
use crate::passes::common::function_overrides;
use crate::types::TypeRegistry;

/// One contract's dependencies, keyed by the dependency's id and mapped to
/// the first expression referencing it.
pub(super) type Dependencies = SortedMap<NodeId, ContractReference>;

/// Every contract's dependencies, keyed by contract id.
pub(super) type DependencyMap = SortedMap<NodeId, Dependencies>;

/// Resolved targets of the indirect calls seen during a walk.
type IndirectCalls = OrderedSet<NodeId>;

/// The per-phase bytecode dependencies of every contract and library.
pub(super) struct ContractDependencies {
    pub(super) creation: DependencyMap,
    pub(super) deployed: DependencyMap,
}

/// Computes the creation and deployed bytecode dependency maps of every
/// contract and library. Libraries only have deployed dependencies.
pub(super) fn build(
    binder: &Binder,
    contract_data: &ContractData,
    types: &TypeRegistry,
    unit_references: &Map<NodeId, UnitReferences>,
) -> ContractDependencies {
    let mut creation_dependencies = DependencyMap::default();
    let mut deployed_dependencies = DependencyMap::default();
    for (definition_id, definition) in binder.definitions() {
        let mut collector = DependencyCollector {
            binder,
            contract_data,
            types,
            unit_references,
            contract_id: *definition_id,
            resolved_callables: Map::default(),
        };
        let (creation, deployed) = match definition {
            Definition::Contract(_) => {
                let (creation, indirect_calls) = collector.collect_creation_code();
                let deployed = collector.collect_deployed_code(indirect_calls);
                (creation, deployed)
            }
            Definition::Library(library) => (
                Dependencies::default(),
                collector.collect_library_code(&library.ir_node),
            ),
            _ => continue,
        };
        if !creation.is_empty() {
            creation_dependencies.insert(*definition_id, creation);
        }
        if !deployed.is_empty() {
            deployed_dependencies.insert(*definition_id, deployed);
        }
    }
    ContractDependencies {
        creation: creation_dependencies,
        deployed: deployed_dependencies,
    }
}

/// Collects one contract's dependencies from the unit references reachable
/// from its entry points.
struct DependencyCollector<'a> {
    binder: &'a Binder,
    contract_data: &'a ContractData,
    types: &'a TypeRegistry,
    unit_references: &'a Map<NodeId, UnitReferences>,
    contract_id: NodeId,
    /// Targets of the virtual and super references resolved so far.
    resolved_callables: Map<CallableReference, NodeId>,
}

impl DependencyCollector<'_> {
    /// Walks the code that runs at creation. Also returns the indirectly
    /// referenced callables, which can still run after deployment through
    /// a stored pointer.
    fn collect_creation_code(&mut self) -> (Dependencies, IndirectCalls) {
        let mut units = Vec::new();
        if let Some(bases) = self.binder.get_linearised_bases(self.contract_id) {
            for base_id in bases.iter().rev() {
                let Some(Definition::Contract(base)) = self.binder.find_definition_by_id(*base_id)
                else {
                    continue;
                };
                // Initializers run before the constructor even when it is
                // declared above them, so the constructor is pushed after
                // them. All units share one queue, so a base's constructor
                // is walked before the next base's initializers.
                let mut constructor = None;
                for member in base.ir_node.members.iter() {
                    match member {
                        ir::ContractMember::StateVariableDefinition(state_variable)
                            if !matches!(
                                state_variable.attributes.mutability,
                                ir::StateVariableMutability::Constant
                            ) =>
                        {
                            units.push(state_variable.id());
                        }
                        ir::ContractMember::FunctionDefinition(function)
                            if matches!(function.kind, ir::FunctionKind::Constructor) =>
                        {
                            constructor = Some(function.id());
                        }
                        _ => {}
                    }
                }
                if let Some(constructor) = constructor {
                    units.push(constructor);
                }
                for inheritance_type in base.ir_node.inheritance_types.iter() {
                    if inheritance_type.arguments.is_some() {
                        units.push(inheritance_type.id());
                    }
                }
            }
        }
        self.walk(units)
    }

    /// Walks the code that runs after deployment, reachable from the
    /// externally callable functions and constants and from the callables
    /// indirectly referenced during creation.
    fn collect_deployed_code(&mut self, indirect_calls: IndirectCalls) -> Dependencies {
        let mut units = Vec::new();
        // Entry points come in linearised list order, unnamed ones first
        // and then by name. Several can reach the same dependency, and the
        // first one walked wins.
        for function in self.contract_data.linearised_functions(self.contract_id) {
            let is_entry_point = match function.kind {
                ir::FunctionKind::Regular => function.is_externally_visible(),
                ir::FunctionKind::Fallback | ir::FunctionKind::Receive => true,
                _ => false,
            };
            if is_entry_point {
                units.push(function.id());
            }
        }
        // A public constant's getter returns the value, so the value is
        // compiled in even when nothing reads the constant. A constant
        // without a getter is an `ir::ConstantDefinition` and is reached
        // only through the units referencing it.
        for state_variable in self
            .contract_data
            .linearised_state_variables(self.contract_id)
        {
            if matches!(
                state_variable.attributes.mutability,
                ir::StateVariableMutability::Constant
            ) {
                units.push(state_variable.id());
            }
        }
        units.extend(indirect_calls);
        let (dependencies, _) = self.walk(units);
        dependencies
    }

    /// Walks a library's code, reachable from its externally callable
    /// functions and constants.
    fn collect_library_code(&mut self, library: &ir::LibraryDefinitionStruct) -> Dependencies {
        let mut units = Vec::new();
        for member in library.members.iter() {
            match member {
                ir::ContractMember::FunctionDefinition(function)
                    if matches!(function.kind, ir::FunctionKind::Regular)
                        && function.is_externally_visible() =>
                {
                    units.push(function.id());
                }
                ir::ContractMember::StateVariableDefinition(state_variable)
                    if matches!(
                        state_variable.attributes.mutability,
                        ir::StateVariableMutability::Constant
                    ) =>
                {
                    units.push(state_variable.id());
                }
                _ => {}
            }
        }
        let (dependencies, _) = self.walk(units);
        dependencies
    }

    /// Follows the given units and everything reachable from them breadth
    /// first, so the entry order decides which reference gets recorded for
    /// a dependency. Each call has its own visited set, so both phases
    /// record a dependency they share. Returns the dependencies and the
    /// resolved targets of the indirect calls.
    fn walk(&mut self, units: Vec<NodeId>) -> (Dependencies, IndirectCalls) {
        let mut dependencies = Dependencies::default();
        let mut indirect_calls = IndirectCalls::default();
        let mut visited = Set::default();
        let mut queue = VecDeque::from(units);
        while let Some(unit_id) = queue.pop_front() {
            if !visited.insert(unit_id) {
                continue;
            }
            let Some(unit) = self.unit_references.get(&unit_id) else {
                continue;
            };
            for (target, reference) in &unit.contracts {
                // The first reference to each dependency wins.
                dependencies
                    .entry(*target)
                    .or_insert_with(|| reference.clone());
            }
            for callable in &unit.calls {
                queue.push_back(self.resolve_callable(*callable));
            }
            // Indirect callables are followed like calls, and their
            // resolved targets are returned to the caller.
            for callable in &unit.indirect_calls {
                let target = self.resolve_callable(*callable);
                indirect_calls.insert(target);
                queue.push_back(target);
            }
        }
        (dependencies, indirect_calls)
    }

    fn resolve_callable(&mut self, callable: CallableReference) -> NodeId {
        // A static reference already names its target.
        if let CallableReference::Static(target) = callable {
            return target;
        }
        // Searching the linearisation is not cheap, and the same reference
        // appears in many units, so the cache is checked first.
        if let Some(target) = self.resolved_callables.get(&callable) {
            return *target;
        }
        let target = match callable {
            CallableReference::Virtual(declaration) => self.resolve_virtual(declaration),
            CallableReference::Super {
                declaration,
                enclosing_contract,
            } => self.resolve_super(declaration, enclosing_contract),
            CallableReference::Static(_) => unreachable!("static references handled above"),
        };
        self.resolved_callables.insert(callable, target);
        target
    }

    fn resolve_virtual(&self, declaration: NodeId) -> NodeId {
        // Only a contract has overrides. For any other collected definition
        // the declaration is already the target.
        if !matches!(
            self.binder.find_definition_by_id(self.contract_id),
            Some(Definition::Contract(_))
        ) {
            return declaration;
        }

        match self.binder.find_definition_by_id(declaration) {
            Some(Definition::Function(function)) => self
                .contract_data
                .linearised_functions(self.contract_id)
                .iter()
                .find(|candidate| {
                    function_overrides(self.binder, self.types, candidate, &function.ir_node)
                })
                .map_or(declaration, |resolved| resolved.id()),
            Some(Definition::Modifier(modifier)) => {
                let name = modifier
                    .ir_node
                    .name
                    .as_ref()
                    .expect("modifiers are named")
                    .unparse();
                self.resolve_modifier_by_name(name).unwrap_or(declaration)
            }
            _ => declaration,
        }
    }

    fn resolve_modifier_by_name(&self, name: &str) -> Option<NodeId> {
        // Most-derived first. Modifiers cannot overload, the name suffices.
        for base_id in self.binder.get_linearised_bases(self.contract_id)? {
            let Some(Definition::Contract(base)) = self.binder.find_definition_by_id(*base_id)
            else {
                continue;
            };
            for member in base.ir_node.members.iter() {
                if let ir::ContractMember::FunctionDefinition(function) = member
                    && matches!(function.kind, ir::FunctionKind::Modifier)
                    && function
                        .name
                        .as_ref()
                        .is_some_and(|candidate| candidate.unparse() == name)
                {
                    return Some(function.id());
                }
            }
        }
        None
    }

    /// Resolves which implementation a `super.f()` call runs when compiled
    /// into the collected contract. Falls back to the referenced
    /// declaration when nothing matches.
    fn resolve_super(&self, declaration: NodeId, enclosing_contract: NodeId) -> NodeId {
        let Some(Definition::Function(function)) = self.binder.find_definition_by_id(declaration)
        else {
            return declaration;
        };

        // `super` searches the collected contract's linearisation, starting
        // after the contract the call is written in.
        let Some(bases) = self.binder.get_linearised_bases(self.contract_id) else {
            return declaration;
        };
        let Some(position) = bases.iter().position(|base| *base == enclosing_contract) else {
            return declaration;
        };

        // The nearest override with a body wins.
        for base_id in &bases[position + 1..] {
            let members = match self.binder.find_definition_by_id(*base_id) {
                Some(Definition::Contract(base)) => &base.ir_node.members[..],
                Some(Definition::Interface(base)) => &base.ir_node.members[..],
                _ => continue,
            };
            for member in members {
                if let ir::ContractMember::FunctionDefinition(candidate) = member
                    && matches!(candidate.kind, ir::FunctionKind::Regular)
                    && candidate.body.is_some()
                    && function_overrides(self.binder, self.types, candidate, &function.ir_node)
                {
                    return candidate.id();
                }
            }
        }
        declaration
    }
}
