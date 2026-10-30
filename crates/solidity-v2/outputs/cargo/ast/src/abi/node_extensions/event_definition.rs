use crate::abi::entries::{Entry, Event};
use crate::abi::hash_from_signature;
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

    pub(crate) fn compute_abi_entry(&self) -> Option<Entry> {
        let inputs = self.parameters().compute_abi_parameters()?;

        Some(Entry::Event(Event {
            node_id: self.ir_node.id(),
            name: self.ir_node.name.unparse().to_string(),
            inputs,
            anonymous: self.ir_node.is_anonymous,
        }))
    }

    pub fn compute_topic0(&self) -> Option<[u8; 32]> {
        if self.ir_node.is_anonymous {
            return None;
        }
        self.compute_canonical_signature()
            .map(|sig| hash_from_signature(&sig))
    }
}
