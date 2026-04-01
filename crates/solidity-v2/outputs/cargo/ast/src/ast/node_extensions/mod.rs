use super::{ContractDefinition, SourceUnitMember, SourceUnitStruct};

mod contracts;
pub use contracts::ContractBase;

mod errors;
mod events;
mod functions;
mod identifiers;
mod state_variables;

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
}
