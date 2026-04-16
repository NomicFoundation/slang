use crate::abi::AbiEntry;
use crate::ast::ErrorDefinitionStruct;

impl ErrorDefinitionStruct {
    pub fn compute_abi_entry(&self) -> Option<AbiEntry> {
        let inputs = self.parameters().compute_abi_parameters()?;

        Some(AbiEntry::Error {
            node_id: self.ir_node.id(),
            name: self.ir_node.name.unparse().to_string(),
            inputs,
        })
    }
}
