use std::rc::Rc;

use super::{
    ContractDefinition, ContractDefinitionStruct, Definition, DefinitionStruct, InterfaceDefinition,
};

pub enum ContractBase {
    Contract(ContractDefinition),
    Interface(InterfaceDefinition),
}

impl ContractBase {
    fn from_definition(definition: &Definition) -> Option<Self> {
        definition
            .as_contract()
            .map(Self::Contract)
            .or_else(|| definition.as_interface().map(Self::Interface))
    }
}

impl ContractDefinitionStruct {
    pub fn direct_bases(&self) -> Vec<ContractBase> {
        self.inheritance_types()
            .iter()
            .filter_map(|inheritance_type| {
                let base = inheritance_type.type_name().resolve_to_definition()?;
                ContractBase::from_definition(&base)
            })
            .collect()
    }

    pub fn linearised_bases(&self) -> Vec<ContractBase> {
        let Some(base_node_ids) = self
            .semantic
            .binder()
            .get_linearised_bases(self.ir_node.node_id)
        else {
            // TODO(validation): once we have validation implemented, this
            // branch should not be reachable, or we should generate an error
            // while building the `SemanticAnalysis`.
            return Vec::new();
        };
        base_node_ids
            .iter()
            .map(|node_id| {
                let base_definition = Rc::new(DefinitionStruct::create(*node_id, &self.semantic));
                ContractBase::from_definition(&base_definition)
                    .expect("Linearised base is either a contract or interface")
            })
            .collect()
    }
}
