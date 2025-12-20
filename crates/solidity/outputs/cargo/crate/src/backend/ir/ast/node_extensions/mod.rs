use super::nodes::{
    ContractDefinition, ContractDefinitionStruct, IdentifierPathStruct, InterfaceDefinition,
    SourceUnitStruct,
};

mod contracts;
pub use contracts::ContractBase;

mod definitions;
pub use definitions::{Definition, DefinitionStruct};

mod identifiers;
pub use identifiers::{Identifier, IdentifierStruct, YulIdentifier, YulIdentifierStruct};

impl SourceUnitStruct {
    pub fn file_id(&self) -> String {
        self.semantic
            .node_id_to_file_id(self.ir_node.node_id)
            .unwrap()
    }

    pub fn contracts(&self) -> Vec<ContractDefinition> {
        self.members()
            .iter()
            .filter_map(|member| member.as_contract_definition())
            .collect()
    }
}
