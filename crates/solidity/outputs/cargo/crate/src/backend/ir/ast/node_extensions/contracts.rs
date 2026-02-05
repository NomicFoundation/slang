use std::cmp::Ordering;
use std::rc::Rc;

use super::super::{
    ContractDefinition, ContractDefinitionStruct, ContractMember, ContractMembersStruct,
    Definition, FunctionDefinition, FunctionKind, InterfaceDefinition, StateVariableDefinition,
};
use crate::backend::ir::ast::{
    ErrorDefinition, EventDefinition, FunctionDefinitionStruct, InterfaceMembersStruct,
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

    /// Returns the list of contracts/interfaces in the hierarchy (including
    /// self) in the order given by the C3 linearisation, with self contract
    /// always first
    pub fn compute_linearised_bases(&self) -> Vec<ContractBase> {
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
                let base_definition =
                    Definition::try_create(*node_id, &self.semantic).expect("node is a definition");
                ContractBase::from_definition(&base_definition)
                    .expect("Linearised base is either a contract or interface")
            })
            .collect()
    }

    pub fn state_variables(&self) -> Vec<StateVariableDefinition> {
        self.members().iter_state_variable_definitions().collect()
    }

    /// Returns the list of state variable definitions in the order laid out in storage
    pub fn compute_linearised_state_variables(&self) -> Vec<StateVariableDefinition> {
        let mut state_variables = Vec::new();
        let bases = self.compute_linearised_bases();
        for base in bases.iter().rev() {
            let ContractBase::Contract(contract) = base else {
                continue;
            };
            state_variables.extend(contract.members().iter_state_variable_definitions());
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
    pub fn compute_linearised_functions(&self) -> Vec<FunctionDefinition> {
        let mut functions: Vec<FunctionDefinition> = Vec::new();
        let bases = self.compute_linearised_bases();
        for base in &bases {
            // TODO(validation): we don't pick up functions defined in
            // interfaces because they should be implemented in inheriting
            // contracts, but this is not yet enforced anywhere
            let ContractBase::Contract(contract) = base else {
                continue;
            };

            // Handle function overriding
            let contract_functions = contract
                .functions()
                .into_iter()
                .filter(|function| {
                    // check the existing functions and remove any duplicates
                    // because they should be overridden by them
                    let existing = functions
                        .iter()
                        .any(|linearised_function| linearised_function.overrides(function));
                    !existing
                })
                // collect to avoid holding the read-borrow on `functions`
                .collect::<Vec<_>>();

            functions.extend(contract_functions);
        }

        // sort returned functions by name
        functions.sort_by(|a, b| match (a.name(), b.name()) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(a), Some(b)) => a.name().cmp(&b.name()),
        });
        functions
    }

    pub fn errors(&self) -> Vec<ErrorDefinition> {
        self.members().iter_error_definitions().collect()
    }

    pub fn compute_linearised_errors(&self) -> Vec<ErrorDefinition> {
        let mut errors = Vec::new();
        let bases = self.compute_linearised_bases();
        for base in bases.iter().rev() {
            match base {
                ContractBase::Contract(contract) => {
                    errors.extend(contract.members().iter_error_definitions());
                }
                ContractBase::Interface(interface) => {
                    errors.extend(interface.members().iter_error_definitions());
                }
            }
        }
        errors
    }

    pub fn events(&self) -> Vec<EventDefinition> {
        self.members().iter_event_definitions().collect()
    }

    pub fn compute_linearised_events(&self) -> Vec<EventDefinition> {
        let mut events = Vec::new();
        let bases = self.compute_linearised_bases();
        for base in bases.iter().rev() {
            match base {
                ContractBase::Contract(contract) => {
                    events.extend(contract.members().iter_event_definitions());
                }
                ContractBase::Interface(interface) => {
                    events.extend(interface.members().iter_event_definitions());
                }
            }
        }
        events
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

    pub(crate) fn iter_error_definitions(&self) -> impl Iterator<Item = ErrorDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::ErrorDefinition(error_definition) = member {
                Some(error_definition)
            } else {
                None
            }
        })
    }

    pub(crate) fn iter_event_definitions(&self) -> impl Iterator<Item = EventDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::EventDefinition(event_definition) = member {
                Some(event_definition)
            } else {
                None
            }
        })
    }
}

impl InterfaceMembersStruct {
    pub(crate) fn iter_error_definitions(&self) -> impl Iterator<Item = ErrorDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::ErrorDefinition(error_definition) = member {
                Some(error_definition)
            } else {
                None
            }
        })
    }

    pub(crate) fn iter_event_definitions(&self) -> impl Iterator<Item = EventDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::EventDefinition(event_definition) = member {
                Some(event_definition)
            } else {
                None
            }
        })
    }
}

impl FunctionDefinitionStruct {
    pub(crate) fn overrides(&self, other: &FunctionDefinition) -> bool {
        let name_matches = match (&self.ir_node.name, &other.ir_node.name) {
            (None, None) => {
                // for unnamed functions, we check the kind because `receive`
                // and `fallback` may have the same parameters but they are
                // different functions
                self.ir_node.kind == other.ir_node.kind
            }
            (Some(name), Some(other_name)) => name.unparse() == other_name.unparse(),
            _ => false,
        };
        if !name_matches {
            return false;
        }
        let type_id = self
            .semantic
            .binder()
            .node_typing(self.ir_node.node_id)
            .as_type_id();
        let other_type_id = self
            .semantic
            .binder()
            .node_typing(other.ir_node.node_id)
            .as_type_id();

        match (type_id, other_type_id) {
            (Some(type_id), Some(other_type_id)) => self
                .semantic
                .types()
                .type_id_is_function_and_overrides(type_id, other_type_id),
            _ => false,
        }

        // TODO(validation): check also that the function mutability is stricter than other's
    }
}
