use crate::abi;
use crate::ast::{SourceUnitMember, SourceUnitStruct};

impl SourceUnitStruct {
    /// Returns the ABIs of every contract, interface, and library in this
    /// source unit. A definition whose ABI cannot be computed is left out.
    pub fn compute_abi(&self) -> Vec<abi::ContractAbi> {
        self.members()
            .iter()
            .filter_map(|member| match member {
                SourceUnitMember::ContractDefinition(contract) => contract.compute_abi(),
                SourceUnitMember::InterfaceDefinition(interface) => interface.compute_abi(),
                SourceUnitMember::LibraryDefinition(library) => library.compute_abi(),
                _ => None,
            })
            .collect()
    }
}
