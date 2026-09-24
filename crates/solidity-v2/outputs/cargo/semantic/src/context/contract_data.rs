use std::ops::Range;
use std::sync::OnceLock;

use slang_solidity_v2_common::collections::{Map, SortedMap};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

/// Pre-computed member linearisations for a single contract or interface.
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub(crate) struct ContractLinearisations {
    pub(crate) functions: Vec<ir::FunctionDefinition>,
    pub(crate) state_variables: Vec<ir::StateVariableDefinition>,
    pub(crate) errors: Vec<ir::ErrorDefinition>,
    pub(crate) events: Vec<ir::EventDefinition>,
}

/// An expression embedding another contract's bytecode.
#[derive(Clone, Debug)]
pub enum ContractReference {
    /// A `new C` expression.
    New(ir::NewExpression),
    /// A `type(C).creationCode` or `type(C).runtimeCode` access.
    CodeAccess(ir::MemberAccessExpression),
}

impl ContractReference {
    /// The id of the referencing expression node.
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::New(expression) => expression.id(),
            Self::CodeAccess(expression) => expression.id(),
        }
    }

    /// The source range of the referencing expression.
    pub fn range(&self) -> Range<usize> {
        match self {
            Self::New(expression) => expression.range.clone(),
            Self::CodeAccess(expression) => expression.range.clone(),
        }
    }
}

/// The errors each contract's or library's creation or deployed code can
/// revert with and the events it can emit, in first-reached order.
#[derive(Default)]
pub(crate) struct UsedErrorsAndEvents {
    pub(crate) errors: Map<NodeId, Vec<ir::ErrorDefinition>>,
    pub(crate) events: Map<NodeId, Vec<ir::EventDefinition>>,
}

/// Cache of derived data about contracts stored on the `SemanticContext`. Every
/// contract's and interface's `NodeId` has an entry in `linearisations`.
pub(crate) struct ContractData {
    /// All contract definitions in this compilation unit, in registration
    /// order (deterministic iteration for `all_contracts`).
    contracts: Vec<ir::ContractDefinition>,
    /// Linearised members, keyed by contract or interface `NodeId`.
    linearisations: Map<NodeId, ContractLinearisations>,
    /// For each contract, the contracts that its creation code embeds,
    /// mapped to the first expression embedding them. Keyed by the
    /// embedding contract's id and then by the embedded contract's id.
    creation_bytecode_dependencies: SortedMap<NodeId, SortedMap<NodeId, ContractReference>>,
    /// The same for the deployed code.
    deployed_bytecode_dependencies: SortedMap<NodeId, SortedMap<NodeId, ContractReference>>,
    /// Filled by p7 when it walks the code for bytecode dependencies anyway,
    /// otherwise on first use.
    used_errors_and_events: OnceLock<UsedErrorsAndEvents>,
}

impl ContractData {
    pub(crate) fn new(
        contracts: Vec<ir::ContractDefinition>,
        data: Map<NodeId, ContractLinearisations>,
    ) -> Self {
        Self {
            contracts,
            linearisations: data,
            creation_bytecode_dependencies: SortedMap::default(),
            deployed_bytecode_dependencies: SortedMap::default(),
            used_errors_and_events: OnceLock::new(),
        }
    }

    pub(crate) fn set_contract_dependencies(
        &mut self,
        creation: SortedMap<NodeId, SortedMap<NodeId, ContractReference>>,
        deployed: SortedMap<NodeId, SortedMap<NodeId, ContractReference>>,
    ) {
        self.creation_bytecode_dependencies = creation;
        self.deployed_bytecode_dependencies = deployed;
    }

    pub(crate) fn set_used_errors_and_events(&mut self, used: UsedErrorsAndEvents) {
        assert!(
            self.used_errors_and_events.set(used).is_ok(),
            "used errors and events are computed once"
        );
    }

    pub(crate) fn used_errors_and_events(
        &self,
        compute: impl FnOnce() -> UsedErrorsAndEvents,
    ) -> &UsedErrorsAndEvents {
        self.used_errors_and_events.get_or_init(compute)
    }

    /// For each contract, the contracts that its creation code embeds.
    /// Contracts without dependencies have no entry.
    pub(crate) fn creation_bytecode_dependencies(
        &self,
    ) -> &SortedMap<NodeId, SortedMap<NodeId, ContractReference>> {
        &self.creation_bytecode_dependencies
    }

    /// The same for the deployed code.
    pub(crate) fn deployed_bytecode_dependencies(
        &self,
    ) -> &SortedMap<NodeId, SortedMap<NodeId, ContractReference>> {
        &self.deployed_bytecode_dependencies
    }

    /// The creation and deployed dependencies combined into one map. When
    /// both embed the same contract, the creation expression is the one
    /// recorded.
    pub(crate) fn contract_dependencies(
        &self,
    ) -> SortedMap<NodeId, SortedMap<NodeId, ContractReference>> {
        let mut dependencies = self.creation_bytecode_dependencies.clone();
        for (contract_id, targets) in &self.deployed_bytecode_dependencies {
            let entry = dependencies.entry(*contract_id).or_default();
            for (target, reference) in targets {
                entry.entry(*target).or_insert_with(|| reference.clone());
            }
        }
        dependencies
    }

    fn get(&self, contract_id: NodeId) -> &ContractLinearisations {
        self.linearisations
            .get(&contract_id)
            .expect("contract_id is a registered contract or interface")
    }

    pub(super) fn all_contracts(&self) -> impl Iterator<Item = &ir::ContractDefinition> {
        self.contracts.iter()
    }

    pub(super) fn find_contract_by_name<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = ir::ContractDefinition> + use<'a> {
        self.contracts
            .iter()
            .filter(move |contract| contract.name.unparse() == name)
            .cloned()
    }

    pub(crate) fn linearised_functions(&self, contract_id: NodeId) -> &[ir::FunctionDefinition] {
        &self.get(contract_id).functions
    }

    pub(crate) fn linearised_state_variables(
        &self,
        contract_id: NodeId,
    ) -> &[ir::StateVariableDefinition] {
        &self.get(contract_id).state_variables
    }

    pub(super) fn linearised_errors(&self, contract_id: NodeId) -> &[ir::ErrorDefinition] {
        &self.get(contract_id).errors
    }

    pub(super) fn linearised_events(&self, contract_id: NodeId) -> &[ir::EventDefinition] {
        &self.get(contract_id).events
    }
}
