use std::sync::Arc;

use super::super::{ContractDefinition, Definition, InterfaceDefinition};

pub enum ContractBase {
    Contract(ContractDefinition),
    Interface(InterfaceDefinition),
}

impl ContractBase {
    pub(crate) fn from_definition(definition: &Definition) -> Option<Self> {
        match definition {
            Definition::Contract(contract) => Some(Self::Contract(Arc::clone(contract))),
            Definition::Interface(interface) => Some(Self::Interface(Arc::clone(interface))),
            _ => None,
        }
    }
}
