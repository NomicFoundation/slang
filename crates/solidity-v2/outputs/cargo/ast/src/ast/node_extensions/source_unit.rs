use super::super::{ContractDefinition, SourceUnitMember, SourceUnitStruct};
use crate::abi;

impl SourceUnitStruct {
    pub fn file_id(&self) -> String {
        self.semantic.node_id_to_file_id(self.ir_node.id()).unwrap()
    }

    pub fn contracts(&self) -> Vec<ContractDefinition> {
        self.members()
            .iter()
            .filter_map(|member| {
                if let SourceUnitMember::ContractDefinition(contract) = member {
                    Some(contract)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn compute_contracts_abi(&self) -> Vec<abi::ContractAbi> {
        let file_id = self.file_id();
        self.contracts()
            .iter()
            .filter_map(|contract| {
                if contract.abstract_keyword() {
                    None
                } else {
                    contract.compute_abi_with_file_id(file_id.clone())
                }
            })
            .collect()
    }
}
