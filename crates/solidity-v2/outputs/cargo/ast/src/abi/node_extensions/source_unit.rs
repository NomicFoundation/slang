use crate::abi;
use crate::ast::SourceUnitStruct;

impl SourceUnitStruct {
    pub fn compute_contracts_abi(&self) -> Vec<abi::ContractAbi> {
        let file_id = self.file_id();
        self.contracts()
            .iter()
            .filter_map(|contract| {
                if contract.abstract_keyword() {
                    None
                } else {
                    contract.compute_abi_with_file_id(file_id.to_string())
                }
            })
            .collect()
    }
}
