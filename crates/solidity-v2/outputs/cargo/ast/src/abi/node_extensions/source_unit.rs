use crate::abi;
use crate::ast::SourceUnitStruct;

impl SourceUnitStruct {
    pub fn compute_contracts_abi(&self) -> Vec<abi::ContractAbi> {
        self.contracts()
            .iter()
            .filter_map(|contract| {
                if contract.abstract_keyword().is_some() {
                    None
                } else {
                    contract.compute_abi()
                }
            })
            .collect()
    }
}
