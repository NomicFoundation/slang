use crate::abi::{AbiEntry, AbiEvent};
use crate::ast::EventDefinitionStruct;

impl EventDefinitionStruct {
    pub fn compute_abi_entry(&self) -> Option<AbiEntry> {
        let inputs = self.parameters().compute_abi_parameters()?;

        Some(AbiEntry::Event(AbiEvent {
            node_id: self.ir_node.id(),
            name: self.ir_node.name.unparse().to_string(),
            inputs,
            anonymous: self.ir_node.anonymous_keyword,
        }))
    }
}
