use super::super::{ContractDefinition, SourceUnitMember, SourceUnitStruct};

impl SourceUnitStruct {
    pub fn file_id(&self) -> String {
        self.semantic.file_id_from_node_id(self.ir_node.id())
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
