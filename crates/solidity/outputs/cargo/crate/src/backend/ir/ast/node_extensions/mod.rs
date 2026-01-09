use super::{ContractDefinition, SourceUnitMember, SourceUnitStruct};

mod contracts;
pub use contracts::ContractBase;

mod definitions;
pub use definitions::Definition;

mod identifiers;
pub(crate) use identifiers::{create_identifier, create_yul_identifier};
pub use identifiers::{
    Identifier, IdentifierStruct, Reference, YulIdentifier, YulIdentifierStruct,
};

impl SourceUnitStruct {
    pub fn file_id(&self) -> String {
        self.semantic
            .node_id_to_file_id(self.ir_node.node_id)
            .unwrap()
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
