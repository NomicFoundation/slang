use std::ops::Range;

use slang_solidity_v2_common::collections::{Map, SortedMap};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

/// Pre-computed member linearisations for a single contract.
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

/// Cache of derived data about contracts stored on the `SemanticContext`. Every
/// contract's `NodeId` has an entry in `data`.
pub(crate) struct ContractData {
    /// All contract definitions in this compilation unit, in registration
    /// order (deterministic iteration for `all_contracts`).
    contracts: Vec<ir::ContractDefinition>,
    /// Per-contract linearised members, keyed by contract `NodeId`.
    linearisations: Map<NodeId, ContractLinearisations>,
    /// For each contract, the contracts that its creation code embeds,
    /// mapped to the first expression embedding them. Keyed by the
    /// embedding contract's id and then by the embedded contract's id.
    creation_bytecode_dependencies: SortedMap<NodeId, SortedMap<NodeId, ContractReference>>,
    /// The same for the deployed code.
    deployed_bytecode_dependencies: SortedMap<NodeId, SortedMap<NodeId, ContractReference>>,
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
            .expect("contract_id is a registered contract")
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
