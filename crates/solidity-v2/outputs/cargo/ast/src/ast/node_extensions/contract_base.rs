use super::super::{ContractDefinition, Definition, InterfaceDefinition};

pub enum ContractBase {
    Contract(ContractDefinition),
    Interface(InterfaceDefinition),
}

impl ContractBase {
    pub(crate) fn from_definition(definition: &Definition) -> Option<Self> {
        match definition {
            Definition::Contract(contract) => Some(Self::Contract(contract.clone())),
            Definition::Interface(interface) => Some(Self::Interface(interface.clone())),
            _ => None,
        }
    }
}
