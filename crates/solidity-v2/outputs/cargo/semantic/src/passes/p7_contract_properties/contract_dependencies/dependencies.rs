use std::collections::VecDeque;
use std::sync::Arc;

use slang_solidity_v2_common::collections::{Map, OrderedSet, Set, SortedMap};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::references::{CallableReference, PointerSignature, UnitReferences};
use super::units::CodeUnit;
use crate::binder::{Binder, Definition};
use crate::context::dispatch::{function_target, modifier_target, super_target};
use crate::context::{ContractData, ContractReference};
use crate::passes::common::Overridable;
use crate::types::{Type, TypeRegistry};

/// One contract's dependencies, keyed by the dependency's id and mapped to
/// the first expression referencing it.
pub(super) type Dependencies = SortedMap<NodeId, ContractReference>;

/// Every contract's dependencies, keyed by contract id.
pub(super) type DependencyMap = SortedMap<NodeId, Dependencies>;

/// Resolved targets of the indirect calls seen during a walk.
type IndirectCalls = OrderedSet<NodeId>;

/// Every contract's or library's functions of one segment, keyed by its id.
pub(super) type FunctionMap = SortedMap<NodeId, Vec<ir::FunctionDefinition>>;

/// The per-phase bytecode dependencies, functions and pointer targets of
/// every contract and library, and the errors and events their code can
/// revert with or emit.
pub(super) struct ContractDependencies {
    pub(super) creation: DependencyMap,
    pub(super) deployed: DependencyMap,
    pub(super) creation_functions: FunctionMap,
    pub(super) creation_pointer_targets: FunctionMap,
    pub(super) deployed_functions: FunctionMap,
    pub(super) deployed_pointer_targets: FunctionMap,
    pub(super) used_errors: SortedMap<NodeId, Vec<ir::ErrorDefinition>>,
    pub(super) used_events: SortedMap<NodeId, Vec<ir::EventDefinition>>,
}

/// Computes the creation and deployed bytecode dependency maps, functions
/// and pointer targets of every contract and library, and the errors and
/// events reached from either phase. Libraries only have deployed code.
pub(super) fn build(
    binder: &Binder,
    contract_data: &ContractData,
    types: &TypeRegistry,
    units: &[CodeUnit<'_>],
    unit_references: &Map<NodeId, UnitReferences>,
) -> ContractDependencies {
    let callables: Map<NodeId, &ir::FunctionDefinition> = units
        .iter()
        .filter_map(|unit| Some((unit.id, unit.callable()?)))
        .collect();
    let mut creation_dependencies = DependencyMap::default();
    let mut deployed_dependencies = DependencyMap::default();
    let mut creation_functions = FunctionMap::default();
    let mut creation_pointer_targets = FunctionMap::default();
    let mut deployed_functions = FunctionMap::default();
    let mut deployed_pointer_targets = FunctionMap::default();
    let mut used_errors = SortedMap::default();
    let mut used_events = SortedMap::default();
    for (definition_id, definition) in binder.definitions() {
        let mut collector = DependencyCollector {
            binder,
            contract_data,
            types,
            unit_references,
            contract_id: *definition_id,
            resolved_callables: Map::default(),
            errors: OrderedSet::default(),
            events: OrderedSet::default(),
        };
        let (creation, deployed, creation_segment, deployed_segment) = match definition {
            Definition::Contract(_) => {
                let (creation, indirect_calls) = collector.collect_creation_code();
                let deployed = collector.collect_deployed_code(indirect_calls);
                let creation_segment =
                    collector.walk_segment(collector.creation_roots(), &IndirectCalls::default());
                // The creation code can store a function the deployed code
                // calls through a pointer.
                let deployed_segment =
                    collector.walk_segment(collector.deployed_roots(), &creation_segment.taken);
                (creation, deployed, creation_segment, deployed_segment)
            }
            Definition::Library(library) => (
                Dependencies::default(),
                collector.collect_library_code(&library.ir_node),
                SegmentWalk::default(),
                collector.walk_segment(library_roots(&library.ir_node), &IndirectCalls::default()),
            ),
            _ => continue,
        };
        if !creation.is_empty() {
            creation_dependencies.insert(*definition_id, creation);
        }
        if !deployed.is_empty() {
            deployed_dependencies.insert(*definition_id, deployed);
        }
        let (functions, pointer_targets) = creation_segment.into_functions(&callables);
        if !functions.is_empty() {
            creation_functions.insert(*definition_id, functions);
        }
        if !pointer_targets.is_empty() {
            creation_pointer_targets.insert(*definition_id, pointer_targets);
        }
        let (functions, pointer_targets) = deployed_segment.into_functions(&callables);
        if !functions.is_empty() {
            deployed_functions.insert(*definition_id, functions);
        }
        if !pointer_targets.is_empty() {
            deployed_pointer_targets.insert(*definition_id, pointer_targets);
        }
        if !collector.errors.is_empty() {
            let errors = collector.errors.iter();
            let errors = errors.map(|id| error_definition(binder, *id)).collect();
            used_errors.insert(*definition_id, errors);
        }
        if !collector.events.is_empty() {
            let events = collector.events.iter();
            let events = events.map(|id| event_definition(binder, *id)).collect();
            used_events.insert(*definition_id, events);
        }
    }
    ContractDependencies {
        creation: creation_dependencies,
        deployed: deployed_dependencies,
        creation_functions,
        creation_pointer_targets,
        deployed_functions,
        deployed_pointer_targets,
        used_errors,
        used_events,
    }
}

fn error_definition(binder: &Binder, id: NodeId) -> ir::ErrorDefinition {
    match binder.find_definition_by_id(id) {
        Some(Definition::Error(error)) => Arc::clone(&error.ir_node),
        _ => unreachable!("a collected error is an error definition"),
    }
}

fn event_definition(binder: &Binder, id: NodeId) -> ir::EventDefinition {
    match binder.find_definition_by_id(id) {
        Some(Definition::Event(event)) => Arc::clone(&event.ir_node),
        _ => unreachable!("a collected event is an event definition"),
    }
}

/// A walk over the calls of one segment. A function its code takes as a
/// value is pending until the walk reaches a pointer call of its signature.
#[derive(Default)]
struct SegmentWalk<'a> {
    queue: VecDeque<NodeId>,
    /// The visited units, in the order the walk first reaches them.
    visited: OrderedSet<NodeId>,
    /// The functions taken as values, each once, in first-reference order.
    taken: IndirectCalls,
    /// The taken functions that are no pointer targets yet, with their
    /// signatures.
    pending: Vec<(NodeId, PointerSignature)>,
    pointer_calls: Set<&'a PointerSignature>,
    /// The taken functions that became pointer targets, in the order they
    /// did.
    pointer_targets: Vec<NodeId>,
}

impl SegmentWalk<'_> {
    /// The segment's functions and its pointer targets.
    fn into_functions(
        self,
        callables: &Map<NodeId, &ir::FunctionDefinition>,
    ) -> (Vec<ir::FunctionDefinition>, Vec<ir::FunctionDefinition>) {
        // Initializers, constants and inheritance arguments are visited for
        // their references, but they are no functions.
        let functions = self
            .visited
            .iter()
            .filter_map(|unit_id| callables.get(unit_id))
            .map(|function| Arc::clone(function))
            .collect();
        let pointer_targets = self
            .pointer_targets
            .iter()
            .map(|function_id| {
                let function = callables
                    .get(function_id)
                    .expect("a pointer target is a function definition");
                Arc::clone(function)
            })
            .collect();
        (functions, pointer_targets)
    }
}

/// A library's externally callable functions and constants.
fn library_roots(library: &ir::LibraryDefinitionStruct) -> Vec<NodeId> {
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
    units
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
    /// Errors and events reached by any walk so far, each once.
    errors: OrderedSet<NodeId>,
    events: OrderedSet<NodeId>,
}

impl<'a> DependencyCollector<'a> {
    /// Walks the code that runs at creation. Also returns the indirectly
    /// referenced callables, which can still run after deployment through
    /// a stored pointer.
    fn collect_creation_code(&mut self) -> (Dependencies, IndirectCalls) {
        self.walk(self.creation_roots())
    }

    /// The units that run at creation, in the order they run.
    fn creation_roots(&self) -> Vec<NodeId> {
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
        units
    }

    /// Walks the code that runs after deployment, reachable from the
    /// externally callable functions and constants and from the callables
    /// indirectly referenced during creation.
    fn collect_deployed_code(&mut self, indirect_calls: IndirectCalls) -> Dependencies {
        let mut units = self.deployed_roots();
        units.extend(indirect_calls);
        let (dependencies, _) = self.walk(units);
        dependencies
    }

    /// The externally callable functions and constants.
    fn deployed_roots(&self) -> Vec<NodeId> {
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
        units
    }

    /// Walks a library's code, reachable from its externally callable
    /// functions and constants.
    fn collect_library_code(&mut self, library: &ir::LibraryDefinitionStruct) -> Dependencies {
        let (dependencies, _) = self.walk(library_roots(library));
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
            self.errors.extend(unit.errors.iter().copied());
            self.events.extend(unit.events.iter().copied());
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

    /// Walks one segment from `roots`, following calls only. A function
    /// taken as a value runs only through a pointer, so it becomes a pointer
    /// target and joins the walk once a pointer call of its signature is
    /// reached. `taken` holds the functions the segment's code can find
    /// already stored.
    fn walk_segment(&mut self, roots: Vec<NodeId>, taken: &IndirectCalls) -> SegmentWalk<'a> {
        let mut walk = SegmentWalk {
            queue: VecDeque::from(roots),
            ..SegmentWalk::default()
        };
        for function_id in taken {
            self.take(&mut walk, *function_id);
        }
        // A joined function can reach a pointer call another pending one
        // matches, so the walk resumes until none joins.
        loop {
            while let Some(unit_id) = walk.queue.pop_front() {
                if !walk.visited.insert(unit_id) {
                    continue;
                }
                let Some(unit) = self.unit_references.get(&unit_id) else {
                    continue;
                };
                for callable in &unit.calls {
                    walk.queue.push_back(self.resolve_callable(*callable));
                }
                for callable in &unit.indirect_calls {
                    let target = self.resolve_callable(*callable);
                    self.take(&mut walk, target);
                }
                walk.pointer_calls.extend(&unit.pointer_calls);
            }
            let joined: Vec<_> = walk
                .pending
                .extract_if(.., |(_, signature)| walk.pointer_calls.contains(signature))
                .collect();
            if joined.is_empty() {
                return walk;
            }
            // A joined function the walk already visited is not visited
            // again.
            for (function_id, _) in joined {
                walk.pointer_targets.push(function_id);
                walk.queue.push_back(function_id);
            }
        }
    }

    /// Makes a function the walk has not taken before pending.
    fn take(&self, walk: &mut SegmentWalk<'_>, function_id: NodeId) {
        if walk.taken.insert(function_id)
            && let Some(signature) = self.signature(function_id)
        {
            walk.pending.push((function_id, signature));
        }
    }

    /// The signature of a function taken as a value. Only invalid input
    /// leaves a function without a type, and it matches no pointer call.
    fn signature(&self, function_id: NodeId) -> Option<PointerSignature> {
        let type_id = self.binder.node_typing(function_id).as_type_id()?;
        let Type::Function(function_type) = self.types.get_type_by_id(type_id) else {
            unreachable!("a function definition is typed as a function");
        };
        Some(PointerSignature::new(function_type))
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
            Some(Definition::Function(function)) => {
                // Dispatch only asks whether two members share a slot, which
                // never reads the interface flag, so it is left `false`.
                let overridden = Overridable::of_function(&function.ir_node, false)
                    .expect("a function definition is a regular function");
                function_target(
                    self.binder,
                    self.types,
                    self.contract_data.linearised_functions(self.contract_id),
                    overridden,
                )
                .map_or(declaration, |resolved| resolved.id())
            }
            Some(Definition::Modifier(modifier)) => self
                .resolve_modifier(&modifier.ir_node)
                .unwrap_or(declaration),
            _ => declaration,
        }
    }

    fn resolve_modifier(&self, modifier: &ir::FunctionDefinition) -> Option<NodeId> {
        // Most-derived first. Modifiers cannot overload, the name suffices.
        // The interface flag is never read here, so it is left `false`.
        let overridden =
            Overridable::of_function(modifier, false).expect("a modifier definition is a modifier");
        modifier_target(
            self.binder,
            self.types,
            self.binder
                .get_linearised_bases(self.contract_id)
                .expect("p2 linearises every contract"),
            overridden,
        )
        .map(|target| target.id())
    }

    /// Resolves which implementation a `super.f()` call runs when compiled
    /// into the collected contract. Falls back to the referenced
    /// declaration when nothing matches.
    fn resolve_super(&self, declaration: NodeId, enclosing_contract: NodeId) -> NodeId {
        let Some(Definition::Function(function)) = self.binder.find_definition_by_id(declaration)
        else {
            return declaration;
        };

        // The nearest override with a body wins.
        // The interface flag is never read here, so it is left `false`.
        let overridden = Overridable::of_function(&function.ir_node, false)
            .expect("a function definition is a regular function");
        super_target(
            self.binder,
            self.types,
            self.binder
                .get_linearised_bases(self.contract_id)
                .expect("p2 linearises every contract"),
            enclosing_contract,
            overridden,
        )
        .map_or(declaration, |target| target.id())
    }
}
