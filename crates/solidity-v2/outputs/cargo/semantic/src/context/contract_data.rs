use slang_solidity_v2_common::collections::Map;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

/// Pre-computed member linearisations for a single contract.
#[allow(clippy::struct_field_names)]
pub(crate) struct ContractLinearisations {
    pub(crate) functions: Vec<ir::FunctionDefinition>,
    pub(crate) state_variables: Vec<ir::StateVariableDefinition>,
    pub(crate) errors: Vec<ir::ErrorDefinition>,
    pub(crate) events: Vec<ir::EventDefinition>,
}

/// Cache of derived data about contracts stored on the `SemanticContext`. Every
/// contract's `NodeId` has an entry in `data`.
pub(crate) struct ContractData {
    /// All contract definitions in this compilation unit, in registration
    /// order (deterministic iteration for `all_contracts`).
    contracts: Vec<ir::ContractDefinition>,
    /// Per-contract linearised members, keyed by contract `NodeId`.
    linearisations: Map<NodeId, ContractLinearisations>,
}

impl ContractData {
    pub(crate) fn new(
        contracts: Vec<ir::ContractDefinition>,
        data: Map<NodeId, ContractLinearisations>,
    ) -> Self {
        Self {
            contracts,
            linearisations: data,
        }
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

    pub(super) fn linearised_functions(&self, contract_id: NodeId) -> &[ir::FunctionDefinition] {
        &self.get(contract_id).functions
    }

    pub(super) fn linearised_state_variables(
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
