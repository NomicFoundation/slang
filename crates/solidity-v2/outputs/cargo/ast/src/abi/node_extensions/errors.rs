use crate::abi::{AbiEntry, AbiError};
use crate::ast::ErrorDefinitionStruct;

impl ErrorDefinitionStruct {
    pub fn compute_abi_entry(&self) -> Option<AbiEntry> {
        let inputs = self.parameters().compute_abi_parameters()?;

        Some(AbiEntry::Error(AbiError {
            node_id: self.ir_node.id(),
            name: self.ir_node.name.unparse().to_string(),
            inputs,
        }))
    }
}
