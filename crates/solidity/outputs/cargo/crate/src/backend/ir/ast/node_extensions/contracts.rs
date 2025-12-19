use std::{cmp::Ordering, rc::Rc};

use super::super::{
    ContractDefinition, ContractDefinitionStruct, ContractMember, ContractMembersStruct,
    Definition, FunctionDefinition, FunctionKind, InterfaceDefinition, StateVariableDefinition,
};

pub enum ContractBase {
    Contract(ContractDefinition),
    Interface(InterfaceDefinition),
}

impl ContractBase {
    fn from_definition(definition: &Definition) -> Option<Self> {
        match definition {
            Definition::Contract(contract) => Some(Self::Contract(Rc::clone(contract))),
            Definition::Interface(interface) => Some(Self::Interface(Rc::clone(interface))),
            _ => None,
        }
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
                let base_definition = Rc::new(Definition::create(*node_id, &self.semantic));
                ContractBase::from_definition(&base_definition)
                    .expect("Linearised base is either a contract or interface")
            })
            .collect()
    }

    pub fn state_variables(&self) -> Vec<StateVariableDefinition> {
        self.members().iter_state_variable_definitions().collect()
    }

    /// Returns the list of state variable definitions in the order laid out in storage
    pub fn linearised_state_variables(&self) -> Vec<StateVariableDefinition> {
        let mut state_variables = Vec::new();
        let bases = self.linearised_bases();
        for base in bases.iter().rev() {
            let ContractBase::Contract(contract) = base else {
                continue;
            };
            state_variables.extend(contract.state_variables());
        }
        state_variables
    }

    pub fn functions(&self) -> Vec<FunctionDefinition> {
        self.members()
            .iter_function_definitions()
            .filter(|function| {
                matches!(
                    function.kind(),
                    FunctionKind::Regular
                        | FunctionKind::Fallback
                        | FunctionKind::Receive
                        | FunctionKind::Unnamed
                )
            })
            .collect()
    }

    pub fn modifiers(&self) -> Vec<FunctionDefinition> {
        self.members()
            .iter_function_definitions()
            .filter(|function| matches!(function.kind(), FunctionKind::Modifier))
            .collect()
    }

    pub fn constructor(&self) -> Option<FunctionDefinition> {
        self.members()
            .iter_function_definitions()
            .find(|function| matches!(function.kind(), FunctionKind::Constructor))
    }

    /// Returns the list of functions defined in all the hierarchy of the
    /// contract, in alphabetical order
    pub fn linearised_functions(&self) -> Vec<FunctionDefinition> {
        let mut functions = Vec::new();
        let bases = self.linearised_bases();
        for base in &bases {
            // TODO(validation): we don't pick up functions defined in
            // interfaces because they should be implemented in inheriting
            // contracts, but this is not yet enforced anywhere
            let ContractBase::Contract(contract) = base else {
                continue;
            };
            // FIXME: handle function overriding
            functions.extend(contract.functions());
        }
        functions.sort_by(|a, b| match (a.name(), b.name()) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(a), Some(b)) => a.unparse().cmp(&b.unparse()),
        });
        functions
    }
}

impl ContractMembersStruct {
    pub(crate) fn iter_function_definitions(
        &self,
    ) -> impl Iterator<Item = FunctionDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::FunctionDefinition(function) = member {
                Some(function)
            } else {
                None
            }
        })
    }

    pub(crate) fn iter_state_variable_definitions(
        &self,
    ) -> impl Iterator<Item = StateVariableDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::StateVariableDefinition(state_variable) = member {
                Some(state_variable)
            } else {
                None
            }
        })
    }
}
