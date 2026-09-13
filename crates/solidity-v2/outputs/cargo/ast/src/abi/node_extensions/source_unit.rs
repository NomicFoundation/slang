use crate::abi;
use crate::abi::types::AbiTypeCache;
use crate::ast::SourceUnitStruct;

impl SourceUnitStruct {
    pub fn compute_contracts_abi(&self) -> Vec<abi::ContractAbi> {
        // One cache per source unit: the contracts of a file share most of their types.
        let mut cache = AbiTypeCache::default();
        self.contracts()
            .iter()
            .filter_map(|contract| {
                if contract.is_abstract() {
                    None
                } else {
                    contract.compute_abi_cached(&mut cache)
                }
            })
            .collect()
    }
}
