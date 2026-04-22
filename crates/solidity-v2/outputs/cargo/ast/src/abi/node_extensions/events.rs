use crate::abi::{selector_from_signature, AbiEntry, AbiEvent};
use crate::ast::EventDefinitionStruct;

impl EventDefinitionStruct {
    pub fn compute_canonical_signature(&self) -> Option<String> {
        let name = self.ir_node.name.unparse();
        let parameters = self.parameters().compute_canonical_signature()?;
        Some(format!("{name}({parameters})"))
    }

    pub fn compute_internal_signature(&self) -> Option<String> {
        let name = self.ir_node.name.unparse();
        let parameters = self.parameters().compute_internal_signature()?;
        Some(format!("{name}({parameters})"))
    }

    pub fn compute_abi_entry(&self) -> Option<AbiEntry> {
        let inputs = self.parameters().compute_abi_parameters()?;

        Some(AbiEntry::Event(AbiEvent {
            node_id: self.ir_node.id(),
            name: self.ir_node.name.unparse().to_string(),
            inputs,
            anonymous: self.ir_node.anonymous_keyword,
        }))
    }

    pub fn compute_selector(&self) -> Option<u32> {
        self.compute_canonical_signature()
            .map(|sig| selector_from_signature(&sig))
    }
}
